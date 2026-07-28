//! 3DM — a modern Mandelbulb explorer.
//!
//! The fractal is rendered entirely on the GPU by sphere-tracing a distance
//! estimator in a fragment shader; the CPU only maintains the camera and the
//! parameter set. The same code runs natively and, via WebGPU, in the browser.

pub mod app;
pub mod camera;
pub mod formulas;
pub mod params;
pub mod render;

pub use app::App;
pub use camera::Camera;
pub use formulas::{FormulaKind, FormulaSlot, FormulaStack};
pub use params::SceneParams;
