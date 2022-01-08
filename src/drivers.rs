use sdl2::Sdl;
use sdl2::EventPump;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::collections::HashMap;


pub struct DisplayDriver {
    // sdl_context: Sdl,
    canvas: Canvas<Window>,
    display_scale: u32,
}

impl DisplayDriver {
    pub fn new(sdl_context: &Sdl, display_scale: u32, pix_width: u32, pix_height: u32) -> DisplayDriver {
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window("Game", pix_height*display_scale, pix_width*display_scale)
            .resizable()
            .build()
            .unwrap();
        
        let mut canvas : Canvas<Window> = window.into_canvas()
            .present_vsync() //< this means the screen cannot
            // render faster than your display rate (usually 60Hz or 144Hz
            .build().unwrap();
    
        canvas.set_draw_color(Color::WHITE);
        canvas.clear();
        canvas.set_draw_color(Color::BLACK);

        DisplayDriver {
            canvas,
            display_scale,
        }
    }

    pub fn draw_pixel(&mut self, x: i32, y: i32, color: Color) {
        self.canvas.set_draw_color(color);

        let scaled_x: i32 = x * self.display_scale as i32;
        let scaled_y: i32 = y * self.display_scale as i32;
    
        self.canvas.fill_rect(Rect::new(scaled_x, scaled_y, self.display_scale, self.display_scale));
    }

    pub fn update_display(&mut self) {
        // self.canvas.clear();
        self.canvas.present();
    }
}

pub type KeyState = HashMap<Keycode, bool>;

pub fn create_key_state() -> KeyState {
    HashMap::from([
        (Keycode::Num1, false),
        (Keycode::Num2, false),
        (Keycode::Num3, false),
        (Keycode::C, false),
        (Keycode::Num4, false),
        (Keycode::Num5, false),
        (Keycode::Num6, false),
        (Keycode::D, false),
        (Keycode::Num7, false),
        (Keycode::Num8, false),
        (Keycode::Num9, false),
        (Keycode::E, false),
        (Keycode::A, false),
        (Keycode::Num0, false),
        (Keycode::B, false),
        (Keycode::F, false),
    ])
}

type KeyMap = HashMap<Keycode, Keycode>;

pub fn create_key_map() -> KeyMap {
    HashMap::from([
        (Keycode::Num1, Keycode::Num1),
        (Keycode::Num2, Keycode::Num2),
        (Keycode::Num3, Keycode::Num3),
        (Keycode::Num4,  Keycode::C),   
        (Keycode::Q,    Keycode::Num4),
        (Keycode::W,    Keycode::Num5),
        (Keycode::E,    Keycode::Num6),
        (Keycode::R,    Keycode::D),   
        (Keycode::A,    Keycode::Num7),
        (Keycode::S,    Keycode::Num8),
        (Keycode::D,    Keycode::Num9),
        (Keycode::F,    Keycode::E),  
        (Keycode::Z,    Keycode::A),   
        (Keycode::X,    Keycode::Num0),
        (Keycode::C,    Keycode::B),   
        (Keycode::V,    Keycode::F),   
    ])
}


pub struct KeyboardDriver {
    event_pump: EventPump,
    key_map: KeyMap,
    pub key_state: KeyState,
    pub exit_requested: bool,
}


impl KeyboardDriver {
    pub fn new(sdl: &Sdl) -> KeyboardDriver {
        let event_pump = sdl.event_pump().unwrap();
        let key_map = create_key_map();
        let key_state = create_key_state();

        let exit_requested = false;

        KeyboardDriver{event_pump, key_map, key_state, exit_requested}
    }

    pub fn update(&mut self) {
        let p = &mut self.event_pump;
        for event in p.poll_iter() {
            match event {
                Event::Quit {..} => self.exit_requested = true,
                Event::KeyDown {keycode, ..} => {
                    match keycode {
                        Some(code) => {
                            let key_mapped = self.key_map.get(&code);
                            match key_mapped {
                                Some(k) => {self.key_state.insert(*k, true); println!("State Updated Down");},
                                None => (),
                            }
                        },
                        None => continue,
                    }
                } 
                Event::KeyUp {keycode, ..} => {
                    match keycode {
                        Some(code) => {
                            let key_mapped = self.key_map.get(&code);
                            match key_mapped {
                                Some(k) => {self.key_state.insert(*k, false); println!("State Update Up");},
                                None => (),
                            }
                        },
                        None => continue,
                    }
                } 
                _ => (),
            }
        }
    }

    pub fn int_to_key(val: u8) -> Option<Keycode> {
        match val {
            0 => Some(Keycode::Num0),
            1 => Some(Keycode::Num1),
            2 => Some(Keycode::Num2),
            3 => Some(Keycode::Num3),
            4 => Some(Keycode::Num4),
            5 => Some(Keycode::Num5),
            6 => Some(Keycode::Num6),
            7 => Some(Keycode::Num7),
            8 => Some(Keycode::Num8),
            9 => Some(Keycode::Num9),
            0xA => Some(Keycode::A),
            0xB => Some(Keycode::B),
            0xC => Some(Keycode::C),
            0xD => Some(Keycode::D),
            0xE => Some(Keycode::E),
            0xF => Some(Keycode::F),
            _ => None,
        }
    }
    
    pub fn key_to_int(kc: &Keycode) -> Option<u8> {
        match *kc {
            Keycode::Num0 => Some(0),
            Keycode::Num1 => Some(1),
            Keycode::Num2 => Some(2),
            Keycode::Num3 => Some(3),
            Keycode::Num4 => Some(4),
            Keycode::Num5 => Some(5),
            Keycode::Num6 => Some(6),
            Keycode::Num7 => Some(7),
            Keycode::Num8 => Some(8),
            Keycode::Num9 => Some(9),
            Keycode::A => Some(0xA),
            Keycode::B => Some(0xB),
            Keycode::C => Some(0xC),
            Keycode::D => Some(0xD),
            Keycode::E => Some(0xE),
            Keycode::F => Some(0xF),
            _ => None,
        }
    }
}

pub struct SoundDriver {

}
