use image::{DynamicImage, GenericImage, GrayImage, Luma, Rgba};

use crate::filter::Filter;

fn get_intensity(img: &GrayImage) -> Vec<[usize; 1]> {
    let mut temp: Vec<[usize; 1]> = Vec::new();
    for Luma(x) in img.pixels() {
        temp.push([x[0].into()]);
    }
    temp
}
pub fn calculate_threshold(gray_levels: &Vec<[usize; 1]>) -> usize {
    let mut previous =
        gray_levels.iter().fold(0, |total, next| total + next[0]) / gray_levels.len();
    let (mut r1, mut c1) = (0, 0);
    let (mut r2, mut c2) = (0, 0);

    loop {
        for x in gray_levels {
            if x[0] > previous {
                r1 += previous;
                c1 += 1;
            } else {
                r2 += previous;
                c2 += 1;
            }
        }
        r1 /= c1;
        r2 /= c2;
        let new = (r1 + r2) / 2;
        if new == previous {
            return new;
        }
        previous = new;
    }
}

fn filtering(filter: [[i64; 3]; 3], img: &GrayImage, x: u32, y: u32) -> i64 {
    let mut total = 0;
    let w = img.width();
    let h = img.height();
    // (x-1,y)
    if x >= 1 {
        total += filter[1][0] * img.get_pixel(x - 1, y)[0] as i64;
    }
    // (x+1,y)
    if x + 1 < w {
        total += filter[1][2] * img.get_pixel(x + 1, y)[0] as i64;
    }
    if y >= 1 {
        // (x,y-1)
        total += filter[0][1] * img.get_pixel(x, y - 1)[0] as i64;
        if x >= 1 {
            // (x-1,y-1)
            total += filter[0][0] * img.get_pixel(x - 1, y - 1)[0] as i64;
        }
        if x + 1 < w {
            // (x+1,y-1)
            total += filter[0][2] * img.get_pixel(x + 1, y - 1)[0] as i64;
        }
    }
    if y + 1 < h {
        // (x,y+1)
        total += filter[2][1] * img.get_pixel(x, y + 1)[0] as i64;
        if x >= 1 {
            // (x-1,y+1)
            total += filter[2][0] * img.get_pixel(x - 1, y + 1)[0] as i64;
        }
        if x + 1 < w {
            // (x+1,y+1)
            total += filter[2][2] * img.get_pixel(x + 1, y + 1)[0] as i64;
        }
    }
    total
}
pub fn convolution(filter: Filter, input: DynamicImage) -> DynamicImage {
    let img = input.into_luma8();
    let gray_levels = get_intensity(&img);
    let threshold = calculate_threshold(&gray_levels);
    let w = img.width();
    let h = img.height();
    let mut output = DynamicImage::new_rgba16(w, h);
    for y in 0..h {
        for x in 0..w {
            let gx = filtering(filter.gx, &img, x, y);
            let gy = filtering(filter.gy, &img, x, y);
            let m = (((gx * gx) + (gy * gy)) as f64).sqrt();
            if m > threshold as f64 {
                output.put_pixel(x, y, Rgba([255, 0, 0, 255]));
            } else {
                output.put_pixel(x, y, Rgba([255, 255, 255, 255]));
            }
        }
    }
    output
}
