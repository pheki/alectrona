use std::ops::Range;

use image::{DynamicImage, GenericImageView};

use crate::LogoError;
use LogoError::*;

/// Decodes image data for harpia boot logo
/// Returns data and dimensions in a tuple
pub fn decode(data: &[u8], width: u32, height: u32) -> Result<DynamicImage, LogoError> {
    let mut range = 0..data.len();

    let expected_image_size = (width * height * 3) as usize;
    let mut buffer = Vec::with_capacity(expected_image_size);
    while let Some(i) = range.next() {
        match data[i] & 0xF0 {
            0x80 => {
                for _ in 0..4 {
                    range.next();
                }
                let pixel = (data[i + 2], data[i + 3], data[i + 4]);
                for _ in 0..((data[i] as u16 * 0x100 + data[i + 1] as u16) - 0x8000) {
                    buffer.push(pixel.2);
                    buffer.push(pixel.1);
                    buffer.push(pixel.0);
                }
            }
            0x00 => {
                range.next();
                for j in 0..(data[i] as usize * 0x100 + data[i + 1] as usize) {
                    buffer.push(data[i + 4 + j * 3]);
                    buffer.push(data[i + 3 + j * 3]);
                    buffer.push(data[i + 2 + j * 3]);
                    for _ in 0..3 {
                        range.next();
                    }
                }
            }
            _ => {
                return Err(WrongImageFormat);
            }
        }
    }
    if buffer.len() != expected_image_size {
        return Err(WrongImageSize);
    }
    Ok(DynamicImage::ImageRgb8(
        image::ImageBuffer::from_raw(width, height, buffer).unwrap(),
    ))
}

/// Encodes by row for MotoKitKat
pub fn encode(img: DynamicImage) -> Vec<u8> {
    let dimensions = img.dimensions();
    let img = match img {
        // Rgb8 is actually 8-bits PER CHANNEL
        DynamicImage::ImageRgb8(i) => i,
        _ => img.to_rgb8(),
    };

    let mut encoded = Vec::new();

    assert!(img.len() % 3 == 0);

    let pixels: Vec<&image::Rgb<u8>> = img.pixels().collect();
    for i in 0..dimensions.1 {
        let line_pixels =
            &pixels[((i * dimensions.0) as usize)..(((i + 1) * dimensions.0) as usize)];
        encode_row(line_pixels, &mut encoded);
    }
    encoded
}

fn encode_row(pixels: &[&image::Rgb<u8>], encoded: &mut Vec<u8>) {
    let mut repeats: Vec<Range<usize>> = Vec::new();
    let mut start: Option<usize> = None;
    let mut last_pix: Option<&image::Rgb<u8>> = None;
    for (i, pixel) in pixels.iter().cloned().enumerate() {
        if let Some(last_pixel) = last_pix {
            match start {
                Some(start_pos) => {
                    if pixel != last_pixel {
                        if i - start_pos > 2 {
                            repeats.push(start_pos..i);
                        }
                        start = None;
                    }
                }
                None => {
                    if pixel == last_pixel {
                        start = Some(i - 1);
                    }
                }
            }
        }
        last_pix = Some(pixel);
    }
    if let Some(start_pos) = start {
        repeats.push(start_pos..pixels.len());
        // start = None;
    }

    let mut cursor = 0;
    let mut repeat_iter = repeats.iter();
    let mut repeat = repeat_iter.next();
    let width = pixels.len();
    while cursor < width {
        match repeat {
            Some(r) if r.start == cursor => {
                let pixels_written = r.end - r.start;
                let pixel = pixels[cursor];
                debug_assert!(pixels_written < 0x7FFF);
                encoded.push(0x80 | (pixels_written >> 8) as u8);
                encoded.push(pixels_written as u8);
                encoded.push(pixel[2]);
                encoded.push(pixel[1]);
                encoded.push(pixel[0]);
                cursor += pixels_written;
                repeat = repeat_iter.next();
            }
            Some(range) => {
                let pixels_written = range.start - cursor;
                debug_assert!(pixels_written < 0x7FFF);
                encoded.push((pixels_written >> 8) as u8);
                encoded.push(pixels_written as u8);
                for i in 0..pixels_written {
                    let pixel = pixels[cursor + i];
                    encoded.push(pixel[2]);
                    encoded.push(pixel[1]);
                    encoded.push(pixel[0]);
                }
                cursor += pixels_written;
            }
            None => {
                let pixels_written = pixels.len() - cursor;
                debug_assert!(pixels_written < 0x7FFF);
                encoded.push((pixels_written >> 8) as u8);
                encoded.push(pixels_written as u8);
                for i in 0..pixels_written {
                    let pixel = pixels[cursor + i];
                    encoded.push(pixel[2]);
                    encoded.push(pixel[1]);
                    encoded.push(pixel[0]);
                }
                cursor += pixels_written;
            }
        }
    }
    assert_eq!(cursor, width);
}
