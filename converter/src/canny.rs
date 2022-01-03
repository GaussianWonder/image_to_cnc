#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use serde::{Deserialize, Serialize};
use imageproc::point::Point;
use std::cmp::Ordering;

// Serializable PixelIndex and its edge equivalents
#[derive(Serialize, Deserialize)]
pub struct SPixelIndex<T> {
    x: T,
    y: T,
}
pub type SEdge<T> = Vec<SPixelIndex<T>>;
pub type SEdges<T> = Vec<SEdge<T>>;

// PixelIndex and edge equivalents, used to comply with the image crate
// TODO impl deserialize and serialize for imageproc::point::Point to replace duplicate types
pub type PixelIndex = Point<usize>;
type PixelRelativeIndex = Point<i32>;
pub type Edge<T> = Vec<Point<T>>;
pub type Edges<T> = Vec<Edge<T>>;

type Matrix<T> = Vec<Vec<T>>;

const RELATIVE_NEIGHBORS: [PixelRelativeIndex; 8] = [
  PixelRelativeIndex {x: -1, y: -1},
  PixelRelativeIndex {x: -1, y: 0},
  PixelRelativeIndex {x: -1, y: 1},
  PixelRelativeIndex {x: 0, y: -1},
  PixelRelativeIndex {x: 0, y: 1},
  PixelRelativeIndex {x: 1, y: -1},
  PixelRelativeIndex {x: 1, y: 0},
  PixelRelativeIndex {x: 1, y: 1},
];

pub fn get_neighbors(point: PixelIndex, rows: u32, cols: u32) -> Vec<PixelIndex> {
  let pt = PixelRelativeIndex {
    x: point.x as i32,
    y: point.y as i32,
  };
  RELATIVE_NEIGHBORS
    .to_vec()
    .into_iter()
    .map(|PixelRelativeIndex {x, y}| (pt.x + x, pt.y + y)) // index's neighbors
    .filter(|&(x, y)| x >= 0 && x < rows as i32 && y >= 0 && y < cols as i32) // inner bounded neighbors
    .map(|(x, y)| PixelIndex {x: x as usize, y: y as usize}) // convert back to index
    .collect()
}

pub fn get_edge_neighbors(point: PixelIndex, image: &image::GrayImage) -> Vec<PixelIndex> {
  get_neighbors(point, image.width(), image.height())
    .into_iter()
    .filter(|&p| image.get_pixel(p.x as u32, p.y as u32)[0] > 128)
    .collect()
}

fn cmp_points_clockwise(_a: PixelIndex, _b: PixelIndex, _center: PixelIndex) -> Ordering {
  let a = PixelRelativeIndex {
    x: _a.x as i32,
    y: _a.y as i32,
  };
  let b = PixelRelativeIndex {
    x: _b.x as i32,
    y: _b.y as i32,
  };
  let center = PixelRelativeIndex { // center of edge component
    x: _center.x as i32,
    y: _center.y as i32,
  };

  if a.x - center.x >= 0 && b.x - center.x < 0 {
    return Ordering::Less;
  }
  else {
    if a.x - center.x < 0 && b.x - center.x >= 0 {
      return Ordering::Greater;
    }
    else {
      if a.x - center.x == 0 && b.x - center.x == 0 {
        if a.y - center.y >= 0 || b.y - center.y >= 0 {
          return if a.y > b.y { Ordering::Less } else { Ordering::Greater };
        }
        else {
          return if b.y > a.y { Ordering::Less } else { Ordering::Greater };
        }
      }
      else {
        // compute the cross product of vectors (center -> a) x (center -> b)
        let det = (a.x - center.x) * (b.y - center.y) - (b.x - center.x) * (a.y - center.y);
        if det < 0 {
          return Ordering::Less;
        }
        else if det > 0 {
          return Ordering::Greater;
        }
        else {
          // points a and b are on the same line from the center
          // check which point is closer to the center
          let d1 = (a.x - center.x) * (a.x - center.x) + (a.y - center.y) * (a.y - center.y);
          let d2 = (b.x - center.x) * (b.x - center.x) + (b.y - center.y) * (b.y - center.y);
          return if d1 > d2 { Ordering::Less } else { Ordering::Greater };
        }
      }
    }
  }
}

