use std::io::{File, IoResult};

use gfx;
use gfx::{Device, DeviceHelper};
use device;
use device::draw::CommandBuffer;

use super::{Renderer};

pub struct Program {
    pub handle: device::Handle<u32, device::shade::ProgramInfo>
}

impl Program {
    pub fn new<D: Device<C>, C: CommandBuffer>(r: &mut Renderer<D, C>, _filename: &str) -> Program {
        let vert = match load_shader_source("data/shader.vert") {
            Ok(src) => src,
            Err(err) => panic!(err.desc)
        };
        let vert_src = gfx::ShaderSource {
            glsl_120: None,
            glsl_130: None,
            glsl_140: None,
            glsl_150: Some(vert.as_bytes())
        };
        let frag = match load_shader_source("data/shader.frag") {
            Ok(src) => src,
            Err(err) => panic!(err.desc)
        };
        let frag_src = gfx::ShaderSource {
            glsl_120: None,
            glsl_130: None,
            glsl_140: None,
            glsl_150: Some(frag.as_bytes())
        };

        Program {
            handle: r.graphics.device.link_program(vert_src, frag_src).unwrap()
        }
    }
}

fn load_shader_source(filename: &str) -> IoResult<String> {
    let mut f = match File::open(&Path::new(filename)) {
        Err(why) => panic!("Could not open {}: {}", filename, why.desc),
        Ok(file) => file
    };
    f.read_to_string()
}
