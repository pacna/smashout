use bevy::{prelude::{default, PluginGroup, App }, DefaultPlugins, window::{WindowPlugin, Window}};
use smashout::renderer::{render_score_board, render_ball, render_background, render_bricks, render_player};
use smashout::sizes::{WINDOW_WIDTH, WINDOW_HEIGHT};

fn main() {

    App::new()
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Smashout".to_string(),
            resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
            resizable: false,
            ..default()
        }),
        ..default()
    }))
    .add_startup_system(render_background)
    .add_startup_system(render_score_board)
    .add_startup_system(render_bricks)
    .add_startup_system(render_ball)
    .add_startup_system(render_player)
    .run();
}
