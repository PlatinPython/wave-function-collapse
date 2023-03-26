use std::collections::HashMap;

use image::io::Reader;
use show_image::{create_window, AsImageView, WindowOptions};

use crate::grid::Grid;
use crate::prototype::direction::Direction::*;

#[show_image::main]
fn main() {
    let one_way = Reader::open("images/one_way.png")
        .unwrap()
        .decode()
        .unwrap();
    let two_way = Reader::open("images/two_way.png")
        .unwrap()
        .decode()
        .unwrap();
    let three_way = Reader::open("images/three_way.png")
        .unwrap()
        .decode()
        .unwrap();
    let four_way = Reader::open("images/four_way.png")
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
                one_way,
                HashMap::from([(North, true), (East, false), (South, false), (West, false)]),
            ),
            (
                two_way,
                HashMap::from([(North, false), (East, true), (South, false), (West, true)]),
            ),
            (
                three_way,
                HashMap::from([(North, true), (East, true), (South, false), (West, true)]),
            ),
            (
                four_way,
                HashMap::from([(North, true), (East, true), (South, true), (West, true)]),
            ),
        ],
        no_way,
    );
    let window = create_window("image", WindowOptions::default()).unwrap();
    let image = grid.to_image();
    let image_view = image.as_image_view().unwrap();
    window.set_image("grid", image_view).unwrap();
    while !grid.is_collapsed() {
        grid.iterate();
        let image = grid.to_image();
        let image_view = image.as_image_view().unwrap();
        window.set_image("grid", image_view).unwrap();
    }
    window.wait_until_destroyed().unwrap();
}

mod grid;
mod prototype;
