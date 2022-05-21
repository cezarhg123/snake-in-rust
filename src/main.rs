use glium::glutin::{self, dpi::{Size, PhysicalSize}};
use glium::Surface;
use vertex::Vertex;

pub mod vertex;
pub mod utils;

extern crate glium;

fn main() {
    #[allow(unused_mut)]
    let mut event_loop = glutin::event_loop::EventLoop::new();
    let mut wb = glutin::window::WindowBuilder::new();
    wb.window.resizable = false;
    wb.window.title = String::from("Snake with rust");
    wb.window.inner_size = Some(Size::Physical(PhysicalSize::new(1280, 720)));
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let vertex_shader_src = utils::read_file("shaders/default.vert");
    let fragment_shader_src = utils::read_file("shaders/default.frag");
    let shader_program = glium::Program::from_source(&display, &vertex_shader_src, &fragment_shader_src, None).unwrap();

    let square = 
    std::vec!
    [
        Vertex {
            position: (300, 300),
            color: (1.0, 0.0, 0.0)
        },

        Vertex {
            position: (300, 200),
            color: (0.0, 1.0, 0.0)
        },

        Vertex {
            position: (200, 200),
            color: (0.0, 0.0, 1.0)
        },

        Vertex {
            position: (200, 300),
            color: (1.0, 0.0, 1.0)
        }
    ];
    let vb = glium::VertexBuffer::new(&display, square.as_slice()).unwrap();
    let indices:[u32; 6] = [0, 2, 1, 0, 3, 2];
    let eb = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, &indices).unwrap();

    let mut prev_time = std::time::Instant::now();
    //main loop
    event_loop.run(move |ev, _, control_flow| {
        let crnt_time = std::time::Instant::now();
        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(10000);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
        match ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                _ => return,
            },
            _ => (),
        }

        //limits window refresh to 100 fps
        if crnt_time - prev_time >= std::time::Duration::from_millis(10) {
            prev_time = std::time::Instant::now();
            let mut target = display.draw();
            target.clear_color(0.0, 0.0, 0.0, 1.0);
            target.draw(&vb, &eb, &shader_program, &glium::uniforms::EmptyUniforms, &Default::default()).unwrap();
            target.finish().unwrap();
        }
    });//main loop
}