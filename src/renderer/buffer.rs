use gfx;
use gfx::{Device, DeviceHelper, VertexFormat, BufferHandle};
use gfx::shade::ShaderParam;
use device;
use device::draw::CommandBuffer;
use render::batch::RefBatch;

use super::Renderer;
use super::program::Program;

pub struct Buffer<V: VertexFormat, P: ShaderParam<L>, L> {
    pub buf: BufferHandle<V>,
    pub batch: RefBatch<L, P>
}

impl<V: VertexFormat, P: ShaderParam<L>, L> Buffer<V, P, L> {
    /// Creates a new buffer from `data`.
    pub fn new<D: Device<C>, C: CommandBuffer>(r: &mut Renderer<D, C>,
                                               program: &Program,
                                               data: &[V]) -> Buffer<V, P, L> {
        let buf = r.graphics.device.create_buffer(data.len(), gfx::UsageStatic);
        r.graphics.device.update_buffer(buf, &data, 0);

        let mesh = gfx::Mesh::from_format(buf, data.len() as u32);

        Buffer {
            buf: buf,
            batch: r.graphics.make_batch(&mesh, mesh.get_slice(gfx::TriangleStrip),
                                         &program.handle, &r.drawstate).unwrap()
        }
    }
}
