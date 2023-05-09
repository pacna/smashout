use bevy::prelude::*;


fn main() {
    App::new()
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Smashout".into(),
            resolution: (1280.0, 720.0).into(),
            ..default()
        }),
        ..default()
    }))
    .run();
}
