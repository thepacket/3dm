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

/// What shape a light is and how its rays reach the surface.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum LightKind {
    /// Infinitely far away: every ray is parallel and nothing attenuates.
    #[default]
    Directional,
    /// Radiates from a point, falling off with the decay function.
    Point,
    /// A point light restricted to a cone about its direction.
    Cone,
}

impl LightKind {
    pub const ALL: [Self; 3] = [Self::Directional, Self::Point, Self::Cone];

    pub fn label(self) -> &'static str {
        match self {
            Self::Directional => "directional light",
            Self::Point => "point light",
            Self::Cone => "cone light",
        }
    }
}

/// How quickly a positional light thins out with distance.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum LightDecay {
    /// Cylindrical falloff. Reaches much further than physics allows.
    InvR,
    /// The inverse-square law.
    #[default]
    InvR2,
    /// Falls off faster than physics, for a tightly localised pool of light.
    InvR3,
}

impl LightDecay {
    pub const ALL: [Self; 3] = [Self::InvR, Self::InvR2, Self::InvR3];

    pub fn label(self) -> &'static str {
        match self {
            Self::InvR => "1/r",
            Self::InvR2 => "1/r\u{b2}",
            Self::InvR3 => "1/r\u{b3}",
        }
    }

    /// The exponent the shader raises the distance to.
    pub fn exponent(self) -> f32 {
        match self {
            Self::InvR => 1.0,
            Self::InvR2 => 2.0,
            Self::InvR3 => 3.0,
        }
    }
}

/// One light in the scene.
///
/// Mandelbulber states a light's orientation as angles and its place as a
/// position, and derives the direction from whichever of the two the light
/// actually uses. 3DM keeps `direction` as the stored truth for directional
/// lights — it is what the shader wants, and it means the default scene's key
/// light survives verbatim rather than being reconstructed from angles that
/// only round-trip to six figures.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Light {
    pub enabled: bool,
    pub kind: LightKind,
    /// Linear, not sRGB — as everywhere else in this file.
    pub color: [f32; 3],
    pub intensity: f32,
    /// Brightness of the light's own disc where it is visible in the
    /// background. Zero means the light itself is never drawn.
    pub visibility: f32,
    /// Angular size of that disc, and — for positional lights — the radius
    /// that decides how soft its shadows are.
    pub size: f32,
    /// Edge hardness of the visible disc. High is a sharp-edged ball.
    pub contour_sharpness: f32,
    pub decay: LightDecay,
    /// Where the light sits. Ignored by a directional light.
    pub position: [f32; 3],
    /// Direction the light travels *from*, toward the scene. Unit length is
    /// not required; the shader normalises.
    pub direction: [f32; 3],
    /// Interpret `position` and `direction` in the camera's frame, so the
    /// light rides along instead of staying put as the camera moves.
    pub relative_to_camera: bool,
    pub cast_shadows: bool,
    /// Half-angle of the shadow's penumbra, in degrees, for a directional
    /// light. Positional lights take their penumbra from `size` instead,
    /// since a lamp's softness is set by how big it is and how far away.
    pub soft_shadow_cone: f32,
    /// Fade a shadow by how far along the ray to the light it was cast, so
    /// distant occluders block less. Mandelbulber's "penetrating light".
    pub penetrating: bool,
    /// Full opening angle of a cone light, in degrees.
    pub cone_angle: f32,
    /// How much of that opening is a soft edge, in degrees.
    pub cone_soft_angle: f32,
}

impl Default for Light {
    fn default() -> Self {
        Self {
            enabled: true,
            kind: LightKind::Directional,
            color: [1.0, 1.0, 1.0],
            intensity: 1.0,
            visibility: 0.0,
            size: 1.0,
            contour_sharpness: 1.0,
            decay: LightDecay::default(),
            position: [0.0, 0.0, 0.0],
            direction: [0.6, 0.7, -0.4],
            relative_to_camera: false,
            cast_shadows: true,
            // tan(3.576°) = 1/16, the penumbra constant the shader was tuned
            // with when there was one hard-coded light. Stated as an angle
            // here because that is how Mandelbulber states it, and because an
            // angle keeps its meaning as the scene is scaled.
            soft_shadow_cone: 3.576,
            penetrating: false,
            cone_angle: 30.0,
            cone_soft_angle: 10.0,
        }
    }
}

