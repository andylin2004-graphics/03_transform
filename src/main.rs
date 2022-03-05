use parser::parse_file;
use image::Image;
use color::Color;
use matrix::Matrix;
mod parser;
mod matrix;
mod image;
mod color;
mod draw;

fn main() {
    // println!("Hello, world!");
    // println!("{}", f32::sin(f32::consts::FRAC_PI_2));
    let mut screen = Image::new(500, 500);
    let mut color = Color::new_color(0, 255, 0);
    let mut edges = Matrix::new(4, 4);
    let mut transform = Matrix::new(4,4);
    parse_file("script", edges, transform, screen, color);
}
