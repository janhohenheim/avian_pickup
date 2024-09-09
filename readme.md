# Avian Pickup

A plugin for implementing picking up dynamic rigid bodies in [Avian physics](https://github.com/Jondolf/avian/) for the [Bevy engine](https://bevyengine.org/).
Modeled after Half Life 2's gravity gun.

## Examples

[`examples/play_catch.rs`](https://github.com/janhohenheim/avian_pickup/blob/main/examples/play_catch.rs):

<img src="docs/play_catch.gif" alt="A video showing the player playing catch with an NPC" width="50%">

## Features

- Pick up nearby dynamic rigid bodies.
- Pull far away ones towards you.
- Throw them around or drop them gently.
- Manipulate them while holding them, a bit like how the physics gun in Garry's Mod works.
- Nearly everything is configurable. Lots of knobs to turn, if you feel like it!
  - The default configuration is set up to emulate picking things up with your hands.
  - Can very easily be configured to emulate a gravity gun or a tractor beam.
- Scheduled in fixed updates for deterministic physics.
  - Parts of the plugin use randomness, which can be overridden by a user-provided `Rng`.
- Events keep you informed about what's happening so you can react with sound effects, particles, etc.
- Works for the player and AI alike.
  - Input is done with events, so you can provide your own input system.
- I think the documentation is alright :)

## Limitations

- Since the physics are running only in fixed updates but are also
    right in front of the camera, which should run in a variable update,
    you *need* some sort of interpolation to make it look good. I recommend
    [`bevy_transform_interpolation`](https://github.com/Jondolf/bevy_transform_interpolation).
- Only a single object can be picked up per actor at a time.
- An object cannot be pulled away while it is being held by someone else.
- Only works in 3D.
- Only works with dynamic rigid bodies, not static or kinematic ones.
- Performance should be alrigt, but I did not optimize much for it.
- Not tested with complex collider hierarchies or compound colliders.
- Not tested with networking.
- Not tested with Wasm (pretty sure it should work, though).

## Guide

### Installation

```sh
cargo add avian_pickup --git https://github.com/janhohenheim/avian_pickup
```

it's not on crates.io yet because I'm waiting for a new `Avian` release, as this was made
targeting the `main` branch. This means you also need to use the `main` branch of `Avian`:

```sh
cargo add avian3d --git https://github.com/Jondolf/avian
```

Additionally, you need some sort of interpolation for anything to look smooth at all:

```sh
cargo add avian_interpolation3d --git https://github.com/janhohenheim/avian_interpolation
```

Finally, add these plugins to your app. Make sure to add Avian Pickup after Avian:

```rust,no_run
use bevy::prelude::*;
use avian3d::prelude::*;
use avian_pickup::prelude::*;
use avian_interpolation3d::prelude::*;

App::new()
    .add_plugins((
        DefaultPlugins,
        // Add Avian
        PhysicsPlugins::default(),
        // Add Avian Pickup
        AvianPickupPlugin::default(),
        // Add interpolation
        AvianInterpolationPlugin::default(),
    ));
```

### Usage

The main two concepts of Avian Pickup are *actors* and *props*. It's simple:

- An *actor* is something that can pick up *props*.
    These are spatial entities with an [`AvianPickupActor`] component.
- An *prop* is an object to be picked up.
    These are spatial entities with a regular old [`RigidBody::Dynamic`] component and associated colliders.

As such, this is the minimum version of these two:

```rust
use bevy::prelude::*;
use avian3d::prelude::*;
use avian_pickup::prelude::*;

fn setup(mut commands: Commands) {
    // Actor
    commands.spawn((
        SpatialBundle::default(),
        AvianPickupActor::default(),
    ));

    // Prop
    commands.spawn((
        SpatialBundle::default(),
        RigidBody::Dynamic,
        Collider::sphere(0.5),
    ));
}
```

In order for an actor to try picking up a prop, you need to send an [`AvianPickupInput`] event:

```rust
use bevy::prelude::*;
use avian_pickup::prelude::*;

fn handle_input(
    mut avian_pickup_input_writer: EventWriter<AvianPickupInput>,
) {
    let actor_entity = todo!("Your entity goes here");
    avian_pickup_input_writer.send(AvianPickupInput {
        action: AvianPickupAction::Pull,
        actor: actor_entity,
    }); 
}
```

When using a `AvianPickupAction::Pull` action, the actor will try to pull the nearest prop they're facing towards
them. Once they have picked it up, its [`AvianPickupActorState`] will reflect that by becoming
[`AvianPickupActorState::Holding(..)`]. Note that [`AvianPickupActorState`] is a component that will automatically
get added to every actor.

That's it! You can use other actions to further instruct the actor to manipulate the prop.
The [`AvianPickupActor`] holds a lot of configuration options to tweak the behavior of the actor.
Many of these can be overridden for a specific prop by using components in the [`prop`] module.
Finally, you can also read the events in the [`output`] module to react to what's happening.

### First Personal Camera

If you want to use a first person perspective for your player and allow him to be an [`AvianPickupActor`],
you need to make sure to move the camera *before* the physics update takes place. Usually, all movement code
for physicsal entities in the world should be in the fixed timestep, but the camera is a notable exception.
A player will want to have a camera that works as smoothly as possible and updates every frame. That's why you need to place the camera in the last variable timestep schedule before the physics update. You do this like so:

```rust
use bevy::{
    app::RunFixedMainLoop,
    prelude::*,
    time::run_fixed_main_schedule,
};

App::new()
    .add_systems(
        RunFixedMainLoop,
        move_camera.before(run_fixed_main_schedule),
    );

fn move_camera() { todo!() }
```

## Version Compatibility

| `avian_pickup` | `avian` | `bevy` |
|---------------|---------|-------|
| `main`       | `main` | `0.14` |

[`AvianPickupActor`]: https://github.com/janhohenheim/avian_pickup/blob/main/src/actor.rs
[`RigidBody::Dynamic`]: https://docs.rs/avian3d/latest/avian3d/dynamics/rigid_body/enum.RigidBody.html#variant.Dynamic
[`AvianPickupActorState`]: https://github.com/janhohenheim/avian_pickup/blob/main/src/actor.rs
[`AvianPickupInput`]: https://github.com/janhohenheim/avian_pickup/blob/main/src/input.rs
[`AvianPickupActorState::Holding(..)`]: https://github.com/janhohenheim/avian_pickup/blob/main/src/actor.rs
[`prop`]: https://github.com/janhohenheim/avian_pickup/blob/main/src/prop.rs
[`output`]: https://github.com/janhohenheim/avian_pickup/blob/main/src/output.rs
