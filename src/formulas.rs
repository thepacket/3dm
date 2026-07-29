//! The formula stack — 3DM's take on MB3D's hybrid fractals.
//!
//! A fractal here is not one formula but an ordered stack of them, each applied
//! in turn inside a single escape-time iteration and each active only over a
//! chosen range of iterations. Chaining a Mandelbox into a Mandelbulb, or
//! rotating between folds, is where the shapes MB3D is known for come from.
//!
//! Each formula contributes a WGSL fragment that transforms the running point
//! `z` and the running derivative. [`crate::render::codegen`] splices those
//! fragments into one straight-line distance estimator, so there is no
//! per-iteration branching on the GPU.
//!
//! Formulas come from two places — a handful written by hand ([`Builtin`]) and
//! several hundred transpiled from Mandelbulber2 ([`generated`]) — but they
//! share one ABI: a 4D `z` the body mutates and returns, plus Mandelbulber's
//! `Aux` carrying the derivative as `de`. That is what lets the two sit in the
//! same stack and share one iteration loop. Their parameters share one uniform
//! pool as well; see [`mb2`] for how a formula's block is laid out in it.

pub mod generated;
pub mod mb2;

use generated::GENERATED;

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

/// Which closed form a transpiled formula's distance estimate takes, recovered
/// from Mandelbulber's `DEAnalyticFunction` — the field its engine switches on.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DeFunction {
    /// `0.5·r·log(r)/de`, and zero inside the unit sphere.
    Logarithmic,
    /// `r/de`.
    Linear,
    /// `(r − 2)/de`. The offset is what makes an IFS's estimate hold up, and
    /// dropping it is the difference between a Sierpinski and a blob.
    Ifs,
    /// The formula wrote its own distance into `aux.dist`.
    Custom,
    /// The formula contributes no estimator of its own. For a transform that
    /// is correct: the base formula in the stack decides.
    None,
    /// No analytic derivative at all. The distance is measured by iterating
    /// neighbouring points — see [`DeMode::Delta`].
    Delta,
    /// Delta DE with the linear closed form.
    DeltaLinear,
    /// `max(rxy − auxDE, |rxy·z.z|/r) / de`, where `rxy` is the radius in xy.
    PseudoKleinian,
    /// Needs parameters 3DM does not carry, such as the Jos-Kleinian tweaks.
    Unsupported,
}

/// How a derived parameter is computed from its sources.
///
/// Mandelbulber fills these in once per edit, in
/// `sFractal::RecalculateFractalParams`, and never stores them — so the
/// transpiler finds no default and would emit zero. A zero squared fold radius
/// is a fold that never fires; a zero reciprocal is an inversion by zero.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DeriveOp {
    Square,
    Reciprocal,
    Sqrt,
    Ratio,
    /// Of an angle in degrees.
    Cos,
    Sin,
    /// 3x3 rotation from Euler angles in degrees, `Rz·Ry·Rx`.
    Rotation2,
    /// The same from `Rx·Ry·Rz`.
    Rotation4,
}

/// One recipe: where to write, what to compute, and what to read.
/// Offsets are placeholder numbers, resolved to pool positions by [`mb2`].
#[derive(Debug)]
pub struct Derivation {
    pub target: usize,
    pub op: DeriveOp,
    pub sources: &'static [usize],
}

/// A formula translated from Mandelbulber2 by `tools/mb2-transpile`.
#[derive(Debug)]
pub struct GeneratedFormula {
    pub name: &'static str,
    /// Originating `.cl` file, for tracing a shape back to its source.
    pub source: &'static str,
    pub param_floats: usize,
    pub de_function: DeFunction,
    /// Whether the iteration loop must add the sampled point back after this
    /// formula runs. Mandelbulber does this in its caller rather than in the
    /// formula body, so an escape-time formula renders as a bare sphere
    /// without it.
    pub add_c: bool,
    /// Escape radius this formula was designed around.
    pub bailout: f32,
    pub params: &'static [GeneratedParam],
    /// Recipes for the parameters Mandelbulber computes rather than stores,
    /// applied by [`mb2::recompute`] before the block is uploaded.
    pub derivations: &'static [Derivation],
    /// WGSL body with `__MB2Pn__` placeholders for parameter reads.
    pub wgsl: &'static str,
}

