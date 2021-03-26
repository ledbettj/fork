use std::time::Instant;

use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

const WIDTH : u32 = 400;
const HEIGHT : u32 = 300;

mod fractal;

use fractal::Fractal;

fn main() -> Result<(), Error> {
  let event_loop = EventLoop::new();
  let mut input = WinitInputHelper::new();
  let mut fractal = Fractal::new(WIDTH as usize, HEIGHT as usize);

  let window = {
    let size = LogicalSize::new(WIDTH * 3, HEIGHT * 3);
    WindowBuilder::new()
      .with_title("Forktal")
      .with_inner_size(size)
      .with_min_inner_size(size)
      .build(&event_loop)
      .expect("Failed to create window")
  };

  let mut pixels = {
    let size = window.inner_size();
    let texture = SurfaceTexture::new(size.width, size.height, &window);
    Pixels::new(WIDTH, HEIGHT, texture)?
  };

  let mut paused = false;
  let mut time = Instant::now();

  event_loop.run(move |event, _, cf| {
    if let Event::RedrawRequested(_) = &event {
      let frame = pixels.get_frame();

      fractal.draw(frame);

      if pixels.render().is_err() {
        *cf = ControlFlow::Exit;
        return;
      }
    }

    if input.update(&event) {
      if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
        *cf = ControlFlow::Exit;
        return;
      }
      if input.key_pressed(VirtualKeyCode::Space) {
        fractal.zoom();
      }

      let shift_x = if input.key_held(VirtualKeyCode::Right) {
        fractal.scale_width() / 10.0
      } else if input.key_held(VirtualKeyCode::Left) {
        -fractal.scale_width() / 10.0
      } else { 0.0 };

      let shift_y = if input.key_held(VirtualKeyCode::Up) {
        -fractal.scale_height() / 10.0
      } else if input.key_held(VirtualKeyCode::Down) {
        fractal.scale_height() / 10.0
      } else { 0.0 };

      fractal.shift(shift_x, shift_y);

      let now = Instant::now();
      let dt = now.duration_since(time);
      time = now;
      if !paused {
        fractal.step(&dt);
      }
      window.request_redraw();
    }
  });
}
