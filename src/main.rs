extern crate alectrona;
extern crate clap;
extern crate toml;

use alectrona::{Config, Action, run, DeviceFamily};
use clap::{Arg, App, SubCommand};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use toml::Value;

use std::str::FromStr;

use std::process;

fn main() {
    let matches = App::new("bootlogomix")
        .arg(Arg::with_name("input")
            .short("i")
            .long("input")
            .value_name("INPUT_FILE")
            .required(true)
            .help("Input binary logo file")
            .takes_value(true)
        )
        .arg(Arg::with_name("device")
            .short("d")
            .long("device")
            .value_name("DEVICE_CODENAME")
            .help("Codename of the device codename correspondent to the binary boot logo file")
            .takes_value(true)
        )
        .subcommand(SubCommand::with_name("extract")
            .arg(Arg::with_name("identifier")
                .value_name("LOGO_IDENTIFIER")
                .required(true)
                .help("Sets to extract the logo with this identifier")
                .takes_value(true)
            )
            .arg(Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("OUTPUT_FILE")
                .required(true)
                .help("Output binary logo file")
                .takes_value(true)
            )
        )
        .subcommand(SubCommand::with_name("extract-all")
            .arg(Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("OUTPUT_FOLDER")
                .required(true)
                .help("Output folder for images")
                .takes_value(true)
            )
        )
        .subcommand(SubCommand::with_name("header")
            .help("Sets to print the header with all identifiers")
        )
        .subcommand(SubCommand::with_name("replace")
            .arg(Arg::with_name("identifier")
                .value_name("LOGO_IDENTIFIER INPUT_IMAGE")
                .required(true)
                .help("Sets the INPUT_IMAGE to replace logo with LOGO_IDENTIFIER, can be defined multiple times")
                .takes_value(true)
                .multiple(true)
            )
            .arg(Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("OUTPUT_FILE")
                .required(true)
                .help("Output binary logo file")
                .takes_value(true)
            )
        )
        .get_matches();


    let infile = matches.value_of("input").unwrap().to_string();
    let action = match matches.subcommand() {
        ("header", Some(_)) => {
            Action::GetLogoBin
        },

        ("extract", Some(submatches)) => {
            Action::Extract(
                submatches.value_of("identifier").unwrap().to_string(),
                submatches.value_of("output").unwrap().to_string(),
            )
        },

        ("extract-all", Some(submatches)) => {
            Action::ExtractAll(
                submatches.value_of("output").unwrap().to_string(),
            )
        },

        ("replace", Some(submatches)) => {
            let to_replace = submatches.values_of("identifier").unwrap()
                                       .zip(submatches.values_of("identifier").unwrap().skip(1))
                                       .enumerate()
                                       .filter(|x| x.0 % 2 == 0) // ignores "odd" values
                                       .map(|x| x.1)
                                       .fold(HashMap::new(), |mut map, x|  {
                                           map.insert(x.0.to_string(), x.1.to_string());
                                           map
                                       });
            Action::Replace(
                to_replace,
                submatches.value_of("output").unwrap().to_string(),
            )

        },

        _ => {
            eprintln!("You need to specify an action.\nUse -h or --help for help.");
            std::process::exit(-1);
        },
    };

    let (device_family, resize) = if let Some(device_str) = matches.value_of("device") {
        // terrible error handling for now.......
        let mut devices_string = String::new();
        File::open("devices.toml").expect("Could not open devices.toml")
            .read_to_string(&mut devices_string).expect("Could not read devices.toml");
        let devices = Value::from_str(&devices_string).expect("invalid devices.toml");
        let device = devices.get(device_str).expect("UnsupportedDevice");

        let device_family = {
                DeviceFamily::from_str(
                    device["family"].as_str().expect("device.family is not a string")
                ).expect("device family not supported")
        };
        let resize = {
                (device["width"].as_integer().unwrap() as u32,
                device["height"].as_integer().unwrap() as u32
                )
        };
        (Some(device_family), Some(resize))
    } else {
        (None, None)
    };

    // println!("{:?}", device);

    let config = Config {
        action,
        input_filename: String::from(infile),
        device_family,
        overwrite: true,
        resize,
    };
    match run(config) {
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1);
        },
        Ok(Some(logo_bin)) => {
            println!("{}", logo_bin);
        },
        Ok(None) => (),
    };
}
