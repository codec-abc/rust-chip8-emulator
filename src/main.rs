#[macro_use]

extern crate glium;
extern crate rand;

mod chip8;
use rand::*;
use glium::index::PrimitiveType;

fn main()
{
    use glium::{DisplayBuild, Surface};
    let display = glium::glutin::WindowBuilder::new().with_dimensions(640,320).with_title(String::from("Chip8 Emulator")).build_glium().unwrap();

    let vertex_buffer =
    {
        #[derive(Copy, Clone)]
        struct Vertex {
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

    let index_buffer = glium::IndexBuffer::new(&display, PrimitiveType::TriangleStrip,
                                               &[1 as u16, 2, 0, 3]).unwrap();

    // compiling shaders and linking them together
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
    let image_dimensions : (u32, u32) = (64 , 32);
    let (width, height) = image_dimensions;
    let size = width as usize *  height as usize * 4;
    let mut image_data : Vec<u8> = Vec::with_capacity( size );

    for _ in 0 .. size/4
    {
        let value = thread_rng().gen::<u8>();
        image_data.push(value);
        image_data.push(value);
        image_data.push(value);
        image_data.push(255);
    }

    loop
    {
        alter_video_buffer(&mut image_data, size);
        let image = glium::texture::RawImage2d::from_raw_rgba(image_data.clone(), image_dimensions);
        let opengl_texture = glium::texture::SrgbTexture2d ::new(&display, image).unwrap();

        let uniforms = uniform! {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0f32]
            ],
            tex: &opengl_texture,
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

fn alter_video_buffer(buffer : &mut Vec<u8>, size : usize)
{
    for i in 0 .. size/4
    {
        let value = thread_rng().gen::<u8>();
        buffer[i * 4 + 0]=value;
        buffer[i * 4 + 1]=value;
        buffer[i * 4 + 2]=value;
    }
}
