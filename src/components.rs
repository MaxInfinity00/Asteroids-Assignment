use specs::prelude::*;
use specs_derive::Component;
use vector2d::Vector2D;

#[derive(Component)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub rot: f64,
    pub section: u32
}

// Renderable Item and image deets
#[derive(Component)]
pub struct Renderable {
    pub tex_name: String, //Texture name
    pub i_w: u32, //Image Width
    pub i_h: u32, //Image Height
    pub o_w: u32, //Output Width
    pub o_h: u32, //Output Height
    pub frame: u32, //Current Frame
    pub total_frames: u32, //Total Frames
    pub rot: f64 //Rotation of Image
}

// Player Component
#[derive(Component)]
pub struct Player {
    pub impulse: Vector2D<f64>,
    pub cur_speed: Vector2D<f64>,
    pub lives: u32,
    pub died: bool,
    pub invulnerable: bool
}

// Asteroid Component
#[derive(Component)]
pub struct Asteroid {
    pub speed: f64,
    pub rot_speed: f64
}

#[derive(Component)]
pub struct Missile {
    pub speed: f64
}

pub struct PendingAsteroid{
    pub x: f64,
    pub y: f64,
    pub rot: f64,
    pub section: u32,
    pub size: u32
}

#[derive(Component)]
pub struct GameData{
    pub score: u32,
    pub level: u32,
    pub showControls: bool
}

// #[derive(Component)]
// pub struct Star{
//     pub size: u32
// }

#[derive(PartialEq)]
pub enum SoundCueType{
    PlaySound,
    LoopSound,
    StopSound
}

#[derive(Component)]
pub struct SoundCue{
    pub filename: String,
    pub sc_type: SoundCueType
}

