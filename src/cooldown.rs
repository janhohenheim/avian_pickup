use std::time::Duration;

use avian3d::prelude::*;
use bevy::prelude::*;

use crate::AvianPickupSystem;

pub(super) mod prelude {
    pub(crate) use super::Cooldown;
}

pub(super) fn plugin(app: &mut App) {
    app.add_systems(PhysicsSchedule, tick.in_set(AvianPickupSystem::TickTimers));
}

/// Timings taken from [`CWeaponPhysCannon::SecondaryAttack`](https://github.com/ValveSoftware/source-sdk-2013/blob/master/sp/src/game/server/hl2/weapon_physcannon.cpp#L2284)
#[derive(Debug, Clone, PartialEq, Component, Default)]
pub(crate) struct Cooldown {
    pub(crate) left: Timer,
    pub(crate) right: Timer,
}

impl Cooldown {
    pub(crate) fn throw(&mut self) {
        // Happens to be the same as `drop`, but that's a coincidence
        // Also, the CD does not differentiate between throwing a held object
        // and throwing an object in front of us.
        self.left = Timer::from_seconds(0.5, TimerMode::Once);
        self.right = Timer::from_seconds(0.5, TimerMode::Once);
    }

    pub(crate) fn drop(&mut self) {
        self.left = Timer::from_seconds(0.5, TimerMode::Once);
        self.right = Timer::from_seconds(0.5, TimerMode::Once);
    }

    pub(crate) fn hold(&mut self) {
        // Sneakily updated in two places:
        // - [+ 0.5](https://github.com/ValveSoftware/source-sdk-2013/blob/master/sp/src/game/server/hl2/weapon_physcannon.cpp#L2316)
        // - [+ 0.4](https://github.com/ValveSoftware/source-sdk-2013/blob/master/sp/src/game/server/hl2/weapon_physcannon.cpp#L2438)
        // Let's use just 0.4, that feels nicer.
        self.right = Timer::from_seconds(0.4, TimerMode::Once);
    }

    pub(crate) fn pull(&mut self) {
        self.right = Timer::from_seconds(0.1, TimerMode::Once);
    }

    pub(crate) fn tick(&mut self, time: Duration) {
        self.left.tick(time);
        self.right.tick(time);
    }
}

fn tick(mut query: Query<&mut Cooldown>, time: Res<Time>) {
    for mut cooldown in query.iter_mut() {
        cooldown.tick(time.delta());
    }
}
