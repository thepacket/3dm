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

use super::{GeneratedFormula, GeneratedParam, ParamKind};

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
