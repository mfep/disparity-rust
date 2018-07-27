extern crate disparity;

fn main() {
    let px = disparity::load_png_to_pixels("im0.png");
    disparity::save_pixels_to_png(&px, "out.png");
}
