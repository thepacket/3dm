//! Application state and UI.

use std::sync::Arc;
use std::time::Duration;

use crate::camera::Camera;
use crate::formulas::{Builtin, DeMode, FormulaKind, FormulaSlot, generated::GENERATED};
use crate::params::{DebugView, SceneParams};
use crate::render::{CursorProbe, FractalPipeline, Uniforms, callback};

/// Radians of orbit per point of mouse travel.
const ORBIT_SENSITIVITY: f32 = 0.007;

/// Keyboard navigation rates, all per second so that holding a key moves the
/// same distance regardless of frame rate — which matters here, where the frame
/// rate depends on the fractal.
const KEY_ORBIT: f32 = 1.6;
/// How fast the gap to the surface closes, in e-foldings per second.
///
/// Forward speed is proportional to the *measured clear space*, not to the
/// orbit radius. That difference is the whole feel of the thing: scaled by the
/// orbit radius the camera crawls at one fixed speed whether the surface is two
/// units away or two thousandths, because flying does not change the radius.
/// Scaled by clearance it crosses open space quickly and slows only where there
/// is genuinely something close — and since each step is a fraction of the gap,
/// it still cannot cross.
///
/// Chosen by simulating the approach rather than by feel. At 16 it takes about
/// half a second to close any gap — 0.40s across half a unit, 0.48s across two,
/// 0.57s across eight — against 0.43s, 1.00s and 3.27s for the fixed-speed
/// version this replaced. Uniform across scales is the point: a descent should
/// not get slower simply because it is going further.
const KEY_FLY: f32 = 16.0;

/// Most of the clear space a single frame may consume.
///
/// The exponential above already keeps each step below the gap, but only if the
/// gap is current, and the reading is up to three frames old — the probe copies
/// on one frame, maps on the next, and lands on the one after. At a low frame
/// rate one exponential step can be most of the gap, and three of those on the
/// same stale number would cross. Capping the per-frame fraction at a third
/// bounds three stale steps to the whole gap and no further.
const FLY_MAX_FRACTION: f32 = 0.3;

/// Orbit radii per second, used where there is no measurement to scale by:
/// retreating, which has nothing to hit, and the first frames before the probe
/// has reported anything.
const KEY_FLY_BLIND: f32 = 0.8;
/// Screen heights of pan per second.
const KEY_PAN: f32 = 0.9;

/// Fraction of the way to the cursor's target that one wheel notch travels.
///
/// Kept well under one so the gesture is repeatable: each notch closes a fifth
/// of what is left, which never arrives and so never overshoots into the solid.
const WHEEL_APPROACH: f32 = 0.2;

pub struct App {
    camera: Camera,
    params: SceneParams,
    /// Search text for the transpiled-formula picker. Lives on the app rather
    /// than in egui's temp state so that it survives the combo box closing.
    formula_filter: String,
    /// Set when the surface format is linear, so the shader must apply sRGB.
    encode_srgb: bool,
    /// Drop iteration and step counts while the camera is moving, so dragging
    /// stays responsive on a fractal tuned for a detailed still image.
    adaptive_quality: bool,
    /// True on frames where the user is actively moving the camera.
    interacting: bool,
    /// Wall-clock seconds since launch, fed to the shader for future animation.
    elapsed: f32,
    /// Smoothed frame time, for the readout in the top bar.
    frame_ms: f32,
    /// What the GPU last reported about the cursor ray and the space ahead.
    /// Drives both the zoom-toward-cursor gesture and the flight speed.
    cursor: CursorProbe,
    /// Cursor position over the viewport in normalised device coords, y up.
    cursor_ndc: [f32; 2],
    /// Generated WGSL for the current stack, together with the signature it was
    /// generated from. Regenerated only when the stack's structure changes, so
    /// dragging a parameter never rebuilds a shader.
    shader: (u64, Arc<str>),
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Option<Self> {
        // `wgpu_render_state` is None if eframe fell back to another backend.
        let render_state = cc.wgpu_render_state.as_ref()?;

        let pipeline = FractalPipeline::new(&render_state.device, render_state.target_format);
        let encode_srgb = pipeline.encode_srgb;
        let cursor = pipeline.cursor.clone();

        // egui hands these resources back to us inside the paint callback.
        render_state
            .renderer
            .write()
            .callback_resources
            .insert(pipeline);

        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        cc.egui_ctx.all_styles_mut(|style| {
            style.spacing.slider_width = 150.0;
        });

        let mut params = SceneParams::default();
        params.retune_for_stack();
        let (key, source) = params.shader();

        Some(Self {
            camera: Camera::default(),
            params,
            formula_filter: String::new(),
            cursor,
            cursor_ndc: [0.0, 0.0],
            encode_srgb,
            adaptive_quality: true,
            interacting: false,
            elapsed: 0.0,
            frame_ms: 0.0,
            shader: (key, source.into()),
        })
    }

