/*
conways-game
*/

use bevy::prelude::*;
use crate::GRID_SIZE;
use crate::simulation::SPRITE_SIZE;

const NORMAL_BUTTON: Color = Color::rgb(0.0, 0.8, 0.8);
const HOVERED_BUTTON: Color = Color::rgb(0.4, 0.8, 0.8);
const PRESSED_BUTTON: Color = Color::rgb(0.4, 1.0, 1.0);

pub struct GameExitEvent;
pub struct SimulationStartEvent;
pub struct SimulationStopEvent;

#[derive(Component)]
struct ClassicButton(ButtonType);

#[derive(PartialEq, Clone, Copy)]
enum ButtonType {
    Start,
    Stop,
    Exit,
}

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GameExitEvent>()
            .add_event::<SimulationStopEvent>()
            .add_event::<SimulationStartEvent>()
            .add_startup_system(setup)
            .add_system(button_system);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::FlexEnd,
                align_items: AlignItems::FlexEnd,
                ..Default::default()
            },
            background_color: Color::NONE.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                        ..Default::default()
                    },
                    //background_color: Color::rgb(0.1, 0.1, 0.1,).into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    // Start button
                    parent
                        .spawn(build_classic_button())
                        .with_children(|parent| {
                            parent.spawn(build_classic_text("Start", &asset_server));
                        })
                        .insert(ClassicButton(ButtonType::Start));

                    // Stop button
                    parent
                        .spawn(build_classic_button())
                        .with_children(|parent| {
                            parent.spawn(build_classic_text("Stop", &asset_server));
                        })
                        .insert(ClassicButton(ButtonType::Stop));

                    // Exit Button
                    parent
                        .spawn(build_classic_button())
                        .with_children(|parent| {
                            parent.spawn(build_classic_text("Exit", &asset_server));
                        })
                        .insert(ClassicButton(ButtonType::Exit));
                });
        });
}

fn build_classic_button() -> ButtonBundle {
    ButtonBundle {
        style: Style {
            size: Size::new(Val::Px(150.0), Val::Px(50.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect::all(Val::Px(10.0)),
            ..Default::default()
        },
        background_color: NORMAL_BUTTON.into(),
        ..Default::default()
    }
}

fn build_classic_text(value: &str, asset_server: &Res<AssetServer>) -> TextBundle {
    let style = TextStyle {
        font: asset_server.load("fonts/JetBrainsMono.ttf"),
        font_size: 30.0,
        color: Color::WHITE,
    };

    TextBundle {
        text: Text::from_section(value, style),
        ..Default::default()
    }
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &ClassicButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut start_writer: EventWriter<SimulationStartEvent>,
    mut stop_writer: EventWriter<SimulationStopEvent>,
    mut exit_writer: EventWriter<GameExitEvent>,
) {
    for (interaction, mut color, classic_button) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();

                match classic_button.0 {
                    ButtonType::Start => start_writer.send(SimulationStartEvent),
                    ButtonType::Stop => stop_writer.send(SimulationStopEvent),
                    ButtonType::Exit => exit_writer.send(GameExitEvent),
                }
            }

            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}
