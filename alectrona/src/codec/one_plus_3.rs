extern crate image;

use crate::LogoError;
use LogoError::*;

const MAX_REPEATED: u8 = 255;

pub fn decode(data: &[u8], width: u32, height: u32) -> Result<image::DynamicImage, LogoError> {
    let expected_image_size = (width * height * 3) as usize;
    let mut buffer: Vec<u8> = Vec::with_capacity(expected_image_size);

    // necessary while step isn't stabilized
    let mut skip = false;
    for (&byte, &quantity) in data.iter().zip(data.iter().skip(1)).filter(|_| {
        skip = !skip;
        skip
    }) {
        for _ in 0..quantity {
            buffer.push(byte);
        }
    }
    if buffer.len() != expected_image_size {
        return Err(WrongImageSize);
    }
    // bgr to rgb
    for i in 0..buffer.len() / 3 {
        buffer.swap(i * 3, i * 3 + 2);
    }
    Ok(image::DynamicImage::ImageRgb8(
        image::ImageBuffer::from_raw(width, height, buffer).unwrap(),
    ))
}

pub fn encode(img: image::DynamicImage) -> Vec<u8> {
    let mut img = match img {
        // Rgb8 is actually 8-bits PER CHANNEL
        image::ImageRgb8(i) => i,
        _ => img.to_rgb(),
    };

    // rgb to bgr
    for pixel in img.pixels_mut() {
        let temp = pixel[0];
        pixel[0] = pixel[2];
        pixel[2] = temp;
    }

    let mut encoded: Vec<u8> = Vec::new();

    assert!(img.len() % 3 == 0);
    let orig_buffer = img.into_raw();

    let mut queue_size: u32 = 0;
    let mut last_byte = None;
    for &byte in orig_buffer.iter() {
        if let Some(last_byte) = last_byte {
            if byte == last_byte {
                // queue_size = queue_size.wrapping_add(1);
                queue_size += 1;
            } else {
                for _ in 0..(queue_size / MAX_REPEATED as u32) {
                    encoded.push(last_byte);
                    encoded.push(MAX_REPEATED);
                }
                let not_pushed = (queue_size % MAX_REPEATED as u32) as u8;
                encoded.push(last_byte);
                encoded.push(not_pushed + 1);
                queue_size = 0;
            }
        }
        last_byte = Some(byte);
    }
    if let Some(last_byte) = last_byte {
        for _ in 0..(queue_size / MAX_REPEATED as u32) {
            encoded.push(last_byte);
            encoded.push(MAX_REPEATED);
        }
        let not_pushed = (queue_size % MAX_REPEATED as u32) as u8;
        if not_pushed != MAX_REPEATED - 1 {
            encoded.push(last_byte);
            encoded.push(not_pushed + 1)
        }
    }
    encoded
}
