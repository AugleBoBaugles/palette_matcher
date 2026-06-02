use bevy::prelude::*;
use rand::Rng;

const STEP: f32 = 0.05;
const TOTAL_ROUNDS: u32 = 3;
const SCORES_FILE: &str = "scores.txt";

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<AppState>()
        .init_resource::<GameState>()
        .init_resource::<HighScores>()
        .add_systems(Startup, (spawn_camera, load_scores))
        .add_systems(OnEnter(AppState::Title), setup_title)
        .add_systems(OnExit(AppState::Title), teardown::<TitleScreen>)
        .add_systems(OnEnter(AppState::Playing), setup_game)
        .add_systems(OnExit(AppState::Playing), teardown::<GameScreen>)
        .add_systems(OnEnter(AppState::GameOver), setup_game_over)
        .add_systems(OnExit(AppState::GameOver), teardown::<GameOverScreen>)
        .add_systems(
            Update,
            handle_start_button.run_if(in_state(AppState::Title)),
        )
        .add_systems(
            Update,
            (
                (handle_slider_buttons, handle_submit_button),
                (
                    update_target_swatch,
                    update_player_swatch,
                    update_slider_fills,
                    update_score_text,
                    update_round_text,
                ),
            )
                .chain()
                .run_if(in_state(AppState::Playing)),
        )
        .add_systems(
            Update,
            handle_play_again_button.run_if(in_state(AppState::GameOver)),
        )
        .run();
}

// ── States ────────────────────────────────────────────────────────────────────

#[derive(States, Default, Clone, Eq, PartialEq, Hash, Debug)]
enum AppState {
    #[default]
    Title,
    Playing,
    GameOver,
}

// ── Resources ─────────────────────────────────────────────────────────────────

#[derive(Resource)]
struct GameState {
    target: [f32; 3],
    player: [f32; 3],
    total_score: u32,
    round: u32,
}

impl Default for GameState {
    fn default() -> Self {
        Self { target: [0.0; 3], player: [0.0; 3], total_score: 0, round: 1 }
    }
}

#[derive(Resource, Default)]
struct HighScores(Vec<u32>);

fn load_scores(mut high_scores: ResMut<HighScores>) {
    if let Ok(content) = std::fs::read_to_string(SCORES_FILE) {
        let mut scores: Vec<u32> = content
            .lines()
            .filter_map(|l| l.trim().parse().ok())
            .collect();
        scores.sort_unstable_by(|a, b| b.cmp(a));
        scores.truncate(10);
        high_scores.0 = scores;
    }
}

fn save_scores(scores: &[u32]) {
    let content = scores.iter().map(|s| s.to_string()).collect::<Vec<_>>().join("\n");
    let _ = std::fs::write(SCORES_FILE, content);
}

// ── Components ────────────────────────────────────────────────────────────────

#[derive(Component, Clone, Copy)]
enum ColorChannel { Red, Green, Blue }

#[derive(Component)]
struct SliderButton { channel: ColorChannel, delta: f32 }

#[derive(Component)] struct TargetSwatch;
#[derive(Component)] struct PlayerSwatch;
#[derive(Component)] struct SliderFill(ColorChannel);
#[derive(Component)] struct ScoreText;
#[derive(Component)] struct RoundText;
#[derive(Component)] struct TitleScreen;
#[derive(Component)] struct GameScreen;
#[derive(Component)] struct GameOverScreen;
#[derive(Component)] struct StartButton;
#[derive(Component)] struct SubmitButton;
#[derive(Component)] struct PlayAgainButton;

// ── Generic teardown ──────────────────────────────────────────────────────────

fn teardown<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

// ── Startup ───────────────────────────────────────────────────────────────────

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

// ── Title screen ──────────────────────────────────────────────────────────────

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
                Text::new("Match the target color using R, G, B sliders. 3 rounds."),
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

// ── Game screen ───────────────────────────────────────────────────────────────

fn setup_game(mut commands: Commands, mut game_state: ResMut<GameState>) {
    *game_state = GameState::default();
    let mut rng = rand::rng();
    game_state.target = [rng.random(), rng.random(), rng.random()];

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                row_gap: Val::Px(20.0),
                ..default()
            },
            GameScreen,
        ))
        .with_children(|root| {
            // Round and score header
            root.spawn(Node {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(40.0),
                ..default()
            })
            .with_children(|row| {
                row.spawn((
                    Text::new(format!("Round 1 / {TOTAL_ROUNDS}")),
                    RoundText,
                ));
                row.spawn((Text::new("Score: 0"), ScoreText));
            });

            // Two swatches
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
                        if is_target { swatch.insert(TargetSwatch); }
                        else         { swatch.insert(PlayerSwatch); }
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

            root.spawn((
                Button,
                Node { padding: UiRect::all(Val::Px(14.0)), ..default() },
                BackgroundColor(Color::srgb(0.55, 0.25, 0.25)),
                SubmitButton,
            ))
            .with_children(|b| { b.spawn(Text::new("Submit")); });
        });
}