    /// The parameters actually sent to the GPU this frame, after any
    /// interaction-time quality reduction.
    fn effective_params(&self) -> SceneParams {
        let mut p = self.params.clone();
        if self.adaptive_quality && self.interacting {
            p.fractal.iterations = (p.fractal.iterations * 2 / 3).max(4);
            p.march.max_steps = (p.march.max_steps / 2).max(32);
            p.march.epsilon_scale *= 2.5;
        }
        p
    }

    fn controls(&mut self, ui: &mut egui::Ui) {
        // Split the borrow so the reset buttons below can still touch `self`.
        let Self {
            params,
            camera,
            adaptive_quality,
            formula_filter,
            ..
        } = self;
        let mut reset_view = false;
        let mut reset_all = false;

        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.add_space(4.0);

            egui::CollapsingHeader::new("Formula stack")
                .default_open(true)
                .show(ui, |ui| stack_editor(params, formula_filter, ui));

            egui::CollapsingHeader::new("Iteration")
                .default_open(true)
                .show(ui, |ui| {
                    let f = &mut params.fractal;
                    ui.add(
                        egui::Slider::new(&mut f.iterations, 2..=512)
                            .text("iterations")
                            .logarithmic(true),
                    )
                    .on_hover_text(
                        "How deep the escape-time loop runs. Detail at a given \
                         zoom is limited by this before it is limited by \
                         anything else, so raise it as you descend.",
                    );
                    ui.add(egui::Slider::new(&mut f.bailout, 1.5..=64.0).text("bailout"));
                    ui.add(
                        egui::Slider::new(&mut f.bounding_radius, 0.5..=20.0)
                            .text("bounds")
                            .logarithmic(true),
                    )
                    .on_hover_text(
                        "Radius of the sphere the marcher searches. Too small \
                         clips the fractal; too large only costs a few steps. \
                         Retuned automatically when the stack changes.",
                    );
                    ui.add(
                        egui::Slider::new(&mut f.de_scale, 0.2..=1.0)
                            .text("DE scale")
                            .step_by(0.01),
                    )
                    .on_hover_text(
                        "Shortens each march step. Lower values fix banding and \
                         holes at the cost of speed.",
                    );
                });

            egui::CollapsingHeader::new("Raymarch")
                .default_open(true)
                .show(ui, |ui| {
                    let m = &mut params.march;
                    ui.add(egui::Slider::new(&mut m.max_steps, 32..=512).text("max steps"));
                    ui.add(
                        egui::Slider::new(&mut m.epsilon_scale, 0.2..=4.0)
                            .text("detail")
                            .logarithmic(true),
                    )
                    .on_hover_text("Hit threshold in pixels. Lower is sharper and slower.");
                    ui.add(egui::Slider::new(&mut m.max_dist, 2.0..=40.0).text("max distance"));
                    ui.checkbox(adaptive_quality, "Fast preview while moving");
                });

            egui::CollapsingHeader::new("Shading")
                .default_open(true)
                .show(ui, |ui| {
                    let s = &mut params.shading;
                    ui.label("light direction");
                    ui.horizontal(|ui| {
                        for (axis, value) in ["x", "y", "z"].iter().zip(s.light_dir.iter_mut()) {
                            ui.add(
                                egui::DragValue::new(value)
                                    .speed(0.02)
                                    .prefix(format!("{axis} ")),
                            );
                        }
                    });
                    ui.add(egui::Slider::new(&mut s.ambient, 0.0..=1.0).text("ambient"));
                    ui.add(egui::Slider::new(&mut s.ao_strength, 0.0..=1.0).text("occlusion"));
                    ui.add(
                        egui::Slider::new(&mut s.shadow_softness, 1.0..=64.0)
                            .text("shadow sharpness")
                            .logarithmic(true),
                    );
                    ui.add(egui::Slider::new(&mut s.specular, 0.0..=2.0).text("specular"));
                    ui.add(egui::Slider::new(&mut s.glow, 0.0..=1.5).text("glow"));
                });

            egui::CollapsingHeader::new("Palette")
                .default_open(true)
                .show(ui, |ui| {
                    let s = &mut params.shading;
                    ui.horizontal(|ui| {
                        ui.color_edit_button_rgb(&mut s.palette_base);
                        ui.label("base");
                    });
                    ui.horizontal(|ui| {
                        ui.color_edit_button_rgb(&mut s.palette_amp);
                        ui.label("range");
                    });
                    ui.add(egui::Slider::new(&mut s.palette_phase, 0.0..=1.0).text("hue shift"));
                    ui.add(egui::Slider::new(&mut s.palette_freq, 0.0..=6.0).text("banding"));
                    ui.separator();
                    ui.horizontal(|ui| {
                        ui.color_edit_button_rgb(&mut s.background);
                        ui.label("background");
                    });
                    ui.add(egui::Slider::new(&mut s.fog, 0.0..=1.0).text("fog"));
                });

            egui::CollapsingHeader::new("Camera")
                .default_open(false)
                .show(ui, |ui| {
                    ui.add(egui::Slider::new(&mut camera.fov_y, 15.0..=110.0).text("field of view"));
                    ui.add(
                        egui::Slider::new(&mut camera.distance, 0.05..=20.0)
                            .text("distance")
                            .logarithmic(true),
                    );
                    reset_view = ui.button("Reset view").clicked();
                });

            egui::CollapsingHeader::new("Diagnostics")
                .default_open(false)
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label("show");
                        egui::ComboBox::from_id_salt("debug_view")
                            .selected_text(params.debug_view.label())
                            .width(130.0)
                            .show_ui(ui, |ui| {
                                for view in DebugView::ALL {
                                    if ui
                                        .selectable_label(params.debug_view == view, view.label())
                                        .clicked()
                                    {
                                        params.debug_view = view;
                                    }
                                }
                            });
                    })
                    .response
                    .on_hover_text(
                        "Render one shading term on its own. The final image is a \
                         product of all of them, so a dark result cannot tell you \
                         which one is responsible — this can.",
                    );
                });

            egui::CollapsingHeader::new("Keyboard")
                .default_open(false)
                .show(ui, shortcuts);

            ui.add_space(8.0);
            ui.separator();
            reset_all = ui.button("Reset everything").clicked();
            ui.add_space(8.0);
        });

        if reset_view || reset_all {
            self.camera = Camera::default();
        }
        if reset_all {
            self.params = SceneParams::default();
        }
    }

    /// The target cross and its readout.
    ///
    /// The numbers are the point the wheel would travel to and how far away it
    /// is, which is worth showing for the same reason Mandelbulber shows them:
    /// the gesture is "go *there*", and without the cross there is no there.
    /// `dist` doubles as a depth gauge — watching it fall by orders of
    /// magnitude is how a descent is read.
    fn draw_crosshair(&self, ui: &egui::Ui, rect: egui::Rect, pos: egui::Pos2) {
        let painter = ui.painter_at(rect);
        let Some((point, distance)) = self.cursor.target() else {
            return;
        };

        let pale = egui::Color32::from_white_alpha(120);
        let stroke = egui::Stroke::new(1.0, pale);
        let gap = 6.0;
        for (from, to) in [
            (egui::pos2(rect.left(), pos.y), egui::pos2(pos.x - gap, pos.y)),
            (egui::pos2(pos.x + gap, pos.y), egui::pos2(rect.right(), pos.y)),
            (egui::pos2(pos.x, rect.top()), egui::pos2(pos.x, pos.y - gap)),
            (egui::pos2(pos.x, pos.y + gap), egui::pos2(pos.x, rect.bottom())),
        ] {
            painter.line_segment([from, to], stroke);
        }

        // Enough digits to watch a deep descent: by the time `dist` is 1e-6 the
        // leading figures have stopped changing and only the tail moves.
        let text = format!(
            "x: {:.9}\ny: {:.9}\nz: {:.9}\ndist: {:.9}",
            point[0], point[1], point[2], distance
        );
        let anchor = egui::pos2(pos.x + 12.0, pos.y + 12.0);
        let galley = painter.layout(
            text,
            egui::FontId::monospace(11.0),
            egui::Color32::from_white_alpha(220),
            f32::INFINITY,
        );
        // Flip the label back inside when the pointer nears an edge.
        let size = galley.size();
        let origin = egui::pos2(
            anchor.x.min(rect.right() - size.x - 4.0),
            anchor.y.min(rect.bottom() - size.y - 4.0),
        );
        painter.rect_filled(
            egui::Rect::from_min_size(origin, size).expand(4.0),
            3.0,
            egui::Color32::from_black_alpha(160),
        );
        painter.galley(origin, galley, egui::Color32::PLACEHOLDER);
    }

    /// Camera keys: WASD to move, QE to rise and fall, arrows to orbit.
    ///
    /// Reads the keyboard and hands the result to [`Nav::apply`], which is
    /// where the camera maths lives — a window is the one thing a test cannot
    /// open, so the decision is kept clear of egui.
    fn keyboard_navigation(&mut self, ui: &mut egui::Ui, dt: f32) -> bool {
        // A focused text field owns the keyboard. Without this, typing a
        // formula name into the filter box would fly the camera instead.
        if ui.ctx().egui_wants_keyboard_input() {
            return false;
        }

        let axis = |negative: egui::Key, positive: egui::Key| {
            ui.input(|i| i.key_down(positive) as i32 as f32 - i.key_down(negative) as i32 as f32)
        };

        let nav = Nav {
            yaw: axis(egui::Key::ArrowLeft, egui::Key::ArrowRight),
            pitch: axis(egui::Key::ArrowDown, egui::Key::ArrowUp),
            forward: axis(egui::Key::S, egui::Key::W),
            strafe: axis(egui::Key::A, egui::Key::D),
            lift: axis(egui::Key::Q, egui::Key::E),
            // Shift hurries, Alt creeps — the range the mouse gets from moving
            // faster or slower, which a held key cannot express on its own.
            clearance: self.cursor.clearance(),
            speed: ui.input(|i| {
                if i.modifiers.shift {
                    3.0
                } else if i.modifiers.alt {
                    0.3
                } else {
                    1.0
                }
            }),
        };

        nav.apply(&mut self.camera, dt)
    }

    /// Draws the fractal and handles camera input over it.
    fn viewport(&mut self, ui: &mut egui::Ui) {
        let (rect, response) =
            ui.allocate_exact_size(ui.available_size(), egui::Sense::click_and_drag());

        self.interacting = false;

        // A held key produces no further events, so the frame that would apply
        // the next step of the motion has to be asked for explicitly.
        let dt = ui.input(|i| i.stable_dt).min(0.1);
        if self.keyboard_navigation(ui, dt) {
            self.interacting = true;
            ui.ctx().request_repaint();
        }

        if response.dragged() {
            let delta = response.drag_delta();
            let pan = response.dragged_by(egui::PointerButton::Secondary)
                || ui.input(|i| i.modifiers.shift);

            if pan {
                self.camera
                    .pan(delta.x / rect.height(), delta.y / rect.height());
            } else {
                self.camera
                    .orbit(-delta.x * ORBIT_SENSITIVITY, delta.y * ORBIT_SENSITIVITY);
            }
            self.interacting = delta != egui::Vec2::ZERO;
        }

        // Where the pointer is, in the same normalised coordinates the shader
        // builds rays from. The probe uses it to report what is under it.
        if let Some(pos) = response.hover_pos() {
            self.cursor_ndc = [
                ((pos.x - rect.left()) / rect.width().max(1.0)) * 2.0 - 1.0,
                1.0 - ((pos.y - rect.top()) / rect.height().max(1.0)) * 2.0,
            ];
        }

        if response.hovered() {
            let scroll = ui.input(|i| i.smooth_scroll_delta.y);
            if scroll != 0.0 {
                match self.cursor.target() {
                    // Go where the pointer is aiming. Approaching a point the
                    // user picked is the whole difference between inspecting
                    // the scene and travelling through it.
                    Some((point, _)) => {
                        let notches = scroll * 0.01;
                        let fraction = 1.0 - (1.0 - WHEEL_APPROACH).powf(notches.abs());
                        if notches > 0.0 {
                            self.camera.approach(point, fraction);
                        } else {
                            // Backing off along the same line, so the gesture
                            // retraces its own path rather than drifting.
                            self.camera.approach(point, -fraction);
                        }
                    }
                    // Pointing at empty space: fall back to plain dollying, or
                    // the wheel would do nothing at all off the silhouette.
                    None => self.camera.zoom((-scroll * 0.002).exp()),
                }
                self.interacting = true;
            }
        }

        // A zero-area viewport would divide by zero building the ray basis.
        if rect.width() < 1.0 || rect.height() < 1.0 {
            return;
        }

        let ppp = ui.ctx().pixels_per_point();
        let uniforms = Uniforms::build(
            &self.camera,
            &self.effective_params(),
            [rect.width() * ppp, rect.height() * ppp],
            self.elapsed,
            self.encode_srgb,
            self.cursor_ndc,
        );
        ui.painter().add(callback(
            rect,
            uniforms,
            self.shader.0,
            Arc::clone(&self.shader.1),
        ));

        // After the fractal, not before. egui paints a layer in the order
        // things were added, so a cross drawn first is a cross the fractal
        // paints over — present, correct, and invisible.
        if let Some(pos) = response.hover_pos() {
            self.draw_crosshair(ui, rect, pos);
        }
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // Regenerate only when the stack's *structure* changed. Parameter and
        // iteration-range edits ride along in uniforms and reuse the pipeline.
        let signature = self.params.stack.signature();
        if signature != self.shader.0 {
            self.shader = (
                signature,
                crate::render::codegen::generate(&self.params.stack).into(),
            );
            self.params.retune_for_stack();
            // Swapping a bulb for a box changes the scene's scale by several
            // times over; without reframing the camera ends up inside it.
            self.camera.frame(self.params.fractal.bounding_radius);
        }

        let dt = ui.input(|i| i.stable_dt).min(0.1);
        self.elapsed += dt;
        self.frame_ms += (dt * 1000.0 - self.frame_ms) * 0.1;

        egui::Panel::top("3dm.topbar").show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.heading("3DM");
                ui.label(egui::RichText::new("Mandelbulb").weak());
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(
                        egui::RichText::new(format!("{:.1} ms", self.frame_ms))
                            .monospace()
                            .weak(),
                    );
                });
            });
        });

        egui::Panel::left("3dm.controls")
            .default_size(272.0)
            .min_size(220.0)
            .show(ui, |ui| self.controls(ui));

        egui::CentralPanel::no_frame().show(ui, |ui| self.viewport(ui));

        // While the camera is moving we render a cheap preview; schedule one
        // more frame shortly after so the full-quality image replaces it.
        if self.interacting {
            ui.ctx().request_repaint_after(Duration::from_millis(120));
        }
    }

    #[cfg(target_arch = "wasm32")]
    fn as_any_mut(&mut self) -> Option<&mut dyn std::any::Any> {
        Some(&mut *self)
    }
}

