use std::{ borrow::{ Borrow, BorrowMut }, io::Read };

use image::{ DynamicImage, EncodableLayout, GenericImageView, ImageBuffer, RgbImage };

pub fn append_part(img: &mut DynamicImage, part_img: &RgbImage, indeks: usize) {
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
    let mut counter = 0;
    let part_bytes = part_img.as_bytes();
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
                    let part_index: u8 = (p * partw) as u8;
                    let start = start_index + (((10 * partw) / 3) * parth) / 3;
                    let end = start + partw;
                    // println!("{},{}", start, end);
                    bytes[start..end].copy_from_slice(
                        &part_bytes[part_index as usize..(part_index + (partw as u8)) as usize]
                    );
                    start_index += ow;
                    end_index = start_index + partw;
                }
            }
            counter += 1;
        }
        counter += 1;
    }

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
