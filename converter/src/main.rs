
#[cfg(feature = "display-window")]
fn main() {
    use imageproc::window::display_multiple_images;
    use std::env;

    let image_path = match env::args().nth(1) {
        Some(path) => path,
        None => {
            println!("No image path provided. Using default image.");
            "assets/image1.jpg".to_owned()
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
    panic!("Displaying images is only supported if the display-window feature is enabled.");
}