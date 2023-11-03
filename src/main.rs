use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::{WindowCanvas, TextureCreator};
use sdl2::video::WindowContext;
use sdl2::pixels::Color;
use sdl2::rect::{Rect,Point};
use specs::{World, WorldExt, Join, DispatcherBuilder};
use std::time::Instant;

use std::sync::Mutex;
use std::time::Duration;
use std::path::Path;
use std::collections::HashMap;
use kira::track::effect::delay::DelayBuilder;

use once_cell::sync::Lazy;

pub mod texture_manager;
pub mod sound_manager;
pub mod utils;
pub mod components;
pub mod game;
pub mod asteroid;
pub mod missile;

// const IMG_WIDTH: u32 = 1000;
// const IMG_HEIGHT: u32 = 1000;
// const OUTPUT_WIDTH: u32 = 100;
// const OUTPUT_HEIGHT: u32 = 100;
const SCREEN_WIDTH: u32 = 1920;
const SCREEN_HEIGHT: u32 = 1080;

const MUSIC_FILENAME: &str = "sounds/music/space_ranger.wav";
const THRUSTER_FILENAME: &str = "sounds/fx/thrusters.mp3";
const SHOOT_FILENAME: &str = "sounds/fx/shoot.mp3";
const RELOAD_FILENAME: &str = "sounds/fx/reload.wav";

#[derive(Default)]
pub struct DeltaTime(pub f64);

