use std::collections::HashMap;
use std::thread;
use std::time::Duration;

use image::DynamicImage;
use image::io::Reader;
use show_image::{AsImageView, create_window, WindowOptions};

use crate::grid::Grid;
use crate::prototype::direction::Direction::*;

#[show_image::main]
fn main() {
    let img = Reader::open("./images/tile.png").unwrap().decode().unwrap();
    let empty = DynamicImage::default();

    let window = create_window("image", WindowOptions::default()).unwrap();
    let mut grid = Grid::generate_grid(
        20,
        20,
        vec![
            (
                empty.clone(),
                HashMap::from([(North, false), (East, false), (South, false), (West, false)]),
            ),
            (
                img,
                HashMap::from([(North, true), (East, true), (South, false), (West, true)]),
            ),
        ],
        empty,
    );
    for x in 0..grid.width() as usize {
        for y in 0..grid.height() as usize {
            grid.change(x, y);
            let image = grid.to_image();
            let image_view = image.as_image_view().unwrap();
            window.set_image("grid", image_view).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    }
    window.wait_until_destroyed().unwrap();
}

mod grid;
mod prototype;
