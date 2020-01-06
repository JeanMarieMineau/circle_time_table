//extern crate sdl2;
mod circle;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

use crate::circle::Circle;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let (xm, ym) = (800, 800);
    let white = Color::RGB(255, 255, 255);
    let window = video_subsystem.window("time table", xm, ym)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();

    let mut n = 10;
    let n_max = 500;
    let n_min = 10;
    let mut grow = true;
    let mut circle = Circle::new(n, xm, ym);

    canvas.set_draw_color(white);
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        canvas.set_draw_color(white);
        canvas.clear();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        circle.draw_table(&mut canvas, 60);
        if grow{
            n += 1;
        }else{
            n -= 1;
        }
        circle.set_n(n);
        if n == n_max {
            grow = false;
        }else if n == n_min {
            grow = true;
        }
        canvas.present();
        ::std::thread::sleep(Duration::from_millis(500));
    }
}