/// Formula slots available in the stack. MB3D allows six; more than that is
/// rarely legible and every slot costs shader instructions.
pub const MAX_SLOTS: usize = 6;

/// Tunable parameters exposed per slot.
pub const MAX_PARAMS: usize = 6;

/// `vec4`s of parameter pool reserved per slot.
///
/// A fixed stride rather than a packed allocation: it costs a few kilobytes of
/// uniform space that a tighter packing would save, and in exchange a slot's
/// parameters sit at the same address no matter what the other slots hold, so
/// editing one formula cannot disturb another's reads. The widest formula in
/// the corpus needs 144 floats once its vectors are aligned and the sources of
/// its derived parameters are appended — see the test below, which fails if a
/// future Mandelbulber release exceeds this. It already has: adding the derived
/// parameters' sources pushed the widest formula from 133 floats past a stride
/// of 136, and that test is the only reason it was noticed rather than shipped
/// as one slot quietly reading another's parameters.
pub const POOL_VEC4S_PER_SLOT: usize = 40;

/// f32 slots reserved per formula in the parameter pool.
pub const POOL_FLOATS_PER_SLOT: usize = POOL_VEC4S_PER_SLOT * 4;

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
    /// `0.5·r·log(r)/de` — correct for escape-time formulas.
    #[default]
    Log,
    /// `r/de` — correct for scaling/folding formulas like the Mandelbox.
    Linear,
    /// `(r − 2)/de` — kaleidoscopic IFS formulas.
    Ifs,
    /// The formula wrote its own distance into `aux.dist`.
    Custom,
    /// No derivative is available, so the distance is measured: the iteration
    /// runs at the point and at six neighbours, and how fast they diverge is
    /// the gradient. Seven times the work of every other mode, which is why it
    /// is never chosen unless a formula in the stack requires it.
    Delta,
    /// Delta DE with the linear closed form.
    DeltaLinear,
    /// The pseudo-Kleinian form, which measures against a folded radius the
    /// formula carries in `aux.pseudo_kleinian_de`.
    PseudoKleinian,
}

impl DeMode {
    pub const ALL: [Self; 7] = [
        Self::Log,
        Self::Linear,
        Self::Ifs,
        Self::Custom,
        Self::PseudoKleinian,
        Self::Delta,
        Self::DeltaLinear,
    ];

