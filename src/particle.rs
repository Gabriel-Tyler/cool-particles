// [position, velocity, appearance, spawning, interaction]

use bevy::{
    math::prelude::Circle,
    prelude::*,
    sprite::{ColorMaterial, MaterialMesh2dBundle},
    window::PrimaryWindow,
};

use rand::prelude::*;

use crate::config::*;

pub struct ParticlePlugin;
impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_particles)
            .add_systems(Update, (interaction, update_position).chain());
    }
}

#[derive(Component)]
pub struct Particle;
#[derive(Component)]
pub struct Velocity(pub Vec2);
#[derive(Component)]
pub struct Group(pub usize);

#[derive(Bundle)]
pub struct ParticleBundle {
    pub particle: Particle,
    pub velocity: Velocity,
    pub group: Group,
    pub appearance: MaterialMesh2dBundle<ColorMaterial>,
}

fn spawn_particles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window.get_single().unwrap();
    let mut rng = rand::thread_rng();
    for group_i in 0..N_GROUPS {
        for _ in 0..GROUP_COUNTS[group_i] {
            let x: f32 = rng.gen::<f32>() * window.width();
            let y: f32 = rng.gen::<f32>() * window.height();
            let z = GROUP_Z_POSITION[group_i];

            commands.spawn(ParticleBundle {
                particle: Particle,
                velocity: Velocity(Vec2::ZERO),
                group: Group(group_i),
                appearance: MaterialMesh2dBundle {
                    mesh: meshes.add(Circle::new(PARTICLE_RADIUS)).into(),
                    material: materials.add(ColorMaterial::from(GROUP_COLORS[group_i])),
                    transform: Transform::from_xyz(x, y, z),
                    ..default()
                },
            });
        }
    }
}

/*
fn interaction(mut query: Query<(&Group, &mut Velocity, &GlobalTransform), With<Particle>>) {
    let mut iter = query.iter_combinations_mut();

    while let Some([(Group(id1), mut v1, t1), (Group(id2), mut v2, t2)]) = iter.fetch_next() {
        let id1: usize = *id1;
        let id2: usize = *id2;
        let p1 = ATTRACTIONS[id1][id2];
        let p2 = ATTRACTIONS[id2][id1];
        let r1 = ATTRACTIONS_RADIUS[id1][id2];
        let r2 = ATTRACTIONS_RADIUS[id2][id1];
        let g1 = -p1 * 10.0;
        let g2 = -p2 * 10.0;
        let d = t1.translation() - t2.translation();
        let r = d.length();
        let mut f1 = Vec2::ZERO;
        let mut f2 = Vec2::ZERO;

        if r < r1 {
            let unit = d / r;
            f1.x += unit.x;
            f1.y += unit.y;
        }
        if r < r2 {
            let unit = d / r;
            f2.x += unit.x;
            f2.y += unit.y;
        }
        v1.0 = (v1.0 + f1 * g1) * 0.5;
        v2.0 = (v2.0 + f2 * g2) * 0.5;
    }
}
*/
fn interaction(mut query: Query<(&Group, &mut Velocity, &GlobalTransform), With<Particle>>) {
    let mut iter = query.iter_combinations_mut();

    while let Some([(Group(id1), mut v1, t1), (Group(id2), mut v2, t2)]) = iter.fetch_next() {
        let id1: usize = *id1;
        let id2: usize = *id2;

        let r1 = ATTRACTIONS_RADIUS[id1][id2];
        let r2 = ATTRACTIONS_RADIUS[id2][id1];

        let delta = t1.translation() - t2.translation();
        let dist = delta.length();
        let dist_sq: f32 = delta.length_squared().max(f32::EPSILON); // to prevent divide by zero

        // update particle 1:
        if dist < r1 {
            let f = GRAVITY_CONSTANT * ATTRACTIONS[id1][id2] / dist_sq;
            let dv = delta * f;
            v1.0.x += dv.x;
            v1.0.y += dv.y;
        }

        // update particle 2:
        if dist < r2 {
            let f = GRAVITY_CONSTANT * ATTRACTIONS[id2][id1] / dist_sq;
            let dv = delta * f;
            v2.0.x += dv.x;
            v2.0.y += dv.y;
        }
    }
}

fn update_position(
    time: Res<Time>,
    mut query: Query<(&mut Velocity, &mut Transform), With<Particle>>,
) {
    let dt = time.delta_seconds();
    for (mut v, mut transform) in &mut query {
        transform.translation.x += v.0.x * dt;
        transform.translation.y += v.0.y * dt;

        let vx = v.0.x;
        let vy = v.0.y;
        let x = transform.translation.x;
        let y = transform.translation.y;

        if (x < 0.0 && vx < 0.0) || (x > WORLD_WIDTH && vx > 0.0) {
            v.0.x *= -1.0;
        }
        if (y < 0.0 && vy < 0.0) || (y > WORLD_HEIGHT && vy > 0.0) {
            v.0.y *= -1.0;
        }
    }
    /*
    for (v, mut transform) in &mut query {
        transform.translation.x += v.0.x * dt;
        if transform.translation.x < 0.0 || transform.translation.x > WORLD_WIDTH {
            transform.translation.x *= -1.0;
            transform.translation.x = transform.translation.x.clamp(0.0, WORLD_WIDTH);
        }
        transform.translation.y += v.0.y * dt;
        if transform.translation.y < 0.0 || transform.translation.y > WORLD_HEIGHT {
            transform.translation.y *= -1.0;
            transform.translation.y = transform.translation.y.clamp(0.0, WORLD_HEIGHT);
        }
    }
    */

    /*
    for (v, mut transform) in &mut query {
        transform.translation.x = (transform.translation.x + v.0.x * dt).clamp(0.0, WORLD_WIDTH);
        transform.translation.y = (transform.translation.y + v.0.y * dt).clamp(0.0, WORLD_HEIGHT);
    }
    */
}
