use color::Color;
use image::Image;
use matrix::Matrix;
mod color;
mod draw;
mod image;
mod matrix;

const XRES: i32 = 500;
const YRES: i32 = 500;

fn main() {
    let mut image = Image::new(XRES as usize, YRES as usize);
    let mut color = Color::new_color(0, 255, 0);
    let mut m1 = Matrix::new(4, 4);
    let mut m2 = Matrix::new(0, 0);
    let mut edges = Matrix::new(1, 0);

    println!("Testing add_edge. Adding (1, 2, 3), (4, 5, 6) m2 =");
    m2.add_edge(1.0, 2.0, 3.0, 4.0, 5.0, 6.0);
    m2.print_matrix();

    println!("Testing ident. m1 = ");
    m1.identity();
    m1.print_matrix();

    println!("Testing matrix_mult. m1 * m2 =");
    m2.multiply_matrixes(m1);
    m2.print_matrix();

    println!("Testing Matrix mult. m1 =");
    m1 = Matrix::new(0, 0);
    m1.add_edge(1.0, 2.0, 3.0, 4.0, 5.0, 6.0);
    m1.add_edge(7.0, 8.0, 9.0, 10.0, 11.0, 12.0);
    m1.print_matrix();

    println!("\nTesting Matrix mult. m1 * m2 =");
    m2.multiply_matrixes(m1);
    m2.print_matrix();

    edges.add_edge(50.0, 450.0, 0.0, 100.0, 450.0, 0.0);
    edges.add_edge(50.0, 450.0, 0.0, 50.0, 400.0, 0.0);
    edges.add_edge(100.0, 450.0, 0.0, 100.0, 400.0, 0.0);
    edges.add_edge(100.0, 400.0, 0.0, 50.0, 400.0, 0.0);

    edges.add_edge(200.0, 450.0, 0.0, 250.0, 450.0, 0.0);
    edges.add_edge(200.0, 450.0, 0.0, 200.0, 400.0, 0.0);
    edges.add_edge(250.0, 450.0, 0.0, 250.0, 400.0, 0.0);
    edges.add_edge(250.0, 400.0, 0.0, 200.0, 400.0, 0.0);

    edges.add_edge(150.0, 400.0, 0.0, 130.0, 360.0, 0.0);
    edges.add_edge(150.0, 400.0, 0.0, 170.0, 360.0, 0.0);
    edges.add_edge(130.0, 360.0, 0.0, 170.0, 360.0, 0.0);

    edges.add_edge(100.0, 340.0, 0.0, 200.0, 340.0, 0.0);
    edges.add_edge(100.0, 320.0, 0.0, 200.0, 320.0, 0.0);
    edges.add_edge(100.0, 340.0, 0.0, 100.0, 320.0, 0.0);
    edges.add_edge(200.0, 340.0, 0.0, 200.0, 320.0, 0.0);

    image.draw_lines(edges, color);

    image.create_file("bob.ppm".to_owned());

    image = Image::new(300, 400);
    edges = Matrix::new(4, 4);
    color.r = 255;
    color.b = 255;
    for i in 0..image.width as i32 {
        edges.add_edge_int(i, 0, 0, i, image.height as i32 - 1, 0);
    }

    image.draw_lines(edges, color);

    color.b = 0;
    color.g = 0;
    edges = Matrix::new(4, 4);
    for i in 0..20 {
        for v in (0..60).step_by(20) {
            edges.add_edge_int(0, i + v, 0, image.width as i32 - 1, i + v, 0);
        }
        edges.add_edge_int(i, 0, 0, i, image.height as i32 - 1, 0);
        edges.add_edge_int(
            0,
            image.height as i32 - 1 - i,
            0,
            image.width as i32 - 1,
            image.height as i32 - 1 - i,
            0,
        );
        edges.add_edge_int(
            image.width as i32 - 1 - i,
            0,
            0,
            image.width as i32 - 1 - i,
            image.height as i32 - 1,
            0,
        );
    }
    image.draw_lines(edges, color);

    color.g = 255;
    color.b = 255;

    edges = Matrix::new(4, 4);
    let mut hashtag_start = 20;
    for _ in 0..5 {
        for i in 1..3 {
            edges.add_edge_int(
                hashtag_start,
                2 + 15 * i,
                0,
                hashtag_start + 30,
                2 + 15 * i,
                0,
            );
        }
        for i in 1..3 {
            edges.add_edge_int(
                hashtag_start + (10 * i),
                7,
                0,
                hashtag_start + (10 * i),
                45,
                0,
            );
        }
        hashtag_start += 57;
    }

    image.draw_lines(edges, color);
    color.r = 0;
    color.g = 0;
    color.b = 0;

    edges = Matrix::new(4, 4);
    edges.add_edge_int(90, 325, 0, 90, 275, 0);
    edges.add_edge_int(210, 325, 0, 210, 275, 0);
    edges.add_edge_int(65, 150, 0, 65, 120, 0);
    edges.add_edge_int(235, 150, 0, 235, 120, 0);
    edges.add_edge_int(65, 120, 0, 235, 120, 0);
    image.draw_lines(edges, color);
    image.create_file("rubensim.ppm".to_owned());
}
