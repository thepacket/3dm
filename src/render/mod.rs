//! The wgpu side of 3DM.
//!
//! The fractal is drawn as an egui paint callback: egui owns the render pass and
//! the surface, and we inject one fullscreen triangle into it. That keeps the UI
//! and the fractal in a single pass with no intermediate textures.
//!
//! Because the distance estimator is generated from the formula stack, there is
//! one pipeline per stack *structure*, cached by signature. Parameter edits
//! reuse a pipeline; adding or reordering formulas compiles a new one.

pub mod codegen;

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicU32, AtomicU8, Ordering};

// Use eframe's re-export so we are guaranteed to be on the same wgpu version
// egui-wgpu was built against.
use eframe::egui_wgpu::{self, CallbackResources, CallbackTrait, ScreenDescriptor};
use eframe::wgpu;

use crate::camera::Camera;
use crate::formulas::{MAX_SLOTS, POOL_FLOATS_PER_SLOT, POOL_VEC4S_PER_SLOT};
use crate::params::SceneParams;

/// `vec4`s of parameter pool in the uniform block. Must match the array
/// length declared in `fractal.wgsl`.
pub const POOL_VEC4S: usize = MAX_SLOTS * POOL_VEC4S_PER_SLOT;

/// A single float per probe reading. `r32float` is renderable without any
/// optional feature, which matters because this has to work on the web too.
const PROBE_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba32Float;

/// Two pixels: the cursor ray's landing point, and the clear space ahead.
const PROBE_WIDTH: u32 = 2;

/// Copies the offscreen fractal into egui's target.
///
/// The fractal is not drawn straight into the window. At this display's full
/// screen size that is 21.5 megapixels of raymarching, which costs about half a
/// second on empty space and one and a half once the fractal fills the frame —
/// so the window grows and the app stops being interactive. It is rendered
/// smaller instead and stretched, at a scale the app picks from how long the
/// last frames took.
const BLIT_WGSL: &str = r#"
@group(0) @binding(0) var src: texture_2d<f32>;
@group(0) @binding(1) var samp: sampler;
// xy = the fraction of the texture actually drawn this frame. The texture is
// allocated once at the window's size and a smaller frame occupies its
// top-left corner, so nothing is reallocated when the scale changes.
@group(0) @binding(2) var<uniform> region: vec4<f32>;

struct VsOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@vertex
fn vs_blit(@builtin(vertex_index) vi: u32) -> VsOut {
    var corners = array<vec2<f32>, 3>(
        vec2<f32>(-1.0, -1.0),
        vec2<f32>(3.0, -1.0),
        vec2<f32>(-1.0, 3.0),
    );
    let c = corners[vi];
    var out: VsOut;
    out.pos = vec4<f32>(c, 0.0, 1.0);
    // Flip y: clip space is y-up, texture space is y-down.
    out.uv = vec2<f32>(c.x, -c.y) * 0.5 + 0.5;
    return out;
}

@fragment
fn fs_blit(in: VsOut) -> @location(0) vec4<f32> {
    // Clamped just inside the drawn region: without it the sampler's filter
    // reaches past the edge into whatever the rest of the texture holds.
    let limit = region.xy - region.zw;
    return textureSample(src, samp, min(in.uv * region.xy, limit));
}
"#;

/// GPU-side scene description. Laid out as `vec4`s so that WGSL's `uniform`
/// address space alignment rules are satisfied without any padding fields.
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Uniforms {
    cam_pos: [f32; 4],
    cam_right: [f32; 4],
    cam_up: [f32; 4],
    cam_fwd: [f32; 4],
    screen: [f32; 4],
    fractal: [f32; 4],
    march: [f32; 4],
    shade: [f32; 4],
    light: [f32; 4],
    palette_base: [f32; 4],
    palette_amp: [f32; 4],
    bg: [f32; 4],
    /// x = start iteration, y = end iteration, per slot.
    ranges: [[f32; 4]; MAX_SLOTS],
    pool: [[f32; 4]; POOL_VEC4S],
}