impl Light {
    /// The penumbra constant the soft-shadow march wants: large is sharp.
    ///
    /// A directional light's penumbra is an angle, so it is constant with
    /// distance. A positional light's is the angle its disc subtends, which
    /// the shader has to work out per pixel — hence the two paths.
    pub fn shadow_k(&self) -> f32 {
        let range = self.soft_shadow_cone.to_radians().tan();
        if range > 1e-6 { 1.0 / range } else { 1.0e6 }
    }
}

/// Everything on Mandelbulber's Lights tab that is not one specific light.
#[derive(Clone, PartialEq, Debug)]
pub struct LightsParams {
    /// Tint of the skylight that fills whatever the lights do not reach.
    /// White leaves the ambient term neutral.
    pub fill_color: [f32; 3],
    /// Scales every light's intensity at once.
    pub all_intensity: f32,
    /// Scales every light's visible disc, and every positional light's
    /// shadow softness, at once.
    pub all_size: f32,
    pub lights: Vec<Light>,
}

/// More than this and the uniform grows faster than the panel is usable.
/// Mandelbulber has no fixed limit; a uniform block does.
pub const MAX_LIGHTS: usize = 8;

impl Default for LightsParams {
    fn default() -> Self {
        Self {
            fill_color: [1.0, 1.0, 1.0],
            all_intensity: 1.0,
            all_size: 1.0,
            lights: vec![Light::default()],
        }
    }
}

/// How a background image is wrapped around the scene.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum BackgroundMap {
    /// Longitude across, latitude up: the usual 2:1 panorama.
    #[default]
    Equirectangular,
    /// Two fisheye circles side by side, as a 360 camera produces.
    DoubleHemisphere,
    /// Projected flat, as though pinned on a wall the camera faces.
    Flat,
}

impl BackgroundMap {
    pub const ALL: [Self; 3] = [
        Self::Equirectangular,
        Self::DoubleHemisphere,
        Self::Flat,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Self::Equirectangular => "equirectangular",
            Self::DoubleHemisphere => "double hemisphere",
            Self::Flat => "flat",
        }
    }
}

/// What sits behind the fractal.
///
/// Kept apart from [`ShadingParams`] because none of it is lighting: the fog
/// target and the ambient sky tint read `color1` and nothing else, so a
/// gradient or an image changes what you see past the shape without changing
/// how the shape itself is lit.
#[derive(Clone, PartialEq, Debug)]
pub struct BackgroundParams {
    /// The colour overhead, and the one flat backgrounds use throughout.
    /// Linear, not sRGB.
    pub color1: [f32; 3],
    /// Enables `color2` at the horizon and `color3` below it.
    pub three_colors: bool,
    pub color2: [f32; 3],
    pub color3: [f32; 3],
    pub brightness: f32,
    pub gamma: f32,
    /// Draw `texture` instead of the colours. Ignored while none is loaded.
    pub textured: bool,
    pub map: BackgroundMap,
    pub h_scale: f32,
    pub v_scale: f32,
    pub offset: [f32; 2],
    /// Yaw, pitch and roll applied to the ray before it samples the image,
    /// in degrees — which is how the panel states it.
    pub rotation: [f32; 3],
}

impl Default for BackgroundParams {
    fn default() -> Self {
        Self {
            color1: [0.004, 0.006, 0.011],
            three_colors: false,
            // Mandelbulber's own blue / white / green, kept so that ticking
            // the box gives the gradient it gives there rather than three
            // shades of the existing near-black.
            color2: [0.7, 0.7, 0.7],
            color3: [0.0, 0.06, 0.0],
            brightness: 1.0,
            gamma: 1.0,
            textured: false,
            map: BackgroundMap::default(),
            h_scale: 1.0,
            v_scale: 1.0,
            offset: [0.0, 0.0],
            rotation: [0.0, 0.0, 0.0],
        }
    }
}

