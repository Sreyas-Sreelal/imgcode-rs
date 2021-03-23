pub type Mask3x3 = [[i64; 3]; 3];

pub struct Filter {
    pub gx: Mask3x3,
    pub gy: Mask3x3,
}

impl Filter {
    /// 3x3 filter to perform convolution on image
    pub fn new(gx: Mask3x3, gy: Mask3x3) -> Self {
        Self { gx, gy }
    }
}

pub fn sobel_mask() -> Filter {
    let gx = [[-1, -2, -1], [0, 0, 0], [1, 2, 1]];
    let gy = [[-1, 0, 1], [-2, 0, 2], [-1, 0, 1]];

    Filter::new(gx, gy)
}
