use crate::{
    components::component::Position, consts::r#const::DIMENSION_CELL, resources::resource::Grid,
};
use bevy::{
    ecs::{
        query::{Changed, With},
        system::{Query, ResMut},
    },
    hierarchy::Children,
    ui::{
        Interaction,
        widget::{Button, Text},
    },
};

pub fn set_value(grid: &mut Grid, mine: i32) {
    grid.grid[mine as usize] = -1;

    for i in -1..2 {
        if mine % DIMENSION_CELL.0 as i32 + i >= DIMENSION_CELL.0 as i32
            || mine % DIMENSION_CELL.0 as i32 + i < 0
        {
            continue;
        }

        for j in -1..2 {
            if mine / DIMENSION_CELL.0 as i32 + j < DIMENSION_CELL.1 as i32
                && mine / DIMENSION_CELL.0 as i32 + j >= 0
                && (j != 0 || i != 0)
            {
                grid.grid[(mine + j * DIMENSION_CELL.0 as i32 + i) as usize] += 1;
            }
        }
    }
}

pub fn generate_grid(grid: &mut Grid, x: f32, y: f32) {
    rand::rng();

    let mut mine: i32;

    let pos_index = y.mul_add(DIMENSION_CELL.0, x) as i32;

    let mut rand_factor: f32 = 0.;
    let mut mine_offset = 0;
    let mut x_offset = -1.;
    let error_margin = 1.;

    if (x - (DIMENSION_CELL.0 - 1.)).abs() < error_margin {
        rand_factor = 1.;
        mine_offset = -1;
    } else if x == 0. {
        rand_factor = 1.;
        mine_offset = -1;
        x_offset = 0.;
    }

    if y == 0. {
        mine = rand::random_range(
            (0.)..(DIMENSION_CELL
                .0
                .mul_add(DIMENSION_CELL.1, rand_factor.mul_add(2., -6.))),
        )
        .floor() as i32;
        if mine >= pos_index + x_offset as i32 {
            mine += 3 + mine_offset;
        }
        if mine >= (y + 1.).mul_add(DIMENSION_CELL.0, x + x_offset) as i32 {
            mine += 3 + mine_offset;
        }
    } else if (y - (DIMENSION_CELL.1 - 1.)).abs() < error_margin {
        mine = rand::random_range(
            (0.)..(DIMENSION_CELL
                .0
                .mul_add(DIMENSION_CELL.1, rand_factor.mul_add(2., -6.))),
        )
        .floor() as i32;

        if mine >= (y - 1.).mul_add(DIMENSION_CELL.0, x + x_offset) as i32 {
            mine += 3 + mine_offset;
        }
        if mine >= pos_index + x_offset as i32 {
            mine += 3 + mine_offset;
        }
    } else {
        mine = rand::random_range(
            (0.)..(DIMENSION_CELL
                .0
                .mul_add(DIMENSION_CELL.1, rand_factor.mul_add(2., -9.))),
        )
        .floor() as i32;

        if mine >= (y - 1.).mul_add(DIMENSION_CELL.0, x + x_offset) as i32 {
            mine += 3 + mine_offset;
        }
        if mine >= pos_index + x_offset as i32 {
            mine += 3 + mine_offset;
        }
        if mine >= (y + 1.).mul_add(DIMENSION_CELL.0, x + x_offset) as i32 {
            mine += 3 + mine_offset;
        }
    }

    mine %= (DIMENSION_CELL.0 * DIMENSION_CELL.1) as i32;

    set_value(grid, mine);
}

type SubcellButtonType = (Changed<Interaction>, With<Button>);

pub fn subcell_system(
    mut interaction_query: Query<(&Interaction, &Position, &Children), SubcellButtonType>,
    mut grid: ResMut<Grid>,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, pos, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                if !grid.is_initialized {
                    grid.is_initialized = true;
                    generate_grid(&mut grid, pos.x, pos.y);
                }
                let val = grid.grid[pos.y.mul_add(DIMENSION_CELL.0, pos.x) as usize];
                if val == -1 {
                    **text = "*".to_string();
                } else {
                    **text = val.to_string();
                }
            }
            Interaction::Hovered | Interaction::None => {}
        }
    }
}
