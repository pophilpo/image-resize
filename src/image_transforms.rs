use image::imageops::{Nearest, Triangle};
use image::io::Reader as ImageReader;
use image::{DynamicImage, GenericImageView, ImageFormat};
use std::{fs, os::unix::fs::MetadataExt};

pub fn check_encoded_size(image_path: &str) -> Result<u64, Box<dyn std::error::Error>> {
    let meta = fs::metadata(image_path)?;
    let filesize = meta.size();
    return Ok(filesize);
}

pub fn process_image(image_path: &str, image_size: u64) -> Result<(), Box<dyn std::error::Error>> {
    let image = read_image(image_path)?;
    let ratio = compute_ratio_fast(image_size);

    let dimensions = image.dimensions();
    let new_width = (dimensions.0 as f64 / ratio) as u32;
    let new_height = (dimensions.1 as f64 / ratio) as u32;
    let tmp_image = image.resize(new_width, new_height, Nearest);
    tmp_image.save_with_format(image_path, ImageFormat::Jpeg)?;
    Ok(())
}

pub fn read_image(image_path: &str) -> Result<DynamicImage, Box<dyn std::error::Error>> {
    let image = ImageReader::open(image_path)?.decode()?;
    return Ok(image);
}

pub fn compute_ratio_fast(image_size: u64) -> f64 {
    return image_size as f64 / 1000000.0;
}

fn compute_ratio_slow(image: DynamicImage) -> f32 {
    // This is omega slow. Maybe do a binary search ?

    let mut ratio = 2.5;
    let mut max_image_size = 0;
    let dimensions = image.dimensions();

    while ratio > 1.3 && max_image_size < 1000000 {
        ratio = ratio - 0.3;
        let new_width = (dimensions.0 as f32 / ratio) as u32;
        let new_height = (dimensions.1 as f32 / ratio) as u32;
        let tmp_image = image.resize(new_width, new_height, Triangle);
        //let mut buff = Vec::new();
        //tmp_image
            //.write_to(&mut buff, image::ImageFormat::Jpeg)
            //.unwrap();
        //max_image_size = buff.len();
        //println!("Ratio {}, image_size {} bytes", ratio, max_image_size);
    }

    return ratio;
}
