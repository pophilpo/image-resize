use indicatif::{ProgressBar, ProgressStyle};
use std::env;
use std::process::exit;
use walkdir::WalkDir;

mod image_transforms;
use image_transforms::{check_encoded_size, process_image};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Provide the source path as the argument to the program:");
        eprintln!("{} <PATH>", args[0]);
        exit(1);
    }

    let images_path = &args[1];

    let walker = WalkDir::new(images_path);
    let filecount = walker.into_iter().count() as u64;
    let walker = WalkDir::new(images_path);
    let progress_bar = ProgressBar::new(filecount);

    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}]")
            .unwrap(),
    );

    for entry in walker.into_iter().flatten() {
        if entry.metadata()?.is_file() {
            if let Some(filename) = entry.path().to_str() {
                let filesize = check_encoded_size(filename)?;
                if filesize <= 1000000 {
                    continue;
                }

                if let Err(e) = process_image(filename, filesize) {
                    eprintln!("Skipping file {filename}: {e}")
                }
            }
        }
        progress_bar.inc(1);
        std::thread::sleep(std::time::Duration::from_secs(2));
    }

    progress_bar.finish();
    Ok(())
}
