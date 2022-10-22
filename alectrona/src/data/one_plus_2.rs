use std::borrow::Cow;
use std::io::prelude::*;
use std::io::SeekFrom;

use DeviceFamily::OnePlus2;

use crate::data::*;

const NAME_SIZE: usize = 64;
const IDENTIFIER_SIZE: usize = 288;
const MIME: &str = "SPLASH!!";

pub fn logo_bin_from_file<F: Read + Seek>(file: &mut F) -> Result<LogoBin, LogoError> {
    file.seek(SeekFrom::Start(0))?;
    let mut mime = [0u8; 8];
    file.read_exact(&mut mime)?;
    let mime = String::from_utf8_lossy(&mime).into_owned();
    if &mime[..] != MIME {
        return Err(WrongImageMagicNumber);
    }

    file.seek(SeekFrom::Start(48))?;
    let mut offsets: Vec<usize> = Vec::new();
    let mut offset = [0u8; 4];
    for _ in 0..28 {
        file.read_exact(&mut offset)?;
        let (offset, _) = offset.iter().fold((0, 0), little_endian_to_usize);
        offsets.push(offset);
    }

    // Checks size of buffer by verifying start of run-length encoding (first image)
    // The second byte should not be 0 on start of image, as it would mean
    // that the first byte would be repeated 0 times
    let mut buffer = [0u8; 2];
    let mut header_size: Option<usize> = None;
    for position in (0x0..0x10).map(|o| 0x100 + o * 0x100) {
        file.seek(SeekFrom::Start(position))?;
        file.read_exact(&mut buffer)?;
        if buffer[1] != 0 {
            header_size = Some(position as usize);
            break;
        }
    }
    let header_size = header_size.ok_or(WrongImageFormat)?;

    let mut logos = Vec::new();
    for offset in offsets {
        file.seek(SeekFrom::Start(offset as u64 + 32))?;
        let mut width = [0u8; 4];
        file.read_exact(&mut width)?;
        let width = little_endian_to_u32(width);

        let mut height = [0u8; 4];
        file.read_exact(&mut height)?;
        let height = little_endian_to_u32(height);

        let mut size = [0u8; 4];
        file.read_exact(&mut size)?;
        let (size, _) = size.iter().fold((0, 0), little_endian_to_usize);

        let mut some_number = [0u8; 4];
        file.read_exact(&mut some_number)?;
        let some_number = little_endian_to_u32(some_number);
        // we should check for this when changing the logo later probably
        // now I'm supposing that all logos that have data also have this number == 1,
        // the other ones have this number == 0

        assert!(if size != 0 {some_number == 1} else {some_number == 0},
            "your logo file seems to be different than the ones used for testing, please report it to the developer");

        file.seek(SeekFrom::Current(28 * 4))?;

        let mut _name = [0u8; 64];
        file.read_exact(&mut _name)?;

        let mut identifier = [0u8; 288];
        file.read_exact(&mut identifier)?;
        let identifier: Vec<u8> = identifier
            .iter()
            .take_while(|&&b| b != 0)
            .cloned()
            .collect();
        let identifier = match String::from_utf8_lossy(&identifier[..]) {
            Cow::Borrowed(b) => b.to_owned(),
            Cow::Owned(b) => b,
        };

        file.seek(SeekFrom::Start(offset as u64 + header_size as u64))?;

        let mut data = Vec::with_capacity(size);
        if size > 0 {
            let mut data_file = file.take(size as u64);
            data_file.read_to_end(&mut data)?;
        }
        debug_assert!(data.len() == size);

        logos.push(Logo {
            identifier,
            location: offset,
            width,
            height,
            data,
        })
    }

    Ok(LogoBin {
        family: OnePlus2,
        mime,
        header_size,
        logos,
        inconsistent: false,
    })
}

pub fn process_changes(logo_bin: &mut LogoBin) {
    let mut last_used = None;
    for logo in logo_bin.logos.iter_mut() {
        let location = match last_used {
            None => 0,
            Some(last_used) => {
                last_used + (logo_bin.header_size - (last_used % logo_bin.header_size))
            }
        };
        logo.set_location(location);
        last_used = Some(location + logo_bin.header_size + logo.data().len() - 1);
    }
}

pub fn logo_bin_to_file<F: Write + Seek>(
    logo_bin: &mut LogoBin,
    new_file: &mut F,
) -> Result<(), LogoError> {
    for logo in logo_bin.logos() {
        let fill_data = vec![0u8; logo.location() - new_file.seek(SeekFrom::Current(0))? as usize];
        new_file.write_all(&fill_data[..])?;

        let has_data = !logo.data.is_empty();
        // writes mime type
        let buf = if has_data { b"SPLASH!!" } else { &[0u8; 8] };
        new_file.write_all(&buf[..])?;
        let buf = [0u8; 24];
        new_file.write_all(&buf[..])?;

        let mut buf = [0u8; 4];
        u32_to_little_endian(logo.width(), &mut buf);
        new_file.write_all(&buf[..])?;

        u32_to_little_endian(logo.height(), &mut buf);
        new_file.write_all(&buf[..])?;

        usize_to_little_endian(logo.data().len(), &mut buf);
        new_file.write_all(&buf[..])?;

        // writes "some_number"
        let some_number = if has_data { 1 } else { 0 };
        u32_to_little_endian(some_number, &mut buf);
        new_file.write_all(&buf[..])?;

        // writes offsets
        if logo.location() == 0 {
            let mut offset = [0u8; 4];
            assert_eq!(logo_bin.logos().len(), 28,
                "your logo file seems to be different than the ones used for testing, please report it to the developer");
            for logo in logo_bin.logos() {
                u32_to_little_endian(logo.location() as u32, &mut offset);
                new_file.write_all(&offset[..])?;
            }
        } else {
            let buf = [0u8; 4];
            for _ in 0..28 {
                new_file.write_all(&buf[..])?;
            }
        }

        // guess name from file properties...
        let name = if has_data {
            format!(
                "{}_{}_{}_result.raw",
                logo.identifier(),
                logo.width(),
                logo.height
            )
            .into_bytes()
        } else {
            Vec::new()
        };
        new_file.write_all(&name[..])?;
        let name = vec![0u8; NAME_SIZE - name.len()];
        new_file.write_all(&name[..])?;

        let identifier = logo.identifier().as_bytes();
        new_file.write_all(identifier)?;
        let identifier = vec![0u8; IDENTIFIER_SIZE - name.len()];
        new_file.write_all(&identifier[..])?;

        let fill_header = vec![
            0u8;
            logo_bin.header_size
                - (new_file.seek(SeekFrom::Current(0))? as usize
                    - logo.location())
        ];
        new_file.write_all(&fill_header)?;

        new_file.write_all(logo.data())?;
    }

    // checks if file is larger than 16 MiB
    let filesize = new_file.seek(SeekFrom::End(0))?;
    if filesize > 16_777_220 {
        Err(TooBig)
    } else {
        Ok(())
    }
}
