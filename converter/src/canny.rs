use imageproc::point::Point;

pub fn to_points(image: &image::GrayImage, point_precision: f32) -> Vec<Point<u32>> {
  let mut points = Vec::new();
  // TODO fill here and change point_precision to number of pixels to skip

  for (x, y, pixel) in image.enumerate_pixels() {
    if pixel[0] == 255 {
      points.push(Point::new(x, y));
    }
  }

  points
}