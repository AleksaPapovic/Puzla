//E2-80-2023 Aleksa Papovic
use std::{
    borrow::{ Borrow, BorrowMut },
    fs::{ self, File },
    io::{ BufReader, Read, Seek, SeekFrom },
    path::Path,
};

use egui::CentralPanel;
use image::{
    io::Reader as ImageReader,
    DynamicImage,
    GenericImageView,
    ImageBuffer,
    ImageFormat,
    RgbImage,
};

fn main() {
    let redni_broj = 1;
    let putnja_slike = format!("src/examples/picture{redni_broj}.jpg");
    let putanja_delova = format!("src/examples/slika {redni_broj}");
    let mut img = ImageReader::open(putnja_slike).unwrap().decode().unwrap();
    create_image(img, putanja_delova);
}

fn read_images(path_str: String) -> Vec<DynamicImage> {
    let mut images = Vec::new();
    let path = Path::new(&path_str);
    if path.is_dir() {
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();

                    if path.is_file() {
                        let file = File::open(path).unwrap();
                        let mut reader = BufReader::new(file);
                        let mut start = [0; 8];
                        reader.read_exact(&mut start).unwrap();
                        let format = image::guess_format(&start).unwrap();
                        match format {
                            ImageFormat::Png => {
                                println!("png");
                                reader.seek(SeekFrom::Start(0)).unwrap();
                                let img = image::load(reader, format).unwrap();
                                images.push(img);
                            }
                            ImageFormat::Jpeg => {
                                println!("jpg");
                                reader.seek(SeekFrom::Start(0)).unwrap();
                                let img = image::load(reader, format).unwrap();
                                images.push(img);
                            }
                            _ => {}
                        }
                    }
                }
            }
        } else {
            println!("Could not read directory");
        }
    } else {
        println!("Path is not a directory");
    }
    return images;
}

fn create_image(img: DynamicImage, putanja_delova: String) {
    let mut bytes = img.to_bytes();
    for i in 0..bytes.len() {
        bytes[i] = 0;
    }

    let parts = read_images(putanja_delova);
    for (index, part) in parts.iter().enumerate() {
        let original_parts = read_parts(&mut img.clone(), part.width(), part.height());
        let diffs: Vec<f64> = compare(part, original_parts.clone(), part.width(), part.height());
        println!("diffs {:?}", diffs);
        let (indeks, val) = find_max_with_index(&diffs);
        println!("indeks {},val {}", indeks, val);
        // append_part(&mut bytes.clone(), &mut img.clone(), part.clone(), 10); vrati
    }
}

