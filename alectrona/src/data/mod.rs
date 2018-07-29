mod moto_kit_kat;
mod one_plus_3;

extern crate image;
use image::GenericImage;
use std::io;

use std::fmt;
use std::io::prelude::*;

use DeviceFamily;
use LogoError;
use LogoError::*;

use codec;

/// The data of a single logo in a logo binary.
#[derive(Debug)]
pub struct Logo {
    identifier: String,
    location: usize,
    width: u32,
    height: u32,
    data: Vec<u8>,
}

impl Logo {
    /// Gets the identifier of the logo.
    pub fn identifier(&self) -> &str {
        &self.identifier[..]
    }
    /// Gets the location of the logo in the binary file.
    pub fn location(&self) -> usize {
        self.location
    }
    /// Gets the width of the logo.
    pub fn width(&self) -> u32 {
        self.width
    }
    /// Gets the height of the logo.
    pub fn height(&self) -> u32 {
        self.height
    }
    /// Returns reference a reference to the image data.
    pub fn data(&self) -> &Vec<u8> {
        &self.data
    }
    // /// Sets data to write in file later.
    // /// Will not update location of this logo, nor subsequent Logos.
    // pub fn set_data(&mut self, new_data: Vec<u8>) {
    //     self.data = new_data;
    // }
    /// Sets data to write in file later.
    /// Will not update location of this logo, nor subsequent Logos.
    pub fn set_new_image(&mut self, data: Vec<u8>, width: u32, height: u32) {
        self.data = data;
        self.width = width;
        self.height = height;
    }

    /// Set new location of the logo in the binary file.
    pub fn set_location(&mut self, location: usize) {
        self.location = location;
    }
}

/// Represents the data inside a binary boot logo file.
#[derive(Debug)]
pub struct LogoBin {
    family: DeviceFamily,
    // probably should abandon mime in favor of family
    mime: String,
    // is it useful to save header_size?
    header_size: usize,
    logos: Vec<Logo>,
    // used to indicate whether any of the logos have been modified, so process_changes is executed before writing
    inconsistent: bool,
}

impl LogoBin {
    /// Returns LogoBin from boot logo file (read-only recommended).
    ///
    /// May change the seek position of the file.
    pub fn from_file<F: Read + Seek>(
        file: &mut F,
        family: DeviceFamily,
    ) -> Result<LogoBin, LogoError> {
        match family {
            DeviceFamily::MotoKitKat => moto_kit_kat::logo_bin_from_file(file),
            DeviceFamily::OnePlus3 => one_plus_3::logo_bin_from_file(file),
        }
    }

    /// This method is used internally, it does the post-processing stuff needed after replacing logos,
    /// like calculating new "locations" for the logos in the file.
    ///
    /// It is not intended to be public, but it's used in tests, which are all organized as integration tests for now.
    pub fn process_changes(&mut self) {
        match self.family {
            DeviceFamily::MotoKitKat => moto_kit_kat::process_changes(self),
            DeviceFamily::OnePlus3 => one_plus_3::process_changes(self),
        }
    }

    /// Returns a reference to the logo struct with the specified id.
    pub fn logo_with_id(&self, id: &str) -> Option<&Logo> {
        for logo in &self.logos {
            if logo.identifier() == id {
                return Some(logo);
            }
        }
        None
    }

    /// Returns a mutable reference to the logo struct with the specified id.
    ///
    /// Also sets a flag to make all internal logo locations be recalculated before writing the file.
    pub fn mut_logo_with_id(&mut self, id: &str) -> Option<&mut Logo> {
        for logo in self.logos.iter_mut() {
            if logo.identifier() == id {
                self.inconsistent = true;
                return Some(logo);
            }
        }
        None
    }

