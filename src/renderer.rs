use bevy::prelude::{Commands, AssetServer, Res, default, ResMut, Assets, Mesh, Transform, Vec3, Vec2, shape, Camera2dBundle, Camera2d};
use bevy::ui::{Val, UiRect};
use bevy::sprite::{ColorMaterial, MaterialMesh2dBundle, SpriteBundle, Sprite};
use bevy::core_pipeline::clear_color::ClearColorConfig;

use crate::colors::{HOT_TODDY, SALEM, KEYLIMEPIE, WHITE, BLACK, TOTEM_POLE, LOCHMARA};
use crate::sizes::{ROW, COLUMN, BRICK_HEIGHT, BRICK_WIDTH, BRICK_BORDER};
use crate::entities::{ScoreBoardBundle, BrickBundle};


pub fn render_background(mut commands: Commands) -> () {
    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(BLACK),
        },
        ..default()
    });
}

pub fn render_score_board(mut commands: Commands, asset_server: Res<AssetServer>) -> () {
    let score: i32 = 0;

    commands.spawn(ScoreBoardBundle::new(
        format!("Score {}", score), 
        score, 
        asset_server.load("fonts/OpenSans-Regular.ttf"), 
        UiRect { left: Val::Px(40.0), top: Val::Px(40.0), ..default()}
    ));
}

pub fn render_ball(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) -> () {
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(10.0).into()).into(),
        material: materials.add(ColorMaterial::from(WHITE)),
        transform: Transform::from_translation(Vec3::new(0.0, -150.0, 0.0)),
        ..default()
    });
}

pub fn render_bricks(mut commands: Commands) -> () {
    let starting_x: f32 = -555.0;
    let mut starting_y: f32 = 220.0;

    for i in 0 .. ROW {
        let mut current_x: f32 = starting_x;
        let (color, point) = match i {
            2 | 3 => (*HOT_TODDY, 5),
            4 | 5 => (*SALEM, 3),
            6 | 7 => (*KEYLIMEPIE, 1),
            _ => (*TOTEM_POLE, 7),
        };

        for _ in 0 .. COLUMN {
            commands.spawn(BrickBundle::new(point, color, Vec3::new(current_x, starting_y, 0.0)));
            current_x = current_x + BRICK_HEIGHT + BRICK_BORDER;
        }
        starting_y = starting_y - BRICK_WIDTH - BRICK_BORDER;
    }

}

pub fn render_player(mut commands: Commands) -> () {
    commands.spawn(SpriteBundle { 
        sprite: Sprite {
            color: *LOCHMARA,
            custom_size: Some(Vec2::new(BRICK_HEIGHT, BRICK_WIDTH)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(0.0, -300.0, 0.0)),
        ..default()
    });
}