impl Default for Uniforms {
    fn default() -> Self {
        bytemuck::Zeroable::zeroed()
    }
}

impl Uniforms {
    pub fn build(
        camera: &Camera,
        params: &SceneParams,
        viewport_px: [f32; 2],
        time: f32,
        encode_srgb: bool,
        // Cursor position in normalised device coords, y up.
        cursor_ndc: [f32; 2],
    ) -> Self {
        let eye = camera.position();
        let (right, up, fwd) = camera.basis();
        let f = &params.fractal;
        let m = &params.march;
        let s = &params.shading;

        // Each slot's parameters go to the same fixed address the generated
        // shader reads them from; anything the formula does not use stays zero.
        let mut ranges = [[0.0f32; 4]; MAX_SLOTS];
        let mut pool = [[0.0f32; 4]; POOL_VEC4S];
        for (index, slot) in params.stack.active() {
            ranges[index] = [slot.start_iter as f32, slot.end_iter as f32, 0.0, 0.0];
            // Mandelbulber recomputes its derived parameters after every edit;
            // 3DM does it here, on the way to the GPU, so the editable values
            // and the ones the shader reads can share one block.
            let mut block: Vec<f32> = slot.params.clone();
            if let crate::formulas::FormulaKind::Generated(g) = slot.kind {
                crate::formulas::mb2::recompute(
                    &crate::formulas::generated::GENERATED[g],
                    &mut block,
                );
            }

            let base = index * POOL_FLOATS_PER_SLOT;
            for (i, v) in block.iter().take(POOL_FLOATS_PER_SLOT).enumerate() {
                pool[(base + i) / 4][(base + i) % 4] = *v;
            }
        }

        Self {
            cam_pos: [eye[0], eye[1], eye[2], camera.tan_half_fov()],
            // The basis `w` slots were padding; the cursor's normalised
            // position rides in them so the probe can build its ray.
            cam_right: [right[0], right[1], right[2], cursor_ndc[0]],
            cam_up: [up[0], up[1], up[2], cursor_ndc[1]],
            cam_fwd: [fwd[0], fwd[1], fwd[2], 0.0],
            screen: [
                viewport_px[0],
                viewport_px[1],
                time,
                if encode_srgb { 1.0 } else { 0.0 },
            ],
            // x carries the debug view; `power` now belongs to the formulas.
            fractal: [
                params.debug_view as u32 as f32,
                f.iterations as f32,
                f.bailout,
                f.de_scale,
            ],
            march: [
                m.max_steps as f32,
                m.max_dist,
                m.epsilon_scale,
                f.bounding_radius,
            ],
            shade: [s.ao_strength, s.shadow_softness, s.specular, s.glow],
            light: [s.light_dir[0], s.light_dir[1], s.light_dir[2], s.ambient],
            palette_base: [
                s.palette_base[0],
                s.palette_base[1],
                s.palette_base[2],
                s.palette_phase,
            ],
            palette_amp: [
                s.palette_amp[0],
                s.palette_amp[1],
                s.palette_amp[2],
                s.palette_freq,
            ],
            bg: [s.background[0], s.background[1], s.background[2], s.fog],
            ranges,
            pool,
        }
    }
}

/// Where the distance probe is in its copy/map cycle.
///
/// A buffer cannot be mapped while a copy into it is still in flight, so the
/// probe alternates: encode a copy one frame, ask for the mapping the next,
/// take the value when it lands. Three frames per reading at worst, which for
/// steering a camera is far more often than anyone can react.
mod probe_state {
    pub const IDLE: u8 = 0;
    pub const COPIED: u8 = 1;
    pub const MAPPING: u8 = 2;
}

