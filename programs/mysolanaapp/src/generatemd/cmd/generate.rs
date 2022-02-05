use crate::Generate;
use crate::{art, metadata};
use std::{fs::remove_dir_all, path::Path};

pub fn handle(options: Generate) {
    if !options.skip_metadata {
        println!("Cleaning output directory...");
        let output_directory_path = Path::new(&options.output);
        if output_directory_path.exists() {
            remove_dir_all(&output_directory_path)
                .expect("Error occured cleaning output directory");
        }

        metadata::generate(&options.config, &options.assets, &options.output);
    } else {
        println!("Skipping metadata generation");
    }

    if !options.skip_art {
        art::generate(&options.config, options.assets, options.output);
    } else {
        println!("Skipping art generation");
    }
}
