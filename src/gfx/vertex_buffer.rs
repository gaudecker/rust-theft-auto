use gl;
use gl::types::{GLuint, GLsizeiptr};
use std::mem::size_of;
use gfx::libc::c_void;

pub struct VertexBuffer {
    id: uint
}

impl VertexBuffer {
    pub fn new() -> VertexBuffer {
        let mut id: GLuint = 0;
        unsafe { gl::GenBuffers(1, &mut id as *mut GLuint); }

        VertexBuffer {
            id: id as uint
        }
    }

    pub fn from_data<T>(data: &[T]) -> VertexBuffer {
        let vbo = VertexBuffer::new();
        vbo.load_data(data);
        vbo
    }

    pub fn load_data<T>(&self, data: &[T]) {
        self.bind();
        unsafe {
            gl::BufferData(gl::ARRAY_BUFFER,
                           (data.len() * size_of::<T>()) as GLsizeiptr,
                           data.as_ptr() as *const c_void, gl::STATIC_DRAW);
        }
    }

    pub fn bind(&self) {
        gl::BindBuffer(gl::ARRAY_BUFFER, self.id as GLuint);
    }
}

impl Drop for VertexBuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &(self.id as GLuint));
        }
    }
}
