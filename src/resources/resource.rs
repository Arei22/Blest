use bevy::ecs::resource::Resource;

use crate::consts::r#const::DIMENSION_CELL;

#[derive(Resource)]
pub struct Grid {
    pub grid: [i32; (DIMENSION_CELL.0 * DIMENSION_CELL.1) as usize],
    pub is_initialized: bool,
}