/// The camera motion a frame's held keys ask for, in units of "how much of
/// each axis", before any rate or timestep is applied.
#[derive(Clone, Copy, Default, PartialEq, Debug)]
struct Nav {
    yaw: f32,
    pitch: f32,
    forward: f32,
    strafe: f32,
    lift: f32,
    /// Modifier multiplier: 3 with shift, 0.3 with alt, 1 otherwise.
    speed: f32,
    /// Clear space ahead of the camera, from the GPU probe. Infinite until the
    /// first reading lands.
    clearance: f32,
}

impl Nav {
    /// Moves `camera` by one frame's worth. Returns whether anything moved.
    fn apply(self, camera: &mut Camera, dt: f32) -> bool {
        if [self.yaw, self.pitch, self.forward, self.strafe, self.lift]
            .iter()
            .all(|v| *v == 0.0)
        {
            return false;
        }

        let step = dt * self.speed;

        // Negated to match the drag directions: the right arrow orbits the way
        // dragging right does, so the two controls never disagree.
        camera.orbit(-self.yaw * KEY_ORBIT * step, -self.pitch * KEY_ORBIT * step);
        if self.forward != 0.0 {
            let travel = if self.forward > 0.0 && self.clearance.is_finite() {
                // Approaching: close a proportion of the measured gap. Being
                // exponential it composes exactly frame to frame, and being a
                // proportion it can never arrive.
                let fraction = (1.0 - (-KEY_FLY * step).exp()).min(FLY_MAX_FRACTION);
                self.clearance * fraction
            } else {
                // Retreating has nothing ahead to respect, and has to work when
                // boxed in against a surface where the gap reads as almost
                // nothing. Same path covers the first frames, before the probe
                // has reported at all.
                self.forward * camera.distance * KEY_FLY_BLIND * step
            };
            camera.advance(travel / camera.distance);
        }
        if self.strafe != 0.0 || self.lift != 0.0 {
            camera.pan(-self.strafe * KEY_PAN * step, self.lift * KEY_PAN * step);
        }
        true
    }
}



