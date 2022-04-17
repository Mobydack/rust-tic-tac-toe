use winit::event_loop::ControlFlow;

pub fn handler(control_flow: &mut ControlFlow) {
    *control_flow = ControlFlow::Exit
}
