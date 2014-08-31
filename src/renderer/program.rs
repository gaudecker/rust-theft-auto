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
    pub fn new<D: Device<C>, C: CommandBuffer>(r: &mut Renderer<D, C>, name: &str) -> Program {
        let vert_src = match load_shader_source("../data/shader.vert") {
            Ok(src) => src,
            Err(err) => fail!(err.desc)
        };
        let frag_src = match load_shader_source("../data/shader.frag") {
            Ok(src) => src,
            Err(err) => fail!(err.desc)
        };

        Program {
            handle: r.graphics.device.link_program(vert_src, frag_src).unwrap()
        }
    }
}

fn load_shader_source(filename: &str) -> IoResult<gfx::ShaderSource> {
    let mut f = match File::open(&Path::new(filename)) {
        Err(why) => fail!("Could not open {}: {}", filename, why.desc),
        Ok(file) => file
    };
    let mut src = match f.read_to_string() {
        Err(why) => fail!("Could not read shader from {}: {}", filename, why.desc),
        Ok(src) => src
    };

    Ok(gfx::ShaderSource {
        glsl_120: None,
        glsl_150: Some(gfx::OwnedBytes(src.into_bytes()))
    })
}
