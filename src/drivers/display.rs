use sdl2::Sdl;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

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