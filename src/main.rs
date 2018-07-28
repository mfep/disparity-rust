extern crate disparity;

fn main() {
    let px = disparity::load_png_to_pixels("im0.png");
    let mean = disparity::mean_filter(&px, 9);
    disparity::save_pixels_to_png(&mean, "out.png");
}
