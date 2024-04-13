# Topics
## Vectors
A vector is a concept meant to denote a direction with magnitude, unlike scalar which denotes only magnitude.
This means that there should be more than one value for a vector, though instead of using angles with scalars directly, we use (x, y, z) numbers.
These numbers tell the difference of each of their respective axis, not the total amount from (0,0,0).

This actually gives us the direction (because we have all three axis values) as well as magnitude (the length of the vector).

Vectors can be two dimensional or three dimensional, in fact it can be N-dimensional, it is up to us to determine.

## Transformation
Transformations are the operations that are done on vectors.
Of the multiple transformations that there are, they are:
- **Rotation:** rotates the vector by some amount.
- **Scaling:** grows the vector or shrinks it, or if it is negative then it does that while also flipping it.
- **Translation:** moves the vector values.

## Plugins
Plugins in bevy are a way to organize the multiple systems that we will be adding. This way it becomes easy to disable some and enable some without affecting other parts.

___
# Additions, Deletions, Changes

## Addition

```rust
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
```

The plugins that we will need.

## Deletion

```rust
add_systems(Startup, spawn_entity)
add_systems(Update, (update_pos, debug_entity))
```

Both add_system functions were replaced by the plugins.

## Changes

```rust
#[derive(Component, Debug)]
struct Velocity {
    vel: Vec3,
}
```

**INSTEAD OF**

```rust
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

```

**AND**

```rust
add_plugins(SpawnPlugin)
add_plugins(MovementPlugin)
add_plugins(DebugPlugin)

```

**INSTEAD OF**

```rust
add_systems(Startup, spawn_entity)
add_systems(Update, (update_pos, debug_entity))
```

**AND**

```rust
cmd.spawn((
    SpatialBundle::default(),
    Velocity {
        vel: Vec3::new(0.1, 0.1, 0.1),
    },
));
```

**INSTEAD OF**

```rust
cmd.spawn((Position { x: 0.0, y: 0.0 }, Velocity { x: 0.1, y: 0.1 }));
```

**AND**

```rust
pos.translation.x += velocity.vel.x;
pos.translation.y += velocity.vel.y;
pos.translation.z += velocity.vel.z;
```

**INSTEAD OF**

```rust
pos.x += vel.x;
pos.y += vel.y;
```

**AND**

```rust
info!("Entity {entity:?} is at position {:?}", pos.translation);
```

**INSTEAD OF**

```rust
info!("Entity {entity:?} is at position {pos:?}");
```

___
# Syntax

```rust
impl Plugin for SpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_entity);
    }
}
```

In here we are using the `impl` block with a `trait`
An `impl` block is a part of a `struct` where the functions are defined.
A `struct` defines the variables, and the `impl` block defines functions.

We can however inherit[^1] from a `trait` (A thing that has a collection of functions, sort of like an `impl` block that has no `struct`) where we take its functionality and make it our struct's functionality. Here we are implementing the `Plugin` `trait` for our `struct`.
When we do inherit[^1] from a `trait` we must define all of the functions that the `trait` did not define but declared.

An `impl` block can have no `trait` to inherit[^1] from where the functions will be attributed to the `struct` only, meaning that we will just be defining functions that can only be present in the `struct` and not inheritable by any other `struct` . To do that we just write `impl` `struct_name` then open a code block.

___
# Results
Nothing different, same as before.

[^1]: Rust does not support inheritance the same way `OOP` languages do.