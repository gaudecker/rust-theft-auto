pub struct Block {
    pub type_map: u16,
    pub type_map_ext: u8,

    pub west: u8,
    pub east: u8,
    pub north: u8,
    pub south: u8,
    pub lid: u8
}

// Methods for type_map.
impl Block {
    /// Returns `true` if vehicles are allowed to drive north.
    pub fn is_north_direction_allowed(&self) -> bool {
        self.type_map & 1 != 0
    }

    /// Returns `true` if vehicles are allowed to drive south.
    pub fn is_south_direction_allowed(&self) -> bool {
        self.type_map & 2 != 0
    }

    /// Returns `true` if vehicles are allowed to drive west.
    pub fn is_west_direction_allowed(&self) -> bool {
        self.type_map & 4 != 0
    }

    /// Returns `true` if vehicles are allowed to drive east.
    pub fn is_east_direction_allowed(&self) -> bool {
        self.type_map & 8 != 0
    }

    pub fn get_block_type(&self) -> BlockType {
        let v1: u8 = if self.type_map & 16 != 0 { 1 } else { 0 };
        let v2: u8 = if self.type_map & 32 != 0 { 2 } else { 0 };
        let v3: u8 = if self.type_map & 64 != 0 { 4 } else { 0 };

        match v1 + v2 + v3 {
            0 => Air,
            1 => Water,
            2 => Road,
            3 => Pavement,
            4 => Field,
            5 => Building,
            _ => Unused
        }
    }

    pub fn get_slope_type(&self) -> u8 {
        let v1: u8 = if self.type_map & 256 != 0 { 1 } else { 0 };
        let v2: u8 = if self.type_map & 512 != 0 { 2 } else { 0 };
        let v3: u8 = if self.type_map & 1024 != 0 { 4 } else { 0 };
        let v4: u8 = if self.type_map & 2048 != 0 { 8 } else { 0 };
        let v5: u8 = if self.type_map & 4096 != 0 { 16 } else { 0 };
        let v6: u8 = if self.type_map & 8192 != 0 { 32 } else { 0 };

        v1 + v2 + v3 + v4 + v5 + v6
    }

    pub fn is_flat(&self) -> bool {
        self.type_map & 128 != 0
    }

    /// Returns the rotation of the lid in degrees.
    pub fn get_lid_rotation(&self) -> u16 {
        let v1: u8 = if self.type_map & 16384 != 0 { 1 } else { 0 };
        let v2: u8 = if self.type_map & 32768 != 0 { 2 } else { 0 };

        match v1 + v2 {
            1 => 90,
            2 => 180,
            3 => 270,
            _ => 0
        }
    }
}

// Methods for type_map_ext.
impl Block {
    pub fn is_traffic_light(&self) -> bool {
        self.type_map_ext & 1 != 0
    }

    pub fn is_railway_end_turn(&self) -> bool {
        self.type_map_ext & 4 != 0
    }

    pub fn is_railway_start_turn(&self) -> bool {
        self.type_map_ext & 4 != 0 && self.type_map_ext & 1 != 0
    }

    pub fn is_railway_station(&self) -> bool {
        self.type_map_ext & 4 != 0 && self.type_map_ext & 2 != 0
    }

    pub fn is_railway_train(&self) -> bool {
        self.type_map_ext & 4 != 0 && self.type_map_ext & 2 != 0 &&
            self.type_map_ext & 1 != 0
    }

    pub fn get_remap_index(&self) -> bool {
        let first_cond: u8 = if self.type_map_ext & 8 != 0 { 1 } else { 0 };
        let second_cond: u8 = if self.type_map_ext & 16 != 0 { 2 } else { 0 };
        first_cond + second_cond != 0
    }

    pub fn should_flip_north_south(&self) -> bool {
        self.type_map_ext & 32 != 0
    }

    pub fn should_flip_east_west(&self) -> bool {
        self.type_map_ext & 64 != 0
    }

    pub fn is_railway(&self) -> bool {
        self.type_map_ext & 128 != 0
    }
}

#[deriving(PartialEq)]
pub enum BlockType {
    Air,
    Water,
    Road,
    Pavement,
    Field,
    Building,
    Unused
}
