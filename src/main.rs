use glium::glutin::{self, dpi::{Size, PhysicalSize}};
use glium::Surface;

pub mod vertex;

extern crate glium;

fn main() {
    let mut event_loop = glutin::event_loop::EventLoop::new();
    let mut wb = glutin::window::WindowBuilder::new();
    wb.window.resizable = false;
    wb.window.title = String::from("Snake with rust");
    wb.window.inner_size = Some(Size::Physical(PhysicalSize::new(1280, 720)));
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let mut prev_time = std::time::Instant::now();

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
            target.clear_color(0.0, 0.6, 1.0, 1.0);
            target.finish().unwrap();
        }
    });
}