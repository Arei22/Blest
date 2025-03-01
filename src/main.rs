// pour faire plusieur grosse cellule utiliser hashmap

use bevy::{
    DefaultPlugins,
    app::{App, FixedUpdate, Startup},
    render::camera::ClearColor,
};
use blest::{consts::r#const::BACKGROUND_COLOR, resources::resource::Grid};
use blest::{
    consts::r#const::DIMENSION_CELL,
    systems::{setup, subcell},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(Grid {
            grid: [0; (DIMENSION_CELL.0 * DIMENSION_CELL.1) as usize],
            is_initialized: false,
        })
        .add_systems(Startup, setup::setup_system)
        .add_systems(FixedUpdate, subcell::subcell_system)
        .run();
}
