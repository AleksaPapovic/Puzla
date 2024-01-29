//E2-80-2023 Aleksa Papovic
pub mod app;
pub mod model;
pub mod read;
pub mod save;
pub mod algorithm;

use read::{ read_images, read_parts };
use save::{ append_part };
use image::{ io::Reader as ImageReader, DynamicImage, GenericImageView, RgbaImage };
use algorithm::{ mean_square_error };

fn main() {
    let redni_broj_slike = 3;
    let parts = "3";

    let putnja_slike = format!("src/examples/picture{redni_broj_slike}.jpg");
    let putanja_delova = format!("src/examples/slika {parts}");

    let mut img = ImageReader::open(putnja_slike).unwrap().decode().unwrap();

    create_image(&img, putanja_delova);
}

fn create_image(img: &DynamicImage, putanja_delova: String) {
    // let mut empty = vec![0; (img.width() * img.height()*9) as usize];
    // let result: RgbImage = ImageBuffer::from_raw(
    //     img.width() as u32,
    //     img.height() as u32,
    //     empty.clone()
    // ).expect("error");
    // result.save("result.jpg").unwrap();

    let parts = read_images(putanja_delova);
    // println!("parts {},height{}", parts.1, parts.2);

    let mut new_img = RgbaImage::new(img.width(), img.height());
    for (_, part) in parts.0.iter().enumerate() {
        let mut original_parts: Vec<RgbaImage> = read_parts(&mut img.clone(), parts.1, parts.2);
        let mut mse_values: Vec<f64> = vec![];
        for original_part in original_parts.iter() {
            let mse = mean_square_error(&original_part, part);
            mse_values.push(mse);
        }
        if
            let Some((index, _)) = mse_values
                .iter()
                .enumerate()
                .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        {
            // println!("Indeks: {}  min: {:?}", index, min);

            append_part(&mut img.clone(), &part, index, &mut new_img);
            // append_part(&mut empty, &mut img.clone(), &part, index, &mut new_img);

            original_parts.remove(index as usize);
        } else {
            println!("The vector is empty");
        }
    }
}
