mod args_parse;
mod canny;

use std::io::prelude::*;
use std::fs::{File};

#[cfg(feature = "display-window")]
fn main() {
    use imageproc::window::display_multiple_images;
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
    // let image = image::open(&config.input_file)
    //     .expect("No image found at input_file path")
    //     .grayscale()
    //     .to_luma8();

    let original = image::open(&config.input_file)
        .expect("No image found at input_file path")
        .to_rgb8();

    let gray_image = image::DynamicImage::ImageRgb8(canny::copy_image(&original))
        .grayscale()
        .to_luma8();

    let edges_image = image::DynamicImage::ImageLuma8(
        imageproc::edges::canny(&gray_image, config.low_threshold, config.high_threshold)
    );

    // if image export is enabled, write the edges image to the output file
    if config.export_options.image {
        let result = edges_image.save(
            config.export_path.join(format!("{}_edges.{}", config.input_name, config.input_extension)
        ));
        if result.is_err() {
            println!("Error saving image: {:?}", result.err());
        }
    }
    // TODO add option to skip the canny edge detection and use the image as-is

    // if point precision is enabled, convert the edges to a JSON file
    if let Some(point_precision) = config.export_options.point_precision {
        // convert points to json
        let points_json = serde_json::to_string(
            &canny::to_serializable_points(&edges_image, point_precision)
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
        let mut draw_image = image::DynamicImage::ImageRgb8(canny::copy_image(&original));
        let computation = canny::to_serializable_points(&edges_image, point_precision);

        canny::draw_each_edge_of(
            &mut draw_image,
            &computation,
            &canny::MultiExport {
                path: config.export_path.join("edges"),
                input_name: config.input_name.clone(),
                extension: config.input_extension.clone(),
            },
        );
        let saved_debug_preview = draw_image.save(
            config.export_path.join(format!("{}_debug_preview.{}", config.input_name, config.input_extension))
        );

        if saved_debug_preview.is_err() {
            println!("Error saving debug preview: {:?}", saved_debug_preview.err());
        }
    }
}
