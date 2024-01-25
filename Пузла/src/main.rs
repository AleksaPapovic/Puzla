use std::{borrow::Borrow};
use image::{io::Reader as ImageReader, DynamicImage, GenericImageView, ImageBuffer, RgbImage};
use egui::{CentralPanel, Window, Label};

fn main() {
    let img = ImageReader::open("src/examples/picture1.jpg")
        .unwrap()
        .decode()
        .unwrap();


    let img_bytes = img.to_bytes();
    createImage(img,&mut img_bytes.clone());

    let native_options = eframe::NativeOptions::default();
    // eframe::run_native(
    //     "Image Bytes Display",
    //     native_options,
    //     Box::new(|_cc| Box::new(MyApp::new(img_bytes))),
    // );


    let imgx = ImageReader::open("src/examples/slika 1/part x.jpg")
    .unwrap()
    .decode()
    .unwrap();
let imgx_bytes = imgx.to_bytes();
createImage(imgx,&mut imgx_bytes.clone());
let native_options2 = eframe::NativeOptions::default();
eframe::run_native(
    "Image Bytes Display2",
    native_options2,
    Box::new(|_cc| Box::new(MyApp::new(imgx_bytes))),
);

}


fn createImage(img: DynamicImage,bytes: &mut Vec<u8>){
    for i in 0.. bytes.len() {
        bytes[i]= 0
    }
    let img2 = ImageReader::open("src/examples/slika 1/part b.jpg")
    .unwrap()
    .decode()
    .unwrap();

    compare(&mut bytes.clone(),img.clone(),img2.clone());
    
    appendPart(&mut bytes.clone(),img,img2);
}

fn compare(bytes: &mut Vec<u8>,img: DynamicImage,part_img: DynamicImage){

    let ow = img.width();
    let oh = img.height();
    let partw = part_img.width();
    let parth = part_img.height();
    let diffw = ow-partw;
    let bytes_per_pixel = 3;


    let x = ow / partw;
    let y = oh / parth;

    let mut image_index = 1;

    for x in 0..x {
        for y in 0..y {
            let mut img_section = ImageBuffer::new(partw, parth);

            // Copy pixels from the main image to the section
            for i in 0..partw {
                for j in 0..parth {
                    let pixel = img.get_pixel(x * partw + i, y * parth + j);
                    img_section.put_pixel(i, j, pixel);
                }
            }

            let file_name = format!("output{}.jpg", image_index);
            img_section.save(file_name).expect("Failed to save image");
            image_index += 1;
        }
    }
}


fn appendPart(bytes: &mut Vec<u8>,img: DynamicImage, part_img: DynamicImage){

    let ow = img.width() as usize;
    let oh = img.height() as usize;
    println!("{}",ow);
    println!("{}",oh);
    let partw = part_img.width() as usize;
    let parth = part_img.height() as usize;
    println!("{}",partw);
    println!("{}",parth);

    let diffw = ow-partw;
    println!("{}",diffw);
    let bytes_per_pixel = 3;


    let x = ow / partw;
    let y = oh / parth;

    let bytes_per_pixel = 3;

    let start_index = 123;
    let part_bytes = part_img.to_bytes();

for j in 0..parth {
    let src_row_start = j * partw * bytes_per_pixel;
    let dest_row_start = start_index + j * (partw + diffw) * bytes_per_pixel;

    if dest_row_start + partw * bytes_per_pixel <= bytes.len() {
        bytes[dest_row_start..dest_row_start + partw * bytes_per_pixel]
            .copy_from_slice(&part_bytes[src_row_start..src_row_start + partw * bytes_per_pixel]);
    }
}

    let img: RgbImage = ImageBuffer::from_raw(img.width(), img.height(), bytes.clone())
        .expect("greska");

    img.save("output.png").unwrap();
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