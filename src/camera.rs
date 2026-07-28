//! Orbit camera.
//!
//! The camera is stored in spherical coordinates around a target point, which
//! is what you want for inspecting an object that sits at the origin. Position
//! and basis vectors are derived on demand rather than stored, so there is no
//! state to keep in sync.

use std::f32::consts::{FRAC_PI_2, TAU};

/// Never let the pitch reach straight up/down — the basis degenerates there.
const PITCH_LIMIT: f32 = FRAC_PI_2 - 0.001;

/// Orbit radius bounds. Wide enough that f32 runs out of significant digits
/// before these do: the old floor of 0.05 stopped a zoom roughly sixty times
/// out from the surface, which is nowhere near where the numbers give up.
const DISTANCE_MIN: f32 = 1e-6;
const DISTANCE_MAX: f32 = 1e5;

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
        self.distance =
            (radius * 1.15 / self.tan_half_fov().max(1e-3)).clamp(DISTANCE_MIN, DISTANCE_MAX);
    }

    /// Orbit by a screen-space drag, in radians per unit.
    pub fn orbit(&mut self, delta_yaw: f32, delta_pitch: f32) {
        self.yaw = (self.yaw + delta_yaw).rem_euclid(TAU);
        self.pitch = (self.pitch + delta_pitch).clamp(-PITCH_LIMIT, PITCH_LIMIT);
    }

    /// Dolly in/out multiplicatively, so zoom feels the same at every scale.
    ///
    /// This shrinks the orbit radius, so it converges on the target and cannot
    /// pass it. To descend into the fractal, use [`Camera::advance`].
    pub fn zoom(&mut self, factor: f32) {
        self.distance = (self.distance * factor).clamp(DISTANCE_MIN, DISTANCE_MAX);
    }

    /// Fly along the view direction, carrying the orbit target along.
    ///
    /// This is the difference between inspecting a fractal and exploring one.
    /// Zooming shrinks the orbit radius, so it converges on the target — and
    /// the target starts at the origin, which is *inside* the solid, so zooming
    /// in eventually buries the camera in it. Advancing moves the point being
    /// orbited too, so a crevice can be descended for as long as f32 has bits
    /// left, which is where the limit ought to be.
    ///
    /// `amount` is a fraction of the current orbit radius, so the step shrinks
    /// as you close in and the descent stays self-similar.
    pub fn advance(&mut self, amount: f32) {
        let (_, _, forward) = self.basis();
        let step = self.distance * amount;
        for (target, axis) in self.target.iter_mut().zip(forward) {
            *target += axis * step;
        }
    }

    /// Close on a world point by a fraction of the gap between it and the eye.
    ///
    /// This is the gesture Mandelbulber's wheel performs: pick a spot, then
    /// approach *that*, rather than whatever the camera happens to orbit. The
    /// orbit target travels with the eye, so the point stays put on screen and
    /// what follows — orbiting, panning — pivots around where you were heading.
    ///
    /// A fraction, so it can be applied repeatedly without ever arriving.
    pub fn approach(&mut self, point: [f32; 3], fraction: f32) {
        let eye = self.position();
        for (i, target) in self.target.iter_mut().enumerate() {
            *target += (point[i] - eye[i]) * fraction;
        }
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
