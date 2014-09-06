pub struct ObjectInfo {
    pub width: u32, 
    pub height: u32,
    pub depth: u32,
    pub spr_num: u16,
    pub weight: u16,
    pub aux: u16,
    pub status: Status,
    pub breaks_into: u8,
    //pub into: Vec<u16>
}

/// A descriptor for an object, which determines how it behaves.
pub enum Status {
    Normal,
    /// Can be driven over.
    Ignorable,
    /// Breaks on landing
    Smashable,
    Invisible,
    Animation,
    CarUpgrade,
    Helipad,
    Powerup
}

impl Status {
    pub fn new(n: i8) -> Status {
        match n {
            1 => Ignorable,
            2 => Smashable,
            3 => Invisible,
            5 => Animation,
            6 => CarUpgrade,
            8 => Helipad,
            9 => Powerup,
            _ => Normal
        }
    }
}
