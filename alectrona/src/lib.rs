#![warn(missing_docs)]
//! You can find the README at README.md

pub extern crate image;

#[cfg(feature = "serde_")]
extern crate serde;

#[cfg(feature = "serde_")]
#[macro_use]
extern crate serde_derive;

mod codec;
/// This module contains data structures for the binary files and the logos inside them.
pub mod data;

pub use data::LogoBin;

use std::collections::HashMap;
use std::convert;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::str::FromStr;
use LogoError::*;

use std::path::Path;

use image::FilterType;
use image::GenericImageView;

/// Default devices.toml file with all known devices.
pub static DEVICES_TOML: &str = include_str!("devices.toml");

/// DeviceFamily of the device related to the boot logo binary.
#[cfg_attr(feature = "serde_", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug)]
pub enum DeviceFamily {
    /// This family is for Motorola devices that came with Android KitKat (4.4) or newer installed.
    /// Those devices should all have the same format for the logo binary (logo.bin) file.
    ///
    /// (I am not sure about devices that have been updated from older versions to KitKat).
    MotoKitKat,
    /// This family is for the OnePlus 2 and OnePlus 3 (it could possibly work for OnePlus 3T devices, but I'm not sure).
    OnePlus2,
    // Custom(String),
}

impl FromStr for DeviceFamily {
    type Err = LogoError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "MotoKitKat" => Ok(DeviceFamily::MotoKitKat),
            "OnePlus2" => Ok(DeviceFamily::OnePlus2),
            _ => Err(UnsupportedDevice),
        }
    }
}

/// Defines actions for the [`run`] function.
pub enum Action {
    /// Extracts the image with the identifier to the path.
    Extract(String, String),
    /// Extract all images in the binary file to the directory in the path.
    ExtractAll(String),
    /// Gets the data::LogoBin struct
    GetLogoBin,
    /// Replace takes a HashMap of logo identifiers, replace image paths and the new logo.bin path.
    Replace(HashMap<String, String>, String),
}

/// Configuration to be used with [`run`];
pub struct Config {
    /// Value of the configuration action.
    pub action: Action,
    /// String that represents input filename
    pub input_filename: String,
    /// "Family" of the device related to the boot logo binary.
    pub device_family: Option<DeviceFamily>,
    /// Defines if should overwrite output file(s)
    pub overwrite: bool,
    // Yes, this should be inside the replace subcommand and inside the replace action, but it's not for now
    /// If Some((width, height)), resize images to the (width, height) dimensions if replacing logo
    /// and they're not in those dimensions already. Uses the Lanczos3 filter.
    pub resize: Option<(u32, u32)>,
}

/// Runs the program, based on the config struct.
pub fn run(config: Config) -> Result<Option<LogoBin>, LogoError> {
    let mut infile = File::open(config.input_filename)?;
    let device_family = config.device_family.unwrap_or(DeviceFamily::MotoKitKat);
    let mut logo_bin = LogoBin::from_file(&mut infile, device_family)?;

    match config.action {
        Action::GetLogoBin => return Ok(Some(logo_bin)),
        Action::Extract(ref id, ref outfilename) => {
            let outpath = Path::new(&outfilename);
            let mut outfile = create_file(&outpath, config.overwrite)?;
            let extension = outpath
                .extension()
                .and_then(|s| s.to_str())
                .map_or("".to_string(), |s| s.to_lowercase());
            logo_bin.extract_logo_with_id_to_file(id, &mut outfile, &extension)?;
        }
        Action::ExtractAll(ref outdirectoryname) => {
            let directory_path = Path::new(outdirectoryname);
            if !directory_path.is_dir() {
                return Err(NotADirectory);
            }
            for logo in logo_bin.logos() {
                if logo.width() == 0 || logo.height() == 0 {
                    continue;
                }
                let id = logo.identifier();
                let mut outpath = directory_path.join(
                    Path::new(id)
                        .file_name()
                        .unwrap_or("invalid_filename".as_ref()),
                );
                assert!(outpath.set_extension("png"));
                let mut outfile = create_file(&outpath, config.overwrite)?;
                let extension = outpath
                    .extension()
                    .and_then(|s| s.to_str())
                    .map_or("".to_string(), |s| s.to_lowercase());

                logo_bin.extract_logo_with_id_to_file(id, &mut outfile, &extension)?;
            }
        }
        Action::Replace(replace_map, outfilename) => {
            for (id, image_location) in replace_map.into_iter() {
                let mut img = image::open(image_location).expect("Could not open image");
                // shadows image, resizing if config.resize is set
                if let Some(dimensions) = config.resize {
                    // only resizes if the resize dimension is different than the actual one
                    if dimensions != img.dimensions() {
                        img = img.resize_exact(dimensions.0, dimensions.1, FilterType::Lanczos3);
                    }
                };
                logo_bin.replace_logo_with_id(img, &id)?;
            }

            let outpath = Path::new(&outfilename);
            let mut outfile = create_file(outpath, config.overwrite)?;
            logo_bin.write_to_file(&mut outfile)?;
        }
    }
    Ok(None)
}

fn create_file(path: &Path, overwrite: bool) -> Result<File, LogoError> {
    match OpenOptions::new().create_new(true).write(true).open(path) {
        Ok(file) => Ok(file),
        Err(ref err) if err.kind() == io::ErrorKind::AlreadyExists => {
            if overwrite {
                File::create(path).map_err(IOError)
            } else {
                Err(WouldOverwrite)
            }
        }
        Err(err) => Err(IOError(err)),
    }
}

/// Error type for functions in the alectrona module.
#[derive(Debug)]
pub enum LogoError {
    #[allow(dead_code)]
    /// Error thrown when the device defined is not supported.
    UnsupportedDevice,
    /// Error thrown when the image MIME type does not match.
    WrongImageMagicNumber,
    /// Error thrown when the image identifier does not match.
    WrongIdentifier,
    /// Error thrown when the image is not on the expected format.
    WrongImageFormat,
    /// Decoded image does not have the expected image size.
    WrongImageSize,
    /// Input/output error
    IOError(io::Error),
    /// There was an error in the image library.
    ImageError(image::ImageError),
    /// Would overwrite output file when config.overwrite is set to false.
    WouldOverwrite,
    /// Generated file is to big for device.
    TooBig,
    /// When extracting all files, output path is not a directory.
    NotADirectory,
}

impl std::fmt::Display for LogoError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        writeln!(fmt, "Error: {}", self.description())?;
        if let Some(cause) = self.source() {
            writeln!(fmt, "Cause: {}", cause)
        } else {
            Ok(())
        }
    }
}

impl std::error::Error for LogoError {
    fn description(&self) -> &str {
        match *self {
            UnsupportedDevice => "Device not supported",
            WrongImageMagicNumber => "The image magic number does not match",
            WrongIdentifier => "The logo identifier does not match",
            WrongImageFormat => "There is something wrong in the image format",
            WrongImageSize => "The image decoded into the wrong size",
            IOError(_) => "There was an i/o error",
            ImageError(_) => "There was an error in the image library",
            WouldOverwrite => "Would overwrite a file",
            TooBig => "Generated file is too big",
            NotADirectory => "File path does not exist or is not a directory",
        }
    }

    fn source(&self) -> Option<&(Error + 'static)> {
        match *self {
            IOError(ref cause) => Some(cause),
            ImageError(ref cause) => Some(cause),
            _ => None,
        }
    }
}

impl convert::From<io::Error> for LogoError {
    fn from(error: io::Error) -> Self {
        IOError(error)
    }
}
