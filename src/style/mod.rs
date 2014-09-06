
use std::vec::Vec;
use std::io::{File, IoResult};
use std::mem::size_of;

pub use self::animation::{Animation, AreaType};
pub use self::object_info::{ObjectInfo, Status};
pub use self::car_info::{CarInfo, Door, VehicleType};
pub use self::sprite_info::{SpriteInfo, Delta};

pub mod animation;
pub mod object_info;
pub mod car_info;
pub mod sprite_info;

pub struct Style {
    side_tile_faces: Vec<u8>,
    lid_tile_faces: Vec<u8>,
    aux_tile_faces: Vec<u8>
}

struct Header {
    version: u32,
    side_size: u32,
    lid_size: u32,
    aux_size: u32,
    anim_size: u32,
    clut_size: u32,
    tileclut_size: u32,
    spriteclut_size: u32,
    newcarclut_size: u32,
    fontclut_size: u32,
    palette_index_size: u32,
    object_info_size: u32,
    car_info_size: u32,
    sprite_info_size: u32,
    sprite_graphics_size: u32,
    sprite_numbers_size: u32
}

struct HlsInfo {
    h: i16,
    l: i16,
    s: i16
}

impl HlsInfo {
    pub fn new(h: i16, l: i16, s: i16) -> HlsInfo {
        HlsInfo {
            h: h,
            l: l,
            s: s
        }
    }

    pub fn zero() -> HlsInfo {
        HlsInfo {
            h: 0,
            l: 0,
            s: 0
        }
    }
}

impl Style {
    pub fn from_file(filename: &str) -> IoResult<Style> {
        println!("Loading style {}", filename);

        let mut f = match File::open(&Path::new(filename)) {
            Err(why) => fail!("Could not read {}: {}", filename, why.desc),
            Ok(file) => file
        };

        let header = Header {
            version: try!(f.read_le_u32()),
            side_size: try!(f.read_le_u32()),
            lid_size: try!(f.read_le_u32()),
            aux_size: try!(f.read_le_u32()),
            anim_size: try!(f.read_le_u32()),
            clut_size: try!(f.read_le_u32()),
            tileclut_size: try!(f.read_le_u32()),
            spriteclut_size: try!(f.read_le_u32()),
            newcarclut_size: try!(f.read_le_u32()),
            fontclut_size: try!(f.read_le_u32()),
            palette_index_size: try!(f.read_le_u32()),
            object_info_size: try!(f.read_le_u32()),
            car_info_size: try!(f.read_le_u32()),
            sprite_info_size: try!(f.read_le_u32()),
            sprite_graphics_size: try!(f.read_le_u32()),
            sprite_numbers_size: try!(f.read_le_u32())
        };

        let side_faces = try!(read_faces(header.side_size, &mut f));
        let lid_faces = try!(read_faces(header.lid_size, &mut f));
        let aux_faces = try!(read_faces(header.aux_size, &mut f));

        let anims = try!(read_anims(&mut f));
        let cluts = try!(read_cluts(header.clut_size, &mut f));
        let pal_index = try!(read_palette(header.palette_index_size, &mut f));
        let objs = try!(read_object_info(header.object_info_size, &mut f));
        let cars = try!(read_car_info(header.car_info_size, &mut f));
        let sprites = try!(read_sprite_info(header.sprite_info_size, &mut f));

        println!("Sprites {}", sprites.len());

        Ok(Style {
            side_tile_faces: side_faces,
            lid_tile_faces: lid_faces,
            aux_tile_faces: aux_faces
        })
    }
}

/// Reads `num_faces` tile faces.
fn read_faces(num_faces: u32, f: &mut File) -> IoResult<Vec<u8>> {
    let mut faces = Vec::with_capacity(num_faces as uint);
    for n in range(0, num_faces) {
        faces.push(try!(f.read_u8()));
    }

    Ok(faces)
}

