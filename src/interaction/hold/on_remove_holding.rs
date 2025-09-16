use crate::{prelude::*, verb::Holding};

pub(super) fn plugin(app: &mut App) {
    app.add_observer(on_remove_holding);
}

fn on_remove_holding(
    trigger: On<Remove, Holding>,
    mut commands: Commands,
    q_actor: Query<&Holding>,
    mut q_prop: Query<(Entity, Option<&NonPickupMass>, Has<HeldProp>)>,
) {
    // Safety: We are removing a `Holding` component, so we know that the entity has
    // one.
    let holding = q_actor.get(trigger.entity).unwrap();
    let prop = holding.0;
    let Ok((entity, non_pickup_mass, has_held_marker)) = q_prop.get_mut(prop) else {
        error!("Prop entity was deleted or in an invalid state. Ignoring.");
        return;
    };
    if !has_held_marker {
        error!(
            "A held prop that is no longer being held was not actually marked as held. This is supremely weird. Ignoring."
        );
        return;
    }
    commands.entity(prop).remove::<HeldProp>();
    if let Some(non_pickup_mass) = non_pickup_mass {
        commands.entity(entity).try_insert(non_pickup_mass.0);
        commands.entity(entity).remove::<NonPickupMass>();
    } else {
        commands.entity(entity).remove::<Mass>();
    }
}
