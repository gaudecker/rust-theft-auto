#![feature(phase)]

extern crate piston;
extern crate sdl2_game_window;
extern crate gfx;
extern crate device;
extern crate render;
extern crate image;

use sdl2_game_window::WindowSDL2;
use gfx::{Device, DeviceHelper};
use piston::{cam, Window};

use map::{Map, block, block_data};
use renderer::{Renderer, Texture, Vertex, Params, _ParamsLink};
use renderer::program::Program;
use renderer::buffer::Buffer;
use chunk::Chunk;

mod map;
mod renderer;
mod chunk;

fn main() {
    let mut window = WindowSDL2::new(
        piston::shader_version::opengl::OpenGL_3_2, 
        piston::WindowSettings {
            title: "gta".to_string(),
            size: [1920, 1080],
            fullscreen: true,
            exit_on_esc: true,
            samples: 4
        }
    );
    window.capture_cursor(true);

    let map = match Map::from_file("data/nyc.cmp") {
        Err(why) => fail!("Could not load map: {}", why.desc),
        Ok(map) => map
    };

    let (vertex_data, index_data) = match Chunk::from_map(&map, [0, 0]) {
        Some(chunk) => (chunk.verts, chunk.indices),
        None => fail!("Couldn't generate chunk from map!")
    };

    let (mut device, frame) = window.gfx();

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
        [128.0, 6.0, 128.0],
        first_person_settings
    );

    let mut game_iter = piston::EventIterator::new(
        &mut window,
        &piston::EventSettings {
            updates_per_second: 120,
            max_frames_per_second: 60
        }
    );

    let texture = Texture::from_file(&Path::new("data/texture.png"), &mut device).unwrap();
    let sam = device.create_sampler(gfx::tex::SamplerInfo::new(gfx::tex::Scale, gfx::tex::Tile));

    let mut renderer = Renderer::new(device, frame);

    let prog: Program = Program::new(&mut renderer, "shader");
    let buf: Buffer<Vertex, Params, _ParamsLink> = Buffer::new(
        &mut renderer,
        &prog,
        vertex_data.as_slice(),
        index_data.as_slice()
    );

    let mut data = renderer::Params {
        projection: projection,
        view: first_person.camera(0.0).orthogonal(),
        texture: (texture.handle, Some(sam))
    };
    
    for e in game_iter {
        match e {
            piston::Render(_args) => {
                data.view = first_person.camera(0.0).orthogonal();

                renderer.clear();
                renderer.render(buf, data);
                renderer.end_frame();
            },
            piston::Update(args) => first_person.update(args.dt),
            piston::Input(e) => first_person.input(&e),
        }
    }
}
