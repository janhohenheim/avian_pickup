use std::os::unix::raw::time_t;

use bevy::render::render_resource::encase::rts_array::Length;

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.get_schedule_mut(PhysicsSchedule)
        .unwrap()
        .add_systems(hold.in_set(AvianPickupSystem::HoldObject));
}

fn hold(q_actor: Query<(&AvianPickupActorState, &GlobalTransform)>) {
    for (&state, transform) in q_actor.iter() {
        let AvianPickupActorState::Holding(_entity) = state else {
            continue;
        };
        let _transform = transform.compute_transform();
        info!("Hold!")
    }
}

#[derive(Debug, Copy, Clone, Component)]
struct ShadowParams {
    target_position: Vec3,
    target_rotation: Quat,
    max_angular: f32,
    max_damp_angular: f32,
    max_speed: f32,
    max_damp_speed: f32,
    // damp_factor = 1
    // teleport_distance = 0
}

#[derive(Debug, Copy, Clone, Component)]
struct GrabParams {
    contact_amount: f32,
    time_to_arrive: f32,
    error_time: f32,
}

fn grabcontroller_simulate(
    time: Res<Time>,
    mut q_object: Query<(
        &ShadowParams,
        &Mass,
        &mut LinearVelocity,
        &AngularVelocity,
        &mut GrabParams,
    )>,
) {
    let dt = time.delta_seconds();
    for (shadow, mass, mut velocity, angvel, mut grab) in q_object.iter_mut() {
        // imo InContactWithHeavyObject will always be false,
        // as we are effectively asking "is the current object heavier than the
        // current object?"
        // TODO: make this smooth_nudge
        grab.contact_amount = grab.contact_amount.lerp(1.0, dt * 2.0);
        let mut shadow = *shadow;
        shadow.max_angular *= grab.contact_amount * grab.contact_amount * grab.contact_amount;

        grab.time_to_arrive = compute_shadow_control(shadow, grab.time_to_arrive, dt);

        // Slide along the current contact points to fix bouncing problems
        *velocity = phys_compute_slide_direction(*velocity, *angvel, *mass);
        grab.error_time += dt;
    }
}

/*
IMotionEvent::simresult_e CGrabController::Simulate( IPhysicsMotionController *pController, IPhysicsObject *pObject, float deltaTime, Vector &linear, AngularImpulse &angular )
{
    game_shadowcontrol_params_t shadowParams = m_shadow;
    if ( InContactWithHeavyObject( pObject, GetLoadWeight() ) )
    {
        m_contactAmount = Approach( 0.1f, m_contactAmount, deltaTime*2.0f );
    }
    else
    {
        m_contactAmount = Approach( 1.0f, m_contactAmount, deltaTime*2.0f );
    }
    shadowParams.maxAngular = m_shadow.maxAngular * m_contactAmount * m_contactAmount * m_contactAmount;
    m_timeToArrive = pObject->ComputeShadowControl( shadowParams, m_timeToArrive, deltaTime );

    // Slide along the current contact points to fix bouncing problems
    Vector velocity;
    AngularImpulse angVel;
    pObject->GetVelocity( &velocity, &angVel );
    PhysComputeSlideDirection( pObject, velocity, angVel, &velocity, &angVel, GetLoadWeight() );
    pObject->SetVelocityInstantaneous( &velocity, NULL );

    linear.Init();
    angular.Init();
    m_errorTime += deltaTime;

    return SIM_LOCAL_ACCELERATION;
}

*/