impl BackgroundParams {
    /// The ray rotation as three row vectors, ready for the uniform.
    ///
    /// Built here rather than in the shader: it is the same matrix for every
    /// pixel, and the shader already has enough trigonometry per ray.
    pub fn rotation_rows(&self) -> [[f32; 4]; 3] {
        let (a, b, c) = (
            self.rotation[0].to_radians(),
            self.rotation[1].to_radians(),
            self.rotation[2].to_radians(),
        );
        let (sa, ca) = (a.sin(), a.cos());
        let (sb, cb) = (b.sin(), b.cos());
        let (sc, cc) = (c.sin(), c.cos());
        // Yaw about y, then pitch about x, then roll about z.
        [
            [
                ca * cc + sa * sb * sc,
                cb * sc,
                -sa * cc + ca * sb * sc,
                0.0,
            ],
            [
                -ca * sc + sa * sb * cc,
                cb * cc,
                sa * sc + ca * sb * cc,
                0.0,
            ],
            [sa * cb, -sb, ca * cb, 0.0],
        ]
    }
}

/// Lighting, ambient occlusion and colour.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct ShadingParams {
    /// Fraction of skylight applied where nothing else lights the surface.
    pub ambient: f32,
    /// Strength of the ambient-occlusion darkening in creases.
    pub ao_strength: f32,
    /// Tint of the occluded light. White leaves occlusion neutral.
    pub ao_color: [f32; 3],
    /// Samples the occlusion term takes along the normal.
    pub ao_quality: i32,
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
    /// Distance fade toward the background colour.
    pub fog: f32,
    /// Distance over which basic fog closes in. Zero turns it off.
    pub fog_visibility: f32,
    /// Colour of that fog. Separate from the background, so haze can sit in
    /// front of a dark sky.
    pub fog_color: [f32; 3],
    /// The glow runs between these two as the ray grazes closer.
    pub glow_near: [f32; 3],
    pub glow_far: [f32; 3],
    /// Fog laid on by how many iterations a point survived — the shape's own
    /// structure rather than its distance, which is what makes it read as
    /// depth inside a fractal rather than across a room.
    pub iter_fog: f32,
    /// Iteration fractions between which the fog ramps up.
    pub iter_fog_low: f32,
    pub iter_fog_high: f32,
    pub iter_fog_near: [f32; 3],
    pub iter_fog_far: [f32; 3],
}

impl Default for ShadingParams {
    fn default() -> Self {
        Self {
            ambient: 0.45,
            ao_strength: 0.85,
            ao_color: [1.0, 1.0, 1.0],
            ao_quality: 5,
            specular: 0.35,
            glow: 0.30,
            // Note: all colours here are *linear*, not sRGB — a value of 0.005
            // is a near-black background once the display transfer curve is
            // applied, not a dark grey.
            palette_base: [0.50, 0.46, 0.42],
            palette_amp: [0.45, 0.42, 0.40],
            palette_phase: 0.0,
            palette_freq: 1.0,
            fog: 0.10,
            fog_visibility: 0.0,
            fog_color: [0.55, 0.62, 0.75],
            // What the old `palette(0.35)` evaluated to. The glow used to be
            // tinted by the palette at a fixed point; making it two explicit
            // colours must not change how the default scene looks.
            glow_near: [0.2354, 0.0612, 0.0395],
            glow_far: [0.2354, 0.0612, 0.0395],
            iter_fog: 0.0,
            iter_fog_low: 0.15,
            iter_fog_high: 0.9,
            iter_fog_near: [0.30, 0.55, 0.95],
            iter_fog_far: [1.0, 1.0, 1.0],
        }
    }
}

