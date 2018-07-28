extern crate disparity;
extern crate time;

fn main() {
    let window = 9;
    let max_disp = 65;
    let l_px = disparity::load_png_to_pixels("im0.png");
    let r_px = disparity::load_png_to_pixels("im1.png");
    let mean = disparity::mean_filter(&l_px, window);
    let std = disparity::std_filter(&l_px, window);
    let start_time = time::now();
    let depth = disparity::best_disp_map(&l_px, &r_px, window, max_disp);
    println!("depth map calculation took {} us", (time::now() - start_time).num_microseconds().unwrap());
    disparity::save_pixels_to_png(&l_px, "px.png");
    disparity::save_pixels_to_png(&mean, "mean.png");
    disparity::save_pixels_to_png(&std, "std.png");
    disparity::save_pixels_to_png(&depth, "depth.png");
}
