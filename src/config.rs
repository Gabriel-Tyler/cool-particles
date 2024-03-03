use bevy::prelude::Color;

pub const WORLD_WIDTH: f32 = 500.0;
pub const WORLD_HEIGHT: f32 = 500.0;

pub const N_GROUPS: usize = 4;
pub const GROUP_COUNTS: [u32; N_GROUPS] = [100, 100, 100, 100];
pub const GROUP_COLORS: [Color; N_GROUPS] = [Color::RED, Color::GREEN, Color::BLUE, Color::WHITE];
pub const GROUP_NAMES: [&str; N_GROUPS] = ["alpha", "lambda", "beta", "delta"];
pub const GROUP_Z_POSITION: [f32; N_GROUPS] = [0.0, 0.0, 0.0, 0.0];

pub const GRAVITY_CONSTANT: f32 = 0.1;
// attractions[i][j] is how attracted i is to j
pub const ATTRACTIONS: [[f32; N_GROUPS]; N_GROUPS] = [
    // r,   g,   b,   w
    [30.0, -2.0, 35.0, -17.0],   // r
    [40.0, -30.0, -15.0, -20.0], // g
    [-14.0, -30.0, -35.0, 35.0], // b
    [11.0, 21.0, 35.0, 40.0],    // w
];
pub const ATTRACTIONS_RADIUS: [[f32; N_GROUPS]; N_GROUPS] = [
    // r,   g,   b,   w
    [75.0, 245.0, 80.0, 270.0],  // r
    [250.0, 240.0, 6.0, 115.0],  // g
    [270.0, 11.0, 240.0, 106.0], // b
    [27.0, 118.0, 265.0, 127.0], // w
];

pub const PARTICLE_RADIUS: f32 = 2.0;