fn find_max_with_index(numbers: &[f64]) -> (usize, f64) {
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

fn read_parts(img: &mut DynamicImage, partw: u32, parth: u32) -> Vec<DynamicImage> {
    let mut images = Vec::new();
    let img_width = img.width();
    let img_height = img.height();
    let cols = img_width / partw;
    let rows = ((img_height as f64) / (parth as f64)).round() as u32;

    for y in 0..rows {
        for x in 0..cols {
            let top_left_x = x * partw;
            let top_left_y = y * parth;

            let part_img = img.crop_imm(top_left_x, top_left_y, partw, parth);
            let filename = format!("output{}.png", y * cols + x + 1);
            part_img.save(filename);
            // let mut novi_img = ImageReader::open(format!("output{}.png", y * cols + x + 1))
            //     .unwrap()
            //     .decode()
            //     .unwrap();
            images.push(part_img);
        }
    }
    return images;
}

fn compare(part: &DynamicImage, originals: Vec<DynamicImage>, width: u32, height: u32) -> Vec<f64> {
    let tolerance = Tolerance {
        red: 20,
        green: 20,
        blue: 20,
        min_brightness: 16,
        max_brightness: 240,
    };
    originals
        .iter()
        .map(|original| compare_images(&part, original, width, height, &tolerance))
        .collect()
}

fn compare_images(
    img1: &DynamicImage,
    img2: &DynamicImage,
    width: u32,
    height: u32,
    tolerance: &Tolerance
) -> f64 {
    println!("img1 w{}h{}", img1.width(), img1.height());
    // println!("img2 w{}h{}", img2.width(), img2.height());
    let mut mse = 0f64;
    let mut count = 0usize;
    for y in 0..img2.height() {
        for x in 0..img2.width() {
            let pixel1 = img1.to_rgb().to_vec();
            let pixel2 = img2.to_rgb().to_vec();

            for i in 0..3 {
                mse += (((pixel1[i] as f64) - (pixel2[i] as f64)) / 255.0).powi(2);
                count += 1;
            }
        }
    }

    let diff = mse / (count as f64);
    return diff;
}

fn append_part(bytes: &mut Vec<u8>, img: &mut DynamicImage, part_img: DynamicImage, indeks: usize) {
    let ow = (img.width() * 3) as usize;
    let oh = (img.height() * 3) as usize;
    // println!("Original width: {} height: {}", ow, oh);

    let partw = (part_img.width() * 3) as usize;
    let parth = (part_img.height() * 3) as usize;
    // println!("Part width: {} height: {}", partw, parth);

    let x = ow / partw;
    let y = ((oh as f64) / (parth as f64)).round() as usize;
    // println!("x: {} y: {}", x, y);

    let diff = ow % partw;
    let img1: RgbImage = ImageBuffer::from_raw(img.width(), img.height(), bytes.clone()).expect(
        "error"
    );
    let mut counter = 0;
    let part_bytes = part_img.to_bytes();
    let mut start_index = 0;
    for j in 0..x {
        for i in 0..y {
            // println!("Part i: {} j: {} counter: {}", i, j, counter);
            start_index = i * partw;
            let mut end_index = start_index + partw;
            // println!("{}", start_index);
            // println!("{}", end_index);
            if counter == 6 {
                for p in 0..parth / 3 {
                    // println!("s{}",start_index);
                    // println!("e{}",end_index);
                    // println!("ps{}",p*partw);
                    // println!("pe{}",p*partw +partw);
                    let part_index = p * partw;
                    let start = start_index + (((10 * partw) / 3) * parth) / 3;
                    let end = start + partw;
                    // println!("{},{}", start, end);
                    bytes[start..end].copy_from_slice(&part_bytes[part_index..part_index + partw]);
                    start_index += ow;
                    end_index = start_index + partw;
                }
            }
            counter += 1;
        }
        counter += 1;
    }

    let img: RgbImage = ImageBuffer::from_raw(
        img.width() as u32,
        img.height() as u32,
        bytes.clone()
    ).expect("test");
    img.save("oput.jpg").unwrap();
    // let w =  (partw/3) as u32;
    // let h =  (parth/3) as u32;

    // let mut new_img = RgbaImage::new((x * w).try_into().unwrap(), y * h);

    // let part_pixels = part_img.to_rgba8();

    // for i in 0..y {
    //     for j in 0..x {
    //         let start_x = i * w;
    //         let start_y = j * h;
    //         for part_y in 0..h {
    //             for part_x in 0..w {
    //                 let pixel = part_pixels.get_pixel(part_x, part_y);
    //                 new_img.put_pixel(start_x + part_x, start_y + part_y, *pixel);
    //             }
    //         }
    //     }
    // }

    // new_img.save("oput_pixelbypixel.jpg").unwrap();
}

struct MyApp {
    img_bytes: Vec<u8>,
}

impl MyApp {
    pub fn new(img_bytes: Vec<u8>) -> Self {
        Self { img_bytes }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.label(format!("Image Bytes: {:?}", self.img_bytes));
        });
    }
}

struct Tolerance {
    red: u8,
    green: u8,
    blue: u8,
    min_brightness: u8,
    max_brightness: u8,
}
