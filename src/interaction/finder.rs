use crate::{math::METERS_PER_INCH, prelude::*};
use avian3d::math::Scalar;
use bevy_ecs::{prelude::*, relationship::Relationship, system::SystemParam};

/// A [`SystemParam`] to find prop in front of the entity.
///
/// you can use these method method on it to get dynamic [`RigidBody`] forward an [`Entity`].
#[derive(SystemParam)]
pub struct PropFinder<'w, 's> {
    spatial_query: SpatialQuery<'w, 's>,
    q_collider_parent: Query<'w, 's, &'static ColliderOf>,
    q_rigid_body: Query<'w, 's, (&'static RigidBody, &'static GlobalTransform)>,
}

/// Result of an [`PropFinder`]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FindProp {
    /// Entity find
    pub entity: Entity,
    /// Distance between entity found and origin
    pub toi: Scalar,
}

impl<'w, 's> PropFinder<'w, 's> {
    /// Use [`PropFinder::find_prop_in_trace`] and fallback to [`PropFinder::find_prop_in_cone`].
    ///
    /// Inspired by [`CWeaponPhysCannon::FindObject`](https://github.com/ValveSoftware/source-sdk-2013/blob/master/src/game/server/hl2/weapon_physcannon.cpp#L2497)
    pub fn find_prop(
        &self,
        origin: GlobalTransform,
        config: &AvianPickupActor,
    ) -> Option<FindProp> {
        self.find_prop_in_trace(origin, config)
            .or_else(|| self.find_prop_in_cone(origin, config))
    }

    /// Find prop by casting a ray forward
    pub fn find_prop_in_trace(
        &self,
        origin: GlobalTransform,
        config: &AvianPickupActor,
    ) -> Option<FindProp> {
        // Fun fact: Valve lies to you and actually multiplies this by 4 at this point.
        let max_distance: Scalar = config.interaction_distance;

        let origin_pos = origin.translation();
        let origin_forward = origin.forward();

        let is_dynamic = |collider: Entity| self.dynamic_rigid_body(collider).is_some();

        let is_visible = |prop_body: Entity, distance: Scalar| {
            self.spatial_query
                .cast_ray(
                    origin_pos,
                    origin_forward,
                    distance,
                    true,
                    &config.obstacle_filter,
                )
                .is_none_or(|obstacle| self.rigid_body_entity(obstacle.entity) == prop_body)
        };

        if let Some(hit) = self.spatial_query.cast_ray_predicate(
            origin_pos,
            origin_forward,
            max_distance,
            true,
            &config.prop_filter,
            &is_dynamic,
        ) {
            let prop_body = self.rigid_body_entity(hit.entity);

            if is_visible(prop_body, hit.distance) {
                return Some(FindProp {
                    entity: prop_body,
                    toi: hit.distance,
                });
            }
        }

        // Forgiving half-entend of 4 inches in the 2013 code, which is about 1 cm
        const PICKUP_HALF_EXTEND: Scalar = 0.01;
        let pickup_box = Cuboid::from_size(Vec3::splat(PICKUP_HALF_EXTEND * 2.0)).into();

        let hit = self.spatial_query.cast_shape_predicate(
            &pickup_box,
            origin_pos,
            origin.rotation(),
            origin_forward,
            &ShapeCastConfig::from_max_distance(max_distance),
            &config.prop_filter,
            &is_dynamic,
        )?;

        let prop_body = self.rigid_body_entity(hit.entity);

        if !is_visible(prop_body, hit.distance) {
            return None;
        }

        Some(FindProp {
            entity: prop_body,
            toi: hit.distance,
        })
    }

    /// Find prop by casting a cone forward
    pub fn find_prop_in_cone(
        &self,
        origin: GlobalTransform,
        config: &AvianPickupActor,
    ) -> Option<FindProp> {
        const MAGIC_OFFSET_ASK_VALVE: Scalar = METERS_PER_INCH;
        // Reminder that the actual trace is done with 4 times the
        // configured trace length in the 2013 code, eek
        let mut nearest_dist = config.interaction_distance + MAGIC_OFFSET_ASK_VALVE;
        let box_collider = Cuboid::from_size(Vec3::splat(2.0 * nearest_dist)).into();
        let origin_pos = origin.translation();

        let mut candidate = None;

        for collider in self.spatial_query.shape_intersections(
            &box_collider,
            origin_pos,
            origin.rotation(),
            &config.prop_filter,
        ) {
            let Some((rigid_body, transform)) = self.dynamic_rigid_body(collider) else {
                continue;
            };

            let line_of_sight = transform.translation() - origin_pos;
            let distance_squared: Scalar = line_of_sight.length_squared();

            if distance_squared >= nearest_dist * nearest_dist {
                continue;
            }

            let Ok((direction, distance)) = Dir3::new_and_length(line_of_sight) else {
                continue;
            };

            if direction.dot(origin.forward().into()) <= config.interaction_cone {
                continue;
            }

            let occulded = self
                .spatial_query
                .cast_ray(
                    origin_pos,
                    direction,
                    distance,
                    true,
                    &config.obstacle_filter,
                )
                .is_some_and(|hit| self.rigid_body_entity(hit.entity) != rigid_body);

            if occulded {
                continue;
            }

            nearest_dist = distance;
            candidate = Some(FindProp {
                entity: rigid_body,
                toi: distance,
            })
        }

        candidate
    }

    fn rigid_body_entity(&self, collider: Entity) -> Entity {
        self.q_collider_parent
            .get(collider)
            .map_or(collider, ColliderOf::get)
    }

    fn dynamic_rigid_body(&self, collider: Entity) -> Option<(Entity, &GlobalTransform)> {
        let entity = self.rigid_body_entity(collider);

        let (body, transform) = self.q_rigid_body.get(entity).ok()?;

        body.is_dynamic().then_some((entity, transform))
    }
}
