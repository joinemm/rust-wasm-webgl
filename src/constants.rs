pub const GRID_SIZE: usize = 200;
pub const FOV: f32 = 45. * std::f32::consts::PI / 180.; // in radians
pub const Z_FAR: f32 = 100.;
pub const Z_NEAR: f32 = 0.1;
pub const Z_PLANE: f32 = -2.414213 - 1.73; // -1 / tan(pi/8)
