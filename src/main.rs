use image::{Rgb, RgbImage};

fn f(x: u32, y: u32) -> Rgb<u8> {
    Rgb([x as u8, y as u8, 0])
}

fn main() {
    let img = RgbImage::from_fn(256, 256, f);
    img.save("out.png").unwrap();
}