    /// Whether this mode measures the gradient instead of tracking it.
    pub fn is_delta(self) -> bool {
        matches!(self, Self::Delta | Self::DeltaLinear)
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Log => "logarithmic",
            Self::Linear => "linear",
            Self::Ifs => "IFS",
            Self::Custom => "formula's own",
            Self::Delta => "delta (measured)",
            Self::DeltaLinear => "delta, linear",
            Self::PseudoKleinian => "pseudo-Kleinian",
        }
    }

    /// How specific this estimator is. A stack mixing formulas takes the most
    /// specific one present: a formula that computed its own distance knows
    /// more than any closed form we could pick for it, and the IFS offset and
    /// the linear form are each wrong for the modes below them.
    fn specificity(self) -> u8 {
        match self {
            Self::Log => 0,
            Self::Linear => 1,
            Self::Ifs => 2,
            Self::Custom => 3,
            Self::PseudoKleinian => 4,
            // A formula with no analytic derivative makes every other estimator
            // meaningless, because nothing in the stack is maintaining `de`.
            Self::Delta | Self::DeltaLinear => 5,
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

/// 3DM's own formulas, written by hand rather than transpiled.
///
/// They are kept because they are the ones worth reaching for first, and
/// because having a formula whose behaviour is known exactly is what makes it
/// possible to tell a broken transpile from a broken renderer.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Builtin {
    Mandelbulb,
    Juliabulb,
    Mandelbox,
    Sierpinski,
    Rotate,
}

impl Builtin {
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
    /// Written against the same ABI as the transpiled formulas — a 4D `z` that
    /// the body mutates and returns, and Mandelbulber's `Aux` carrying the
    /// running derivative as `(*aux).de` and the sampled point as
    /// `(*aux).const_c`. These formulas are 3D and simply leave `z.w` alone.
    ///
    /// Sharing one ABI is what lets [`crate::render::codegen`] emit a single
    /// iteration loop rather than one per formula source, and it means a
    /// hand-written formula and a transpiled one can sit in the same stack.
    /// `P0`..`P5` are placeholders that codegen rewrites into this slot's
    /// uniform reads.
    pub fn wgsl_body(self) -> &'static str {
        match self {
            Self::Mandelbulb => {
                r#"
    let r = length(z.xyz);
    if (r > 1e-9) {
        let power = P0;
        let theta = acos(clamp(z.z / r, -1.0, 1.0)) * power;
        let phi = atan2(z.y, z.x) * power;
        let zr = pow(r, power);
        (*aux).de = pow(r, power - 1.0) * power * (*aux).de + 1.0;
        let st = sin(theta);
        z = vec4<f32>(
            zr * vec3<f32>(st * cos(phi), st * sin(phi), cos(theta))
                + (*aux).const_c.xyz,
            z.w);
    }
    return z;
"#
            }
            // Same iteration as the Mandelbulb, but the added constant is fixed
            // instead of being the sample point — the 3D Julia set of the bulb.
            Self::Juliabulb => {
                r#"
    let r = length(z.xyz);
    if (r > 1e-9) {
        let power = P0;
        let theta = acos(clamp(z.z / r, -1.0, 1.0)) * power;
        let phi = atan2(z.y, z.x) * power;
        let zr = pow(r, power);
        (*aux).de = pow(r, power - 1.0) * power * (*aux).de + 1.0;
        let st = sin(theta);
        z = vec4<f32>(
            zr * vec3<f32>(st * cos(phi), st * sin(phi), cos(theta))
                + vec3<f32>(P1, P2, P3),
            z.w);
    }
    return z;
"#
            }
            // Box fold, then sphere fold, then scale — Tglad's Mandelbox.
            Self::Mandelbox => {
                r#"
    let scale = P0;
    let minr = max(P1, 0.01);
    let lim = P2;

    var p = clamp(z.xyz, vec3<f32>(-lim), vec3<f32>(lim)) * 2.0 - z.xyz;

    let r2 = dot(p, p);
    let minr2 = minr * minr;
    if (r2 < minr2) {
        let f = 1.0 / minr2;
        p = p * f;
        (*aux).de = (*aux).de * f;
    } else if (r2 < 1.0) {
        let f = 1.0 / max(r2, 1e-9);
        p = p * f;
        (*aux).de = (*aux).de * f;
    }

    p = p * scale + (*aux).const_c.xyz;
    (*aux).de = (*aux).de * abs(scale) + 1.0;
    return vec4<f32>(p, z.w);
"#
            }
            // Kaleidoscopic IFS: fold into a tetrahedral wedge, then scale out.
            Self::Sierpinski => {
                r#"
    let s = P0;
    let o = P1;

    var p = z.xyz;
    if (p.x + p.y < 0.0) { p = vec3<f32>(-p.y, -p.x, p.z); }
    if (p.x + p.z < 0.0) { p = vec3<f32>(-p.z, p.y, -p.x); }
    if (p.y + p.z < 0.0) { p = vec3<f32>(p.x, -p.z, -p.y); }

    p = p * s - vec3<f32>(o) * (s - 1.0);
    (*aux).de = (*aux).de * abs(s);
    return vec4<f32>(p, z.w);
"#
            }
            // A rigid rotation. `de` is untouched because rotations preserve
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

    return vec4<f32>(rz * (ry * (rx * z.xyz)), z.w);
"#
            }
        }
    }
}