    // TODO only borrow mutably F
    /// Extracts logo with specified id to anything that implements Write and Seek.
    /// Extension should be all lowercase already.
    pub fn extract_logo_with_id_to_file<F: Write + Seek>(
        &self,
        id: &str,
        outfile: &mut F,
        extension: &str,
    ) -> Result<(), LogoError> {
        let img = self.decode_logo_with_id(id)?;
        let format = match &extension[..] {
            "ico" => image::ICO,
            "jpg" | "jpeg" => image::JPEG,
            "png" => image::PNG,
            "bmp" => image::BMP,
            format => {
                return Err(IOError(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    &format!("Unsupported image format image/{:?}", format)[..],
                )))
            }
        };
        match img.save(outfile, format) {
            Ok(()) => Ok(()),
            Err(image::ImageError::IoError(err)) => Err(IOError(err)),
            Err(err) => Err(ImageError(err)),
        }
    }

    /// Returns the decoded logo with the specified id in the image::DynamicImage format.
    pub fn decode_logo_with_id(&self, id: &str) -> Result<image::DynamicImage, LogoError> {
        let logo = self.logo_with_id(id).ok_or(WrongIdentifier)?;
        codec::decode(self.family, logo.data(), logo.width(), logo.height())
    }

    /// Encodes the image and replaces the logo with the specified id with it.
    pub fn encode_to_logo_with_id(
        &mut self,
        img: image::DynamicImage,
        id: &str,
    ) -> Result<(), LogoError> {
        let width = img.dimensions().0;
        let height = img.dimensions().1;
        let data = codec::encode(self.family, img);
        let logo = self.mut_logo_with_id(id).ok_or(WrongIdentifier)?;
        logo.set_new_image(data, width, height);
        Ok(())
    }

    /// Returns DeviceFamily of the logo binary.
    pub fn family(&self) -> DeviceFamily {
        self.family
    }

    /// Returns MIME type/magic number of the logo binary.
    pub fn mime(&self) -> &str {
        &self.mime[..]
    }

    /// Return a reference to the Vector with all logos.
    pub fn logos(&self) -> &Vec<Logo> {
        &self.logos
    }

    /// Returns the size of the header of the binary file.
    pub fn header_size(&self) -> usize {
        self.header_size
    }

    // Seek is only used to assert_eq! sizes (header size and file size) for now
    /// Writes the logo binary to anything that implements Write and Seek.
    pub fn write_to_file<F: Write + Seek>(&mut self, new_file: &mut F) -> Result<(), LogoError> {
        if self.inconsistent {
            self.process_changes();
        }
        match self.family {
            DeviceFamily::MotoKitKat => moto_kit_kat::logo_bin_to_file(self, new_file),
            DeviceFamily::OnePlus3 => one_plus_3::logo_bin_to_file(self, new_file),
        }
    }
}

impl fmt::Display for Logo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}, location: {}, size: {}, width: {}, height: {}",
            self.identifier,
            self.location,
            self.data().len(),
            self.width,
            self.height
        )
    }
}

impl fmt::Display for LogoBin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "mime: {}\nheader size: {}\n",
            self.mime, self.header_size
        )?;
        for logo in &self.logos {
            write!(f, "{}\n", logo)?;
        }
        Ok(())
    }
}

// into must have at least target_size
fn usize_to_little_endian(data: usize, into: &mut [u8]) {
    (0..into.len()).into_iter().fold(data, |acc, i| {
        into[i] = acc as u8;
        acc >> 8
    });
}

fn u32_to_little_endian(data: u32, into: &mut [u8]) {
    (0..into.len()).into_iter().fold(data, |acc, i| {
        into[i] = acc as u8;
        acc >> 8
    });
}

// to be used in iter.fold() with (0, 0) as starting values
fn little_endian_to_usize((acc, i): (usize, usize), b: &u8) -> (usize, usize) {
    (acc + ((*b as usize) << (i * 8)), i + 1)
}

// // this is better
fn little_endian_to_u32(little_endian: [u8; 4]) -> u32 {
    let (ret, _) = little_endian.iter().fold((0, 0), |(acc, i), b| {
        (acc + ((*b as u32) << (i * 8)), i + 1)
    });
    ret
}
