use parser::parse_file;
use image::Image;
use color::Color;
use matrix::Matrix;
use std::env;
mod parser;
mod matrix;
mod image;
mod color;
mod draw;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && args[1] == "art"{
        let mut screen = Image::new(500, 500);
        let color = Color::new_color(0, 255, 0);
        let mut edges = Matrix::new(0,0);
        let mut transform = Matrix::new(4,4);
        parse_file("oofscript", &mut edges, &mut transform, &mut screen, color);
    }else{
        let mut screen = Image::new(500, 500);
        let color = Color::new_color(0, 255, 0);
        let mut edges = Matrix::new(0,0);
        let mut transform = Matrix::new(4,4);
        parse_file("script", &mut edges, &mut transform, &mut screen, color);
    }
}
