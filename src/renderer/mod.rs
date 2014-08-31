#[phase(plugin)]
extern crate gfx_macros;

use gfx;
use gfx::{Device, DeviceHelper, BufferHandle, VertexFormat};
use gfx::shade::ShaderParam;
use device;
use device::draw::CommandBuffer;

pub mod buffer;
pub mod program;

#[vertex_format]
pub struct Vertex {
    #[as_float]
    pub pos: [f32, ..3],
    #[as_float]
    pub uv: [f32, ..2],
    #[as_float]
    pub color: [f32, ..3]
}

#[shader_param(Program)]
pub struct Params {
    #[name = "projection"]
    pub projection: [[f32, ..4], ..4],
    #[name = "view"]
    pub view: [[f32, ..4], ..4]
}

impl Vertex {
    pub fn new(pos: [f32, ..3], uv: [f32, ..2], color: [f32, ..3]) -> Vertex {
        Vertex {
            pos: pos,
            uv: uv,
            color: color
        }
    }
}

impl Clone for Vertex {
    fn clone(&self) -> Vertex {
        *self
    }
}

pub struct Renderer<D: Device<C>, C: CommandBuffer> {
    pub graphics: gfx::Graphics<D, C>,
    frame: gfx::Frame,
    clear_data: gfx::ClearData,
    drawstate: gfx::DrawState
}

impl<D: Device<C>, C: CommandBuffer> Renderer<D, C> {
    pub fn new(mut device: D, frame: gfx::Frame) -> Renderer<D, C> {
        let mut drawstate = gfx::DrawState::new().depth(gfx::state::LessEqual, true);
        drawstate.primitive.front_face = gfx::state::Clockwise;

        Renderer {
            graphics: gfx::Graphics::new(device),
            frame: frame,
            clear_data: gfx::ClearData {
                color: Some([0.05, 0.0, 0.06, 1.0]),
                depth: Some(1.0),
                stencil: None,
            },
            drawstate: drawstate,
        }
    }

    pub fn clear(&mut self) {
        self.graphics.clear(self.clear_data, &self.frame);
    }

    pub fn render<V: VertexFormat, P: ShaderParam<L>, L>(&mut self,
                                                         buffer: buffer::Buffer<V, P, L>,
                                                         params: P) {
        self.graphics.draw(&buffer.batch, &params, &self.frame);
    }

    pub fn end_frame(&mut self) {
        self.graphics.end_frame();
    }
}
