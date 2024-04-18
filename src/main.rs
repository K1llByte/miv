use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let _window = WindowBuilder::new()
        .with_title("miv")
        .with_inner_size(LogicalSize::new(800, 600))
        .with_resizable(true)
        .build(&event_loop)
        .unwrap();

    event_loop
        .run(|event, event_loop| match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                event_loop.exit();
            }
            _ => {}
        })
        .unwrap();
}