/// Reads all block animations.
fn read_anims(f: &mut File) -> IoResult<Vec<Animation>> {
    let num_anims = try!(f.read_u8());
    let mut anims = Vec::with_capacity(num_anims as uint);
    for n in range(0, num_anims) {
        let block = try!(f.read_u8());
        let area_type = AreaType::new(try!(f.read_u8()));
        let speed = try!(f.read_u8());

        let num_frames = try!(f.read_u8());
        
        let mut frames = Vec::with_capacity(num_frames as uint);
        for l in range(0, num_frames) {
            frames.push(try!(f.read_u8()));
        }

        anims.push(Animation {
            block: block,
            area_type: area_type,
            speed: speed,
            frames: frames
        });
    }

    Ok(anims)
}

fn read_cluts(clut_size: u32, f: &mut File) -> IoResult<Vec<u8>> {
    let mut paged_clut_size = clut_size;
    if (clut_size % 65536 != 0) {
      paged_clut_size += (65536 - (clut_size % 65536));
    }

    let mut clut = Vec::with_capacity(paged_clut_size as uint);
    for n in range(0, paged_clut_size) {
        clut.push(try!(f.read_u8()));
    }

    Ok(clut)
}

fn read_palette(palette_index_size: u32, f: &mut File) -> IoResult<Vec<u16>> {
    let mut index = Vec::with_capacity(palette_index_size as uint);
    for n in range(0, palette_index_size / 2) {
        index.push(try!(f.read_le_u16()));
    }

    Ok(index)
}

fn read_object_info(object_info_size: u32, f: &mut File) -> IoResult<Vec<ObjectInfo>> {
    let mut objs = Vec::with_capacity(object_info_size as uint / 20);
    for n in range(0, object_info_size / 20) {
        objs.push(ObjectInfo {
            width: try!(f.read_le_u32()),
            height: try!(f.read_le_u32()),
            depth: try!(f.read_le_u32()),

            spr_num: try!(f.read_le_u16()),
            weight: try!(f.read_le_u16()),
            aux: try!(f.read_le_u16()),

            status: Status::new(try!(f.read_i8())),
            breaks_into: try!(f.read_u8())
        });
    }

    Ok(objs)
}

