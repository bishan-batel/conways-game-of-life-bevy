/*
conways-game
 */
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::{ui::MainMenuPlugin, input::InputPlugin, simulation::SimulationPlugin};

mod input;
mod ui;
mod simulation;

const GRID_SIZE: i32 = 100;

fn main() {
    println!("Hello, world!");
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: 800.0,
                height: 800.0,
                title: String::from("Game of Life"),
                ..Default::default()
            },
            ..Default::default()
        }))
        // .add_plugin(WorldInspectorPlugin)
        .add_plugin(MainMenuPlugin)
        .add_plugin(InputPlugin)
        .add_plugin(SimulationPlugin)
        .run();
}