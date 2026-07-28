//! Placing a transpiled formula's parameters in the uniform pool.
//!
//! A hand-written formula has at most six scalars, which fit in a slot's two
//! `vec4`s. A transpiled one reaches 120 floats — Mandelbulber's formulas carry
//! whole rotation matrices and 4D offsets — so they read from a shared pool
//! instead, and each slot is handed a block within it.
//!
//! The `offset` recorded in [`generated`](super::generated) is *not* a pool
//! position: it is only the number in the `__MB2Pn__` placeholder, and the
//! transpiler packed those densely, leaving `Float3`/`Float4`/`Matrix33`
//! straddling `vec4` boundaries. A `vec4` array can only be indexed a `vec4` at
//! a time, so this module re-lays the parameters out with the vector kinds
//! aligned, and that aligned layout is what both the shader reads and the
//! uploader writes.

use super::{DeriveOp, GeneratedFormula, GeneratedParam, ParamKind};

/// Declarations every transpiled body is written against.
pub const PRELUDE: &str = include_str!("mb2_prelude.wgsl");

/// Where one parameter ended up in its formula's block.
#[derive(Clone, Copy, Debug)]
pub struct Placement<'a> {
    pub param: &'a GeneratedParam,
    /// Float index within the formula's block, `vec4`-aligned for vector kinds.
    pub index: usize,
}

/// A formula's parameters, laid out for the pool.
#[derive(Debug)]
pub struct Layout<'a> {
    pub placements: Vec<Placement<'a>>,
    /// Floats the block occupies, including alignment padding.
    pub floats: usize,
}

impl ParamKind {
    /// Alignment in f32 units. Vector and matrix kinds are read a whole `vec4`
    /// at a time, so they must start on a `vec4` boundary.
    fn align(self) -> usize {
        match self {
            Self::Float | Self::Int | Self::Bool => 1,
            Self::Float3 | Self::Float4 | Self::Matrix33 => 4,
        }
    }
}

/// Assigns each parameter an aligned position within the formula's block.
pub fn layout(f: &GeneratedFormula) -> Layout<'_> {
    // The transpiler emits parameters in placeholder order already, but the
    // layout must not depend on that.
    let mut params: Vec<&GeneratedParam> = f.params.iter().collect();
    params.sort_by_key(|p| p.offset);

    let mut next = 0usize;
    let mut placements = Vec::with_capacity(params.len());
    for param in params {
        let align = param.kind.align();
        let index = next.next_multiple_of(align);
        next = index + param.kind.floats();
        placements.push(Placement { param, index });
    }

    Layout {
        placements,
        floats: next,
    }
}

impl Layout<'_> {
    /// This formula's defaults, packed into a block ready to upload.
    pub fn defaults(&self) -> Vec<f32> {
        let mut block = vec![0.0; self.floats];
        for p in &self.placements {
            // A `Float3` default is stored with four components (the fourth is
            // padding), so copying the whole slice is right for every kind.
            for (i, v) in p.param.default.iter().enumerate() {
                if let Some(cell) = block.get_mut(p.index + i) {
                    *cell = *v;
                }
            }
        }
        block
    }
}

/// The formula's WGSL with every `__MB2Pn__` placeholder replaced by a read
/// from the pool.
///
/// `pool` is the WGSL expression naming the `array<vec4<f32>>` to read, and
/// `base` is this formula's block position within it, in floats. `base` must be
/// a multiple of 4 or the aligned placements stop being aligned.
pub fn substitute(f: &GeneratedFormula, pool: &str, base: usize) -> String {
    debug_assert_eq!(base % 4, 0, "a formula block must start on a vec4");

    let mut body = f.wgsl.to_owned();
    for p in layout(f).placements {
        let at = base + p.index;
        let (v, c) = (at / 4, ["x", "y", "z", "w"][at % 4]);
        let read = match p.param.kind {
            // `Int` stays an f32. Mandelbulber's integer parameters are
            // iteration bounds and flags that its C++ freely mixes into float
            // arithmetic; WGSL will not, and casting to `i32` here breaks far
            // more expressions than it fixes. `Aux::i` is an f32 for the same
            // reason, so the comparisons these appear in still typecheck.
            ParamKind::Float | ParamKind::Int => format!("{pool}[{v}].{c}"),
            // Written back out as a bool because the bodies use these directly
            // in conditions.
            ParamKind::Bool => format!("({pool}[{v}].{c} > 0.5)"),
            ParamKind::Float3 => format!("{pool}[{v}].xyz"),
            ParamKind::Float4 => format!("{pool}[{v}]"),
            ParamKind::Matrix33 => format!(
                "mat3x3<f32>({pool}[{v}].xyz, {pool}[{}].xyz, {pool}[{}].xyz)",
                v + 1,
                v + 2
            ),
        };
        let placeholder = format!("__MB2P{}__", p.param.offset);

        // A `switch` selector is the one place WGSL insists on a real integer,
        // so it gets the cast that the general reads deliberately omit.
        body = body.replace(
            &format!("switch {placeholder}"),
            &format!("switch i32({read})"),
        );

        // Placeholders are delimited on both sides, so `__MB2P1__` cannot match
        // inside `__MB2P12__` and the replacement order does not matter.
        body = body.replace(&placeholder, &read);
    }
    body
}

