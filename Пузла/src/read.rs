use std::{ fs::{ self, File }, io::{ BufReader, Read, Seek, SeekFrom }, path::Path };

use image::{ DynamicImage, GenericImageView, ImageFormat, RgbaImage };

pub fn read_images(path_str: String) -> (Vec<RgbaImage>, u32, u32) {
    let mut mw = 0;
    let mut mh = 0;
    let mut images: Vec<RgbaImage> = Vec::new();
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
                                // println!("png");
                                reader.seek(SeekFrom::Start(0)).unwrap();
                                let img = image::load(reader, format).unwrap();
                                (mw, mh) = max_size(mw, mh, img.width(), img.height());
                                images.push(img.to_rgba8());
                            }
                            ImageFormat::Jpeg => {
                                // println!("jpg");
                                reader.seek(SeekFrom::Start(0)).unwrap();
                                let img = image::load(reader, format).unwrap();
                                (mw, mh) = max_size(mw, mh, img.width(), img.height());
                                images.push(img.to_rgba8());
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

    return (images, mw, mh);
}

pub fn read_parts(img: &mut DynamicImage, partw: u32, parth: u32) -> Vec<RgbaImage> {
    let mut images = Vec::new();
    let ow = img.width();
    let oh = img.height();
    let cols = ((ow as f64) / (partw as f64)).round() as u32;
    let rows = ((oh as f64) / (parth as f64)).round() as u32;

    for y in 0..rows {
        for x in 0..cols {
            let top_left_x = x * partw;
            let top_left_y = y * parth;
            let part_img = img.crop_imm(top_left_x, top_left_y, partw, parth);
            // let filename = format!("output{}.png", y * cols + x + 1);
            // part_img.save(filename);
            // let mut novi_img = ImageReader::open(format!("output{}.png", y * cols + x + 1))
            //     .unwrap()
            //     .decode()
            //     .unwrap();
            images.push(part_img.to_rgba8());
        }
    }
    return images;
}

fn max_size(mut mw: u32, mut mh: u32, cw: u32, ch: u32) -> (u32, u32) {
    if cw > mw {
        mw = cw;
    }
    if ch > mh {
        mh = ch;
    }
    return (mw, mh);
}
