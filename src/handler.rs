use bevy::prelude::{Input, KeyCode, Res, Query, Transform, With, Mut, Without, Entity, Commands};
use bevy::sprite::{Sprite, collide_aabb::{collide, Collision}};
use bevy::text::Text;
use bevy::window::Window;

use crate::components::{BrickQueryIdentifier, PlayerQueryIdentifier, BallQueryIdentifier, Direction, Velocity, ScoreQueryIdentifier, Count, YouWonQueryIdentifier};
use crate::constants::sizes::{BALL_HEIGHT_WIDTH, BRICK_WIDTH};
use crate::constants::movements::{VELOCITY_MULTIPLIER, BALL_SPEED, PLAYER_SPEED};
use crate::entities::{BallBundle, ScoreBoardBundle};

pub fn move_player(
    mut windows: Query<&mut Window>, 
    keyboard_input: Res<Input<KeyCode>>, 
    mut query: Query<&mut Transform, With<PlayerQueryIdentifier>>
) -> () {
    let mut player_transform: Mut<'_, Transform>  = query.single_mut();
    let window: Mut<'_, Window> = windows.single_mut();

    let screen_width: f32 = (window.width() / 2.0) - (BRICK_WIDTH / 2.0);

    if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
        player_transform.translation.x -= PLAYER_SPEED;
    }

    if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
        player_transform.translation.x += PLAYER_SPEED;
    }

    player_transform.translation.x = player_transform.translation.x.clamp(-screen_width, screen_width);
}

pub fn move_ball(
    mut windows: Query<&mut Window>, 
    mut query: Query<(&mut Transform, &Direction, &mut Velocity), With<BallQueryIdentifier>>
) -> () {
    let (mut ball_transform,ball_direction,  mut ball_velocity) = query.single_mut();
    
    let window = windows.single_mut();
    let screen_height: f32 = (window.height() / 2.0) - (BALL_HEIGHT_WIDTH / 2.0);

    ball_transform.translation.x += ball_direction.x * BALL_SPEED;
    ball_transform.translation.y += ball_direction.y * BALL_SPEED * ball_velocity.multiplier;

    ball_transform.translation.y = ball_transform.translation.y.clamp(-screen_height, screen_height);

    if ball_transform.translation.y <= -screen_height {
        ball_transform.translation = BallBundle::reset_pos();
        ball_velocity.multiplier = 1.0;
    }
}

pub fn check_window_collision(
    mut windows: Query<&mut Window>,
    mut ball_query: Query<(&mut Transform, &mut Direction), With<BallQueryIdentifier>> 
) -> () {
    let window: Mut<'_, Window> = windows.single_mut();
    let screen_height: f32 = (window.height() / 2.0) - (BALL_HEIGHT_WIDTH / 2.0);
    let screen_width: f32 = (window.width() / 2.0) - (BALL_HEIGHT_WIDTH / 2.0);

    let (ball_transform, mut ball_direction) = ball_query.single_mut();

    if ball_transform.translation.y >= screen_height {
        ball_direction.y = -ball_direction.y; 
    }

    if  ball_transform.translation.x <= -screen_width || ball_transform.translation.x >= screen_width {
        ball_direction.x = -ball_direction.x;
    }
}

pub fn check_player_collision(
    mut ball_query: Query<(&mut Transform, &Sprite, &mut Direction, &mut Velocity), With<BallQueryIdentifier>>,
    mut player_query: Query<(&mut Transform, &Sprite), (Without<BallQueryIdentifier>, With<PlayerQueryIdentifier>)>,
) -> () {
    let (ball_transform, ball_sprite, mut ball_direction, mut ball_velocity) = ball_query.single_mut();
    let (player_transform, player_sprite) = player_query.single_mut();

    let collision: Option<Collision> = collide(
        ball_transform.translation, 
        ball_sprite.custom_size.unwrap_or_default(), 
        player_transform.translation, 
        player_sprite.custom_size.unwrap_or_default());

    collision_detection(collision, &mut ball_direction, &mut ball_velocity);
}

pub fn check_brick_collision(
    mut commands: Commands,
    mut ball_query: Query<(&mut Transform, &Sprite, &mut Direction, &mut Velocity), With<BallQueryIdentifier>>,
    brick_query: Query<(&mut Transform, &Sprite, Entity, &Count), (Without<BallQueryIdentifier>, With<BrickQueryIdentifier>)>,
    mut score_board_query: Query<(&mut Text, &mut Count), (With<ScoreQueryIdentifier>, Without<BrickQueryIdentifier>)>

) -> () {
    let (ball_transform, ball_sprite, mut ball_direction, mut ball_velocity) = ball_query.single_mut();
    let (mut score_text, mut current_score) = score_board_query.single_mut();

    for(brick_transform, brick_sprite, brick_entity, brick_cost) in &brick_query {
        let collision: Option<Collision> = collide(
            ball_transform.translation, 
            ball_sprite.custom_size.unwrap_or_default(), 
            brick_transform.translation, 
            brick_sprite.custom_size.unwrap_or_default());

        if collision_detection(collision, &mut ball_direction, &mut ball_velocity) {
            current_score.counter += brick_cost.counter;
            score_text.sections[0].value = ScoreBoardBundle::display_score_text(current_score.counter);
            commands.entity(brick_entity).despawn();
        }
    }
}

pub fn you_won(
    brick_query: Query<With<BrickQueryIdentifier>>,
    mut victory_query: Query<&mut Text, With<YouWonQueryIdentifier>>
) -> () {
    let mut victory_text: Mut<'_, Text> = victory_query.single_mut();

    if brick_query.iter().count() <= 0 {
        victory_text.sections[0].value = "You won!".to_string();
    }
}

fn collision_detection(collision: Option<Collision>, ball_direction: &mut Direction, ball_velocity: &mut Velocity) -> bool {
    if let Some(current_collision_type) = collision {
        let mut reflect_x: bool = false;
        let mut reflect_y: bool = false;

        match current_collision_type {
            Collision::Left => reflect_x = ball_direction.x > 0.0,
            Collision::Right => reflect_x = ball_direction.x < 0.0,
            Collision::Top => reflect_y = ball_direction.y < 0.0,
            Collision::Bottom => reflect_y = ball_direction.y > 0.0,
            Collision::Inside => {}
        }

        if reflect_x {
            ball_direction.x = -ball_direction.x;
        }

        if reflect_y {
            ball_direction.y = -ball_direction.y;
            ball_velocity.multiplier *= VELOCITY_MULTIPLIER;
        }

        return true;
    }

    return false;
}
