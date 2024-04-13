use bevy::prelude::*;

#[derive(Component, Debug)]
struct Velocity {
    vel: Vec3,
}

struct SpawnPlugin;

impl Plugin for SpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_entity);
    }
}

struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_pos);
    }
}

struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, debug_entity);
    }
}

fn main() {
    App::new()
        .add_plugins(SpawnPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(DebugPlugin)
        .add_plugins(DefaultPlugins)
        .run();
}

fn spawn_entity(mut cmd: Commands) {
    cmd.spawn((
        SpatialBundle::default(),
        Velocity {
            vel: Vec3::new(0.1, 0.1, 0.1),
        },
    ));
}

fn update_pos(mut query: Query<(&Velocity, &mut Transform)>) {
    for (velocity, mut pos) in query.iter_mut() {
        pos.translation.x += velocity.vel.x;
        pos.translation.y += velocity.vel.y;
        pos.translation.z += velocity.vel.z;
    }
}

fn debug_entity(query: Query<(Entity, &Transform)>) {
    for (entity, pos) in query.iter() {
        info!("Entity {entity:?} is at position {:?}", pos.translation);
    }
}
