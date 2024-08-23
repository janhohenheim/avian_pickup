use crate::{prelude::*, verb::Holding};

pub(super) fn plugin(app: &mut App) {
    app.observe(on_remove_holding);
}

fn on_remove_holding(
    trigger: Trigger<OnRemove, Holding>,
    mut commands: Commands,
    q_actor: Query<&Holding>,
    mut q_prop: Query<(&mut Mass, Option<&NonPickupMass>, Has<HeldProp>)>,
) {
    let holding = q_actor.get(trigger.entity()).unwrap();
    let prop = holding.0;
    // Safety: All props are rigid bodies, so they are guaranteed to have a `Mass`.
    let (mut mass, non_pickup_mass, has_held_marker) = q_prop.get_mut(prop).unwrap();
    if !has_held_marker {
        error!(
            "A held prop that is no longer being held was not actually marked as held. This is supremely weird. Ignoring."
        );
        return;
    }
    commands.entity(prop).remove::<HeldProp>();
    let Some(non_pickup_mass) = non_pickup_mass else {
        error!(
            "A held prop that is no longer being held failed to get its pre-pickup mass back. Ignoring."
        );
        return;
    };
    mass.0 = non_pickup_mass.0;
}
