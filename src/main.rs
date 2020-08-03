pub mod color;
pub mod point;
pub mod ray;
pub mod vec3;

use color::Color;
use point::Point;
use ray::Ray;
use vec3::Vec3;

fn hit_sphere(center: &Point, radius: f32, ray: &Ray) -> f32 {
    let oc: Vec3 = ray.origin - *center;
    let a = ray.direction.length_squared();
    let half_b = oc.dot(&ray.direction);
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;

    if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - discriminant.sqrt()) / a
    }
}

fn ray_color(ray: &Ray) -> Color {
    let t = hit_sphere(&Point::new(0.0, 0.0, -1.0), 0.5, &ray);

    if t > 0.0 {
        let normal = (ray.at(t) - Vec3::new(0.0, 0.0, -1.0)).unit_vector();
        0.5 * Color::new(normal.x + 1.0, normal.y + 1.0, normal.z + 1.0)
    } else {
        let unit_direction: Vec3 = ray.direction.unit_vector();
        let t = 0.5 * (unit_direction.y + 1.0);

        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}

fn render(name: &str) -> std::io::Result<()> {
    let aspect_ratio: f32 = 16.0 / 9.0;
    let image_width: u32 = 400;
    let image_height: u32 = (image_width as f32 / aspect_ratio) as u32;

    let viewport_height: f32 = 2.0;
    let viewport_width: f32 = aspect_ratio * viewport_height;
    let focal_length: f32 = 1.0;

    let origin: Point = Point::origin();
    let horizontal: Point = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical: Point = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner: Point =
        origin - (horizontal / 2.0) - (vertical / 2.0) - Vec3::new(0.0, 0.0, focal_length);

    image::ImageBuffer::from_fn(image_width, image_height, |x, y| {
        let u: f32 = (x as f32) / ((image_width - 1) as f32);
        let v: f32 = (y as f32) / ((image_height - 1) as f32);
        let direction: Vec3 = lower_left_corner + (u * horizontal) + (v * vertical) - origin;
        let ray: Ray = Ray::new(origin, direction);

        ray_color(&ray).to_rgb()
    })
    .save(name)
    .expect("Unable to write PNG data to file");

    Ok(())
}

fn main() -> std::io::Result<()> {
    render("render.png")
}
