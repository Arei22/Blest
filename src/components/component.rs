use bevy::ecs::{component::Component, entity::Entity};

#[derive(Component)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct Reset {
    pub e: Entity,
}
