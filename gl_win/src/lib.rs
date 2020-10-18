/*
	Some windowing and event handling utilities.
*/
extern crate glutin;
use std::sync::Arc;

pub use glutin::event::{Event, WindowEvent};
pub use glutin::event_loop::ControlFlow;

pub type Window = glutin::ContextWrapper<glutin::PossiblyCurrent, glutin::window::Window>;
pub type EventLoop = glutin::event_loop::EventLoop<()>;

/// Initialize the GL context
/// and create a window and associated event loop.
pub fn init_gl_window(w: u32, h: u32, title: &str) -> (Arc<Window>, EventLoop) {
	let size = glutin::dpi::LogicalSize::new(w, h); // ?
	let event_loop = glutin::event_loop::EventLoop::new();
	let window = glutin::window::WindowBuilder::new() //
		.with_inner_size(size)
		.with_title(title)
		.with_resizable(false);
	let gl_window = glutin::ContextBuilder::new() //
		.with_vsync(true)
		.build_windowed(window, &event_loop)
		.unwrap();
	let gl_window = unsafe { gl_window.make_current() }.unwrap();
	gl::load_with(|symbol| gl_window.get_proc_address(symbol));
	(Arc::new(gl_window), event_loop)
}

/// Enter a continuous redraw loop, calling `draw` at VSync rate.
/// Function returns when window is closed.
pub fn redraw_loop<D>(gl_window: Arc<Window>, event_loop: EventLoop, draw: D)
where
	D: Fn(&Window) + 'static,
{
	event_loop.run(move |event, _, control_flow| {
		*control_flow = ControlFlow::Wait;
		match event {
			Event::RedrawRequested(_) => {
				draw(&gl_window);
				gl_window.swap_buffers().unwrap();
				gl_window.window().request_redraw(); //infinite redraw
			}
			Event::WindowEvent { event, .. } => match event {
				WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
				_ => (),
			},
			Event::LoopDestroyed => return,
			_ => (),
		}
	});
}

/// Enter an event handling loop until window is closed.
/// `draw` is called when a redraw is required.
/// `handle_event` is called on each window event.
pub fn run_event_loop<D, E>(gl_window: Arc<Window>, event_loop: EventLoop, draw: D, handle_event: E)
where
	D: Fn() + 'static,
	E: Fn(&Window, WindowEvent) + 'static,
{
	event_loop.run(move |event, _, control_flow| {
		*control_flow = ControlFlow::Wait;
		match event {
			Event::RedrawRequested(_) => {
				draw();
				gl_window.swap_buffers().unwrap();
			}
			Event::WindowEvent { event, .. } => match event {
				WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
				e => handle_event(&gl_window, e),
			},
			Event::LoopDestroyed => return,
			_ => (),
		}
	});
}
