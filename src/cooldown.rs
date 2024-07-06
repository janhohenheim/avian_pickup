use std::time::Duration;

use avian3d::prelude::*;
use bevy::prelude::*;

use crate::AvianPickupSystem;

pub(super) mod prelude {
    pub(crate) use super::AvianPickupCooldown;
}

pub(super) fn plugin(app: &mut App) {
    app.get_schedule_mut(PhysicsSchedule)
        .unwrap()
        .add_systems(tick.in_set(AvianPickupSystem::TickTimers));
}

/// Timings taken from [`CWeaponPhysCannon::SecondaryAttack`](https://github.com/ValveSoftware/source-sdk-2013/blob/master/sp/src/game/server/hl2/weapon_physcannon.cpp#L2284)
#[derive(Debug, Clone, PartialEq, Component, Default)]
pub(crate) struct AvianPickupCooldown {
    pub(crate) left: Timer,
    pub(crate) right: Timer,
}

impl AvianPickupCooldown {
    pub(crate) fn drop_held(&mut self) {
        self.left = Timer::from_seconds(0.5, TimerMode::Once);
        self.right = Timer::from_seconds(0.5, TimerMode::Once);
    }

    pub(crate) fn hold(&mut self) {
        self.right = Timer::from_seconds(0.5, TimerMode::Once);
    }

    pub(crate) fn pull(&mut self) {
        self.right = Timer::from_seconds(0.1, TimerMode::Once);
    }

    pub(crate) fn tick(&mut self, time: Duration) {
        self.left.tick(time);
        self.right.tick(time);
    }
}

fn tick(mut query: Query<&mut AvianPickupCooldown>, time: Res<Time>) {
    for mut cooldown in query.iter_mut() {
        cooldown.tick(time.delta());
    }
}
