mod utils;
mod mandelbrot;
mod pixelmapper;
mod grid;
mod control;

use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop, EventLoopBuilder};
use winit::window::WindowBuilder;
use winit::dpi::LogicalSize;

use std::num::NonZeroU32;
use std::thread;

use pixelmapper::PixelMapper;
use utils::*;

const STARTING_WIDTH: u32 = 1920;
const STARTING_HEIGHT: u32 = 1080;

const STARTING_WINDOW_WIDTH: u32 = 640;
const STARTING_WINDOW_HEIGHT: u32 = 360;
const STARTING_WINDOW_SIZE: LogicalSize<u32> = LogicalSize::new(STARTING_WINDOW_WIDTH, STARTING_WINDOW_HEIGHT);

fn main() {
    if let Some(_) = std::env::args().nth(1) {
        control::control_loop(None);
        return
    }

    let event_loop: EventLoop<control::ViewUpdate> = EventLoopBuilder::with_user_event().build();
    let proxy = event_loop.create_proxy();
    thread::spawn(move || control::control_loop(Some(proxy)));

    let window = WindowBuilder::new()
        .with_title("fractals!")
        .with_inner_size(STARTING_WINDOW_SIZE)
        .with_resizable(false)
        .build(&event_loop).unwrap();
    let context = unsafe { softbuffer::Context::new(&window) }.unwrap();
    let mut surface = unsafe { softbuffer::Surface::new(&context, &window) }.unwrap();
    surface
        .resize(
            NonZeroU32::new(STARTING_WINDOW_WIDTH).unwrap(),
            NonZeroU32::new(STARTING_WINDOW_HEIGHT).unwrap(),
        )
        .unwrap();

    let mut pm = PixelMapper::new_radx(Complex { real: -1.0, imag: 0.0 }, 1.0, 1.0, STARTING_WINDOW_WIDTH, STARTING_WINDOW_HEIGHT);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                //eprintln!("redraw requested");
                let width = window.inner_size().width;
                let height = window.inner_size().height;

                let mut buffer = surface.buffer_mut().unwrap();
                mandelbrot::draw_into_buffer(&pm, width as usize, height as usize, &mut buffer, 100);
                buffer.present().unwrap();
            }
            Event::UserEvent(e) => {
                //eprintln!("manual redraw");
                pm = e.pm;
                window.set_inner_size(LogicalSize::new(e.wi, e.hi));
                surface
                    .resize(
                        NonZeroU32::new(e.wi).unwrap(),
                        NonZeroU32::new(e.hi).unwrap(),
                    )
                    .unwrap();

                let mut buffer = surface.buffer_mut().unwrap();
                mandelbrot::draw_into_buffer(&pm, e.wi as usize, e.hi as usize, &mut buffer, 100);
                buffer.present().unwrap();
            }

            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => {
                *control_flow = ControlFlow::Exit;
            }
            _ => {}
        }
    });
}
