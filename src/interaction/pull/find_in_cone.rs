use super::Prop;
use crate::{math::METERS_PER_INCH, prelude::*};

/// Inspired by [`CWeaponPhysCannon::FindObjectInCone`](https://github.com/ValveSoftware/source-sdk-2013/blob/master/mp/src/game/server/hl2/weapon_physcannon.cpp#L2690)
pub(super) fn find_prop_in_cone(
    spatial_query: &SpatialQuery,
    origin: Transform,
    config: &AvianPickupActor,
    q_collider: &Query<&Position>,
) -> Option<Prop> {
    const MAGIC_OFFSET_ASK_VALVE: f32 = 1.0 * METERS_PER_INCH;
    // Valve uses the trace length here, but imo using the hold distance makes more
    // sense, as the raw trace length is what is also used for the hold check in
    // the 2013 code. (Reminder that the actual trace is done with 4 times the
    // configured trace length, eek)
    let mut nearest_dist = config.hold.distance_to_allow_holding + MAGIC_OFFSET_ASK_VALVE;
    let box_collider = Cuboid::from_size(Vec3::splat(2.0 * nearest_dist)).into();

    let colliders = spatial_query.shape_intersections(
        &box_collider,
        origin.translation,
        origin.rotation,
        &config.prop_filter,
    );
    let mut canditate = None;

    for collider in colliders {
        // Safety: Pretty sure a `shape_intersection` will never return an entity without a `Position`.
        let object_translation = q_collider.get(collider).unwrap().0;

        // Closer than other objects
        let los = object_translation - origin.translation;
        let (los, dist) = Dir3::new_and_length(los).expect("Failed to normalize line of sight");
        if dist >= nearest_dist {
            continue;
        }

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
            &|entity| q_collider.contains(entity),
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
            &|entity| q_collider.contains(entity),
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
