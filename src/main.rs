use bevy::prelude::*;

#[derive(Component, Debug)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Component, Debug)]
struct Velocity {
    x: f32,
    y: f32,
}

fn main() {
    App::new()
        .add_systems(Startup, spawn_entity)
        .add_systems(Update, (update_pos, debug_entity))
        .add_plugins(DefaultPlugins)
        .run();
}

fn spawn_entity(mut cmd: Commands) {
    cmd.spawn((Position { x: 0.0, y: 0.0 }, Velocity { x: 0.1, y: 0.1 }));
}

fn update_pos(mut query: Query<(&Velocity, &mut Position)>) {
    for (vel, mut pos) in query.iter_mut() {
        pos.x += vel.x;
        pos.y += vel.y;
    }
}

fn debug_entity(query: Query<(Entity, &Position)>) {
    for (entity, pos) in query.iter() {
        info!("Entity {entity:?} is at position {pos:?}");
    }
}
