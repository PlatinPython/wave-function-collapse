use std::thread;
use std::time::Duration;

use image::io::Reader;
use image::{DynamicImage, ImageFormat, Rgb, RgbImage};
use show_image::{create_window, AsImageView, WindowOptions};

use crate::grid::Grid;
use crate::grid::Tiles::{Inverted, Normal};

#[show_image::main]
fn main() {
    let img = Reader::open("img.png").unwrap().decode().unwrap();
    let img = img.into_rgb8();
    let img_inverted = RgbImage::from_fn(img.width(), img.height(), |x, y| {
        let Rgb([r, g, b]) = *img.get_pixel(x, y);
        Rgb([!r, !g, !b])
    });
    let mut final_img = RgbImage::new(img.width() * 6, img.height() * 4);
    final_img.enumerate_pixels_mut().for_each(|(x, y, pix)| {
        match (
            (x % (img.width() * 2)) < img.width(),
            (y % (img.height() * 2)) < img.height(),
        ) {
            (true, true) => *pix = *img.get_pixel(x % img.width(), y % img.height()),
            (true, false) => {
                *pix = *img_inverted.get_pixel(x % img_inverted.width(), y % img_inverted.height())
            }
            (false, true) => {
                *pix = *img_inverted.get_pixel(x % img_inverted.width(), y % img_inverted.height())
            }
            (false, false) => *pix = *img.get_pixel(x % img.width(), y % img.height()),
        }
    });
    final_img
        .save_with_format("final_img.png", ImageFormat::Png)
        .unwrap();
    let mut grid = Grid::new(40, 40);
    for x in 0..grid.width() as usize {
        for y in 0..grid.height() as usize {
            grid[x][y] = if (x + y) % 2 == 0 { Normal } else { Inverted }
        }
    }
    let empty = DynamicImage::ImageRgb8(RgbImage::default());
    let normal = DynamicImage::ImageRgb8(img);
    let inverted = DynamicImage::ImageRgb8(img_inverted);
    let grid_image = grid.to_image(&empty, &normal, &inverted);
    grid_image
        .save_with_format("grid.png", ImageFormat::Png)
        .unwrap();

    let window = create_window("image", WindowOptions::default()).unwrap();
    let mut grid = Grid::new(20, 20);
    for x in 0..grid.width() as usize {
        for y in 0..grid.height() as usize {
            grid[x][y] = if (x + y) % 3 == 0 { Normal } else { Inverted };

            let image = grid.to_image(&empty, &normal, &inverted);
            let image_view = image.as_image_view().unwrap();
            window.set_image("grid", image_view).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    }
    window.wait_until_destroyed().unwrap();
}

mod grid;
mod prototype;
