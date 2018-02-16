extern crate image;

pub mod data;
mod codec;

#[cfg(test)]
mod tests;

pub use data::LogoBin;

use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::fs::OpenOptions;
use std::error::Error;
use std::convert;
use std::str::FromStr;
use LogoError::*;

use std::path::Path;

#[derive(Copy, Clone, Debug)]
pub enum DeviceFamily {
    MotoKitKat,
    OnePlus3,
    // Custom(String),
}

impl FromStr for DeviceFamily {
    type Err = LogoError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "MotoKitKat" => Ok(DeviceFamily::MotoKitKat),
            "OnePlus3" => Ok(DeviceFamily::OnePlus3),
            _ => Err(UnsupportedDevice),
        }
    }
}

/// Defines actions for the main program to do
pub enum Action {
    // extract should need logo_identifier and extraction_path
    Extract(String, String),
    ExtractAll(String),
    GetLogoBin,
    // replace needs logo_identifier, replace_image_path and new_logo_path
    Replace(HashMap<String, String>, String),
}

pub struct Config {
    pub action: Action,
    // should be a Path? AsRef<Path?>
    pub input_filename: String,
    pub device_family: Option<DeviceFamily>,
    pub overwrite: bool,
    // not yet used field
    pub resize: Option<(u32, u32)>,
}


pub fn run(config: Config) -> Result<Option<LogoBin>, LogoError> {


    let mut infile = File::open(config.input_filename)?;
    let device_family = config.device_family.unwrap_or(DeviceFamily::MotoKitKat);
    let mut logo_bin = LogoBin::from_file(&mut infile, device_family)?;


    match config.action {
        Action::GetLogoBin => return Ok(Some(logo_bin)),
        Action::Extract(ref id, ref outfilename) => {
            let outpath = Path::new(&outfilename);
            let outfile = create_file(&outpath, config.overwrite)?;
            let extension = outpath.extension().and_then(|s| s.to_str())
                                   .map_or("".to_string(), |s| s.to_lowercase());
            logo_bin.extract_logo_with_id_to_file(id, outfile, &extension)?;
        },
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
                let mut outpath = directory_path.join(Path::new(id).file_name().unwrap_or("invalid_filename".as_ref()));
                assert!(outpath.set_extension("png"));
                let outfile = create_file(&outpath, config.overwrite)?;
                let extension = outpath.extension().and_then(|s| s.to_str())
                                       .map_or("".to_string(), |s| s.to_lowercase());

                logo_bin.extract_logo_with_id_to_file(id, outfile, &extension)?;
            }
        },
        Action::Replace(replace_map, outfilename) => {
            for (id, image_location) in replace_map.into_iter() {
                let img = image::open(image_location).expect("Could not open image");
                logo_bin.encode_to_logo_with_id(img, &id)?;
            }
            logo_bin.process_changes();

            let outpath = Path::new(&outfilename);
            let outfile = create_file(outpath, config.overwrite)?;
            logo_bin.write_to_file(outfile)?;

        }
    }
    Ok(None)

}


fn create_file(path: &Path, overwrite: bool) -> Result<File, LogoError> {
    match OpenOptions::new()
                      .create_new(true)
                      .write(true)
                      .open(path)
    {
        Ok(file) => Ok(file),
        Err(ref err) if err.kind() == io::ErrorKind::AlreadyExists => {
            if overwrite {
                File::create(path).map_err(|err| IOError(err))
            } else {
                Err(WouldOverwrite)
            }
        },
        Err(err) => Err(IOError(err)),
    }
}

#[derive(Debug)]
pub enum LogoError {
    #[allow(dead_code)]
    UnsupportedDevice,
    WrongImageMagicNumber,
    WrongIdentifier,
    WrongImageFormat,
    WrongImageSize,
    IOError(std::io::Error),
    ImageError(image::ImageError),
    WouldOverwrite,
    TooBig,
    NotADirectory,
}

impl std::fmt::Display for LogoError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(fmt, "Error: {}", self.description())?;
        if let Some(cause) = self.cause() {
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

    fn cause(&self) -> Option<&std::error::Error> {
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
