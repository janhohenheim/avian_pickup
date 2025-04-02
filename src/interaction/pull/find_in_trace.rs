use super::Prop;
use crate::prelude::*;

/// Inspired by [`CWeaponPhysCannon::FindObjectTrace`](https://github.com/ValveSoftware/source-sdk-2013/blob/master/mp/src/game/server/hl2/weapon_physcannon.cpp#L2470)
pub(super) fn find_prop_in_trace(
    spatial_query: &SpatialQuery,
    origin: Transform,
    config: &AvianPickupActor,
    q_rigid_body: &Query<&RigidBody>,
) -> Option<Prop> {
    // Fun fact: Valve lies to you and actually multiplies this by 4 at this point.
    let test_length = config.interaction_distance;
    let is_dynamic = |entity: Entity| {
        q_rigid_body
            .get(entity)
            .is_ok_and(|rigid_body| rigid_body.is_dynamic())
    };
    let hit = spatial_query.cast_ray_predicate(
        origin.translation,
        origin.forward(),
        test_length,
        true,
        &config.prop_filter,
        &is_dynamic,
    );

    hit.filter(|hit| {
        if let Some(terrain_hit) = spatial_query.cast_ray(
            origin.translation,
            origin.forward(),
            test_length,
            true,
            &config.obstacle_filter,
        ) {
            let occluded = terrain_hit.entity != hit.entity && terrain_hit.distance <= hit.distance;
            !occluded
        } else {
            true
        }
    });

    if let Some(hit) = hit {
        Prop {
            entity: hit.entity,
            toi: hit.distance,
        }
        .into()
    } else {
        // This has a half-extent of 4 inches in the 2013 code, which is about 1 cm
        const MAGIC_HALF_EXTENT_ASK_VALVE: f32 = 0.01;
        let fake_aabb_because_parry_cannot_do_aabb_casts =
            Cuboid::from_size(Vec3::splat(2. * MAGIC_HALF_EXTENT_ASK_VALVE)).into();
        let hit = spatial_query.cast_shape_predicate(
            &fake_aabb_because_parry_cannot_do_aabb_casts,
            origin.translation,
            origin.rotation,
            origin.forward(),
            &ShapeCastConfig::from_max_distance(test_length),
            &config.prop_filter,
            &is_dynamic,
        );
        hit.filter(|hit| {
            if let Some(terrain_hit) = spatial_query.cast_shape(
                &fake_aabb_because_parry_cannot_do_aabb_casts,
                origin.translation,
                origin.rotation,
                origin.forward(),
                &ShapeCastConfig::from_max_distance(hit.distance),
                &config.obstacle_filter,
            ) {
                terrain_hit.entity == hit.entity
            } else {
                true
            }
        })
        .map(|hit| Prop {
            entity: hit.entity,
            toi: hit.distance,
        })
    }
}
