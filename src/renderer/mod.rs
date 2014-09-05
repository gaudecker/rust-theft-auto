#[phase(plugin)]
extern crate gfx_macros;

use gfx;
use gfx::{Device, VertexFormat};
use gfx::shade::{ShaderParam, TextureParam};
use device;
use device::draw::CommandBuffer;
use image;
use image::{GenericImage, ImageBuf, MutableRefImage, Pixel, Rgba};

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
    pub view: [[f32, ..4], ..4],
    #[name = "s_texture"]
    pub texture: TextureParam
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

pub struct Texture {
    pub handle: gfx::TextureHandle,
    width: u32,
    height: u32
}

impl Texture {
    /// Loads image by relative file name to the asset root.
    pub fn from_file<D: Device<C>, C: CommandBuffer>(path: &Path, d: &mut D) -> Result<Texture, String> {
        Ok(Texture::from_rgba8(try!(load_rgba8(path)), d))
    }

    pub fn from_rgba8<D: Device<C>, C: CommandBuffer>(img: ImageBuf<Rgba<u8>>, d: &mut D) -> Texture {
        let (width, height) = img.dimensions();

        let mut ti = gfx::tex::TextureInfo::new();
        ti.width = width as u16;
        ti.height = height as u16;
        ti.kind = gfx::tex::Texture2D;
        ti.format = gfx::tex::RGBA8;

        let tex = d.create_texture(ti).unwrap();

        d.update_texture(&tex, &ti.to_image_info(), &img.into_vec()).unwrap();
        d.generate_mipmap(&tex);

        Texture {
            handle: tex,
            width: width,
            height: height
        }
    }
}

fn load_rgba8(path: &Path) -> Result<ImageBuf<Rgba<u8>>, String> {
    match image::open(path) {
        Ok(image::ImageRgba8(img)) => Ok(img),
        Ok(image::ImageRgb8(img)) => {
            let (w, h) = img.dimensions();
            Ok(ImageBuf::from_fn(w, h, |x, y| img.get_pixel(x, y).to_rgba()))
        }
        Ok(img) => return Err(format!("Unsupported color type {} in '{}'",
                                      img.color(), path.display())),
        Err(e) => return Err(format!("Could not load '{}': {}", path.display(), e))
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