fn compute_shadow_control(shadow: ShadowParams, seconds_to_arrival: f32, dt: f32) -> f32 {
    todo!()
}
/*
float JoltPhysicsObject::ComputeShadowControl( const hlshadowcontrol_params_t &params, float flSecondsToArrival, float flDeltaTime )
{
    JoltShadowControlParams joltParams =
    {
        .TargetPosition		= SourceToJolt::Distance( params.targetPosition ),
        .TargetRotation		= SourceToJolt::Angle( params.targetRotation ),
        .MaxAngular			= SourceToJolt::Angle( params.maxAngular ),
        .MaxDampAngular		= SourceToJolt::Angle( params.maxDampAngular ),
        .MaxSpeed			= SourceToJolt::Distance( params.maxSpeed ),
        .MaxDampSpeed		= SourceToJolt::Distance( params.maxDampSpeed ),
        .DampFactor			= params.dampFactor,
        .TeleportDistance	= SourceToJolt::Distance( params.teleportDistance ),
    };

    JPH::BodyInterface& bodyInterface = m_pPhysicsSystem->GetBodyInterfaceNoLock();

    JPH::Vec3 position;
    JPH::Quat rotation;
    bodyInterface.GetPositionAndRotation( m_pBody->GetID(), position, rotation );
    JPH::Vec3 linearVelocity;
    JPH::Vec3 angularVelocity;
    bodyInterface.GetLinearAndAngularVelocity( m_pBody->GetID(), linearVelocity, angularVelocity );

    JPH::Vec3 scratchPosition = position;
    JPH::Quat scratchRotation = rotation;
    JPH::Vec3 scratchLinearVelocity = linearVelocity;
    JPH::Vec3 scratchAngularVelocity = angularVelocity;
    float flNewSecondsToArrival =
        ComputeShadowController( joltParams, scratchPosition, scratchRotation, scratchLinearVelocity, scratchAngularVelocity, flSecondsToArrival, flDeltaTime );

    if ( scratchPosition != position || scratchRotation != rotation )
        bodyInterface.SetPositionAndRotation( m_pBody->GetID(), scratchPosition, scratchRotation, JPH::EActivation::Activate );

    if ( scratchLinearVelocity != linearVelocity || scratchAngularVelocity != angularVelocity )
        bodyInterface.SetLinearAndAngularVelocity( m_pBody->GetID(), scratchLinearVelocity, scratchAngularVelocity );

    return flNewSecondsToArrival;
}
 */

fn phys_compute_slide_direction(
    velocity: LinearVelocity,
    angular_velocity: AngularVelocity,
    min_mass: Mass,
) -> LinearVelocity {
    // No need to return angular velocity, as we are not using it
    todo!()
}

/*

void PhysComputeSlideDirection( IPhysicsObject *pPhysics, const Vector &inputVelocity, const AngularImpulse &inputAngularVelocity,
                               Vector *pOutputVelocity, Vector *pOutputAngularVelocity, float minMass )
{
    Vector velocity = inputVelocity;
    AngularImpulse angVel = inputAngularVelocity;
    Vector pos;

    IPhysicsFrictionSnapshot *pSnapshot = pPhysics->CreateFrictionSnapshot();
    while ( pSnapshot->IsValid() )
    {
        IPhysicsObject *pOther = pSnapshot->GetObject( 1 );
        if ( !pOther->IsMoveable() || pOther->GetMass() > minMass )
        {
            Vector normal;
            pSnapshot->GetSurfaceNormal( normal );

            // BUGBUG: Figure out the correct rotation clipping equation
            if ( pOutputAngularVelocity )
            {
                angVel = normal * DotProduct( angVel, normal );
#if 0
                pSnapshot->GetContactPoint( point );
                Vector point, dummy;
                AngularImpulse angularClip, clip2;

                pPhysics->CalculateVelocityOffset( normal, point, dummy, angularClip );
                VectorNormalize( angularClip );
                float proj = DotProduct( angVel, angularClip );
                if ( proj > 0 )
                {
                    angVel -= angularClip * proj;
                }
                CrossProduct( angularClip, normal, clip2 );
                proj = DotProduct( angVel, clip2 );
                if ( proj > 0 )
                {
                    angVel -= clip2 * proj;
                }
                //NDebugOverlay::Line( point, point - normal * 20, 255, 0, 0, true, 0.1 );
#endif
            }

            // Determine how far along plane to slide based on incoming direction.
            // NOTE: Normal points away from this object
            float proj = DotProduct( velocity, normal );
            if ( proj > 0.0f )
            {
                velocity -= normal * proj;
            }
        }
        pSnapshot->NextFrictionData();
    }
    pPhysics->DestroyFrictionSnapshot( pSnapshot );

    //NDebugOverlay::Line( pos, pos + unitVel * 20, 0, 0, 255, true, 0.1 );

    if ( pOutputVelocity )
    {
        *pOutputVelocity = velocity;
    }
    if ( pOutputAngularVelocity )
    {
        *pOutputAngularVelocity = angVel;
    }
}
 */

fn compute_controller_2003(current_velocity: &mut Vec3, delta: Vec3, max_speed: f32) {
    if current_velocity.length_squared() < 1e-6 {
        *current_velocity = Vec3::ZERO;
    }
    let mut acceleration = delta.to_array();
    for i in 0..3 {
        acceleration[i] = acceleration[i].clamp(-max_speed, max_speed);
    }
    *current_velocity = Vec3::from(acceleration);
}
