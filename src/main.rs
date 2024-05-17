use bevy::prelude::*;

use crate::{debug::DebugPlugin, movement::MovementPlugin, object::ObjectPlugin};

mod debug;
mod movement;
mod object;

fn main() {
    App::new()
        .add_plugins(ObjectPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(DebugPlugin)
        .add_plugins(DefaultPlugins)
        .run();
}
