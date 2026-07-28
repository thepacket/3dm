//! Scene parameters: the fractal formula, the raymarcher, and the shading.
//!
//! Everything the GPU needs lives here as plain data so that a future preset /
//! project-file system can serialize it without touching the renderer.

/// Which intermediate quantity to display instead of the shaded image.
///
/// Every shading term here is a product of several others, so inferring one
/// from the final picture is guesswork. Rendering a term directly is the only
/// way to tell "this surface is unlit" from "this surface is black".
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum DebugView {
    #[default]
    Off,
    /// Surface normal, remapped to RGB. Noise here means a bad hit point.
    Normal,
    /// The soft-shadow term alone, white = fully lit.
    Shadow,
    /// Ambient occlusion alone.
    Occlusion,
    /// `max(dot(n, l), 0)` alone, before shadowing.
    Diffuse,
    /// Sphere-tracing steps used, as a heat ramp. Bright = the marcher
    /// struggled, which is where distance estimates are worst.
    Steps,
    /// The orbit trap that drives the palette.
    Trap,
}

impl DebugView {
    pub const ALL: [Self; 7] = [
        Self::Off,
        Self::Normal,
        Self::Shadow,
        Self::Occlusion,
        Self::Diffuse,
        Self::Steps,
        Self::Trap,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Self::Off => "off",
            Self::Normal => "normal",
            Self::Shadow => "shadow",
            Self::Occlusion => "occlusion",
            Self::Diffuse => "diffuse",
            Self::Steps => "steps",
            Self::Trap => "orbit trap",
        }
    }
}

/// Iteration settings shared by every formula in the stack. Shape parameters
/// live on the individual formulas — see [`crate::formulas`].
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct FractalParams {
    /// Iterations of the escape-time loop. Higher reveals finer detail.
    pub iterations: i32,
    /// Escape radius. Points that exceed this are considered outside.
    pub bailout: f32,
    /// Radius of the sphere the raymarcher treats as containing the fractal.
    /// Too small silently clips the shape; too large only wastes a few steps.
    pub bounding_radius: f32,
    /// Scales the distance estimate. Below 1.0 the marcher takes more,
    /// shorter steps — slower but fixes overstepping artifacts.
    pub de_scale: f32,
}

impl Default for FractalParams {
    fn default() -> Self {
        Self {
            iterations: 12,
            bailout: 4.0,
            bounding_radius: 1.5,
            de_scale: 0.9,
        }
    }
}

/// Parameters of the sphere-tracing loop.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct MarchParams {
    /// Hard cap on sphere-tracing steps per ray.
    pub max_steps: i32,
    /// Distance at which a ray is declared to have escaped to the background.
    pub max_dist: f32,
    /// Multiplier on the pixel-sized hit threshold. Below 1.0 = sharper and
    /// slower, above = softer and faster.
    pub epsilon_scale: f32,
}

impl Default for MarchParams {
    fn default() -> Self {
        Self {
            max_steps: 160,
            max_dist: 12.0,
            epsilon_scale: 1.0,
        }
    }
}

/// Lighting, ambient occlusion and colour.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct ShadingParams {
    /// Direction the key light travels *from*, in world space.
    pub light_dir: [f32; 3],
    /// Fraction of skylight applied where nothing else lights the surface.
    pub ambient: f32,
    /// Strength of the ambient-occlusion darkening in creases.
    pub ao_strength: f32,
    /// Penumbra width of the ray-marched shadow. Low = hard, high = soft.
    pub shadow_softness: f32,
    /// Specular highlight intensity.
    pub specular: f32,
    /// Additive glow proportional to how much the ray struggled to converge.
    pub glow: f32,
    /// Base colour of the cosine palette driven by the orbit trap.
    pub palette_base: [f32; 3],
    /// Amplitude of the cosine palette.
    pub palette_amp: [f32; 3],
    /// Phase of the cosine palette — the main "hue rotation" knob.
    pub palette_phase: f32,
    /// How strongly the orbit trap modulates colour.
    pub palette_freq: f32,
    /// Background colour behind the fractal.
    pub background: [f32; 3],
    /// Distance fade toward the background colour.
    pub fog: f32,
}

impl Default for ShadingParams {
    fn default() -> Self {
        Self {
            light_dir: [0.6, 0.7, -0.4],
            ambient: 0.45,
            ao_strength: 0.85,
            shadow_softness: 16.0,
            specular: 0.35,
            glow: 0.30,
            // Note: all colours here are *linear*, not sRGB — a value of 0.005
            // is a near-black background once the display transfer curve is
            // applied, not a dark grey.
            palette_base: [0.50, 0.46, 0.42],
            palette_amp: [0.45, 0.42, 0.40],
            palette_phase: 0.0,
            palette_freq: 1.0,
            background: [0.004, 0.006, 0.011],
            fog: 0.10,
        }
    }
}

/// The complete scene.
#[derive(Clone, PartialEq, Debug, Default)]
pub struct SceneParams {
    /// Iteration budget and bailout shared by every formula in the stack.
    pub fractal: FractalParams,
    /// The hybrid formula stack that defines the shape.
    pub stack: crate::formulas::FormulaStack,
    pub march: MarchParams,
    pub shading: ShadingParams,
    /// Diagnostic override; [`DebugView::Off`] renders normally.
    pub debug_view: DebugView,
}

impl SceneParams {
    /// Retune the settings that depend on which formulas are in the stack.
    ///
    /// A Mandelbox needs a far wider bounding sphere and a larger bailout than a
    /// Mandelbulb; leaving the Mandelbulb's values in place puts the camera
    /// inside the solid and renders noise. Called when the stack's structure
    /// changes — the user is free to edit the results afterwards.
    pub fn retune_for_stack(&mut self) {
        self.fractal.bounding_radius = self.stack.suggested_bounds();
        self.fractal.bailout = self.stack.suggested_bailout();
        self.march.max_dist = (self.fractal.bounding_radius * 6.0).max(12.0);
    }

    /// Convenience for the many places that need both.
    pub fn shader(&self) -> (u64, String) {
        (
            self.stack.signature(),
            crate::render::codegen::generate(&self.stack),
        )
    }
}
