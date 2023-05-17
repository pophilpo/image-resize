use image::imageops::Triangle;
use image::io::Reader as ImageReader;
use image::{DynamicImage, GenericImageView, ImageFormat};
use std::{fs, io::BufReader, os::unix::fs::MetadataExt};

pub fn read_exif_metadata(image_path: &str) -> Option<u32> {
    let file = fs::File::open(image_path).unwrap();

    let exif_metadata = exif::Reader::new().read_from_container(&mut BufReader::new(&file));

    // If read_from_container results in Err, this means that there is no EXIF metadata
    // https://github.com/kamadak/exif-rs/issues/24
    match exif_metadata {
        Ok(exif_metadata) => {
            let orientation = exif_metadata.get_field(exif::Tag::Orientation, exif::In::PRIMARY);
            match orientation {
                Some(orientation) => orientation.value.get_uint(0),
                None => None,
            }
        }
        Err(_) => None,
    }
}

pub fn check_encoded_size(image_path: &str) -> std::io::Result<u64> {
    let meta = fs::metadata(image_path)?;
    let filesize = meta.size();
    Ok(filesize)
}

pub fn process_image(image_path: &str, image_size: u64) -> image::ImageResult<()> {
    let orientation = read_exif_metadata(image_path);
    let image = read_image(image_path)?;
    let ratio = compute_ratio_fast(image_size);

    let dimensions = image.dimensions();
    let new_width = (dimensions.0 as f64 / ratio) as u32;
    let new_height = (dimensions.1 as f64 / ratio) as u32;
    let tmp_image = image.resize(new_width, new_height, Triangle);

    let rotated_image: DynamicImage = match orientation {
        Some(2) => tmp_image.fliph(),
        Some(3) => tmp_image.rotate180(),
        Some(4) => tmp_image.flipv(),
        Some(5) => tmp_image.rotate90().fliph(),
        Some(6) => tmp_image.rotate90(),
        Some(7) => tmp_image.rotate270().fliph(),
        Some(8) => tmp_image.rotate270(),
        _ => tmp_image,
    };

    rotated_image.save_with_format(image_path, ImageFormat::Jpeg)?;
    Ok(())
}

pub fn read_image(image_path: &str) -> image::ImageResult<DynamicImage> {
    let image = ImageReader::open(image_path)?.decode()?;
    Ok(image)
}

pub fn compute_ratio_fast(image_size: u64) -> f64 {
    image_size as f64 / 1000000.0
}
