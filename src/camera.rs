//! Orbit camera.
//!
//! The camera is stored in spherical coordinates around a target point, which
//! is what you want for inspecting an object that sits at the origin. Position
//! and basis vectors are derived on demand rather than stored, so there is no
//! state to keep in sync.

use std::f32::consts::{FRAC_PI_2, TAU};

/// Never let the pitch reach straight up/down — the basis degenerates there.
const PITCH_LIMIT: f32 = FRAC_PI_2 - 0.001;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Camera {
    /// Point the camera orbits and looks at.
    pub target: [f32; 3],
    /// Horizontal angle, radians.
    pub yaw: f32,
    /// Vertical angle, radians. Clamped just shy of the poles.
    pub pitch: f32,
    /// Distance from `target`.
    pub distance: f32,
    /// Vertical field of view, degrees.
    pub fov_y: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            target: [0.0, 0.0, 0.0],
            yaw: 0.9,
            pitch: 0.35,
            distance: 2.8,
            fov_y: 55.0,
        }
    }
}

impl Camera {
    /// World-space eye position.
    pub fn position(&self) -> [f32; 3] {
        let (sp, cp) = self.pitch.sin_cos();
        let (sy, cy) = self.yaw.sin_cos();
        [
            self.target[0] + self.distance * cp * cy,
            self.target[1] + self.distance * sp,
            self.target[2] + self.distance * cp * sy,
        ]
    }

    /// Orthonormal `(right, up, forward)` basis, forward pointing at the target.
    pub fn basis(&self) -> ([f32; 3], [f32; 3], [f32; 3]) {
        let eye = self.position();
        let forward = normalize(sub(self.target, eye));
        // World up is +Y. Pitch is clamped, so this cross is never degenerate.
        let right = normalize(cross(forward, [0.0, 1.0, 0.0]));
        let up = cross(right, forward);
        (right, up, forward)
    }

    /// `tan(fov_y / 2)`, the vertical half-extent of the image plane at unit depth.
    pub fn tan_half_fov(&self) -> f32 {
        (self.fov_y.to_radians() * 0.5).tan()
    }

    /// Pull back far enough to see a fractal of the given radius, keeping the
    /// current orientation. The 1.15 leaves a little margin around the shape.
    pub fn frame(&mut self, radius: f32) {
        self.target = [0.0, 0.0, 0.0];
        self.distance = (radius * 1.15 / self.tan_half_fov().max(1e-3)).clamp(0.05, 100.0);
    }

    /// Orbit by a screen-space drag, in radians per unit.
    pub fn orbit(&mut self, delta_yaw: f32, delta_pitch: f32) {
        self.yaw = (self.yaw + delta_yaw).rem_euclid(TAU);
        self.pitch = (self.pitch + delta_pitch).clamp(-PITCH_LIMIT, PITCH_LIMIT);
    }

    /// Dolly in/out multiplicatively, so zoom feels the same at every scale.
    pub fn zoom(&mut self, factor: f32) {
        self.distance = (self.distance * factor).clamp(0.05, 100.0);
    }

    /// Pan the target across the view plane. `dx`/`dy` are in screen fractions;
    /// the movement is scaled by distance so it tracks the cursor at any zoom.
    pub fn pan(&mut self, dx: f32, dy: f32) {
        let (right, up, _) = self.basis();
        let scale = self.distance * self.tan_half_fov() * 2.0;
        for i in 0..3 {
            self.target[i] += (-right[i] * dx + up[i] * dy) * scale;
        }
    }
}

fn sub(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn cross(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

fn normalize(v: [f32; 3]) -> [f32; 3] {
    let len = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    if len > 1e-6 {
        [v[0] / len, v[1] / len, v[2] / len]
    } else {
        [0.0, 0.0, 1.0]
    }
}
