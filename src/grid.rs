use std::cmp;

use image::imageops::Nearest;
use image::{DynamicImage, GenericImageView, RgbaImage};
use ouroboros::self_referencing;
use rand::prelude::*;

use crate::prototype::direction::Direction;
use crate::prototype::{Prototype, Sockets};

#[self_referencing]
pub(crate) struct Grid<'a> {
    width: u32,
    height: u32,
    prototypes: Vec<Prototype<'a>>,
    #[borrows(prototypes)]
    #[covariant]
    grid: Vec<Vec<Vec<&'this Prototype<'a>>>>,
    img_width: u32,
    img_height: u32,
}

impl<'a> Grid<'a> {
    pub(crate) fn generate_grid(
        width: u32,
        height: u32,
        base_prototypes: Vec<(DynamicImage, Sockets)>,
        empty: DynamicImage,
    ) -> Self {
        let img_width = cmp::max(
            empty.width(),
            base_prototypes
                .iter()
                .map(|(img, _)| img.width())
                .reduce(cmp::max)
                .unwrap_or(0),
        );
        let img_height = cmp::max(
            empty.height(),
            base_prototypes
                .iter()
                .map(|(img, _)| img.height())
                .reduce(cmp::max)
                .unwrap_or(0),
        );
        let mut prototypes = vec![];
        for (img, sockets) in base_prototypes {
            let img = img.resize_exact(img_width, img_height, Nearest);
            for prototype in Prototype::new(&img, sockets) {
                prototypes.push(prototype);
            }
        }
        GridBuilder {
            width,
            height,
            prototypes,
            grid_builder: |prototypes| {
                vec![vec![prototypes.iter().collect(); height as usize]; width as usize]
            },
            img_width,
            img_height,
        }
        .build()
    }

    pub(crate) fn width(&self) -> u32 {
        *self.borrow_width()
    }

    pub(crate) fn height(&self) -> u32 {
        *self.borrow_height()
    }

    pub(crate) fn grid<'this>(&'this self) -> &'this Vec<Vec<Vec<&'this Prototype<'a>>>> {
        self.borrow_grid()
    }

    pub(crate) fn is_collapsed(&self) -> bool {
        self.grid().iter().flatten().all(|v| v.len() == 1)
    }

    pub(crate) fn iterate(&mut self) {
        let (x, y) = self.get_min_entropy_coords();
        self.collapse_at(x, y);
        self.propagate(x, y);
    }

    fn get_min_entropy_coords(&self) -> (usize, usize) {
        let mut min_entropy = self.borrow_prototypes().len();
        for x in 0..self.width() as usize {
            for y in 0..self.height() as usize {
                if self.grid()[x][y].len() < min_entropy && self.grid()[x][y].len() != 1 {
                    min_entropy = self.grid()[x][y].len()
                }
            }
        }
        self.grid()
            .iter()
            .enumerate()
            .flat_map(|(x, columns)| {
                columns
                    .iter()
                    .enumerate()
                    .map(move |(y, options)| (x, y, options))
            })
            .filter(|(_, _, options)| options.len() == min_entropy)
            .map(|(x, y, _)| (x, y))
            .choose(&mut thread_rng())
            .expect("There should always be at least one element with minimal entropy")
    }

    fn collapse_at(&mut self, x: usize, y: usize) {
        self.with_grid_mut(|grid| {
            grid[x][y] = grid[x][y]
                .iter()
                .copied()
                .choose_multiple(&mut thread_rng(), 1);
        });
    }

    fn propagate(&mut self, x: usize, y: usize) {
        let mut stack = Vec::new();
        stack.push((x, y));
        while let Some((x, y)) = stack.pop() {
            self.neighbors(x, y)
                .iter()
                .for_each(|(other_x, other_y, direction)| {
                    self.with_grid_mut(|grid| {
                        let prototypes = grid[x][y].clone();
                        let orig_len = grid[*other_x][*other_y].len();
                        if orig_len == 1 {
                            return;
                        }
                        grid[*other_x][*other_y].retain(|other| {
                            prototypes
                                .iter()
                                .any(|prototype| prototype.matches(other, *direction))
                        });
                        if orig_len != grid[*other_x][*other_y].len()
                            && !stack.contains(&(*other_x, *other_y))
                        {
                            stack.push((*other_x, *other_y));
                        }
                    });
                });
        }
    }

    fn neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize, Direction)> {
        let mut neighbors = Vec::with_capacity(4);
        if x != 0 {
            neighbors.push((x - 1, y, Direction::West));
        }
        if x != self.width() as usize - 1 {
            neighbors.push((x + 1, y, Direction::East));
        }
        if y != 0 {
            neighbors.push((x, y - 1, Direction::North));
        }
        if y != self.height() as usize - 1 {
            neighbors.push((x, y + 1, Direction::South));
        }
        neighbors
    }

    pub(crate) fn to_image(&self) -> DynamicImage {
        let empty = DynamicImage::default().resize_exact(
            *self.borrow_img_width(),
            *self.borrow_img_height(),
            Nearest,
        );

        let mut image = RgbaImage::new(
            *self.borrow_width() * *self.borrow_img_width(),
            *self.borrow_height() * *self.borrow_img_height(),
        );
        image.enumerate_pixels_mut().for_each(|(x, y, pix)| {
            let x_index = (x / *self.borrow_img_width()) as usize;
            let y_index = (y / *self.borrow_img_height()) as usize;
            if self.grid()[x_index][y_index].len() == 1 {
                *pix = self.grid()[x_index][y_index][0]
                    .image()
                    .get_pixel(x % *self.borrow_img_width(), y % *self.borrow_img_height());
            } else {
                *pix = empty.get_pixel(x % *self.borrow_img_width(), y % *self.borrow_img_height());
            }
        });
        DynamicImage::ImageRgba8(image)
    }
}
