use bevy::prelude::*;

use crate::{
    camera::CameraPlugin, debug::DebugPlugin, movement::MovementPlugin, object::ObjectPlugin,
};

mod camera;
mod debug;
mod movement;
mod object;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.5, 0.5)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 75.0,
        })
        .add_plugins(CameraPlugin)
        .add_plugins(ObjectPlugin)
        .add_plugins(MovementPlugin)
        // .add_plugins(DebugPlugin)
        .add_plugins(DefaultPlugins)
        .run();
}
