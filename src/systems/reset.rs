use crate::{
    components::component::Reset, consts::r#const::DIMENSION_CELL, resources::resource::Grid,
    utils::grid::display_grid,
};
use bevy::{
    ecs::{
        query::{Changed, With},
        system::{Commands, Query, ResMut},
    },
    ui::{Interaction, widget::Button},
};

type SubcellButtonType = (Changed<Interaction>, With<Button>);

pub fn reset_system(
    mut commands: Commands,
    mut interaction_query: Query<(&Interaction, &mut Reset), SubcellButtonType>,
    mut grid: ResMut<Grid>,
) {
    for (interaction, mut r) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *grid = Grid {
                    grid: [0; (DIMENSION_CELL.0 * DIMENSION_CELL.1) as usize],
                    is_initialized: false,
                };
                commands.get_entity(r.e).unwrap().despawn();
                let e = display_grid(&mut commands);

                r.e = e;
            }
            Interaction::Hovered | Interaction::None => {}
        }
    }
}
