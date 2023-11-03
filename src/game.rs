use std::collections::HashMap;
use specs::{World, WorldExt, Builder, Join};

use rand::Rng;

use crate::{components, SHOOT_FILENAME};
use crate::utils;

const ROTATION_SPEED: f64 = 120.0;
const IMPULSE_SPEED: f64 = 300.0;
pub fn update(ecs: &mut World, key_manager: &mut HashMap<String,bool>, deltaTime: f64){
    //Check status of the game world
    let mut must_reload_world = false;
    let mut current_player_position = components::Position{x:0.0, y: 0.0, rot: 0.0};

    {
        let mut players = ecs.write_storage::<crate::components::Player>();
        let mut positions = ecs.write_storage::<crate::components::Position>();
        for(pos,player) in (&mut positions,&mut players).join(){
            if player.died {
                player.died = false;
                player.lives -= 1;
                player.cur_speed.x = 0.0;
                player.cur_speed.y = 0.0;
                player.impulse.x = 0.0;
                player.impulse.y = 0.0;
                pos.x = crate::SCREEN_WIDTH as f64 / 2.0;
                pos.y = crate::SCREEN_HEIGHT as f64 / 2.0;
            }
            current_player_position.x = pos.x;
            current_player_position.y = pos.y;
        }
        if players.join().count() < 1 {
            must_reload_world = true;
        }

    }

    if must_reload_world {
        ecs.delete_all();
        load_world(ecs);
    }

    let mut must_create_asteroid = false;
    let mut number_asteroids: u32 = 0;
    {
        let asteroids = ecs.read_storage::<crate::components::Asteroid>();
        if asteroids.join().count() < 1 {
            must_create_asteroid = true;

            let mut gamedatas = ecs.write_storage::<crate::components::GameData>();
            for mut gamedata in (&mut gamedatas).join(){
                gamedata.level += 1;
                number_asteroids = (gamedata.level /3) + 1;
            }
        }
    }

    if must_create_asteroid {
        let mut asteroid_count: u32 = 0;
        while asteroid_count < number_asteroids {
            let mut rng = rand::thread_rng();
            let next_x = rng.gen_range(50.0..crate::SCREEN_WIDTH as f64 - 50.0);
            let next_y = rng.gen_range(50.0..crate::SCREEN_HEIGHT as f64 - 50.0);
            let next_rot = rng.gen_range(0.0..360.0);

            let diff_x = (next_x - current_player_position.x).abs();
            let diff_y = (next_y - current_player_position.y).abs();
            let dist = (diff_x * diff_x + diff_y * diff_y).sqrt();
            if dist < 150.0 {
                continue;
            }

            asteroid_count += 1;
            let new_asteroid = components::Position{
                x: next_x,
                y: next_y,
                rot: next_rot
            };
            create_asteroid(ecs,new_asteroid,100);
        }

    }

    let mut player_pos = components::Position{x: 0.0, y: 0.0, rot: 0.0};
    let mut must_fire_missile = false;
    let mut thruster_pushed = false;
    {
        let mut positions =  ecs.write_storage::<crate::components::Position>();
        let mut players = ecs.write_storage::<crate::components::Player>();
        let mut renderables = ecs.write_storage::<crate::components::Renderable>();

        for(player,pos, renderable) in (&mut players, &mut positions, &mut renderables).join(){

            if crate::utils::is_key_pressed(&key_manager, "D"){
                pos.rot += ROTATION_SPEED * deltaTime;
                thruster_pushed = true;
            }
            if crate::utils::is_key_pressed(&key_manager, "A"){
                pos.rot -= ROTATION_SPEED*deltaTime;
                thruster_pushed = true;
            }
            if pos.rot > 360.0 {
                pos.rot -= 360.0;
            }
            if pos.rot < 0.0 {
                pos.rot += 360.0;
            }

            if crate::utils::is_key_pressed(&key_manager, "W"){
                player.impulse.y -= pos.rot.to_radians().cos() * IMPULSE_SPEED;
                player.impulse.x += pos.rot.to_radians().sin() * IMPULSE_SPEED;
                thruster_pushed = true;
            }
            update_movement(pos,player,deltaTime as f64);

            if pos.x > crate::SCREEN_WIDTH.into() {
                pos.x -= crate::SCREEN_WIDTH as f64;
            }
            else if pos.x < 0.0 {
                pos.x += crate::SCREEN_WIDTH as f64;
            }

            if pos.y > crate::SCREEN_HEIGHT.into() {
                pos.y -= crate::SCREEN_HEIGHT as f64;
            }
            else if pos.y < 0.0 {
                pos.y += crate::SCREEN_HEIGHT as f64;
            }

            if utils::is_key_pressed(&key_manager, " "){
                utils::key_up(key_manager, " ".to_string());
                must_fire_missile = true;
                player_pos.x = pos.x;
                player_pos.y = pos.y;
                player_pos.rot = pos.rot;
            }
            //Update the graphic to reflect the rotation
            renderable.rot = pos.rot;
        }
    }

    if thruster_pushed {
        ecs.create_entity()
            .with(components::SoundCue{
                filename: crate::THRUSTER_FILENAME.to_string(),
                sc_type: components::SoundCueType::LoopSound
            })
            .build();
    }
    else{
        ecs.create_entity()
            .with(components::SoundCue{
                filename: crate::THRUSTER_FILENAME.to_string(),
                sc_type: components::SoundCueType::StopSound
            })
            .build();
    }

    if must_fire_missile {
        fire_missile(ecs, player_pos);
    }
}

