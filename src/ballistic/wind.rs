#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Wind {
    /// Wind velocity along the bullet path in feet per second.
    /// Positive values represent headwind, negative values represent tailwind.
    pub headwind_fps: f64,

    /// Wind velocity perpendicular to the bullet path in feet per second.
    /// Positive values represent right-to-left crosswind.
    pub crosswind_fps: f64,
}

impl Wind {
    pub const fn calm() -> Self {
        Self {
            headwind_fps: 0.0,
            crosswind_fps: 0.0,
        }
    }
}

impl Default for Wind {
    fn default() -> Self {
        Self::calm()
    }
}
