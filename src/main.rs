//! Renderall

#[macro_use]
extern crate glium;
extern crate image;
extern crate yaml_rust;

mod config;

use glium::DisplayBuild;
use std::error::Error;
use config::*;

fn main() {
    let config = Config::from_file_or_empty("./config.yml").unwrap_or_else(|e| {
        println!("Could not read config: {}", e.description());
        std::process::exit(5);
    });

    let (width, height) = config.at("graphics").at("dimensions").get().unwrap_or((800, 600));


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
