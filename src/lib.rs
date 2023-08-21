#![windows_subsystem = "windows"]

use engine::{
    camera::Camera3d,
    cecs::prelude::*,
    glam::Vec3,
    renderer::camera_bundle,
    transform::{self, transform_bundle},
    App, DefaultPlugins, Plugin, Stage,
};

fn setup_boids(mut cmd: Commands) {
    //camera
    cmd.spawn()
        .insert_bundle(camera_bundle(Camera3d {
            eye: Vec3::new(0.0, 0.0, 20.0),
            target: Vec3::ZERO,
            up: Vec3::Y,
            aspect: 16.0 / 9.0,
            fovy: 45.0,
            znear: 5.0,
            zfar: 50.0,
        }))
        .insert_bundle(transform_bundle(transform::Transform::default()));
}

struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(self, app: &mut App) {
        app.stage(Stage::Update);

        app.add_startup_system(setup_boids);
    }
}

pub async fn game() {
    let mut app = App::default();
    app.add_plugin(DefaultPlugins);
    app.add_plugin(GamePlugin);
    app.run().await;
}
