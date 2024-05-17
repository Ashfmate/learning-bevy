# Topics
## Modules
In previous lessons, we had the `main.rs` file handle many things like creating the object, moving it, logging it and also running our application.

This is fine, but it is best to keep one functionality or task for our main file. That task would be to create our app and also run it.
Now what about the other things we mentioned? Well, we could make a module for each one. Each module would be its own rust file.
This type of organization is not just helpful for uncluttering our main file, but it also helps with readability since it makes our files spell out what you would expect within them.

___
# Changes
Added three files.
- **`src/movement.rs`**
```rust
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
```
- **`src/debug.rs`**
```rust
use bevy::prelude::*;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, debug_entity);
    }
}

fn debug_entity(query: Query<(Entity, &Transform)>) {
    for (entity, pos) in query.iter() {
        info!("Entity {entity:?} is at position {:?}", pos.translation);
    }
}
```
- **`src/object.rs`**
```rust
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
```

**`src/main.rs`** file now looks like this
```rust
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
```

___
# Syntax
```rust
mod debug;
mod movement;
mod object;
```
Here mod means that you will create a module and link it through the `main.rs` file.
Whenever there is a rust file in the `src` directory, its name will have to be linked and the way to do so, is by using the keyword `mod` then its name then a semicolon.

```rust
use crate::{debug::DebugPlugin, movement::MovementPlugin, object::ObjectPlugin};
```
Here we see like before, a `use` statement, but this one has `crate`, this is because modules is like a hierarchy, you can put modules inside of modules. The keyword `crate` means that you would want to get at the root of the hierarchy.
Another keyword `super` is used to get to the module that is above it. To get the module that is inside of it, just use the name and it will suffice.

___
# Results
Nothing different, same as before.