use super::Prop;
use crate::{math::METERS_PER_INCH, prelude::*};

/// Inspired by [`CWeaponPhysCannon::FindObjectInCone`](https://github.com/ValveSoftware/source-sdk-2013/blob/master/mp/src/game/server/hl2/weapon_physcannon.cpp#L2690)
pub(super) fn find_prop_in_cone(
    spatial_query: &SpatialQuery,
    origin: Transform,
    config: &AvianPickupActor,
    q_position: &Query<&Position>,
    q_rigid_body: &Query<&RigidBody>,
) -> Option<Prop> {
    let is_dynamic = |entity: Entity| {
        q_rigid_body
            .get(entity)
            .is_ok_and(|rigid_body| rigid_body.is_dynamic())
    };
    let is_not_dynamic = |entity: Entity| !is_dynamic(entity);

    const MAGIC_OFFSET_ASK_VALVE: f32 = 1.0 * METERS_PER_INCH;
    // Reminder that the actual trace is done with 4 times the
    // configured trace length in the 2013 code, eek
    let mut nearest_dist = config.interaction_distance + MAGIC_OFFSET_ASK_VALVE;
    let box_collider = Cuboid::from_size(Vec3::splat(2.0 * nearest_dist)).into();

    let colliders = spatial_query
        .shape_intersections(
            &box_collider,
            origin.translation,
            origin.rotation,
            &config.prop_filter,
        )
        .into_iter()
        .filter(|entity| is_dynamic(*entity))
        .collect::<Vec<_>>();
    let mut canditate = None;

    for collider in colliders {
        // Safety: Pretty sure a `shape_intersection` will never return an entity without a `Position`.
        let object_translation = q_position.get(collider).unwrap().0;

        // Closer than other objects
        let los = object_translation - origin.translation;
        if los.length_squared() >= nearest_dist * nearest_dist {
            continue;
        }
        let (los, dist) = Dir3::new_and_length(los).expect("Failed to normalize line of sight");

        // Cull to the cone
        let max_dot = config.interaction_cone;
        if los.dot(origin.forward().into()) <= max_dot {
            continue;
        }

        // Make sure it isn't occluded by terrain
        if let Some(hit) = spatial_query.cast_ray_predicate(
            origin.translation,
            los,
            dist,
            true,
            &config.obstacle_filter,
            &is_not_dynamic,
        ) {
            let occluded = hit.entity != collider && hit.distance <= dist;
            if occluded {
                continue;
            }
        }

        // Make sure it isn't occluded by other props
        if let Some(hit) = spatial_query.cast_ray_predicate(
            origin.translation,
            los,
            dist,
            true,
            &config.prop_filter,
            &is_dynamic,
        ) {
            if hit.entity == collider {
                nearest_dist = dist;
                canditate.replace(Prop {
                    entity: collider,
                    toi: hit.distance,
                });
            }
        }
    }
    canditate
}