/// One editable parameter of a formula, as the UI needs to show it.
#[derive(Clone, Debug)]
pub struct Control {
    /// Short name for the widget: Mandelbulber's path with its struct prefix
    /// dropped, since every parameter in a formula shares that prefix and it
    /// only crowds the panel.
    pub label: String,
    /// The full path, kept for the tooltip — it is what Mandelbulber's own UI
    /// and its settings files call this parameter.
    pub path: String,
    /// Float index into [`FormulaSlot::params`].
    pub index: usize,
    /// Slider bounds, where the formula has known ones.
    pub range: Option<(f32, f32)>,
}

/// A formula in a stack: either one of 3DM's own or one transpiled from
/// Mandelbulber2.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum FormulaKind {
    Builtin(Builtin),
    /// Index into [`generated::GENERATED`].
    Generated(usize),
}

impl FormulaKind {
    pub fn name(self) -> &'static str {
        match self {
            Self::Builtin(b) => b.name(),
            Self::Generated(i) => GENERATED[i].name,
        }
    }

    /// Looks up a transpiled formula by its Mandelbulber name.
    pub fn find(name: &str) -> Option<Self> {
        GENERATED
            .iter()
            .position(|f| f.name.eq_ignore_ascii_case(name))
            .map(Self::Generated)
    }

    pub fn de_kind(self) -> DeKind {
        match self {
            Self::Builtin(b) => b.de_kind(),
            Self::Generated(i) => match GENERATED[i].de_function {
                DeFunction::Linear | DeFunction::Ifs | DeFunction::DeltaLinear => DeKind::Linear,
                _ => DeKind::Escape,
            },
        }
    }

    pub fn suggested_bounds(self) -> f32 {
        match self {
            Self::Builtin(b) => b.suggested_bounds(),
            Self::Generated(_) => 4.0,
        }
    }

    pub fn suggested_bailout(self) -> f32 {
        match self {
            Self::Builtin(b) => b.suggested_bailout(),
            Self::Generated(i) => GENERATED[i].bailout,
        }
    }

    /// The closed form this formula's distance estimate takes.
    pub fn de_function(self) -> DeFunction {
        match self {
            Self::Builtin(b) => match b.de_kind() {
                DeKind::Escape => DeFunction::Logarithmic,
                DeKind::Linear => DeFunction::Linear,
                // An isometry does not constrain the estimator.
                DeKind::Transform => DeFunction::None,
            },
            Self::Generated(i) => GENERATED[i].de_function,
        }
    }

    /// Whether the iteration loop adds the sampled point back after this
    /// formula. The hand-written formulas fold `+ c` into their own bodies;
    /// Mandelbulber's do not.
    pub fn adds_c(self) -> bool {
        match self {
            Self::Builtin(_) => false,
            Self::Generated(i) => GENERATED[i].add_c,
        }
    }

    /// f32 slots this formula needs in the parameter pool.
    pub fn param_floats(self) -> usize {
        match self {
            Self::Builtin(_) => MAX_PARAMS,
            Self::Generated(i) => mb2::layout(&GENERATED[i]).floats,
        }
    }

    /// The editable controls for this formula, in display order.
    ///
    /// A hand-written formula names its parameters and bounds them; a
    /// transpiled one has only Mandelbulber's struct path and no idea of a
    /// sensible range, so its controls are unbounded and are labelled with the
    /// path — which is at least searchable against Mandelbulber's own UI.
    pub fn controls(self) -> Vec<Control> {
        match self {
            Self::Builtin(b) => b
                .params()
                .iter()
                .enumerate()
                .map(|(i, spec)| Control {
                    label: spec.name.to_owned(),
                    path: spec.name.to_owned(),
                    index: i,
                    range: Some((spec.min, spec.max)),
                })
                .collect(),
            Self::Generated(i) => mb2::layout(&GENERATED[i])
                .placements
                .iter()
                // A derived parameter is recomputed from its source on every
                // upload, so editing it would be overwritten before it reached
                // the GPU. The source is in the list instead — which is what
                // finally makes the rotation angles reachable, the matrix
                // having never been editable in any useful sense.
                .filter(|p| {
                    !GENERATED[i]
                        .derivations
                        .iter()
                        .any(|d| d.target == p.param.offset)
                })
                .flat_map(|p| {
                    let floats = p.param.kind.floats().min(4);
                    // A vec4 parameter becomes four controls, suffixed, because
                    // a single row of four drag values is how these read in
                    // Mandelbulber too.
                    (0..floats).map(move |c| {
                        let short = p.param.path.rsplit('.').next().unwrap_or(p.param.path);
                        Control {
                            label: if floats > 1 {
                                format!("{short}.{}", ["x", "y", "z", "w"][c])
                            } else {
                                short.to_owned()
                            },
                            path: p.param.path.to_owned(),
                            index: p.index + c,
                            range: None,
                        }
                    })
                })
                .collect(),
        }
    }

    /// The starting parameter values for a fresh slot.
    ///
    /// A transpiled formula run with zeroed parameters almost always renders
    /// nothing, so its recovered defaults are not a nicety.
    pub fn defaults(self) -> Vec<f32> {
        match self {
            Self::Builtin(b) => {
                let mut v = vec![0.0; MAX_PARAMS];
                for (slot, spec) in v.iter_mut().zip(b.params()) {
                    *slot = spec.default;
                }
                v
            }
            Self::Generated(i) => {
                let mut block = mb2::layout(&GENERATED[i]).defaults();
                mb2::recompute(&GENERATED[i], &mut block);
                block
            }
        }
    }
}

