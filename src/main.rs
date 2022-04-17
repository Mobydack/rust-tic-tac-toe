mod config;

use winit::{
    event::{Event, WindowEvent},
    window::WindowBuilder,
    event_loop::{EventLoop, ControlFlow},
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
            } => {
                *control_flow = ControlFlow::Exit
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            _ => ()
        }
    });
}
