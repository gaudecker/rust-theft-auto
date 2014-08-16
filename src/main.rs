#![feature(globs)]

extern crate piston;
extern crate glfw_game_window;
extern crate gl;

use Window = glfw_game_window::GameWindowGLFW;
use piston::{
    GameIterator,
    GameIteratorSettings,
    GameWindowSettings,
    Render
};

use map::{Map};
use slopes = map::block_data::slope_verts;
use gfx::shader::{Shader, ShaderType};
use gfx::shader_program::{ShaderProgram};

mod map;
mod gfx;

fn main() {
    let ref mut window = Window::new(GameWindowSettings {
        title: "gta".to_string(),
        size: [960, 540],
        fullscreen: false,
        exit_on_esc: true
    });

    let map = match Map::from_file("data/nyc.cmp") {
        Err(why) => fail!("Could not load map: {}", why.desc),
        Ok(map) => map
    };

    let mut program = match ShaderProgram::link([Shader::from_file("data/shader.vert", gfx::shader::VertexShader),
                                             Shader::from_file("data/shader.frag", gfx::shader::FragmentShader)]) {
        Err(msg) => fail!("Could not link shader program: {}", msg),
        Ok(program) => program
    };
    program.bind();

    let mut game_iter = GameIterator::new(window, &GameIteratorSettings {
        updates_per_second: 120,
        max_frames_per_second: 60,
    });

    for e in game_iter {
        match e {
            Render(_args) => {
                gl::ClearColor(1.0, 1.0, 1.0, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);

                // for x in range(0, 256) {
                //     for y in range(0, 256) {
                //         let blocks = map.blocks[x][y];
                //         let height = blocks.len();

                //         let block = blocks[height - 1];
                //         let st = block.get_slope_type();
                        
                //         gl::Begin(gl::types::GL_LINE_STRIP);
                //         for j in range(0, 4) {
                //             gl::Vertex3f(slopes[st][2][j][0],
                //                          slopes[st][2][j][1],
                //                          slopes[st][2][j][2]);
                //         }
                //         gl::Vertex3f(slopes[st][2][0][0],
                //                      slopes[st][2][0][1],
                //                      slopes[st][2][0][2]);
                //         for j in range(0, 4) {
                //             gl::Vertex3f(slopes[st][1][j][0],
                //                          slopes[st][1][j][1],
                //                          slopes[st][1][j][2] - 1.001);
                //         }
                //         gl::Vertex3f(slopes[st][1][0][0],
                //                      slopes[st][1][0][1],
                //                      slopes[st][1][0][2] - 1.001);
                //         gl::End();
                //     }
                // }
            },
            _ => {},
        }
    }
}