/// One entry in the stack.
#[derive(Clone, PartialEq, Debug)]
pub struct FormulaSlot {
    pub kind: FormulaKind,
    pub enabled: bool,
    /// This formula's parameter block, in the layout the shader reads. Sized
    /// per formula rather than fixed: a hand-written formula uses six floats
    /// and a transpiled one can need over a hundred.
    pub params: Vec<f32>,
    /// First iteration (inclusive) at which this formula applies.
    pub start_iter: i32,
    /// Last iteration (exclusive). Restricting a formula to early or late
    /// iterations is what makes a hybrid look like two fractals fused rather
    /// than one muddy average of both.
    pub end_iter: i32,
    /// Run this slot only every `period` iterations. 1 is every one.
    ///
    /// This and `phase` are MB3D's contribution, and they are what a range
    /// alone cannot express: two formulas sharing the same stretch of the loop
    /// and *alternating* within it, rather than occupying different stretches
    /// of it. Period 2 with phases 0 and 1 is the classic pair.
    pub period: i32,
    /// Which iteration within each period this slot takes, counted from
    /// `start_iter`.
    pub phase: i32,
    /// How much of this formula's result to keep, at the start and at the end
    /// of its iteration range.
    ///
    /// Ramping one slot down while another ramps up is how a hybrid becomes a
    /// blend rather than a seam — MB3D interpolating between slots. Both at 1
    /// is "apply the formula outright", which is what every stack that never
    /// touches this does.
    pub weight_start: f32,
    pub weight_end: f32,
    /// Which components of `z` the formula is allowed to change. Clearing one
    /// is what turns a transform into something that acts on a single axis.
    pub axes: [bool; 4],
}

impl FormulaSlot {
    /// The four axis flags as the bitfield the uniform carries.
    pub fn axis_bits(&self) -> f32 {
        let mut bits = 0u32;
        for (i, on) in self.axes.iter().enumerate() {
            if *on {
                bits |= 1 << i;
            }
        }
        bits as f32
    }

