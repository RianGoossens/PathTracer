use image::{Rgba, RgbaImage};

fn main() {
    let image = RgbaImage::from_fn(100, 100, |x, y| {
        if x % 10 == 0 || y % 10 == 0 {
            Rgba([0, 0, 0, 255])
        } else {
            Rgba([255, 255, 255, 255])
        }
    });
    image.save("image.png").expect("Failed to save image");
}
