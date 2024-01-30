use image::{ DynamicImage, GenericImageView, RgbaImage };

pub fn append_part(
    // bytes: &mut Vec<u8>,
    img: &mut DynamicImage,
    part_img: &RgbaImage,
    indeks: usize,
    new_img: &mut RgbaImage,
    putanja_delova: &String
) {
    let ow = (img.width() * 3) as usize;
    let oh = (img.height() * 3) as usize;
    // println!("Original width: {} height: {}", ow, oh);
    let partw = (part_img.width() * 3) as usize;
    let parth = (part_img.height() * 3) as usize;
    // println!("Part width: {} height: {}", partw, parth);

    let x = ow / partw;
    let y = ((oh as f64) / (parth as f64)).round() as usize;
    // println!("x: {} y: {}", x, y);

    // let diff = ow % partw;
    // let part_bytes = part_img.as_bytes();
    // let mut start_index = 0;
    // for j in 0..x {
    //     for i in 0..y {
    //         // println!("Part i: {} j: {} counter: {}", i, j, counter);
    //         start_index = indeks * partw * parth;
    //         let mut end_index = start_index + partw;
    //         // println!("{}", start_index);
    //         // println!("{}", end_index);
    //         if i * j == 6 {
    //             for p in 0..parth / 3 {
    //                 // println!("s{}",start_index);
    //                 // println!("e{}",end_index);
    //                 // println!("ps{}",p*partw);
    //                 // println!("pe{}",p*partw +partw);
    //                 let part_index: u8 = (p * partw) as u8;
    //                 let p_start = start_index + (((10 * partw) / 3) * parth) / 3;
    //                 let p_end: usize = start + partw;
    //                 // println!("{},{}", start, end);
    //                 bytes[start..end].copy_from_slice(&part_bytes[p_start..p_end]);
    //                 start_index += ow;
    //                 end_index = start_index + partw;
    //             }
    //         }
    //     }
    // }

    // let result: RgbaImage = ImageBuffer::from_raw(
    //     img.width() as u32,
    //     img.height() as u32,
    //     bytes.to_vec()
    // ).expect("error");
    // result.save("result.jpg").unwrap();

    let w = (partw / 3) as u32;
    let h = (parth / 3) as u32;

    let mut counter = 0;
    for i in 0..y {
        for j in 0..x {
            if counter == (indeks as u32) {
                for part_y in 0..h {
                    for part_x in 0..w {
                        let pixel = part_img.get_pixel(part_x, part_y);
                        let mut xp = part_x + ((j as u32) * (partw as u32)) / (3 as u32);
                        let mut yp = part_y + ((i as u32) * (parth as u32)) / (3 as u32);
                        if yp >= (img.height() as u32) {
                            yp = img.height() - 1;
                        }
                        new_img.put_pixel(xp, yp, *pixel);
                    }
                }
            }
            counter += 1;
        }
    }

    new_img.save(format!("result{putanja_delova}.jpg")).unwrap();
}
