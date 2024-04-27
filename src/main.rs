use std::sync::Arc;

use wgpu::SurfaceError;
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{EventLoop, EventLoopWindowTarget};
use winit::window::WindowBuilder;

mod gfx_context;
use gfx_context::Renderer;

fn handle_window_event(
    win_event: WindowEvent,
    event_loop: &EventLoopWindowTarget<()>,
    renderer: &mut Renderer,
) {
    match win_event {
        // Close
        WindowEvent::CloseRequested => {
            event_loop.exit();
        }
        // Resize
        WindowEvent::Resized(new_size) => {
            renderer.resize(new_size);
        }
        WindowEvent::ScaleFactorChanged {
            inner_size_writer, ..
        } => {
            todo!()
        }
        WindowEvent::RedrawRequested => {
            match renderer.render() {
                Ok(_) => {}
                // Reconfigure the surface if lost
                Err(SurfaceError::Lost) => renderer.resize(renderer.size),
                // The system is out of memory, we should probably quit
                Err(SurfaceError::OutOfMemory) => event_loop.exit(),
                // All other errors (Outdated, Timeout) should be resolved by the next frame
                Err(e) => eprintln!("{:?}", e),
            }
        }

        _ => {}
    }
}

fn main() {
    // Initialize logger
    env_logger::init();
    // Create window
    let event_loop = EventLoop::new().unwrap();
    let window = Arc::new(
        WindowBuilder::new()
            .with_title("miv")
            .with_inner_size(LogicalSize::new(800, 600))
            .with_resizable(true)
            .build(&event_loop)
            .unwrap(),
    );

    // Create renderer
    let mut renderer = pollster::block_on(Renderer::new(&window));

    event_loop
        .run(|event, event_loop| match event {
            Event::WindowEvent { event, .. } => {
                handle_window_event(event, &event_loop, &mut renderer)
            }
            _ => {}
        })
        .unwrap();
}
