# Topics
## ECS
If you want something to exist in your game, you are going to have to declare it to be so.
Declaring a an object as part of the game itself (meaning something that can be interacted with in the game) can be done in many systems, but we use ECS.
ECS stands for Entity Component Systems and its idea is that you have entities and components in your systems, each having its purpose in defining the interaction between each others and between you, the player, and the game object.

**Entity**: That is the game object itself, its the thing we want to interact with.
**Component**: That is the part of an entity, one that is shared among other entities in terms of functionality.

Basically if you have a player in game, that is an **entity**. If you have a health system, that is a **component**.

You can learn more about ECS in this [wiki](https://en.wikipedia.org/wiki/Entity_component_system)

In practice you would create a struct, make the struct derive from component.

## Query
Query is a generic struct that works based on the bevy core parts. As its name applies, it queries stuff, but it does so as an input to a function.
The way to use it is certainly like magic, you just put whatever struct name you would want to be queried (If you have multiple then just put them in a tuple).

the query has a lot of amazing utilities but the one we will be using is the iteration functionality, this will make us iterate through the queried structs.

## Adding systems
To add functionality into the game (i.e you have functions that are executed), just use the add systems function into the game, define the schedule[^1] to define when it is executed then give it the name of the function it should execute (if you have multiple then wrap them in a tuple).

___
# Additions, Deletions and Changes

## Addition

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

Added a few structs, made them derive from `Component` as well as `Debug` so that we can print the values later.

```rust
fn spawn_entity(mut cmd: Commands) {
    cmd.spawn((Position { x: 0.0, y: 0.0 }, Velocity { x: 0.1, y: 0.1 }));
}

fn update_pos(mut query: Query<(&Velocity, &mut Position)>) {
    for (vel, mut pos) in query.iter_mut() {
        pos.x += vel.x;
        pos.y += vel.y;
    }
}

fn debug_entity(query: Query<(Entity, &Position)>) {
    for (entity, pos) in query.iter() {
        info!("Entity {entity:?} is at position {pos:?}");
    }
}
```

Added the `spawn_entity` function which just uses a command and spawns a `Position` and a `Velocity`. This is what is planned to run at startup.
Added the `update_pos` function which just adds the values of `Velocity` into `Position`. This is what is planned to be run on update.
Added the `debug_entity` function which just prints the entity we create (in `spawn_entity`) and its corresponding position. This is what is planned to be running on update.

```rust
.add_systems(Startup, spawn_entity)
.add_systems(Update, (update_pos, debug_entity))
```

Added these to use `spawn_entity` on startup, `update_pos` and `debug_entity` on update through the `add_systems` function which is a function in `App::new()`.

## Deletion

NONE

## Changes

NONE

___
# Syntactic Explanation

```rust
#[derive(Component, Debug)]
struct Velocity {
    x: f32,
    y: f32,
}
```

This right here we create a struct called `Velocity` with fields `x` and `y` with each being an `f32`, which means that they are floating numbers; real numbers.
The `#[derive(Component, Debug)]` part is `proc_macro`, which means that it is a procedural macro and it alters the behavior of the struct where it can be features of a [component](<Lesson 2>#ECS) as well as have debugging features.

```rust
Query<(&Velocity, &mut Position)>
```

This right here is type of the input we want when querying some components.

```rust
for (vel, mut pos) in query.iter_mut()
```

This is a for loop that iterates through the iterations in `query`, we get that through the function `iter_mut`, though this will also allow us to mutated the element. We do get two values `vel` and `pos` (this is mutable) as a tuple (multiple data from one source).

```rust
info!("Entity {entity:?} is at position {pos:?}");
```

This is a macro where it is used like a function, this macro's just prints whatever is inside the string in a formatted way, `entity` is actually a variable, so is `pos`, they are just formatted so that the value of `entity` is copied in string from into the spot right where we placed it in the string. The colon and question mark are there to allow for debugging purposes since neither `entity` nor `pos` are strings.

___
# Results

Still a blank screen though with some debugging features, we are adding `0.1` to the position of the entity and it is printed in the command line.

[^1]: The multiple types of schedules that we used are:
	- **`StartUp`**: This is a schedule where the function(s) will be run only once and at the start of the game creation.
	- **`Update`**: This is a schedule where the function(s) will be run once every frame.