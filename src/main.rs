mod prelude;
use std::time::Duration;

use prelude::*;
use rendering::{polygon::Poly, renderer, Renderer};
use sdl2::{event::Event, pixels::Color};

const HEIGHT: usize = 800;
const WIDTH: usize = 800;

fn main() -> Result<(), String> {
    let screen = Vct::from_array([HEIGHT as f32, WIDTH as f32]);
    let center = screen / 2.0;
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rmat demo", 800, 800)
        .position_centered()
        .build()
        .unwrap();

    let rotate = rotation_mat(0.01, 0.01, 0.01);

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::BLACK);
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump()?;

    const SCALE: f32 = 200.0;
    let mut poly: Poly = [
        [0.0, SCALE, 1.0, 1.0],
        [SCALE, 0.0, 1.0, 1.0],
        [-SCALE, 0.0, 1.0, 1.0],
    ]
    .into();
    let mut renderer = Renderer::new(vec![poly], center);

    'running: loop {
        renderer.modify_polys(|p| {
            &rotate * p;
        });
        for event in event_pump.poll_iter() {
            if let Event::Quit { .. } = event {
                break 'running;
            }
        }
        canvas.set_draw_color(Color::WHITE);
        renderer.draw(&mut canvas)?;
        canvas.present();
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 144));
    }

    Ok(())
}

#[rustfmt::skip]
// https://en.wikipedia.org/wiki/Rotation_matrix
fn rotation_mat(yaw: f32, pitch: f32, roll:f32) -> Mat<4, 4> {
    let roll_t:Mat<4,4,> = [
        [f32::cos(roll), -f32::sin(roll), 0.0, 0.0],
        [f32::sin(roll),  f32::cos(roll), 0.0, 0.0],
        [                 0.0,                   0.0, 1.0, 0.0],
        [                 0.0,                   0.0, 0.0, 1.0]
    ].into();
    let pitch_t: Mat<4,4,> = [
        [ f32::cos(pitch), 0.0, f32::sin(pitch), 0.0],
        [                   0.0, 1.0,                   0.0, 0.0],
        [-f32::sin(pitch), 0.0, f32::cos(pitch), 0.0],
        [                  0.0,  0.0,                   0.0, 1.0]
    ].into();
    let yaw_t :Mat<4,4>= [
        [1.0,       0.0,        0.0, 0.0],
        [0.0, yaw.cos(), -yaw.sin(), 0.0],
        [0.0, yaw.sin(),  yaw.cos(), 0.0],
        [0.0,       0.0,        0.0, 1.0]
    ].into();

        roll_t
}
