//! glenders

extern crate yaml_rust;
extern crate nalgebra;
#[macro_use]
extern crate glium;

pub mod config;
pub mod vec;
pub mod node;

use glium::backend::Facade;
use vec::Vec3;

#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub struct Vertex {
    position: [f32; 3],
}
implement_vertex!(Vertex, position);

impl Vertex {
    pub fn new(x: f32, y: f32, z: f32) -> Vertex {
        Vertex { position: [x, y, z] }
    }

    pub fn from_vec(v: Vec3) -> Vertex {
        Vertex { position: [v[0], v[1], v[2]] }
    }
}

pub fn get_program<F>(display: &F) -> glium::Program where F: Facade {
    glium::Program::from_source(
        display,
        include_str!("./shaders/vertex_shader.glsl"),
        include_str!("./shaders/fragment_shader.glsl"),
        None
    ).unwrap()
}
