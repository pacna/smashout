use bevy::prelude::{default, Color, Vec3, Vec2, Transform, Bundle, TextBundle};
use bevy::sprite::{SpriteBundle, Sprite};
use bevy::text::{TextAlignment, TextStyle};
use bevy:: ui::{Style, PositionType, Val};

use crate::constants::colors::WHITE;
use crate::constants::sizes::{BRICK_HEIGHT, BRICK_WIDTH, BALL_HEIGHT_WIDTH, WINDOW_WIDTH, WINDOW_HEIGHT, FONT_SIZE};
use crate::components::{Count, BallQueryIdentifier, Direction, BrickQueryIdentifier, Velocity, PlayerQueryIdentifier, ScoreQueryIdentifier, YouWonQueryIdentifier};


#[derive(Bundle)]
pub struct BrickBundle {
    pub brick: SpriteBundle,
    pub cost: Count,
    pub query: BrickQueryIdentifier
}

impl BrickBundle {
    pub fn new(point: i32, color: Color, pos: Vec3) -> BrickBundle {
        return BrickBundle { 
            brick: SpriteBundle { 
                sprite: Sprite {
                    color: color,
                    custom_size: Some(Vec2::new(BRICK_WIDTH, BRICK_HEIGHT)),
                    ..default()
                },
                transform: Transform::from_translation(pos),
                ..default()
            },
            cost: Count::new(point),
            query: BrickQueryIdentifier
        }
    }
}

#[derive(Bundle)]
pub struct BallBundle {
    pub ball: SpriteBundle,
    pub query: BallQueryIdentifier,
    pub direction: Direction,
    pub velocity: Velocity
}

impl BallBundle {
    pub fn new(color: Color) -> BallBundle {
        return BallBundle {
            ball: SpriteBundle {
                sprite: Sprite {
                    color: color,
                    custom_size: Some(Vec2::new(BALL_HEIGHT_WIDTH, BALL_HEIGHT_WIDTH)),
                    ..default()
                },
                transform: Transform::from_translation(BallBundle::reset_pos()),
                ..default()
            },
            query: BallQueryIdentifier,
            direction: Direction::new(-1.0, -1.0),
            velocity: Velocity::new(1.0)
        };
    }

    pub fn reset_pos() -> Vec3 {
        return Vec3::new(0.0, -150.0, 0.0)
    }
}

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: SpriteBundle,
    pub query: PlayerQueryIdentifier,
}

impl PlayerBundle {
    pub fn new(color: Color) -> PlayerBundle {
        return PlayerBundle { 
            player: SpriteBundle { 
                sprite: Sprite {
                    color: color,
                    custom_size: Some(Vec2::new(BRICK_WIDTH, BRICK_HEIGHT)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(0.0, -300.0, 0.0)),
                ..default()
            },
            query: PlayerQueryIdentifier
        }
    }
}

#[derive(Bundle)]
pub struct ScoreBoardBundle {
    pub score: TextBundle,
    pub counter: Count,
    pub query: ScoreQueryIdentifier
}

impl ScoreBoardBundle {
    pub fn new(text: String, score: i32) -> ScoreBoardBundle {
        return ScoreBoardBundle { score: TextBundle::from_section(text, TextStyle { 
                font_size: FONT_SIZE, color: WHITE, ..default()
            })
            .with_text_alignment(TextAlignment::Center)
            .with_style(Style {
                position_type: PositionType::Absolute,
                left: Val::Px(40.0),
                top: Val::Px(40.0),
                ..default()
            }), 
            counter: Count::new(score) ,
            query: ScoreQueryIdentifier
        }
    }

    pub fn display_score_text(point: i32) -> String {
        return format!("Score {}", point);
    }
}

#[derive(Bundle)]
pub struct YouWonBundle {
    pub won: TextBundle,
    pub query: YouWonQueryIdentifier
}

impl YouWonBundle {
    pub fn new(text: String) -> YouWonBundle {
        return YouWonBundle { 
            won: TextBundle::from_section(text, TextStyle { 
                font_size: FONT_SIZE, color: WHITE, ..default()
            })
            .with_text_alignment(TextAlignment::Center)
            .with_style(Style {
                position_type: PositionType::Absolute,
                left: Val::Px((WINDOW_WIDTH / 2.0) - FONT_SIZE),
                top: Val::Px((WINDOW_HEIGHT / 2.0) - FONT_SIZE),
                ..default()
            }),
            query: YouWonQueryIdentifier
        }
    }
}