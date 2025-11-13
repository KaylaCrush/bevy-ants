use bevy::prelude::*;

pub struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_steering,update_physics));
    }
}

pub const MAX_SPEED:f32 = 200.0;
pub const MAX_FORCE:f32 = 100.0;

#[derive(Component)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct Acceleration(pub Vec2);

#[derive(Component)]
pub struct MaxSpeed(pub f32);

#[derive(Component)]
pub struct MaxForce(pub f32);


#[derive(Bundle)]
pub struct PhysicsObject {
    pub velocity: Velocity,
    pub acceleration: Acceleration,
    pub max_speed: MaxSpeed,
    pub max_force: MaxForce,
}

pub fn update_physics(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Velocity, &mut Acceleration, &MaxSpeed), With<crate::ants::Ant>>,
) {
    for (mut transform, mut velocity, mut accel, max_speed) in &mut query {
        velocity.0 += accel.0 * time.delta_secs();
        velocity.0 = velocity.0.clamp_length_max(max_speed.0);
        transform.translation += velocity.0.extend(0.0) * time.delta_secs();

        // rotate to face velocity
        if velocity.0.length_squared() > 0.0001 {
            let angle = velocity.0.y.atan2(velocity.0.x);
            transform.rotation = Quat::from_rotation_z(angle);
        }

        accel.0 = Vec2::ZERO;
    }
}

#[derive(Component)]
pub struct Target(pub Vec2);

pub fn update_steering(
    mut query: Query<(&mut Acceleration, &Velocity, &Transform, &MaxSpeed, &MaxForce, &Target)>,
) {
    for (mut accel, velocity, transform, max_speed, max_force, target) in &mut query {
        let desired = (target.0-transform.translation.truncate()).normalize_or_zero() * max_speed.0;
        let steer = (desired-velocity.0).normalize_or_zero()*max_force.0;
        accel.0 = accel.0+steer;
    }
}