fn handle_slider_buttons(
    query: Query<(&Interaction, &SliderButton), Changed<Interaction>>,
    mut game_state: ResMut<GameState>,
) {
    for (interaction, btn) in &query {
        if *interaction == Interaction::Pressed {
            let ch = match btn.channel {
                ColorChannel::Red   => &mut game_state.player[0],
                ColorChannel::Green => &mut game_state.player[1],
                ColorChannel::Blue  => &mut game_state.player[2],
            };
            *ch = (*ch + btn.delta).clamp(0.0, 1.0);
        }
    }
}

fn handle_submit_button(
    query: Query<&Interaction, (Changed<Interaction>, With<SubmitButton>)>,
    mut game_state: ResMut<GameState>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for interaction in &query {
        if *interaction == Interaction::Pressed {
            let [tr, tg, tb] = game_state.target;
            let [pr, pg, pb] = game_state.player;
            let dist = ((tr-pr).powi(2) + (tg-pg).powi(2) + (tb-pb).powi(2)).sqrt();
            let points = ((1.0 - dist / 3.0_f32.sqrt()) * 1000.0) as u32;
            game_state.total_score += points;

            if game_state.round >= TOTAL_ROUNDS {
                next_state.set(AppState::GameOver);
            } else {
                game_state.round += 1;
                let mut rng = rand::rng();
                game_state.target = [rng.random(), rng.random(), rng.random()];
                game_state.player = [0.0; 3];
            }
        }
    }
}

fn update_target_swatch(
    gs: Res<GameState>,
    mut q: Query<&mut BackgroundColor, (With<TargetSwatch>, Without<PlayerSwatch>)>,
) {
    if gs.is_changed() {
        if let Ok(mut c) = q.single_mut() {
            let [r, g, b] = gs.target;
            *c = BackgroundColor(Color::srgb(r, g, b));
        }
    }
}

fn update_player_swatch(
    gs: Res<GameState>,
    mut q: Query<&mut BackgroundColor, (With<PlayerSwatch>, Without<TargetSwatch>)>,
) {
    if gs.is_changed() {
        if let Ok(mut c) = q.single_mut() {
            let [r, g, b] = gs.player;
            *c = BackgroundColor(Color::srgb(r, g, b));
        }
    }
}

fn update_slider_fills(gs: Res<GameState>, mut q: Query<(&mut Node, &SliderFill)>) {
    if gs.is_changed() {
        for (mut node, fill) in &mut q {
            let v = match fill.0 {
                ColorChannel::Red   => gs.player[0],
                ColorChannel::Green => gs.player[1],
                ColorChannel::Blue  => gs.player[2],
            };
            node.width = Val::Percent(v * 100.0);
        }
    }
}

fn update_score_text(gs: Res<GameState>, mut q: Query<&mut Text, With<ScoreText>>) {
    if gs.is_changed() {
        if let Ok(mut t) = q.single_mut() {
            t.0 = format!("Score: {}", gs.total_score);
        }
    }
}

fn update_round_text(gs: Res<GameState>, mut q: Query<&mut Text, With<RoundText>>) {
    if gs.is_changed() {
        if let Ok(mut t) = q.single_mut() {
            t.0 = format!("Round {} / {}", gs.round, TOTAL_ROUNDS);
        }
    }
}

// ── Game over screen ──────────────────────────────────────────────────────────

fn setup_game_over(
    mut commands: Commands,
    game_state: Res<GameState>,
    mut high_scores: ResMut<HighScores>,
) {
    let new_score = game_state.total_score;
    high_scores.0.push(new_score);
    high_scores.0.sort_unstable_by(|a, b| b.cmp(a));
    high_scores.0.truncate(10);
    save_scores(&high_scores.0);

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                row_gap: Val::Px(16.0),
                ..default()
            },
            GameOverScreen,
        ))
        .with_children(|root| {
            root.spawn((
                Text::new("Game Over!"),
                TextFont { font_size: 48.0, ..default() },
            ));
            root.spawn((
                Text::new(format!("Your score: {new_score}")),
                TextFont { font_size: 28.0, ..default() },
            ));
            root.spawn((
                Text::new("-- High Scores --"),
                TextFont { font_size: 22.0, ..default() },
            ));

            for (i, &score) in high_scores.0.iter().enumerate() {
                let label = if score == new_score && i == high_scores.0.iter().position(|&s| s == new_score).unwrap_or(usize::MAX) {
                    format!("{}. {} <-- you", i + 1, score)
                } else {
                    format!("{}. {}", i + 1, score)
                };
                root.spawn((Text::new(label), TextFont { font_size: 20.0, ..default() }));
            }

            root.spawn((
                Button,
                Node { padding: UiRect::all(Val::Px(16.0)), margin: UiRect::top(Val::Px(16.0)), ..default() },
                BackgroundColor(Color::srgb(0.25, 0.55, 0.25)),
                PlayAgainButton,
            ))
            .with_children(|b| {
                b.spawn((Text::new("Play Again"), TextFont { font_size: 22.0, ..default() }));
            });
        });
}

fn handle_play_again_button(
    query: Query<&Interaction, (Changed<Interaction>, With<PlayAgainButton>)>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for interaction in &query {
        if *interaction == Interaction::Pressed {
            next_state.set(AppState::Title);
        }
    }
}
