use bevy::prelude::*;

use crate::physics::{Target, Velocity, Acceleration, MaxSpeed, MaxForce, PhysicsObject, MAX_SPEED, MAX_FORCE};

const RED:Color = Color::srgb(0.5, 0.02, 0.02);

#[derive(Component)]
pub struct Ant;

#[derive(Component)]
pub struct AntennaSensors {
    pub left_offset: Vec2,
    pub right_offset: Vec2,
}

pub fn setup_ants(
    commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
) {
    spawn_ant(commands, meshes, materials, Vec2::ZERO);
}

pub fn spawn_ant(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    position: Vec2,
) {
    let size = 4.0;
    // Using the same shapes/meshes you used previously
    let body = Ellipse::new(size, size*0.5);
    let body_mesh = meshes.add(body);
    let body_material = materials.add(RED);
    let left_antenna = Vec2::new(size*1.25, size*0.5);
    let right_antenna = Vec2::new(size*1.25, -size * 0.5);

    commands
        .spawn((
            Ant,
            PhysicsObject {
                velocity: Velocity(Vec2::new(20.0,20.0)),
                acceleration: Acceleration(Vec2::ZERO),
                max_speed: MaxSpeed(MAX_SPEED),
                max_force: MaxForce(MAX_FORCE),
            },
            AntennaSensors {
                left_offset: left_antenna,
                right_offset: right_antenna,
            },
            Target(Vec2::ZERO),
            Mesh2d(body_mesh),
            MeshMaterial2d(body_material),
            Transform::from_translation(position.extend(0.0)),
            GlobalTransform::default(),
        ))
        .with_children(|parent| {
            // Left antenna
            let left_antenna_mesh = meshes.add(Polyline2d::new(vec![
                Vec2::new(body.semi_major()*0.75, 0.0),   // base
                left_antenna,  // tip
            ]));
            let antenna_material = materials.add(RED);
            parent.spawn((
                Mesh2d(left_antenna_mesh),
                MeshMaterial2d(antenna_material.clone()),
                Transform::from_translation(Vec3::new(0.0, 0.0, 0.1)),
                GlobalTransform::default(),
            ));

            // Right antenna
            let right_antenna_mesh = meshes.add(Polyline2d::new(vec![
                Vec2::new(body.focal_length(), 0.0),   // base
                right_antenna,  // tip
            ]));
            parent.spawn((
                Mesh2d(right_antenna_mesh),
                MeshMaterial2d(antenna_material),
                Transform::from_translation(Vec3::new(0.0, 0.0, 0.1)),
                GlobalTransform::default(),
            ));
        });
}

