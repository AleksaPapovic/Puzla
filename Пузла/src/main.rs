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
    let mut img = ImageReader::open("src/examples/picture1.jpg").unwrap().decode().unwrap();
    create_image(img);
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

fn create_image(img: DynamicImage) {
    let mut bytes = img.to_bytes();
    for i in 0..bytes.len() {
        bytes[i] = 0;
    }
    let putanja = String::from("src/examples/slika 1");
    let parts = read_images(putanja);
    let width = parts[0].width();
    let height: u32 = parts[0].height();
    let original_parts = read_parts(&mut img.clone(), width, height);
    for (index, part) in parts.iter().enumerate() {
        let diffs = compare(part, original_parts.clone(), width, height);
        println!("diffs {:?}", diffs);
        // append_part(&mut bytes.clone(), &mut img.clone(), part.clone());
    }
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
            images.push(part_img);
        }
    }
    return images;
}

fn compare(part: &DynamicImage, originals: Vec<DynamicImage>, width: u32, height: u32) -> Vec<f32> {
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
) -> f32 {
    let mut diff = 0u64;
    let mut count = 0;
    for y in 0..height {
        for x in 0..width {
            let pixel1 = img1.get_pixel(x, y);
            let pixel2 = img2.get_pixel(x, y);

            if
                !is_within_tolerance(pixel1[0], pixel2[0], tolerance.red) ||
                !is_within_tolerance(pixel1[1], pixel2[1], tolerance.green) ||
                !is_within_tolerance(pixel1[2], pixel2[2], tolerance.blue)
            {
                count += 1;
            }
        }
    }
    let total_pixels = (width * height) as u64;
    (count as f32) / (total_pixels as f32)
}

fn is_within_tolerance(value1: u8, value2: u8, tolerance: u8) -> bool {
    let diff = ((value1 as i16) - (value2 as i16)).abs() as u8;
    diff <= tolerance
}

fn append_part(bytes: &mut Vec<u8>, img: &mut DynamicImage, part_img: DynamicImage) {
    let ow = (img.width() * 3) as usize;
    let oh = (img.height() * 3) as usize;
    println!("Original width: {} height: {}", ow, oh);

    let partw = (part_img.width() * 3) as usize;
    let parth = (part_img.height() * 3) as usize;
    println!("Part width: {} height: {}", partw, parth);

    let x = ow / partw;
    let y = ((oh as f64) / (parth as f64)).round() as usize;
    println!("x: {} y: {}", x, y);

    let diff = ow % partw;
    let img1: RgbImage = ImageBuffer::from_raw(img.width(), img.height(), bytes.clone()).expect(
        "error"
    );
    let part_bytes = part_img.to_bytes();
    let mut start_index = 0;
    for j in 0..x {
        for i in 0..y {
            start_index = i * partw;
            let mut end_index = start_index + partw;
            println!("{}", start_index);
            println!("{}", end_index);
            // if(x ==0){

            for p in 0..parth / 3 {
                // println!("s{}",start_index);
                // println!("e{}",end_index);
                // println!("ps{}",p*partw);
                // println!("pe{}",p*partw +partw);
                let part_index = p * partw;
                bytes[start_index..end_index].copy_from_slice(
                    &part_bytes[part_index..part_index + partw]
                );
                start_index += ow;
                end_index = start_index + partw;
            }
            // }
        }
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
