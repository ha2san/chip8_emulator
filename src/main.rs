use core::time::Duration;
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

const WIDTH: usize = 64;
const HEIGHT: usize = 32;
const M: usize = 20;
const RWIDTH: u32 = (M * WIDTH) as u32;
const RHEIGHT: u32 = (M * HEIGHT) as u32;
const BLACK: Color = Color::RGB(0, 0, 0);
const WHITE: Color = Color::RGB(0xff, 0xff, 0xff);
type Array = [[bool; HEIGHT]; WIDTH];

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let mut array_2d = [[false; HEIGHT]; WIDTH];
    array_2d = init_array(array_2d);

    let window = video_subsystem
        .window("chip8", RWIDTH, RHEIGHT)
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    'run: loop {
        (canvas, array_2d) = draw(canvas, array_2d);
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'run,
                _ => {}
            }
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

fn draw(mut canvas: Canvas<Window>, drawing_array: Array) -> (Canvas<Window>, Array) {
    canvas.set_draw_color(BLACK);
    canvas.clear();
    for i in 0..WIDTH {
        for j in 0..HEIGHT {
            if drawing_array[i][j] {
                canvas.set_draw_color(WHITE);
                let width = (i * M) as i32;
                let height = (j * M) as i32;
                canvas
                    .fill_rect(Rect::new(width, height, M as u32, M as u32))
                    .unwrap();
            }
        }
    }
    canvas.present();
    println!("Finished!");
    return (canvas, drawing_array);
}
