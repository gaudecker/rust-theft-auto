use piston::image::{
    Rgba
};

pub struct PaletteIndex {
    pub index: Vec<u16>,
}

impl PaletteIndex {
    pub fn new(index: Vec<u16>) -> PaletteIndex {
        PaletteIndex {
            index: index
        }
    }

    pub fn look_tile(&self, tile: uint) -> uint {
        self.index[4 * tile] as uint
    }
}
