extern crate gl;
extern crate libc;
extern crate vecmath;

pub use self::shader::{VertexShader, FragmentShader, GeometryShader, Shader};
pub use self::shader_program::ShaderProgram;
pub use self::vertex_buffer::VertexBuffer;
pub use self::vertex_array::VertexArray;

pub mod shader;
pub mod shader_program;
pub mod vertex_buffer;
pub mod vertex_array;

pub enum Primitive {
    Points,
    Lines,
    LineStrip,
    LineLoop,
    Triangles,
    TriangleStrip,
    TriangleFan
}

impl Primitive {
    pub fn to_glenum(&self) ->  gl::types::GLenum {
        match *self {
            Points => gl::POINTS,
            Lines => gl::LINES,
            LineStrip => gl::LINE_STRIP,
            LineLoop => gl::LINE_LOOP,
            Triangles => gl::TRIANGLES,
            TriangleStrip => gl::TRIANGLE_STRIP,
            TriangleFan => gl::TRIANGLE_FAN
        }
    }
}
