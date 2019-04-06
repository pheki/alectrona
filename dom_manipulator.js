
export function reset_file() {
  document.getElementById("input-file").value = "";
}

// TODO translate set_logo_list from rust

export function set_logo_list(identifier_array) {
  const select_element = document.getElementById("select-logo-id");
  // TODO use a HTML template instead of dummy option
  const dummy_option = select_element.children.item(0);
  
  for (const identifier of identifier_array) {
    const new_option = dummy_option.cloneNode(true);
    new_option.removeAttribute("disabled");
    new_option.removeAttribute("selected");
    new_option.setAttribute("value", identifier);
    new_option.textContent = identifier;
    select_element.appendChild(new_option);
  }
  
  select_element.removeAttribute("disabled");
}

export function reset_logo_list() {
  const select_element = document.getElementById("select-logo-id");

  for (const child of Array.from(select_element.children)) {
    if (child.disabled === false) {
      console.log('1', child);
      child.remove();
      console.log('2', child);
    } else {
      child.selected = true;
    }
  }
  
  select_element.setAttribute("disabled", "");
}

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

export function enable_bin_input() {
  document.getElementById("input-file").removeAttribute("disabled");
}

export function reset_shown_image() {
  // Resets image properties to initial values
  const image_box = document.getElementsByClassName("image-box")[0]
  const img = image_box.getElementsByTagName("img")[0];  
  img.src = "imgs/placeholder.jpeg";
  img.onclick = () => { return; };
  img.classList.remove("downloadable");

  // Resets downloader properties to default values
  const image_downloader = document.getElementById("image-downloader");
  image_downloader.removeAttribute("download");
  image_downloader.href = "";
}

export function enable_replace() {
  const replace_input = document.getElementById("input-image");
  replace_input.removeAttribute("disabled");
}

export function reset_replace() {
  const replace_input = document.getElementById("input-image");
  replace_input.setAttribute("disabled", "");
}

export function enable_download(filename) {
  const replace_input = document.getElementById("logo-bin-download-button");
  replace_input.removeAttribute("disabled");
  
  // Sets downloader filename
  const downloader = document.getElementById('logo-bin-downloader');
  downloader.setAttribute("download", filename);
}

export function reset_download() {
  const replace_input = document.getElementById("logo-bin-download-button");
  replace_input.setAttribute("disabled", "");  
}
