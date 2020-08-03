pub mod color;
pub mod point;
pub mod vec3;

use color::Color;
use point::Point;

fn write_png(name: &str) -> std::io::Result<()> {
    let width: u32 = 256;
    let height: u32 = 256;

    image::ImageBuffer::from_fn(width, height, |x, y| {
        Color::new(
            x as f32 / (height - 1) as f32,
            y as f32 / (width - 1) as f32,
            0.25,
        )
        .to_rgb()
    })
    .save(name)
    .expect("Unable to write PNG data to file");

    Ok(())
}

fn main() -> std::io::Result<()> {
    let p1 = Point::new(1.0, 2.0, 3.0);
    let p2 = p1 * 3.14159;
    let p3 = p2.clone().unit_vector();
    println!("p1 = {:?} p2 = {:?} p3 = {:?}", p1, p2, p3);

    let c1 = Color::new(0.1, 0.1, 0.5);
    println!("c1 = {:?}", c1);

    write_png("test.png")
}
