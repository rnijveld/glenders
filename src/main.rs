//! Renderall

#[macro_use]
extern crate glium;
extern crate image;
extern crate yaml_rust;

mod config;

use glium::DisplayBuild;
use std::error::Error;
pub use config::*;

fn main() {
    let config = Config::from_yaml_file_or_empty("./config.yml").unwrap_or_else(|e| {
        println!("Could not read config: {}", e.description());
        std::process::exit(5);
    });

    let (width, height) = config["graphics"]["dimensions"].unwrap_or((800, 600));
    let strs: (&str, &str, &str, &str) = config["strs"].unwrap();
    println!("{:?}", strs);
    // let fullscreen = config["graphics"]["fullscreen"].unwrap_or(false);
    // let dims: [f64; 4] = config["dims"].unwrap_or_else(|| [0.0; 4]);

    let builder = glium::glutin::WindowBuilder::new().with_dimensions(width, height);

    let window = builder.build_glium().unwrap();

    loop {
        for ev in window.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,   // the window has been closed by the user
                _ => (),
            }
        }
    }
}
