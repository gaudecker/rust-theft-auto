use std::vec::Vec;

use renderer::Vertex;
use map;
use map::Map;
use map::block;

static X: uint = 0;
static Y: uint = 1;
static MAX_WIDTH: uint = 256;
static MAX_HEIGHT: uint = 256;
static CHUNK_SIZE: uint = 255;

pub struct Chunk {
    pub pos: [uint, ..2],
    pub verts: Vec<Vertex>
}

impl Chunk {
    pub fn from_map(map: &Map, offset: [uint, ..2]) -> Option<Chunk> {
        assert!(offset[X] % CHUNK_SIZE == 0);
        assert!(offset[Y] % CHUNK_SIZE == 0);

        if !offset_in_range(offset) {
            return None;
        }

        let mut v = Vec::with_capacity(CHUNK_SIZE * CHUNK_SIZE * 36);
        for x in range(offset[X], offset[X] + CHUNK_SIZE) {
            for y in range(offset[Y], offset[Y] + CHUNK_SIZE) {
                let h = map.blocks[x][y].len();
                for z in range(0, h) {
                    let block = map.blocks[x][y][z];

                    // No need to draw air
                    if block.get_block_type() == block::Air {
                        continue;
                    }
                    
                    let block_verts = map::block_data::from_block(
                        block,
                        [x as f32, y as f32, z as f32]
                    );
                    v.push_all(block_verts.as_slice());
                }
            }
        }

        Some(Chunk {
            pos: offset,
            verts: v
        })
    }
}

fn offset_in_range(offset: [uint, ..2]) -> bool {
    offset[X] + CHUNK_SIZE < MAX_WIDTH && offset[X] >= 0 &&
        offset[Y] + CHUNK_SIZE < MAX_HEIGHT && offset[Y] >= 0
}
