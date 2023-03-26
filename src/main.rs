use std::collections::HashMap;
use std::time::Instant;

use image::ImageFormat;
use image::io::Reader;
use show_image::{AsImageView, create_window, WindowOptions};

use crate::grid::Grid;
use crate::prototype::direction::Direction::*;

#[show_image::main]
fn main() {
    let instant = Instant::now();
    let three_way = Reader::open("images/three_way.png")
        .unwrap()
        .decode()
        .unwrap();
    let no_way = Reader::open("images/no_way.png").unwrap().decode().unwrap();

    let mut grid = Grid::generate_grid(
        50,
        50,
        vec![
            (
                no_way.clone(),
                HashMap::from([(North, false), (East, false), (South, false), (West, false)]),
            ),
            (
                three_way,
                HashMap::from([(North, true), (East, true), (South, false), (West, true)]),
            ),
        ],
        no_way,
    );
    let window = create_window("image", WindowOptions::default()).unwrap();
    let image = grid.img();
    let image_view = image.as_image_view().unwrap();
    window.set_image("grid", image_view).unwrap();
    while !grid.is_collapsed() {
        grid.iterate();
        let image = grid.img();
        let image_view = image.as_image_view().unwrap();
        window.set_image("grid", image_view).unwrap();
    }
    grid.img()
        .save_with_format("output.png", ImageFormat::Png)
        .unwrap();
    println!("{:?}", instant.elapsed());
    window.wait_until_destroyed().unwrap();
}

mod grid;
mod prototype;
