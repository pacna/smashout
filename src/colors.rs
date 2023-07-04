use bevy::{prelude::Color};
use lazy_static::lazy_static;


pub const WHITE: Color = Color::WHITE;
pub const BLACK: Color = Color::BLACK;

lazy_static! {
    pub static ref TOTEM_POLE: Color = Color::rgb_u8(163, 30, 10);
    pub static ref HOT_TODDY: Color = Color::rgb_u8(194, 133, 10);
    pub static ref SALEM: Color = Color::rgb_u8(10, 133, 51);
    pub static ref KEYLIMEPIE: Color = Color::rgb_u8(194, 194, 41);
    pub static ref LOCHMARA: Color = Color::rgb_u8(10, 133, 194);
}