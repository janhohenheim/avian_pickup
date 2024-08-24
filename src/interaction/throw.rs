use std::ops::RangeInclusive;

use avian3d::math::Scalar;
use rand::Rng as _;

use crate::{math::GetBestGlobalTransform, prelude::*, rng::RngSource, verb::Throwing};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(PhysicsSchedule, throw.in_set(HandleVerbSystem::Throw));
}

/// DetachObject
fn throw(
    mut commands: Commands,
    mut q_actor: Query<(Entity, &AvianPickupActor, &mut Cooldown, &Throwing)>,
    q_actor_transform: Query<(&GlobalTransform, Option<&Position>, Option<&Rotation>)>,
    mut q_prop: Query<(
        &mut LinearVelocity,
        &mut AngularVelocity,
        &mut ExternalImpulse,
        &mut ExternalAngularImpulse,
        &Position,
    )>,
    mut w_throw_event: EventWriter<PropThrown>,
    mut rng: ResMut<RngSource>,
) {
    for (actor, config, mut cooldown, throw) in q_actor.iter_mut() {
        let prop = throw.0;
        info!("Throw!");
        commands.entity(actor).remove::<Throwing>();
        if let Some(prop) = prop {
            let actor_transform = q_actor_transform.get_best_global_transform(actor);
            // Safety: All props are rigid bodies, which are guaranteed to have a
            // `Position`.
            let (mut velocity, mut angvel, mut lin_impulse, mut ang_impulse, prop_position) =
                q_prop.get_mut(prop).unwrap();
            let prop_dist_sq = actor_transform
                .translation
                .distance_squared(prop_position.0);
            if prop_dist_sq > config.interaction_distance * config.interaction_distance {
                // Note: I don't think this will ever happen, but the 2013 code
                // does this check, so let's keep it just in case.
                continue;
            }

            velocity.0 = Vec3::ZERO;
            angvel.0 = Vec3::ZERO;

            let direction = actor_transform.forward();
            //let impulse = direction * config.throw.max_linear_velocity;
            //lin_impulse.apply_impulse(impulse);
            let rand_direction = Sphere::new(1.0).sample_boundary(rng.as_mut());
            let rand_magnitude = rng
                .as_mut()
                .gen_range(config.throw.angular_velocity_range.clone());
            let torque = rand_direction * rand_magnitude;
            ang_impulse.apply_impulse(torque);

            w_throw_event.send(PropThrown {
                actor,
                prop,
                was_held: true,
            });
        } else {
            // YTODO: eet next object in front of us

            w_throw_event.send(PropThrown {
                actor,
                prop: Entity::PLACEHOLDER,
                was_held: false,
            });
        }

        // TODO: only CD when we actually threw something
        cooldown.throw();
    }
}

/// Corresponds to 2013's Pickup_DefaultPhysGunLaunchVelocity
fn calculate_launch_speed(config: &AvianPickupActor, mass: Mass) -> Scalar {
    todo!()
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
    use super::*;

    // USES INCHES!
    const MINFORCE: f32 = 700.0;
    const MAXFORCE: f32 = 1500.0;

    #[test]
    fn test_remap_through_spline() {
        let remap = |val: f32| remap_through_spline(val, (100.0..=600.), (MAXFORCE..=MINFORCE));
        // The speed we can muster is lower the heavier the object is.
        assert_eq!(remap(100.), MAXFORCE);
        assert_eq!(remap(600.), MINFORCE);
        assert_eq!(remap(350.), 1100.0);
        assert_eq!(remap(1000.), MINFORCE);
    }
}
