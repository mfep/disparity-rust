extern crate disparity;

fn main() {
    let window = 9;
    let px = disparity::load_png_to_pixels("im0.png");
    let mean = disparity::mean_filter(&px, window);
    let std = disparity::std_filter(&px, window);
    disparity::save_pixels_to_png(&px, "px.png");
    disparity::save_pixels_to_png(&mean, "mean.png");
    disparity::save_pixels_to_png(&std, "std.png");
}