/// Where the cursor is pointing, and how much room is ahead — read back from
/// the GPU because only the GPU can evaluate the estimator.
///
/// Two separate questions. The cursor's landing point is what lets the wheel
/// zoom somewhere the user chose rather than at the middle of the scene. The
/// clear space at the camera is what keeps forward flight from crossing a
/// surface, and it follows the view axis, which the cursor usually does not.
#[derive(Clone)]
pub struct CursorProbe {
    /// World position under the cursor, then the ray distance to it. A
    /// negative distance means the cursor is not over the fractal.
    target: Arc<[AtomicU32; 4]>,
    clearance: Arc<AtomicU32>,
    state: Arc<AtomicU8>,
}

impl CursorProbe {
    fn new() -> Self {
        Self {
            target: Arc::new([0, 0, 0, (-1.0f32).to_bits()].map(AtomicU32::new)),
            // Effectively unbounded until a reading lands, so nothing is
            // restricted by a measurement that does not exist yet.
            clearance: Arc::new(AtomicU32::new(f32::INFINITY.to_bits())),
            state: Arc::new(AtomicU8::new(probe_state::IDLE)),
        }
    }

    /// Clear space directly ahead of the camera, or infinity before the first
    /// reading arrives.
    pub fn clearance(&self) -> f32 {
        f32::from_bits(self.clearance.load(Ordering::Relaxed))
    }

    /// World point under the cursor, and how far along the ray it sits.
    /// `None` when the cursor is not over the fractal.
    pub fn target(&self) -> Option<([f32; 3], f32)> {
        let v: [f32; 4] =
            std::array::from_fn(|i| f32::from_bits(self.target[i].load(Ordering::Relaxed)));
        let distance: f32 = v[3];
        (distance > 0.0 && distance.is_finite()).then(|| ([v[0], v[1], v[2]], distance))
    }
}

/// Long-lived GPU objects, stashed in egui's `callback_resources` so they are
/// created once rather than per frame.
pub struct FractalPipeline {
    bind_group_layout: wgpu::BindGroupLayout,
    bind_group: wgpu::BindGroup,
    uniform_buffer: wgpu::Buffer,
    target_format: wgpu::TextureFormat,
    /// One compiled pipeline per formula-stack signature.
    pipelines: HashMap<u64, wgpu::RenderPipeline>,
    /// The 1x1 probe pipeline for each of those, sharing the shader module.
    probes: HashMap<u64, wgpu::RenderPipeline>,
    probe_target: wgpu::Texture,
    probe_view: wgpu::TextureView,
    probe_readback: wgpu::Buffer,
    /// Where the fractal is actually drawn, at or below the window's size.
    offscreen: Option<(wgpu::Texture, wgpu::TextureView, wgpu::BindGroup)>,
    offscreen_size: [u32; 2],
    blit: wgpu::RenderPipeline,
    blit_layout: wgpu::BindGroupLayout,
    blit_uniform: wgpu::Buffer,
    sampler: wgpu::Sampler,
    /// Viewport size the last frame was drawn at. A probe cycle spans three
    /// frames, and a surface reconfigure in the middle of one leaves a copy or
    /// a mapping straddling it.
    last_viewport: [f32; 2],
    pub cursor: CursorProbe,
    /// True when the surface format is linear and we must apply the sRGB
    /// transfer function in the shader ourselves.
    pub encode_srgb: bool,
}

