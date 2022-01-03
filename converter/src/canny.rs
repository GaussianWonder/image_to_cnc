#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use image::*;
use imageproc::point::Point;
use serde::{Deserialize, Serialize};

// Serializable PixelIndex and its edge equivalents
#[derive(Serialize, Deserialize, Copy, Clone)]
pub struct SPixelIndex<T> {
    x: T,
    y: T,
}
pub type SEdge<T> = Vec<SPixelIndex<T>>;
pub type SEdges<T> = Vec<SEdge<T>>;

#[derive(Serialize, Deserialize, Clone)]
pub struct SerializebleComputation {
  pub edges: SEdges<usize>, // Vec<Vec<Point>>
  pub width: usize, // image width (max x)
  pub height: usize, // image height (max y)
  pub dx_skip: usize, // pixel skip on Ox
  pub dy_skip: usize, // pixel skip on Oy
  pub px_skip: usize, // pixel skip that's taken into account
}

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

pub fn get_edge_neighbors(point: PixelIndex, image: &DynamicImage) -> Vec<PixelIndex> {
  get_neighbors(point, image.width(), image.height())
    .into_iter()
    .filter(|&p| image.get_pixel(p.x as u32, p.y as u32)[0] > 128)
    .collect()
}

pub fn to_points(image: &DynamicImage, point_precision: f32) -> Edges<usize> {
  let width: u32 = (image.width() as f32 * point_precision).abs() as u32;
  let height: u32 = (image.height() as f32 * point_precision).abs() as u32;

  // number of pixels to skip in each direction
  // when dealing with curves, a weighted average between these two is performed
  // the weights are calculated based on the current and the previous accounted pixel
  let dx_skip = (image.width() as f32 / width as f32) as i32;
  let dy_skip = (image.height() as f32 / height as f32) as i32;
  let cost_skip = dx_skip + dy_skip;

  let mut edges = Edges::<usize>::new();
  let mut visited: Matrix<bool> = vec![vec![false; (image.width() + 2) as usize]; (image.height() + 2) as usize];

  // take each pixel from the image
  for (p_x, p_y, _pixel) in image.pixels() {
    let mut edge = Edge::<usize>::new();
    let i = p_x as usize;
    let j = p_y as usize;

    // if the current pixel is not visited
    if !visited[j][i] {
      visited[j][i] = true;
      let mut queue = vec![PixelIndex {x: i, y: j}];

      // TODO adjust this algorithm to perform DFS
      // TODO for each DFS new branch construct a new edge
      // perform fill on this edge
      while queue.len() > 0 {
        let current = queue.pop().unwrap(); // DFS
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
          // to keep point order correct
          edge.push(best_fit);
        }
      }

      // add the edge to the edges list only when not empty
      if edge.len() > 0 {
        edges.push(edge);
      }
    }
  }

  edges
}

pub fn to_serializable_points(image: &DynamicImage, point_precision: f32) -> SerializebleComputation {
  // explained in to_points()
  let width = (image.width() as f32 * point_precision).abs() as usize;
  let height = (image.height() as f32 * point_precision).abs() as usize;
  let dx_skip = (image.width() as f32 / width as f32) as usize;
  let dy_skip = (image.height() as f32 / height as f32) as usize;
  let cost_skip = dx_skip + dy_skip;

  SerializebleComputation {
    edges: to_points(image, point_precision)
      .into_iter()
      .map(|edge| edge.into_iter().map(|p| SPixelIndex { x: p.x, y: p.y }).collect())
      .collect(),
    width: image.width() as usize,
    height: image.height() as usize,
    dx_skip,
    dy_skip,
    px_skip: cost_skip,
  }
}

use imageproc::drawing::{draw_filled_circle_mut, draw_text_mut, draw_line_segment_mut};
use rand::Rng;
use rusttype::{Font, Scale};
use std::io::prelude::*;
use std::fs::{File};
use std::path::PathBuf;

pub struct MultiExport {
  pub path: PathBuf,
  pub input_name: String,
  pub extension: String,
}

fn rand_rgba() -> Rgba<u8> {
  let mut rng = rand::thread_rng();
  Rgba([rng.gen(), rng.gen(), rng.gen(), 255u8])
}

fn draw_edges(image: &mut DynamicImage, computation: &SerializebleComputation, radius: i32, per_edge_export_path: Option<&MultiExport>) {
  let font = Vec::from(include_bytes!("../assets/DejaVuSans.ttf") as &[u8]);
  let font = Font::try_from_vec(font).unwrap();
  let font_size = 12.0;
  let scale = Scale {
      x: font_size,
      y: font_size,
  };

  let original_copy = image.clone();
  let SerializebleComputation { edges, .. } = computation;

  for (edge_index, edge) in edges.iter().enumerate() {
    let mut edge_copy = original_copy.clone();

    let color = rand_rgba();
    for point_pair in edge.windows(2) {
      let p1 = point_pair[0];
      let p2 = point_pair[1];
      draw_line_segment_mut(
        image,
        (p1.x as f32, p1.y as f32),
        (p2.x as f32, p2.y as f32),
        color
      );
      draw_line_segment_mut(
        &mut edge_copy,
        (p1.x as f32, p1.y as f32),
        (p2.x as f32, p2.y as f32),
        color
      );
    }

    for point in edge {
      draw_filled_circle_mut(
        image,
        (point.x as i32, point.y as i32),
        radius,
        color,
      );
      draw_filled_circle_mut(
        &mut edge_copy,
        (point.x as i32, point.y as i32),
        radius,
        color,
      );
    }

    if let Some(export) = per_edge_export_path {
      for (index, point) in edge.iter().enumerate() {  
        if index % 10 == 0 {
          let mut inverted_color = original_copy.get_pixel(point.x as u32, point.y as u32);
          inverted_color[0] = 255 - inverted_color[0];
          inverted_color[1] = 255 - inverted_color[1];
          inverted_color[2] = 255 - inverted_color[2];

          draw_text_mut(
            &mut edge_copy,
            inverted_color,
            point.x as u32,
            point.y as u32,
            scale,
            &font,
            index.to_string().as_str(),
          );
        }
      }

      let export_path = export.path.join(format!("{}_{}_{}pt.{}", export.input_name, edge_index, edge.len(), export.extension));
      let file_creation = File::create(&export_path);
      if file_creation.is_ok() {
        let save_result = edge_copy.save(&export_path);
        if save_result.is_err() {
          println!("Failed to save {}", &export_path.into_os_string().into_string().unwrap());
        }
      }
      else {
        println!("Failed to create {}", &export_path.into_os_string().into_string().unwrap());
      }
    }
  }
}

pub fn draw_edges_on(image: &mut DynamicImage, computation: &SerializebleComputation) {
  draw_edges(image, computation, 2, None);
}

pub fn draw_each_edge_of(image: &mut DynamicImage, computation: &SerializebleComputation, export: &MultiExport) {
  draw_edges(image, computation, 2, Some(export));
}

pub fn copy_image<P, C>(image: &image::ImageBuffer<P, Vec<C>>) -> image::ImageBuffer<P, Vec<C>>
where
  P: image::Pixel<Subpixel = C> + 'static,
  C: 'static,
{
  let mut copy = image::ImageBuffer::new(image.width(), image.height());
  copy.copy_from(image, 0, 0).unwrap();
  copy
}
