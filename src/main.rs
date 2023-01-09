use pbr::ProgressBar;
use std::env;
use walkdir::WalkDir;

mod image_transforms;
use image_transforms::{check_encoded_size, process_image};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let images_path = &args[1];

    let walker = WalkDir::new(images_path);
    let filecount = walker.into_iter().count() as u64;
    let walker = WalkDir::new(images_path);
    let mut progress_bar = ProgressBar::new(filecount);

    for entry in walker.into_iter().flatten() {
        if entry.metadata()?.is_file() {
            if let Some(filename) = entry.path().to_str() {
                let filesize = check_encoded_size(filename)?;

                if filesize >= 1000000 {
                    process_image(filename, filesize)?;
                }
            }
        }
        progress_bar.inc();
    }

    Ok(())
}
