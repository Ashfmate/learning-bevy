use crate::movement::Velocity;
use bevy::prelude::*;

pub struct ObjectPlugin;

impl Plugin for ObjectPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn);
    }
}

fn spawn(mut cmd: Commands) {
    cmd.spawn((
        SpatialBundle::default(),
        Velocity {
            vel: Vec3::new(0.1, 0.1, 0.1),
        },
    ));
}
