
use std::vec::Vec;
use std::io::{File, IoResult, SeekSet};
use std::mem::size_of;
use piston::image::{Rgba, Luma};

pub use self::animation::{Animation, AreaType};
pub use self::object_info::{ObjectInfo, Status};
pub use self::car_info::{CarInfo, Door, VehicleType};
pub use self::sprite_info::{SpriteInfo, Delta};
pub use self::sprite_numbers::{SpriteNumbers};

use super::TileSet;

use self::hls_info::HlsInfo;
use self::palette_index::PaletteIndex;

pub mod animation;
pub mod object_info;
pub mod car_info;
pub mod sprite_info;
pub mod sprite_numbers;

mod hls_info;
mod palette_index;

pub struct Style {
    pub tiles: TileSet
}

struct Header {
    version: uint,
    side_size: uint,
    lid_size: uint,
    aux_size: uint,
    anim_size: uint,
    clut_size: uint,
    tileclut_size: uint,
    spriteclut_size: uint,
    newcarclut_size: uint,
    fontclut_size: uint,
    palette_index_size: uint,
    object_info_size: uint,
    car_info_size: uint,
    sprite_info_size: uint,
    sprite_graphics_size: uint,
    sprite_numbers_size: uint
}

impl Style {
    pub fn from_file(filename: &str) -> IoResult<Style> {
        println!("Loading style {}", filename);

        let mut f = match File::open(&Path::new(filename)) {
            Err(why) => panic!("Could not read {}: {}", filename, why.desc),
            Ok(file) => file
        };

        let h = Header {
            version: try!(f.read_le_u32()) as uint,
            side_size: try!(f.read_le_u32()) as uint,
            lid_size: try!(f.read_le_u32()) as uint,
            aux_size: try!(f.read_le_u32()) as uint,
            anim_size: try!(f.read_le_u32()) as uint,
            clut_size: try!(f.read_le_u32()) as uint,
            tileclut_size: try!(f.read_le_u32()) as uint,
            spriteclut_size: try!(f.read_le_u32()) as uint,
            newcarclut_size: try!(f.read_le_u32()) as uint,
            fontclut_size: try!(f.read_le_u32()) as uint,
            palette_index_size: try!(f.read_le_u32()) as uint,
            object_info_size: try!(f.read_le_u32()) as uint,
            car_info_size: try!(f.read_le_u32()) as uint,
            sprite_info_size: try!(f.read_le_u32()) as uint,
            sprite_graphics_size: try!(f.read_le_u32()) as uint,
            sprite_numbers_size: try!(f.read_le_u32()) as uint
        };

        let mut r = StyleReader::new(&mut f, h);
        {
            let s_size = h.side_size as uint;
            let l_size = h.lid_size as uint;
            let a_size = h.aux_size as uint;
            let pad = (4 - s_size / 4096 + l_size / 4096 + a_size / 4096) * 4096;
            r.face_size = s_size + l_size + a_size + pad;

            r.clut_size = h.clut_size;
            if h.clut_size % 65536 != 0 {
                r.clut_size += 65536 - (h.clut_size % 65536);
            }
        }

        let tiles = try!(r.read_tiles());

        // let faces = try!(r.read_faces());
        // let anims = try!(r.read_anims());
        // let cluts = try!(r.read_cluts());
        // let pal_index = try!(r.read_palette_index());
        // let objs = try!(r.read_object_info());
        // let cars = try!(r.read_car_info());
        // let sprite_info = try!(r.read_sprite_info());
        // let sprite_gfx = try!(r.read_sprite_graphics());
        // let sprite_numbers = try!(r.read_sprite_numbers());

        // println!("{} tiles", faces.len());
        // println!("{} animations", anims.len());
        // println!("{} cluts", cluts.len());
        // println!("palette index size {}", pal_index.len());
        // println!("{} objects", objs.len());
        // println!("{} cars", cars.len());
        // println!("{} sprites", sprite_info.len());
        // println!("{} sprite_graphics", sprite_gfx.len());
        // println!("{}", sprite_numbers);

        Ok(Style {
            tiles: tiles
        })
    }
}

struct StyleReader<'a> {
    f: &'a mut File,
    h: Header,
    header_size: uint,
    face_size: uint,
    clut_size: uint
}

impl<'a> StyleReader<'a> {
    fn new(f: &'a mut File, h: Header) -> StyleReader {
        StyleReader {
            f: f,
            h: h,
            header_size: size_of::<Header>(),
            face_size: 0,
            clut_size: 0
        }
    }

