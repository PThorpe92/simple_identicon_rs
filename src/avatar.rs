extern crate image;
use image::{ImageBuffer, Rgba};
use xxhash_rust::xxh3::xxh3_128;

pub fn identicon(data: &str) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let hash = xxh3_128(data.as_bytes());
    let digest = hash.to_le_bytes();

    let mut pixels = vec![];
    for i in 0..16 {
        pixels.push(digest[i]);
    }

    let width = 540;
    let height = 540;
    let block_size = 60;
    let mut img = ImageBuffer::new(width, height);

    let color1 = Rgba([digest, (digest >> 8) as u8, (digest >> 16) as u8, 0xff]);
    let color2 = Rgba([
        (0xff ^ (digest as u8)),
        (0xff ^ (digest >> 8) as u8),
        (0xff ^ (digest >> 16) as u8),
        0xff,
    ]);

    for x in 0..length {
        for y in 0..length {
            let xi = x / block_length;
            let yi = y / block_length;
            let xf = x % block_length;
            let yf = y % block_length;

            if xf < block_length / 2 && yf < block_length / 2 {
                img.put_pixel(x, y, color1);
            } else if xf >= block_length / 2 && yf < block_length / 2 {
                img.put_pixel(x, y, color2);
            } else if xf < block_length / 2
                && yf >= block_length / 2
                && xi < length / block_length / 2
            {
                img.put_pixel(x, y, color2);
            } else if xf >= block_length / 2
                && yf >= block_length / 2
                && yi < length / block_length / 2
            {
                img.put_pixel(x, y, color1);
            } else {
                img.put_pixel(x, y, Rgba([0, 0, 0, 0]));
            }
        }
    }

    img
}
