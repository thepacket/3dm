//! Application state and UI.

use std::sync::Arc;
use std::time::Duration;

use crate::camera::Camera;
use crate::formulas::{Builtin, DeMode, FormulaKind, FormulaSlot, generated::GENERATED};
use crate::params::{DebugView, SceneParams};
use crate::render::{FractalPipeline, Uniforms, callback};

/// Radians of orbit per point of mouse travel.
const ORBIT_SENSITIVITY: f32 = 0.007;

/// Keyboard navigation rates, all per second so that holding a key moves the
/// same distance regardless of frame rate — which matters here, where the frame
/// rate depends on the fractal.
const KEY_ORBIT: f32 = 1.6;
/// Orbit radii of forward travel per second. Flying moves the orbit target
/// too, so this descends into the fractal rather than converging on its centre
/// — see [`Camera::advance`].
const KEY_FLY: f32 = 0.9;
/// Screen heights of pan per second.
const KEY_PAN: f32 = 0.9;

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

            ui.add_space(8.0);
            ui.separator();
            reset_all = ui.button("Reset everything").clicked();
            ui.add_space(8.0);
            ui.label(
                egui::RichText::new(
                    "drag to orbit · scroll to zoom · right-drag or shift-drag to pan\n\
                         WASD to move · QE up/down · arrows to orbit · shift faster, alt slower",
                )
                .small()
                .weak(),
            );
        });

        if reset_view || reset_all {
            self.camera = Camera::default();
        }
        if reset_all {
            self.params = SceneParams::default();
        }
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

        if response.hovered() {
            let scroll = ui.input(|i| i.smooth_scroll_delta.y);
            if scroll != 0.0 {
                self.camera.zoom((-scroll * 0.002).exp());
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
        );
        ui.painter().add(callback(
            rect,
            uniforms,
            self.shader.0,
            Arc::clone(&self.shader.1),
        ));
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
            camera.advance(self.forward * KEY_FLY * step);
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

        let (one, two) = (
            travel(Nav { forward: 1.0, ..nav() }, 1),
            travel(Nav { forward: 1.0, ..nav() }, 2),
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
