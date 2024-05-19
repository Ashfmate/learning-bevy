use crate::movement::Velocity;
use bevy::prelude::*;

#[derive(Bundle)]
struct ObjectBundle {
    model: SceneBundle,
    vel: Velocity,
}

pub struct ObjectPlugin;

impl Plugin for ObjectPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn);
    }
}

fn spawn(mut cmd: Commands, asset_server: Res<AssetServer>) {
    cmd.spawn(ObjectBundle {
        model: SceneBundle {
            scene: asset_server.load("Tree.glb#Scene0"),
            transform: Transform::from_translation(Vec3::new(-10.0, 0.0, 0.0)),
            ..Default::default()
        },
        vel: Velocity {
            vel: Vec3::new(1.0, 0.0, 0.0),
        },
    });
}
