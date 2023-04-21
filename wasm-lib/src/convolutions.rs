pub struct matrix<T> {
    shape: u32,
    data: Vec<T>,
}

impl<T> matrix<T> {
    pub fn new(data: Vec<T>) -> Self {
        Self {
            shape: (data.len() as f32).sqrt().ceil() as u32,
            data: data,
        }
    }

    pub fn apply(self, pixels: Vec<u8>, width: i32, height: i32) -> Vec<u8> {
        let groups: Vec<Vec<u8>> = pixels.chunks_exact(4).map(|chunk| chunk.to_vec()).collect();
        let mut processed: Vec<u8> = Vec::new();

        for y in 0..height {
            for x in 0..width {
                let pos = x * y;
                let r = groups[pos as usize][0];
                let g = groups[pos as usize][1];
                let b = groups[pos as usize][2];

            }
        }

        pixels
    }
}
