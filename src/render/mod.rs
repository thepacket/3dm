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
            let base = index * POOL_FLOATS_PER_SLOT;
            for (i, v) in slot.params.iter().take(POOL_FLOATS_PER_SLOT).enumerate() {
                pool[(base + i) / 4][(base + i) % 4] = *v;
            }
        }

        Self {
            cam_pos: [eye[0], eye[1], eye[2], camera.tan_half_fov()],
            cam_right: [right[0], right[1], right[2], 0.0],
            cam_up: [up[0], up[1], up[2], 0.0],
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

/// Long-lived GPU objects, stashed in egui's `callback_resources` so they are
/// created once rather than per frame.
pub struct FractalPipeline {
    bind_group_layout: wgpu::BindGroupLayout,
    bind_group: wgpu::BindGroup,
    uniform_buffer: wgpu::Buffer,
    target_format: wgpu::TextureFormat,
    /// One compiled pipeline per formula-stack signature.
    pipelines: HashMap<u64, wgpu::RenderPipeline>,
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

        Self {
            bind_group_layout,
            bind_group,
            uniform_buffer,
            target_format,
            pipelines: HashMap::new(),
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

        self.pipelines.insert(key, pipeline);
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
        _encoder: &mut wgpu::CommandEncoder,
        resources: &mut CallbackResources,
    ) -> Vec<wgpu::CommandBuffer> {
        if let Some(res) = resources.get_mut::<FractalPipeline>() {
            res.ensure(device, self.shader_key, &self.shader_source);
            res.upload(queue, &self.uniforms);
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
        res.draw(render_pass, self.shader_key);
    }
}

/// Convenience wrapper matching egui's paint-callback shape.
pub fn callback(
    rect: egui::Rect,
    uniforms: Uniforms,
    shader_key: u64,
    shader_source: Arc<str>,
) -> egui::PaintCallback {
    egui_wgpu::Callback::new_paint_callback(
        rect,
        FractalCallback {
            uniforms,
            shader_key,
            shader_source,
        },
    )
}
