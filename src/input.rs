/*
conways-game
*/
use bevy::{
    input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel},
    prelude::*,
    render::camera::CameraProjection,
    time::FixedTimestep,
};

const CAMERA_MOVE_SPEED: f32 = 10.0;
const CAMERA_MAX_ZOOM_SPEED: f32 = 100.0;
const CAMERA_ZOOM_SPEED: f32 = 5.0;

static MIN_CAMERA_SPEED: Vec3 = Vec3::splat(-CAMERA_MOVE_SPEED);
static MAX_CAMERA_SPEED: Vec3 = Vec3::splat(CAMERA_MOVE_SPEED);

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
struct Movement {
    plane_speed: Vec3,
    zoom_speed: f32,
}

pub struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup).add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.033))
                .with_system(camera_move)
                .with_system(camera_zoom),
        );
    }
}

fn setup(mut commands: Commands) {
    commands
        .spawn(Camera2dBundle::default())
        .insert(MainCamera)
        .insert(Movement {
            plane_speed: Vec3::ZERO,
            zoom_speed: 0.0,
        });
}

fn camera_move(
    mut camera: Query<(&mut Transform, &mut Movement), With<MainCamera>>,
    //time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let mut move_direction = Vec3::ZERO;

    if keyboard_input.pressed(KeyCode::W) {
        move_direction.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::S) {
        move_direction.y -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::A) {
        move_direction.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::D) {
        move_direction.x += 1.0;
    }

    let (mut transform, mut movement) = camera
        .iter_mut()
        .next()
        .expect("No transform on main camera");

    let move_direction = if move_direction.length_squared() == 0.0 {
        -movement.plane_speed * 0.1
    } else {
        move_direction.normalize()
    };

    movement.plane_speed =
        (movement.plane_speed + move_direction).clamp(MIN_CAMERA_SPEED, MAX_CAMERA_SPEED);

    if keyboard_input.pressed(KeyCode::Space) {
        movement.plane_speed = Vec3::ZERO
    }

    transform.translation += movement.plane_speed;
}

fn camera_zoom(
    mut camera: Query<(&mut Movement, &mut OrthographicProjection), With<MainCamera>>,
    mut scroll: EventReader<MouseWheel>,
    time: Res<Time>,
) {
    let mut delta = 0.0;

    for ev in scroll.iter() {
        match ev.unit {
            MouseScrollUnit::Pixel => delta -= ev.y * 10.0,
            MouseScrollUnit::Line => delta -= ev.y,
        }
    }

    let (mut movement, mut proj) = camera.single_mut();

    let delta = if delta == 0.0 {
        -movement.zoom_speed * 0.2
    } else {
        delta * CAMERA_ZOOM_SPEED
    };

    movement.zoom_speed =
        (movement.zoom_speed + delta).clamp(-CAMERA_MAX_ZOOM_SPEED, CAMERA_MAX_ZOOM_SPEED);
    proj.scale = (proj.scale + movement.zoom_speed * time.delta_seconds()).clamp(0.0, 10.0);
}

