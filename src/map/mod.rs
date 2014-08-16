use std::collections::HashMap;
use std::vec::Vec;
use std::io::{File, IoResult};
use std::mem::size_of;

pub mod block;
pub mod block_data;
pub mod route;
pub mod object;
pub mod location;
pub mod zone;
pub mod position;

static header_size: u64 = 28;
static base_size: u64 = 262144;

pub struct Map {
    blocks: Vec<Vec<Vec<block::Block>>>,
    objects: Vec<object::Object>,
    routes: Vec<route::Route>,
    locations: HashMap<location::LocationType, Vec<location::Location>>,
    zones: Vec<zone::Zone>
}

impl Map {
    pub fn from_file(filename: &str) -> IoResult<Map> {
        println!("Loading map {}", filename);
        let mut f = match File::open(&Path::new(filename)) {
            Err(why) => fail!("could not read {}: {}", filename, why.desc),
            Ok(file) => file
        };
        let header = Header {
            version: try!(f.read_le_u32()),
            style: try!(f.read_u8()),
            sample: try!(f.read_u8()),

            reserved: try!(f.read_le_u16()),

            route_size: try!(f.read_le_u32()),
            object_size: try!(f.read_le_u32()),
            column_size: try!(f.read_le_u32()),
            block_size: try!(f.read_le_u32()),
            zone_size: try!(f.read_le_u32())
        };

        let base = try!(read_base(&mut f));
        let columns = try!(read_columns(header.column_size, &mut f));
        let blocks = try!(read_blocks(header.block_size, &mut f));
        let objects = try!(read_objects(header.object_size, &mut f));
        let routes = try!(read_routes(header.route_size, &mut f));
        let locations = try!(read_locations(&mut f));
        let zones = try!(read_zones(header.zone_size, &mut f));
        
        Ok(Map {
            blocks: uncompress(&base, &columns, &blocks),
            objects: objects,
            routes: routes,
            locations: locations,
            zones: zones
        })
    }

    pub fn get_locations(&self, location_type: &location::LocationType) -> &Vec<location::Location> {
        self.locations.get(location_type)
    }
}

struct Header {
    version: u32,
    style: u8,
    sample: u8,

    reserved: u16,

    route_size: u32,
    object_size: u32,
    column_size: u32,
    block_size: u32,
    zone_size: u32
}

fn read_base(f: &mut File) -> IoResult<[[u32, ..256], ..256]> {
    let mut base = [[0, ..256], ..256];
    for y in range(0, 256) {
        for x in range(0, 256) {
            base[x][y] = try!(f.read_le_u32());
        }
    }
    Ok(base)
}

fn read_columns(size: u32, f: &mut File) -> IoResult<Vec<u16>> {
    let capacity: uint = size as uint / 2;
    let mut columns = Vec::with_capacity(capacity);
    for n in range(0, capacity) {
        columns.push(try!(f.read_le_u16()));
    }
    Ok(columns)
}

fn read_blocks(size: u32, f: &mut File) -> IoResult<Vec<block::Block>> {
    let capacity: uint = size as uint / size_of::<block::Block>();
    let mut blocks = Vec::with_capacity(capacity);
    for n in range(0, capacity) {
        blocks.push(block::Block {
            type_map: try!(f.read_le_u16()),
            type_map_ext: try!(f.read_u8()),
            west: try!(f.read_u8()),
            east: try!(f.read_u8()),
            north: try!(f.read_u8()),
            south: try!(f.read_u8()),
            lid: try!(f.read_u8())
        });
    }
    Ok(blocks)
}

fn read_objects(size: u32, f: &mut File) -> IoResult<Vec<object::Object>> {
    let capacity = size as uint / size_of::<object::Object>();
    let mut objects = Vec::with_capacity(capacity);
    for n in range(0, capacity) {
        objects.push(object::Object {
            x: try!(f.read_le_u16()),
            y: try!(f.read_le_u16()),
            z: try!(f.read_le_u16()),

            object_type: try!(f.read_u8()),
            remap: try!(f.read_u8()),

            yaw: try!(f.read_le_u16()),
            pitch: try!(f.read_le_u16()),
            roll: try!(f.read_le_u16())
        });
    }
    Ok(objects)
}

fn read_routes(size: u32, f: &mut File) -> IoResult<Vec<route::Route>> {
    let max_offset = try!(f.tell()) + size as u64;
    let capacity = size as uint / size_of::<route::Route>();
    let mut routes = Vec::with_capacity(capacity);
    while try!(f.tell()) < max_offset {
        let count = try!(f.read_u8());
        let route_type = try!(f.read_u8());
        let mut points = Vec::with_capacity(count as uint);
        for n in range(0, count) {
            points.push(position::Position {
                x: try!(f.read_u8()),
                y: try!(f.read_u8()),
                z: try!(f.read_u8())
            });
        }

        routes.push(route::Route {
            route_type: route_type,
            points: points
        });
    }
    Ok(routes)
}

fn read_locations(f: &mut File) -> IoResult<HashMap<location::LocationType, Vec<location::Location>>> {
    let mut locations: HashMap<location::LocationType, Vec<location::Location>> = HashMap::new();
    locations.insert(location::PoliceStation, Vec::new());
    locations.insert(location::Hospital, Vec::new());
    locations.insert(location::FireStation, Vec::new());

    for n in range(0, 36u8) {
        let pos = position::Position {
            x: try!(f.read_u8()),
            y: try!(f.read_u8()),
            z: try!(f.read_u8())
        };

        let location_type = match n {
            0..5 => location::PoliceStation,
            6..11 => location::Hospital,
            24..29 => location::FireStation,
            _ => location::Unknown
        };

        if location_type == location::Unknown {
            continue;
        }

        locations.get_mut(&location_type).push(location::Location {
            location_type: location_type,
            position: pos
        });
    }
    Ok(locations)
}

fn read_zones(size: u32, f: &mut File) -> IoResult<Vec<zone::Zone>> {
    let max_offset = try!(f.tell()) + size as u64;
    let mut zones = Vec::new();
    while try!(f.tell()) < max_offset {
        let zone = zone::Zone {
            area: zone::Rect {
                x: try!(f.read_u8()),
                y: try!(f.read_u8()),
                width: try!(f.read_u8()),
                height: try!(f.read_u8())
            },
            sample: try!(f.read_u8()),
            name: String::from_utf8(try!(f.read_exact(30))).unwrap()
        };
        if (zone.area.width != 0 || zone.area.height != 0) {
            zones.push(zone);
        }
    }
    Ok(zones)
}

///
/// Uncompress block information to more easy to use format.
/// 
fn uncompress(base: &[[u32, ..256], ..256], cols: &Vec<u16>,
              blocks: &Vec<block::Block>) -> Vec<Vec<Vec<block::Block>>> {
    let mut x_row = Vec::with_capacity(256);
    for x in range(0, 256) {
        let mut y_row = Vec::with_capacity(256);
        for y in range(0, 256) {
            let height = 6 - cols[base[y][x] as uint / 2] as uint;
            let mut z_row = Vec::with_capacity(height);

            for z in range(0, height) {
                let mut block_index = 6 - cols[base[y][x] as uint / 2] as uint;
                if block_index > z {
                    block_index -= z;
                }
                block_index = cols[base[y][x] as uint / 2 + block_index] as uint;
                let block = blocks[block_index];
                z_row.push(block);
            }

            y_row.push(z_row);
        }
        x_row.push(y_row);
    }
    x_row
}
