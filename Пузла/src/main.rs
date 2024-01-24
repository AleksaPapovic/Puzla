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
    eframe::run_native(
        "Image Bytes Display",
        native_options,
        Box::new(|_cc| Box::new(MyApp::new(img_bytes))),
    );
}


fn createImage(image: DynamicImage,bytes: &mut Vec<u8>){
    for i in 0.. bytes.len() {
        bytes[i]= 0
    }
    let img2 = ImageReader::open("src/examples/slika 1/part a.jpg")
    .unwrap()
    .decode()
    .unwrap();

    println!("{}",img2.height());
    println!("{}",img2.width());
    println!("{:?}",img2.as_bytes().len());
    let size: usize= img2.as_bytes().len();
    let bytes2 = img2.as_bytes();

    let ow = image.width() as usize;
    let oh = image.height() as usize;
    println!("{}",ow);
    println!("{}",oh);
    let width = img2.width() as usize; 
    let height = img2.height() as usize; 
    println!("{}",width);
    println!("{}",height);

    let diffw = ow-width;  
    println!("{}",diffw);
    let bytes_per_pixel = 3;
    for i in 0.. bytes.len() {
        if i ==0 {  
             for j in  0..height{                                
                let row_start = j * width * bytes_per_pixel + j*diffw;
                let row_end = row_start + width * bytes_per_pixel;
                bytes[row_start..row_end].copy_from_slice(&bytes2[0..65]);
        }
      }
    }

    let img: RgbImage = ImageBuffer::from_raw(image.width(), image.height(), bytes.clone())
        .expect("greska");

    img.save("output.png").unwrap();
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