pub fn to_points(image: &image::GrayImage, point_precision: f32) -> Edges<usize> {
  let width: u32 = (image.width() as f32 * point_precision) as u32;
  let height: u32 = (image.height() as f32 * point_precision) as u32;

  // let width_diff = image.width() - width;
  // let height_diff = image.height() - height;

  // number of pixels to skip in each direction
  // when dealing with curves, a weighted average between these two is performed
  // the weights are calculated based on the current and the previous accounted pixel
  let dx_skip = (image.width() as f32 / width as f32) as i32;
  let dy_skip = (image.height() as f32 / height as f32) as i32;
  let cost_skip = dx_skip + dy_skip;

  let mut edges = Edges::<usize>::new();
  let mut visited: Matrix<bool> = vec![vec![false; (image.width() + 2) as usize]; (image.height() + 2) as usize];

  // take each pixel from the image
  for (p_x, p_y, _pixel) in image.enumerate_pixels() {
    let mut edge = Edge::<usize>::new();
    let i = p_x as usize;
    let j = p_y as usize;

    // if the current pixel is not visited
    if !visited[j][i] {
      visited[j][i] = true;
      let mut queue = vec![PixelIndex {x: i, y: j}];

      // perform fill on this edge
      while queue.len() > 0 {
        let current = queue.remove(0);
        // current unvisited neighbors bounded by the image and part of an edge
        let neighbors = get_edge_neighbors(current, image)
          .into_iter()
          .filter(|&p| !visited[p.y][p.x])
          .collect::<Vec<PixelIndex>>();

        let mut chosen = false; // determine if the point (0, 0) was an active choise 
        let mut min_cost: i32 = -1;
        let mut best_fit = PixelIndex {x: 0, y: 0};
        // take each neighbor of the given candidate pixel
        for neighbor in neighbors {
          // if it is not visited, we can check the precision condition
          if !visited[neighbor.y][neighbor.x] {
            visited[neighbor.y][neighbor.x] = true;
            queue.push(neighbor); // push it in the candidates either way

            // check precision condition and find best matching point
            let mut is_precision_condition_met = true;
            let mut cost: i32 = -1;
            for selection in edge.iter() {
              // no need to perform x and y swap because all calculations are performed on PixelIndex
              let x_diff = (neighbor.x as i32 - selection.x as i32).abs();
              let y_diff = (neighbor.y as i32 - selection.y as i32).abs();
              let _cost = x_diff + y_diff;

              if _cost < cost_skip {
                is_precision_condition_met = false;
                break;
              }

              if cost == -1 || _cost < cost {
                cost = _cost;
              }
            }

            // if valid, add it to the edge
            if is_precision_condition_met {
              if min_cost == -1 || cost < min_cost {
                chosen = true;
                min_cost = cost;
                best_fit = neighbor;
              }
            }
          }
        }
        // now that all neighbors are checked, and we have a best fit, add it to the edge
        if chosen {
          edge.push(best_fit);
        }
      }

      // add the edge to the edges list only when not empty
      if edge.len() > 0 {
        let mut center = PixelIndex {x: 0, y: 0};
        for point in edge.iter() {
          center.x += point.x;
          center.y += point.y;
        }
        center.x /= edge.len();
        center.y /= edge.len();

        edge.sort_by(|a, b| cmp_points_clockwise(*a, *b, center));
        edges.push(edge);
      }
    }
  }

  edges
}

pub fn to_serializable_points(image: &image::GrayImage, point_precision: f32) -> SEdges<usize> {
  to_points(image, point_precision)
    .into_iter()
    .map(|edge| edge.into_iter().map(|p| SPixelIndex { x: p.x, y: p.y }).collect())
    .collect()
}

use image::{Rgb};
use rand::Rng;
use imageproc::drawing::{draw_filled_circle_mut, draw_text_mut, draw_line_segment_mut};
use rusttype::{Font, Scale};

pub fn rand_rgb() -> Rgb<u8> {
  let mut rng = rand::thread_rng();
  Rgb([rng.gen(), rng.gen(), rng.gen()])
}

pub fn rand_rgb_vec(len: usize) -> Vec<Rgb<u8>> {
  (0..len).map(|_| rand_rgb()).collect()
}

pub fn draw_edges_on<I>(image: &mut I, edges: &Edges<usize>, radius: i32, palette: &Vec<I::Pixel>)
where
  I: image::GenericImage,
  I::Pixel: 'static,
  // <<I as image::GenericImageView>::Pixel as image::Pixel>::Subpixel: imageproc::definitions::Clamp<f32>,
{
  for edge in edges {
    let color = palette[edge.len() % palette.len()];
    for point_pair in edge.windows(2) {
      let p1 = point_pair[0];
      let p2 = point_pair[1];
      draw_line_segment_mut(
        image,
        (p1.x as f32, p1.y as f32),
        (p2.x as f32, p2.y as f32),
        color
      );
    }

    // let font = Vec::from(include_bytes!("../assets/DejaVuSans.ttf") as &[u8]);
    // let font = Font::try_from_vec(font).unwrap();
    // let font_size = 12.4;
    // let scale = Scale {
    //     x: font_size,
    //     y: font_size,
    // };

    for (_index, point) in edge.iter().enumerate() {
      draw_filled_circle_mut(
        image,
        (point.x as i32, point.y as i32),
        radius,
        color,
      );

      // draw_text_mut(
      //   image,
      //   image.get_pixel(point.x as u32, point.y as u32),
      //   point.x as u32,
      //   point.y as u32,
      //   scale,
      //   &font,
      //   index.to_string().as_str(),
      // );
    }
  }
}
