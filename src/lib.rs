#![windows_subsystem = "windows"]

use engine::{
    assets::{self, Assets, Handle},
    camera::Camera3d,
    cecs::prelude::*,
    glam::{Quat, Vec2, Vec3},
    renderer::{
        self, camera_bundle,
        sprite_renderer::{self, SpriteSheet},
        GraphicsState,
    },
    transform::{self, transform_bundle, GlobalTransform, Transform},
    App, DefaultPlugins, DeltaTime, Plugin, Stage,
};

struct Boid;

struct Velocity(pub Vec3);
struct LastVelocity(pub Vec3);

struct BoidConfig {
    radius: f32,
    separation_radius: f32,
    min_vel: f32,
}

const N: usize = 1000;

fn update_boids(
    mut q: Query<(EntityId, &mut Transform, &mut Velocity, &LastVelocity), With<Boid>>,
    positions: Query<(&GlobalTransform, EntityId), With<Boid>>,
    conf: Res<BoidConfig>,
    dt: Res<DeltaTime>,
) {
    let radius = conf.radius;
    let sepa = conf.separation_radius;
    let min_vel = conf.min_vel;
    let dt = dt.0.as_secs_f32();
    q.par_for_each_mut(|(id, tr, vel, last_vel)| {
        let pos = tr.pos;
        let mut dir = -min_vel * tr.pos.normalize_or_zero(); // move towards the center if no other
                                                             // boids are in sight
        positions.iter().for_each(|(gtr, boid_id)| {
            if id == boid_id {
                return;
            }
            let d = pos - gtr.0.pos;
            let mag = d.length();
            if mag < radius && vel.0.dot(d) < 0.0 {
                let ratio = (mag / sepa).clamp(0.01, 1.0);
                dir -= d / ratio;
            }
        });
        vel.0 = dir.lerp(last_vel.0, 0.5);
        tr.pos += vel.0 * dt;
    });
}

fn sprite_rotate(mut q: Query<(&mut Transform, &Velocity)>) {
    q.par_for_each_mut(|(tr, Velocity(vel))| {
        let angle = -vel.x.atan2(vel.y);
        tr.rot = Quat::from_rotation_z(angle);
    });
}

fn update_boids_vel(mut q: Query<(&mut LastVelocity, &Velocity)>) {
    q.par_for_each_mut(move |(l, vel)| {
        l.0 = vel.0;
    });
}

fn setup_boids(
    mut cmd: Commands,
    graphics_state: Res<GraphicsState>,
    mut assets: ResMut<assets::Assets<SpriteSheet>>,
) {
    //camera
    cmd.spawn()
        .insert_bundle(camera_bundle(Camera3d {
            eye: Vec3::new(0.0, 0.0, 100.0),
            target: Vec3::ZERO,
            up: Vec3::Y,
            aspect: 16.0 / 9.0,
            fovy: 45.0,
            znear: 5.0,
            zfar: 5000.0,
        }))
        .insert_bundle(transform_bundle(transform::Transform::default()));

    let boid = load_sprite_sheet(
        &graphics_state,
        include_bytes!("../assets/boid.png"),
        Vec2::splat(32.0),
        1,
        "boid",
        &mut assets,
    );

    for _ in 0..N {
        // TODO scale by map size
        let x = fastrand::f32();
        let y = fastrand::f32();

        let vx = fastrand::f32();
        let vy = fastrand::f32();
        cmd.spawn()
            .insert_bundle(transform_bundle(transform::Transform::from_position(
                Vec3::new(x, y, 0.0),
            )))
            .insert_bundle(sprite_renderer::sprite_sheet_bundle(boid.clone(), None))
            .insert_bundle((
                Boid,
                LastVelocity(Vec3::ZERO),
                Velocity(Vec3::new(vx, vy, 0.0)),
            ));
    }
}

fn load_sprite_sheet(
    graphics_state: &GraphicsState,
    bytes: &[u8],
    box_size: Vec2,
    num_cols: u32,
    label: &str,
    assets: &mut Assets<SpriteSheet>,
) -> Handle<SpriteSheet> {
    let texture = renderer::texture::Texture::from_bytes(
        graphics_state.device(),
        graphics_state.queue(),
        bytes,
        label,
    )
    .unwrap();
    let sprite_sheet = SpriteSheet::from_texture(Vec2::ZERO, box_size, num_cols, texture);

    assets.insert(sprite_sheet)
}

struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(self, app: &mut App) {
        app.stage(Stage::Update)
            .add_system(update_boids)
            .add_system(update_boids_vel.after(update_boids))
            .add_system(sprite_rotate.after(update_boids));

        app.add_startup_system(setup_boids);
        app.insert_resource(BoidConfig {
            radius: 30.0,
            separation_radius: 10.0,
            min_vel: 10.0,
        });
    }
}

pub async fn game() {
    let mut app = App::default();
    app.add_plugin(DefaultPlugins);
    app.add_plugin(GamePlugin);
    app.run().await;
}