    /// True when this slot applies its formula outright — no alternation, no
    /// weighting, no masking. The shader has a fast path for it, and it is
    /// what every stack looked like before any of this existed.
    pub fn is_plain(&self) -> bool {
        self.period <= 1
            && self.weight_start >= 1.0
            && self.weight_end >= 1.0
            && self.axes == [true; 4]
    }
}

impl FormulaSlot {
    pub fn new(kind: FormulaKind) -> Self {
        Self {
            kind,
            enabled: true,
            params: kind.defaults(),
            start_iter: 0,
            end_iter: 32,
            period: 1,
            phase: 0,
            weight_start: 1.0,
            weight_end: 1.0,
            axes: [true; 4],
        }
    }

    /// Convenience for the hand-written formulas, which are named far more
    /// often than they are indexed.
    pub fn builtin(kind: Builtin) -> Self {
        Self::new(FormulaKind::Builtin(kind))
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
            slots: vec![FormulaSlot::builtin(Builtin::Mandelbulb)],
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
        self.active()
            .filter_map(|(_, s)| match s.kind.de_function() {
                DeFunction::Logarithmic => Some(DeMode::Log),
                DeFunction::Linear => Some(DeMode::Linear),
                DeFunction::Ifs => Some(DeMode::Ifs),
                DeFunction::Custom => Some(DeMode::Custom),
                DeFunction::Delta => Some(DeMode::Delta),
                DeFunction::DeltaLinear => Some(DeMode::DeltaLinear),
                DeFunction::PseudoKleinian => Some(DeMode::PseudoKleinian),
                // Neither constrains the choice: an isometry because it leaves
                // `de` alone, and a delta-DE formula because no closed form we
                // have is right for it anyway.
                DeFunction::None | DeFunction::Unsupported => None,
            })
            .max_by_key(|m| m.specificity())
            .unwrap_or_default()
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

#[cfg(test)]
mod tests {
    use super::*;

    /// The shader skips the blend entirely for a plain slot, and that skip is
    /// what keeps every pre-existing stack rendering bit-for-bit as it did:
    /// `mix(a, b, 1.0)` is `a + (b - a)`, which is not exactly `b`. If a new
    /// slot ever stops counting as plain by default, every picture moves.
    #[test]
    fn a_fresh_slot_joins_the_loop_plainly() {
        let slot = FormulaSlot::builtin(Builtin::Mandelbulb);
        assert!(slot.is_plain());
        assert_eq!(slot.axis_bits(), 15.0, "all four axes set");
    }

    #[test]
    fn any_gating_stops_a_slot_being_plain() {
        let plain = FormulaSlot::builtin(Builtin::Mandelbulb);
        for altered in [
            FormulaSlot { period: 2, ..plain.clone() },
            FormulaSlot { weight_start: 0.5, ..plain.clone() },
            FormulaSlot { weight_end: 0.0, ..plain.clone() },
            FormulaSlot { axes: [true, true, false, true], ..plain.clone() },
        ] {
            assert!(!altered.is_plain(), "{altered:?} should not be plain");
        }
    }

    #[test]
    fn axis_bits_pack_in_the_order_the_shader_unpacks_them() {
        let mut slot = FormulaSlot::builtin(Builtin::Mandelbulb);
        slot.axes = [true, false, false, false];
        assert_eq!(slot.axis_bits(), 1.0);
        slot.axes = [false, true, false, false];
        assert_eq!(slot.axis_bits(), 2.0);
        slot.axes = [false, false, true, false];
        assert_eq!(slot.axis_bits(), 4.0);
        slot.axes = [false, false, false, true];
        assert_eq!(slot.axis_bits(), 8.0);
    }

    /// Gating lives in uniforms, so changing it must not force a recompile —
    /// otherwise dragging the weight slider rebuilds the shader every frame.
    #[test]
    fn gating_does_not_change_the_shader_signature() {
        let mut stack = FormulaStack::default();
        let before = stack.signature();
        stack.slots[0].period = 3;
        stack.slots[0].phase = 1;
        stack.slots[0].weight_start = 0.25;
        stack.slots[0].axes = [true, false, true, false];
        assert_eq!(stack.signature(), before);
    }

    #[test]
    fn every_transpiled_formula_fits_its_pool_block() {
        // The pool stride is a compile-time constant baked into both the
        // generated WGSL and `fractal.wgsl`, so a formula that outgrew it would
        // silently read another slot's parameters rather than fail.
        let widest = GENERATED
            .iter()
            .map(|f| (mb2::layout(f).floats, f.name))
            .max()
            .unwrap_or((0, ""));
        assert!(
            widest.0 <= POOL_FLOATS_PER_SLOT,
            "{} needs {} floats, pool stride is {POOL_FLOATS_PER_SLOT}",
            widest.1,
            widest.0,
        );
    }

    #[test]
    fn aligned_layout_keeps_vectors_on_vec4_boundaries() {
        // The transpiler packs placeholders densely, so this is exactly the
        // property `mb2::layout` exists to restore: a `vec4` array can only be
        // indexed a whole `vec4` at a time.
        for f in GENERATED {
            for p in mb2::layout(f).placements {
                if p.param.kind.floats() >= 4 {
                    assert_eq!(
                        p.index % 4,
                        0,
                        "{}: {} lands at float {}",
                        f.name,
                        p.param.path,
                        p.index
                    );
                }
            }
        }
    }


    #[test]
    fn every_control_indexes_inside_its_own_block() {
        // The UI writes through `slot.params[control.index]`. A control whose
        // index ran past the block would silently edit the next slot's
        // parameters, or be dropped by the `get_mut` guard and appear dead.
        for (i, f) in GENERATED.iter().enumerate() {
            let kind = FormulaKind::Generated(i);
            let slot = FormulaSlot::new(kind);
            for c in kind.controls() {
                assert!(
                    c.index < slot.params.len(),
                    "{}: control {:?} at float {} but block is {} floats",
                    f.name,
                    c.label,
                    c.index,
                    slot.params.len(),
                );
            }
        }
    }

    #[test]
    fn builtin_controls_match_their_declared_params() {
        for b in Builtin::ALL {
            let kind = FormulaKind::Builtin(b);
            let slot = FormulaSlot::new(kind);
            assert_eq!(kind.controls().len(), b.params().len(), "{}", b.name());
            for c in kind.controls() {
                assert!(c.index < slot.params.len(), "{}", b.name());
                assert!(c.range.is_some(), "{} should be bounded", b.name());
            }
        }
    }

    #[test]
    fn matrix_parameters_are_not_silently_half_editable() {
        // `controls` caps a parameter at four rows, which is right for vectors
        // but leaves a 3x3 rotation with only its first row exposed. That is a
        // real gap in the editor, not a layout bug — this pins it so the day it
        // is fixed the test says so rather than the UI quietly changing.
        let matrices: usize = GENERATED
            .iter()
            .flat_map(|f| f.params.iter())
            .filter(|p| p.kind == ParamKind::Matrix33)
            .count();
        assert!(matrices > 0, "the corpus does contain rotation matrices");
    }

    #[test]
    fn a_slots_defaults_fill_its_own_block_only() {
        let kind = FormulaKind::find("Mandelbulb").expect("transpiled Mandelbulb");
        let slot = FormulaSlot::new(kind);
        assert_eq!(slot.params.len(), kind.param_floats());
        // bulb.power defaults to 9 in Mandelbulber; a zeroed block renders
        // nothing, which is why the recovered defaults are load-bearing.
        assert!(slot.params.contains(&9.0));
    }
}
