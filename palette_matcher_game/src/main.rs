use bevy::prelude::*;
use rand::Rng;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, handle_start_button)
        .run();
}

#[derive(Component)]
struct TargetColorDisplay;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(30.0),
            ..default()
        })
        .with_children(|parent| {
            // The color swatch the player needs to match
            parent.spawn((
                Node {
                    width: Val::Px(200.0),
                    height: Val::Px(200.0),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                TargetColorDisplay,
            ));

            // Start button
            parent
                .spawn((
                    Button,
                    Node {
                        padding: UiRect::all(Val::Px(16.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.25, 0.55, 0.25)),
                ))
                .with_children(|parent| {
                    parent.spawn(Text::new("Start"));
                });
        });
}

fn handle_start_button(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>)>,
    mut swatch_query: Query<&mut BackgroundColor, With<TargetColorDisplay>>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            let mut rng = rand::rng();
            let r: f32 = rng.random();
            let g: f32 = rng.random();
            let b: f32 = rng.random();

            if let Ok(mut color) = swatch_query.single_mut() {
                *color = BackgroundColor(Color::srgb(r, g, b));
            }
        }
    }
}
