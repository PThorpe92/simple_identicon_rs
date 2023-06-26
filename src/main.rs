use clap::{Arg, Command};
use image::codecs::png::PngEncoder;
use image::{ImageBuffer, ImageEncoder, Pixel, PixelWithColorType, Rgba};
use std::io::Read;
use std::io::Write;
use std::iter::repeat;
use std::net::{TcpListener, TcpStream};
use std::str;
use xxhash_rust::xxh3::xxh3_64;

fn main() {
    let matches = Command::new("Simple Identicon")
        .author("PThorpe92")
        .about("Serves Identicon images on a specified port")
        .arg(
            Arg::new("port")
                .short('p')
                .long("port")
                .value_name("PORT")
                .help("Sets the port number")
                .default_value("6660"),
        )
        .get_matches();

    let port = matches.get_one::<String>("port").unwrap();

    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();
    println!("Server started on port {}", port);
    for stream in listener.incoming() {
        let stream = stream.expect("Failed to establish connection");
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).expect("Failed to read stream");
    let request = str::from_utf8(&buffer).unwrap();

    let identicon = identicon(request.as_bytes(), 540, 60);
    let response = create_response(identicon);

    stream
        .write_all(response.as_bytes())
        .expect("Failed to write response");
    stream.flush().expect("Failed to flush stream");
}

fn identicon(data: &[u8], width: u32, block_length: u32) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let digest = xxh3_64(data);
    let mut img = ImageBuffer::from_pixel(
        width,
        width,
        Rgba([
            digest as u8,
            (digest >> 8) as u8,
            (digest >> 16) as u8,
            0xff,
        ]),
    );

    let mut columns_count = width / block_length;
    let mut padding = block_length / 2;
    if width % block_length != 0 {
        padding = (width - block_length * columns_count) / 2;
    } else if columns_count > 1 {
        columns_count -= 1;
    } else {
        padding = 0;
    }

    let filled = columns_count == 1;
    let pixels = repeat(1).take(block_length as usize).collect::<Vec<u8>>();
    let mut ri = 0;
    let mut ci = 0;
    for i in 0..(columns_count * (columns_count + 1) / 2) {
        if filled || (digest >> (i % 64)) & 1 == 1 {
            for j in 0..block_length {
                let x = padding + ri * block_length;
                let y = padding + ci * block_length + j;
                let start_x = x - block_length / 2;
                let start_y = y - block_length / 2;
                let end_x = x + block_length / 2;
                let end_y = y + block_length / 2;

                for px in start_x..=end_x {
                    for py in start_y..=end_y {
                        let offset = img.get_pixel_mut(px, py);
                        for (channel, pixel) in offset.channels_mut().iter_mut().zip(&pixels) {
                            *channel = *pixel;
                        }
                    }
                }
            }
        }

        ci += 1;
        if ci == columns_count - ri {
            ci = 0;
            ri += 1;
        }
    }

    img
}

fn create_response(identicon: ImageBuffer<Rgba<u8>, Vec<u8>>) -> String {
    let mut buf = Vec::new();
    let encoder = PngEncoder::new(&mut buf);
    encoder
        .write_image(
            &identicon,
            identicon.width(),
            identicon.height(),
            Rgba::<u8>::COLOR_TYPE,
        )
        .expect("Failed to encode image to PNG");

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: image/png\r\nContent-Length: {}\r\n\r\n",
        buf.len()
    );
    let mut response = response.into_bytes();
    response.extend_from_slice(&buf);
    unsafe { String::from_utf8_unchecked(response) }
}
