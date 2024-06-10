extern crate sdl2;

use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::{event::Event, rect::Rect};
use std::thread::sleep;
use std::time::Duration;

use sdl2::render::{Canvas, Texture, TextureCreator};

use sdl2::video::{Window , WindowContext};


#[derive(Clone , Copy)]
enum TextureColor {
    Green , 
    Blue
}

fn create_texture_rect<'a>(canvas : &mut Canvas<Window> , texture_creator : &'a TextureCreator<WindowContext> , 
color : TextureColor , size : u32) -> Option<Texture<'a>> { 
    if let Ok(mut square_texture) = texture_creator.create_texture_target(None, size, size) { 
        canvas.with_texture_canvas(&mut square_texture, |texture| {
            match color { 
                TextureColor::Blue => texture.set_draw_color(Color::RGB(0, 0, 255)), 
                TextureColor::Green => texture.set_draw_color(Color::RGB(0, 255, 0)),
            }

            texture.clear()
        }).expect("Failed to execute color");
        Some(square_texture)
    }else {
        None
    }
}


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
    const TEXTURE_SIZE: u32 = 32;

    let mut blue_square = create_texture_rect(&mut canvas, &texture_creator, TextureColor::Blue, TEXTURE_SIZE).expect("Failed to create a texture");
   
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
        canvas.set_draw_color(Color::RGB(0, 255, 0));
        canvas.clear();
        canvas
            .copy(
                &blue_square,
                None,
                Rect::new(0, 0, TEXTURE_SIZE, TEXTURE_SIZE),
            )
            .expect("Couldn't copy texture into window");
        canvas.present();

        sleep(Duration::new(0, 1_000_000_000u32 / 60))
    }
}
