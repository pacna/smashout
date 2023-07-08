use bevy::{prelude::{Component}};

#[derive(Component)]
pub struct Collided(bool);

impl Collided {
    pub fn new(value: bool) -> Collided {
        return Collided(value);
    }
}

#[derive(Component)]
pub struct Count(i32);

impl Count {
    pub fn new(value: i32) -> Count {
        return Count(value);
    }
}