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
