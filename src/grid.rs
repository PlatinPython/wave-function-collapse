use std::cmp;

use image::imageops::Nearest;
use image::{DynamicImage, GenericImageView, RgbaImage};
use ouroboros::self_referencing;

use crate::prototype::{Prototype, Sockets};

#[self_referencing]
pub(crate) struct Grid<'a> {
    width: u32,
    height: u32,
    prototypes: Vec<Prototype<'a>>,
    #[borrows(prototypes)]
    #[covariant]
    grid: Vec<Vec<Option<&'this Prototype<'a>>>>,
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
        let grid = vec![vec![None; height as usize]; width as usize];
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
            grid_builder: |_| grid,
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

    pub(crate) fn grid<'this>(&'this self) -> &'this Vec<Vec<Option<&'this Prototype<'a>>>> {
        self.borrow_grid()
    }

    pub(crate) fn change(&mut self, x: usize, y: usize) {
        self.with_mut(|fields| {
            fields.grid[x][y] = Some(
                fields
                    .prototypes
                    .get((x + y) % fields.prototypes.len())
                    .unwrap(),
            )
        });
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
            match self.grid()[x_index][y_index] {
                Some(prototype) => {
                    *pix = prototype
                        .image()
                        .get_pixel(x % *self.borrow_img_width(), y % *self.borrow_img_height())
                }
                None => {
                    *pix =
                        empty.get_pixel(x % *self.borrow_img_width(), y % *self.borrow_img_height())
                }
            }
        });
        DynamicImage::ImageRgba8(image)
    }
}
