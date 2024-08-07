use std::{env, path::Path, path::PathBuf, process};
use image::{ImageBuffer, ImageFormat, Luma};
use image_processor::*;

mod lib;

fn main() {
    let mut args = env::args();
    // consume first arg and do nothing since it is calling this program
    args.next();

    // 2nd arg is file path
    // get file path
    let in_file = match args.next() {
        Some(fp) => fp,
        None => {
            eprintln!("No input file path");
            process::exit(1);
        }
    };

    // check if file exists
    let in_path = Path::new(&in_file);
    if !in_path.exists() {
        eprintln!("File does not exist. Exiting!");
        process::exit(1);
    }

    // consume the 3rd arg that is the output file path
    let out_file = args.next().unwrap_or_else(|| String::new());

    let out_path: PathBuf = if out_file.is_empty() {
        let par_dir = in_path.parent().unwrap();
        let file_name = in_path.file_name().unwrap().to_str().unwrap();
        let mut new_file_name = String::from("NEW_");
        new_file_name.push_str(file_name);
        par_dir.join(new_file_name)
    } else {
        PathBuf::from(out_file)
    };

    let img = match open_image(in_path) {
        Ok(i) => i,
        Err(e) => {
            eprintln!("{:#?}", e);
            process::exit(1);
        }
    };
    let img = img.into_luma16();
    //let img = resize_image(img, 512, 512);
    let img = manual_rotate_image(&img, RotateDegrees::D90);
    //let img = resize_exact_image(img, 512, 512);
    let res = img.save_with_format(out_path, ImageFormat::Png);
    match res {
        Ok(o) => {},
        Err(e) => {
            eprintln!("{:#?}", e);
        }
    }
    //save_image(img, out_path.as_path(), ImageFormat::Jpeg);


}
