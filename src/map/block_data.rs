use std::vec::Vec;

use super::super::renderer::{Vertex};
use super::block;
use super::block::{Block, BlockType};

/// Returns vertices matching the topology of `block` offset by `offset`.
pub fn from_block(block: Block, offset: [f32, ..3]) -> Vec<Vertex> {
    let t = block.get_slope_type() as f32;

    // Calculate the lid vertices based on slope type.
    let (y1, y2, y3, y4) = rotate(match t {
        1.0..8.0 => {
            ((((t - 1.0) % 2.0) + ord(t, [1.0, 2.0, 5.0, 6.0])) / 2.0,
             (((t - 1.0) % 2.0) + ord(t, [1.0, 2.0, 7.0, 8.0])) / 2.0,
             (((t - 1.0) % 2.0) + ord(t, [3.0, 4.0, 5.0, 6.0])) / 2.0,
             (((t - 1.0) % 2.0) + ord(t, [3.0, 5.0, 7.0, 8.0])) / 2.0)
        },
        9.0..40.0 => {
            ((((t - 9.0) % 8.0) + ord(t, [9.0, 16.0, 25.0, 32.0])) / 8.0,
             (((t - 9.0) % 8.0) + ord(t, [9.0, 16.0, 33.0, 40.0])) / 8.0,
             (((t - 9.0) % 8.0) + ord(t, [17.0, 24.0, 25.0, 32.0])) / 8.0,
             (((t - 9.0) % 8.0) + ord(t, [17.0, 24.0, 33.0, 40.0])) / 8.0)
        },
        41.0..44.0 => {
            (ord(t, [41.0, 43.0]),
             ord(t, [41.0, 44.0]),
             ord(t, [42.0, 43.0]),
             ord(t, [42.0, 44.0]))
        },
        _ => (1.0, 1.0, 1.0, 1.0)
    }, block.get_lid_rotation());

    let col = color_from_block_type(block.get_block_type());
    let (x, y, z) = (offset[0], offset[1], offset[2]);

    vec!(
        // front
        Vertex::new([x +  0.0, y +  0.0, z +  1.0], [0.0, 1.0], col),
        Vertex::new([x +  1.0, y +  0.0, z +  1.0], [1.0, 1.0], col),
        Vertex::new([x +  0.0, y +  y3,  z +  1.0], [0.0, 0.0], col),
        Vertex::new([x +  1.0, y +  y4,  z +  1.0], [1.0, 0.0], col),
        // back
        Vertex::new([x +  1.0, y +  0.0, z +  0.0], [0.0, 1.0], col),
        Vertex::new([x +  0.0, y +  0.0, z +  0.0], [1.0, 1.0], col),
        Vertex::new([x +  1.0, y +  y2,  z +  0.0], [0.0, 0.0], col),
        Vertex::new([x +  0.0, y +  y1,  z +  0.0], [1.0, 0.0], col),
        // right
        Vertex::new([x +  1.0, y +  0.0, z +  1.0], [0.0, 1.0], col),
        Vertex::new([x +  1.0, y +  0.0, z +  0.0], [1.0, 1.0], col),
        Vertex::new([x +  1.0, y +  y4,  z +  1.0], [0.0, 0.0], col),
        Vertex::new([x +  1.0, y +  y2,  z +  0.0], [1.0, 0.0], col),
        // left
        Vertex::new([x +  0.0, y +  0.0, z +  0.0], [0.0, 1.0], col),
        Vertex::new([x +  0.0, y +  0.0, z +  1.0], [1.0, 1.0], col),
        Vertex::new([x +  0.0, y +  y1,  z +  0.0], [0.0, 0.0], col),
        Vertex::new([x +  0.0, y +  y3,  z +  1.0], [1.0, 0.0], col),
        // top
        Vertex::new([x +  0.0, y +  y3,  z +  1.0], [0.0, 1.0], col),
        Vertex::new([x +  1.0, y +  y4,  z +  1.0], [1.0, 1.0], col),
        Vertex::new([x +  0.0, y +  y1,  z +  0.0], [0.0, 0.0], col),
        Vertex::new([x +  1.0, y +  y2,  z +  0.0], [1.0, 0.0], col),
    )
}

/// Returns a color vector based on `BlockType`.
pub fn color_from_block_type(block_type: BlockType) -> [f32, ..3] {
    match block_type {
        block::Water => [0.0, 0.0, 1.0],
        block::Road => [0.7, 0.7, 0.7],
        block::Pavement => [0.8, 0.8, 0.8],
        block::Field => [0.0, 1.0, 0.0],
        block::Building => [0.5, 0.5, 0.5],
        block::Air | block::Unused => [0.0, 0.0, 0.0]
    }
}

/// Rotates the lid corners by `rot` degrees.
fn rotate((y1, y2, y3, y4): (f32, f32, f32, f32), rot: u16) -> (f32, f32, f32, f32) {
    match rot {
        90 => (y3, y1, y2, y4),
        180 => (y4, y3, y1, y2),
        270 => (y2, y4, y3, y1),
        _ => (y1, y2, y3, y4)
    }
}

fn ord(n: f32, v: &[f32]) -> f32 {
    if (v.contains(&n)) { 1.0 } else { 0.0 }
}
