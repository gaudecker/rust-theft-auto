#![feature(globs)]

extern crate piston;
extern crate sdl2_game_window;
extern crate glfw_game_window;
extern crate gl;
extern crate vecmath;

use Window = sdl2_game_window::GameWindowSDL2;//glfw_game_window::GameWindowGLFW;
use piston::{
    GameIterator,
    GameIteratorSettings,
    GameWindowSettings,
    KeyPress,
    KeyRelease,
    MouseRelativeMove,
    Render,
    Update
};
use piston::shader_version::opengl::{OpenGL_3_2};
use vecmath::{Vector3};

use std::mem::size_of;

use map::{Map};
use map::block_data::from_slope_type;
use gfx::{VertexShader, FragmentShader, Shader, ShaderProgram, VertexBuffer, VertexArray};
use camera::{Camera, CameraSettings};
use camera_controller::{CameraController};

mod map;
mod gfx;
mod camera;
mod camera_controller;

fn main() {
    let ref mut window = Window::new(OpenGL_3_2, GameWindowSettings {
        title: "gta".to_string(),
        size: [960, 540],
        fullscreen: false,
        exit_on_esc: true
    });
    //window.capture_cursor(true);

    let map = match Map::from_file("data/nyc.cmp") {
        Err(why) => fail!("Could not load map: {}", why.desc),
        Ok(map) => map
    };

    let mut blocks = Vec::with_capacity(45);
    for n in range(0, 45) {
        blocks.push(from_slope_type(n));
    }

    let mut vao = VertexArray::new();
    vao.bind();

    let mut program = match ShaderProgram::link([Shader::from_file("data/shader.vert", VertexShader),
                                                 Shader::from_file("data/shader.frag", FragmentShader)]) {
        Err(msg) => fail!("Could not link shader program: {}", msg),
        Ok(program) => program
    };
    program.bind_fragment_output(0, "out_color").bind();
    program.set_uniform_matrix("projection", CameraSettings {
        fov: 90.0,
        near_clip: 0.0,
        far_clip: 256.0,
        aspect_ratio: 1.0
    }.projection());

    let vbo = VertexBuffer::from_data(blocks.as_slice());

    vao.define_attribute_data(&program, "pos", 3, 12 * size_of::<f32>() as i32, 0);
    //vao.define_attribute_data(&program, "col", 3, 5 * size_of::<f32>() as i32, 2 * size_of::<f32>());
    vbo.bind();

    let mut game_iter = GameIterator::new(window, &GameIteratorSettings {
        updates_per_second: 120,
        max_frames_per_second: 60,
    });

    let mut camera = Camera::new(0.0, 5.0, 0.0);
    let mut camera_controller = CameraController::new();
    camera.set_yaw_pitch(camera_controller.yaw, camera_controller.pitch);

    gl::Enable(gl::CULL_FACE);
    gl::CullFace(gl::BACK);

    for e in game_iter {
        match e {
            Render(_args) => {
                gl::ClearColor(0.0, 0.0, 0.0, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

                program.set_uniform_matrix("view", camera.orthogonal());

                for x in range(0, 256) {
                    for y in range(0, 256) {
                        let blocks = map.blocks.get(x).get(y);
                        let height = blocks.len();
                        
                        for z in range(0, height) {
                            let block = blocks[z];
                            let st = block.get_slope_type() as i32;

                            program.set_uniform_vector("model", [x as f32, y as f32, z as f32]);

                            if block.is_railway() {
                                vao.draw_array(gfx::LineStrip, st, st + 4);
                            }
                        }
                    }
                }
            },
            e => camera_controller.event(&e, &mut camera)
        }
    }
}
