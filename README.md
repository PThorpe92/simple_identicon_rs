# Yet another simple Identicon Generator

A simple Rust library/app that generates Identicons, which are unique visual representations of data such as usernames, email addresses, or any other input. This application serves Identicons as PNG images to the browser, but can also be used as a library and images can be saved .

An Identicon is a recognizable visual pattern that is generated based on the input data. It provides a quick and visually appealing way to represent data without revealing sensitive information.

## Features

- Generates Identicons based on input data.
- Produces Identicons as PNG images.
- Customizable parameters for image size and block size.

## Usage in code
```
use image::ImageBuffer;
use simple_identicon::identicon;

fn main() {
    // Generate an Identicon for the input data "username"
    let data = "username".as_bytes();
    let length = 256; // Image size
    let block_length = 32; // Block size

    let img: ImageBuffer<Rgba<u8>, Vec<u8>> = identicon(data, length, block_length);

    // Save the Identicon as a PNG image
    img.save("identicon.png").expect("Failed to save the Identicon image.");
}
```

## App usage
``` 
./simple_identicon --port {specify port} 
//default: 6660
```
in a browser, go to ```localhost:{port}/{username, ipaddr, etc}```
and your image will be displayed in the browser.

## Installation

To use the Identicon Generator in your Rust project, add the following dependency to your `Cargo.toml` file:

```toml
[dependencies]
image = "0.24"
xxhash-rust = "0.8"
