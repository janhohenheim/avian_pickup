use super::Prop;
use crate::{math::METERS_PER_INCH, prelude::*};

/// Inspired by [`CWeaponPhysCannon::FindObjectInCone`](https://github.com/ValveSoftware/source-sdk-2013/blob/master/mp/src/game/server/hl2/weapon_physcannon.cpp#L2690)
pub(super) fn find_prop_in_cone(
    spatial_query: &SpatialQuery,
    origin: Transform,
    config: &AvianPickupActor,
    q_transform: &Query<&GlobalTransform>,
) -> Option<Prop> {
    const MAGIC_OFFSET_ASK_VALVE: f32 = 1.0 * METERS_PER_INCH;
    let mut nearest_dist = config.trace_length + MAGIC_OFFSET_ASK_VALVE;
    let box_collider = Cuboid::from_size(Vec3::splat(2.0 * nearest_dist)).into();

    let colliders = spatial_query.shape_intersections(
        &box_collider,
        origin.translation,
        origin.rotation,
        &config.spatial_query_filter,
    );
    let mut canditate = None;

    for collider in colliders {
        // Unwrap cannot fail: colliders are guarateed to have a `GlobalTransform`
        let object_translation = q_transform.get(collider).unwrap().translation();

        // Closer than other objects
        let los = object_translation - origin.translation;
        let (los, dist) = Dir3::new_and_length(los).expect("Failed to normalize line of sight");
        if dist >= nearest_dist {
            continue;
        }

        // Cull to the cone
        let max_dot = config.cone;
        if los.dot(origin.forward().into()) <= max_dot {
            continue;
        }

        // Make sure it isn't occluded!
        if let Some(hit) = spatial_query.cast_ray(
            origin.translation,
            los,
            dist,
            true,
            &config.spatial_query_filter,
        ) {
            if hit.entity == collider {
                nearest_dist = dist;
                canditate.replace(Prop {
                    entity: collider,
                    toi: hit.time_of_impact,
                });
            }
        }
    }
    canditate
}
