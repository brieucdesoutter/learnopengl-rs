//! Goal: move the triangle to the right side of the screen using an horizontal offset via a
//! uniform.

use std::ffi::{c_void, CString};

use gl::{self, types::*};

use learnopengl_rs::{OpenGLApp, shaders, vao};
use learnopengl_rs::glutin::run_in_window;
use learnopengl_rs::shaders::ShaderProgram;
use learnopengl_rs::vao::{VertexArrayObject, VertexAttribPointer};

struct Shaders {
    vao: VertexArrayObject,
    prgm: ShaderProgram,
}

impl Shaders {
    fn new() -> Self {
        Self {
            vao: VertexArrayObject::default(),
            prgm: ShaderProgram::default(),
        }
    }
}

#[repr(C)]
struct Vertex {
    position: [GLfloat; 3],
    color: [GLfloat; 3],
}

impl OpenGLApp for Shaders {
    fn title(&self) -> &str {
        "Shaders Exercise 2"
    }

    fn initialize(&mut self) {
        let vertices = [
            Vertex { position: [0.5, -0.5, 0.0], color: [1.0, 0.0, 0.0] },
            Vertex { position: [-0.5, -0.5, 0.0], color: [0.0, 1.0, 0.0] },
            Vertex { position: [0.0, 0.5, 0.0], color: [0.0, 0.0, 1.0] },
        ];

        self.vao = vao::create(&vertices, &[
            VertexAttribPointer {
                index: 0,
                size: 3,
                ty: gl::FLOAT,
                normalized: gl::FALSE as GLboolean,
                stride: std::mem::size_of::<Vertex>() as GLsizei,
                pointer: 0 as *const c_void,
            },
            VertexAttribPointer {
                index: 1,
                size: 3,
                ty: gl::FLOAT,
                normalized: gl::FALSE as GLboolean,
                stride: std::mem::size_of::<Vertex>() as GLsizei,
                pointer: std::mem::size_of::<[GLfloat; 3]>() as *const c_void,
            }
        ]);

        let vs = shaders::compile(include_str!("../res/shaders/shaders_ex2.vs"), gl::VERTEX_SHADER).unwrap();
        let fs = shaders::compile(include_str!("../res/shaders/shaders.fs"), gl::FRAGMENT_SHADER).unwrap();
        self.prgm = shaders::link(&vs, &fs).unwrap();
    }

    fn render(&self) {
        unsafe {
            gl::ClearColor(0.6, 0.6, 0.6, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::UseProgram(self.prgm.id);
            let offset_location = gl::GetUniformLocation(self.prgm.id, CString::new("aOffset").unwrap().as_ptr());
            gl::Uniform4f(offset_location, 0.5, 0.0, 0.0, 0.0);
            gl::BindVertexArray(self.vao.id);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
    }
}

fn main() {
    let app = Shaders::new();
    run_in_window(app);
}