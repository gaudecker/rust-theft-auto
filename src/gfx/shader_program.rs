use gl;
use gl::types::{GLsizei, GLuint, GLint, GLchar};
use std::str;

use super::shader::{Shader};

pub struct ShaderProgram {
    pub id: uint
}

impl ShaderProgram {
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

    pub fn bind(&mut self) {
        gl::UseProgram(self.id as GLuint);
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        gl::DeleteProgram(self.id as GLuint);
    }
}

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
