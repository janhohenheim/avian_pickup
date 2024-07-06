use super::Prop;
use crate::prelude::*;

/// Inspired by [`CWeaponPhysCannon::FindObjectTrace`](https://github.com/ValveSoftware/source-sdk-2013/blob/master/mp/src/game/server/hl2/weapon_physcannon.cpp#L2470)
pub(super) fn find_prop_in_trace(
    spatial_query: &SpatialQuery,
    origin: Transform,
    config: &AvianPickupActor,
) -> Option<Prop> {
    const MAGIC_FACTOR_ASK_VALVE: f32 = 4.0;
    let test_length = config.trace_length * MAGIC_FACTOR_ASK_VALVE;
    let hit = spatial_query.cast_ray(
        origin.translation,
        origin.forward(),
        test_length,
        true,
        config.spatial_query_filter.clone(),
    );

    if let Some(hit) = hit {
        Prop {
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
            Prop {
                entity: hit.entity,
                toi: hit.time_of_impact,
            }
            .into()
        } else {
            None
        }
    }
}
