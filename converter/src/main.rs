
#[cfg(feature = "display-window")]
fn main() {
    use imageproc::window::display_multiple_images;
    use std::env;

    let image_path = match env::args().nth(1) {
        Some(path) => path,
        None => {
            println!("No image path provided. Using default image.");
            "assets/test.jpg".to_owned()
        }
    };

    let image = image::open(&image_path)
        .expect("No image found at provided path")
        .grayscale()
        .to_luma8();

    let edges = imageproc::edges::canny(&image, 50.0f32, 60.0f32);

    display_multiple_images("", &vec![&image, &edges], 500, 500);
}

#[cfg(not(feature = "display-window"))]
fn main() {
    use std::env;

    let image_path = match env::args().nth(1) {
        Some(path) => path,
        None => {
            println!("No image path provided. Using default image.");
            "assets/test.jpg".to_owned()
        }
    };

    let image = image::open(&image_path)
        .expect("No image found at provided path")
        .grayscale()
        .to_luma8();

    let edges = imageproc::edges::canny(&image, 50.0f32, 60.0f32);

    let result = image::save_buffer(
        &image_path.replace(".jpg", "_edges.jpg"),
        &edges,
        edges.width() as u32,
        edges.height() as u32,
        image::ColorType::L8,
    );

    if result.is_err() {
        println!("Error saving image: {:?}", result.err());
    }
}