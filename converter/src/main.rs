mod args_parse;
mod canny;

#[cfg(feature = "display-window")]
fn main() {
    let config = args_parse::get();

    let image = image::open(config.input_file)
        .expect("No image found at provided path")
        .grayscale()
        .to_luma8();

    let edges = imageproc::edges::canny(&image, config.low_threshold, config.high_threshold);

    display_multiple_images("", &vec![&image, &edges], image.width() as u32, image.height() as u32);
}

#[cfg(not(feature = "display-window"))]
fn main() {
    let config = args_parse::get();

    let image = image::open(config.input_file)
        .expect("No image found at provided path")
        .grayscale()
        .to_luma8();

    let edges = imageproc::edges::canny(&image, config.low_threshold, config.high_threshold);

    let result = image::save_buffer(
        config.export_path.join(format!("{}_edges.{}", config.input_name, config.input_extension)),
        &edges,
        edges.width() as u32,
        edges.height() as u32,
        image::ColorType::L8,
    );

    if result.is_err() {
        println!("Error saving image: {:?}", result.err());
    }
}
