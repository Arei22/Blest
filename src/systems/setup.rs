use bevy::{
    color::Color,
    core_pipeline::core_2d::Camera2d,
    ecs::system::Commands,
    text::TextColor,
    ui::{
        BorderColor, Node, UiRect, Val,
        widget::{Button, Text},
    },
    utils::default,
};

use crate::components::component::Reset;
use crate::utils::grid::display_grid;

pub fn setup_system(mut commands: Commands) {
    commands.spawn(Camera2d);

    let e = display_grid(&mut commands);

    commands
        .spawn((
            Node {
                width: Val::Auto,
                height: Val::Auto,
                top: Val::Px(60.),
                left: Val::Px(60.),
                border: UiRect::all(Val::Px(1.)),
                ..default()
            },
            BorderColor(Color::BLACK),
            Button,
            Reset { e: e },
        ))
        .with_child((Text("Rejouer".to_string()), TextColor(Color::BLACK)));
}
