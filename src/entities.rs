use bevy::{prelude::{Bundle, TextBundle}, sprite::{SpriteBundle, MaterialMesh2dBundle, Material2d}};
use crate::components::{Collided, Counter};

#[derive(Bundle)]
pub struct BrickBundle {
    pub sprite: SpriteBundle,
    pub has_collided: Collided
}

#[derive(Bundle)]
pub struct  BallBundle<T: Material2d> {
    pub sprite: MaterialMesh2dBundle<T>
}

#[derive(Bundle)]
pub struct  ScoreBoardBundle {
    pub text: TextBundle,
    pub counter: Counter
}