    /// Reads all tile faces.
    fn read_faces(&mut self) -> IoResult<Vec<u8>> {
        let hs = self.header_size;
        try!(self.seek(hs));
        assert!(try!(self.tell()) == self.header_size);

        let num_faces = self.h.side_size + self.h.lid_size + self.h.aux_size;
        let mut faces = Vec::with_capacity(num_faces as uint);
        for n in range(0, num_faces) {
            faces.push(try!(self.read_u8()));
        }

        Ok(faces)
    }

    /// Reads all block animations.
    fn read_anims(&mut self) -> IoResult<Vec<Animation>> {
        let offset = self.header_size + self.face_size;
        try!(self.seek(offset));
        assert!(try!(self.tell()) == self.header_size + self.face_size);

        let num_anims = try!(self.read_u8());
        let mut anims = Vec::with_capacity(num_anims as uint);
        for n in range(0, num_anims) {
            let block = try!(self.read_u8());
            let area_type = AreaType::new(try!(self.read_u8()));
            let speed = try!(self.read_u8());

            let num_frames = try!(self.read_u8());
            
            let mut frames = Vec::with_capacity(num_frames as uint);
            for l in range(0, num_frames) {
                frames.push(try!(self.read_u8()));
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

    /// Reads all cluts.
    // fn read_cluts(&mut self) -> IoResult<Vec<u8>> {
    //     let offset = self.header_size + self.face_size + self.h.anim_size as u64;
    //     try!(self.seek(offset));
    //     assert!(try!(self.tell()) ==  self.header_size + self.face_size + self.h.anim_size as u64);

    //     let mut paged_clut_size = self.h.clut_size;
    //     if (self.h.clut_size % 65536 != 0) {
    //         paged_clut_size += (65536 - (self.h.clut_size % 65536));
    //     }

    //     let mut clut = Vec::with_capacity(paged_clut_size as uint);
    //     for n in range(0, paged_clut_size) {
    //         clut.push(try!(self.read_u8()));
    //     }

    //     Ok(clut)
    // }

    // fn read_cluts(&mut self) -> IoResult<Vec<Vec<Vec<Rgba<u8>>>>> {
    //     let offset = self.header_size + self.face_size + self.h.anim_size;
    //     try!(self.seek(offset));
    //     assert!(try!(self.tell()) == offset);

    //     let num_pages = self.h.clut_size as uint / (64 * 256 * 4);
    //     let mut clut_pages: Vec<Vec<Vec<Rgba<u8>>>> = Vec::with_capacity(num_pages);
        
    //     for page in range(0, num_pages) {
    //         let num_data = 64 * 256;
    //         let mut clut_data: Vec<Vec<Rgba<u8>>> = Vec::with_capacity(num_data);

    //         for p in range(0, num_data) {
    //             if p < 64 {
    //                 clut_data.insert(p, Vec::new());
    //             }

    //             let b = try!(self.read_u8());
    //             let g = try!(self.read_u8());
    //             let r = try!(self.read_u8());
    //             let a = try!(self.read_u8());

    //             if p % 64 > clut_data.len() {
    //                 println!("This should not happen");
    //             }

    //             clut_data.get_mut(p % 64).push(Rgba(r, g, b, a));
    //         }

    //         clut_pages.push(clut_data);
    //     }

    //     Ok(clut_pages)
    // }

    fn read_tiles(&mut self) -> IoResult<TileSet> {
        let width: uint = 256;
        let height: uint = self.face_size / width;
        let tile_size: uint = 64;

        println!("Tileset size {}x{}", width, height);

        // let palette = try!(self.read_palette_index());
        // let cluts = try!(self.read_cluts());

        let header_size = self.header_size;
        try!(self.seek(header_size));
        // for y in range(0, height) {
        //     for x in range(0, width) {
        //         let color_index = try!(self.read_u8());
        //         let tile = (((y as uint / tile_size) * width) + x as uint) / tile_size;
        //         let clut = palette.lookup_tile(tile);
        //     }
        // }

        let tiles = try!(self.read_faces());
        // let palette = try!(self.read_palette_index());

        // println!("Index {}", palette.index.len());
        // println!("Color {}", palette.palettes.len());

        // try!(self.seek(size_of::<Header>()));

        Ok(TileSet::new(width, height, |x, y| {
            let offset = x as uint * width + y as uint;
            let color = tiles[offset];
            Luma(color)
        }))
    }

    /// Reads the palette index.
    fn read_palette_index(&mut self) -> IoResult<PaletteIndex> {
        let offset =  self.header_size + self.face_size + 
            self.h.anim_size + self.clut_size;
        try!(self.seek(offset));
        assert!(try!(self.tell()) == offset);

        
        let mut index = Vec::with_capacity(self.h.palette_index_size as uint);
        for n in range(0, self.h.palette_index_size / 2) {
            index.push(try!(self.read_u16()));
        }

        Ok(PaletteIndex::new(index))
    }

    /// Reads all object infos.
    fn read_object_info(&mut self) -> IoResult<Vec<ObjectInfo>> {
        let offset =  self.header_size + self.face_size + 
            self.h.anim_size + self.h.clut_size + self.h.palette_index_size;
        try!(self.seek(offset));
        assert!(try!(self.tell()) == offset);

        let mut objs = Vec::with_capacity(self.h.object_info_size as uint / 20);
        for n in range(0, self.h.object_info_size / 20) {
            objs.push(ObjectInfo {
                width: try!(self.read_u32()),
                height: try!(self.read_u32()),
                depth: try!(self.read_u32()),

                spr_num: try!(self.read_u16()),
                weight: try!(self.read_u16()),
                aux: try!(self.read_u16()),

                status: Status::new(try!(self.read_i8())),
                breaks_into: try!(self.read_u8())
            });
        }

        Ok(objs)
    }

    fn read_car_info(&mut self) -> IoResult<Vec<CarInfo>> {
        let offset =  self.header_size + self.face_size + 
            self.h.anim_size + self.h.clut_size +
            self.h.palette_index_size + self.h.object_info_size;
        try!(self.seek(offset));
        assert!(try!(self.tell()) == offset);

        fn read_remap_24(f: &mut File) -> IoResult<[HlsInfo, ..12]> {
            let mut remap24 = [HlsInfo::zero(), ..12];
            for i in range(0, 12) {
                remap24[i] = HlsInfo::new(
                    try!(f.read_le_i16()),
                    try!(f.read_le_i16()),
                    try!(f.read_le_i16())
                );
            }
            Ok(remap24)
        }

        fn read_remap_8(f: &mut File) -> IoResult<[u8, ..12]> {
            let mut remap8 = [0, ..12];
            for i in range(0, 12) {
                remap8[i] = try!(f.read_u8());
            }
            Ok(remap8)
        }

        fn read_value(f: &mut File) -> IoResult<[u16, ..4]> {
            let mut value = [0, ..4];
            for i in range(0, 4) {
                value[i] = try!(f.read_le_u16());
            }
            Ok(value)
        }

        fn read_doors(f: &mut File) -> IoResult<Vec<Door>> {
            let mut num_doors = try!(f.read_le_i16());
            if num_doors > 2 { num_doors = 0; }

            let mut doors = Vec::with_capacity(num_doors as uint);
            for i in range(0, num_doors) {
                doors.push(Door {
                    x: try!(f.read_le_i16()),
                    y: try!(f.read_le_i16()),
                    object: try!(f.read_le_i16()),
                    delta :try!(f.read_le_i16())
                });
            }
            Ok(doors)
        }

        let mut cars = Vec::new();

        let max_offset = try!(self.tell()) + self.h.car_info_size;
        while try!(self.tell()) < max_offset {
            cars.push(CarInfo {
                width: try!(self.read_i16()),
                height: try!(self.read_i16()),
                depth: try!(self.read_i16()),

                sprite_number: try!(self.read_i16()),

                weight: try!(self.read_i16()),
                max_speed: try!(self.read_i16()),
                min_speed: try!(self.read_i16()),
                acceleration: try!(self.read_i16()),
                braking: try!(self.read_i16()),
                grip: try!(self.read_i16()),
                handling: try!(self.read_i16()),

                remap24: try!(read_remap_24(self.f)),
                remap8: try!(read_remap_8(self.f)),

                vehicle_type: VehicleType::new(try!(self.read_u8())),
                model: try!(self.read_u8()),
                turning: try!(self.read_u8()),
                damageable: try!(self.read_u8()),

                value: try!(read_value(self.f)),

                cx: try!(self.read_i8()),
                cy: try!(self.read_i8()),
                moment: try!(self.read_i32()),
                mass: try!(self.read_u32()) as f32 / 65536.0,

                gear_thrust_ratio: try!(self.read_u32()) as f32 / 65536.0,
                tyre_adhesion_x: try!(self.read_u32()) as f32 / 65536.0,
                tyre_adhesion_y: try!(self.read_u32()) as f32 / 65536.0,
                handbrake_friction: try!(self.read_u32()) as f32 / 65536.0,
                footbrake_friction: try!(self.read_u32()) as f32 / 65536.0,
                front_brake_bias: try!(self.read_u32()) as f32 / 65536.0,
                turn_ratio: try!(self.read_i16()),

                drive_wheel_offset: try!(self.read_i16()),
                steering_wheel_offset: try!(self.read_i16()),

                back_end_slide_value: try!(self.read_u32()) as f32 / 65536.0,
                handbrake_slide_value: try!(self.read_u32()) as f32 / 65536.0,

                convertible: try!(self.read_u8()) == 1,
            
                engine: try!(self.read_u8()),
                radio: try!(self.read_u8()),
                horn: try!(self.read_u8()),

                sound_function: try!(self.read_u8()),
                fast_change_flag: try!(self.read_u8()),

                doors: try!(read_doors(self.f))
            })
        }

        Ok(cars)
    }

    fn read_sprite_info(&mut self) -> IoResult<Vec<SpriteInfo>> {
        let offset =  self.header_size + self.face_size + 
            self.h.anim_size + self.h.clut_size +
            self.h.palette_index_size + self.h.object_info_size +
            self.h.car_info_size;
        try!(self.seek(offset));
        assert!(try!(self.tell()) == offset);

        let max_offset = try!(self.tell()) + self.h.sprite_info_size;
        let mut sprites = Vec::new();

        while try!(self.tell()) < max_offset {
            let width = try!(self.read_u8());
            let height = try!(self.read_u8());

            let mut num_deltas = try!(self.read_u16());
            if num_deltas > 32 { 
                num_deltas = 32; 
            }

            // Skip scaling flag
            try!(self.read_u8());

            let size = try!(self.read_u16());
            let clut = try!(self.read_u16());
            let x = try!(self.read_u8());
            let y = try!(self.read_u8());

            let page = try!(self.read_u16());

            let mut deltas = Vec::with_capacity(num_deltas as uint);
            for i in range(0, num_deltas) {
                deltas.push(Delta {
                    size: try!(self.read_u16()),
                    w: try!(self.read_u32())
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

    fn read_sprite_numbers(&mut self) -> IoResult<SpriteNumbers> {
        let offset =  self.header_size + self.face_size + 
            self.h.anim_size + self.h.clut_size +
            self.h.palette_index_size + self.h.object_info_size +
            self.h.car_info_size + self.h.sprite_info_size +
            self.h.sprite_graphics_size;
        //try!(self.seek(offset));
        assert!(try!(self.tell()) == offset);

        Ok(SpriteNumbers {
            arrow: try!(self.read_u16()),
            digits: try!(self.read_u16()),
            boat: try!(self.read_u16()),
            case: try!(self.read_u16()),
            bus: try!(self.read_u16()),
            car: try!(self.read_u16()),
            object: try!(self.read_u16()),
            pedestrian: try!(self.read_u16()),
            speedo: try!(self.read_u16()),
            tank: try!(self.read_u16()),
            traffic_lights: try!(self.read_u16()),
            train: try!(self.read_u16()),
            trdoors: try!(self.read_u16()),
            bike: try!(self.read_u16()),
            tram: try!(self.read_u16()),
            wrecked_bus: try!(self.read_u16()),
            wrecked_car: try!(self.read_u16()),
            ex: try!(self.read_u16()),
            tumcar: try!(self.read_u16()),
            tumtruck: try!(self.read_u16()),
            ferry: try!(self.read_u16())
        })
    }
}

impl<'a> StyleReader<'a> {
    /// Returns the current offset of the file.
    fn tell(&self) -> IoResult<uint> {
        Ok(try!(self.f.tell()) as uint)
    }

    /// Seeks to the file `offset`.
    fn seek(&mut self, offset: uint) -> IoResult<()> {
        Ok(try!(self.f.seek(offset as i64, SeekSet)))
    }

    fn read_u8(&mut self) -> IoResult<u8> {
        Ok(try!(self.f.read_u8()))
    }

    fn read_i8(&mut self) -> IoResult<i8> {
        Ok(try!(self.f.read_i8()))
    }

    fn read_u16(&mut self) -> IoResult<u16> {
        Ok(try!(self.f.read_le_u16()))
    }

    fn read_i16(&mut self) -> IoResult<i16> {
        Ok(try!(self.f.read_le_i16()))
    }

    fn read_u32(&mut self) -> IoResult<u32> {
        Ok(try!(self.f.read_le_u32()))
    }

    fn read_i32(&mut self) -> IoResult<i32> {
        Ok(try!(self.f.read_le_i32()))
    }

    fn read_f32(&mut self) -> IoResult<f32> {
        Ok(try!(self.f.read_le_f32()))
    }
}
