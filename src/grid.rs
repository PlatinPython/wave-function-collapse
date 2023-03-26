use std::cmp;

use image::imageops::Nearest;
use image::{DynamicImage, GenericImage, GenericImageView, RgbaImage};
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
    img: DynamicImage,
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
        let img = Grid::create_image(img_width, img_height, width, height);
        GridBuilder {
            width,
            height,
            prototypes,
            grid_builder: |prototypes| {
                vec![vec![prototypes.iter().collect(); height as usize]; width as usize]
            },
            img_width,
            img_height,
            img,
        }
        .build()
    }

    fn create_image(img_width: u32, img_height: u32, width: u32, height: u32) -> DynamicImage {
        let empty = DynamicImage::default().resize_exact(img_width, img_height, Nearest);

        let mut image = RgbaImage::new(width * img_width, height * img_height);
        image
            .enumerate_pixels_mut()
            .for_each(|(x, y, pix)| *pix = empty.get_pixel(x % img_width, y % img_height));
        DynamicImage::ImageRgba8(image)
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

    pub(crate) fn img(&self) -> &DynamicImage {
        self.borrow_img()
    }

    pub(crate) fn is_collapsed(&self) -> bool {
        self.grid().iter().flatten().all(|v| v.len() == 1)
    }

    pub(crate) fn iterate(&mut self) {
        let (x, y) = self.get_min_entropy_coords();
        self.collapse_at(x, y);
        let collapsed = self.propagate(x, y);
        self.update_img(collapsed);
    }

    fn get_min_entropy_coords(&self) -> (usize, usize) {
        let min_entropy = self
            .borrow_grid()
            .iter()
            .flatten()
            .map(Vec::len)
            .filter(|l| *l != 1)
            .min()
            .unwrap_or(self.borrow_prototypes().len());
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

    fn propagate(&mut self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut collapsed = Vec::new();
        let mut stack = Vec::new();
        stack.push((x, y));
        while let Some((x, y)) = stack.pop() {
            if self.grid()[x][y].len() == 1 {
                collapsed.push((x, y));
            }
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
        collapsed
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

    fn update_img(&mut self, collapsed: Vec<(usize, usize)>) {
        collapsed.iter().for_each(|(x, y)| {
            self.with_mut(|fields| {
                fields.grid[*x][*y][0]
                    .image()
                    .pixels()
                    .for_each(|(img_x, img_y, pix)| {
                        fields.img.put_pixel(
                            *x as u32 * *fields.img_width + img_x,
                            *y as u32 * *fields.img_height + img_y,
                            pix,
                        )
                    })
            })
        });
    }
}
