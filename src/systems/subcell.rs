use crate::{
    components::component::Position,
    consts::r#const::{DIMENSION_CELL, MAX_MINE, MIN_MINE},
    resources::resource::Grid,
};
use bevy::{
    ecs::{
        hierarchy::Children,
        query::{Changed, With},
        system::{Query, ResMut},
    },
    ui::{
        Interaction,
        widget::{Button, Text},
    },
};
use rand::random_range;

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
                && grid.grid[(mine + j * DIMENSION_CELL.0 as i32 + i) as usize] != -1
            {
                grid.grid[(mine + j * DIMENSION_CELL.0 as i32 + i) as usize] += 1;
            }
        }
    }
}

#[must_use]
pub fn do_offset_mines(mut mine: i32, offset_mines: i32, mines: &[i32]) -> i32 {
    for i in 0..offset_mines {
        if mine >= mines[i as usize] {
            mine += 1;
        }
    }
    mine
}

pub fn generate_mine(grid: &mut Grid, x: f32, offset_mines: f32, mines: &[i32]) -> i32 {
    let mut mine: i32;

    let mut rand_factor: f32 = 0.;
    let error_margin = 1.;

    if (x - (DIMENSION_CELL.0 - 1.)).abs() < error_margin || x == 0. {
        rand_factor = 1.;
    }

    mine = random_range(
        (0.)..(DIMENSION_CELL.0.mul_add(
            DIMENSION_CELL.1,
            rand_factor.mul_add(2., -6.) - offset_mines,
        )),
    )
    .floor() as i32;

    mine = do_offset_mines(mine, offset_mines as i32, mines);
    mine %= (DIMENSION_CELL.0 * DIMENSION_CELL.1) as i32;

    set_value(grid, mine);

    mine
}

pub fn generate_grid(grid: &mut Grid, x: f32, y: f32) {
    rand::rng();

    let nb_mines = random_range(MIN_MINE..=MAX_MINE);
    let mut mines: Vec<i32> = Vec::with_capacity(nb_mines as usize);

    let error_margin = 1.;
    let mut x_start: f32 = -1.;
    let mut x_end: f32 = 2.;
    let mut y_start: f32 = -1.;
    let mut y_end: f32 = 2.;

    if (x - (DIMENSION_CELL.0 - 1.)).abs() < error_margin {
        x_end -= 1.;
    } else if x == 0. {
        x_start += 1.;
    }
    if (y - (DIMENSION_CELL.1 - 1.)).abs() < error_margin {
        y_end -= 1.;
    } else if x == 0. {
        y_start += 1.;
    }

    for i in (x_start as i32)..(x_end as i32) {
        for j in (y_start as i32)..(y_end as i32) {
            mines.push((y as i32 + i) * DIMENSION_CELL.0 as i32 + (x as i32 + j));
        }
    }

    let mut i = 0.;
    while (i as i32) < nb_mines {
        mines.push(generate_mine(
            grid,
            x,
            (x_end - x_start).mul_add(y_end - y_start, i),
            &mines.clone(),
        ));
        mines.sort_unstable();
        i += 1.;
    }
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