/// How camera rays are laid out across the frame.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum Perspective {
    /// A flat image plane. What every ordinary camera does.
    #[default]
    ThreePoint,
    /// Angle grows with distance from centre, so the frame bends outwards.
    FishEye,
    /// The whole sphere in one frame — longitude across, latitude up.
    Equirectangular,
    /// The upward hemisphere, for a planetarium dome.
    Fulldome,
}

impl Perspective {
    pub const ALL: [Self; 4] = [
        Self::ThreePoint,
        Self::FishEye,
        Self::Equirectangular,
        Self::Fulldome,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Self::ThreePoint => "three-point",
            Self::FishEye => "fish eye",
            Self::Equirectangular => "equirectangular",
            Self::Fulldome => "fulldome",
        }
    }
}

/// Adjustments applied to the finished image.
///
/// These are the last thing that happens, after the fractal is shaded and tone
/// mapped, which is why they are their own group: nothing here changes the
/// geometry or the lighting, only how the result is presented.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Picture {
    pub brightness: f32,
    pub contrast: f32,
    pub gamma: f32,
    pub saturation: f32,
    /// Skip the tone mapping and let bright values run past white.
    pub hdr: bool,
    pub perspective: Perspective,
    /// Distance from the camera that stays sharp. Zero disables the effect.
    pub focus_distance: f32,
    /// Aperture: how fast blur grows either side of the focus distance.
    pub aperture: f32,
    /// Ceiling on the blur, as a fraction of the frame height.
    pub max_blur: f32,
    /// How much of the blurred image is mixed in.
    pub blur_opacity: f32,
}

impl Default for Picture {
    fn default() -> Self {
        Self {
            brightness: 1.0,
            contrast: 1.0,
            gamma: 1.0,
            saturation: 1.0,
            hdr: false,
            perspective: Perspective::default(),
            focus_distance: 0.0,
            aperture: 1.0,
            max_blur: 0.02,
            blur_opacity: 1.0,
        }
    }
}

/// File format for an exported image.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum ImageFormat {
    /// Lossless, and the only one of the two that keeps a clean gradient in a
    /// dark background without banding.
    #[default]
    Png,
    Jpeg,
}

impl ImageFormat {
    pub const ALL: [Self; 2] = [Self::Png, Self::Jpeg];

    pub fn label(self) -> &'static str {
        match self {
            Self::Png => "PNG",
            Self::Jpeg => "JPEG",
        }
    }

    pub fn extension(self) -> &'static str {
        match self {
            Self::Png => "png",
            Self::Jpeg => "jpg",
        }
    }
}

/// What to write when the picture is exported.
///
/// Separate from the scene: none of it changes what is rendered on screen, it
/// describes a file to produce. Mandelbulber keeps the resolution here for the
/// same reason.
#[derive(Clone, PartialEq, Debug)]
pub struct Export {
    /// Base name, without the extension — that comes from `format`.
    pub name: String,
    pub format: ImageFormat,
    pub width: u32,
    pub height: u32,
}

/// Beyond this an export is likely to exceed what the GPU will allocate for a
/// single texture, and the failure arrives as a device loss rather than an
/// error worth reporting.
pub const MAX_EXPORT_PIXELS: u32 = 16384;

impl Default for Export {
    fn default() -> Self {
        Self {
            name: "3dm".to_owned(),
            format: ImageFormat::default(),
            width: 1920,
            height: 1080,
        }
    }
}

impl Export {
    /// The name the file is written under.
    pub fn file_name(&self) -> String {
        let stem = self.name.trim();
        let stem = if stem.is_empty() { "3dm" } else { stem };
        // A name that already carries the right extension should not gain a
        // second one, but a name ending in some *other* extension is a stem
        // the user typed and keeps.
        if stem
            .rsplit_once('.')
            .is_some_and(|(_, ext)| ext.eq_ignore_ascii_case(self.format.extension()))
        {
            return stem.to_owned();
        }
        format!("{stem}.{}", self.format.extension())
    }
}

