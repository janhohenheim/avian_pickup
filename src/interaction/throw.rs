use std::ops::RangeInclusive;

use avian3d::math::Scalar;
use rand::Rng;

use crate::{math::GetBestGlobalTransform, prelude::*, rng::RngSource, verb::Throwing};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(PhysicsSchedule, throw.in_set(HandleVerbSystem::Throw));
}

/// Note: in constrast to the physcannon, we do not allow punting when not
/// holding any prop. I think this should be handled by the user.
fn throw(
    mut commands: Commands,
    mut q_actor: Query<(
        Entity,
        &AvianPickupActor,
        &mut AvianPickupActorState,
        &mut Cooldown,
        &Throwing,
    )>,
    q_actor_transform: Query<(&GlobalTransform, Option<&Position>, Option<&Rotation>)>,
    mut q_prop: Query<(
        &mut LinearVelocity,
        &mut AngularVelocity,
        &Mass,
        &Position,
        Option<&ThrownLinearSpeedOverride>,
        Option<&ThrownAngularSpeedOverride>,
    )>,
    mut w_throw_event: EventWriter<PropThrown>,
    mut rng: ResMut<RngSource>,
) {
    for (actor, config, mut states, mut cooldown, throw) in q_actor.iter_mut() {
        let prop = throw.0;
        commands.entity(actor).remove::<Throwing>();
        let actor_transform = q_actor_transform.get_best_global_transform(actor);
        // Safety: All props are rigid bodies, which are guaranteed to have a
        // `Position`.
        let (mut velocity, mut angvel, mass, prop_position, lin_speed_override, ang_speed_override) =
            q_prop.get_mut(prop).unwrap();
        let prop_dist_sq = actor_transform
            .translation
            .distance_squared(prop_position.0);
        if prop_dist_sq > config.interaction_distance * config.interaction_distance {
            // Note: I don't think this will ever happen, but the 2013 code
            // does this check, so let's keep it just in case.
            continue;
        }

        let lin_direction = actor_transform.forward();
        let lin_speed = lin_speed_override
            .map(|s| s.0)
            .unwrap_or_else(|| calculate_launch_speed(config, *mass));
        velocity.0 = lin_direction * lin_speed;

        let rand_direction = random_unit_vector(rng.as_mut());
        let rand_magnitude = ang_speed_override.map(|s| s.0).unwrap_or_else(|| {
            rng.as_mut()
                .gen_range(config.throw.angular_speed_range.clone())
        });
        angvel.0 = rand_direction * rand_magnitude;

        *states = AvianPickupActorState::Idle;
        w_throw_event.send(PropThrown { actor, prop });
        cooldown.throw();
    }
}

fn random_unit_vector(rng: &mut impl Rng) -> Vec3 {
    Sphere::new(1.0).sample_boundary(rng)
}

/// Corresponds to 2013's Pickup_DefaultPhysGunLaunchVelocity
fn calculate_launch_speed(config: &AvianPickupActor, mass: Mass) -> Scalar {
    let speed_range = &config.throw.linear_speed_range;
    let (min_speed, max_speed) = (*speed_range.start(), *speed_range.end());
    if mass.0 < config.throw.cutoff_mass_for_slowdown {
        max_speed
    } else {
        remap_through_spline(
            mass.0,
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
    use rand::thread_rng;

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
        let mut rng = thread_rng();
        for _ in 0..1000 {
            let v = random_unit_vector(&mut rng);
            assert!(v.is_normalized());
        }
    }
}