/// What a row in the stack editor asked to do. Applied after the loop, because
/// mutating the vector while iterating it is not allowed.
enum SlotAction {
    Remove(usize),
    MoveUp(usize),
    MoveDown(usize),
}


/// The formula picker: 3DM's own handful first, then the transpiled
/// Mandelbulber corpus behind a text filter, because 361 entries in a combo box
/// is a list nobody can read.
fn formula_picker(ui: &mut egui::Ui, filter: &mut String, mut choose: impl FnMut(FormulaKind)) {
    for b in Builtin::ALL {
        if ui.selectable_label(false, b.name()).clicked() {
            choose(FormulaKind::Builtin(b));
        }
    }

    ui.separator();
    ui.add(egui::TextEdit::singleline(filter).hint_text("filter Mandelbulber…"));

    let needle = filter.to_lowercase();
    egui::ScrollArea::vertical()
        .max_height(240.0)
        .show(ui, |ui| {
            let mut shown = 0usize;
            for (i, f) in GENERATED.iter().enumerate() {
                if !needle.is_empty() && !f.name.to_lowercase().contains(&needle) {
                    continue;
                }
                // Cap the unfiltered list: scrolling 361 rows to find one is
                // what the filter is for.
                shown += 1;
                if shown > 80 {
                    ui.label(egui::RichText::new("…keep typing to narrow").small().weak());
                    break;
                }
                if ui
                    .selectable_label(false, f.name)
                    .on_hover_text(f.source)
                    .clicked()
                {
                    choose(FormulaKind::Generated(i));
                }
            }
        });
}

