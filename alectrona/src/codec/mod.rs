use super::DeviceFamily;
use crate::LogoError;

mod moto_kit_kat;
mod one_plus_2;

pub fn decode(
    family: DeviceFamily,
    data: &[u8],
    width: u32,
    height: u32,
) -> Result<image::DynamicImage, LogoError> {
    match family {
        DeviceFamily::MotoKitKat => moto_kit_kat::decode(data, width, height),
        DeviceFamily::OnePlus2 => one_plus_2::decode(data, width, height),
        // _ => Err(UnsupportedDevice),
    }
}

pub fn encode(family: DeviceFamily, img: image::DynamicImage) -> Vec<u8> {
    match family {
        DeviceFamily::MotoKitKat => moto_kit_kat::encode(img),
        DeviceFamily::OnePlus2 => one_plus_2::encode(img),
        // _ => Err(UnsupportedDevice),
    }
}
