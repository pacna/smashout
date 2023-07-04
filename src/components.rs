use bevy::{prelude::{Component}};

#[derive(Component)]
pub struct Collided(bool);

impl Collided {
    pub fn new(value: bool) -> Self {
        Collided(value)
    }
}