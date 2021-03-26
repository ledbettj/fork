use std::time::Duration;
use num_complex::Complex;

struct Point {
  iterations: usize,
  z: Complex<f32>,
  c: Complex<f32>
}

impl Point {
  const LIMIT : f32 = 100.0;

  pub fn new(real: f32, imaginary: f32) -> Point {
    let c = Complex::new(real, imaginary);
    let z = Complex::new(0.0, 0.0);

    Point { c, z, iterations: 0 }
  }

  pub fn step(&mut self) {
    if !self.is_escaped() {
      self.z = self.z * self.z + self.c;
      self.iterations += 1;
    }
  }

  pub fn is_escaped(&self) -> bool {
    self.z.norm() > Point::LIMIT
  }

  pub fn count(&self) -> usize {
    self.iterations
  }
}

pub struct Fractal {
  step: usize,
  data: Vec<Point>,
  elapsed: Duration,
  x_range: (f32, f32),
  y_range: (f32, f32),
  width: usize,
  height: usize
}


impl Fractal {
  const WIN_MIN_X : f32 = -2.25;
  const WIN_MAX_X : f32 =  0.75;
  const WIN_MIN_Y : f32 = -1.25;
  const WIN_MAX_Y : f32 =  1.75;

  pub fn new(width: usize, height: usize) -> Fractal {
    let x_range = (Fractal::WIN_MIN_X, Fractal::WIN_MAX_X);
    let y_range = (Fractal::WIN_MIN_Y, Fractal::WIN_MAX_Y);

    let data = vec![];
    let mut f = Fractal { width, height, data, x_range, y_range, step: 0, elapsed: Duration::from_millis(0) };
    f.reset_data();
    f
  }

  fn reset_data(&mut self) {
    let data = (0..(self.width * self.height))
      .map(|index| {
        let x = index % self.width;
        let y = index / self.width;
        let r = self.x_range.0 + (x as f32 / self.width as f32) * (self.x_range.1 - self.x_range.0);
        let i = self.y_range.0 + (y as f32 / self.width as f32) * (self.y_range.1 - self.y_range.0);
        Point::new(r, i)
      })
      .collect();

    self.step = 0;
    self.data = data;
  }

  pub fn scale_width(&self) -> f32 {
    self.x_range.1 - self.x_range.0
  }

  pub fn scale_height(&self) -> f32 {
    self.y_range.1 - self.y_range.0
  }

  pub fn zoom(&mut self) {
    let new_x = self.x_range.0 + self.scale_width() / 4.0;
    let new_y = self.y_range.0 + self.scale_height() / 4.0;

    let mag_x = self.scale_width() / 2.0;
    let mag_y = self.scale_height() / 2.0;

    self.x_range = (new_x, new_x + mag_x);
    self.y_range = (new_y, new_y + mag_y);

    self.reset_data();

  }

  pub fn shift(&mut self, x_off: f32, y_off: f32) {
    if x_off == 0.0 && y_off == 0.0 {
      return;
    }

    self.x_range = (self.x_range.0 + x_off, self.x_range.1 + x_off);
    self.y_range = (self.y_range.0 + y_off, self.y_range.1 + y_off);

    self.reset_data();
  }

  pub fn draw(&self, frame: &mut [u8]) {
    for (index, pixel) in frame.chunks_exact_mut(4).enumerate() {
      self.draw_point(pixel, index);
    }
  }

  fn draw_point(&self, pixel: &mut [u8], index: usize) {
    let data = &self.data[index];

    let color = if data.is_escaped() {
      let v = data.count() as f32 / self.step as f32 * 255.0;
      let n = v as u8;
      [n, 0, 0, 0xFF]
    } else {
      [0x00, 0x00, 0x00, 0xFF]
    };

    pixel.copy_from_slice(&color);
  }

  pub fn step(&mut self, dt: &Duration) {
    self.elapsed += *dt;
    let freq = Duration::from_millis(50);

    while self.elapsed > freq {
      self.elapsed -= freq;
      self.data.iter_mut().for_each(|item| item.step());
      self.step += 1;
    }
  }
}
