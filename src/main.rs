use bevy::prelude::*;

const MAX_SPEED:f32 = 200.0;
const MAX_FORCE:f32 = 400.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(PheromoneLayer::new(
            200,    // width in cells
            200,    // height in cells
            4.0,    // cell size in world units
        ))
        .add_systems(Startup, (setup_camera, setup_ants))
        .add_systems(Update, (
            mouse_deposit_pheromone,
            update_ants,
            update_physics,
        ))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

///////////
//PHYSICS//
///////////
#[derive(Component)]
struct Velocity(Vec2);

#[derive(Component)]
struct Acceleration(Vec2);

#[derive(Component)]
struct MaxSpeed(f32);

#[derive(Component)]
struct MaxForce(f32);

#[derive(Bundle)]
struct PhysicsObject {
    velocity: Velocity,
    acceleration: Acceleration,
    max_speed: MaxSpeed,
    max_force: MaxForce,
}

fn update_physics(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Velocity, &mut Acceleration, &MaxSpeed), With<Ant>>,
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

//////////////
//PHEROMONES//
//////////////
#[derive(Resource)]
struct PheromoneLayer {
    width: usize,
    height: usize,
    cell_size: f32,            // size of one grid cell in world units
    values: Vec<f32>,          // row-major flattened grid: y * width + x
}

impl PheromoneLayer {
    fn new(width: usize, height: usize, cell_size: f32) -> Self {
        Self {
            width,
            height,
            cell_size,
            values: vec![0.0; width * height],
        }
    }

    // Convert world position to grid index
    fn pos_to_index(&self, pos: Vec2) -> Option<usize> {
        let x = (pos.x / self.cell_size).floor() as isize;
        let y = (pos.y / self.cell_size).floor() as isize;

        if x >= 0 && x < self.width as isize && y >= 0 && y < self.height as isize {
            Some((y as usize) * self.width + (x as usize))
        } else {
            None
        }
    }

    fn deposit(&mut self, pos: Vec2, amount: f32) {
        if let Some(idx) = self.pos_to_index(pos) {
            self.values[idx] += amount;
        }
    }

    fn get(&self, pos: Vec2) -> f32 {
        self.pos_to_index(pos).map_or(0.0, |idx| self.values[idx])
    }

    // Optionally implement diffusion/evaporation here
    fn diffuse(&mut self, diffusion_rate: f32, decay: f32) {
        let mut new_values = self.values.clone();
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = y * self.width + x;
                let mut sum = 0.0;
                let mut count = 0.0;
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        let nx = x as isize + dx;
                        let ny = y as isize + dy;
                        if nx >= 0 && nx < self.width as isize && ny >= 0 && ny < self.height as isize {
                            let nidx = (ny as usize) * self.width + (nx as usize);
                            sum += self.values[nidx];
                            count += 1.0;
                        }
                    }
                }
                new_values[idx] = sum / count * (1.0 - decay);
                new_values[idx] *= 1.0 - diffusion_rate;
            }
        }
        self.values = new_values;
    }
}

fn mouse_deposit_pheromone(
    window:Single<&Window, With<bevy::window::PrimaryWindow>>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut pheromones: ResMut<PheromoneLayer>,
) {
    if let Some(position) = window.cursor_position() && buttons.pressed(MouseButton::Left) {
        let deposit_amount = 5.0; // tune as needed
        pheromones.deposit(position, deposit_amount);
    }
}

////////
//ANTS//
////////
#[derive(Component)]
struct Ant;

#[derive(Component)]
struct AntennaSensors {
    left_offset: Vec2,
    right_offset: Vec2,
}

fn setup_ants(
    commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
) {
    spawn_ant(commands, meshes, materials, Vec2::ZERO);
}

fn spawn_ant(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    position: Vec2,
) {
    let size = 4.0;
    let body = Ellipse::new(size, size*0.5);
    // --- Ant body ---
    let body_mesh = meshes.add(body);
    let body_material = materials.add(Color::srgb(0.5, 0.02, 0.02));

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
                left_offset: Vec2::new(size*1.25, size*0.5),
                right_offset: Vec2::new(size*1.25, -size * 0.5),
            },
            Mesh2d(body_mesh),
            MeshMaterial2d(body_material),
            Transform::from_translation(position.extend(0.0)),
            GlobalTransform::default(),
        ))
        .with_children(|parent| {
            // Left antenna
            let left_antenna_mesh = meshes.add(Polyline2d::new(vec![
                Vec2::new(body.focal_length(), 0.0),   // base
                Vec2::new(size*1.25, size*0.5),  // tip
            ]));
            let antenna_material = materials.add(Color::WHITE);
            parent.spawn((
                Mesh2d(left_antenna_mesh),
                MeshMaterial2d(antenna_material.clone()),
                Transform::from_translation(Vec3::new(0.0, 0.0, 0.1)),
                GlobalTransform::default(),
            ));

            // Right antenna
            let right_antenna_mesh = meshes.add(Polyline2d::new(vec![
                Vec2::new(body.focal_length(), 0.0),   // base
                Vec2::new(size*1.25, -size*0.5),  // tip
            ]));
            parent.spawn((
                Mesh2d(right_antenna_mesh),
                MeshMaterial2d(antenna_material),
                Transform::from_translation(Vec3::new(0.0, 0.0, 0.1)),
                GlobalTransform::default(),
            ));
        });
}


fn update_ants(
    mut query: Query<(&Transform, &AntennaSensors, &mut Acceleration, &MaxForce), With<Ant>>,
    pheromones: Res<PheromoneLayer>,
) {
    for (transform, sensors, mut accel, max_force) in &mut query {
        let rotation = transform.rotation;
        let left_pos = transform.translation.truncate() + rotation.mul_vec3(sensors.left_offset.extend(0.0)).truncate();
        let right_pos = transform.translation.truncate() + rotation.mul_vec3(sensors.right_offset.extend(0.0)).truncate();

        let left_strength = pheromones.get(left_pos);
        let right_strength = pheromones.get(right_pos);

        let steer_dir = right_strength - left_strength;
        accel.0 += Vec2::new(steer_dir, 0.0).clamp_length_max(max_force.0);
    }
}

