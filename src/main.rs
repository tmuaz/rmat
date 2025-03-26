mod prelude;
use std::time::Duration;

use prelude::*;
use sdl2::{event::Event, pixels::Color};

const HEIGHT: usize = 800;
const WIDTH: usize = 800;

fn main() {
    let screen = Vct::from_array([HEIGHT as f32, WIDTH as f32]);
    let center = screen / 2.0;
    // a vector with an angle of 45 degrees relative to x axis
    let mut vct = Vct::from_array([1.0, 1.0]);
    let rotate_mat = rotation_mat(0.01);
    // scale it to the size of the window
    vct *= Vct::from_array([HEIGHT as f32 / 4.0, WIDTH as f32 / 4.0]);
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rmat demo", 800, 800)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::BLACK);
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            if let Event::Quit { .. } = event {
                break 'running;
            }
        }
        // rotate the point
        vct = &rotate_mat * vct;
        // this is the point we want to draw a line until
        let point = &vct + &center;
        canvas.set_draw_color(Color::WHITE);
        canvas.draw_line(&center, &point).unwrap();
        canvas.present();
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

#[rustfmt::skip]
// https://en.wikipedia.org/wiki/Rotation_matrix
fn rotation_mat(theta: f32) -> Mat<2, 2> {
    Mat::from_arrays([
        [f32::cos(theta), -f32::sin(theta)],
        [f32::sin(theta),  f32::cos(theta)],
    ])
}
