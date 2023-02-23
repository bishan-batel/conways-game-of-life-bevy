/*
conways-game
 */
use bevy::{app::AppExit, prelude::*, time::FixedTimestep};
use bevy_inspector_egui::egui::util::hash;

use crate::{
    input::MainCamera,
    ui::{GameExitEvent, SimulationStartEvent, SimulationStopEvent},
    GRID_SIZE,
};

pub const SPRITE_SIZE: f32 = 32.0;
const BACKGROUND_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(BACKGROUND_COLOR))
            .insert_resource(MouseWorldPositionDraw(None))
            .insert_resource(MouseWorldPositionErase(None))
            .insert_resource(IsSimulationRunning(false))
            .add_startup_system(setup)
            .add_system(cell_interaction)
            .add_system(start_simulation)
            .add_system(stop_simulation)
            .add_system(exit_game)
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(0.016))
                    .with_system(set_cursor_world_position.label(CellInteraction::Input))
                    .with_system(
                        cell_interaction
                            .label(CellInteraction::Setting)
                            .after(CellInteraction::Input),
                    ),
            )
            .add_system_set(
                SystemSet::new()
                    // .with_run_criteria(FixedTimestep::step(0.25))
                    .with_system(
                        simulation_step
                            .label(CellInteraction::Simulation)
                            .after(CellInteraction::Setting),
                    ),
            );
    }
}

#[derive(SystemLabel, Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum CellInteraction {
    Input,
    Setting,
    Simulation,
}

#[derive(Resource)]
struct MouseWorldPositionDraw(Option<(f32, f32)>);

#[derive(Resource)]
struct MouseWorldPositionErase(Option<(f32, f32)>);

#[derive(Component)]
struct Cell {
    state: CellState,
}

enum CellState {
    Alive,
    Empty,
}

#[derive(Resource)]
struct SpriteImages {
    empty_cell: Handle<Image>,
    alive_cell: Handle<Image>,
}

#[derive(Resource)]
struct IsSimulationRunning(bool);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let images = SpriteImages {
        empty_cell: asset_server.load("sprites/empty_cell.png"),
        alive_cell: asset_server.load("sprites/alive_cell.png"),
    };

    for x in 0..GRID_SIZE {
        for y in 0..GRID_SIZE {
            let (state, texture) = if (x+y) % 2 == 0 {
                (CellState::Empty, images.empty_cell.clone())
            } else {
                (CellState::Alive, images.alive_cell.clone())
            };
            commands
                .spawn(SpriteBundle {
                    transform: Transform::from_xyz(
                        (x as f32) * SPRITE_SIZE,
                        (y as f32) * SPRITE_SIZE,
                        0.0,
                    ),
                    sprite: Sprite {
                        ..Default::default()
                    },
                    texture,
                    ..Default::default()
                })
                .insert(Cell {
                    state
                });
        }
    }

    commands.insert_resource(images);
}

fn set_cursor_world_position(
    windows: Res<Windows>,
    camera: Query<(&Transform, &OrthographicProjection), With<MainCamera>>,
    mouse_btn: Res<Input<MouseButton>>,
    mut mouse_world_pos_draw: ResMut<MouseWorldPositionDraw>,
    mut mouse_world_pos_erase: ResMut<MouseWorldPositionErase>,
    is_running: Res<IsSimulationRunning>,
) {
    let window = windows
        .get_primary()
        .expect("Could not retrieve primary window");

    if !is_running.0 {
        if let Some(pos) = window.cursor_position() {
            let (transform, proj) = camera.single();
            let pos = get_mouse_world(pos, transform, window, proj);

            if mouse_btn.pressed(MouseButton::Left) {
                *mouse_world_pos_draw = MouseWorldPositionDraw(Some((pos.x, pos.y)));
            } else if mouse_btn.pressed(MouseButton::Right) {
                *mouse_world_pos_erase = MouseWorldPositionErase(Some((pos.x, pos.y)));
            }
        }
    }
}

fn get_mouse_world(
    pos: Vec2,
    main_transform: &Transform,
    window: &Window,
    proj: &OrthographicProjection,
) -> Vec3 {
    let center = main_transform.translation.truncate();

    let half_width = (window.width() / 2.0) * proj.scale;
    let half_height = (window.height() / 2.0) * proj.scale;
    let left = center.x - half_width;
    let bottom = center.y - half_height;

    Vec3::new(left + pos.x * proj.scale, bottom + pos.y * proj.scale, 0.0)
}

