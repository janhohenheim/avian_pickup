use std::ops::RangeInclusive;

use avian3d::math::Scalar;
use rand::Rng;

use crate::{prelude::*, rng::RngSource, verb::Throwing};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(PhysicsSchedule, throw.in_set(HandleVerbSystem::Throw));
}

/// Note: in constrast to the physcannon, we do not allow punting when not
/// holding any prop. I think this should be handled by the user.
fn throw(
    mut commands: Commands,
    mut q_actor: Query<(
        Entity,
        &GlobalTransform,
        &AvianPickupActor,
        &mut AvianPickupActorState,
        &mut Cooldown,
        &Throwing,
    )>,
    mut q_prop: Query<(
        &mut LinearVelocity,
        &mut AngularVelocity,
        &ComputedMass,
        Option<&ThrownLinearSpeedOverride>,
        Option<&ThrownAngularSpeedOverride>,
    )>,
    mut w_throw_event: MessageWriter<PropThrown>,
    mut rng: ResMut<RngSource>,
) {
    for (actor, actor_transform, config, mut states, mut cooldown, throw) in q_actor.iter_mut() {
        let actor_transform = actor_transform.compute_transform();
        let prop = throw.0;
        commands.entity(actor).remove::<Throwing>();
        // Safety: All props are rigid bodies, which are guaranteed to have a
        // `LinearVelocity`, `AngularVelocity`, and `Mass`.
        let Ok((mut velocity, mut angvel, mass, lin_speed_override, ang_speed_override)) =
            q_prop.get_mut(prop)
        else {
            error!("Prop entity was deleted or in an invalid state. Ignoring.");
            continue;
        };
        // The 2013 code now does a `continue` on
        // `prop_dist_sq > config.interaction_distance * config.interaction_distance`
        // but eh, that's fine. Better to respect players' input in such edge cases.

        let lin_direction = actor_transform.forward();
        let lin_speed = lin_speed_override
            .map(|s| s.0)
            .unwrap_or_else(|| calculate_launch_speed(config, *mass));
        velocity.0 = lin_direction * lin_speed;

        let rand_direction = random_unit_vector(rng.as_mut());
        let rand_magnitude = ang_speed_override.map(|s| s.0).unwrap_or_else(|| {
            rng.as_mut()
                .random_range(config.throw.angular_speed_range.clone())
        });
        angvel.0 = rand_direction * rand_magnitude;

        *states = AvianPickupActorState::Idle;
        w_throw_event.write(PropThrown { actor, prop });
        cooldown.throw();
    }
}

fn random_unit_vector(rng: &mut impl Rng) -> Vec3 {
    Sphere::new(1.0).sample_boundary(rng)
}

/// Corresponds to 2013's Pickup_DefaultPhysGunLaunchVelocity
fn calculate_launch_speed(config: &AvianPickupActor, mass: ComputedMass) -> Scalar {
    let speed_range = &config.throw.linear_speed_range;
    let (min_speed, max_speed) = (*speed_range.start(), *speed_range.end());
    if mass.value() < config.throw.cutoff_mass_for_slowdown {
        max_speed
    } else {
        remap_through_spline(
            mass.value(),
            config.throw.cutoff_mass_for_slowdown..=config.pull.max_prop_mass,
            max_speed..=min_speed,
        )
    }
}

/// Remaps a value `val` in range `domain` from linear
/// to spline using `simple_spline`, giving an output in `image`.
/// `domain` and `image` are mathematical terms.
///
/// Corresponds to 2013's `SimpleSplineRemapValClamped`
fn remap_through_spline(
    val: Scalar,
    domain: RangeInclusive<Scalar>,
    image: RangeInclusive<Scalar>,
) -> Scalar {
    let (a, b) = (*domain.start(), *domain.end());
    let (c, d) = (*image.start(), *image.end());
    if a == b {
        return if val >= b { d } else { c };
    }
    let mut c_val = (val - a) / (b - a);
    c_val = c_val.clamp(0.0, 1.0);
    c + (d - c) * simple_spline(c_val)
}

/// Hermite basis function for smooth interpolation
/// Assumes `value` is between 0 and 1 inclusive.
/// Corresponds to 2013's `SimpleSpline`
fn simple_spline(value: f32) -> f32 {
    let value_2 = value * value;
    let value_3 = value_2 * value;
    -2.0 * value_3 + 3.0 * value_2
}

#[cfg(test)]
mod test {
    use rand::rng;

    use super::*;

    // USES INCHES!
    const MINFORCE: f32 = 700.0;
    const MAXFORCE: f32 = 1500.0;

    #[test]
    fn test_remap_through_spline() {
        let remap = |val: f32| remap_through_spline(val, 100.0..=600., MAXFORCE..=MINFORCE);
        // The speed we can muster is lower the heavier the object is.
        assert_eq!(remap(100.), MAXFORCE);
        assert_eq!(remap(600.), MINFORCE);
        assert_eq!(remap(350.), 1100.0);
        assert_eq!(remap(1000.), MINFORCE);
    }

    #[test]
    fn is_random_unit_vector_actually_unit() {
        let mut rng = rng();
        // What?! A unit test that uses randomness?
        // In this part of the codebase, localized entirely within your for-loop?
        // Yes.
        for _ in 0..1000 {
            let v = random_unit_vector(&mut rng);
            assert!(v.is_normalized());
        }
    }
}
