use std::io::{File, BufferedReader, IoResult};
use std::str;
use gl;
use gl::types::{GLsizei, GLchar, GLuint, GLint, GLenum};

pub struct Shader {
    pub id: uint,
    pub shader_type: ShaderType
}

impl Shader {
    pub fn from_file(filename: &str, shader_type: ShaderType) -> Result<Shader, String> {
        println!("Loading shader {}", filename);

        let mut f = match File::open(&Path::new(filename)) {
            Err(why) => fail!("Could not open {}: {}", filename, why.desc),
            Ok(file) => file
        };
        let mut src = match f.read_to_string() {
            Err(why) => fail!("Could not read {}: {}", filename, why.desc),
            Ok(source) => source
        };
        
        compile_source(src.as_slice(), shader_type)
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        gl::DeleteShader(self.id as GLuint);
    }
}

fn compile_source(src: &str, shader_type: ShaderType) -> Result<Shader, String> {
    let shader: GLuint = gl::CreateShader(shader_type.to_glenum()) as GLuint;
    unsafe {
        gl::ShaderSource(shader, 1 as GLsizei, 
                         &(src.as_ptr() as *const GLchar) as *const *const GLchar,
                         &(src.len() as GLint) as *const GLint);
    }
    gl::CompileShader(shader);

    match get_compile_status(shader) {
        Some(err) => Err(err),
        None => Ok(Shader {
            id: shader as uint,
            shader_type: shader_type
        })
    }
}

fn get_compile_status(shader_id: GLuint) -> Option<String> {
    let mut status = gl::FALSE as GLint;
    unsafe {
        gl::GetShaderiv(shader_id, gl::COMPILE_STATUS, &mut status);
    }

    if status == gl::TRUE as GLint {
        None
    } else {
        let mut len = 0;
        unsafe {
            gl::GetShaderiv(shader_id, gl::INFO_LOG_LENGTH, &mut len as *mut GLint);
        }

        let mut buf = Vec::with_capacity(len as uint);
        unsafe {
            gl::GetShaderInfoLog(shader_id, len,
                                 &mut len as *mut GLsizei,
                                 buf.as_mut_slice().as_mut_ptr() as *mut GLchar);
        }
        Some(str::from_utf8_owned(buf).unwrap())
    }
}

pub enum ShaderType {
    VertexShader,
    FragmentShader,
    GeometryShader
}

impl ShaderType {
    pub fn to_glenum(&self) -> GLenum {
        match *self {
            VertexShader => gl::VERTEX_SHADER,
            FragmentShader => gl::FRAGMENT_SHADER,
            GeometryShader => gl::GEOMETRY_SHADER
        }
    }
}
