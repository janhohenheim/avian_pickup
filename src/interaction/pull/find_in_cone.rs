use super::Prop;
use crate::{math::METERS_PER_INCH, prelude::*};

/// Inspired by [`CWeaponPhysCannon::FindObjectInCone`](https://github.com/ValveSoftware/source-sdk-2013/blob/master/mp/src/game/server/hl2/weapon_physcannon.cpp#L2690)
pub(super) fn find_prop_in_cone(
    spatial_query: &SpatialQuery,
    origin: Transform,
    config: &AvianPickupActor,
    q_position: &Query<&Position>,
    q_rigid_body: &Query<&RigidBody>,
    q_collider_parent: &Query<&ColliderParent>,
) -> Option<Prop> {
    let is_dynamic = |entity: Entity| {
        q_rigid_body
            .get(entity)
            .is_ok_and(|rigid_body| rigid_body.is_dynamic())
    };

    const MAGIC_OFFSET_ASK_VALVE: f32 = 1.0 * METERS_PER_INCH;
    // Reminder that the actual trace is done with 4 times the
    // configured trace length in the 2013 code, eek
    let mut nearest_dist = config.interaction_distance + MAGIC_OFFSET_ASK_VALVE;
    let box_collider = Cuboid::from_size(Vec3::splat(2.0 * nearest_dist)).into();

    let rigid_bodies = spatial_query
        .shape_intersections(
            &box_collider,
            origin.translation,
            origin.rotation,
            &config.prop_filter,
        )
        .into_iter()
        .filter_map(|entity| q_collider_parent.get(entity).ok())
        .map(|collider_parent| collider_parent.get())
        .filter(|entity| is_dynamic(*entity))
        .collect::<Vec<_>>();
    let mut canditate = None;

    for rigid_body in rigid_bodies {
        // Safety: Pretty sure a `shape_intersection` will never return an entity without a `Position`.
        let object_translation = q_position.get(rigid_body).unwrap().0;

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
        if let Some(hit) =
            spatial_query.cast_ray(origin.translation, los, dist, true, &config.obstacle_filter)
        {
            let hit_rigid_body = q_collider_parent
                .get(hit.entity)
                .map_or(hit.entity, |collider_parent| collider_parent.get());
            if hit_rigid_body != rigid_body {
                continue;
            }
        }

        nearest_dist = dist;
        canditate.replace(Prop {
            entity: rigid_body,
            toi: dist,
        });
    }
    canditate
}
