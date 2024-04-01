# Additions, Deletions and Changes

## Addition

```rs
use bevy::prelude::*;

fn main() {
    App::new().add_plugins(DefaultPlugins).run();
}
```

It is the first commit, as such this is only what is added, the reason for this is explained in the headers later in this page.

## Deletion

NONE

## Changes

NONE

___

# Explanation

## Syntax

`use bevy::prelude::*;` Imports the standard, most common components to the main.rs file

```rs
fn main() {
    App::new().add_plugins(DefaultPlugins).run();
}
```

In here the `fn` keyword defines a function, `main` is the name of the function (It is also a special name, where it is the first function that will be executed by the program).
`App` is the struct that will hold our game instance.
`new` is a static function that creates a game instance.
`add_plugins` is a member function that will take plugins, pieces of code that have functionalities.
`DefaultPlugins` is a plugin that will generate the default functionalities, like window creation.
`run` is a member function that will start the game.

## Semantic

When creating a new game, you must import from bevy's modules. Now they are many and so you will need to write so many use statements, but the developers of bevy have been kind to make one module that encompasses all of the major and common modules, structs, functions, etc.
This one modules is the prelude module and it is located in the bevy module.

Once everything is imported, we can start using the components that are given by bevy.
The component that we will use is `App`, it is a struct that contains the game that we will create and configure.
We use the static function `new` to create a new game instance, then we add `DefaultPlugins` into our game (sort of configuring it), lastly we run the game, with the configurations that we have assigned.

___

# Results

As of now it only creates a blank window, but later, wil change the color, add a camera and all of the necessary stuff needed in the game
