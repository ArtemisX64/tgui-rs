//!# Mesh
//!Creates a mesh and render them
//!## Usage
//!```
//!use opengl::mesh::Mesh;
//!
//!let mut mesh = Mesh::new();
//!//Create Mesh
//!mesh.create_mesh();
//!//Render Mesh
//!mesh.render_mesh();
//!```

use super::{ptr, types::*};
use crate::cfg;


use nalgebra_glm as glm;
use std::mem;
use std::os::raw::c_void;

/// Creates struct Mesh
pub struct Mesh {
    vao: GLuint,
    vbo: GLuint,
    ibo: GLuint,
    x: GLfloat,
    y: GLfloat,
}

impl Mesh {
    ///Creates a new Mesh
    pub fn new(x: u8, y: u8) -> Self {
        let x = (2.0 * x as GLfloat) - (cfg::WIDTH as GLfloat - 1.0);
        let y = (cfg::HEIGHT as GLfloat - 1.0) - (2.0 * y as GLfloat);

        Mesh {
            vao: 0,
            vbo: 0,
            ibo: 0,
            x,
            y,
        }
    }

    ///Creates the mesh
    pub unsafe fn create_mesh(&mut self) {
        #[rustfmt::skip]
        let indices: [GLuint; 6] = [
        	0, 1, 2,
        	2, 3, 1
        ];

        #[rustfmt::skip]
        let vertices: [GLfloat; 12] = [
            -1.0, 1.0, 0.0,
             1.0, 1.0, 0.0,
            -1.0,-1.0, 0.0,
             1.0,-1.0, 0.0,
        ];

        gl::GenVertexArrays(1, &mut self.vao);
        gl::BindVertexArray(self.vao);

        gl::GenBuffers(1, &mut self.ibo);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ibo);

        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (indices.len() * mem::size_of::<GLuint>()) as GLsizeiptr,
            &indices[0] as *const GLuint as *const c_void,
            gl::STATIC_DRAW,
        );

        gl::GenBuffers(1, &mut self.vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);

        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
            &vertices[0] as *const GLfloat as *const c_void,
            gl::STATIC_DRAW,
        );

        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            3 * mem::size_of::<GLfloat>() as GLsizei,
            ptr::null(),
        );

        gl::EnableVertexAttribArray(0);

        gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
        gl::BindVertexArray(self.vao);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ibo);
    }

    #[doc(hidden)]
    unsafe fn clear_mesh(&mut self) {
        if self.vao != 0 {
            gl::DeleteVertexArrays(1, &self.vao);
            self.vao = 0;
        }

        if self.vbo != 0 {
            gl::DeleteBuffers(1, &self.vbo);
            self.vbo = 0;
        }

        if self.ibo != 0 {
            gl::DeleteBuffers(1, &self.ibo);
            self.ibo = 0;
        }
    }

    ///Render the mesh
    pub unsafe fn render_mesh(
        &mut self,
        uniform_model: GLint,
        uniform_color: GLint,
        config: &crate::Config,
    ) {
        //Color of Shaders
        let color = nalgebra_glm::Vec3::new(config.fg.0, config.fg.1, config.fg.2);
        gl::Uniform3fv(uniform_color, 1, color.as_ptr());

        //Location of shaders
        let mut mat: glm::Mat4 = glm::one();

        mat = glm::scale(
            &mat,
            &glm::Vec3::new(
                1.0 / cfg::WIDTH as GLfloat,
                1.0 / cfg::HEIGHT as GLfloat,
                0.0,
            ),
        );
        mat = glm::translate(&mat, &glm::Vec3::new(self.x, self.y, 0.0));
        gl::UniformMatrix4fv(uniform_model, 1, gl::FALSE, mat.as_ptr());
        gl::BindVertexArray(self.vao);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ibo);

        gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }
}

impl Drop for Mesh {
    fn drop(&mut self) {
        unsafe {
            self.clear_mesh();
        }
    }
}
