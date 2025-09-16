use std::time::Duration;

use crate::prelude::*;
use avian3d::prelude::*;
use bevy_platform::collections::HashMap;
use bevy_time::prelude::*;

use crate::{AvianPickupSystem, prelude::AvianPickupAction};

pub(super) mod prelude {
    pub(crate) use super::Cooldown;
}

pub(super) fn plugin(app: &mut App) {
    app.add_systems(PhysicsSchedule, tick.in_set(AvianPickupSystem::TickTimers));
}

/// Timings taken from [`CWeaponPhysCannon::SecondaryAttack`](https://github.com/ValveSoftware/source-sdk-2013/blob/master/src/game/server/hl2/weapon_physcannon.cpp#L2284)
#[derive(Debug, Clone, Component)]
pub(crate) struct Cooldown(HashMap<AvianPickupAction, Timer>);

impl Default for Cooldown {
    fn default() -> Self {
        let map = AvianPickupAction::iter()
            .map(|action| (action, default()))
            .collect();

        Self(map)
    }
}

impl Cooldown {
    fn get(&self, action: &AvianPickupAction) -> &Timer {
        // Safety: all actions are always present in the map as we initialize them in `default`.
        self.0.get(action).unwrap()
    }

    fn set(&mut self, action: AvianPickupAction, seconds: f32) {
        self.0
            .insert(action, Timer::from_seconds(seconds, TimerMode::Once));
    }

    pub(crate) fn is_finished(&self, action: AvianPickupAction) -> bool {
        self.get(&action).is_finished()
    }

    pub(crate) fn throw(&mut self) {
        // Happens to be the same as `drop`, but that's a coincidence.
        self.set(AvianPickupAction::Pull, 0.5);
    }

    pub(crate) fn drop(&mut self) {
        self.set(AvianPickupAction::Pull, 0.5);
    }

    pub(crate) fn hold(&mut self) {
        // Sneakily updated in two places:
        // - [+ 0.5](https://github.com/ValveSoftware/source-sdk-2013/blob/master/src/game/server/hl2/weapon_physcannon.cpp#L2316)
        // - [+ 0.4](https://github.com/ValveSoftware/source-sdk-2013/blob/master/src/game/server/hl2/weapon_physcannon.cpp#L2438)
        // Let's use just 0.4, that feels nicer.
        self.set(AvianPickupAction::Drop, 0.4);
    }

    pub(crate) fn pull(&mut self) {
        self.set(AvianPickupAction::Pull, 0.1);
    }

    pub(crate) fn tick(&mut self, time: Duration) {
        for timer in self.0.values_mut() {
            timer.tick(time);
        }
    }
}

fn tick(mut query: Query<&mut Cooldown>, time: Res<Time>) {
    for mut cooldown in query.iter_mut() {
        cooldown.tick(time.delta());
    }
}
