use crate::prelude::*;

pub(super) struct Candidate {
    pub entity: Entity,
    pub toi: f32,
    pub toi_fraction: f32,
}

/// Inspired by [`CWeaponPhysCannon::FindObjectTrace`](https://github.com/ValveSoftware/source-sdk-2013/blob/master/mp/src/game/server/hl2/weapon_physcannon.cpp#L2470)
pub(super) fn get_object_candidate(
    spatial_query: &SpatialQuery,
    origin: Transform,
    config: &AvianPickupActor,
) -> Option<Candidate> {
    const MAGIC_NUMBER_ASK_VALVE: f32 = 4.0;
    let test_length = config.trace_length * MAGIC_NUMBER_ASK_VALVE;
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
            toi_fraction: hit.time_of_impact / test_length,
        }
        .into()
    } else {
        let fake_aabb_because_parry_cannot_do_aabb_casts =
            Cuboid::from_size(Vec3::splat(MAGIC_NUMBER_ASK_VALVE * 2.)).into();
        let hit = spatial_query.cast_shape(
            &fake_aabb_because_parry_cannot_do_aabb_casts,
            origin.translation,
            Quat::IDENTITY,
            origin.forward(),
            test_length,
            false,
            config.spatial_query_filter.clone(),
        );
        if let Some(hit) = hit {
            Candidate {
                entity: hit.entity,
                toi: hit.time_of_impact,
                toi_fraction: hit.time_of_impact / test_length,
            }
            .into()
        } else {
            None
        }
    }
}
