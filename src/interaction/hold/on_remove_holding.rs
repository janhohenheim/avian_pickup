use avian3d::math::{Scalar, TAU};

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(on_remove_holding);
}

fn on_remove_holding(
    trigger: On<Remove, HeldBy>,
    mut commands: Commands,
    mut q_prop: Query<(
        Option<&NonPickupMass>,
        &mut LinearVelocity,
        &mut AngularVelocity,
    )>,
) {
    let entity = trigger.entity;

    let Ok((non_pickup_mass, mut velocity, mut angvel)) = q_prop.get_mut(entity) else {
        error!("Prop entity was deleted or in an invalid state. Ignoring.");
        return;
    };

    let non_pickup_mass: Option<Mass> = non_pickup_mass.map(|non_pickup_mass| non_pickup_mass.0);

    // HL2 uses 190 inches per second, which is 4.826 meters per second.
    // let's round that to 5 m/s.
    const HL2_NORM_SPEED: Scalar = 5.0;
    const MAX_DROP_LINEAR_SPEED: Scalar = HL2_NORM_SPEED * 1.5;
    const MAX_DROP_ANGULAR_SPEED: Scalar = TAU * 2.0;
    velocity.0 = velocity.clamp_length_max(MAX_DROP_LINEAR_SPEED);
    angvel.0 = angvel.clamp_length_max(MAX_DROP_ANGULAR_SPEED);

    // We queue in case HeldBy is removed by the fact the entity is being deleted.
    commands.queue(move |world: &mut World| {
        let Ok(mut entity) = world.get_entity_mut(entity) else {
            return;
        };

        if let Some(mass) = non_pickup_mass {
            entity.insert(mass);
            entity.remove::<NonPickupMass>();
        } else {
            entity.remove::<Mass>();
        }
    });
}
