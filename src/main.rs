mod config;
mod event_handlers;

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() {
    let event_loop_instance = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title(config::WINDOW_TITLE)
        .with_inner_size(config::DEFAULT_INNER_SIZE)
        .build(&event_loop_instance)
        .unwrap();

    event_loop_instance.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => event_handlers::close_window::handler(control_flow),
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            _ => (),
        }
    });
}
