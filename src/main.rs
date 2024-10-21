use std::fs;
use std::path::Path;
use std::io::Read;
use colored::*;

fn main() {

    let src = "C:\\Users\\jarrod\\AppData\\Local\\Microsoft\\BingWallpaperApp\\WPImages\\";
    let dest = "\\\\vesuvius\\pictures\\Wallpapers\\Bing\\";

    let src_path = Path::new(src);
    let dest_path = Path::new(dest);

    println!("Copying Bing Wallpapers\nfrom {}\n  to {}", src_path.display().to_string().cyan(), dest_path.display().to_string().bright_cyan());

    let src_files = match fs::read_dir(src_path) {
        Ok(files) => files,
        Err(e) => {
            println!("Error reading source directory: {}", e);
            return;
        }
    };

    for file in src_files {
        let file = match file {
            Ok(f) => f,
            Err(e) => {
                println!("Error reading file: {}", e);
                continue;
            }
        };

        let file_name = file.file_name();
        let dest_file = dest_path.join(file_name);
        
        if file.path().extension().unwrap() != "jpg" {
            continue;
        }

        if dest_file.exists() {
            println!("{} {}", "File already exists:".bright_red(), dest_file.display().to_string().bright_magenta());
            continue;
        }
        
        match fs::copy(file.path(), dest_file.clone()) {
            Ok(_) => println!("{} {}", "Copied file:".bright_cyan(), dest_file.display().to_string().bright_magenta()),
            Err(e) => println!("Error copying file: {}", e),
        }
    }

    println!("{}","Press Return to exit...".bright_green());
    let _ = std::io::stdin().read(&mut [0u8]).unwrap();

}

