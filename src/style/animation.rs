pub struct Animation {
    /// The block number.
    pub block: u8,
    pub area_type: AreaType,
    /// The number of frames to show each frame.
    pub speed: u8,
    /// The animation frames, these refer to aux faces.
    pub frames: Vec<u8>
}

pub enum AreaType {
    Side = 0,
    Lid = 1
}

impl AreaType {
    pub fn new(t: u8) -> AreaType {
        match t {
            0 => Side,
            _ => Lid
        }
    }
}