fn render(canvas: &mut WindowCanvas, texture_manager: &mut texture_manager::TextureManager<WindowContext>, texture_creator: &TextureCreator<WindowContext>, font: &sdl2::ttf::Font, ecs: &World, fps: u64) -> Result<(),String> {
    let color = Color::RGB(255,255,255);
    canvas.set_draw_color(color);
    canvas.clear();

    // // Draw Greeting
    // let hello_text: String = "Hello World".to_string();
    // let surface = _font
    //     .render(&hello_text)
    //     .blended(Color::RGBA(255,0,0,120))
    //     .map_err(|e| e.to_string())?;
    //
    // let texture = _texture_creator
    //     .create_texture_from_surface(&surface)
    //     .map_err(|e| e.to_string())?;
    //
    // let target = Rect::new(10 as i32,0 as i32,350 as u32,100 as u32);
    // canvas.copy(&texture, None, Some(target))?;

    // //Draw Image
    // let src = Rect::new(0,0,IMG_WIDTH,IMG_HEIGHT);
    // let x = (SCREEN_WIDTH/2) as i32;
    // let y = (SCREEN_HEIGHT/2) as i32;
    //
    // let dest = Rect::new(x - ((OUTPUT_WIDTH/2) as i32),y- ((OUTPUT_HEIGHT/2) as i32),OUTPUT_WIDTH,OUTPUT_HEIGHT);
    // let center = Point::new((OUTPUT_WIDTH/2) as i32,(OUTPUT_HEIGHT/2) as i32);
    //
    // let texture = texture_manager.load("img/pepecry.jpg")?;
    // let mut angle: f64 = 0.0;
    // if utils::is_key_pressed(&key_manager,"W"){
    //     angle = 0.0;
    // } else if utils::is_key_pressed(&key_manager,"A"){
    //     angle = 270.0;
    // } else if utils::is_key_pressed(&key_manager,"S"){
    //     angle = 180.0;
    // } else if utils::is_key_pressed(&key_manager,"D"){
    //     angle = 90.0;
    // }
    // canvas.copy_ex(
    //     &texture, //Texture Object
    //     src, //Source Rectangle
    //     dest, //Destination Rectangle
    //     angle, //Rotation
    //     center, //Rotation Center
    //     false, //Flip Horizontal
    //     false //Flip Vertical
    // )?;

    let positions = ecs.read_storage::<components::Position>();
    {
        ////Draw Stars
        // canvas.set_draw_color(Color::RGBA(0,0,0,128));
        // let stars = ecs.read_storage::<components::Star>();
        // for (pos,star) in (&positions,&stars).join(){
        //     let x = pos.x as i32;
        //     let y = pos.y as i32;
        //     let dest = Rect::new(x,y,star.size,star.size);
        //     canvas.fill_rect(dest)?;
        // }
    }

    let renderables = ecs.read_storage::<components::Renderable>();

    for(renderable, pos) in (&renderables,&positions).join(){
        let src = Rect::new(0,0,renderable.i_w,renderable.i_h);
        let x = pos.x as i32;
        let y = pos.y as i32;
        let dest = Rect::new(x - ((renderable.o_w/2) as i32), y - ((renderable.o_h/2) as i32),renderable.o_w,renderable.o_h);

        let center = Point::new((renderable.o_w/2) as i32,(renderable.o_h/2) as i32);
        let texture = texture_manager.load(&renderable.tex_name)?;
        canvas.copy_ex(
            &texture, //Texture Object
            src, //Source Rectangle
            dest, //Destination Rectangle
            renderable.rot, //Rotation
            center, //Rotation Center
            false, //Flip Horizontal
            false //Flip Vertical
        )?;
    }

    let players = ecs.read_storage::<components::Player>();
    for(renderable, pos, player) in (&renderables, &positions, &players).join(){

        //Show Lives
        let lives: String = "Lives: ".to_string() + &player.lives.to_string();
        let surface = font
            .render(&lives)
            .blended(Color::RGBA(0,0,0,255))
            .map_err(|e| e.to_string())?;
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        let target = Rect::new((SCREEN_WIDTH - 135) as i32,10 as i32,125 as u32,50 as u32);
        canvas.copy(&texture, None, Some(target));

        let src = Rect::new(0,0,renderable.i_w, renderable.i_h);
        let x: i32 = pos.x as i32;
        let y: i32 = pos.y as i32;
        let mut dest = Rect::new(x - ((renderable.o_w/2) as i32), y - ((renderable.o_h/2) as i32),renderable.o_w,renderable.o_h);

        let mut draw_second = false;
        if dest.x < (renderable.o_w/2).try_into().unwrap() {
            dest.x += crate::SCREEN_WIDTH as i32;
            draw_second = true;
        } else if dest.x > (crate::SCREEN_WIDTH - renderable.o_w/2) as i32 {
            dest.x -= crate::SCREEN_WIDTH as i32;
            draw_second = true;
        }

        if dest.y < (renderable.o_h/2).try_into().unwrap() {
            dest.y += crate::SCREEN_HEIGHT as i32;
            draw_second = true;
        } else if dest.y > (crate::SCREEN_HEIGHT - renderable.o_h/2) as i32 {
            dest.y -= crate::SCREEN_HEIGHT as i32;
            draw_second = true;
        }

        if !draw_second {
            break;
        }

        let center = Point::new((renderable.o_w/2) as i32,(renderable.o_h/2) as i32);
        let texture = texture_manager.load(&renderable.tex_name)?;
        canvas.copy_ex(
            &texture, //Texture Object
            src, //Source Rectangle
            dest, //Destination Rectangle
            renderable.rot, //Rotation
            center, //Rotation Center
            false, //Flip Horizontal
            false //Flip Vertical
        )?;

    }


    let gamedatas = ecs.read_storage::<components::GameData>();
    for gamedata in (gamedatas).join(){

        if &GAMESTATE.lock().unwrap().highscore > &gamedata.score {
            //Show Score
            let score: String = "Score: ".to_string() + &gamedata.score.to_string();
            let surface = font
                .render(&score)
                .blended(Color::RGBA(0,0,0,255))
                .map_err(|e| e.to_string())?;
            let texture = texture_creator
                .create_texture_from_surface(&surface)
                .map_err(|e| e.to_string())?;


            let target = Rect::new(10 as i32,0 as i32,125 as u32,50 as u32);
            canvas.copy(&texture, None, Some(target));

            //Show Highscore
            let highscore: String = "High Score: ".to_string() + &GAMESTATE.lock().unwrap().highscore.to_string();
            let surface = font
                .render(&highscore)
                .blended(Color::RGBA(0,0,0,255))
                .map_err(|e| e.to_string())?;
            let texture = texture_creator
                .create_texture_from_surface(&surface)
                .map_err(|e| e.to_string())?;

            let target = Rect::new(10 as i32,50 as i32,150 as u32,35 as u32);
            canvas.copy(&texture, None, Some(target));

        }
        else{
            //Show Score (High)
            let highscore: String = "Score: ".to_string() + &GAMESTATE.lock().unwrap().highscore.to_string() + " High!";
            let surface = font
                .render(&highscore)
                .blended(Color::RGBA(0,0,0,255))
                .map_err(|e| e.to_string())?;
            let texture = texture_creator
                .create_texture_from_surface(&surface)
                .map_err(|e| e.to_string())?;


            let target = Rect::new(10 as i32,0 as i32,200 as u32,50 as u32);
            canvas.copy(&texture, None, Some(target));
        }

        //Show Level
        let level: String = "Level: ".to_string() + &gamedata.level.to_string();
        let surface = font
            .render(&level)
            .blended(Color::RGBA(0,0,0,255))
            .map_err(|e| e.to_string())?;
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        let target = Rect::new(10 as i32,(SCREEN_HEIGHT - 60 ) as i32,150 as u32,50 as u32);
        canvas.copy(&texture, None, Some(target));

        if gamedata.showControls {
            //Show Controls
            let moveControls: String = "WASD Move".to_string();
            let surface = font
                .render(&moveControls)
                .blended(Color::RGBA(0,0,0,255))
                .map_err(|e| e.to_string())?;
            let texture = texture_creator
                .create_texture_from_surface(&surface)
                .map_err(|e| e.to_string())?;

            let target = Rect::new((SCREEN_WIDTH - 185) as i32,(SCREEN_HEIGHT - 245 ) as i32,175 as u32,35 as u32);
            canvas.copy(&texture, None, Some(target));

            let spaceControls: String = "Space Shoot".to_string();
            let surface = font
                .render(&spaceControls)
                .blended(Color::RGBA(0,0,0,255))
                .map_err(|e| e.to_string())?;
            let texture = texture_creator
                .create_texture_from_surface(&surface)
                .map_err(|e| e.to_string())?;

            let target = Rect::new((SCREEN_WIDTH - 210) as i32,(SCREEN_HEIGHT - 205 ) as i32,200 as u32,35 as u32);
            canvas.copy(&texture, None, Some(target));

            let MusicControls: String = "P Un/Pause Music".to_string();
            let surface = font
                .render(&MusicControls)
                .blended(Color::RGBA(0,0,0,255))
                .map_err(|e| e.to_string())?;
            let texture = texture_creator
                .create_texture_from_surface(&surface)
                .map_err(|e| e.to_string())?;


            let target = Rect::new((SCREEN_WIDTH - 260) as i32,(SCREEN_HEIGHT - 165 ) as i32,250 as u32,35 as u32);
            canvas.copy(&texture, None, Some(target));

            let InvincibleControls: String = "I Invincible".to_string();
            let surface = font
                .render(&InvincibleControls)
                .blended(Color::RGBA(0,0,0,255))
                .map_err(|e| e.to_string())?;
            let texture = texture_creator
                .create_texture_from_surface(&surface)
                .map_err(|e| e.to_string())?;

            let target = Rect::new((SCREEN_WIDTH - 210) as i32,(SCREEN_HEIGHT - 125 ) as i32,200 as u32,35 as u32);
            canvas.copy(&texture, None, Some(target));


            let AsteroidControls: String = "O 1000 Asteroids".to_string();
            let surface = font
                .render(&AsteroidControls)
                .blended(Color::RGBA(0,0,0,255))
                .map_err(|e| e.to_string())?;
            let texture = texture_creator
                .create_texture_from_surface(&surface)
                .map_err(|e| e.to_string())?;

            let target = Rect::new((SCREEN_WIDTH - 260) as i32,(SCREEN_HEIGHT - 85 ) as i32,250 as u32,35 as u32);
            canvas.copy(&texture, None, Some(target));

            let FPSControls: String = "U Unlock FPS".to_string();
            let surface = font
                .render(&FPSControls)
                .blended(Color::RGBA(0,0,0,255))
                .map_err(|e| e.to_string())?;
            let texture = texture_creator
                .create_texture_from_surface(&surface)
                .map_err(|e| e.to_string())?;

            let target = Rect::new((SCREEN_WIDTH - 210) as i32,(SCREEN_HEIGHT - 45 ) as i32,200 as u32,35 as u32);
            canvas.copy(&texture, None, Some(target));



            //Show Asteroids
            let asteroidCounter: String = "Asteroids: ".to_string() + &game::get_asteroid_count(&ecs).to_string() + " ";

            let surface = font
                .render(&asteroidCounter)
                .blended(Color::RGBA(0,0,0,255))
                .map_err(|e| e.to_string())?;
            let texture = texture_creator
                .create_texture_from_surface(&surface)
                .map_err(|e| e.to_string())?;

            let target = Rect::new((SCREEN_WIDTH - 480) as i32,(SCREEN_HEIGHT - 85 ) as i32,225 as u32,35 as u32);
            canvas.copy(&texture, None, Some(target));

            //Show FPS
            let fpsCounter: String = "FPS: ".to_string() + &fps.to_string();
            let surface = font
                .render(&fpsCounter)
                .blended(Color::RGBA(0,0,0,255))
                .map_err(|e| e.to_string())?;
            let texture = texture_creator
                .create_texture_from_surface(&surface)
                .map_err(|e| e.to_string())?;

            let target = Rect::new((SCREEN_WIDTH - 480) as i32,(SCREEN_HEIGHT - 45 ) as i32,125 as u32,35 as u32);
            canvas.copy(&texture, None, Some(target));
        }
        else{
            //Show Info Control
            let info: String = "H Show Info".to_string();
            let surface = font
                .render(&info)
                .blended(Color::RGBA(0,0,0,255))
                .map_err(|e| e.to_string())?;
            let texture = texture_creator
                .create_texture_from_surface(&surface)
                .map_err(|e| e.to_string())?;

            let target = Rect::new((SCREEN_WIDTH - 185) as i32,(SCREEN_HEIGHT - 45 ) as i32,175 as u32,35 as u32);
            canvas.copy(&texture, None, Some(target));
        }
    }

    canvas.present();
    Ok(())
}