/// Every binding, in one place.
///
/// Navigation here is half keyboard and half mouse, and two of the keys do
/// nothing on their own — they change what the mouse is doing. That is not
/// discoverable by pressing things, so it is written down.
fn shortcuts(ui: &mut egui::Ui) {
    fn rows(ui: &mut egui::Ui, id: &str, entries: &[(&str, &str)]) {
        egui::Grid::new(id)
            .num_columns(2)
            .spacing([12.0, 2.0])
            .show(ui, |ui| {
                for (keys, what) in entries {
                    ui.label(egui::RichText::new(*keys).monospace());
                    ui.label(egui::RichText::new(*what).weak());
                    ui.end_row();
                }
            });
    }

    ui.label(egui::RichText::new("Move").strong());
    rows(
        ui,
        "keys.move",
        &[
            ("W  S", "fly forward, back"),
            ("A  D", "strafe left, right"),
            ("Q  E", "down, up"),
            ("← → ↑ ↓", "orbit"),
        ],
    );

    ui.add_space(6.0);
    ui.label(egui::RichText::new("Mouse").strong());
    rows(
        ui,
        "keys.mouse",
        &[
            ("drag", "orbit"),
            ("right-drag", "pan"),
            ("wheel", "approach the target cross"),
        ],
    );

    ui.add_space(6.0);
    ui.label(egui::RichText::new("Held modifiers").strong());
    rows(
        ui,
        "keys.mods",
        &[
            ("shift + drag", "pan, instead of orbit"),
            ("shift", "move three times faster"),
            ("alt", "move three times slower"),
        ],
    );

    ui.add_space(6.0);
    ui.label(
        egui::RichText::new(
            "The wheel closes a fifth of the way to the cross each notch, so it \
             never arrives. Off the fractal there is nothing to approach and it \
             dollies instead.\n\nCamera keys are ignored while a text box has \
             focus, so typing in the formula filter does not fly the camera.",
        )
        .small()
        .weak(),
    );
}

