use gfx;
use gfx::{Device, DeviceHelper, VertexFormat, BufferHandle, ToSlice};
use gfx::shade::ShaderParam;
use device;
use device::draw::CommandBuffer;
use render::batch::RefBatch;

use super::Renderer;
use super::program::Program;

pub struct Buffer<V: VertexFormat + Copy, P: ShaderParam<L>, L> {
    pub buf: BufferHandle<V>,
    pub batch: RefBatch<L, P>
}

impl<V: VertexFormat + Copy, P: ShaderParam<L>, L> Buffer<V, P, L> {
    /// Creates a new buffer from `vertex_data` and `index_data`.
    pub fn new<D: Device<C>, C: CommandBuffer>(r: &mut Renderer<D, C>,
                                               program: &Program,
                                               vertex_data: &[V],
                                               index_data: &[u32]) -> Buffer<V, P, L> {
        let buf = r.graphics.device.create_buffer(vertex_data.len(), gfx::UsageStatic);
        r.graphics.device.update_buffer(buf, vertex_data, 0);

        let mesh = gfx::Mesh::from_format(buf, vertex_data.len() as u32);
        let slice = r.graphics.device.create_buffer_static::<u32>(index_data)
            .to_slice(gfx::TriangleList);

        Buffer {
            buf: buf,
            batch: r.graphics.make_batch(&program.handle, &mesh,
                                         slice,
                                         &r.drawstate).unwrap()
        }
    }
}
