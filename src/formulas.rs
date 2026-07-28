//! The formula stack — 3DM's take on MB3D's hybrid fractals.
//!
//! A fractal here is not one formula but an ordered stack of them, each applied
//! in turn inside a single escape-time iteration and each active only over a
//! chosen range of iterations. Chaining a Mandelbox into a Mandelbulb, or
//! rotating between folds, is where the shapes MB3D is known for come from.
//!
//! Each formula contributes a WGSL fragment that transforms the running point
//! `z` and the running derivative `dr`. [`crate::render::codegen`] splices those
//! fragments into one straight-line distance estimator, so there is no
//! per-iteration branching on the GPU.

pub mod generated;

/// Type of a transpiled formula's parameter, mirroring Mandelbulber's own.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ParamKind {
    Float,
    Float3,
    Float4,
    Int,
    Bool,
    Matrix33,
}

impl ParamKind {
    /// f32 slots this occupies in the parameter pool.
    pub fn floats(self) -> usize {
        match self {
            Self::Float | Self::Int | Self::Bool => 1,
            Self::Float3 | Self::Float4 => 4,
            Self::Matrix33 => 12,
        }
    }
}

/// One parameter of a transpiled formula.
#[derive(Debug)]
pub struct GeneratedParam {
    /// Mandelbulber's own path, e.g. `mandelbox.foldingLimit`. Kept because it
    /// is the only human-readable name we have for the parameter.
    pub path: &'static str,
    pub kind: ParamKind,
    /// Offset in f32 units within this formula's parameter block.
    pub offset: usize,
    /// Default recovered from Mandelbulber's sources. A formula run with zeroed
    /// parameters usually renders nothing at all, so these matter.
    pub default: &'static [f32],
}

/// A formula translated from Mandelbulber2 by `tools/mb2-transpile`.
#[derive(Debug)]
pub struct GeneratedFormula {
    pub name: &'static str,
    /// Originating `.cl` file, for tracing a shape back to its source.
    pub source: &'static str,
    pub param_floats: usize,
    pub params: &'static [GeneratedParam],
    /// WGSL body with `__MB2Pn__` placeholders for parameter reads.
    pub wgsl: &'static str,
}

/// Formula slots available in the stack. MB3D allows six; more than that is
/// rarely legible and every slot costs shader instructions.
pub const MAX_SLOTS: usize = 6;

/// Tunable parameters exposed per slot.
pub const MAX_PARAMS: usize = 6;

/// `vec4`s of uniform storage per slot: params 0-3, then params 4-5 plus the
/// iteration range. Keeping the range in a uniform rather than in generated
/// code means dragging it does not trigger a shader rebuild.
pub const VEC4S_PER_SLOT: usize = 2;

/// How a formula contributes to the distance estimate.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DeKind {
    /// Escape-time formulas (`z -> z^n + c`), estimated with `0.5·log(r)·r/dr`.
    Escape,
    /// Scaling/folding formulas whose estimate is linear: `r/dr`.
    Linear,
    /// Isometries. They move `z` without changing `dr`, so they do not
    /// constrain which estimator the stack must use.
    Transform,
}

/// Which closed form turns the running `(r, dr)` into a distance.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum DeMode {
    /// `0.5·log(r)·r/dr` — correct for escape-time formulas.
    #[default]
    Log,
    /// `r/dr` — correct for scaling/folding formulas like the Mandelbox.
    Linear,
}

