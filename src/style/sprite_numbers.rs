/// A list of numbers which is used to reference particular sprite
/// types.
///
/// Each of these numbers stores the number of sprites of that
/// particular type. The number can be zero if there are no sprites of
/// that type in the style.
#[deriving(Show)]
pub struct SpriteNumbers {
    pub arrow: u16,
    pub digits: u16,
    pub boat: u16,
    pub case: u16,
    pub bus: u16,
    pub car: u16,
    pub object: u16,
    pub pedestrian: u16,
    pub speedo: u16,
    pub tank: u16,
    pub traffic_lights: u16,
    pub train: u16,
    pub trdoors: u16,
    pub bike: u16,
    pub tram: u16,
    pub wrecked_bus: u16,
    pub wrecked_car: u16,
    pub ex: u16,
    pub tumcar: u16,
    pub tumtruck: u16,
    pub ferry: u16
}