/// Mandelbulber's Rendering engine panel: how hard the marcher works and what
/// counts as a surface.
///
/// Every field here defaults to a value that leaves the marcher doing exactly
/// what it did before this group existed. Mandelbulber puts its own numbers
/// behind an "Advanced quality settings" tick for the same reason — they are
/// escape hatches for a formula whose estimate misbehaves, not knobs to reach
/// for first.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Engine {
    /// Fix the hit threshold at this world-space distance rather than letting
    /// it track the pixel's angular size. Zero keeps it tracking, which is
    /// what Mandelbulber calls connecting detail to image resolution.
    pub constant_detail: f32,
    /// Ceiling and floor on the hit threshold. Zero on either lifts it.
    pub max_detail: f32,
    pub min_detail: f32,
    /// Clamps on one march step, in world units. Zero on a maximum lifts it.
    pub abs_min_step: f32,
    pub abs_max_step: f32,
    /// The same, as multiples of the hit threshold — so they follow the zoom
    /// instead of being fixed to the scene.
    pub rel_min_step: f32,
    pub rel_max_step: f32,
    /// Offset the delta estimator samples its neighbours at, relative to the
    /// point's own radius. Only formulas with no analytic derivative use it.
    pub delta_de_delta: f32,
    /// Treat a point that never escaped as solid, so the shape is exactly the
    /// set rather than the estimate's level surface.
    pub stop_at_max_iter: bool,
    /// Scales the spacing of the normal's samples. Above 1 smooths the
    /// surface and loses fine relief; below 1 sharpens it and picks up noise.
    pub smoothness: f32,
    /// Nearest distance a ray may register a hit at. Zero lets it hit
    /// anything, including geometry the camera is sitting inside.
    pub min_view_distance: f32,
    /// Clip the march to an axis-aligned box as well as the bounding sphere.
    pub limits: bool,
    pub limits_min: [f32; 3],
    pub limits_max: [f32; 3],
}

impl Default for Engine {
    fn default() -> Self {
        Self {
            constant_detail: 0.0,
            max_detail: 0.0,
            min_detail: 0.0,
            abs_min_step: 0.0,
            abs_max_step: 0.0,
            rel_min_step: 0.0,
            rel_max_step: 0.0,
            // What the generated delta estimator used before this was
            // adjustable. Mandelbulber's own default is 0.01, against a
            // differently scaled estimate.
            delta_de_delta: 1e-6,
            stop_at_max_iter: false,
            smoothness: 1.0,
            min_view_distance: 0.0,
            limits: false,
            limits_min: [-1.2, -1.2, -1.2],
            limits_max: [1.2, 1.2, 1.2],
        }
    }
}

/// Filters run over the finished frame, after everything else.
///
/// These live in the blit rather than the fractal pass because each one needs
/// to read pixels *around* the one it is producing, which a shader marching a
/// single ray cannot do. That is also why they only appear once the frame has
/// been drawn to the offscreen texture — see `render_offscreen`.
/// Each effect keeps its settings while switched off, so turning one back on
/// returns to what it was rather than to zero — which is why the enables are
/// separate flags rather than a radius of zero.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Post {
    /// Bright areas bleeding into their surroundings.
    ///
    /// Mandelbulber runs this on the image *before* tone mapping, where a
    /// specular highlight is genuinely brighter than white and so bleeds hard.
    /// 3DM tone maps in the fractal pass, so the effect is milder here unless
    /// HDR is on — which is the setting that leaves those values intact.
    pub hdr_blur: bool,
    /// How far the bleed reaches, as Mandelbulber states it: the radius in
    /// pixels is this times a thousandth of the frame's width plus height.
    pub hdr_blur_radius: f32,
    /// How much of the surroundings gets in. This is the limiter in the
    /// weight's denominator, so *small* is subtle — the centre tap is worth
    /// `1/intensity` and drowns everything else out.
    pub hdr_blur_intensity: f32,
    /// Splits the channels radially, as a lens does.
    pub aberration: bool,
    pub aberration_intensity: f32,
    /// How far each channel is smeared along the radius.
    pub aberration_blur: f32,
    /// Swap which way the channels are thrown — red outward or blue outward.
    pub aberration_reverse: bool,
}

