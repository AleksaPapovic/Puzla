use image::{ imageops::resize, RgbaImage };
use rayon::iter::{ IntoParallelIterator, ParallelIterator };
pub fn mean_square_error(img: &RgbaImage, img2: &RgbaImage) -> f64 {
    let (width1, height1) = img.dimensions();
    let (width2, height2) = img2.dimensions();

    let mut width = 0;
    let mut height = 0;
    if width1 < width2 {
        width = width1;
    } else {
        width = width2;
    }

    if height1 < height2 {
        height = height1;
    } else {
        height = height2;
    }
    let mut mse = 0.0;
    for y in 0..height {
        mse += (0..width)
            .into_par_iter()
            .map(move |x| {
                let p1 = img.get_pixel(x, y);
                let p2 = img2.get_pixel(x, y);
                (((p1[0] as f64) - (p2[0] as f64)).powi(2) +
                    ((p1[1] as f64) - (p2[1] as f64)).powi(2) +
                    ((p1[2] as f64) - (p2[2] as f64)).powi(2)) /
                    3.0
            })
            .sum::<f64>();
    }

    return mse / ((width * height) as f64);
}

pub fn find_max_with_index(numbers: &[f64]) -> (usize, f64) {
    let mut max_value = numbers[0];
    let mut max_index = 0;

    for (index, &value) in numbers.iter().enumerate().skip(1) {
        if value > max_value {
            max_value = value;
            max_index = index;
        }
    }
    return (max_index, max_value);
}
