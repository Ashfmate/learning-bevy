use bevy::prelude::*;

#[derive(Component)]
pub struct Velocity {
    pub vel: Vec3,
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update);
    }
}

fn update(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    for (velocity, mut pos) in query.iter_mut() {
        pos.translation += velocity.vel * time.delta_seconds();
    }
}
