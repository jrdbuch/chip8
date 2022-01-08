use std::path::Path;
use std::fs;
use std::env;
use sdl2::pixels::Color;

pub mod drivers;
pub mod chip8;
pub mod utils;

use drivers::keyboard::KeyboardDriver;
use drivers::display::DisplayDriver;
use drivers::sound::SoundDriver;

use chip8::Chip8;

const PIXEL_WIDTH: u32 = 32;
const PIXEL_HEIGHT: u32 = 64;
const DISPLAY_SCALE: u32 = 10;

fn main() {
    let sdl = sdl2::init().unwrap();

    // init drivers
    let mut disp = DisplayDriver::new(&sdl, DISPLAY_SCALE, PIXEL_WIDTH, PIXEL_HEIGHT);
    let mut kb = KeyboardDriver::new(&sdl);
    let mut sound = SoundDriver::new(&sdl);

    // init chip8 VM
    let mut chip8 = Chip8::new();

    // load rom
    let args: Vec<String> = env::args().collect();
    let rom_fp: &Path = Path::new(&args[1]);
    let rom = fs::read(rom_fp).unwrap();
    chip8.load_rom(rom);

    'main: loop {

        kb.update();

        if kb.exit_requested {
            break 'main;
        }

        chip8.exec_cycle(&kb.key_state);

        if !chip8.draw_flag {continue;}

        // draw display memory to screen
        draw_chip8_memory_to_display(&chip8, &mut disp);

        // handle sound
        if chip8.sound_timer > 0 && !sound.on{
            sound.resume();
        } else if chip8.sound_timer == 0 && sound.on {
            sound.pause();
        }

        disp.update_display();
    }
}

fn draw_chip8_memory_to_display(chip8: &Chip8, disp: &mut DisplayDriver) {
        for (x, row) in chip8.display_memory.iter().enumerate() {
            for (y, col) in row.iter().enumerate() {

                let color: Color = match col {
                    1 => Color::WHITE,
                    _ => Color::BLACK,
                };

                disp.draw_pixel(y as i32, x as i32, color);
            }
        }
}