use crate::{prelude::*, pull_object::Candidate};

/// Inspired by [`CWeaponPhysCannon::FindObjectTrace`](https://github.com/ValveSoftware/source-sdk-2013/blob/master/mp/src/game/server/hl2/weapon_physcannon.cpp#L2470)
pub(super) fn get_object_candidate(
    spatial_query: &SpatialQuery,
    origin: Transform,
    config: &AvianPickupActor,
) -> Option<Candidate> {
    const MAGIC_FACTOR_ASK_VALVE: f32 = 4.0 * METERS_PER_HAMMER_UNIT;
    let test_length = config.trace_length * MAGIC_FACTOR_ASK_VALVE;
    let hit = spatial_query.cast_ray(
        origin.translation,
        origin.forward(),
        test_length,
        true,
        config.spatial_query_filter.clone(),
    );

    if let Some(hit) = hit {
        Candidate {
            entity: hit.entity,
            toi: hit.time_of_impact,
        }
        .into()
    } else {
        let fake_aabb_because_parry_cannot_do_aabb_casts =
            Cuboid::from_size(Vec3::splat(MAGIC_FACTOR_ASK_VALVE * 2.)).into();
        let hit = spatial_query.cast_shape(
            &fake_aabb_because_parry_cannot_do_aabb_casts,
            origin.translation,
            origin.rotation,
            origin.forward(),
            test_length,
            false,
            config.spatial_query_filter.clone(),
        );
        if let Some(hit) = hit {
            Candidate {
                entity: hit.entity,
                toi: hit.time_of_impact,
            }
            .into()
        } else {
            None
        }
    }
}
