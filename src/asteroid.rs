use specs::{System, WriteStorage, Join, Read};
use specs::prelude::Entities;

pub struct AsteroidMover;

use crate::components;

impl<'a> System<'a> for AsteroidMover{
    type SystemData = (
        WriteStorage<'a, components::Position>,
        WriteStorage<'a, components::Renderable>,
        WriteStorage<'a, components::Asteroid>,
        Read<'a,crate::DeltaTime>
    );

    fn run(&mut self, mut data: Self::SystemData) {
        let deltatime = data.3.0;
        for(pos,rend,asteroid) in (&mut data.0, &mut data.1, &data.2).join(){
            let radians = pos.rot.to_radians();

            pos.x += asteroid.speed * radians.sin() * deltatime;
            pos.y -= asteroid.speed * radians.cos() * deltatime;

            let half_width = (rend.o_w / 2) as u32;
            let half_height = (rend.o_h / 2) as u32;

            if pos.x > (crate::SCREEN_WIDTH - half_width).into()
                || pos.x < half_width.into(){
                    pos.rot = 360.0 - pos.rot;
            } else if pos.y > (crate::SCREEN_HEIGHT - half_height).into()
                || pos.y < half_height.into(){
                    if pos.rot > 180.0 {
                        pos.rot = 540.0 - pos.rot;
                    } else {
                        pos.rot = 180.0 - pos.rot;
                    }
            }

            rend.rot += asteroid.rot_speed * deltatime;
            if rend.rot > 360.0 {
                rend.rot -= 360.0;
            }
            if rend.rot < 0.0 {
                rend.rot += 360.0;
            }

        }
    }
}

pub struct AsteroidCollider;

impl<'a> System<'a> for AsteroidCollider{
    type SystemData = (
        WriteStorage<'a, components::Position>,
        WriteStorage<'a, components::Renderable>,
        WriteStorage<'a, components::Player>,
        WriteStorage<'a, components::Asteroid>,
        Entities<'a>
    );

    // fn run(&mut self, mut data: Self::SystemData) {
    fn run(&mut self, data: Self::SystemData) {
        let (positions, rends, mut players, asteroids, entities) = data;
        for(players_pos, player_rend, player, entity) in (&positions,&rends, &mut players, &entities).join(){
            if player.invulnerable {
                continue;
            }
            for(asteroid_pos, asteroid_rend, _) in (&positions, &rends, &asteroids).join(){
                let diff_x: f64 = (players_pos.x - asteroid_pos.x).abs();
                let diff_y: f64 = (players_pos.y - asteroid_pos.y).abs();
                let hype: f64 = ((diff_x*diff_x) + (diff_y*diff_y)).sqrt();

                if hype < (player_rend.o_w + asteroid_rend.o_w) as f64 / 2.0 {
                    println!("Collision Detected!");
                    if player.lives > 1 {
                        player.died = true;
                    } else {
                        let _ = entities.delete(entity);
                    }
                }
            }
        }
    }
}