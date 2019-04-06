import { init_devices, handle_file, handle_device, handle_image, export_logo_bin, default as init } from './alectrona_web.js';
import * as alectrona from './alectrona_web.js';

import * as dom_manipulator from './dom_manipulator.js';

async function run() {
  await init('./alectrona_web_bg.wasm');
  
  if (document.readyState == 'complete') {
    init_devices();
  } else {
    window.onload = () => {
      init_devices();
    }
  }
  
  window.handle_device = function (codename) {
    handle_device(codename);
    dom_manipulator.enable_bin_input();
    dom_manipulator.reset_file()
    dom_manipulator.reset_logo_list();
    dom_manipulator.reset_shown_image();
    dom_manipulator.reset_replace();
    dom_manipulator.reset_download();
  }
  
  window.handle_file = function (file) {
    const reader = new FileReader();
    reader.onload = (event) => {
      const buffer = new Uint8Array(event.target.result);
      try {
        handle_file(buffer);
        dom_manipulator.reset_shown_image();
        dom_manipulator.reset_replace();
        // dom_manipulator.reset_download();
        dom_manipulator.enable_download(file.name);
      } catch(error) {
        document.getElementById('input-file').value = '';
        alert('Could not parse logo_bin: ' + error);
      }
    };
    reader.readAsArrayBuffer(file);
  }
  
  window.handle_image = function (file) {
    const reader = new FileReader();
    reader.onload = (event) => {
      const buffer = new Uint8Array(event.target.result);
      try {
        handle_image(buffer, file.name);
        alert("Done!");
      } catch(error) {
        alert('Could not parse new image: ' + error);
      } finally {
        document.getElementById('input_file').value = '';        
      }
    };
    reader.readAsArrayBuffer(file);
  }
  
  window.export_logo_bin = function () {
    console.log("exporting logo.bin");
    const logoBinArray = export_logo_bin();
    const logoBinBlob = new Blob([logoBinArray], {type: "application/octet-stream"});
    const logoBinUrl = URL.createObjectURL(logoBinBlob);

    const downloader = document.getElementById('logo-bin-downloader');
    downloader.href = logoBinUrl;
    downloader.click();
  }
}

run();

window.alectrona = alectrona;
