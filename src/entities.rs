use bevy::{prelude::Bundle, sprite::SpriteBundle};
use crate::components::{Collided};

#[derive(Bundle)]
pub struct BrickBundle {
    pub sprite: SpriteBundle,
    pub has_collided: Collided
}