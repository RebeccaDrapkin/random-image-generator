extern crate image;
extern crate rand;
use image::ImageBuffer;
use image::Rgb;
use rand::prelude::*;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

#[derive(Copy, Clone, Debug)]
pub struct Circle {
    // center x
    pub h: f32,
    // center y
    pub k: f32,
    //radius
    pub radius: f32,
}

#[derive(Copy, Clone, Debug)]
pub struct Rect {
    // center x
    pub x: u32,
    // center y
    pub y: u32,
    //radius
    pub width: u32,

    pub height: u32,
}

fn main() {
    let mut imgbuf: image::RgbImage = image::ImageBuffer::new(800, 800);
    let shapes: i32 = thread_rng().gen_range(5..15);

    for i in (1..shapes) {
        draw_circle(&mut imgbuf);
        draw_rectangle(&mut imgbuf);
    }

    imgbuf.save("test1.png").unwrap();
}

fn color_picker() -> (u8, u8, u8) {
    let mut rng = rand::thread_rng();
    let colors: [(u8, u8, u8); 4] = [
        ((255), (0), (0)),
        ((0), (0), (255)),
        ((0), (230), (50)),
        ((255), (200), (0)),
    ];
    let color = colors.choose(&mut rng).unwrap();
    *color
}

fn circle_parameters() -> (f32, f32, f32) {
    let circle_radius: f32 = thread_rng().gen_range(5.00..100.00);
    let circle_min_h = std::cmp::min(circle_radius as i32, 800 - circle_radius as i32);
    let circle_min_k = std::cmp::min(circle_radius as i32, 800 - circle_radius as i32);
    let circle_max_h: i32 = std::cmp::max(circle_radius as i32, 800 - circle_radius as i32);
    let circle_max_k: i32 = std::cmp::max(circle_radius as i32, 800 - circle_radius as i32);

    let val_circle_h: i32 = thread_rng().gen_range(circle_min_h..circle_max_h);
    let val_circle_k: i32 = thread_rng().gen_range(circle_min_k..circle_max_k);

    (val_circle_h as f32, val_circle_k as f32, circle_radius)
}

fn rect_parameters() -> (u32, u32, u32, u32) {
    // rect size constraints
    let min_rect_area: u32 = 320;
    let max_rect_area: u32 = 1440;
    let rect_area: u32 = thread_rng().gen_range(min_rect_area..max_rect_area);
    let rect_width: u32 = thread_rng().gen_range(1..(rect_area / 4));
    let rect_height: u32 = rect_area / rect_width;

    // rect placement constraints
    let smaller_rect_x = std::cmp::min(rect_width, 800 - rect_width);
    let smaller_rect_y = std::cmp::min(rect_height, 800 - rect_height);
    let bigger_rect_x = std::cmp::max(rect_width, 800 - rect_width);
    let bigger_rect_y = std::cmp::max(rect_height, 800 - rect_height);

    let val_rect_x: u32 = thread_rng().gen_range(smaller_rect_x..bigger_rect_x);
    let val_rect_y: u32 = thread_rng().gen_range(smaller_rect_y..bigger_rect_y);

    (val_rect_x, val_rect_y, rect_width, rect_height)
}

fn create_circle() -> Circle {
    let (circle_h, circle_k, circle_radius) = circle_parameters();
    let circle: Circle = Circle {
        h: circle_h,
        k: circle_k,
        radius: circle_radius,
    };
    circle
}

fn create_rectangle() -> Rect {
    let (val_rect_x, val_rect_y, rect_width, rect_height) = rect_parameters();
    let rectangle = Rect {
        x: val_rect_x,
        y: val_rect_y,
        width: rect_width,
        height: rect_height,
    };
    rectangle
}

fn draw_rectangle(imagebuffer: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) {
    let rectangle = create_rectangle();
    let (r, g, b) = color_picker();

    for x in rectangle.x..(rectangle.x + rectangle.width) {
        for y in rectangle.y..(rectangle.y + rectangle.height) {
            let pixel = imagebuffer.get_pixel_mut(x as u32, y as u32);
            *pixel = image::Rgb([r, g, b]);
        }
    }
}

fn draw_circle(imagebuffer: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) {
    let circle = create_circle();
    let (r, g, b) = color_picker();

    for x in ((circle.h - circle.radius) as i32..(circle.h + circle.radius) as i32) {
        for y in ((circle.k - circle.radius) as i32..(circle.k + circle.radius) as i32) {
            if ((x as f32 - circle.h) * (x as f32 - circle.h)
                + (y as f32 - circle.k) * (y as f32 - circle.k))
                < (circle.radius as f32 * circle.radius as f32 - 5.00)
                || ((x as f32 - circle.h) * (x as f32 - circle.h)
                    + (y as f32 - circle.k) * (y as f32 - circle.k))
                    == (circle.radius as f32 * circle.radius as f32 + 5.00)
            {
                let pixel = imagebuffer.get_pixel_mut(x as u32, y as u32);
                *pixel = image::Rgb([r, g, b]);
            }
        }
    }
}
