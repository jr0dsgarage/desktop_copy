use std::fs;
use std::path::Path;
use std::io::Read;

fn main() {
    // Copy files from C:\Users\jarrod\AppData\Local\Microsoft\BingWallpaperApp\WPImages\
    // to \\vesuvius\pictures\Wallpapers\Bing\
    // only copying new or non-existent files

    let src = "C:\\Users\\jarrod\\AppData\\Local\\Microsoft\\BingWallpaperApp\\WPImages\\";
    let dest = "\\\\vesuvius\\pictures\\Wallpapers\\Bing\\";

    let src_path = Path::new(src);
    let dest_path = Path::new(dest);

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
            println!("File already exists: {:?}", dest_file);
            continue;
        }
        
        match fs::copy(file.path(), dest_file.clone()) {
            Ok(_) => println!("Copied file: {:?}", dest_file),
            Err(e) => println!("Error copying file: {}", e),
        }
    }
    // pause so the window doesn't close
    println!("Press any key to exit...");
    let _ = std::io::stdin().read(&mut [0u8]).unwrap();

}

