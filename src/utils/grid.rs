use bevy::{
    color::Color,
    ecs::{entity::Entity, system::Commands},
    text::TextColor,
    ui::{
        AlignSelf, BorderColor, Display, GridTrack, JustifySelf, Node, UiRect, Val,
        widget::{Button, Text},
    },
    utils::default,
};

use crate::{
    components::component::Position,
    consts::r#const::{
        CELL_BORDER, CELL_BORDER_COLOR, DIMENSION_CELL, SUBCELL_BORDER, SUBCELL_BORDER_COLOR,
    },
};

pub fn display_grid(commands: &mut Commands) -> Entity {
    let subcell = (
        Button,
        Node {
            width: Val::Px(600. / DIMENSION_CELL.0),
            height: Val::Px(600. / DIMENSION_CELL.1),
            border: UiRect::all(Val::Px(SUBCELL_BORDER)),
            ..default()
        },
        BorderColor(SUBCELL_BORDER_COLOR),
    );

    let cell = (
        Node {
            width: Val::Px(4.0f32.mul_add(SUBCELL_BORDER, 600.)),
            height: Val::Px(4.0f32.mul_add(SUBCELL_BORDER, 600.)),
            display: Display::Grid,
            grid_template_columns: vec![
                GridTrack::px(600. / DIMENSION_CELL.0);
                DIMENSION_CELL.0 as usize
            ],
            grid_template_rows: vec![
                GridTrack::px(600. / DIMENSION_CELL.1);
                DIMENSION_CELL.1 as usize
            ],
            border: UiRect::all(Val::Px(CELL_BORDER)),
            ..default()
        },
        BorderColor(CELL_BORDER_COLOR),
    );

    let e = commands
        .spawn(Node {
            width: Val::Auto,
            height: Val::Auto,
            display: Display::Grid,
            align_self: AlignSelf::Center,
            justify_self: JustifySelf::Center,
            grid_template_columns: vec![GridTrack::px(600.)],
            grid_template_rows: vec![GridTrack::px(600.)],
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(cell).with_children(|parent| {
                let mut i = 0.;
                while (i as i32) < DIMENSION_CELL.0 as i32 {
                    let mut j = 0.;
                    while (j as i32) < DIMENSION_CELL.1 as i32 {
                        parent
                            .spawn((subcell.clone(), Position { x: j, y: i }))
                            .with_child((Text::new(""), TextColor(Color::srgb(0., 0., 0.))));
                        j += 1.;
                    }
                    i += 1.;
                }
            });
        })
        .id();

    e
}
