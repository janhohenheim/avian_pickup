use crate::prelude::*;

mod components;
mod on_add_holding;
mod on_remove_holding;
mod set_velocities;
mod update_error;
mod update_targets;
pub(super) fn plugin(app: &mut App) {
    app.configure_sets(
        PhysicsSchedule,
        (
            HoldSystem::UpdateError,
            HoldSystem::SetTargets,
            HoldSystem::SetVelocities,
        )
            .chain()
            .in_set(HandleVerbSystem::Hold),
    )
    .add_plugins((
        on_add_holding::plugin,
        on_remove_holding::plugin,
        components::plugin,
        update_error::plugin,
        update_targets::plugin,
        set_velocities::plugin,
    ));
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
enum HoldSystem {
    UpdateError,
    SetTargets,
    SetVelocities,
}

pub(super) mod prelude {
    pub(crate) use super::components::{HoldError, ShadowParams};
}
