//E2-80-2023 Aleksa Papovic
pub mod app;
pub mod read;
pub mod save;
pub mod algorithm;

use std::{ collections::HashMap, fmt::format };
use rayon::prelude::*;
use read::{ read_images, read_parts, read_originals };
use save::{ append_part };
use image::{ io::Reader as ImageReader, DynamicImage, GenericImageView, RgbaImage };
use algorithm::{ mean_square_error };

fn main() {
    // rayon::ThreadPoolBuilder::new().num_threads(4).build_global().unwrap();
    let redni_broj_slike = 3;
    let parts = "3";
    let map: HashMap<i32, Vec<String>> = HashMap::from([
        (1, vec!["1".to_string(), "1 - 1".to_string()]),
        (2, vec!["2".to_string(), "2 -1".to_string()]),
        (3, vec!["3".to_string()]),
        (4, vec!["4".to_string()]),
        (5, vec!["5".to_string()]),
    ]);
    let originalne_slike = read_originals(String::from("src/examples"));
    originalne_slike
        .par_iter()
        .enumerate()
        .for_each(|(indeks, img)| {
            let key = (indeks + 1) as i32;
            for (ip, part_path) in map.get(&key).iter().enumerate() {
                for deo in part_path.iter() {
                    create_image(&img, &deo);
                }
            }
        });
}

fn create_image(img: &DynamicImage, deo: &String) {
    let parts = read_images(format!("src/examples/slika {deo}"));
    // println!("parts {},height{}", parts.1, parts.2);

    let mut new_img = RgbaImage::new(img.width(), img.height());
    let mut removed: Vec<usize> = Vec::new();
    for (_, part) in parts.0.iter().enumerate() {
        let mut original_parts: Vec<RgbaImage> = read_parts(&mut img.clone(), parts.1, parts.2);
        let mut mse_values: Vec<f64> = vec![];
        for original_part in original_parts.iter() {
            let mse = mean_square_error(&original_part, part);
            mse_values.push(mse);
        }
        // println!("mse{:?}", mse_values);
        let min_indeks = find_min(&mse_values, &removed);

        let index = match min_indeks {
            Some(value) => {
                removed.push(value);
                println!("min_indeks: {:?} ", value);
                append_part(&mut img.clone(), &original_parts[value], value, &mut new_img, &deo);
            }
            None => {}
        };
        // append_part(&mut empty, &mut img.clone(), &part, index, &mut new_img);
    }
}

fn find_min(array: &Vec<f64>, exclude: &[usize]) -> Option<usize> {
    let mut min_val = f64::INFINITY;
    let mut min_index = None;
    for (index, &value) in array.iter().enumerate() {
        if !exclude.contains(&index) && value < min_val {
            min_val = value;
            min_index = Some(index);
        }
    }

    min_index
}
