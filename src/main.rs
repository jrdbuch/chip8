use sdl2::render::Canvas;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::video::Window;
use sdl2::surface::Surface; 
use sdl2::rect::Rect;
use std::path::Path;

// mod chip8;
// use chip8::{PIXEL_HEIGHT, PIXEL_WIDTH};
pub mod drivers;
pub mod chip8;

use drivers::DisplayDriver;
use chip8::Chip8;

const PIXEL_WIDTH: u32 = 32;
const PIXEL_HEIGHT: u32 = 64;
const DISPLAY_SCALE: u32 = 10;

fn main() {
    let sdl = sdl2::init().unwrap();

    let mut d = DisplayDriver::new(&sdl, DISPLAY_SCALE, PIXEL_WIDTH, PIXEL_HEIGHT);
    let mut chip8 = Chip8::new();

    let fp: &Path = Path::new("pong.chp8");
    chip8.load_ROM(fp);

    // d.draw_pixel(10, 10, Color::RGB(0,0, 0));
    // d.draw_pixel(11, 11, Color::RGB(0,0, 0));
    // d.update_display();

    // let surface = Surface::new(512, 512, PixelFormatEnum::RGB24).unwrap();
    // let texture = Surface::from_data(&'a mut [u8], width: u32, height: u32, pitch: u32, format: PixelFormatEnum)
    // let surface = Surface::from_data(&mut buffer, 100, 50, 300, PixelFormatEnum::RGB24).unwrap();

    // let texture = Surface::from_data(&mut buffer, 100, 50, 100*3, PixelFormatEnum::RGB24)
        // .as_texture(canvas.texture_creator());

    // let texture = texture_creator.create_texture_from_surface(surface).unwrap();

    // canvas.copy(&texture,None,None).unwrap();
    // canvas.present();

    //
    // Surface::from_data(data: &'a mut [u8], width: u32, height: u32, pitch: u32, format: pixels::PixelFormatEnum);`

    let mut event_pump = sdl.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => break 'main,
                _ => {},
            }
        }

        // render window contents here
    }
    println!("Hello, world!");
}
