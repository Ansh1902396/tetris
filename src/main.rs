extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::thread::sleep;
use std::time::Duration;

use sdl2::render::{Canvas, Texture, TextureCreator};

fn main() {
    let sdl_context = sdl2::init().expect("SDL initialization failed");
    

    let video_subsystem = sdl_context
        .video()
        .expect("Couldn't get SDL video_subsystem");

    let window = video_subsystem
        .window("rust-sdl2 demo : Video", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .expect("Failed to create window");

    let mut canvas = window
        .into_canvas()
        .target_texture()
        .present_vsync()
        .build()
        .expect("Couldn't get window's canvas");

    let texture_creator = canvas.texture_creator();
    const TEXTURE_SIZE:u32 = 32 ;

    canvas.set_draw_color(Color::RGB(255, 0, 0));

    canvas.clear();

    canvas.present();

    let mut event_pump = sdl_context
        .event_pump()
        .expect("Failed to get SDL event_pump");

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                _ => {}
            }
        }
        sleep(Duration::new(0, 1_000_000_000u32 / 60))
    }
}
