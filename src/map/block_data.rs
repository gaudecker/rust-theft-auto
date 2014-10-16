use std::vec::Vec;
use std::iter::range_step;

use super::super::renderer::{Vertex};
use super::block;
use super::block::{Block, BlockType};

/// Returns vertices matching the topology of `block` offset by `offset`.
pub fn from_block(block: Block, offset: [f32, ..3], n: u32) -> (Vec<Vertex>, Vec<u32>) {
    let t = block.get_slope_type() as f32;

    // Calculate the lid vertices based on slope type.
    let (y1, y2, y3, y4) = rotate(match t {
        1.0 ... 8.0 => {
            ((((t - 1.0) % 2.0) + ord(t, [1.0, 2.0, 5.0, 6.0])) / 2.0,
             (((t - 1.0) % 2.0) + ord(t, [1.0, 2.0, 7.0, 8.0])) / 2.0,
             (((t - 1.0) % 2.0) + ord(t, [3.0, 4.0, 5.0, 6.0])) / 2.0,
             (((t - 1.0) % 2.0) + ord(t, [3.0, 5.0, 7.0, 8.0])) / 2.0)
        },
        9.0 ... 40.0 => {
            ((((t - 9.0) % 8.0) + ord(t, [9.0, 16.0, 25.0, 32.0])) / 8.0,
             (((t - 9.0) % 8.0) + ord(t, [9.0, 16.0, 33.0, 40.0])) / 8.0,
             (((t - 9.0) % 8.0) + ord(t, [17.0, 24.0, 25.0, 32.0])) / 8.0,
             (((t - 9.0) % 8.0) + ord(t, [17.0, 24.0, 33.0, 40.0])) / 8.0)
        },
        41.0 ... 44.0 => {
            (ord(t, [41.0, 43.0]),
             ord(t, [41.0, 44.0]),
             ord(t, [42.0, 43.0]),
             ord(t, [42.0, 44.0]))
        },
        _ => (1.0, 1.0, 1.0, 1.0)
    }, block.get_lid_rotation());

    let col = color_from_block_type(block.get_block_type());
    let (x, y, z) = (offset[0], offset[1], offset[2]);

    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    
    let tc = tex_coords(block.get_block_type());
    // top
    // if block.lid != 0 {
        vertices.push_all(
            vec!(Vertex::new([x +  0.0, y +  y3,  z +  1.0], tc[0], col),
                 Vertex::new([x +  1.0, y +  y4,  z +  1.0], tc[1], col),
                 Vertex::new([x +  0.0, y +  y1,  z +  0.0], tc[2], col),
                 Vertex::new([x +  1.0, y +  y2,  z +  0.0], tc[3], col)
            ).as_slice());
    // }

    // front
    // if block.south != 0 {
        vertices.push_all(
            vec!(Vertex::new([x +  0.0, y +  0.0, z +  1.0], tc[0], col),
                 Vertex::new([x +  1.0, y +  0.0, z +  1.0], tc[1], col),
                 Vertex::new([x +  0.0, y +  y3,  z +  1.0], tc[2], col),
                 Vertex::new([x +  1.0, y +  y4,  z +  1.0], tc[3], col),
            ).as_slice());
    // }

    // back
    // if block.north != 0 {
        vertices.push_all(
            vec!(Vertex::new([x +  1.0, y +  0.0, z +  0.0], tc[0], col),
                 Vertex::new([x +  0.0, y +  0.0, z +  0.0], tc[1], col),
                 Vertex::new([x +  1.0, y +  y2,  z +  0.0], tc[2], col),
                 Vertex::new([x +  0.0, y +  y1,  z +  0.0], tc[3], col),
                 ).as_slice());
    // }

    // right
    // if block.east != 0 {
        vertices.push_all(
            vec!(Vertex::new([x +  1.0, y +  0.0, z +  1.0], tc[0], col),
                 Vertex::new([x +  1.0, y +  0.0, z +  0.0], tc[1], col),
                 Vertex::new([x +  1.0, y +  y4,  z +  1.0], tc[2], col),
                 Vertex::new([x +  1.0, y +  y2,  z +  0.0], tc[3], col),
                 ).as_slice());
    // }

    // left
    // if block.west != 0 {
        vertices.push_all(
            vec!(Vertex::new([x +  0.0, y +  0.0, z +  0.0], tc[0], col),
                 Vertex::new([x +  0.0, y +  0.0, z +  1.0], tc[1], col),
                 Vertex::new([x +  0.0, y +  y1,  z +  0.0], tc[2], col),
                 Vertex::new([x +  0.0, y +  y3,  z +  1.0], tc[3], col)
                 ).as_slice());
    // }

    let num_indices = n + (vertices.len() as f32 * 1.5) as u32;
    for i in range_step(n, num_indices, 4) {
        indices.push_all(vec!(i + 0, i + 1, i + 2, i + 1, i + 2, i + 3).as_slice());
    }

    (vertices, indices)
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

pub fn tex_coords(block_type: BlockType) -> [[f32, ..2], ..4] {
    let none = [[0.0, 0.0], [0.0, 0.0], [0.0, 0.0], [0.0, 0.0]];
    match block_type {
        block::Water => [[0.0, 0.0],[0.33333, 0.0],[0.0, 0.33333],[0.33333, 0.33333]],
        block::Road => [[0.33333, 0.0],[0.66666, 0.0],[0.33333, 0.33333],[0.66666, 0.33333]],
        block::Pavement => [[0.66666, 0.0],[1.0, 0.0],[0.66666, 0.33333],[1.0, 0.33333]],
        block::Field => [[0.0, 0.33333], [0.33333, 0.33333], [0.0, 0.66666], [0.33333, 0.66666]],
        block::Building => [[0.33333, 0.33333], [0.66666, 0.33333], [0.33333, 0.66666], [0.66666, 0.66666]],
        block::Air | block::Unused => none
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
    if v.contains(&n) { 1.0 } else { 0.0 }
}
