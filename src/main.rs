use game::tile::{Tile, draw_tiles, Entity};
use glium::glutin::{self, dpi::{Size, PhysicalSize}};
use glium::Surface;

pub mod vertex;
pub mod utils;
pub mod drawable;
pub mod game;

extern crate glium;

fn main() {
    #[allow(unused_mut)]
    let mut event_loop = glutin::event_loop::EventLoop::new();
    let mut wb = glutin::window::WindowBuilder::new();
    wb.window.resizable = false;
    wb.window.title = String::from("Snake with rust");
    wb.window.inner_size = Some(Size::Physical(PhysicalSize::new(800, 800)));
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let vertex_shader_src = utils::read_file("shaders/default.vert");
    let fragment_shader_src = utils::read_file("shaders/default.frag");
    let shader_program = glium::Program::from_source(&display, &vertex_shader_src, &fragment_shader_src, None).unwrap();

    let mut tiles:Vec<Vec<Tile>> = Vec::new();

    for x in 0..10 {
        let mut temp_vec: Vec<Tile> = Vec::new();
        for y in 0..10 {
            temp_vec.push(Tile::new(x, y, Entity::NoEntity));
        }
        tiles.push(temp_vec);
    }

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
            let tile_drawables = draw_tiles(&display, &tiles);
            for tile_drawable in tile_drawables.iter() {
                target.draw(tile_drawable.get_vb(), tile_drawable.get_eb(), &shader_program, &glium::uniforms::EmptyUniforms, &Default::default()).unwrap();
            }
            target.finish().unwrap();
        }
    });//main loop
}