/// Applies every derivation in `f`, in order, to a parameter block.
///
/// Mandelbulber does this once per edit in `RecalculateFractalParams`; 3DM does
/// it on the way to the GPU, which keeps the editable parameters and the ones
/// the shader reads in the same block without the UI having to know which is
/// which. Order matters: `mandelbox.mboxFactor1` is a ratio of two squares that
/// are themselves derived.
pub fn recompute(f: &GeneratedFormula, block: &mut [f32]) {
    let layout = layout(f);
    let at = |offset: usize| -> Option<usize> {
        layout
            .placements
            .iter()
            .find(|p| p.param.offset == offset)
            .map(|p| p.index)
    };

    for d in f.derivations {
        let Some(target) = at(d.target) else { continue };
        let index: Vec<usize> = d.sources.iter().filter_map(|o| at(*o)).collect();
        if index.len() != d.sources.len() {
            continue;
        }
        let read = |i: usize| block.get(i).copied().unwrap_or(0.0);
        let src: Vec<f32> = index.iter().map(|i| read(*i)).collect();

        match d.op {
            DeriveOp::Square => set(block, target, src[0] * src[0]),
            // Mandelbulber divides without guarding; a zero here would produce
            // an infinity that poisons the whole iteration, so it stays zero.
            DeriveOp::Reciprocal => set(block, target, safe_div(1.0, src[0])),
            DeriveOp::Sqrt => set(block, target, src[0].max(0.0).sqrt()),
            DeriveOp::Ratio => set(block, target, safe_div(src[0], src[1])),
            DeriveOp::Cos => set(block, target, src[0].to_radians().cos()),
            DeriveOp::Sin => set(block, target, src[0].to_radians().sin()),
            DeriveOp::Rotation2 | DeriveOp::Rotation4 => {
                // One source, a 3-vector of degrees held in consecutive floats
                // — not three separate parameters.
                let base = index[0];
                let angles =
                    [read(base), read(base + 1), read(base + 2)].map(f32::to_radians);
                let m = match d.op {
                    DeriveOp::Rotation2 => {
                        mul(rot_z(angles[2]), mul(rot_y(angles[1]), rot_x(angles[0])))
                    }
                    _ => mul(rot_x(angles[0]), mul(rot_y(angles[1]), rot_z(angles[2]))),
                };
                // Stored as three padded rows. WGSL's `mat3x3` constructor
                // takes *columns*, and Mandelbulber's matrices are row-major,
                // so the transpose goes in — otherwise every rotation comes
                // out backwards.
                for (r, row) in m.iter().enumerate() {
                    for (c, value) in row.iter().enumerate() {
                        set(block, target + c * 4 + r, *value);
                    }
                }
            }
        }
    }
}

fn set(block: &mut [f32], index: usize, value: f32) {
    if let Some(cell) = block.get_mut(index) {
        *cell = value;
    }
}

fn safe_div(a: f32, b: f32) -> f32 {
    if b == 0.0 { 0.0 } else { a / b }
}

type Mat3 = [[f32; 3]; 3];

fn mul(a: Mat3, b: Mat3) -> Mat3 {
    let mut out = [[0.0; 3]; 3];
    for (r, row) in out.iter_mut().enumerate() {
        for (c, cell) in row.iter_mut().enumerate() {
            *cell = (0..3).map(|k| a[r][k] * b[k][c]).sum();
        }
    }
    out
}

