use bevy::prelude::{default, Color, Vec3, Vec2, Transform, Bundle, TextBundle, Handle };
use bevy::sprite::{SpriteBundle, Sprite};
use bevy::text::{TextAlignment, TextStyle, Font};
use bevy:: ui::{Style, PositionType, UiRect};

use crate::sizes::{BRICK_HEIGHT, BRICK_WIDTH};
use crate::{components::{Collided, Count}, colors::WHITE};

#[derive(Bundle)]
pub struct BrickBundle {
    pub brick: SpriteBundle,
    pub has_collided: Collided,
    pub cost: Count
}

impl BrickBundle {
    pub fn new(point: i32, color: Color, pos: Vec3) -> BrickBundle {
        return BrickBundle { 
            brick: SpriteBundle { 
                sprite: Sprite {
                    color: color,
                    custom_size: Some(Vec2::new(BRICK_HEIGHT, BRICK_WIDTH)),
                    ..default()
                },
                transform: Transform::from_translation(pos),
                ..default()
            },
            has_collided: Collided::new(false),
            cost: Count::new(point)
        }
    }
}


#[derive(Bundle)]
pub struct  ScoreBoardBundle {
    pub score_text: TextBundle,
    pub counter: Count
}

impl ScoreBoardBundle {
    pub fn new(text: String, score: i32, font: Handle<Font>, pos: UiRect) -> ScoreBoardBundle {
        return ScoreBoardBundle { score_text: TextBundle::from_section(text, TextStyle { 
                font: font, font_size: 50.0, color: WHITE 
            })
            .with_text_alignment(TextAlignment::Center)
            .with_style(Style {
                position_type: PositionType::Absolute,
                position: pos,
                ..default()
            }), 
            counter: Count::new(score) 
        }
    }
}