// Updates cell interactions for manual drawing
fn cell_interaction(
    mut cells: Query<(&mut Cell, &mut Handle<Image>, &Transform)>,
    mut mouse_world_pos_draw: ResMut<MouseWorldPositionDraw>,
    mut mouse_world_pos_erase: ResMut<MouseWorldPositionErase>,
    sprite_images: Res<SpriteImages>,
    is_running: ResMut<IsSimulationRunning>,
) {
    let draw = mouse_world_pos_draw.0.take();
    let erase = mouse_world_pos_erase.0.take();

    if !is_running.0 && (draw.is_some() || erase.is_some()) {
        for (mut cell, mut sprite, transform) in cells.iter_mut() {
            if let Some(pos) = draw {
                if is_cell_in_bounds(
                    pos,
                    (transform.translation.x, transform.translation.y),
                    (SPRITE_SIZE / 2.0, SPRITE_SIZE / 2.0),
                ) {
                    cell.state = CellState::Alive;
                    *sprite = sprite_images.alive_cell.clone();
                }
            }

            if let Some(pos) = erase {
                if is_cell_in_bounds(
                    pos,
                    (transform.translation.x, transform.translation.y),
                    (SPRITE_SIZE / 2.0, SPRITE_SIZE / 2.0),
                ) {
                    cell.state = CellState::Empty;
                    *sprite = sprite_images.empty_cell.clone();
                }
            }
        }
    }
}

fn is_cell_in_bounds(xy: (f32, f32), center: (f32, f32), dims: (f32, f32)) -> bool {
    xy.0 > center.0 - dims.0
        && xy.0 < center.0 + dims.0
        && xy.1 > center.1 - dims.1
        && xy.1 < center.1 + dims.1
}

// Event handler for exiting the game
fn exit_game(mut exit_reader: EventReader<GameExitEvent>, mut exit: EventWriter<AppExit>) {
    if let Some(_) = exit_reader.iter().next() {
        exit.send(AppExit);
    }
}

// Event Handlers for stopping & starting the simulation
fn start_simulation(
    mut event_reader: EventReader<SimulationStartEvent>,
    mut start_sim: ResMut<IsSimulationRunning>,
) {
    if let Some(_) = event_reader.iter().next() {
        start_sim.0 = true;
    }
}

fn stop_simulation(
    mut event_reader: EventReader<SimulationStopEvent>,
    mut start_sim: ResMut<IsSimulationRunning>,
) {
    if let Some(_) = event_reader.iter().next() {
        start_sim.0 = false;
    }
}

fn simulation_step(
    mut cells: Query<(&mut Cell, &mut Handle<Image>)>,
    is_running: Res<IsSimulationRunning>,
    sprite_images: Res<SpriteImages>,
) {
    if !is_running.0 {
        return;
    }

    let mut life_grid: Vec<bool> = Vec::new();

    for (cell, ..) in cells.iter_mut() {
        life_grid.push(match cell.state {
            CellState::Alive => true,
            CellState::Empty => false,
        });
    }

    for (i, (mut cell, mut sprite)) in cells.iter_mut().enumerate() {
        let mut neighbors = 0;
        let x = i as i32 % GRID_SIZE;
        let y = i as i32 / GRID_SIZE;

        for xi in (x - 1)..(x + 2) {
            for yi in (y - 1)..(y + 2) {
                if (xi != x || yi != y)
                    && xi >= 0
                    && xi < GRID_SIZE
                    && yi >= 0
                    && yi < GRID_SIZE
                {
                    if life_grid[(xi + yi * GRID_SIZE) as usize] {
                        neighbors += 1;
                    }
                }
            }
        }

        if neighbors < 2 || neighbors > 3 {
            if let CellState::Alive = cell.state {
                cell.state = CellState::Empty;
                *sprite = sprite_images.empty_cell.clone();
            }
        }

        if neighbors == 3 {
            cell.state = CellState::Alive;
            *sprite = sprite_images.alive_cell.clone();
        }
    }
}
