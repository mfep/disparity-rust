extern crate png;
use png::HasParameters;
use std::fs::File;
use std::io::BufWriter;
use std::cmp::{min, max};
type Range = std::ops::Range<i32>;

pub struct Pixels {
    width: usize,
    height: usize,
    data: Vec<f32>,
}

impl Pixels {
    fn new_with_data(width: usize, height: usize, data: Vec<f32>) -> Pixels {
        assert_eq!(data.len(), width*height);
        Pixels {
            width,
            height,
            data
        }
    }

    fn get(&self, x: i32, y: i32) -> f32 {
        let x = max(min(x, self.width as i32 - 1), 0);
        let y = max(min(y, self.height as i32 - 1), 0);
        self.data[x as usize + y as usize * self.width]
    }
}

fn window_ranges(cx: i32, cy: i32, w: i32) -> (Range, Range) {
    let d = w/2;
    (cx-d..cx+d+1, cy-d..cy+d+1)
}

fn mean_window(pixels: &Pixels, cx: i32, cy: i32, w: i32) -> f32 {
    let (xr, yr) = window_ranges(cx, cy, w);
    let mut sum = 0.0;
    for row in yr {
        for col in xr.clone() {
            sum += pixels.get(col, row)
        }
    }
    sum / (w*w) as f32
}

fn std_window(pixels: &Pixels, cx: i32, cy: i32, w: i32) -> f32 {
    let (xr, yr) = window_ranges(cx, cy, w);
    let mean = mean_window(&pixels, cx, cy, w);
    let mut sum = 0.0;
    for row in yr {
        for col in xr.clone() {
            let val = pixels.get(col, row);
            sum += (val - mean)*(val - mean);
        }
    }
    sum.sqrt()
}

fn zncc_window(l_pix: &Pixels, r_pix: &Pixels, cx: i32, cy: i32, w: i32, disp: i32, l_mean: f32) -> f32 {
    let (xr, yr) = window_ranges(cx, cy, w);
    let r_mean = mean_window(&r_pix, cx, cy, w);
    let mut sum = 0.0;
    for row in yr {
        for col in xr.clone() {
            sum += (l_pix.get(col, row) - l_mean)*(r_pix.get(col - disp, row) - r_mean);
        }
    }
    sum / std_window(&l_pix, cx, cy, w) / std_window(&r_pix, cx - disp, cy, w)
}

fn best_zncc(l_pix: &Pixels, r_pix: &Pixels, cx: i32, cy: i32, w: i32, max_d: usize) -> usize {
    let l_mean = mean_window(&l_pix, cx, cy, w);
    let mut best_zncc = 0.0;
    let mut best_disp = 0;
    for disp in 0..max_d {
        let zncc = zncc_window(&l_pix, &r_pix, cx, cy, w, disp as i32, l_mean);
        if zncc > best_zncc {
            best_zncc = zncc;
            best_disp = disp;
        }
    }
    best_disp
}

pub fn mean_filter(pixels: &Pixels, w: i32) -> Pixels {
    let mut new_data = vec![0.0; pixels.data.len()];
    for row in 0..pixels.height as i32 {
        for col in 0..pixels.width as i32 {
            let idx = col + row*pixels.width as i32;
            new_data[idx as usize] = mean_window(pixels, col, row, w);
        }
    }
    Pixels::new_with_data(pixels.width, pixels.height, new_data)
}

pub fn std_filter(pixels: &Pixels, w: i32) -> Pixels {
    let mut new_data = vec![0.0; pixels.data.len()];
    for row in 0..pixels.height as i32 {
        for col in 0..pixels.width as i32 {
            let idx = col + row*pixels.width as i32;
            new_data[idx as usize] = std_window(pixels, col, row, w);
        }
    }
    Pixels::new_with_data(pixels.width, pixels.height, new_data)
}

pub fn best_disp_map(l_pix: &Pixels, r_pix: &Pixels, w: i32, max_disp: usize) -> Pixels {
    assert_eq!(l_pix.width, r_pix.width);
    assert_eq!(l_pix.height, r_pix.height);
    let mut new_data = vec![0.0; l_pix.data.len()];
    for row in 0..l_pix.height as i32 {
        for col in 0..l_pix.width as i32 {
            let idx = col + row*l_pix.width as i32;
            let best_disp = best_zncc(&l_pix, &r_pix, col, row, w, max_disp);
            new_data[idx as usize] = best_disp as f32 / max_disp as f32;
        }
    }
    Pixels::new_with_data(l_pix.width, l_pix.height, new_data)
}

pub fn load_png_to_pixels(png_path: &str) -> Pixels {
    let decoder = png::Decoder::new(File::open(png_path).unwrap());
    let (info, mut reader) = decoder.read_info().unwrap();
    let mut buf = vec![0; info.buffer_size()];
    reader.next_frame(&mut buf).unwrap();

    let mut data = vec![0.0; (info.width * info.height) as usize];

    for index in 0..data.len() {
        let red = buf[index*3] as f32 / 255.0;
        let green = buf[index*3 + 1] as f32 / 255.0;
        let blue = buf[index*3 + 2] as f32 / 255.0;
        data[index] = 0.2126*red + 0.7152*green + 0.0722*blue;
    }

    Pixels::new_with_data(info.width as usize, info.height as usize, data)
}

pub fn save_pixels_to_png(pixels: &Pixels, png_path: &str) {
    let file = File::create(png_path).unwrap();
    let ref mut w = BufWriter::new(file);
    let mut encoder = png::Encoder::new(w, pixels.width as u32, pixels.height as u32);
    encoder.set(png::ColorType::Grayscale).set(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    let mut data = vec![0; pixels.data.len()];
    for index in 0..data.len() {
        data[index] = (pixels.data[index] * 255.0) as u8;
    }
    writer.write_image_data(&data).unwrap();
}
