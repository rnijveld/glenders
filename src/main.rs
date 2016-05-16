//! Renderall

#[macro_use]
extern crate glium;
extern crate image;
extern crate yaml_rust;
extern crate glenders;

use glium::{Surface, DisplayBuild};
use glium::glutin::{Event, VirtualKeyCode};
use std::error::Error;
use glenders::config::*;
use glenders::Vertex;

fn main() {
    let config = Config::from_file("./config.yml").unwrap_or_else(|e| {
        println!("Could not read config: {}", e.description());
        std::process::exit(5);
    });

    let (width, height) = config["graphics"]["dimensions"].unwrap_or((800, 600));
    // let fullscreen = config["graphics"]["fullscreen"].unwrap_or(false);
    // let dims: [f64; 4] = config["dims"].unwrap_or_else(|| [0.0; 4]);

    let builder = glium::glutin::WindowBuilder::new().with_dimensions(width, height);

    let window = builder.build_glium().unwrap();
    let v1 = Vertex::new(-0.5, -0.5, 1.0);
    let v2 = Vertex::new(0.0, 0.5, 1.0);
    let v3 = Vertex::new(0.5, -0.25, 1.0);
    let shape = vec![v1, v2, v3];

    let vertex_buffer = glium::VertexBuffer::new(&window, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    let program = glenders::get_program(&window);

    loop {
        for ev in window.poll_events() {
            match ev {
                Event::KeyboardInput(_, _, Some(VirtualKeyCode::Escape)) => return,
                Event::Closed => return,
                _ => (),
            }
        }

        let mut target = window.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms, &Default::default()).unwrap();
        target.finish().unwrap();
    }
}
