extern crate disparity;
extern crate time;

const WINDOW: i32 = 9;
const MAX_DISP: usize = 65;
const THREADS: usize = 4;

fn main() {
    let l_px = disparity::load_png_to_pixels("im0.png");
    let r_px = disparity::load_png_to_pixels("im1.png");
    let l_px = disparity::resize_pixels(&l_px, 4);
    let r_px = disparity::resize_pixels(&r_px, 4);

    let start_time = time::now();
    let depth = disparity::best_disp_map(l_px, r_px, WINDOW, MAX_DISP, THREADS);
    println!(
        "depth map calculation took {} us",
        (time::now() - start_time).num_microseconds().unwrap()
    );
    disparity::save_pixels_to_png(&depth, "depth.png");
}
