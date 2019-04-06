/* tslint:disable */
/**
* Initializes devices using POSSIBLE_DEVICES, creating an <option> element on <select id=\"select-device\"> for each device.
* @returns {void} 
*/
export function init_devices(): void;
/**
* Saves selected device and enables the binary input field.
* @param {string} codename 
* @returns {void} 
*/
export function handle_device(codename: string): void;
/**
* Parses the logo.bin file, saving it on SELECTED_LOGO_BIN and creating the list of logo_ids to select.
* @param {Uint8Array} buffer 
* @returns {string} 
*/
export function handle_file(buffer: Uint8Array): string;
/**
* Extracts the image with the id selected and shows it on the <img> element.
* @param {string} logo_id 
* @returns {void} 
*/
export function handle_logo_id(logo_id: string): void;
/**
* Replaces image with the selected logo_id in the logo.bin file.
* @param {Uint8Array} buffer 
* @param {string} filename 
* @returns {void} 
*/
export function handle_image(buffer: Uint8Array, filename: string): void;
/**
* Exports the new logo.bin file for download.
* @returns {Uint8Array} 
*/
export function export_logo_bin(): Uint8Array;
