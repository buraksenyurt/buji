#[derive(Debug, Default)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Position {
        Position { x, y }
    }
}
#[derive(Debug)]
pub struct ScaleFactor(pub f32);

impl Default for ScaleFactor {
    fn default() -> ScaleFactor {
        ScaleFactor(1.0)
    }
}

#[derive(Debug)]
pub struct Rotation {
    pub angle_radians: f32,
}

impl Default for Rotation {
    fn default() -> Rotation {
        Self::ZERO
    }
}

impl Rotation {
    pub const ZERO: Self = Self { angle_radians: 0.0 };
    pub fn from_degrees(degrees: f32) -> Self {
        Self {
            angle_radians: degrees.to_radians(),
        }
    }
    pub fn from_radians(radians: f32) -> Self {
        Self {
            angle_radians: radians,
        }
    }

    pub fn to_degrees(&self) -> f32 {
        self.angle_radians.to_degrees()
    }
}
