#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;

extern crate cgmath;

use gfx::attrib::Floater;
use gfx::traits::*;

fn main() {
    let (mut stream, mut device, mut factory) = gfx_window_glutin::init(
        glutin::WindowBuilder::new()
            .with_title("Demo".to_string())
            .with_dimensions(800, 600)
            .with_gl(glutin::GL_CORE)
            .build().unwrap()
    );
    
    'main: loop {
        for event in stream.out.window.poll_events() {
            use glutin::{Event, VirtualKeyCode};

            match event {
                Event::KeyboardInput(_, _, Some(VirtualKeyCode::Escape)) => break 'main,
                _ => {}
            }
        }

        stream.clear(gfx::ClearData {
            color: [0.3, 0.1, 0.2, 1.0],
            depth: 1.0,
            stencil: 0
        });

        stream.present(&mut device);
    }
}
