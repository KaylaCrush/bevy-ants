use bevy::{input::mouse::MouseWheel, prelude::*};


mod physics;
mod ants;
mod input;
mod pheromone;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(input::MouseInputPlugin)
        .add_plugins(physics::PhysicsPlugin)
        .add_plugins(pheromone::PheromonePlugin)
        .add_systems(Startup, (setup_camera, ants::setup_ants))
        .add_systems(Update, (target_mouse, zoom_camera_transform))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Transform::from_scale(Vec3::splat(0.1)), // smaller = zoom in, larger = zoom out
    ));
}

fn target_mouse(
    mouse_pos: Res<input::MousePosition>,
    mut query: Query<&mut physics::Target, With<ants::Ant>>,
) {
    if let Some(pos) = mouse_pos.0 {
        for mut target in &mut query {
            target.0 = pos;
        }
    }
}

fn zoom_camera_transform(
    mut scroll_events: MessageReader<MouseWheel>,
    mut query: Query<&mut Transform, With<Camera2d>>,
) {
    let mut scroll_delta = 0.0;
    for ev in scroll_events.read() {
        scroll_delta += ev.y;
    }

    if scroll_delta != 0.0 {
        let zoom_speed = 0.1;
        for mut transform in &mut query {
            transform.scale *= Vec3::splat(1.0 - scroll_delta * zoom_speed);
            transform.scale = transform.scale.clamp(Vec3::splat(0.1), Vec3::splat(10.0));
        }
    }
}

