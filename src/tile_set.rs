use piston::image::{
    GenericImage,
    ImageBuf,
    Rgba,
    Luma
};

pub struct TileSet {
    pub width: uint,
    pub height: uint,

    pub buffer: ImageBuf<Luma<u8>>
}

impl TileSet {
    pub fn new(width: uint, height: uint, f: |u32, u32| -> Luma<u8>) -> TileSet {
        TileSet {
            width: width,
            height: height,
            buffer: ImageBuf::from_fn(width as u32, height as u32, f)
        }
    }
}
