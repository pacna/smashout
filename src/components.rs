use bevy::{prelude::{Component}};

#[derive(Component)]
pub struct Collided(bool);

impl Collided {
    pub fn new(value: bool) -> Collided {
        return Collided(value);
    }
}

#[derive(Component)]
pub struct Counter(i32);

impl Counter {
    pub fn new(value: i32) -> Counter {
        return Counter(value);
    }
}