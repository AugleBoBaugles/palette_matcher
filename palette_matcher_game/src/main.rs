use bevy::prelude::*;
use rand::Rng;

const STEP: f32 = 0.05;

#[derive(States, Default, Clone, Eq, PartialEq, Hash, Debug)]
enum AppState {
    #[default]
    Title,
    Playing,
}

#[derive(Resource, Default)]
struct GameState {
    target: [f32; 3],
    player: [f32; 3],
    total_score: u32,
}

#[derive(Component, Clone, Copy)]
enum ColorChannel { Red, Green, Blue }

#[derive(Component)]
struct SliderButton { channel: ColorChannel, delta: f32 }

#[derive(Component)]
struct TargetSwatch;

#[derive(Component)]
struct PlayerSwatch;

#[derive(Component)]
struct SliderFill(ColorChannel);

#[derive(Component)]
struct ScoreText;

#[derive(Component)]
struct TitleScreen;

#[derive(Component)]
struct StartButton;

#[derive(Component)]
struct SubmitButton;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<AppState>()
        .init_resource::<GameState>()
        .add_systems(Startup, spawn_camera)
        .add_systems(OnEnter(AppState::Title), setup_title)
        .add_systems(OnExit(AppState::Title), teardown_title)
        .add_systems(OnEnter(AppState::Playing), setup_game)
        .add_systems(
            Update,
            handle_start_button.run_if(in_state(AppState::Title)),
        )
        .add_systems(
            Update,
            (
                (handle_slider_buttons, handle_submit_button),
                (update_target_swatch, update_player_swatch, update_slider_fills, update_score_text),
            )
                .chain()
                .run_if(in_state(AppState::Playing)),
        )
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn setup_title(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                row_gap: Val::Px(24.0),
                ..default()
            },
            TitleScreen,
        ))
        .with_children(|root| {
            root.spawn((
                Text::new("Palette Matcher"),
                TextFont { font_size: 56.0, ..default() },
            ));
            root.spawn((
                Text::new("Match the target color using the R, G, B sliders."),
                TextFont { font_size: 18.0, ..default() },
            ));
            root.spawn((
                Button,
                Node { padding: UiRect::all(Val::Px(20.0)), ..default() },
                BackgroundColor(Color::srgb(0.25, 0.55, 0.25)),
                StartButton,
            ))
            .with_children(|b| {
                b.spawn((Text::new("Start Game"), TextFont { font_size: 24.0, ..default() }));
            });
        });
}

fn teardown_title(mut commands: Commands, query: Query<Entity, With<TitleScreen>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

fn setup_game(mut commands: Commands, mut game_state: ResMut<GameState>) {
    let mut rng = rand::rng();
    game_state.target = [rng.random(), rng.random(), rng.random()];
    game_state.player = [0.0; 3];

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
            root.spawn((Text::new("Score: 0"), ScoreText));

            // Two swatches side by side
            root.spawn(Node {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(40.0),
                ..default()
            })
            .with_children(|row| {
                for (label, is_target) in [("Target", true), ("Your Color", false)] {
                    row.spawn(Node {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        row_gap: Val::Px(8.0),
                        ..default()
                    })
                    .with_children(|col| {
                        col.spawn(Text::new(label));
                        let mut swatch = col.spawn((
                            Node {
                                width: Val::Px(150.0),
                                height: Val::Px(150.0),
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                        ));
                        if is_target {
                            swatch.insert(TargetSwatch);
                        } else {
                            swatch.insert(PlayerSwatch);
                        }
                    });
                }
            });

            // RGB sliders
            for (label, channel, fill_color) in [
                ("R", ColorChannel::Red,   Color::srgb(0.8, 0.2, 0.2)),
                ("G", ColorChannel::Green, Color::srgb(0.2, 0.8, 0.2)),
                ("B", ColorChannel::Blue,  Color::srgb(0.2, 0.2, 0.9)),
            ] {
                root.spawn(Node {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(8.0),
                    ..default()
                })
                .with_children(|row| {
                    row.spawn(Text::new(label));

                    row.spawn((
                        Button,
                        Node { padding: UiRect::all(Val::Px(8.0)), ..default() },
                        BackgroundColor(Color::srgb(0.3, 0.3, 0.3)),
                        SliderButton { channel, delta: -STEP },
                    ))
                    .with_children(|b| { b.spawn(Text::new("<")); });

                    row.spawn(Node {
                        width: Val::Px(250.0),
                        height: Val::Px(24.0),
                        overflow: Overflow::clip(),
                        ..default()
                    })
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

                    row.spawn((
                        Button,
                        Node { padding: UiRect::all(Val::Px(8.0)), ..default() },
                        BackgroundColor(Color::srgb(0.3, 0.3, 0.3)),
                        SliderButton { channel, delta: STEP },
                    ))
                    .with_children(|b| { b.spawn(Text::new(">")); });
                });
            }

            // Submit button
            root.spawn((
                Button,
                Node { padding: UiRect::all(Val::Px(14.0)), ..default() },
                BackgroundColor(Color::srgb(0.55, 0.25, 0.25)),
                SubmitButton,
            ))
            .with_children(|b| { b.spawn(Text::new("Submit")); });
        });
}

fn handle_start_button(
    query: Query<&Interaction, (Changed<Interaction>, With<StartButton>)>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for interaction in &query {
        if *interaction == Interaction::Pressed {
            next_state.set(AppState::Playing);
        }
    }
}

fn handle_slider_buttons(
    query: Query<(&Interaction, &SliderButton), Changed<Interaction>>,
    mut game_state: ResMut<GameState>,
) {
    for (interaction, btn) in &query {
        if *interaction == Interaction::Pressed {
            let channel = match btn.channel {
                ColorChannel::Red   => &mut game_state.player[0],
                ColorChannel::Green => &mut game_state.player[1],
                ColorChannel::Blue  => &mut game_state.player[2],
            };
            *channel = (*channel + btn.delta).clamp(0.0, 1.0);
        }
    }
}

fn handle_submit_button(
    query: Query<&Interaction, (Changed<Interaction>, With<SubmitButton>)>,
    mut game_state: ResMut<GameState>,
) {
    for interaction in &query {
        if *interaction == Interaction::Pressed {
            let [tr, tg, tb] = game_state.target;
            let [pr, pg, pb] = game_state.player;
            let distance = ((tr - pr).powi(2) + (tg - pg).powi(2) + (tb - pb).powi(2)).sqrt();
            let points = ((1.0 - distance / 3.0_f32.sqrt()) * 1000.0) as u32;
            game_state.total_score += points;

            let mut rng = rand::rng();
            game_state.target = [rng.random(), rng.random(), rng.random()];
            game_state.player = [0.0; 3];
        }
    }
}

fn update_target_swatch(
    game_state: Res<GameState>,
    mut query: Query<&mut BackgroundColor, (With<TargetSwatch>, Without<PlayerSwatch>)>,
) {
    if game_state.is_changed() {
        if let Ok(mut color) = query.single_mut() {
            let [r, g, b] = game_state.target;
            *color = BackgroundColor(Color::srgb(r, g, b));
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
                ColorChannel::Red   => game_state.player[0],
                ColorChannel::Green => game_state.player[1],
                ColorChannel::Blue  => game_state.player[2],
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
