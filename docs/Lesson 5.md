# Topics
## Bundles
Bundles in bevy are a collection of components, like if your game character may have health points, speed, attack damage and such. Obviously you would want to *bundle* all of these together.
## Time
Time is easy to understand, it is something that allows for you to add flow into a project where it will be constant wherever you run it. If entities are reliant on time instead of computer cycles, then a high end PC will run at a smooth state but at the same speed as a low end PC.
The way this is done is by calculating the difference in time between previous frames and multiplying the the result with some of the vectors used to transform the world.
## Lighting and Background Color
Lighting is whole study of itself in graphics designing, this is why I will not bore you with lighting much, just know that things need to be lit to be seen. Bevy assumes a brightness level of 0.
Background color is the color of the world.
## Assets
Assets in video games have a not so clear meaning but there are agreed upon definitions. They are files or resources that reside in the disk that are integral parts of the game.
This means:
- Sprites (2-D images)
- 3-D objects
- Sound files
This list extends a lot but these are the most common types of files.
In bevy, assets are all stored in the sub-directory `assets`. When accessing any of them, just spell their path starting from that sub-directory.

___
# Differences
Added a camera module in **`src/camera.rs`**
```rust
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
```

Added it as well as some resources in **`src/main.rs`**
```rust
.insert_resource(ClearColor(Color::rgb(0.0, 0.5, 0.5)))
.insert_resource(AmbientLight {
    color: Color::default(),
    brightness: 75.0,
})
.add_plugins(CameraPlugin)
```

Changed the **`src/object.rs`** file to use assets

```rust
#[derive(Bundle)]
struct ObjectBundle {
    model: SceneBundle,
    vel: Velocity,
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
```

___
# Syntax

```rust
.insert_resource(ClearColor(Color::rgb(0.0, 0.5, 0.5)))
```
Here the struct `ClearColor` is not initialized with curly braces, it is initialized with round braces. This is because it does not contain named variables. It is a tuple, which is like a struct but without functions and without named variables (uses numbers for each variable, so the first variable is 0, the second is 1 and so on).

```rust
// .add_plugins(DebugPlugin)
```
Here you can see that this looks like valid rust code but there is something in the beginning that looks peculiar. Those are two forward slashes and they signify that the line is only a comment, meaning that the rust compiler will not look at that piece of code, it will only serve as a way to talk to the programmer.
Here we are using it to keep the `DebugPlugin` part of the main app but have it not be activated. This is a crude way of doing it but effective since all that we would need to reactivate the plugin is by removing the two forward slashes.

```rust
scene: asset_server.load("Tree.glb#Scene0"),
```
Here the the file name has an extra part, `#Scene0`. This is because video games are divided into scenes and we would want to show this `glb` (3-D object) file in scene 0, which is the default first scene.

___
# Results
Now we have a camera that shows our world and we have an object that moves about.