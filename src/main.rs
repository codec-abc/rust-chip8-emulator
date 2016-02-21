#[macro_use]

extern crate glium;
extern crate rand;
extern crate chrono;

pub mod chip8;

use glium::index::PrimitiveType;
use glium::{DisplayBuild, Surface};
use std::env;
use std::io::prelude::*;
use std::fs::File;
use std::io;

fn main()
{
    let mut buffer = Vec::new();

    if env::args().count() != 2
    {
        panic!("You should pass one and only one argument which is the path to the chip8 rom");
    }
    else
    {
        let mut file : File = File::open(env::args().nth(1).unwrap()).unwrap();
        file.read_to_end(&mut buffer).unwrap();
    }

    let display = glium::glutin::WindowBuilder::new().with_dimensions(640,320).with_title(String::from("Chip8 Emulator")).build_glium().unwrap();

    let vertex_buffer =
    {
        #[derive(Copy, Clone)]
        struct Vertex
        {
            position: [f32; 2],
            tex_coords: [f32; 2],
        }

        implement_vertex!(Vertex, position, tex_coords);

        glium::VertexBuffer::new(&display,
            &[
                Vertex { position: [-1.0, -1.0], tex_coords: [0.0, 0.0] },
                Vertex { position: [-1.0,  1.0], tex_coords: [0.0, 1.0] },
                Vertex { position: [ 1.0,  1.0], tex_coords: [1.0, 1.0] },
                Vertex { position: [ 1.0, -1.0], tex_coords: [1.0, 0.0] }
            ]
        ).unwrap()
    };

    let index_buffer = glium::IndexBuffer::new(&display, PrimitiveType::TriangleStrip, &[1 as u16, 2, 0, 3]).unwrap();
    let program = glium::Program::from_source(&display,

                "#version 140
                uniform mat4 matrix;
                in vec2 position;
                in vec2 tex_coords;
                out vec2 v_tex_coords;
                void main() {
                    gl_Position = matrix * vec4(position, 0.0, 1.0);
                    v_tex_coords = tex_coords;
                }",

                "#version 140
                uniform sampler2D tex;
                in vec2 v_tex_coords;
                out vec4 f_color;
                void main() {
                    f_color = texture(tex, v_tex_coords);
                }",
            None
    ).unwrap();

    let mut frame_count = 0;
    let mut chip8 = chip8::Chip8::new(&buffer);
    let mut iteration = 0;
    loop
    {
        let mut input = String::new();
        io::stdin().read_line(&mut input);
        chip8.run_one_cycle();
        iteration += 1;
        println!("iteration {} ", iteration);
        let image = glium::texture::RawImage2d::from_raw_rgba(chip8.get_video_buffer_as_rgba(), (chip8.screen_width(), chip8.screen_height()) );
        let opengl_texture = glium::texture::SrgbTexture2d ::new(&display, image).unwrap();

        let uniforms = uniform! {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0f32]
            ],
            tex: glium::uniforms::Sampler::new(&opengl_texture)
                        .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest)
        };

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 0.0);
        target.draw(&vertex_buffer, &index_buffer, &program, &uniforms, &Default::default()).unwrap();
        target.finish().unwrap();

        for ev in display.poll_events()
        {
            match ev
            {
                glium::glutin::Event::Closed => return,
                _ => ()
            }
        }

        frame_count = frame_count + 1;
    }
}
