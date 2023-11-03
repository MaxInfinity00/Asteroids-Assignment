use specs::prelude::*;
use specs::{World,WorldExt,Entities,Join};

use crate::sound_manager;

use crate::{components};

pub struct MissileMover;

impl<'a> System<'a> for MissileMover {
    type SystemData = (
        WriteStorage<'a, components::Position>,
        WriteStorage<'a, components::Renderable>,
        WriteStorage<'a, components::Missile>,
        Entities<'a>,
        Read<'a,crate::DeltaTime>
    );

    fn run(&mut self, mut data: Self::SystemData) {
        let (mut positions, mut renderables, missiles, entities, deltatime) = data;
        for(pos, rend, missile, entity) in (&mut positions, &mut renderables, &missiles, &entities).join(){
            let radians = pos.rot.to_radians();

            let move_x = missile.speed * radians.sin() * deltatime.0;
            let move_y = missile.speed * radians.cos() * deltatime.0;
            pos.x += move_x;
            pos.y -= move_y;

            if pos.x > crate::SCREEN_WIDTH.into() || pos.x < 0.0 || pos.y > crate::SCREEN_HEIGHT.into() || pos.y < 0.0 {
                entities.delete(entity).unwrap();

                let mut sound_manager = sound_manager::SoundManager::new();
                sound_manager.load_sound(&crate::RELOAD_FILENAME.to_string(), false);
                sound_manager.play_sound(crate::RELOAD_FILENAME.to_string());
                // let mut ecs = World::default();
                // ecs.create_entity()
                //     .with(components::SoundCue{
                //         filename: crate::SHOOT_FILENAME.to_string(),
                //         sc_type: components::SoundCueType::PlaySound
                //     })
                //     .build();
            }

            rend.rot = pos.rot;
        }
    }
}

pub struct MissileStriker;

impl<'a> System<'a> for MissileStriker {
    type SystemData = (
        WriteStorage<'a, components::Position>,
        WriteStorage<'a, components::Renderable>,
        WriteStorage<'a, components::Missile>,
        WriteStorage<'a, components::Asteroid>,
        WriteStorage<'a, components::GameData>,
        Entities<'a>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (positions, renderables, missiles, asteroids,_, entities) = &data;
        let mut asteroid_creation = Vec::<components::PendingAsteroid>::new();
        let mut score:u32 = 0;

        for( asteroid_pos,asteroid_rend,_, asteroid_entity) in (positions,renderables,asteroids,entities).join(){
            for(missile_pos,_,_,missile_entity) in (positions,renderables,missiles,entities).join(){
                let diff_x = (asteroid_pos.x - missile_pos.x).abs();
                let diff_y = (asteroid_pos.y - missile_pos.y).abs();
                let dist = (diff_x * diff_x + diff_y * diff_y);

                if dist < (asteroid_rend.o_w as f64  / 2.0)*(asteroid_rend.o_w as f64  / 2.0){
                    score += 10;
                    entities.delete(missile_entity).ok();
                    entities.delete(asteroid_entity).ok();

                    let mut sound_manager = sound_manager::SoundManager::new();
                    sound_manager.load_sound(&crate::RELOAD_FILENAME.to_string(), false);
                    sound_manager.play_sound(crate::RELOAD_FILENAME.to_string());

                    let new_size = asteroid_rend.o_w / 2;
                    if new_size >= 25 {
                        asteroid_creation.push(components::PendingAsteroid{
                            x: asteroid_pos.x,
                            y: asteroid_pos.y,
                            rot: asteroid_pos.rot - 90.0,
                            size: new_size
                        });
                        asteroid_creation.push(components::PendingAsteroid{
                            x: asteroid_pos.x,
                            y: asteroid_pos.y,
                            rot: asteroid_pos.rot + 90.0,
                            size: new_size
                        });
                    }
                }
            }
        }

        let (mut positions, mut renderables, _, mut asteroids,_,entities) = data;
        // let (ref positions, ref renderables, _, ref asteroids,ref entities) = data;
        for new_asteroid in asteroid_creation {
            let new_ast = entities.create();
            positions.insert(new_ast, components::Position{x:new_asteroid.x, y:new_asteroid.y, rot:new_asteroid.rot}).ok();
            asteroids.insert(new_ast, components::Asteroid{speed: 150.0, rot_speed: 150.0}).ok();
            renderables.insert(new_ast, components::Renderable{
                tex_name: "img/asteroid1.png".to_string(),
                i_w: 100,
                i_h: 100,
                o_w: new_asteroid.size,
                o_h: new_asteroid.size,
                frame: 0,
                total_frames: 1,
                rot: 0.0
            }).ok();
        }

        let(_,_,_,_,mut game_data,_) = data;
        for mut gamedata in (&mut game_data).join(){
            gamedata.score += score;
            let mut gamestate = crate::GAMESTATE.lock().unwrap();
            if gamedata.score > gamestate.highscore {
                gamestate.highscore = gamedata.score;
            }
        }
    }
}