impl DeMode {
    pub fn label(self) -> &'static str {
        match self {
            Self::Log => "logarithmic",
            Self::Linear => "linear",
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ParamSpec {
    pub name: &'static str,
    pub min: f32,
    pub max: f32,
    pub default: f32,
}

const fn p(name: &'static str, min: f32, max: f32, default: f32) -> ParamSpec {
    ParamSpec {
        name,
        min,
        max,
        default,
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum FormulaKind {
    Mandelbulb,
    Juliabulb,
    Mandelbox,
    Sierpinski,
    Rotate,
}

impl FormulaKind {
    pub const ALL: [Self; 5] = [
        Self::Mandelbulb,
        Self::Juliabulb,
        Self::Mandelbox,
        Self::Sierpinski,
        Self::Rotate,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Self::Mandelbulb => "Mandelbulb",
            Self::Juliabulb => "Juliabulb",
            Self::Mandelbox => "Mandelbox",
            Self::Sierpinski => "Sierpinski",
            Self::Rotate => "Rotate",
        }
    }

    pub fn de_kind(self) -> DeKind {
        match self {
            Self::Mandelbulb | Self::Juliabulb => DeKind::Escape,
            Self::Mandelbox | Self::Sierpinski => DeKind::Linear,
            Self::Rotate => DeKind::Transform,
        }
    }

    /// Radius of a sphere that safely contains this formula's attractor.
    /// The raymarcher skips everything outside it, so it must not be too small
    /// — too large only costs a few wasted steps.
    pub fn suggested_bounds(self) -> f32 {
        match self {
            Self::Mandelbulb | Self::Juliabulb => 1.5,
            // The Mandelbox's scale parameter pushes the set far out; 6 covers
            // the usual scale range comfortably.
            Self::Mandelbox => 6.0,
            Self::Sierpinski => 2.0,
            Self::Rotate => 0.0,
        }
    }

    /// Escape radius that lets this formula's orbit develop before bailing.
    pub fn suggested_bailout(self) -> f32 {
        match self {
            Self::Mandelbulb | Self::Juliabulb => 4.0,
            Self::Mandelbox => 16.0,
            Self::Sierpinski => 8.0,
            Self::Rotate => 0.0,
        }
    }

    pub fn params(self) -> &'static [ParamSpec] {
        // Named constants rather than inline slices: a `&[...]` built from
        // function calls is a temporary and cannot outlive the match arm.
        const MANDELBULB: [ParamSpec; 1] = [p("power", 1.5, 16.0, 8.0)];
        const JULIABULB: [ParamSpec; 4] = [
            p("power", 1.5, 16.0, 8.0),
            p("c.x", -2.0, 2.0, 0.4),
            p("c.y", -2.0, 2.0, 0.2),
            p("c.z", -2.0, 2.0, -0.3),
        ];
        const MANDELBOX: [ParamSpec; 3] = [
            p("scale", -3.0, 3.0, 2.0),
            p("min radius", 0.05, 1.5, 0.5),
            p("fold limit", 0.2, 2.0, 1.0),
        ];
        const SIERPINSKI: [ParamSpec; 2] =
            [p("scale", 1.05, 3.0, 2.0), p("offset", 0.0, 2.0, 1.0)];
        const ROTATE: [ParamSpec; 3] = [
            p("pitch", -3.15, 3.15, 0.0),
            p("yaw", -3.15, 3.15, 0.3),
            p("roll", -3.15, 3.15, 0.0),
        ];

        match self {
            Self::Mandelbulb => &MANDELBULB,
            Self::Juliabulb => &JULIABULB,
            Self::Mandelbox => &MANDELBOX,
            Self::Sierpinski => &SIERPINSKI,
            Self::Rotate => &ROTATE,
        }
    }

    /// The WGSL body for this formula.
    ///
    /// `z` and `dr` are function pointers, `c` is the point being sampled (the
    /// `+ c` of the escape-time iteration). `P0`..`P5` are placeholders that
    /// codegen rewrites into this slot's uniform reads.
    pub fn wgsl_body(self) -> &'static str {
        match self {
            Self::Mandelbulb => {
                r#"
    let r = length(*z);
    if (r > 1e-9) {
        let power = P0;
        let theta = acos(clamp((*z).z / r, -1.0, 1.0)) * power;
        let phi = atan2((*z).y, (*z).x) * power;
        let zr = pow(r, power);
        *dr = pow(r, power - 1.0) * power * (*dr) + 1.0;
        let st = sin(theta);
        *z = zr * vec3<f32>(st * cos(phi), st * sin(phi), cos(theta)) + c;
    }
"#
            }
            // Same iteration as the Mandelbulb, but the added constant is fixed
            // instead of being the sample point — the 3D Julia set of the bulb.
            Self::Juliabulb => {
                r#"
    let r = length(*z);
    if (r > 1e-9) {
        let power = P0;
        let theta = acos(clamp((*z).z / r, -1.0, 1.0)) * power;
        let phi = atan2((*z).y, (*z).x) * power;
        let zr = pow(r, power);
        *dr = pow(r, power - 1.0) * power * (*dr) + 1.0;
        let st = sin(theta);
        *z = zr * vec3<f32>(st * cos(phi), st * sin(phi), cos(theta))
             + vec3<f32>(P1, P2, P3);
    }
"#
            }
            // Box fold, then sphere fold, then scale — Tglad's Mandelbox.
            Self::Mandelbox => {
                r#"
    let scale = P0;
    let minr = max(P1, 0.01);
    let lim = P2;

    *z = clamp(*z, vec3<f32>(-lim), vec3<f32>(lim)) * 2.0 - *z;

    let r2 = dot(*z, *z);
    let minr2 = minr * minr;
    if (r2 < minr2) {
        let f = 1.0 / minr2;
        *z = *z * f;
        *dr = *dr * f;
    } else if (r2 < 1.0) {
        let f = 1.0 / max(r2, 1e-9);
        *z = *z * f;
        *dr = *dr * f;
    }

    *z = *z * scale + c;
    *dr = *dr * abs(scale) + 1.0;
"#
            }
            // Kaleidoscopic IFS: fold into a tetrahedral wedge, then scale out.
            Self::Sierpinski => {
                r#"
    let s = P0;
    let o = P1;

    if ((*z).x + (*z).y < 0.0) { *z = vec3<f32>(-(*z).y, -(*z).x, (*z).z); }
    if ((*z).x + (*z).z < 0.0) { *z = vec3<f32>(-(*z).z, (*z).y, -(*z).x); }
    if ((*z).y + (*z).z < 0.0) { *z = vec3<f32>((*z).x, -(*z).z, -(*z).y); }

    *z = *z * s - vec3<f32>(o) * (s - 1.0);
    *dr = *dr * abs(s);
"#
            }
            // A rigid rotation. `dr` is untouched because rotations preserve
            // length, which is exactly what makes them safe to interleave.
            Self::Rotate => {
                r#"
    let cx = cos(P0); let sx = sin(P0);
    let cy = cos(P1); let sy = sin(P1);
    let cz = cos(P2); let sz = sin(P2);

    let rx = mat3x3<f32>(
        vec3<f32>(1.0, 0.0, 0.0),
        vec3<f32>(0.0, cx, sx),
        vec3<f32>(0.0, -sx, cx));
    let ry = mat3x3<f32>(
        vec3<f32>(cy, 0.0, -sy),
        vec3<f32>(0.0, 1.0, 0.0),
        vec3<f32>(sy, 0.0, cy));
    let rz = mat3x3<f32>(
        vec3<f32>(cz, sz, 0.0),
        vec3<f32>(-sz, cz, 0.0),
        vec3<f32>(0.0, 0.0, 1.0));

    *z = rz * (ry * (rx * (*z)));
"#
            }
        }
    }
}

/// One entry in the stack.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct FormulaSlot {
    pub kind: FormulaKind,
    pub enabled: bool,
    pub params: [f32; MAX_PARAMS],
    /// First iteration (inclusive) at which this formula applies.
    pub start_iter: i32,
    /// Last iteration (exclusive). Restricting a formula to early or late
    /// iterations is what makes a hybrid look like two fractals fused rather
    /// than one muddy average of both.
    pub end_iter: i32,
}

