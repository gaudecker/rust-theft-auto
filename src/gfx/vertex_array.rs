use gl;
use gfx::libc::c_void;
use gl::types::{GLint, GLuint, GLsizei, GLvoid, GLenum};

use super::ShaderProgram;
use super::Primitive;

pub struct VertexArray {
    id: uint
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &(self.id as GLuint));
        }
    }
}

impl VertexArray {
    /// Generates a new vertex array object.
    pub fn new() -> VertexArray {
        let mut id: GLuint = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut id as *mut GLuint);
        }

        VertexArray {
            id: id as uint
        }
    }

    pub fn bind(&self) -> &VertexArray {
        gl::BindVertexArray(self.id as GLuint);
        self
    }

    /// Defines vertex data for attribute named `name`.
    pub fn define_attribute_data(&self, program: &ShaderProgram, name: &str,
                                 size: GLint, stride: GLint, offset: uint) {
        let pos = program.get_attribute_location(name);
        gl::EnableVertexAttribArray(pos as GLuint);
        unsafe {       
            gl::VertexAttribPointer(pos as GLuint, size, gl::FLOAT,
                                    gl::FALSE, stride, offset as *const c_void);
        }
    }

    pub fn disable_attribute(&self, program: &ShaderProgram, name: &str) {
        gl::DisableVertexAttribArray(program.get_attribute_location(name) as GLuint);
    }

    pub fn draw_array(&self, primitive: Primitive, offset: GLint, count: GLsizei) {
        gl::DrawArrays(primitive.to_glenum(), offset, count);
    }

    pub fn draw_elements(&self, primitive: Primitive, offset: GLint, count: GLint) {
        unsafe {
            gl::DrawElements(primitive.to_glenum(), count, gl::UNSIGNED_INT, offset as *const GLvoid);
        }
    }
}
