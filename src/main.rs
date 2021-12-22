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

use drivers::{KeyboardDriver, DisplayDriver};
use chip8::Chip8;

const PIXEL_WIDTH: u32 = 32;
const PIXEL_HEIGHT: u32 = 64;
const DISPLAY_SCALE: u32 = 10;

fn main() {
    let sdl = sdl2::init().unwrap();

    // init drivers
    let mut disp = DisplayDriver::new(&sdl, DISPLAY_SCALE, PIXEL_WIDTH, PIXEL_HEIGHT);
    let mut kb = KeyboardDriver::new(&sdl);

    // init chip8 VM
    let mut chip8 = Chip8::new();

    // let fp: &Path = Path::new("pong.chp8");
    let fp: &Path = Path::new("ibm_logo.chp8");
    chip8.load_ROM(fp);

    'main: loop {

        kb.update();

        if kb.exit_requested {
            break 'main;
        }

        chip8.exec_cycle();

        // draw display memory to screen
        for (x, row) in chip8.display_memory.iter().enumerate() {
            for (y, col) in row.iter().enumerate() {

                let color: Color = match col {
                    1 => Color::BLACK,
                    _ => Color::RED,
                };

                disp.draw_pixel(y as i32, x as i32, color);
            }
        }

        disp.update_display();
    }

    println!("Hello, world!");
}