impl Default for Post {
    fn default() -> Self {
        Self {
            hdr_blur: false,
            // Mandelbulber's own defaults, so that ticking a box gives what
            // ticking it there gives.
            hdr_blur_radius: 10.0,
            hdr_blur_intensity: 0.1,
            aberration: false,
            aberration_intensity: 5.0,
            aberration_blur: 5.0,
            aberration_reverse: false,
        }
    }
}

/// A colour stop in the surface gradient.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Stop {
    /// Position along the gradient, 0..1.
    pub at: f32,
    /// Linear RGB.
    pub color: [f32; 3],
}

/// Entries in the gradient lookup handed to the shader.
///
/// The gradient is edited as a handful of stops and baked into a table, so the
/// shader does one array read rather than walking a list of stops per pixel —
/// and the table size is fixed, so the uniform layout never changes.
pub const GRADIENT_LUT: usize = 64;

/// How the surface is coloured.
///
/// Mandelbulber calls this the material, and its own version is far larger:
/// reflections, refraction, transparency, textures, perlin noise, displacement.
/// Those need renderer capability 3DM does not have — a second ray bounce, rays
/// that continue through a surface, UV mapping on a fractal — so what is here is
/// the part the current renderer can honestly express.
#[derive(Clone, PartialEq, Debug)]
pub struct Material {
    /// Colour the surface from the gradient rather than from one flat colour.
    pub use_gradient: bool,
    /// Stops, in ascending position. Sampled by the orbit trap.
    pub gradient: Vec<Stop>,
    /// Used when `use_gradient` is off.
    pub single_color: [f32; 3],
    /// Shifts the whole gradient along the trap value.
    pub palette_offset: f32,
    /// How quickly the gradient repeats across the trap range.
    pub color_speed: f32,
    /// Strength of the angle-of-incidence term.
    pub shading: f32,
    pub specular_color: [f32; 3],
    pub specular_brightness: f32,
    /// Tightness of the highlight; small is a sharp glint.
    pub specular_width: f32,
    /// Light the surface emits regardless of what reaches it.
    pub luminosity: f32,
    pub luminosity_color: [f32; 3],
    /// How much of the view reflects off the surface. One bounce only.
    pub reflectance: f32,
    pub reflection_color: [f32; 3],
}

impl Default for Material {
    fn default() -> Self {
        Self {
            use_gradient: true,
            // The cosine palette this replaces, sampled — so the default scene
            // looks as it always did while now being editable stop by stop.
            gradient: (0..6)
                .map(|i| {
                    let at = i as f32 / 5.0;
                    let wave = |phase: f32| {
                        0.5 + 0.45 * (std::f32::consts::TAU * (at + phase)).cos()
                    };
                    Stop {
                        at,
                        color: [wave(0.0), wave(0.10), wave(0.20)],
                    }
                })
                .collect(),
            single_color: [0.55, 0.50, 0.45],
            palette_offset: 0.0,
            color_speed: 1.0,
            shading: 1.0,
            specular_color: [1.0, 1.0, 1.0],
            specular_brightness: 0.35,
            specular_width: 0.15,
            luminosity: 0.0,
            luminosity_color: [1.0, 1.0, 1.0],
            reflectance: 0.0,
            reflection_color: [1.0, 1.0, 1.0],
        }
    }
}

