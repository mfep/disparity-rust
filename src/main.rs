extern crate disparity;
extern crate time;

const WINDOW: i32 = 9;
const MAX_DISP: usize = 65;

fn main() {
    let l_px = disparity::load_png_to_pixels("im0.png");
    let r_px = disparity::load_png_to_pixels("im1.png");
    let mean = disparity::mean_filter(&l_px, WINDOW);
    let std = disparity::std_filter(&l_px, WINDOW);
    let start_time = time::now();
    let depth = disparity::best_disp_map(&l_px, &r_px, WINDOW, MAX_DISP);
    println!("depth map calculation took {} us", (time::now() - start_time).num_microseconds().unwrap());
    disparity::save_pixels_to_png(&l_px, "px.png");
    disparity::save_pixels_to_png(&mean, "mean.png");
    disparity::save_pixels_to_png(&std, "std.png");
    disparity::save_pixels_to_png(&depth, "depth.png");
}
