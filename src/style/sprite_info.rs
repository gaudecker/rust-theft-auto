pub struct SpriteInfo {
    pub width: u8,
    pub height: u8,

    pub size: u16,
    pub clut: u16,

    pub x: u8,
    pub y: u8,

    pub page: u16,

    pub deltas: Vec<Delta>
}

pub struct Delta {
    pub size: u16,
    pub w: u32
}
