use bevy::{prelude::{default, PluginGroup, App, Update, Last, Startup}, DefaultPlugins};
use bevy::window::{WindowPlugin, Window};
use smashout::handler::{move_player, move_ball, check_window_collision, check_player_collision, check_brick_collision, you_won};
use smashout::renderer::{render_score_board, render_ball, render_background, render_bricks, render_player, render_victory};
use smashout::constants::sizes::{WINDOW_WIDTH, WINDOW_HEIGHT};

fn main() -> () {

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
    .add_systems(Startup, (
        render_background,
        render_score_board,
        render_victory,
        render_bricks,
        render_ball,
        render_player
    ))
    .add_systems(Update,(
        move_ball,
        move_player
    ))
    .add_systems(Update, (
        check_window_collision,
        check_player_collision,
        check_brick_collision
    ))
    .add_systems(Last, you_won)
    .run();
}
