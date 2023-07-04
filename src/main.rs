use bevy::{prelude::*, core_pipeline::clear_color::ClearColorConfig};
use smashout::colors::{TOTEM_POLE, HOT_TODDY, SALEM, KEYLIMEPIE, LOCHMARA, BLACK};
use smashout::components::Collided;
use smashout::sizes::{WINDOW_WIDTH, WINDOW_HEIGHT};
use smashout::entities::{BrickBundle};

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
    .add_startup_system(change_background)
    .add_startup_system(create_shape)
    .run();
}


fn change_background(mut commands: Commands) -> () {
    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(BLACK),
        },
        ..Default::default()
    });
}

fn create_shape(mut commands: Commands) -> () {
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
