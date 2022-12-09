use image::{GenericImageView};
use std::env;

fn get_ascii_char(red: u8, green: u8, blue: u8) -> char {
    // Get average of color values
    let average = (red as u32 + green as u32 + blue as u32) / 3;

    // Get ascii character from average
    let ascii_char = match average {
        0..=25 => '@',
        26..=50 => '#',
        51..=75 => '8',
        76..=100 => '&',
        101..=125 => 'o',
        126..=150 => ':',
        151..=175 => '*',
        176..=200 => '.',
        201..=225 => ' ',
        _ => ' ',
    };

    ascii_char
}
fn main() {
    let args : Vec<String> = env::args().collect();
    let file_location = &args[1];
    let resolution = &args[2].parse().unwrap();
    let image = image::open(file_location).unwrap();

    let width = image.dimensions().0 / resolution;
    let height  = image.dimensions().1 / resolution;

    let resized_image = image.thumbnail(width, height);

    let mut ascii_image = vec![vec![' '; width as usize]; height as usize];

    for pixel in resized_image.pixels() {
        // Get color from pixel
        let color = pixel.2;
        // Get red, green and blue values
        let red = color[0];
        let green = color[1];
        let blue = color[2];

        // Get ascii character from color values
        let ascii_char = get_ascii_char(red, green, blue);

        // Put ascii character in array
        ascii_image[pixel.1 as usize][pixel.0 as usize] = ascii_char;
    }

    // Print ascii image
    for row in ascii_image {
        for col in row {
            print!("{}", col);
        }
        println!();
    }
}
