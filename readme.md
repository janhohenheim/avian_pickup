# Avian Pickup

A plugin for implementing picking up dynamic rigid bodies in [Avian physics](https://github.com/Jondolf/avian/) for the [Bevy engine](https://bevyengine.org/).
Modeled after Half Life 2's gravity gun.

## Examples

[`examples/play_catch.rs`](https://github.com/janhohenheim/avian_pickup/blob/main/examples/play_catch.rs):\
TODO: gif here!
[`examples/prop_playground.rs`](https://github.com/janhohenheim/avian_pickup/blob/main/examples/prop_playground.rs):\
TODO: gif here!
[`examples/manipulate_prop.rs`](https://github.com/janhohenheim/avian_pickup/blob/main/examples/manipulate_prop.rs):\
TODO: gif here!

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
- Only a single object can be picked up at a time.
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
cargo add avian --git https://github.com/Jondolf/avian3d
```

Additionally, you need some sort of interpolation:

```sh
cargo add bevy_transform_interpolation --git https://github.com/Jondolf/bevy_transform_interpolation
```

### Usage

TODO

## Version Compatibility

| `avian_pickup` | `avian` | `bevy` |
|---------------|---------|-------|
| `main`       | `main` | `0.14` |
