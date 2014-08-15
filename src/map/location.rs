#[deriving(Show)]
pub struct Location {
    pub location_type: LocationType,
    pub position: super::position::Position
}

#[deriving(Hash, Eq, PartialEq, Show)]
pub enum LocationType {
    PoliceStation,
    Hospital,
    FireStation,
    Unknown
}