struct State{ecs: World}

pub struct GameState{
    highscore: u32
}

static GAMESTATE: Lazy<Mutex<GameState>> = Lazy::new(|| {
    Mutex::new(GameState{
        highscore: 0
    })
});

fn main() -> Result<(),String>{
    println!("Starting Asteroids!");
    
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    
    let window = video_subsystem.window("Asteroids",SCREEN_WIDTH,SCREEN_HEIGHT)
        .fullscreen()
        .position_centered()
        .build()
        .expect("Could not initialize video subsystem");
    
    let mut canvas = window.into_canvas().build()
        .expect("Failed ot initialize canvas");
        
    let texture_creator = canvas.texture_creator();
    let mut texture_manager = texture_manager::TextureManager::new(&texture_creator);

    //Load Images
    texture_manager.load("img/ship.png")?; //Loads Ship Texture to Memory
    texture_manager.load("img/asteroid1.png")?; //Loads Asteroid Texture to Memory
    texture_manager.load("img/missile.png")?; //Loads Missile Texture to Memory


    //Sound Manager
    let mut sound_manager = sound_manager::SoundManager::new();

    //Load the soudns to prevent loading during gameplay
    sound_manager.load_sound(&MUSIC_FILENAME.to_string(), true);
    sound_manager.load_sound(&THRUSTER_FILENAME.to_string(), true);
    sound_manager.load_sound(&SHOOT_FILENAME.to_string(), false);
    sound_manager.load_sound(&RELOAD_FILENAME.to_string(), false);

    //Prepare fonts
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let font_path: &Path = Path::new(&"fonts/Monocraft.ttf");
    let mut font = ttf_context.load_font(font_path, 128)?;
    font.set_style(sdl2::ttf::FontStyle::BOLD);
    
    let mut event_pump = sdl_context.event_pump()?;
    let mut key_manager: HashMap<String,bool> = HashMap::new();

    let mut gs = State{
        ecs: World::new()
    };
    gs.ecs.register::<components::Position>();
    gs.ecs.register::<components::Renderable>();
    gs.ecs.register::<components::Player>();
    gs.ecs.register::<components::Asteroid>();
    gs.ecs.register::<components::Missile>();
    gs.ecs.register::<components::GameData>();
    // gs.ecs.register::<components::Star>();
    gs.ecs.register::<components::SoundCue>();
    gs.ecs.insert(DeltaTime(0.0));

    let mut dispatcher = DispatcherBuilder::new() //Creates a dispatcher to run systems
        .with(asteroid::AsteroidMover, "asteroid_mover", &[])
        .with(asteroid::AsteroidCollider, "asteroid_collider", &[])
        .with(missile::MissileMover, "missile_mover", &[])
        .with(missile::MissileStriker, "missile_striker", &[])
        .build();

    game::load_world(&mut gs.ecs);

    //Start Music Playing
    sound_manager.resume_sound(&MUSIC_FILENAME.to_string());

    let mut frame_count = 0u64;
    let mut last_frame_time = Instant::now();
    let mut last_frame_time_fps = Instant::now();
    let mut fps = 0u64;

    //init at 100 to draw initial UI
    // let mut loop_count = 100;

    let mut unlockedFPS = false;
    let mut musicPlaying = true;

    'running: loop {
        for event in event_pump.poll_iter(){
            match event {
                Event::Quit {..} => {
                    break 'running
                }, 
                Event::KeyDown {keycode: Some(Keycode::Escape),..} => {
                    break 'running
                },
                Event::KeyDown {keycode: Some(Keycode::Space),..} => {
                    utils::key_down(&mut key_manager, " ".to_string());
                },
                Event::KeyUp {keycode: Some(Keycode::Space),..} => {
                    utils::key_up(&mut key_manager, " ".to_string());
                },
                Event::KeyUp {keycode: Some(Keycode::P),..} => {
                    musicPlaying = !musicPlaying;
                    if musicPlaying {
                        sound_manager.resume_sound(&MUSIC_FILENAME.to_string())
                    } else {
                        sound_manager.stop_sound(&MUSIC_FILENAME.to_string());
                    }
                },
                Event::KeyUp {keycode: Some(Keycode::U),..} => {
                    println!("FPS Toggle");
                    unlockedFPS = !unlockedFPS;
                    if unlockedFPS {
                        sound_manager.stop_sound(&THRUSTER_FILENAME.to_string());
                    }
                },
                Event::KeyUp {keycode:Some(Keycode::I),..} => {
                    game::toggle_invincibility(&mut gs.ecs);
                },
                Event::KeyUp {keycode:Some(Keycode::O),..} => {
                    game::create_thousand_asteroids(&mut gs.ecs);
                },
                Event::KeyUp {keycode:Some(Keycode::H),..} => {
                    game::toggle_show_controls(&mut gs.ecs);
                },
                Event::KeyDown {keycode,..} => {
                    match keycode {
                        None => {},
                        Some(key) => {
                            utils::key_down(&mut key_manager, key.to_string());
                        }
                    }
                },
                Event::KeyUp {keycode,..} => {
                    match keycode {
                        None => {},
                        Some(key) => {
                            utils::key_up(&mut key_manager, key.to_string());
                        }
                    }
                },
                _ => {}
            }
        }

        let now = Instant::now();
        let delta_time = now.duration_since(last_frame_time).as_secs_f64();
        last_frame_time = now;

        if delta_time != 0.0 {
            fps = (1.0/delta_time) as u64;
        }
        // frame_count += 1;
        //
        // let elapsed_time_fps = last_frame_time_fps.elapsed();
        // if elapsed_time_fps.as_secs() >= 1 {
        //     fps = frame_count;
        //     frame_count = 0;
        //     last_frame_time_fps += Duration::new(1, 0);
        // }

        gs.ecs.write_resource::<DeltaTime>().0 = delta_time;

        game::update(&mut gs.ecs, &mut key_manager,delta_time);
        dispatcher.dispatch(&mut gs.ecs); //Runs the dispatcher and all systems run events
        gs.ecs.maintain(); //Removes all entities that have been deleted

        let _ = render(&mut canvas,&mut texture_manager, &texture_creator,&font, &gs.ecs, fps);
        let cues = gs.ecs.read_storage::<components::SoundCue>();
        let entities = gs.ecs.entities();
        for (cue, entitiy) in (&cues, &entities).join() {
            if(!unlockedFPS) {
                if cue.sc_type == components::SoundCueType::PlaySound {
                    sound_manager.play_sound(cue.filename.to_string());
                } else if cue.sc_type == components::SoundCueType::LoopSound {
                    sound_manager.resume_sound(&cue.filename.to_string());
                } else if cue.sc_type == components::SoundCueType::StopSound {
                    sound_manager.stop_sound(&cue.filename.to_string());
                }
            }
            entities.delete(entitiy).ok();
        }
        if(!unlockedFPS){
            std::thread::sleep(Duration::new(0,1_000_000_000u32/60));
        }
    }

    Ok(())
}


