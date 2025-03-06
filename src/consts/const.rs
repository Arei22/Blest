use bevy::color::Color;

pub const BACKGROUND_COLOR: Color = Color::srgb(1., 1., 1.);

pub const CELL_BORDER: f32 = 10.0;
pub const CELL_BORDER_COLOR: Color = Color::srgb(0.8, 0.8, 0.8);

pub const DIMENSION_CELL: (f32, f32) = (10., 10.);

pub const SUBCELL_BORDER: f32 = 5.0;
pub const SUBCELL_BORDER_COLOR: Color = Color::srgb(0.8, 0.8, 0.8);

pub const MIN_MINE: i32 = 5;
pub const MAX_MINE: i32 = 20;
