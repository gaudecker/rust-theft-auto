use gl;
use gl::types::{GLsizei, GLuint, GLint, GLchar};
use std::str;
use gfx::vecmath::{Matrix4, Vector3};

use super::shader::{Shader};

pub struct ShaderProgram {
    pub id: uint
}

impl ShaderProgram {
    /// Links the shaders into a program.
    pub fn link(shaders: &[Result<Shader, String>]) -> Result<ShaderProgram, String> {
        let program = gl::CreateProgram();
        for shader in shaders.iter() {
            match shader {
                &Ok(ref shader) => {
                    gl::AttachShader(program, shader.id as GLuint);
                },
                &Err(ref msg) => return Err(msg.clone())
            }
        }
        gl::LinkProgram(program);

        match get_link_status(program) {
            Some(msg) => fail!(msg),
            None => Ok(ShaderProgram {
                id: program as uint
            })
        }
    }

    pub fn bind(&self) -> &ShaderProgram {
        gl::UseProgram(self.id as GLuint);
        self
    }

    /// Binds a user-defined varying out variable to a fragment shader
    /// color number.
    pub fn bind_fragment_output(&self, color_number: uint, name: &str) -> &ShaderProgram {
        name.with_c_str(|cstr| unsafe {
            gl::BindFragDataLocation(self.id as GLuint, color_number as GLuint, cstr)
        });
        self
    }

    pub fn set_uniform_matrix(&self, name: &str, mat: Matrix4<f32>) {
        unsafe {
            gl::UniformMatrix4fv(self.get_uniform_location(name) as GLint,
                                 1, gl::FALSE, mat[0].as_ptr());
        }
    }

    pub fn set_uniform_vector(&self, name: &str, vec: Vector3<f32>) {
        unsafe {
            gl::Uniform3fv(self.get_uniform_location(name) as GLint,
                           3, &vec[0]);
        }
    }

    /// Returns the location of a uniform variable.
    pub fn get_uniform_location(&self, name: &str) -> int {
        name.with_c_str(|n| unsafe {
            gl::GetUniformLocation(self.id as GLuint, n) as int
        })
    }

    /// Returns the location of an attribte variable.
    pub fn get_attribute_location(&self, name: &str) -> int {
        name.with_c_str(|n| unsafe {
            gl::GetAttribLocation(self.id as GLuint, n) as int
        })
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        gl::DeleteProgram(self.id as GLuint);
    }
}

/// Returns the status message if linking failed, otherwise None.
fn get_link_status(program_id: GLuint) -> Option<String> {
    let mut status = gl::FALSE as GLint;
    unsafe {
        gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut status);
    }

    if status == gl::TRUE as GLint {
        None
    } else {
        let mut len = 0;
        unsafe {
            gl::GetShaderiv(program_id, gl::INFO_LOG_LENGTH, &mut len as *mut GLint);
        }

        let mut buf = Vec::with_capacity(len as uint);
        unsafe {
            gl::GetProgramInfoLog(program_id, len,
                                 &mut len as *mut GLsizei,
                                 buf.as_mut_slice().as_mut_ptr() as *mut GLchar);
        }
        Some(str::from_utf8_owned(buf).unwrap())
    }
}
