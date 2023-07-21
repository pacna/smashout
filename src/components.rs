use bevy::prelude::Component;

#[derive(Component)]
pub struct Count {
    pub counter: i32
}

impl Count {
    pub fn new(value: i32) -> Count {
        return Count {
            counter: value
        };
    }
}

#[derive(Component)]
pub struct Direction {
    pub x: f32,
    pub y: f32
}

impl Direction {
    pub fn new(x: f32, y: f32) -> Direction {
        return Direction {
            x: x,
            y: y
        };
    } 
}

#[derive(Component)]
pub struct Velocity {
    pub multiplier: f32
}

impl Velocity {
    pub fn new(value: f32) -> Velocity {
        return Velocity {
            multiplier: value
        }
    }
}

#[derive(Component)]
pub struct BallQueryIdentifier;

#[derive(Component)]
pub struct PlayerQueryIdentifier;

#[derive(Component)]
pub struct BrickQueryIdentifier;

#[derive(Component)]
pub struct ScoreQueryIdentifier;

#[derive(Component)]
pub struct YouWonQueryIdentifier;