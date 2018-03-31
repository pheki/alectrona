*Disclaimer: I am NOT responsible for any damage that may arise from the use of this utility, including, but not limited to bricked phones, computers on fire or nuclear reactions.*

# alectrona

alectrona is a boot logo manipulator programmed in rust that was created with the intention to be device-agnostic. For now it can extract header information, images and "replace" the images on the file, generating a new valid logo.bin.

# Features

- Extracts header information and images from boot logo files.
- Replaces images in boot logo files.
- Device-agnostic: in other words, this utility **should** be able to work with the logo.bin format of any device.
- Free and open source.

# Known Problems

- For now, none!

# Devices

| Codename | Name | logo.bin "family" | Tested On Device | "Virtually" tested |
| --- | --- | --- | --- | --- |
| harpia | Moto G4 Play | MotoKitKat | Yes | Yes |
| lux | Moto X Play | MotoKitKat | No | Yes |
| oneplus3 | OnePlus 3 | OnePlus3 | No | Yes |

# Installation Steps

For now, to install you need cargo (which comes with rustup). Just follow the installation steps at https://rustup.rs/ and then run ```cargo install https://github.com/pheki/alectrona.git```.

Make sure you have rust's bin folder ($HOME/.cargo/bin) on your path.

# Usage
Note: this program will overwrite files by default, so be careful with your filenames.

There are 4 possible subcommands to use with the alectrona-cli package:
- header: prints the file header
- extract: extracts a single logo from the file
- extract-all: extracts all logos from the file to a directory (folder)
- replace: replaces some logos in the file with provided images

Every one of them can be used with the following syntax:
```
alectrona -d <DEVICE_CODENAME> -i <ORIGINAL_LOGO_PATH.BIN> <subcommand> ...
```

## Get header info

Syntax:
```
alectrona -d <DEVICE_CODENAME> -i <ORIGINAL_LOGO_PATH.BIN> header
```
Example:
```
alectrona -d harpia -i logo_original.bin header
```

## Extract single image
Syntax:
```
alectrona -d <DEVICE_CODENAME> -i <ORIGINAL_LOGO_PATH.BIN> extract <LOGO_IDENTIFIER> --output <OUTPUT_FILE>
```

Example:
```
alectrona -d harpia -i logo_original.bin extract logo_boot -o logo_boot.png
```

## Extract all images
Syntax:
```
alectrona -d <DEVICE_CODENAME> -i <ORIGINAL_LOGO_PATH.BIN> extract-all --output <OUTPUT_DIRECTORY>
```

Example:
```
alectrona -d harpia -i logo_original.bin extract-all -o extraction_folder/
```

## Replace logo images
NOTE: This does NOT resize images BY DEFAULT, and flashing a boot logo with images with the wrong size may have unexpected results depending on your device. Please be careful.

Syntax:
```
alectrona -d <DEVICE_CODENAME> -i <ORIGINAL_LOGO_PATH.BIN> [--resize] replace <LOGO_IDENTIFIER INPUT_IMAGE>... --output <OUTPUT_FILE>
```

For each LOGO_IDENTIFIER INPUT_IMAGE pair, the logo with identifier LOGO_IDENTIFIER will be replaced with the image INPUT_IMAGE.
In this case, -r or --resize can be used BEFORE (this will probably change in the future) replace and the images will be resized to the device size.

Example:
```
alectrona -d harpia -i logo_original.bin replace logo_boot new_boot_logo.png logo_battery new_battery_image.png -o new_logo.bin
```

# Tests

Some tests are realized with boot logos from other devices, but as the distribution of them might not be legal, I won't publish the logos or the tests:

(tests to be documented here)
