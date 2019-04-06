export function show_image(outArray, extension, logo_id = 'image') {
  const blob = new Blob([outArray], {type: "image/" + extension});
  const url = URL.createObjectURL(blob);

  // Makes this blob the img src
  const image_box = document.getElementsByClassName("image-box")[0]
  const img = image_box.getElementsByTagName("img")[0];
  img.src = url;
  img.onclick = () => { document.getElementById('image-downloader').click() };
  img.classList.add("downloadable");
  
  // Makes clicking the image download it
  const image_downloader = document.getElementById("image-downloader");
  image_downloader.setAttribute("download", logo_id + "." + extension);
  image_downloader.href = url;
}

export function set_step(step_number) {
  switch (step_number) {
    case 1:
      break;
    case 2:
      document.getElementById("input-file").removeAttribute("disabled");
      break;
    case 3:
      break;
    case 4:
      break;
    case 5:
      break;
    case 6:
      break;
    default:
      
  }
}

export function enable_replace() {
  const replace_input = document.getElementById("input-image");
  replace_input.removeAttribute("disabled");
}

export function enable_download() {
  const replace_input = document.getElementById("logo-bin-download-button");
  replace_input.removeAttribute("disabled");
}
