use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::{Window, WindowAttributes},
};
fn main() {
     // Create an event loop
    let event_loop = EventLoop::new().unwrap();
    // Create a window with specified attributes
    let window = event_loop
        .create_window( WindowAttributes::default()
            .with_title("next try")
            .with_inner_size(winit::dpi::LogicalSize::new(800.0, 600.0)),
        ).unwrap();
    // Run the event loop
    event_loop.run(move|event, elwt|{
        match event {
            Event::WindowEvent { event, ..} => match event {
                WindowEvent::CloseRequested => elwt.exit(),

                WindowEvent::CloseRequested => {

                }
                _ => {}
            },
            Event::AboutToWait => {
                // Perform any necessary updates before waiting for the next event
                window.request_redraw();
            }
            _ => {}
        }
    }).unwrap();
}