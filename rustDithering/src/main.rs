use image::RgbImage;

mod color_mode;

fn find_closest<T>(colors: &[T], pixel: T) -> Option<T> 
where
    T: Copy + color_mode::Similarity,
{
    return colors.iter()
        .min_by(|&a, &b| 
            a.similar_to(pixel).partial_cmp(&(b.similar_to(pixel))).unwrap()
        ).copied()
}

fn dither_cie_colours() {
    let colors = vec![color_mode::MyLab::new(255,255,255), 
                                color_mode::MyLab::new(0,0,0),
                                color_mode::MyLab::new(255,0,0), 
                                color_mode::MyLab::new(0,255,0), 
                                color_mode::MyLab::new(0,0,255),
                                color_mode::MyLab::new(255, 255, 0),
                                color_mode::MyLab::new(255, 0, 255),
                                color_mode::MyLab::new(0, 255, 255), 
                                ];
    

    let img = image::open("data/fractal.png").unwrap().to_rgb8();
    let (width, height) = img.dimensions();

    let index = |x: u32, y: u32| -> Option<usize> {
        if x < width && y < height {
            return Some(y as usize * width as usize + x as usize); 
        } else {
            println!("x, width: {x}, {width} y, height: {y}, {height}");
            return None;
        }
    };

    let img_data: Vec<color_mode::MyLab> = img.pixels()
        .map(|p| color_mode::MyLab::new(p[0], p[1], p[2]))
        .collect();


    let mut img_data_copy = img_data.clone();

    for y in 0..height {
        for x in 0..width {
            if let Some(idx) = index(x, y) {
                let oldpixel = img_data_copy[idx];
                let newpixel = find_closest(&colors, oldpixel).unwrap();
                img_data_copy[idx] = newpixel;
                
                let err = oldpixel - newpixel;

                if x + 1 < width {
                    let idx2 = index(x + 1, y).unwrap();
                    img_data_copy[idx2] = img_data_copy[idx2] + (err * 7.0) / 16.0;
                }
                if x >= 1 && y + 1 < height {
                    let idx2 = index(x - 1, y + 1).unwrap();
                    img_data_copy[idx2] = img_data_copy[idx2] +  (err * 3.0) / 16.0;
                }
                if y + 1 < height {
                    let idx2 = index(x, y+1).unwrap();
                    img_data_copy[idx2] = img_data_copy[idx2] + (err * 5.0) / 16.0;
                }
                if x + 1 < width && y + 1 < height  {
                    let idx2 = index(x+1, y+1).unwrap();
                    img_data_copy[idx2] = img_data_copy[idx2] + err / 16.0;
                }
            }
        }
    }

    let img_data_u8: Vec<u8> = img_data_copy.iter().flat_map(|&lab| {
        let rgb = lab.to_rgb();
        return {
            vec![rgb.red.clamp(0, 255) as u8, 
                rgb.green.clamp(0, 255) as u8, 
                rgb.blue.clamp(0, 255) as u8]
        }

    }
    ).collect();

    let output_img = RgbImage::from_raw(width, height, img_data_u8);
    output_img.unwrap().save("output.png").unwrap();

}

