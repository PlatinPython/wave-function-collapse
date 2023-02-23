use std::cmp::max;
use std::ops::{Index, IndexMut};
use std::thread;
use std::time::Duration;

use ::image::io::Reader;
use ::image::{DynamicImage, ImageFormat, Rgb, RgbImage};
use image::imageops::Nearest;
use image::RgbaImage;
use show_image::{create_window, AsImageView, WindowOptions};

use crate::Tiles::{Inverted, Normal};

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

pub(crate) struct Grid {
    width: u32,
    height: u32,
    grid: Vec<Vec<Tiles>>,
}

#[derive(Clone)]
pub(crate) enum Tiles {
    Empty,
    Normal,
    Inverted,
}

impl Grid {
    pub(crate) fn new(width: u32, height: u32) -> Grid {
        let grid = vec![vec![Tiles::Empty; height as usize]; width as usize];
        Grid {
            width,
            height,
            grid,
        }
    }

    pub(crate) fn width(&self) -> u32 {
        self.width
    }

    pub(crate) fn height(&self) -> u32 {
        self.height
    }

    pub(crate) fn to_image(
        &self,
        empty: &DynamicImage,
        normal: &DynamicImage,
        inverted: &DynamicImage,
    ) -> DynamicImage {
        use Tiles::*;

        let width = max(max(empty.width(), normal.width()), inverted.width());
        let height = max(max(empty.height(), normal.height()), inverted.height());

        let empty = empty.resize_exact(width, height, Nearest).into_rgba8();
        let normal = normal.resize_exact(width, height, Nearest).into_rgba8();
        let inverted = inverted.resize_exact(width, height, Nearest).into_rgba8();

        let mut image = RgbaImage::new(self.width * width, self.height * height);
        image.enumerate_pixels_mut().for_each(|(x, y, pix)| {
            let x_index = (x / width) as usize;
            let y_index = (y / height) as usize;
            match self[x_index][y_index] {
                Empty => *pix = *empty.get_pixel(x % width, y % height),
                Normal => *pix = *normal.get_pixel(x % width, y % height),
                Inverted => *pix = *inverted.get_pixel(x % width, y % height),
            }
        });
        DynamicImage::ImageRgba8(image)
    }
}

impl Index<usize> for Grid {
    type Output = Vec<Tiles>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.grid[index]
    }
}

impl IndexMut<usize> for Grid {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.grid[index]
    }
}