impl Material {
    /// Bakes the stops into the fixed-size table the shader reads.
    pub fn lut(&self) -> [[f32; 4]; GRADIENT_LUT] {
        let mut table = [[0.0f32; 4]; GRADIENT_LUT];
        if self.gradient.is_empty() {
            return table;
        }

        let mut stops = self.gradient.clone();
        stops.sort_by(|a, b| a.at.total_cmp(&b.at));

        for (i, entry) in table.iter_mut().enumerate() {
            let at = i as f32 / (GRADIENT_LUT - 1) as f32;
            // Before the first stop and after the last, hold the end colour
            // rather than fading to black.
            let color = match stops.iter().position(|s| s.at >= at) {
                None => stops[stops.len() - 1].color,
                Some(0) => stops[0].color,
                Some(next) => {
                    let (a, b) = (stops[next - 1], stops[next]);
                    let span = (b.at - a.at).max(1e-6);
                    let k = ((at - a.at) / span).clamp(0.0, 1.0);
                    std::array::from_fn(|c| a.color[c] + (b.color[c] - a.color[c]) * k)
                }
            };
            *entry = [color[0], color[1], color[2], 1.0];
        }
        table
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
    pub lights: LightsParams,
    pub background: BackgroundParams,
    pub post: Post,
    pub engine: Engine,
    pub material: Material,
    pub picture: Picture,
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

#[cfg(test)]
mod export_tests {
    use super::*;

    fn named(name: &str, format: ImageFormat) -> String {
        Export {
            name: name.to_owned(),
            format,
            ..Default::default()
        }
        .file_name()
    }

    #[test]
    fn the_extension_follows_the_chosen_format() {
        assert_eq!(named("bulb", ImageFormat::Png), "bulb.png");
        assert_eq!(named("bulb", ImageFormat::Jpeg), "bulb.jpg");
    }

    #[test]
    fn a_name_that_already_ends_in_the_right_extension_keeps_just_the_one() {
        assert_eq!(named("bulb.png", ImageFormat::Png), "bulb.png");
        assert_eq!(named("bulb.PNG", ImageFormat::Png), "bulb.PNG");
    }

    /// A dot in the name is not necessarily an extension — "bulb.v2" is a
    /// stem, and appending to it is right where replacing it would lose work.
    #[test]
    fn some_other_suffix_is_part_of_the_name() {
        assert_eq!(named("bulb.v2", ImageFormat::Png), "bulb.v2.png");
        assert_eq!(named("bulb.png", ImageFormat::Jpeg), "bulb.png.jpg");
    }

    #[test]
    fn an_empty_name_still_produces_a_file() {
        assert_eq!(named("   ", ImageFormat::Png), "3dm.png");
    }
}

#[cfg(test)]
mod material_tests {
    use super::*;

    fn stops(v: &[(f32, f32)]) -> Material {
        Material {
            gradient: v
                .iter()
                .map(|(at, g)| Stop {
                    at: *at,
                    color: [*g, *g, *g],
                })
                .collect(),
            ..Default::default()
        }
    }

    #[test]
    fn the_table_runs_between_the_stops() {
        let lut = stops(&[(0.0, 0.0), (1.0, 1.0)]).lut();
        assert!(lut[0][0] < 0.01, "starts at the first stop");
        assert!(lut[GRADIENT_LUT - 1][0] > 0.99, "ends at the last");
        // Monotonic in between, since the stops are.
        for pair in lut.windows(2) {
            assert!(pair[1][0] >= pair[0][0] - 1e-6, "{pair:?}");
        }
    }

    #[test]
    fn outside_the_stops_the_end_colours_hold() {
        // Otherwise a gradient that does not span 0..1 fades to black at the
        // edges, which reads as a shadow that is not there.
        let lut = stops(&[(0.25, 1.0), (0.75, 1.0)]).lut();
        assert!(lut[0][0] > 0.99, "before the first stop");
        assert!(lut[GRADIENT_LUT - 1][0] > 0.99, "after the last");
    }

    #[test]
    fn stops_need_not_be_given_in_order() {
        // The editor lets a stop be dragged past its neighbour, so the order in
        // the list means nothing.
        let jumbled = stops(&[(1.0, 1.0), (0.0, 0.0), (0.5, 0.5)]).lut();
        let ordered = stops(&[(0.0, 0.0), (0.5, 0.5), (1.0, 1.0)]).lut();
        assert_eq!(jumbled, ordered);
    }

    #[test]
    fn an_empty_gradient_does_not_panic() {
        let lut = Material {
            gradient: vec![],
            ..Default::default()
        }
        .lut();
        assert_eq!(lut[0], [0.0; 4]);
    }
}
