use super::{HlsInfo};

pub struct CarInfo {
    pub width: i16,
    pub height: i16,
    pub depth: i16,

    /// the first sprite number offset for this car. Note that this
    /// number is relative to the first car sprite in the sprites
    /// file.
    pub spr_num: i16,
    /// The weight descriptor for the car.
    pub weight: i16,

    pub min_speed: i16,
    pub max_speed: i16,
    pub acceleration: i16,
    pub braking: i16,
    pub grip: i16,
    pub handling: i16,

    pub remap24: [HlsInfo, ..12],

    pub remap8: [u8, ..12],
    pub vehicle_type: VehicleType,
    /// A sub-type within `vehicle_type` for cars which hold an
    /// identifier for the model of car.
    pub model: u8,
    pub turning: u8,
    /// How easily the car can sustain damage.
    pub damageable: u8,
    /// The monetary value of the car in the GTA mission, in 1000s of
    /// $. There are 4 value entries for the 4 cranes.
    pub value: [u16, ..4],

    // The pixel co-ordinates of the centre of mass of the car,
    // relative to the graphical centre.
    pub cx: i8,
    pub cy: i8,
    /// The moment of inertia of the car.
    pub moment: i32,
    /// The total mass of the car.
    pub mass: f32,
    /// The 
    pub gear_thrust_ratio: f32,
    pub tyre_adhesion_x: f32,
    pub tyre_adhesion_y: f32,
    /// The friction of the handbrake.
    pub handbrake_friction: f32,
    /// The friction of the footbrake.
    pub footbrake_friction: f32,
    /// The front bias of braking.
    pub front_brake_bias: f32,

    /// The turn ratio of the car.
    pub turn_ratio: i16,

    /// More handling controls.
    pub drive_wheel_offset: i16,
    pub steering_wheel_offset: i16,
    pub back_end_slide_value: f32,
    pub handbrake_slide_value: f32,

    /// Is the car convertible.
    pub convertible: bool,

    /// The engine type of the car(for sound effects).
    pub engine: u8,
    /// The radio listening type of the car.
    pub radio: u8,
    /// The horn type of the car.
    pub horn: u8,

    /// For audio information.
    pub sound_function: u8,
    pub fast_change_flag: u8,

    pub doors: Vec<Door>
}

pub enum VehicleType {
    Unknown,
    Bus,
    JuggernautFront,
    JuggernautBack,
    Motorcycle,
    Car,
    Train
}

impl VehicleType {
    pub fn new(n: u8) -> VehicleType {
        match n {
            0 => Bus,
            1 => JuggernautFront,
            2 => JuggernautBack,
            3 => Motorcycle,
            4 => Car,
            8 => Train,
            _ => Unknown
        }
    }
}

pub struct Door {
    /// Relative x-position.
    pub x: i16,
    /// Relative y-position.
    pub y: i16,
    /// Object number. Refers to object in `Map`.
    pub object: i16,
    /// TODO: Unknown
    pub delta: i16
}
