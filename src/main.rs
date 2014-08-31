#![feature(phase)]

extern crate piston;
extern crate sdl2_game_window;
extern crate gfx;
extern crate device;
extern crate render;

use sdl2_game_window::GameWindowSDL2 as Window;
use gfx::{Device, DeviceHelper};
use piston::{cam, GameWindow};

use map::{Map};
use renderer::{Renderer, Vertex, Params, _ParamsLink};
use renderer::program::Program;
use renderer::buffer::Buffer;
use chunk::Chunk;

mod map;
mod renderer;
mod chunk;

fn main() {
    let mut window = Window::new(
        piston::shader_version::opengl::OpenGL_3_2, 
        piston::GameWindowSettings {
            title: "gta".to_string(),
            size: [960, 540],
            fullscreen: false,
            exit_on_esc: true
        }
    );
    window.capture_cursor(true);

    let map = match Map::from_file("data/nyc.cmp") {
        Err(why) => fail!("Could not load map: {}", why.desc),
        Ok(map) => map
    };

    let vertex_data = match Chunk::from_map(&map, [0, 0]) {
        Some(verts) => verts.verts,
        None => fail!("Couldn't generate chunk from map!")
    };

//     let mesh = device.create_mesh(vertex_data);
//     let index_data: Vec<u8> = vec![
//         0, 1, 2, 2, 3, 0, // top
//         4, 5, 6, 6, 7, 4, // bottom
//         8, 9, 10, 10, 11, 8, // right
//         12, 13, 14, 14, 16, 12, // left
//         16, 17, 18, 18, 19, 16, // front
//         20, 21, 22, 22, 23, 20, // back
//     ];
    
    let (mut device, frame) = window.gfx();
    let mut renderer = Renderer::new(device, frame);

    let prog: Program = Program::new(&mut renderer, "shader");
    let buf: Buffer<Vertex, Params, _ParamsLink> = Buffer::new(
        &mut renderer,
        &prog,
        vertex_data.as_slice()
    );

    let projection = cam::CameraPerspective {
        fov: 90.0f32,
        near_clip: 0.1,
        far_clip: 1000.0,
        aspect_ratio: {
            let (w, h) = window.get_size();
            (w as f32) / (h as f32)
        }
    }.projection();

    let mut first_person_settings = cam::FirstPersonSettings::keyboard_wasd();
    first_person_settings.speed_horizontal = 12.0;
    first_person_settings.speed_vertical = 6.0;
    let mut first_person = cam::FirstPerson::new(
        [0.5f32, 0.5, 4.0],
        first_person_settings
    );

    let mut game_iter = piston::GameIterator::new(
        &mut window,
        &piston::GameIteratorSettings {
            updates_per_second: 120,
            max_frames_per_second: 60
        }
    );
    
    for e in game_iter {
        match e {
            piston::Render(_args) => {
                let data = renderer::Params {
                    projection: projection,
                    view: first_person.camera(0.0).orthogonal()
                };

                renderer.clear();
                renderer.render(buf, data);
                renderer.end_frame();
            },
            piston::Update(args) => first_person.update(args.dt),
            piston::Input(e) => first_person.input(&e),
        }
    }
}
