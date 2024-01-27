use std::{borrow::Borrow};
use image::{imageops::crop_imm, io::Reader as ImageReader, DynamicImage, EncodableLayout, GenericImage, GenericImageView, ImageBuffer, Pixel, Rgb, RgbImage, RgbaImage};
use egui::{CentralPanel, Window, Label};


fn main() {
    let mut img = ImageReader::open("src/examples/picture5.jpg")
        .unwrap()
        .decode()
        .unwrap();

    let img_bytes = img.to_bytes();
    create_image(img,&mut img_bytes.clone());

    // open_image(img_bytes);

}

fn open_image(img_bytes:Vec<u8>){
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Image Bytes Display",
        native_options,
        Box::new(|_cc| Box::new(MyApp::new(img_bytes))),
    );
}

fn create_image(img: DynamicImage,bytes: &mut Vec<u8>){
    for i in 0.. bytes.len() {
        bytes[i]= 0
    }
    let img2 = ImageReader::open("src/examples/slika 5/26.jpg")
    .unwrap()
    .decode()
    .unwrap();

    compare(&mut bytes.clone(),&mut img.clone(),&mut img2.clone());
    
    append_part(&mut bytes.clone(),&mut img.clone(),img2);
}

fn sum_of_bytes(bytes: &[u8]) -> u32 {
    bytes.iter().map(|&b| b as u32).sum()
}

fn compare(bytes: &mut Vec<u8>,img: &mut DynamicImage,part_img:  &mut DynamicImage){
    let img_width = img.width();
    let img_height = img.height();
    let partw = part_img.width();
    let parth = part_img.height();

    let mut part_number = 1;

    let cols = img_width / partw;
    let rows = ((img_height as f64) / (parth as f64)).round() as u32;

    for y in 0..rows {
        for x in 0..cols {
            let top_left_x = x * partw;
            let top_left_y = y * parth;

            let part_img = img.crop_imm(top_left_x, top_left_y, partw, parth);
            let filename = format!("output{}.png", y * cols + x + 1);

            part_img.save(filename);
        }
    }
}


fn append_part(bytes: &mut Vec<u8>,img: &mut DynamicImage, part_img: DynamicImage){
        let ow = (img.width()*3) as usize;
        let oh = (img.height()*3) as usize;
        println!("Original image size: {}x{}", ow, oh);
    
        let part_width = (part_img.width()*3) as usize;
        let part_height = (part_img.height()*3) as usize;
        println!("Part image size: {}x{}", part_width, part_height);
    
        let x_parts = ow / part_width;
        let y_parts = ((oh as f64) / (part_height as f64)).round() as usize;
        println!("Number of parts: {}x{}", x_parts, y_parts);
    
        let diff = ( ow %part_width);
        println!("diff {}",diff);
        let img1: RgbImage = ImageBuffer::from_raw(img.width(), img.height(), bytes.clone()).expect("test");
        let part_bytes = part_img.to_bytes();
        let mut start_index=0;
        for y in 0..x_parts {
            for x in 0..y_parts {
                start_index = (x * part_width);
                let mut end_index = start_index + part_width;
                println!("{}",start_index);
                println!("{}",end_index);
                if(x ==0){

                    for p in 0..part_height/3{
                        // println!("s{}",start_index);
                        // println!("e{}",end_index);
                        // println!("ps{}",p*part_width);
                        // println!("pe{}",p*part_width +part_width);
                        bytes[start_index..end_index].copy_from_slice(&part_bytes[p*part_width..(p*part_width +part_width)]);
                        start_index += ow;
                        end_index = start_index + part_width;
                    } 
                }
        }
    }
    
    let img: RgbImage = ImageBuffer::from_raw(img.width() as u32, img.height() as u32, bytes.clone()).expect("test");
    img.save("oput.jpg").unwrap();
    let parts_across = x_parts as u32;
    let parts_down = y_parts as u32;
    let w =  (part_width/3) as u32;
    let h =  (part_height/3) as u32;

    let mut new_img = RgbaImage::new(parts_across * w, parts_down * h);

    let part_pixels = part_img.to_rgba8();

    for y in 0..parts_down {
        for x in 0..parts_across {
            let start_x = x * w;
            let start_y = y * h;
            for part_y in 0..h {
                for part_x in 0..w {
                    let pixel = part_pixels.get_pixel(part_x, part_y);
                    new_img.put_pixel(start_x + part_x, start_y + part_y, *pixel);
                }
            }
        }
    }

    new_img.save("oput_pixelbypixel.jpg").unwrap();
}

fn cutOriginalImage(){
// let img2_width = img2.width();
// let img2_height = img2.height();

// let original_width = original_image.width();
// let original_height = original_image.height();

// let x_count = original_width / img2_width;
// let y_count = original_height / img2_height;

// let mut images_map: HashMap<String, Vec<u8>> = HashMap::new();

// for x in 0..x_count {
//     for y in 0..y_count {
//         let mut img = ImageBuffer::new(img2_width, img2_height);

//         for i in 0..img2_width {
//             for j in 0..img2_height {
//                 let pixel = original_image.get_pixel(x * img2_width + i, y * img2_height + j);
//                 img.put_pixel(i, j, pixel);
//             }
//         }

//         let mut buffer = Cursor::new(Vec::new());
//         img.write_to(&mut buffer, image::ImageOutputFormat::Jpeg)
//            .expect("Failed to write to buffer");

//         let key = format!("image_{}_{}", x, y);
//         images_map.insert(key, buffer.into_inner());
//     }
// }
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