fn dither_rgb_colours() {
    let colors = vec![color_mode::RGB::new(255, 255, 255), 
                                color_mode::RGB::new(0, 0, 0),
                                color_mode::RGB::new(255, 0, 0), 
                                color_mode::RGB::new(0,255, 0), 
                                color_mode::RGB::new(0,0,255),
                                color_mode::RGB::new(255, 255, 0),
                                color_mode::RGB::new(255, 0, 255),
                                color_mode::RGB::new(0, 255, 255), 
                                ];

    let img = image::open("data/butterflies.jpg").unwrap().to_rgb8();
    let (width, height) = img.dimensions();

    let index = |x: u32, y: u32| -> Option<usize> {
        if x < width && y < height {
            return Some(y as usize * width as usize + x as usize);
        } else {
            println!("x, width: {x}, {width} y, height: {y}, {height}");
            return None;
        }
    };

    let img_data: Vec<color_mode::RGB> = img.pixels()
        .map(|p| color_mode::RGB::new(p[0], p[1], p[2]))
        .collect();


    let mut img_data_copy = img_data.clone();

    for y in 0..height {
        for x in 0..width {
            if let Some(idx) = index(x, y) {
                let oldpixel = img_data_copy[idx];
                let newpixel = find_closest(&colors, oldpixel).unwrap();
                img_data_copy[idx] = newpixel;
                
                let err = oldpixel - newpixel;

                if x + 1 < width {
                    let idx2 = index(x + 1, y).unwrap();
                    img_data_copy[idx2] = img_data_copy[idx2] + (err * 7) / 16;
                }
                if x >= 1 && y + 1 < height {
                    let idx2 = index(x - 1, y + 1).unwrap();
                    img_data_copy[idx2] = img_data_copy[idx2] +  (err * 3) / 16;
                }
                if y + 1 < height {
                    let idx2 = index(x, y+1).unwrap();
                    img_data_copy[idx2] = img_data_copy[idx2] + (err * 5) / 16;
                }
                if x + 1 < width && y + 1 < height  {
                    let idx2 = index(x+1, y+1).unwrap();
                    img_data_copy[idx2] = img_data_copy[idx2] + err / 16;
                }
            }
        }
    }

    let img_data_u8: Vec<u8> = img_data_copy.iter().flat_map(|&rgb| vec![rgb.red.clamp(0, 255) as u8, 
    rgb.green.clamp(0, 255) as u8, 
    rgb.blue.clamp(0, 255) as u8]
    ).collect();

    let output_img = RgbImage::from_raw(width, height, img_data_u8);
    output_img.unwrap().save("output.png").unwrap();

}

fn main() {
    dither_rgb_colours();
}




fn dither_rgb_channels() {
    let colors = vec![0, 255];

    let img = image::open("data/bigImage.jpeg").unwrap().to_rgb8();
    let (width, height) = img.dimensions();

    let index = |x: u32, y: u32| -> Option<usize> {
        if x < width && y < height {
            return Some((y as usize * width as usize + x as usize) * 3);
        } else {
            return None;
        }
    };

    let img_data: Vec<i16> = img.pixels()
        .flat_map(|p| p.0.iter().map(|&c| c as i16))
        .collect();

    let mut img_data_copy = img_data.clone();

    for y in 0..height {
        for x in 0..width {
            if let Some(idx) = index(x, y) {
                for channel in 0..3 {
                    let oldpixel = img_data_copy[idx + channel];
                    let newpixel = find_closest(&colors, oldpixel).unwrap();
                    let err = oldpixel - newpixel;
                    img_data_copy[idx + channel] = newpixel;

                    if let Some(idx2) = index(x + 1, y) {

                        img_data_copy[idx2 + channel] += err * 7 / 16;
                    }
                    if let Some(idx2) = index(x - 1, y + 1) {
                        img_data_copy[idx2 + channel] += err * 3 / 16;
                    }
                    if let Some(idx2) = index(x, y + 1) {
                        img_data_copy[idx2 + channel] += err * 5 / 16;
                    }
                    if let Some(idx2) = index(x + 1, y + 1) {
                        img_data_copy[idx2 + channel] += err / 16;
                    }
                }
            }
        }
    }
    for y in 0..height {
        for x in 0..width {
            let idx = index(x, y).unwrap();
            for offset in 0..3 {
                img_data_copy.push(img_data[idx + offset]);
            }
        }
    }

    let img_data_u8: Vec<u8> = img_data_copy.iter().map(|&c| c.clamp(0, 255) as u8).collect();
    let output_img = RgbImage::from_raw(width, 2*height, img_data_u8);
    output_img.unwrap().save("output.png").unwrap();
}