impl FormulaSlot {
    pub fn new(kind: FormulaKind) -> Self {
        let mut params = [0.0; MAX_PARAMS];
        for (slot, spec) in params.iter_mut().zip(kind.params()) {
            *slot = spec.default;
        }
        Self {
            kind,
            enabled: true,
            params,
            start_iter: 0,
            end_iter: 32,
        }
    }
}

/// The ordered stack, plus how to turn the result into a distance.
#[derive(Clone, PartialEq, Debug)]
pub struct FormulaStack {
    pub slots: Vec<FormulaSlot>,
    /// `None` picks an estimator from the formulas present, which is right
    /// almost always; `Some` lets you override when a hybrid disagrees.
    pub de_mode_override: Option<DeMode>,
}

impl Default for FormulaStack {
    fn default() -> Self {
        Self {
            slots: vec![FormulaSlot::new(FormulaKind::Mandelbulb)],
            de_mode_override: None,
        }
    }
}

impl FormulaStack {
    /// Slots that actually contribute, in order.
    pub fn active(&self) -> impl Iterator<Item = (usize, &FormulaSlot)> {
        self.slots
            .iter()
            .enumerate()
            .filter(|(_, s)| s.enabled)
            .take(MAX_SLOTS)
    }

    /// A linear formula anywhere in the stack makes the logarithmic estimate
    /// wrong — its `dr` grows by scaling rather than by differentiation — so
    /// the linear estimator wins unless the user says otherwise.
    pub fn de_mode(&self) -> DeMode {
        if let Some(mode) = self.de_mode_override {
            return mode;
        }
        if self
            .active()
            .any(|(_, s)| s.kind.de_kind() == DeKind::Linear)
        {
            DeMode::Linear
        } else {
            DeMode::Log
        }
    }

    /// Identifies the *shape* of the generated shader. Parameter values and
    /// iteration ranges are deliberately excluded: they live in uniforms, so
    /// changing them must not invalidate the compiled pipeline.
    pub fn signature(&self) -> u64 {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        for (index, slot) in self.active() {
            index.hash(&mut hasher);
            slot.kind.hash(&mut hasher);
        }
        self.de_mode().label().hash(&mut hasher);
        hasher.finish()
    }

    /// Bounding radius wide enough for every formula in the stack.
    pub fn suggested_bounds(&self) -> f32 {
        self.active()
            .map(|(_, s)| s.kind.suggested_bounds())
            .fold(1.5, f32::max)
    }

    /// Bailout large enough for every formula in the stack.
    pub fn suggested_bailout(&self) -> f32 {
        self.active()
            .map(|(_, s)| s.kind.suggested_bailout())
            .fold(4.0, f32::max)
    }

    pub fn can_add(&self) -> bool {
        self.slots.len() < MAX_SLOTS
    }
}
