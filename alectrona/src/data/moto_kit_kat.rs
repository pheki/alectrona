use std::borrow::Cow;
use std::io;
use std::io::prelude::*;
use std::io::SeekFrom;

use crate::LogoError;
use LogoError::*;

use DeviceFamily::MotoKitKat;

use crate::data::*;

const LOGO_HEADER_SIZE: usize = 8 + 2 + 2;
const MIME: &str = "MotoLogo\0";

// there are lots of constants inlined, should be consts...
pub fn logo_bin_from_file<F: Read + Seek>(file: &mut F) -> Result<LogoBin, LogoError> {
    file.seek(SeekFrom::Start(0))?;
    let mut mime = [0; 9];
    file.read_exact(&mut mime)?;
    let mime = String::from_utf8_lossy(&mime).into_owned();
    if &mime[..] != MIME {
        return Err(WrongImageMagicNumber);
    }

    let mut size = [0; 4];
    file.read_exact(&mut size)?;
    let (size, _) = size.iter().fold((0, 0), little_endian_to_usize);

    // enforces header size
    let mut header = Vec::with_capacity(size);
    file.take(size as u64 - 13).read_to_end(&mut header)?;
    let mut header = &header[..];
    let mut logos = Vec::new();
    loop {
        let mut identifier = [0; 24];
        match header.read_exact(&mut identifier) {
            Err(ref err) if err.kind() == io::ErrorKind::UnexpectedEof => break,
            Err(err) => return Err(IOError(err)),
            _ => (),
        }

        let identifier: Vec<u8> = identifier
            .iter()
            .take_while(|&&b| b != 0)
            .cloned()
            .collect();

        // let identifier = String::from_utf8(identifier).unwrap();
        let identifier = match String::from_utf8_lossy(&identifier[..]) {
            Cow::Borrowed(b) => b.to_owned(),
            Cow::Owned(b) => b,
        };

        let mut location = [0; 4];
        header.read_exact(&mut location)?;
        // this panic may happen in end of header, should watch closely
        let (location, _) = location.iter().fold((0, 0), little_endian_to_usize);

        let mut image_size = [0; 4];
        header.read_exact(&mut image_size)?;
        // this panic may happen in end of header, should watch closely
        let (image_size, _) = image_size.iter().fold((0, 0), little_endian_to_usize);
        let logo = extract_logo(file, location, image_size, identifier)?;

        logos.push(logo);
    }

    Ok(LogoBin {
        family: MotoKitKat,
        mime,
        header_size: size,
        logos,
        inconsistent: false,
    })
}

pub fn process_changes(logo_bin: &mut LogoBin) {
    let mut last_used = logo_bin.header_size - 1;
    for logo in logo_bin.logos.iter_mut() {
        let location = last_used + (0x200 - (last_used % 0x200));
        logo.set_location(location);
        last_used = location + LOGO_HEADER_SIZE + logo.data().len() - 1;
    }
}

pub fn logo_bin_to_file<F: Write + Seek>(
    logo_bin: &LogoBin,
    new_file: &mut F,
) -> Result<(), LogoError> {
    // writes header only
    // writes mime type
    let buf = b"MotoLogo\0";
    new_file.write_all(&buf[..])?;
    // writes header size
    let mut buf = [0u8; 4];
    usize_to_little_endian(logo_bin.header_size(), &mut buf);
    new_file.write_all(&buf[..])?;

    // writes the logos part of the header
    for logo in logo_bin.logos() {
        let mut buf = [0u8; 24];
        let identifier_bytes = logo.identifier().as_bytes();
        for (place, data) in buf.iter_mut().zip(identifier_bytes.iter()) {
            *place = *data;
        }
        new_file.write_all(&buf[..])?;
        let mut buf = [0u8; 4];

        usize_to_little_endian(logo.location(), &mut buf);
        new_file.write_all(&buf[..])?;

        usize_to_little_endian(logo.data().len() + LOGO_HEADER_SIZE, &mut buf);
        new_file.write_all(&buf[..])?;
    }

    let mut current_position = logo_bin.header_size();

    assert_eq!(
        new_file
            .seek(SeekFrom::Current(0))
            .expect("Could not seek in assert_eq...") as usize,
        current_position
    );
    // finished writing header

    // writes images
    for logo in logo_bin.logos() {
        // padding with zeroes
        let padding_space = logo.location() - current_position;
        let empty = vec![0xff; padding_space];
        new_file.write_all(&empty[..])?;
        current_position += padding_space;
        // write image data
        new_file.write_all("MotoRun\0".as_bytes())?;
        let buf = [(logo.width() >> 8) as u8, logo.width() as u8];
        new_file.write_all(&buf)?;

        let buf = [(logo.height() >> 8) as u8, logo.height() as u8];
        new_file.write_all(&buf)?;

        new_file.write_all(logo.data())?;
        current_position += logo.data().len() + LOGO_HEADER_SIZE;
    }

    let filesize = new_file.seek(SeekFrom::End(0))?;
    if filesize > 4_194_304 {
        Err(TooBig)
    } else {
        Ok(())
    }
}

// returns logo with compressed image data
fn extract_logo<F: Read + Seek>(
    file: &mut F,
    location: usize,
    size: usize,
    identifier: String,
) -> Result<Logo, LogoError> {
    file.seek(SeekFrom::Start(location as u64))?;
    let mut data_file = file.take(size as u64);

    let mut check = [0u8; 8];
    data_file.read_exact(&mut check)?;
    match std::str::from_utf8(&check) {
        Ok(d) if d == "MotoRun\0" => (),
        _ => return Err(WrongImageMagicNumber),
    }

    let mut width = [0u8; 2];
    data_file.read_exact(&mut width)?;
    let width = width.iter().fold(0, |acc, &b| (acc << 8) + b as u32);

    let mut height = [0u8; 2];
    data_file.read_exact(&mut height)?;
    let height = height.iter().fold(0, |acc, &b| (acc << 8) + b as u32);

    let mut data = Vec::with_capacity(size - LOGO_HEADER_SIZE);
    data_file.read_to_end(&mut data)?;
    debug_assert!(data.len() == (size - LOGO_HEADER_SIZE));

    Ok(Logo {
        identifier,
        location,
        width,
        height,
        data,
    })
}
