// pour faire plusieur grosse cellule utiliser hashmap

use bevy::prelude::PluginGroup;
use bevy::{
    DefaultPlugins,
    app::{App, FixedUpdate, Startup},
    render::camera::ClearColor,
    window::{Window, WindowPlugin},
};
use blest::{consts::r#const::BACKGROUND_COLOR, resources::resource::Grid};
use blest::{
    consts::r#const::DIMENSION_CELL,
    systems::{setup, subcell},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Blest".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(Grid {
            grid: [0; (DIMENSION_CELL.0 * DIMENSION_CELL.1) as usize],
            is_initialized: false,
        })
        .add_systems(Startup, setup::setup_system)
        .add_systems(FixedUpdate, subcell::subcell_system)
        .run();
}