/// The formula stack editor: an ordered list of formulas, each with its own
/// parameters and the range of iterations over which it applies.
fn stack_editor(params: &mut SceneParams, filter: &mut String, ui: &mut egui::Ui) {
    let max_iter = params.fractal.iterations;
    let stack = &mut params.stack;
    let mut action = None;
    let count = stack.slots.len();

    for (index, slot) in stack.slots.iter_mut().enumerate() {
        ui.push_id(index, |ui| {
            egui::Frame::group(ui.style()).show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.checkbox(&mut slot.enabled, "")
                        .on_hover_text("Include this formula in the stack");

                    egui::ComboBox::from_id_salt("kind")
                        .selected_text(slot.kind.name())
                        .width(120.0)
                        .show_ui(ui, |ui| {
                            formula_picker(ui, filter, |kind| {
                                if slot.kind == kind {
                                    return;
                                }
                                // Swapping the formula must reset the
                                // parameters — slot 1 of a Mandelbox means
                                // something different than slot 1 of a bulb.
                                let range = (slot.start_iter, slot.end_iter);
                                *slot = FormulaSlot::new(kind);
                                slot.start_iter = range.0;
                                slot.end_iter = range.1;
                            });
                        });

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.small_button("✕").on_hover_text("Remove").clicked() {
                            action = Some(SlotAction::Remove(index));
                        }
                        if ui
                            .add_enabled(index + 1 < count, egui::Button::new("▼").small())
                            .on_hover_text("Move down")
                            .clicked()
                        {
                            action = Some(SlotAction::MoveDown(index));
                        }
                        if ui
                            .add_enabled(index > 0, egui::Button::new("▲").small())
                            .on_hover_text("Move up")
                            .clicked()
                        {
                            action = Some(SlotAction::MoveUp(index));
                        }
                    });
                });

                ui.add_enabled_ui(slot.enabled, |ui| {
                    for control in slot.kind.controls() {
                        let Some(value) = slot.params.get_mut(control.index) else {
                            continue;
                        };
                        match control.range {
                            Some((min, max)) => {
                                ui.add(egui::Slider::new(value, min..=max).text(&control.label));
                            }
                            // No known bounds, so a drag value rather than a
                            // slider with an invented range.
                            None => {
                                ui.add(
                                    egui::DragValue::new(value)
                                        .speed(0.01)
                                        .prefix(format!("{}: ", control.label)),
                                );
                            }
                        }
                    }

                    ui.horizontal(|ui| {
                        ui.label("iterations");
                        ui.add(
                            egui::DragValue::new(&mut slot.start_iter)
                                .range(0..=max_iter)
                                .speed(0.1),
                        );
                        ui.label("to");
                        ui.add(
                            egui::DragValue::new(&mut slot.end_iter)
                                .range(0..=max_iter)
                                .speed(0.1),
                        );
                    })
                    .response
                    .on_hover_text(
                        "Restrict this formula to part of the iteration loop. \
                         Overlapping ranges blend the formulas; disjoint ones \
                         fuse two distinct shapes.",
                    );
                });
            });
        });
    }

    match action {
        Some(SlotAction::Remove(i)) => {
            stack.slots.remove(i);
        }
        Some(SlotAction::MoveUp(i)) => stack.slots.swap(i - 1, i),
        Some(SlotAction::MoveDown(i)) => stack.slots.swap(i, i + 1),
        None => {}
    }

    ui.add_space(4.0);
    ui.horizontal(|ui| {
        ui.add_enabled_ui(stack.can_add(), |ui| {
            egui::ComboBox::from_id_salt("add_formula")
                .selected_text("+ add formula")
                .width(150.0)
                .show_ui(ui, |ui| {
                    formula_picker(ui, filter, |kind| {
                        stack.slots.push(FormulaSlot::new(kind));
                    });
                });
        });
        if !stack.can_add() {
            ui.label(egui::RichText::new("slots full").small().weak());
        }
    });

    ui.add_space(4.0);
    ui.horizontal(|ui| {
        ui.label("estimator");
        let auto = stack.de_mode_override.is_none();
        let current = stack.de_mode();
        egui::ComboBox::from_id_salt("de_mode")
            .selected_text(if auto {
                format!("auto ({})", current.label())
            } else {
                current.label().to_owned()
            })
            .width(140.0)
            .show_ui(ui, |ui| {
                if ui.selectable_label(auto, "auto").clicked() {
                    stack.de_mode_override = None;
                }
                for mode in DeMode::ALL {
                    if ui
                        .selectable_label(stack.de_mode_override == Some(mode), mode.label())
                        .clicked()
                    {
                        stack.de_mode_override = Some(mode);
                    }
                }
            });
    })
    .response
    .on_hover_text(
        "Scaling formulas like the Mandelbox need a linear distance estimate; \
         escape-time formulas need the logarithmic one. Auto picks for you.",
    );
}

