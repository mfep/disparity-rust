extern crate png;
use png::HasParameters;
use std::fs::File;
use std::io::BufWriter;

pub struct Pixels {
    pub width: usize,
    pub height: usize,
    pub data: Vec<f32>,
}

impl Pixels {
    fn new(width: usize, height: usize) -> Pixels {
        Pixels {
            width,
            height,
            data: vec![0.0; width*height]
        }
    }

    fn new_with_data(width: usize, height: usize, data: Vec<f32>) -> Pixels {
        Pixels {
            width,
            height,
            data
        }
    }
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
