extern crate sdl2;
extern crate rand;

pub mod bird;
pub mod scene;
pub mod pipes;

use std::path::Path;
use std::time::Duration;
use std::thread;

use sdl2::pixels::Color;
use sdl2::image::{INIT_PNG, INIT_JPG};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::render::Renderer;

use scene::Scene;
use bird::Bird;
use pipes::Pipes;

macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Flappy Rust", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let _image_context = sdl2::image::init(INIT_PNG | INIT_JPG).unwrap();    

    let mut renderer = window.renderer().build().unwrap();

    renderer.set_draw_color(Color::RGB(255, 255, 255));
    renderer.clear();
    renderer.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

    // draw_title: Flappy Rust
    draw_title(&mut renderer);

    // sleep 1 second
    thread::sleep(Duration::from_millis(3000));

    // Testing a bird
    let scene = Scene::new(&mut renderer);
    let mut pipes = Pipes::new(&mut renderer);
    let mut flappy = Bird::new(&mut renderer);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit{..}  => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                    flappy.jump();
                }
                _ => {}
            }
        }
        // The rest of the game loop goes here...
        thread::sleep(Duration::from_millis(10));
        renderer.clear();
        
        // Update and paint scene
        scene.paint(&mut renderer);

        // Update and paint pipes
        pipes.update();
        pipes.paint(&mut renderer);

        // Update paint bird.
        flappy.update();
        flappy.paint(&mut renderer);

        renderer.present();
    }
}

fn draw_title(renderer:&mut Renderer){
    renderer.clear();
    
    // Load a font
    let font_path = Path::new("res/fonts/Flappy.ttf");
    let ttf_context = sdl2::ttf::init().unwrap();
    let mut font = ttf_context.load_font(font_path, 50).unwrap();
    font.set_style(sdl2::ttf::STYLE_BOLD);

    // Render the surface
    let surface = font.render("Flappy Rust")
        .blended(Color::RGBA(255, 87, 0, 255)).unwrap();
    let mut texture = renderer.create_texture_from_surface(&surface).unwrap();

    renderer.set_draw_color(Color::RGBA(0, 217, 255, 255));
    renderer.clear();

    renderer.copy(&mut texture, None, Some(rect!(10,10,790,590))).unwrap();

    renderer.present();
}