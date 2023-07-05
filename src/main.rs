use bevy::sprite::MaterialMesh2dBundle;
use bevy::{prelude::*, core_pipeline::clear_color::ClearColorConfig};
use smashout::colors::{TOTEM_POLE,  BLACK, WHITE};
use smashout::components::{Collided, Counter};
use smashout::sizes::{WINDOW_WIDTH, WINDOW_HEIGHT};
use smashout::entities::{BrickBundle, BallBundle, ScoreBoardBundle};

fn main() {

    App::new()
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Smashout".into(),
            resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
            ..default()
        }),
        ..default()
    }))
    .add_startup_system(create_score_board)
    .add_startup_system(change_background)
    .add_startup_system(create_bricks)
    .add_startup_system(create_ball)
    .run();
}

fn create_score_board(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(ScoreBoardBundle {
        text: TextBundle::from_section(
            "Score!",
            TextStyle {
                font: asset_server.load("fonts/OpenSans-Regular.ttf"),
                font_size: 50.0,
                color: Color::WHITE,
            },
        )
        .with_text_alignment(TextAlignment::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                bottom: Val::Px(50.0),
                right: Val::Px(50.0),
                ..default()
            },
            ..default()
        }),
        counter: Counter::new(0)
    });
}

fn create_ball(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) -> () {
    commands.spawn(BallBundle {
        sprite: MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(50.).into()).into(),
            material: materials.add(ColorMaterial::from(WHITE)),
            transform: Transform::from_translation(Vec3::new(-150., 0., 0.)),
            ..default()
        }
    });
}


fn change_background(mut commands: Commands) -> () {
    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(BLACK),
        },
        ..default()
    });
}

fn create_bricks(mut commands: Commands) -> () {
    for _ in 1..2 {
        commands.spawn(BrickBundle {
            sprite: SpriteBundle { sprite: Sprite { 
                color: *TOTEM_POLE,
                custom_size: Some(Vec2::new(50.0, 100.0)),
                ..default()
            }, 
            transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
            ..default() 
            },
            has_collided: Collided::new(false)
        });
    }

}
