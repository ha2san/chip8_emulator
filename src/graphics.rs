use crate::constant::*;
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;

const BLACK: Color = Color::RGB(0, 0, 0);
const WHITE: Color = Color::RGB(0xff, 0xff, 0xff);

pub struct Screen {
    pub canvas: Canvas<Window>,
    pub event_pump: EventPump,
}

impl Screen {
    pub fn draw(&mut self, drawing_array: Array) {
        self.canvas.set_draw_color(BLACK);
        self.canvas.clear();
        for i in 0..WIDTH {
            for j in 0..HEIGHT {
                if drawing_array[i][j] {
                    self.canvas.set_draw_color(WHITE);
                    let width = (i * M) as i32;
                    let height = (j * M) as i32;
                    self.canvas
                        .fill_rect(Rect::new(width, height, M as u32, M as u32))
                        .unwrap();
                }
            }
        }
        self.canvas.present();
    }
    pub fn poll(&mut self) -> bool {
        let mut quit = false;
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    quit = true;
                }
                _ => quit = quit || false,
            }
        }
        return quit;
    }
}
