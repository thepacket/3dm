//! Parses Mandelbulber2's `fractal_cl.h` into a map from parameter path to type.
//!
//! Formulas reference parameters as `fractal->transformCommon.rotationMatrix`.
//! To emit a correct uniform read we need to know whether that is a scalar, a
//! vector, a matrix or a flag, which means resolving the path through the
//! nested `typedef struct { … } sSomethingCl;` declarations in the header.

use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ParamType {
    Float,
    Float3,
    Float4,
    Int,
    Bool,
    Matrix33,
}

impl ParamType {
    /// How many `f32` slots this occupies in the uniform pool.
    pub fn floats(self) -> usize {
        match self {
            Self::Float | Self::Int | Self::Bool => 1,
            Self::Float3 | Self::Float4 => 4,
            // Stored as three padded rows so each starts on a vec4 boundary.
            Self::Matrix33 => 12,
        }
    }

    pub fn wgsl(self) -> &'static str {
        match self {
            Self::Float => "f32",
            Self::Float3 => "vec3<f32>",
            Self::Float4 => "vec4<f32>",
            Self::Int => "i32",
            Self::Bool => "bool",
            Self::Matrix33 => "mat3x3<f32>",
        }
    }

    fn from_c(token: &str) -> Option<Self> {
        Some(match token {
            "cl_float" | "float" => Self::Float,
            "cl_float3" | "float3" => Self::Float3,
            "cl_float4" | "float4" => Self::Float4,
            "cl_int" | "int" => Self::Int,
            "cl_bool" | "bool" => Self::Bool,
            "matrix33" | "cl_matrix33" => Self::Matrix33,
            _ => return None,
        })
    }
}

/// A field of a struct: either a leaf with a known scalar/vector type, or a
/// nested struct we have to keep walking.
#[derive(Clone, Debug)]
enum Field {
    Leaf(ParamType),
    Nested(String),
}

#[derive(Default)]
pub struct TypeMap {
    structs: HashMap<String, HashMap<String, Field>>,
}

impl TypeMap {
    /// Parses every `typedef struct { … } sNameCl;` block in the header.
    pub fn parse(header: &str) -> Self {
        let mut structs: HashMap<String, HashMap<String, Field>> = HashMap::new();
        let bytes: Vec<&str> = header.lines().collect();
        let mut i = 0;

        while i < bytes.len() {
            if !bytes[i].trim_start().starts_with("typedef struct") {
                i += 1;
                continue;
            }
            // Body runs to the closing `} sName;`.
            let mut fields = HashMap::new();
            let mut j = i + 1;
            let mut name = String::new();
            while j < bytes.len() {
                let line = bytes[j].trim();
                if let Some(rest) = line.strip_prefix('}') {
                    name = rest.trim().trim_end_matches(';').trim().to_owned();
                    break;
                }
                if let Some((ty, field)) = parse_field(line) {
                    fields.insert(field, ty);
                }
                j += 1;
            }
            if !name.is_empty() {
                structs.insert(name, fields);
            }
            i = j + 1;
        }

        Self { structs }
    }

    /// Resolves `mandelbox.foldingLimit` (the part after `fractal->`) starting
    /// from the root struct, following nested structs as needed.
    pub fn resolve(&self, root: &str, path: &str) -> Option<ParamType> {
        let mut current = root.to_owned();
        let mut segments = path.split('.').peekable();

        while let Some(segment) = segments.next() {
            // Array subscripts are stripped; we resolve the element type.
            let segment = segment.split('[').next().unwrap_or(segment);
            let field = self.structs.get(&current)?.get(segment)?;
            match field {
                Field::Leaf(ty) => {
                    // A leaf must be the final segment, otherwise the path
                    // reaches into something we do not model (e.g. a union).
                    return segments.peek().is_none().then_some(*ty);
                }
                Field::Nested(next) => current = next.clone(),
            }
        }
        None
    }
}

fn parse_field(line: &str) -> Option<(Field, String)> {
    let line = line.split("//").next()?.trim().trim_end_matches(';').trim();
    if line.is_empty() {
        return None;
    }
    let mut parts = line.split_whitespace();
    let ty = parts.next()?;
    let name = parts.next()?;
    // Strip array declarators from the field name.
    let name = name.split('[').next()?.trim_start_matches('*').to_owned();
    if name.is_empty() {
        return None;
    }

    if let Some(t) = ParamType::from_c(ty) {
        Some((Field::Leaf(t), name))
    } else if ty.starts_with('s') && ty.ends_with("Cl") {
        Some((Field::Nested(ty.to_owned()), name))
    } else {
        None
    }
}
