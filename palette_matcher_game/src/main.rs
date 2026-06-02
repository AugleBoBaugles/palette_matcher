use bevy::prelude::*;
use bevy::ui::RelativeCursorPosition;
use rand::Rng;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<GameState>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                (handle_start_button, handle_slider_input, handle_submit_button),
                (update_target_swatch, update_player_swatch, update_slider_fills, update_score_text),
            )
                .chain(),
        )
        .run();
}

#[derive(Resource, Default)]
struct GameState {
    target: [f32; 3],
    player: [f32; 3],
    total_score: u32,
    active: bool,
}

#[derive(Component, Clone, Copy)]
enum ColorChannel {
    Red,
    Green,
    Blue,
}

#[derive(Component)]
struct TargetSwatch;

#[derive(Component)]
struct PlayerSwatch;

#[derive(Component)]
struct SliderTrack(ColorChannel);

#[derive(Component)]
struct SliderFill(ColorChannel);

#[derive(Component)]
struct ScoreText;

#[derive(Component)]
struct StartButton;

#[derive(Component)]
struct SubmitButton;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            row_gap: Val::Px(20.0),
            ..default()
        })
        .with_children(|root| {
            // Score at the top
            root.spawn((Text::new("Score: 0"), ScoreText));

            // Two swatches side by side
            root.spawn(Node {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(40.0),
                ..default()
            })
            .with_children(|row| {
                // Target swatch
                row.spawn(Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(8.0),
                    ..default()
                })
                .with_children(|col| {
                    col.spawn(Text::new("Target"));
                    col.spawn((
                        Node {
                            width: Val::Px(150.0),
                            height: Val::Px(150.0),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                        TargetSwatch,
                    ));
                });

                // Player swatch
                row.spawn(Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(8.0),
                    ..default()
                })
                .with_children(|col| {
                    col.spawn(Text::new("Your Color"));
                    col.spawn((
                        Node {
                            width: Val::Px(150.0),
                            height: Val::Px(150.0),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.0, 0.0, 0.0)),
                        PlayerSwatch,
                    ));
                });
            });

            // RGB sliders
            for (label, channel, fill_color) in [
                ("R", ColorChannel::Red, Color::srgb(0.8, 0.2, 0.2)),
                ("G", ColorChannel::Green, Color::srgb(0.2, 0.8, 0.2)),
                ("B", ColorChannel::Blue, Color::srgb(0.2, 0.2, 0.9)),
            ] {
                root.spawn(Node {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(10.0),
                    ..default()
                })
                .with_children(|row| {
                    row.spawn(Text::new(label));
                    row.spawn((
                        Node {
                            width: Val::Px(300.0),
                            height: Val::Px(24.0),
                            overflow: Overflow::clip(),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                        Interaction::default(),
                        RelativeCursorPosition::default(),
                        SliderTrack(channel),
                    ))
                    .with_children(|track| {
                        track.spawn((
                            Node {
                                position_type: PositionType::Absolute,
                                left: Val::Px(0.0),
                                top: Val::Px(0.0),
                                bottom: Val::Px(0.0),
                                width: Val::Percent(0.0),
                                ..default()
                            },
                            BackgroundColor(fill_color),
                            SliderFill(channel),
                        ));
                    });
                });
            }

            // Start and Submit buttons
            root.spawn(Node {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(20.0),
                ..default()
            })
            .with_children(|row| {
                row.spawn((
                    Button,
                    Node {
                        padding: UiRect::all(Val::Px(14.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.25, 0.55, 0.25)),
                    StartButton,
                ))
                .with_children(|b| {
                    b.spawn(Text::new("Start"));
                });

                row.spawn((
                    Button,
                    Node {
                        padding: UiRect::all(Val::Px(14.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.55, 0.25, 0.25)),
                    SubmitButton,
                ))
                .with_children(|b| {
                    b.spawn(Text::new("Submit"));
                });
            });
        });
}

fn handle_start_button(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<StartButton>)>,
    mut game_state: ResMut<GameState>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            let mut rng = rand::rng();
            game_state.target = [rng.random(), rng.random(), rng.random()];
            game_state.player = [0.0; 3];
            game_state.active = true;
        }
    }
}

fn handle_slider_input(
    interaction_query: Query<(&Interaction, &RelativeCursorPosition, &SliderTrack)>,
    mut game_state: ResMut<GameState>,
) {
    for (interaction, rel_cursor, track) in &interaction_query {
        if *interaction == Interaction::Pressed {
            if let Some(pos) = rel_cursor.normalized {
                let value = pos.x.clamp(0.0, 1.0);
                match track.0 {
                    ColorChannel::Red => game_state.player[0] = value,
                    ColorChannel::Green => game_state.player[1] = value,
                    ColorChannel::Blue => game_state.player[2] = value,
                }
            }
        }
    }
}

fn handle_submit_button(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<SubmitButton>)>,
    mut game_state: ResMut<GameState>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed && game_state.active {
            let [tr, tg, tb] = game_state.target;
            let [pr, pg, pb] = game_state.player;
            let distance = ((tr - pr).powi(2) + (tg - pg).powi(2) + (tb - pb).powi(2)).sqrt();
            let points = ((1.0 - distance / 3.0_f32.sqrt()) * 1000.0) as u32;
            game_state.total_score += points;
            game_state.target = [0.0; 3];
            game_state.player = [0.0; 3];
            game_state.active = false;
        }
    }
}

fn update_target_swatch(
    game_state: Res<GameState>,
    mut query: Query<&mut BackgroundColor, (With<TargetSwatch>, Without<PlayerSwatch>)>,
) {
    if game_state.is_changed() {
        if let Ok(mut color) = query.single_mut() {
            *color = if game_state.active {
                let [r, g, b] = game_state.target;
                BackgroundColor(Color::srgb(r, g, b))
            } else {
                BackgroundColor(Color::srgb(0.15, 0.15, 0.15))
            };
        }
    }
}

fn update_player_swatch(
    game_state: Res<GameState>,
    mut query: Query<&mut BackgroundColor, (With<PlayerSwatch>, Without<TargetSwatch>)>,
) {
    if game_state.is_changed() {
        if let Ok(mut color) = query.single_mut() {
            let [r, g, b] = game_state.player;
            *color = BackgroundColor(Color::srgb(r, g, b));
        }
    }
}

fn update_slider_fills(
    game_state: Res<GameState>,
    mut query: Query<(&mut Node, &SliderFill)>,
) {
    if game_state.is_changed() {
        for (mut node, fill) in &mut query {
            let value = match fill.0 {
                ColorChannel::Red => game_state.player[0],
                ColorChannel::Green => game_state.player[1],
                ColorChannel::Blue => game_state.player[2],
            };
            node.width = Val::Percent(value * 100.0);
        }
    }
}

fn update_score_text(
    game_state: Res<GameState>,
    mut query: Query<&mut Text, With<ScoreText>>,
) {
    if game_state.is_changed() {
        if let Ok(mut text) = query.single_mut() {
            text.0 = format!("Score: {}", game_state.total_score);
        }
    }
}
