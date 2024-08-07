use std::path::Path;
use divan::Bencher;
use image::{ImageBuffer, ImageFormat, ImageReader, Luma};
use image_processor::{manual_rotate_image, manual_rotate_image_u8, RotateDegrees};

fn main() {
    divan::main();
}
#[divan::bench(sample_size=100)]
fn bench_rotate(bencher: Bencher) {
    let p = Path::new("Z:\\in.png");
    let img = ImageReader::open(p).unwrap().decode().unwrap();
    let img = img.grayscale();
    bencher
        .bench_local(|| {
            let ar = img.rotate90();
            //ar.save_with_format("Z:\\ar_out.png", ImageFormat::Png).unwrap();

        })
}



#[divan::bench(sample_size=100)]
fn bench_manual_rotate_u16(bencher: Bencher) {
    let p = Path::new("Z:\\in.png");
    let img = ImageReader::open(p).unwrap().decode().unwrap();

    bencher
        .bench_local(|| {
            let img = img.to_luma16();
            let mr = manual_rotate_image(&img, RotateDegrees::D90);
            //mr.save_with_format("Z:\\out_u16.png", ImageFormat::Png).unwrap();
        })
}

#[divan::bench(sample_size=100)]
fn bench_manual_rotate_u8(bencher: Bencher) {
    let p = Path::new("Z:\\in.png");
    let img = ImageReader::open(p).unwrap().decode().unwrap();

    bencher
        .bench_local(|| {
            let img: ImageBuffer<Luma<u8>, Vec<u8>> = img.to_luma8();
            let mr = manual_rotate_image_u8(&img, RotateDegrees::D90);
            //mr.save_with_format("Z:\\out_u8.png", ImageFormat::Png).unwrap();
        })
}