#[cfg(test)]
mod nav_tests {
    use super::*;

    fn nav() -> Nav {
        Nav {
            speed: 1.0,
            clearance: f32::INFINITY,
            ..Default::default()
        }
    }

    #[test]
    fn nothing_held_moves_nothing() {
        let mut camera = Camera::default();
        let before = camera;
        assert!(!nav().apply(&mut camera, 0.016));
        assert_eq!(camera, before);
    }

    #[test]
    fn w_flies_forward_rather_than_shrinking_the_orbit() {
        // Shrinking the orbit radius converges on the target, which starts at
        // the origin — inside the solid. Flying carries the target along, so
        // the descent has no floor but the precision of the numbers.
        let mut camera = Camera::default();
        let (_, _, forward) = camera.basis();
        let start = camera.target;
        let radius = camera.distance;

        Nav { forward: 1.0, ..nav() }.apply(&mut camera, 0.1);

        let travelled: f32 = (0..3)
            .map(|i| (camera.target[i] - start[i]) * forward[i])
            .sum();
        assert!(travelled > 0.0, "W should move along the view direction");
        assert!(
            (camera.distance - radius).abs() < 1e-6,
            "flying must not change the orbit radius"
        );

        Nav { forward: -1.0, ..nav() }.apply(&mut camera, 0.1);
        let back: f32 = (0..3)
            .map(|i| (camera.target[i] - start[i]) * forward[i])
            .sum();
        assert!(back.abs() < 1e-5, "S should undo W");
    }

    #[test]
    fn arrows_orbit_the_way_the_matching_drag_does() {
        // Dragging right calls `orbit` with a negative yaw, so the right arrow
        // must too, or mouse and keyboard fight each other.
        let mut dragged = Camera::default();
        dragged.orbit(-10.0 * ORBIT_SENSITIVITY, 0.0);

        let mut keyed = Camera::default();
        Nav { yaw: 1.0, ..nav() }.apply(&mut keyed, 0.1);

        assert!(
            (dragged.yaw - Camera::default().yaw).is_sign_negative()
                == (keyed.yaw - Camera::default().yaw).is_sign_negative(),
            "right arrow and right drag should orbit the same way"
        );
    }

    #[test]
    fn d_moves_the_camera_to_its_own_right() {
        let mut camera = Camera::default();
        let (right, _, _) = camera.basis();
        let before = camera.target;

        Nav { strafe: 1.0, ..nav() }.apply(&mut camera, 0.1);

        let moved: f32 = (0..3).map(|i| (camera.target[i] - before[i]) * right[i]).sum();
        assert!(moved > 0.0, "D should move along the camera's right axis");
    }

    #[test]
    fn e_rises_and_q_falls() {
        let mut camera = Camera::default();
        let (_, up, _) = camera.basis();
        let before = camera.target;

        Nav { lift: 1.0, ..nav() }.apply(&mut camera, 0.1);
        let moved: f32 = (0..3).map(|i| (camera.target[i] - before[i]) * up[i]).sum();
        assert!(moved > 0.0, "E should rise");
    }

