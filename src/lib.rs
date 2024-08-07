
use image::{DynamicImage, imageops::FilterType, ImageReader, ImageError, GenericImageView, ImageBuffer, Luma};
use rayon::prelude::*;
use std::path::{Path};

use divan::{black_box, counter::BytesCount, AllocProfiler, Bencher};

pub fn resize_image(img: DynamicImage, width: u32, height: u32) ->DynamicImage {
    img.resize(width, height, FilterType::Lanczos3)
}

pub fn resize_exact_image(img: DynamicImage, nwidth: u32, nheight: u32) -> DynamicImage {
    img.resize_exact(nwidth, nheight, FilterType::Lanczos3)
}

pub fn save_image(img: DynamicImage, output: &Path, format: image::ImageFormat) {
    img.save_with_format(output, format).expect("Failed to save image");
}

pub fn rotate_image(img: DynamicImage, degrees: RotateDegrees) -> DynamicImage {
    match degrees {
        RotateDegrees::D90 => img.rotate90(),
        RotateDegrees::D180 => img.rotate180(),
        RotateDegrees::D270 => img.rotate270()
    }

}

pub fn open_image(path: &Path) -> Result<DynamicImage, ImageError> {
    let img = match ImageReader::open(path)
        .expect("Failed to open image")
        .decode() {
        Ok(o) => Ok(o),
        Err(e) => Err(e)
    };

    img
}

pub enum RotateDegrees {
    D90,
    D180,
    D270,
}


pub fn manual_rotate_image(img: &ImageBuffer<Luma<u16>, Vec<u16>>, degrees: RotateDegrees) -> ImageBuffer<Luma<u16>, Vec<u16>> {
    let (width, height) = img.dimensions();
    let mut img_buf: ImageBuffer<Luma<u16>, Vec<u16>> = match degrees {
        RotateDegrees::D90 => ImageBuffer::new(height, width),
        RotateDegrees::D270 => ImageBuffer::new(height, width),
        RotateDegrees::D180 => ImageBuffer::new(width, height)
    };

    img_buf.par_enumerate_pixels_mut().for_each(|(x, y, bp)| {
        let pixel = img.get_pixel(y, height - 1 - x);
        bp.0[0] = pixel.0[0];
    });

    img_buf
}

pub fn manual_rotate_image_u8(img: &ImageBuffer<Luma<u8>, Vec<u8>>, degrees: RotateDegrees) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let (width, height) = img.dimensions();
    let mut img_buf: ImageBuffer<Luma<u8>, Vec<u8>> = match degrees {
        RotateDegrees::D90 => ImageBuffer::new(height, width),
        RotateDegrees::D270 => ImageBuffer::new(height, width),
        RotateDegrees::D180 => ImageBuffer::new(width, height)
    };

    img_buf.par_enumerate_pixels_mut().for_each(|(x, y, bp)| {
        let pixel = img.get_pixel(y, height - 1 - x);
        bp.0[0] = pixel.0[0];
    });

    img_buf
}
