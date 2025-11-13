use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::input::ButtonState;
use bevy::input::mouse::MouseButtonInput;
use crate::input;

pub const GRID_W: u32 = 64;
pub const GRID_H: u32 = 64;

pub struct PheromonePlugin;

impl Plugin for PheromonePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Pheromones::new())
            .add_systems(Startup, setup_heatmap)
            .add_systems(Update, (update_heatmap,click_add_pheromones_system));
    }
}

#[derive(Resource)]
pub struct Pheromones {
    pub grid: Vec<f32>,  // size GRID_W * GRID_H
}

impl Pheromones {
    fn new() -> Self {
        Self {
            grid: vec![0.0; (GRID_W * GRID_H) as usize],
        }
    }

    // optional helper to get mutable reference by (x, y)
    fn get_mut(&mut self, x: usize, y: usize) -> &mut f32 {
        &mut self.grid[y * GRID_W as usize + x]
    }
}

#[derive(Resource)]
struct Heatmap {
    handle: Handle<Image>,
}

fn setup_heatmap(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
) {
    // Create image data
    let size = Extent3d {
        width: GRID_W,
        height: GRID_H,
        depth_or_array_layers: 1,
    };
    let mut data = vec![0u8; (GRID_W * GRID_H * 4) as usize];
    for y in 0..GRID_H {
        for x in 0..GRID_W {
            let i = ((y * GRID_W + x) * 4) as usize;
            let fx = (x as f32) / (GRID_W as f32 - 1.0);
            let fy = (y as f32) / (GRID_H as f32 - 1.0);
            data[i + 0] = (fx * 255.0) as u8;
            data[i + 1] = (fy * 255.0) as u8;
            data[i + 2] = 0;
            data[i + 3] = 255;
        }
    }

    let image = Image::new_fill(
        size,
        TextureDimension::D2,
        &data,
        TextureFormat::Rgba8UnormSrgb,
        Default::default(),
    );

    let handle = images.add(image);

    commands.spawn((
        Sprite {
            image: handle.clone(),
            custom_size: Some(Vec2::new(GRID_W as f32, GRID_H as f32)),
            ..default()
        },
        Transform {
            translation: Vec3::ZERO,
            scale: Vec3::splat(6.0),
            ..default()
        },
        GlobalTransform::default(),
    ));

    commands.insert_resource(Heatmap { handle });
}

fn update_heatmap(
    mut images: ResMut<Assets<Image>>,
    pheromones: Res<Pheromones>,
    heatmap: Res<Heatmap>,
) {
    if let Some(image) = images.get_mut(&heatmap.handle) {
        if let Some(ref mut data) = image.data {
            for y in 0..GRID_H as usize {
                for x in 0..GRID_W as usize {
                    let i = (y * GRID_W as usize + x) * 4;
                    let value = pheromones.grid[y * GRID_W as usize + x].clamp(0.0, 1.0);

                    if value == 0.0 {
                        // Fully transparent background
                        data[i] = 0;       // R
                        data[i + 1] = 0;   // G
                        data[i + 2] = 0;   // B
                        data[i + 3] = 0;   // A
                    } else {
                        // Pheromone color (white) with alpha proportional to value
                        let alpha = (value * 255.0) as u8;
                        data[i] = 122;       // R
                        data[i + 1] = 1;   // G
                        data[i + 2] = 119;   // B
                        data[i + 3] = alpha; // A
                    }
                }
            }
        }
    }
}


pub fn click_add_pheromones_system(
    mouse_buttons: Res<input::MouseButtons>,
    mouse_pos: Res<input::MousePosition>,
    mut pheromones: ResMut<Pheromones>,
    heatmap_query: Query<&GlobalTransform, With<Sprite>>,
) {
    let Ok(heatmap_transform) = heatmap_query.single() else { return; };
    if mouse_buttons.left {

    }
    if let Some(world_pos) = mouse_pos.0 {

        let local_pos = heatmap_transform.affine().inverse().transform_point3(world_pos.extend(0.0));

        // Map from local coordinates (-width/2..width/2) to grid indices
        let cell_width = 1.0;
        let cell_height = 1.0;

        let mut grid_x = ((local_pos.x + GRID_W as f32 * 0.5) / cell_width).floor() as isize;
        let mut grid_y = ((GRID_H as f32 * 0.5 - local_pos.y) / cell_height).floor() as isize;

        // Clamp to grid bounds
        grid_x = grid_x.clamp(0, GRID_W as isize - 1);
        grid_y = grid_y.clamp(0, GRID_H as isize - 1);

        pheromones.grid[grid_y as usize * GRID_W as usize + grid_x as usize] = 1.0;
    }
}