    #[test]
    fn motion_does_not_depend_on_the_frame_rate() {
        // Two frames at 30fps must land where one frame at 15fps does, or the
        // fractal's own frame rate would change how far a key press travels.
        //
        // Tested one axis at a time on purpose. Flying uses the basis as it is
        // *now*, so turning while moving traces an arc, and an arc taken in two
        // steps is legitimately not the arc taken in one. That is real
        // behaviour, not drift.
        let travel = |nav: Nav, frames: u32| {
            let mut camera = Camera::default();
            for _ in 0..frames {
                nav.apply(&mut camera, 0.064 / frames as f32);
            }
            camera
        };

        // Retreat, which is linear in time and so composes exactly. Approach
        // is exponential in the *measured* gap, and that gap shrinks as the
        // camera moves, so it is covered by the convergence test instead.
        let (one, two) = (
            travel(Nav { forward: -1.0, ..nav() }, 1),
            travel(Nav { forward: -1.0, ..nav() }, 2),
        );
        for i in 0..3 {
            assert!((one.target[i] - two.target[i]).abs() < 1e-5, "{one:?} {two:?}");
        }

        let (one, two) = (
            travel(Nav { yaw: 1.0, ..nav() }, 1),
            travel(Nav { yaw: 1.0, ..nav() }, 2),
        );
        assert!((one.yaw - two.yaw).abs() < 1e-5, "{one:?} {two:?}");
    }

    #[test]
    fn flying_never_crosses_the_measured_surface() {
        // The whole point of the probe. With a wall 0.4 units ahead, no amount
        // of holding W may put the camera through it — each step takes at most
        // half the remaining gap, so the approach is asymptotic.
        let mut camera = Camera::default();
        let (_, _, forward) = camera.basis();
        let start = camera.target;
        let mut clearance: f32 = 0.4;

        for _ in 0..400 {
            Nav { forward: 1.0, clearance, ..nav() }.apply(&mut camera, 0.05);
            let travelled: f32 = (0..3)
                .map(|i| (camera.target[i] - start[i]) * forward[i])
                .sum();
            clearance = 0.4 - travelled;
            // Never negative is the guarantee. The gap does eventually reach
            // exactly zero, because halving a f32 four hundred times runs out
            // of exponent — that is underflow, not a crossing.
            assert!(clearance >= 0.0, "flew through the surface: {travelled}");
        }

        // And it closed on the wall rather than stalling short of it.
        assert!(clearance < 1e-6, "should have converged onto the surface");
    }

    #[test]
    fn backing_away_is_not_capped_by_the_clearance() {
        // Clearance measures what is in front. Reversing has nothing to hit,
        // so a near-zero reading must not pin the camera in place.
        let mut camera = Camera::default();
        let (_, _, forward) = camera.basis();
        let start = camera.target;

        Nav { forward: -1.0, clearance: 0.0, ..nav() }.apply(&mut camera, 0.1);

        let travelled: f32 = (0..3)
            .map(|i| (camera.target[i] - start[i]) * forward[i])
            .sum();
        assert!(travelled < 0.0, "S should still reverse when boxed in");
    }

    #[test]
    fn flight_still_works_before_the_probe_reports() {
        // The first frames have no reading. Flight must not stall waiting for
        // one — a probe that never reported would otherwise freeze the camera.
        let mut camera = Camera::default();
        let (_, _, forward) = camera.basis();

        Nav { forward: 1.0, ..nav() }.apply(&mut camera, 0.1);

        let travelled: f32 = (0..3).map(|i| camera.target[i] * forward[i]).sum();
        assert!(travelled > 0.0, "should move without a measurement");
    }

    #[test]
    fn open_space_is_crossed_faster_than_a_crevice() {
        // The point of scaling by the measurement rather than the orbit radius:
        // with room ahead the camera covers ground, and it slows only where
        // something is actually close.
        let step = |clearance: f32| {
            let mut camera = Camera::default();
            let (_, _, forward) = camera.basis();
            Nav { forward: 1.0, clearance, ..nav() }.apply(&mut camera, 0.05);
            (0..3)
                .map(|i| camera.target[i] * forward[i])
                .sum::<f32>()
        };
        assert!(step(4.0) > step(0.4) * 5.0, "open space should be much quicker");
    }

    #[test]
    fn shift_hurries_and_alt_creeps() {
        let travel = |speed: f32| {
            let mut camera = Camera::default();
            let (_, _, forward) = camera.basis();
            Nav { forward: 1.0, speed, ..nav() }.apply(&mut camera, 0.1);
            (0..3).map(|i| camera.target[i] * forward[i]).sum::<f32>()
        };
        assert!(travel(3.0) > travel(1.0));
        assert!(travel(0.3) < travel(1.0));
    }
}
