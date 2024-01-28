//E2-80-2023 Aleksa Papovic
pub mod app;
pub mod model;
pub mod read;
pub mod save;
pub mod algorithm;

use egui::Image;
use read::{ read_images, read_parts };
use save::{ append_part };
use image::{ io::Reader as ImageReader, DynamicImage, GenericImageView, ImageBuffer, RgbImage };
use algorithm::{ mean_square_error };

fn main() {
    let redni_broj_slike = 1;
    let parts = "1";

    let putnja_slike = format!("src/examples/picture{redni_broj_slike}.jpg");
    let putanja_delova = format!("src/examples/slika {parts}");

    let mut img = ImageReader::open(putnja_slike).unwrap().decode().unwrap();

    create_image(&img, putanja_delova);
}

fn create_image(img: &DynamicImage, putanja_delova: String) {
    let empty = vec![0; (img.width() * img.height()*9) as usize];
    let result: RgbImage = ImageBuffer::from_raw(
        img.width() as u32,
        img.height() as u32,
        empty
    ).expect("error");
    result.save("result.jpg").unwrap();

    let parts = read_images(putanja_delova);
    println!("parts {},height{}", parts.1, parts.2);
    for (index, part) in parts.0.iter().enumerate() {
        let mut original_parts: Vec<RgbImage> = read_parts(&mut img.clone(), parts.1, parts.2);
        let mut mse_values: Vec<f64> = vec![];
        for original_part in original_parts.iter() {
            let mse = mean_square_error(&original_part, part);
            mse_values.push(mse);
        }
        println!("mse {:?}", mse_values);
        if
            let Some((index, min)) = mse_values
                .iter()
                .enumerate()
                .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        {
            println!("Index of minimum MSE: {}", index);
            println!("Indeks: {}  min: {:?}", index, min);

            append_part(&mut img.clone(), &part, index);

            original_parts.remove(index as usize);
        } else {
            println!("The vector is empty");
        }

        // let diffs: Vec<f64> = compare(part, original_parts.clone(), part.width(), part.height());
        // println!("diffs {:?}", diffs);
        // let (indeks, val) = find_max_with_index(&diffs);
        // println!("indeks {},val {}", indeks, val);
        // append_part(&mut bytes.clone(), &mut img.clone(), part.clone(), 10); vrati
    }
}
