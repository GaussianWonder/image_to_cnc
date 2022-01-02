mod args_parse;
mod canny;

use std::io::prelude::*;
use std::fs::{File};

#[cfg(feature = "display-window")]
fn main() {
    let config = args_parse::get();

    let image = image::open(config.input_file)
        .expect("No image found at input_file path")
        .grayscale()
        .to_luma8();

    let edges = imageproc::edges::canny(&image, config.low_threshold, config.high_threshold);

    display_multiple_images("", &vec![&image, &edges], image.width() as u32, image.height() as u32);
}

#[cfg(not(feature = "display-window"))]
fn main() {
    // parse arguments
    let config = args_parse::get();

    // construct the edges image from the grayscaled input
    let image = image::open(&config.input_file)
        .expect("No image found at input_file path")
        .grayscale()
        .to_luma8();

    let edges = imageproc::edges::canny(&image, config.low_threshold, config.high_threshold);

    // if image export is enabled, write the edges image to the output file
    if config.export_options.image {
        let result = edges.save(config.export_path.join(format!("{}_edges.{}", config.input_name, config.input_extension)));
        if result.is_err() {
            println!("Error saving image: {:?}", result.err());
        }
    }

    // if point precision is enabled, convert the edges to a JSON file
    if let Some(point_precision) = config.export_options.point_precision {
        // convert points to json
        let points_json = serde_json::to_string(
            &canny::to_serializable_points(&edges, point_precision)
        ).unwrap();
        // save points to file
        let handle = File::create(config.export_path.join(format!("{}_points.json", config.input_name)));
        match handle {
            Ok(mut file) => {
                let saved_points = file.write_all(points_json.as_bytes());
                if saved_points.is_err() {
                    println!("Error saving points: {:?}", saved_points.err());
                }
            },
            Err(e) => {
                println!("Error creating json file: {:?}", e);
            }
        }
    }

    // if debug point precision is enabled, draw the edges on the input image and save it
    if let Some(point_precision) = config.export_options.debug_preview {
        let mut original = image::open(&config.input_file)
            .expect("No image found at input_file path")
            .to_rgb8();
        let points = canny::to_points(&edges, point_precision);
        // println!("{:#?}", points);

        canny::draw_edges_on(&mut original, &points, 1, &canny::rand_rgb_vec(20));
        let saved_debug_preview = original.save(
            config.export_path.join(format!("{}_debug_preview.{}", config.input_name, config.input_extension))
        );

        if saved_debug_preview.is_err() {
            println!("Error saving debug preview: {:?}", saved_debug_preview.err());
        }
    }
}