impl FractalPipeline {
    pub fn new(device: &wgpu::Device, target_format: wgpu::TextureFormat) -> Self {
        let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("3dm.fractal.uniforms"),
            size: std::mem::size_of::<Uniforms>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("3dm.fractal.bgl"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("3dm.fractal.bg"),
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        // One float, but a copy destination has to be 256-byte aligned.
        let probe_target = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("3dm.probe.target"),
            size: wgpu::Extent3d {
                width: PROBE_WIDTH,
                height: 1,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: PROBE_FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });
        let probe_view = probe_target.create_view(&wgpu::TextureViewDescriptor::default());
        let probe_readback = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("3dm.probe.readback"),
            size: wgpu::COPY_BYTES_PER_ROW_ALIGNMENT as u64,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });

        let blit_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("3dm.blit.bgl"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });
        let blit_uniform = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("3dm.blit.region"),
            size: 16,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let blit_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("3dm.blit.shader"),
            source: wgpu::ShaderSource::Wgsl(BLIT_WGSL.into()),
        });
        let blit_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("3dm.blit.layout"),
            bind_group_layouts: &[Some(&blit_layout)],
            immediate_size: 0,
        });
        let blit = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("3dm.blit.pipeline"),
            layout: Some(&blit_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &blit_shader,
                entry_point: Some("vs_blit"),
                buffers: &[],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &blit_shader,
                entry_point: Some("fs_blit"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: target_format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                cull_mode: None,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview_mask: None,
            cache: None,
        });
        // Linear, so a reduced-resolution frame reads as soft rather than
        // blocky while the camera is moving.
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("3dm.blit.sampler"),
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });

        Self {
            bind_group_layout,
            bind_group,
            uniform_buffer,
            target_format,
            offscreen: None,
            offscreen_size: [0, 0],
            blit,
            blit_layout,
            blit_uniform,
            sampler,
            pipelines: HashMap::new(),
            probes: HashMap::new(),
            probe_target,
            probe_view,
            probe_readback,
            last_viewport: [0.0, 0.0],
            cursor: CursorProbe::new(),
            encode_srgb: !target_format.is_srgb(),
        }
    }

    /// Compiles the pipeline for `key` if it is not cached yet.
    pub fn ensure(&mut self, device: &wgpu::Device, key: u64, source: &str) {
        if self.pipelines.contains_key(&key) {
            return;
        }

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("3dm.fractal.shader"),
            source: wgpu::ShaderSource::Wgsl(source.into()),
        });

        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("3dm.fractal.layout"),
            bind_group_layouts: &[Some(&self.bind_group_layout)],
            immediate_size: 0,
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("3dm.fractal.pipeline"),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: self.target_format,
                    // The fractal is the backdrop of its panel and always opaque.
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                cull_mode: None,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview_mask: None,
            cache: None,
        });

        let probe = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("3dm.probe.pipeline"),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_probe"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: PROBE_FORMAT,
                    blend: None,
                    // All four: the cursor pixel carries a position *and* a
                    // distance. Masked to RED, as it was when this target held
                    // one number, the position reads back as (x, 0, 0) and the
                    // distance as whatever the clear left behind.
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                cull_mode: None,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview_mask: None,
            cache: None,
        });

        self.pipelines.insert(key, pipeline);
        self.probes.insert(key, probe);
    }

    /// Draws the fractal into the offscreen target, creating it if the size
    /// changed. Returns false if there is nothing to draw yet.
    fn render_offscreen(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        encoder: &mut wgpu::CommandEncoder,
        key: u64,
        full: [u32; 2],
        size: [u32; 2],
    ) -> bool {
        if !self.pipelines.contains_key(&key) {
            return false;
        }
        // Allocated at the window's size and only when *that* changes. Sizing
        // it to the reduced frame instead meant a new texture, view and bind
        // group on every frame the scale moved — which is every frame while the
        // camera is moving. Creating and destroying a multi-megapixel texture
        // that often costs far more than the pixels it saves, which is why
        // reducing the resolution could make the app slower rather than faster.
        if self.offscreen.is_none() || self.offscreen_size != full {
            let texture = device.create_texture(&wgpu::TextureDescriptor {
                label: Some("3dm.offscreen"),
                size: wgpu::Extent3d {
                    width: full[0],
                    height: full[1],
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: self.target_format,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                    | wgpu::TextureUsages::TEXTURE_BINDING,
                view_formats: &[],
            });
            let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
            let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("3dm.blit.bg"),
                layout: &self.blit_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&self.sampler),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: self.blit_uniform.as_entire_binding(),
                    },
                ],
            });
            self.offscreen = Some((texture, view, bind_group));
            self.offscreen_size = full;
        }

        // Which corner of the texture this frame occupies, and half a texel of
        // inset for the sampler to stay inside it.
        let fraction = [
            size[0] as f32 / full[0] as f32,
            size[1] as f32 / full[1] as f32,
        ];
        queue.write_buffer(
            &self.blit_uniform,
            0,
            bytemuck::cast_slice(&[
                fraction[0],
                fraction[1],
                0.5 / full[0] as f32,
                0.5 / full[1] as f32,
            ]),
        );

        let Some((_, view, _)) = &self.offscreen else {
            return false;
        };
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("3dm.offscreen.pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });
        // Draw only into the corner the reduced frame occupies.
        pass.set_viewport(0.0, 0.0, size[0] as f32, size[1] as f32, 0.0, 1.0);
        self.draw(&mut pass, key);
        true
    }

    /// Stretches the offscreen frame across egui's target.
    fn blit(&self, render_pass: &mut wgpu::RenderPass<'_>) {
        let Some((_, _, bind_group)) = &self.offscreen else {
            return;
        };
        render_pass.set_pipeline(&self.blit);
        render_pass.set_bind_group(0, bind_group, &[]);
        render_pass.draw(0..3, 0..1);
    }

    /// Advances the probe one step through its copy/map cycle.
    ///
    /// Encoded into egui's own command encoder, so the copy is submitted with
    /// the frame; the mapping is requested a frame later, once that submission
    /// has certainly happened.
    pub fn step_probe(&self, encoder: &mut wgpu::CommandEncoder, key: u64, stable: bool) {
        let probe = &self.cursor;
        match probe.state.load(Ordering::Acquire) {
            probe_state::IDLE => {
                // Never begin a cycle on a frame where the viewport changed
                // size. The surface is reconfigured on those frames, and
                // reconfiguring waits for outstanding GPU work — a copy or a
                // mapping left in flight across that point is exactly the kind
                // of thing that waits forever. Skipping costs one stale
                // reading; the camera does not notice.
                if !stable {
                    return;
                }
                let Some(pipeline) = self.probes.get(&key) else {
                    return;
                };
                {
                    let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: Some("3dm.probe.pass"),
                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                            view: &self.probe_view,
                            depth_slice: None,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                                store: wgpu::StoreOp::Store,
                            },
                        })],
                        depth_stencil_attachment: None,
                        timestamp_writes: None,
                        occlusion_query_set: None,
                        multiview_mask: None,
                    });
                    pass.set_pipeline(pipeline);
                    pass.set_bind_group(0, &self.bind_group, &[]);
                    pass.draw(0..3, 0..1);
                }
                encoder.copy_texture_to_buffer(
                    self.probe_target.as_image_copy(),
                    wgpu::TexelCopyBufferInfo {
                        buffer: &self.probe_readback,
                        layout: wgpu::TexelCopyBufferLayout {
                            offset: 0,
                            bytes_per_row: Some(wgpu::COPY_BYTES_PER_ROW_ALIGNMENT),
                            rows_per_image: Some(1),
                        },
                    },
                    wgpu::Extent3d {
                        width: PROBE_WIDTH,
                        height: 1,
                        depth_or_array_layers: 1,
                    },
                );
                probe.state.store(probe_state::COPIED, Ordering::Release);
            }
            probe_state::COPIED => {
                probe.state.store(probe_state::MAPPING, Ordering::Release);
                let target = Arc::clone(&probe.target);
                let clearance = Arc::clone(&probe.clearance);
                let state = Arc::clone(&probe.state);
                let buffer = self.probe_readback.clone();
                self.probe_readback.slice(..).map_async(
                    wgpu::MapMode::Read,
                    move |result| {
                        if result.is_ok() {
                            let raw = buffer.slice(..).get_mapped_range();
                            let at = |i: usize| {
                                let b = i * 4;
                                f32::from_le_bytes([raw[b], raw[b + 1], raw[b + 2], raw[b + 3]])
                            };
                            // Pixel 0 is the cursor's landing point, pixel 1 the
                            // clear space at the camera.
                            for i in 0..4 {
                                target[i].store(at(i).to_bits(), Ordering::Relaxed);
                            }
                            let ahead = at(4);
                            drop(raw);
                            buffer.unmap();
                            if ahead.is_finite() && ahead >= 0.0 {
                                clearance.store(ahead.to_bits(), Ordering::Relaxed);
                            }
                        }
                        state.store(probe_state::IDLE, Ordering::Release);
                    },
                );
                // Deliberately no `device.poll` here. This runs inside egui's
                // paint callback, with an encoder open, and driving device
                // maintenance re-entrantly from that point is asking for
                // trouble — the mapping is serviced by the normal per-frame
                // maintenance a moment later instead.
            }
            _ => {}
        }
    }

    /// Push a new scene to the GPU. Cheap enough to do every frame.
    pub fn upload(&self, queue: &wgpu::Queue, uniforms: &Uniforms) {
        queue.write_buffer(&self.uniform_buffer, 0, bytemuck::bytes_of(uniforms));
    }

    /// Issue the fullscreen triangle into an already-configured pass. The
    /// caller owns the viewport, so this works both inside egui's pass and in
    /// a standalone offscreen pass.
    pub fn draw(&self, render_pass: &mut wgpu::RenderPass<'_>, key: u64) {
        let Some(pipeline) = self.pipelines.get(&key) else {
            return;
        };
        render_pass.set_pipeline(pipeline);
        render_pass.set_bind_group(0, &self.bind_group, &[]);
        render_pass.draw(0..3, 0..1);
    }

    /// Number of distinct stack structures compiled so far — surfaced in the UI
    /// so the cost of shader rebuilds is visible rather than mysterious.
    pub fn compiled_count(&self) -> usize {
        self.pipelines.len()
    }
}