const FRICTION: f64 = 2.5;
const MAX_SPEED: f64 = 500.0;
pub fn update_movement(pos: &mut crate::components::Position, player: &mut crate::components::Player, deltaTime: f64){
    // player.cur_speed*=FRICTION;
    player.cur_speed -= player.cur_speed * (FRICTION * deltaTime);

    player.cur_speed+=player.impulse;

    if player.cur_speed.length() > MAX_SPEED {
        player.cur_speed *= MAX_SPEED/player.cur_speed.length();
    }

    pos.x += player.cur_speed.x * deltaTime;
    pos.y += player.cur_speed.y * deltaTime;

    player.impulse = vector2d::Vector2D::new(0.0,0.0);
}

// pub const MAX_STARS: u32 = 100;

pub fn load_world(ecs: &mut World){
    ecs.create_entity()
        .with(crate::components::Position{x: 350.0, y: 250.0, rot: 0.0})
        .with(crate:: components::Renderable{
            tex_name: String::from("img/ship.png"),
            i_w: 100,
            i_h: 100,
            o_w: 50,
            o_h: 50,
            frame: 0,
            total_frames: 1,
            rot: 0.0
        })
        .with(crate::components::Player{
            impulse: vector2d::Vector2D::new(0.0,0.0),
            cur_speed: vector2d::Vector2D::new(0.0,0.0),
            lives: 3,
            died: false,
            invulnerable: false
        })
        .build();

    create_asteroid(ecs, components::Position{x: 400.0, y: 235.0, rot: 45.0},50);

    ecs.create_entity()
        .with(crate::components::GameData{
            score: 0,
            level: 1
        })
        .build();

    // for _ in 0..MAX_STARS { //Create Stars
    //     let mut rng = rand::thread_rng();
    //     let next_x = rng.gen_range(0.0..crate::SCREEN_WIDTH as f64);
    //     let next_y = rng.gen_range(0.0..crate::SCREEN_HEIGHT as f64);
    //     let next_size = rng.gen_range(1..4);
    //     ecs.create_entity()
    //         .with(crate::components::Position{
    //             x: next_x,
    //             y: next_y,
    //             rot: 0.0
    //         })
    //         .with(crate::components::Star{
    //             size: next_size
    //         })
    //         .build();
    // }
}

const MAX_MISSILES: usize = 5;

fn fire_missile(ecs: &mut World, position: components::Position){
    {
        let missiles = ecs.read_storage::<crate::components::Missile>();
        if missiles.count() > MAX_MISSILES - 1{
            return;
        }
    }

    ecs.create_entity()
        .with(position)
        .with(crate::components::Renderable{
            tex_name: String::from("img/missile.png"),
            i_w: 100,
            i_h: 100,
            o_w: 25,
            o_h: 25,
            frame: 0,
            total_frames: 1,
            rot: 0.0
        })
        .with(crate::components::Missile{
            speed: 600.0
        })
        .build();

    ecs.create_entity()
        .with(components::SoundCue{
            filename: crate::SHOOT_FILENAME.to_string(),
            sc_type: components::SoundCueType::PlaySound
        })
        .build();
}

pub fn create_asteroid(ecs: &mut World, position: components::Position, asteroid_size: u32){
    ecs.create_entity()
        .with(position)
        .with(crate::components::Renderable{
            tex_name: String::from("img/asteroid1.png"),
            i_w: 100,
            i_h: 100,
            o_w: asteroid_size,
            o_h: asteroid_size,
            frame: 0,
            total_frames: 1,
            rot: 0.0
        })
        .with(crate::components::Asteroid{
            speed: 150.0,
            rot_speed: 150.0
        })
        .build();
}

pub fn toggle_invincibility(ecs: &mut World){
    let mut players = ecs.write_storage::<crate::components::Player>();
    for mut player in (&mut players).join(){
        player.invulnerable = !player.invulnerable;
    }
}

pub fn create_thousand_asteroids(ecs: &mut World){
    let mut rng = rand::thread_rng();
    for i in 0..1000 {
        let x = rng.gen_range(50.0..crate::SCREEN_WIDTH as f64 - 50.0);
        let y = rng.gen_range(50.0..crate::SCREEN_HEIGHT as f64 - 50.0);
        let rot = rng.gen_range(0.0..360.0);
        create_asteroid(ecs, components::Position{x, y, rot }, 50);
    }
}