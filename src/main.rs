use bevy::prelude::*;

mod physics;
mod ants;
mod input;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(input::MouseInputPlugin)
        .add_plugins(physics::PhysicsPlugin)
        .add_systems(Startup, (setup_camera, ants::setup_ants))
        .add_systems(Update, (target_mouse))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
            Camera2d::default(),
    ));
}

fn target_mouse(
    mouse_pos: Res<input::MousePosition>,
    mut query: Query<(&mut physics::Target), With<ants::Ant>>,
) {
    if let Some(pos) = mouse_pos.0 {
        for mut target in &mut query {
            target.0 = pos;
        }
    }
}