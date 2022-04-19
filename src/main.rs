mod constant;
mod graphics;
use crate::constant::*;
use crate::graphics::Screen;
use core::time::Duration;

pub fn main() {
    let mut array_2d = [[false; HEIGHT]; WIDTH];
    array_2d = init_array(array_2d);
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("chip8", RWIDTH, RHEIGHT)
        .build()
        .unwrap();

    let canvas = window.into_canvas().build().unwrap();
    let event_pump = sdl_context.event_pump().unwrap();

    let mut screen = Screen { canvas, event_pump };

    'run: loop {
        screen.draw(array_2d);
        if screen.poll() {
            break 'run;
        }

        ::std::thread::sleep(Duration::from_millis(16));
    }
}

fn init_array(mut array: Array) -> Array {
    let mut value = true;
    for i in 0..WIDTH {
        value = !value;
        for j in 0..HEIGHT {
            value = !value;
            array[i][j] = value;
        }
    }
    array
}
