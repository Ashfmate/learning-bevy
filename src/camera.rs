use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create);
    }
}

fn create(mut cmd: Commands) {
    cmd.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, -10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}
