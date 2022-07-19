//!# Shader
//!This module handles the shader creation
//!## Example
//!```
//!use opengl::shader::Shader;
//!let mut shader = Shader::new();
//!shader.create_shader_from_file(vertex_shader_loc, fragment_shader_loc);
//!```

use super::ptr;
use super::types::*;

use std::ffi::{CStr, CString};
use std::fs::File;
use std::io::Read;

use std::str;

/// Shader Struct
pub struct Shader {
    shader_id: GLuint,
    uniform_model: GLint,
    uniform_color: GLint,
}

impl Shader {
    ///Creates the Shader
    pub fn new() -> Self {
        Shader {
            shader_id: 0,
            uniform_model: 0,
            uniform_color: 0,
        }
    }

    ///Creates shader from given vertex and fragment shader
    pub fn create_shader_from_file(&mut self, vertex_shader_loc: &str, fragment_shader_loc: &str) {
        let vertex_shader = self.read_shader(vertex_shader_loc);
        let fragment_shader = self.read_shader(fragment_shader_loc);

        unsafe {
            self.compile_shader(&vertex_shader, &fragment_shader);
        }
    }

    #[doc(hidden)]
    fn read_shader(&self, shader_loc: &str) -> CString {
        let mut file = File::open(shader_loc).expect("[Error] Opening Shader.\nAborting...");
        let mut shader_src = String::new();
        file.read_to_string(&mut shader_src).unwrap();
        CString::new(shader_src.as_bytes()).unwrap()
    }

    #[doc(hidden)]
    unsafe fn compile_shader(&mut self, vertex_shader: &CStr, fragment_shader: &CStr) {
        self.shader_id = gl::CreateProgram();

        self.add_shader(vertex_shader, gl::VERTEX_SHADER);
        self.add_shader(fragment_shader, gl::FRAGMENT_SHADER);

        gl::LinkProgram(self.shader_id);
        let mut result: GLint = 0;
        let mut elog = Vec::with_capacity(512);
        elog.set_len(511);

        gl::GetProgramiv(self.shader_id, gl::LINK_STATUS, &mut result);

        if result == 0 {
            gl::GetProgramInfoLog(
                self.shader_id,
                512,
                ptr::null_mut(),
                elog.as_mut_ptr() as *mut GLchar,
            );

            println!(
                "[Error]Linking Program {}.\nAborting...",
                str::from_utf8(&elog)
                    .expect("[Error] Linking Program(failed parsing error).\nAborting...")
            );
        }

        gl::ValidateProgram(self.shader_id);
        gl::GetProgramiv(self.shader_id, gl::VALIDATE_STATUS, &mut result);

        if result == 0 {
            gl::GetProgramInfoLog(
                self.shader_id,
                512,
                ptr::null_mut(),
                elog.as_mut_ptr() as *mut GLchar,
            );

            println!(
                "[Error]Validating Program{}.\nAborting...",
                str::from_utf8(&elog)
                    .expect("[Error]Validating Program(failed parsing error).\nAborting...")
            );
        }

        let uniform_model = CString::new("model".as_bytes()).unwrap();
        self.uniform_model = gl::GetUniformLocation(self.shader_id, uniform_model.as_ptr());

        let uniform_color = CString::new("ucolor".as_bytes()).unwrap();
        self.uniform_color = gl::GetUniformLocation(self.shader_id, uniform_color.as_ptr());
    }

    #[doc(hidden)]
    unsafe fn add_shader(&mut self, shader_src: &CStr, shader_type: GLenum) {
        let the_shader = gl::CreateShader(shader_type);
        gl::ShaderSource(the_shader, 1, &shader_src.as_ptr(), ptr::null());

        gl::CompileShader(the_shader);

        let mut result: GLint = 0;
        let mut elog = Vec::with_capacity(512);
        elog.set_len(511);

        gl::GetShaderiv(the_shader, gl::COMPILE_STATUS, &mut result);

        if result == 0 {
            gl::GetShaderInfoLog(
                the_shader,
                512,
                ptr::null_mut(),
                elog.as_mut_ptr() as *mut GLchar,
            );

            println!(
                "[Error]Compiling Shader {}.\nAborting...",
                str::from_utf8(&elog)
                    .expect("[Error] Compiling Shader (failed parsing error).\nAborting...")
            );
        }

        gl::AttachShader(self.shader_id, the_shader);
        gl::DeleteShader(the_shader);
    }

    #[doc(hidden)]
    unsafe fn clear_shader(&mut self) {
        if self.shader_id != 0 {
            gl::DeleteProgram(self.shader_id);
            self.shader_id = 0;
        }
        self.uniform_model = 0;
    }

    ///Get uniform_model
    pub fn get_uniform_model(&self) -> GLint {
        self.uniform_model
    }

    ///Get uniform_color
    pub fn get_uniform_color(&self) -> GLint {
        self.uniform_color
    }

    ///Uses the shader program
    pub unsafe fn use_program(&self) {
        gl::UseProgram(self.shader_id);
    }
}

#[doc(hidden)]
impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            self.clear_shader();
        }
    }
}
