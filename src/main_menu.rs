use bevy::{prelude::*, text::FontSmoothing};

use crate::{StartGame, app_state::AppState};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), main_menu_setup)
            .add_systems(OnExit(AppState::MainMenu), main_menu_cleanup)
            .add_systems(
                Update,
                (
                    update_button_color_system,
                    play_button_pressed_system,
                    exit_button_pressed_system,
                )
                    .run_if(in_state(AppState::MainMenu)),
            );
    }
}

#[derive(Debug, Component)]
pub struct MainMenu;

#[derive(Debug, Component)]
struct StartButton;

#[derive(Debug, Component)]
struct ExitButton;

#[derive(Debug, Clone, Component)]
struct ButtonColorScheme {
    normal: Color,
    hover: Color,
    pressed: Color,
}

fn main_menu_setup(mut commands: Commands) {
    commands.spawn((
        MainMenu,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            row_gap: Val::Px(12.0),
            ..Default::default()
        },
        children![
            (
                Text::new("BREAKOUT"),
                TextFont {
                    font_size: 64.0,
                    font_smoothing: FontSmoothing::AntiAliased,
                    ..Default::default()
                },
                TextColor(Color::WHITE),
            ),
            (
                Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(12.0),
                    ..Default::default()
                },
                children![
                    (
                        Button,
                        StartButton,
                        ButtonColorScheme::default(),
                        Node {
                            width: Val::Px(220.0),
                            height: Val::Px(60.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..Default::default()
                        },
                        BackgroundColor(Color::linear_rgb(0.3, 0.3, 0.3)),
                        children![(
                            Text::new("Play"),
                            TextFont {
                                font_size: 32.0,
                                font_smoothing: FontSmoothing::AntiAliased,
                                ..Default::default()
                            },
                            TextColor(Color::WHITE),
                        )],
                    ),
                    (
                        Button,
                        ExitButton,
                        ButtonColorScheme::default(),
                        Node {
                            width: Val::Px(220.0),
                            height: Val::Px(60.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..Default::default()
                        },
                        BackgroundColor(Color::linear_rgb(0.3, 0.3, 0.3)),
                        children![(
                            Text::new("Exit"),
                            TextFont {
                                font_size: 32.0,
                                font_smoothing: FontSmoothing::AntiAliased,
                                ..Default::default()
                            },
                            TextColor(Color::WHITE)
                        )],
                    ),
                ]
            )
        ],
    ));
}

fn main_menu_cleanup(mut commands: Commands, main_menu: Query<Entity, With<MainMenu>>) {
    for entity in main_menu {
        commands.entity(entity).despawn();
    }
}

fn update_button_color_system(
    mut buttons: Query<
        (&Interaction, &mut BackgroundColor, &ButtonColorScheme),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut bg_color, btn_color_scheme) in buttons.iter_mut() {
        match interaction {
            Interaction::None => {
                bg_color.0 = btn_color_scheme.normal;
            }
            Interaction::Hovered => {
                bg_color.0 = btn_color_scheme.hover;
            }
            Interaction::Pressed => {
                bg_color.0 = btn_color_scheme.pressed;
            }
        };
    }
}

fn play_button_pressed_system(
    mut commands: Commands,
    buttons: Query<&Interaction, (Changed<Interaction>, With<Button>, With<StartButton>)>,
) {
    for interaction in buttons.iter() {
        if *interaction == Interaction::Pressed {
            commands.trigger(StartGame);
        };
    }
}

fn exit_button_pressed_system(
    mut exit: EventWriter<AppExit>,
    buttons: Query<&Interaction, (Changed<Interaction>, With<Button>, With<ExitButton>)>,
) {
    for interaction in buttons.iter() {
        if *interaction == Interaction::Pressed {
            exit.write(AppExit::Success);
        };
    }
}

impl Default for ButtonColorScheme {
    fn default() -> Self {
        Self {
            normal: Color::linear_rgb(0.3, 0.3, 0.3),
            hover: Color::linear_rgb(0.275, 0.275, 0.275),
            pressed: Color::linear_rgb(0.2, 0.2, 0.2),
        }
    }
}
