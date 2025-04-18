use super::Prop;
use crate::{math::METERS_PER_INCH, prelude::*};

/// Inspired by [`CWeaponPhysCannon::FindObjectInCone`](https://github.com/ValveSoftware/source-sdk-2013/blob/master/mp/src/game/server/hl2/weapon_physcannon.cpp#L2690)
pub(super) fn find_prop_in_cone(
    spatial_query: &SpatialQuery,
    origin: Transform,
    config: &AvianPickupActor,
    q_rigid_body: &Query<(&RigidBody, &GlobalTransform)>,
    q_collider_of: &Query<&ColliderOf>,
) -> Option<Prop> {
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
        .filter_map(|entity| q_collider_of.get(entity).ok())
        .map(|collider_of| collider_of.rigid_body)
        .filter_map(|entity| {
            let (rigid_body, transform) = q_rigid_body.get(entity).ok()?;
            if rigid_body.is_dynamic() {
                let transform = transform.compute_transform();
                Some((entity, transform))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    let mut canditate = None;

    for (rigid_body, transform) in rigid_bodies {
        let object_translation = transform.translation;

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
            let hit_rigid_body = q_collider_of
                .get(hit.entity)
                .map_or(hit.entity, |collider_of| collider_of.rigid_body);
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