/// Per-frame payload handed to egui.
pub struct FractalCallback {
    pub uniforms: Uniforms,
    /// Viewport size in physical pixels, used to spot a resize.
    pub viewport: [f32; 2],
    /// Size the fractal is actually rendered at, at or below the viewport.
    pub render_size: [u32; 2],
    pub shader_key: u64,
    /// Shared with the app so an unchanged stack costs no allocation.
    pub shader_source: Arc<str>,
}

impl CallbackTrait for FractalCallback {
    fn prepare(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        _screen_descriptor: &ScreenDescriptor,
        encoder: &mut wgpu::CommandEncoder,
        resources: &mut CallbackResources,
    ) -> Vec<wgpu::CommandBuffer> {
        if let Some(res) = resources.get_mut::<FractalPipeline>() {
            res.ensure(device, self.shader_key, &self.shader_source);
            res.upload(queue, &self.uniforms);
            let stable = res.last_viewport == self.viewport;
            res.last_viewport = self.viewport;
            res.step_probe(encoder, self.shader_key, stable);
            res.render_offscreen(
                device,
                queue,
                encoder,
                self.shader_key,
                [self.viewport[0] as u32, self.viewport[1] as u32],
                self.render_size,
            );
        }
        Vec::new()
    }

    fn paint(
        &self,
        _info: egui::PaintCallbackInfo,
        render_pass: &mut wgpu::RenderPass<'static>,
        resources: &CallbackResources,
    ) {
        let Some(res) = resources.get::<FractalPipeline>() else {
            return;
        };
        // egui has already set the viewport to our widget's rect, so the
        // fullscreen triangle lands exactly inside the panel.
        res.blit(render_pass);
    }
}

/// Convenience wrapper matching egui's paint-callback shape.
pub fn callback(
    rect: egui::Rect,
    uniforms: Uniforms,
    viewport: [f32; 2],
    render_size: [u32; 2],
    shader_key: u64,
    shader_source: Arc<str>,
) -> egui::PaintCallback {
    egui_wgpu::Callback::new_paint_callback(
        rect,
        FractalCallback {
            uniforms,
            viewport,
            render_size,
            shader_key,
            shader_source,
        },
    )
}