// Ported from Mandelbulber's `CRotationMatrix`, which is row-major.
fn rot_x(a: f32) -> Mat3 {
    let (s, c) = a.sin_cos();
    [[1.0, 0.0, 0.0], [0.0, c, -s], [0.0, s, c]]
}

fn rot_y(a: f32) -> Mat3 {
    let (s, c) = a.sin_cos();
    [[c, 0.0, s], [0.0, 1.0, 0.0], [-s, 0.0, c]]
}

fn rot_z(a: f32) -> Mat3 {
    let (s, c) = a.sin_cos();
    [[c, -s, 0.0], [s, c, 0.0], [0.0, 0.0, 1.0]]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::formulas::generated::GENERATED;

    /// The same product Mandelbulber's `SetRotation2` builds, for checking the
    /// storage convention independently of any formula.
    fn rotation2(x: f32, y: f32, z: f32) -> Mat3 {
        mul(rot_z(z), mul(rot_y(y), rot_x(x)))
    }

    fn apply(m: Mat3, v: [f32; 3]) -> [f32; 3] {
        std::array::from_fn(|r| (0..3).map(|c| m[r][c] * v[c]).sum())
    }

    #[test]
    fn a_quarter_turn_about_z_maps_x_onto_y() {
        let m = rotation2(0.0, 0.0, std::f32::consts::FRAC_PI_2);
        let v = apply(m, [1.0, 0.0, 0.0]);
        assert!((v[0] - 0.0).abs() < 1e-6, "{v:?}");
        assert!((v[1] - 1.0).abs() < 1e-6, "{v:?}");
    }

    #[test]
    fn matrices_are_stored_as_columns_for_wgsl() {
        // WGSL's `mat3x3` constructor takes columns and Mandelbulber's matrices
        // are row-major, so `recompute` stores the transpose. Getting this
        // backwards rotates everything the wrong way — a mistake that renders
        // perfectly plausibly, which is why it is pinned here rather than left
        // to inspection.
        let f = GENERATED
            .iter()
            .find(|f| f.name == "MandelboxFast")
            .expect("MandelboxFast has a mainRot matrix");

        let layout = layout(f);
        let angle = layout
            .placements
            .iter()
            .find(|p| p.param.path == "mandelbox.rotationMain")
            .expect("the rotation source was appended");
        let matrix = layout
            .placements
            .iter()
            .find(|p| p.param.path == "mandelbox.mainRot")
            .expect("the matrix is a parameter");

        let mut block = layout.defaults();
        // 90 degrees about z.
        block[angle.index + 2] = 90.0;
        recompute(f, &mut block);

        let read = |c: usize, r: usize| block[matrix.index + c * 4 + r];
        // Column 0 of the stored matrix must be row-major row 0 read down,
        // i.e. the transpose: M[0][0], M[1][0], M[2][0] = 0, 1, 0.
        assert!((read(0, 0) - 0.0).abs() < 1e-6, "{}", read(0, 0));
        assert!((read(0, 1) - 1.0).abs() < 1e-6, "{}", read(0, 1));
        assert!((read(1, 0) + 1.0).abs() < 1e-6, "{}", read(1, 0));
    }

    #[test]
    fn derived_parameters_are_no_longer_zero() {
        // The bug this whole mechanism exists for: a squared fold radius that
        // arrives as zero is a fold whose condition never fires.
        let f = GENERATED
            .iter()
            .find(|f| f.name == "MandelboxFast")
            .unwrap();
        let layout = layout(f);
        let mut block = layout.defaults();
        recompute(f, &mut block);

        for path in ["mandelbox.fR2", "mandelbox.mR2", "mandelbox.mboxFactor1"] {
            let p = layout
                .placements
                .iter()
                .find(|p| p.param.path == path)
                .unwrap();
            assert!(block[p.index] > 0.0, "{path} is still {}", block[p.index]);
        }
    }

    #[test]
    fn every_derivation_resolves_to_a_real_parameter() {
        for f in GENERATED {
            let layout = layout(f);
            let known = |offset: usize| {
                layout.placements.iter().any(|p| p.param.offset == offset)
            };
            for d in f.derivations {
                assert!(known(d.target), "{}: target {} missing", f.name, d.target);
                for s in d.sources {
                    assert!(known(*s), "{}: source {s} missing", f.name);
                }
            }
        }
    }
}
