extern crate sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::video::{Window, WindowContext};

use std::time::SystemTime;
use std::thread::sleep;
use std::time::Duration;

const TEXTURE_SIZE: u32 = 32;


#[derive(Clone, Copy)]
enum TextureColor {
    Green,
    Blue,
}

fn create_texture_rect<'a>(canvas: &mut Canvas<Window>,
  texture_creator: &'a TextureCreator<WindowContext>,
  color: TextureColor,
  size: u32) -> Option<Texture<'a>> {

  if let Ok(mut square_texture) =
    texture_creator.create_texture_target(None, size, size) {
        canvas.with_texture_canvas(&mut square_texture, |texture| {
            match color {
                //for now, TextureColor only handles two colors
                TextureColor::Green => texture.set_draw_color(Color::RGB(0, 255, 0)),
                TextureColor::Blue => texture.set_draw_color(Color::RGB(0, 0, 255)),
            }
            texture.clear();
        }).expect("failed to color a texture");
        Some(square_texture)
    } else {
        None
    }
  }

fn main() {
    
    let sdl_context = sdl2::init().expect("impossivel inicializar o sdl");

    let video_subsystem = sdl_context.video().expect("nao foi possivel importar o subsistema de video");

    let window = video_subsystem.window("tetris", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .expect("falha ao criar a janela");

    let mut canvas = window.into_canvas()
        .target_texture()
        .present_vsync()
        .build()
        .expect("could not get window canvas");

    let texture_creator: TextureCreator<_> = canvas.texture_creator();

    //we create a texture with a 32x32 size
    let green_square = create_texture_rect(&mut canvas,
        &texture_creator,
        TextureColor::Green,
        TEXTURE_SIZE).expect("failed to create a texture");

    let blue_square = create_texture_rect(&mut canvas,
        &texture_creator,
        TextureColor::Blue,
        TEXTURE_SIZE).expect("failed to create texture");

    let timer = SystemTime::now();


    /*let mut square_texture: Texture = 
        texture_creator.create_texture_target(None, TEXTURE_SIZE,
         TEXTURE_SIZE)
        .expect("failed to create a texture");

    canvas.with_texture_canvas(&mut square_texture, |texture| {
        texture.set_draw_color(Color::RGB(0, 200, 0));
        texture.clear();
    });*/


    let mut event_pump = sdl_context.event_pump().expect("failed to  get sdl event pump");

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running //we break the infinite loop
                },
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(255, 0, 0));
        canvas.clear();

        //the rectangle switch happens here
        let display_green = match timer.elapsed() {
            Ok(elapsed) => elapsed.as_secs() % 2 == 0,
            Err(_) => {
                //in case of erro we do nothing
                true
            }
        };

        let _square_texture = if display_green {
            &green_square
        } else {
            &blue_square
        };

        canvas.copy(_square_texture,
            None,
            
            Rect::new(0, 0, TEXTURE_SIZE, TEXTURE_SIZE))
            .expect("could not copy texture into window");
            canvas.present();


        sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}