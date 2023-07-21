use bevy::prelude::{Commands, default, Vec3, Camera2dBundle, Camera2d, Vec2};
use bevy::core_pipeline::clear_color::ClearColorConfig;

use crate::constants::colors::{HOT_TODDY, SALEM, KEYLIMEPIE, WHITE, BLACK, TOTEM_POLE, LOCHMARA};
use crate::constants::sizes::{ROW, COLUMN, BRICK_HEIGHT, BRICK_WIDTH, BRICK_BORDER};
use crate::entities::{ScoreBoardBundle, BrickBundle, BallBundle, PlayerBundle, YouWonBundle};


pub fn render_background(mut commands: Commands) -> () {
    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(BLACK),
        },
        ..default()
    });
}

pub fn render_score_board(mut commands: Commands) -> () {
    commands.spawn(ScoreBoardBundle::new(
        ScoreBoardBundle::display_score_text(0), 
        0
    ));
}

pub fn render_victory(mut commands: Commands) -> () {
    commands.spawn(YouWonBundle::new("".to_string()));
}

pub fn render_ball(mut commands: Commands) -> () {
    commands.spawn(BallBundle::new(WHITE));
}

pub fn render_bricks(mut commands: Commands) -> () {
    let mut starting_pos: Vec2 = Vec2::new(-555.0, 220.0);

    for i in 0 .. ROW {
        let mut current_x: f32 = starting_pos.x;
        let (color, point) = match i {
            2 | 3 => (*HOT_TODDY, 5),
            4 | 5 => (*SALEM, 3),
            6 | 7 => (*KEYLIMEPIE, 1),
            _ => (*TOTEM_POLE, 7),
        };

        for _ in 0 .. COLUMN {
            commands.spawn(BrickBundle::new(point, color, Vec3::new(current_x, starting_pos.y, 0.0)));
            current_x = current_x + BRICK_WIDTH + BRICK_BORDER;
        }

        starting_pos.y = starting_pos.y - BRICK_HEIGHT - BRICK_BORDER;
    }

}

pub fn render_player(mut commands: Commands) -> () {
    commands.spawn(PlayerBundle::new(*LOCHMARA));
}