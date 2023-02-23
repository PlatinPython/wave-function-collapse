use std::cmp::max;
use std::ops::{Index, IndexMut};

use image::imageops::Nearest;
use image::{DynamicImage, RgbaImage};

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