fn read_car_info(car_info_size: u32, f: &mut File) -> IoResult<Vec<CarInfo>> {
    let mut cars = Vec::new();

    let max_offset = try!(f.tell()) + car_info_size as u64;
    while try!(f.tell()) < max_offset {
        let width = try!(f.read_le_i16());
        let height = try!(f.read_le_i16());
        let depth = try!(f.read_le_i16());

        let sprite_num = try!(f.read_le_i16());

        let weight = try!(f.read_le_i16());
        let max_speed = try!(f.read_le_i16());
        let min_speed = try!(f.read_le_i16());
        let acceleration = try!(f.read_le_i16());
        let braking = try!(f.read_le_i16());
        let grip = try!(f.read_le_i16());
        let handling = try!(f.read_le_i16());

        let mut remap24 = [HlsInfo::zero(), ..12];
        for i in range(0, 12) {
            remap24[i] = HlsInfo::new(
                try!(f.read_le_i16()),
                try!(f.read_le_i16()),
                try!(f.read_le_i16())
            );
        }

        let mut remap8 = [0, ..12];
        for i in range(0, 12) {
            remap8[i] = try!(f.read_u8());
        }

        let vehicle_type = VehicleType::new(try!(f.read_u8()));
        let model = try!(f.read_u8());
        let turning = try!(f.read_u8());
        let damageable = try!(f.read_u8());

        let mut value = [0, ..4];
        for i in range(0, 4) {
            value[i] = try!(f.read_le_u16());
        }

        let cx = try!(f.read_i8());
        let cy = try!(f.read_i8());

        let moment = try!(f.read_le_i32());

        let mass = try!(f.read_le_u32()) as f32 / 65536.0;
        let g1_thrust = try!(f.read_le_u32()) as f32 / 65536.0;
        let tyre_adhesion_x = try!(f.read_le_u32()) as f32 / 65536.0;
        let tyre_adhesion_y = try!(f.read_le_u32()) as f32 / 65536.0;
        let hb_friction = try!(f.read_le_u32()) as f32 / 65536.0;
        let fb_friction = try!(f.read_le_u32()) as f32 / 65536.0;
        let fb_bias = try!(f.read_le_u32()) as f32 / 65536.0;

        let turn_ratio = try!(f.read_le_i16());
        let drive_wheel_offset = try!(f.read_le_i16());
        let streering_wheel_offset = try!(f.read_le_i16());

        let be_slide = try!(f.read_le_u32()) as f32 / 65536.0;
        let hb_slide = try!(f.read_le_u32()) as f32 / 65536.0;

        let convertible = try!(f.read_u8()) == 1;
        
        let engine = try!(f.read_u8());
        let radio = try!(f.read_u8());
        let horn = try!(f.read_u8());

        let sound_fn = try!(f.read_u8());
        let fast_change_flag= try!(f.read_u8());

        let num_doors = try!(f.read_le_i16());
        let mut doors = Vec::with_capacity(num_doors as uint);
        for i in range(0, num_doors) {
            doors.push(Door {
                x: try!(f.read_le_i16()),
                y: try!(f.read_le_i16()),
                object: try!(f.read_le_i16()),
                delta :try!(f.read_le_i16())
            });
        }

        cars.push(CarInfo {
            width: width,
            height: height,
            depth: depth,

            spr_num: sprite_num,

            weight: weight,
            min_speed: min_speed,
            max_speed: max_speed,
            acceleration: acceleration,
            braking: braking,
            grip: grip,
            handling: handling,

            remap24: remap24,
            remap8: remap8,

            vehicle_type: vehicle_type,
            model: model,
            turning: turning,
            damageable: damageable,
            value: value,

            cx: cx,
            cy: cy,

            moment: moment,
            mass: mass,

            gear_thrust_ratio: g1_thrust,
            tyre_adhesion_x: tyre_adhesion_x,
            tyre_adhesion_y: tyre_adhesion_y,
            handbrake_friction: hb_friction,
            footbrake_friction: fb_friction,
            front_brake_bias: fb_bias,

            turn_ratio: turn_ratio,
            drive_wheel_offset: drive_wheel_offset,
            steering_wheel_offset: streering_wheel_offset,
            back_end_slide_value: be_slide,
            handbrake_slide_value: hb_slide,

            convertible: convertible,

            engine: engine,
            radio: radio,
            horn: horn,

            sound_function: sound_fn,
            fast_change_flag: fast_change_flag,

            doors: doors
        })
    }

    Ok(cars)
}

fn read_sprite_info(sprite_info_size: u32, f: &mut File) -> IoResult<Vec<SpriteInfo>> {
    let max_offset = try!(f.tell()) + sprite_info_size as u64;
    let mut sprites = Vec::new();

    while try!(f.tell()) < max_offset {
        let width = try!(f.read_u8());
        let height = try!(f.read_u8());

        let num_deltas = try!(f.read_le_u16());

        // Skip scaling flag
        try!(f.read_u8());

        let size = try!(f.read_le_u16());
        let clut = try!(f.read_le_u16());
        let x = try!(f.read_u8());
        let y = try!(f.read_u8());

        let page = try!(f.read_le_u16());

        let mut deltas = Vec::with_capacity(num_deltas as uint);
        for i in range(0, num_deltas) {
            deltas.push(Delta {
                size: try!(f.read_le_u16()),
                w: try!(f.read_le_u32())
            });
        }

        sprites.push(SpriteInfo {
            width: width,
            height: height,

            size: size,
            clut: clut,
            
            x: x,
            y: y,

            page: page,

            deltas: deltas
        });
    }

    Ok(sprites)
}
