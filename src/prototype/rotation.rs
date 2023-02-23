use std::collections::HashMap;

use image::DynamicImage;

use crate::prototype::direction::Direction;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub(crate) enum Rotation {
    None,
    Clockwise90,
    Clockwise180,
    Counterclockwise90,
}

impl Rotatable for Rotation {
    fn rotate(&self, rotation: Self) -> Self {
        use Rotation::*;
        match rotation {
            None => *self,
            Clockwise90 => match self {
                None => Clockwise90,
                Clockwise90 => Clockwise180,
                Clockwise180 => Counterclockwise90,
                Counterclockwise90 => None,
            },
            Clockwise180 => match self {
                None => Clockwise180,
                Clockwise90 => Counterclockwise90,
                Clockwise180 => None,
                Counterclockwise90 => Clockwise90,
            },
            Counterclockwise90 => match self {
                None => Counterclockwise90,
                Clockwise90 => None,
                Clockwise180 => Clockwise90,
                Counterclockwise90 => Clockwise180,
            },
        }
    }
}

pub(crate) trait Rotatable {
    fn rotate(&self, rotation: Rotation) -> Self;
}

impl<T> Rotatable for HashMap<Direction, T>
where
    T: Clone,
{
    fn rotate(&self, rotation: Rotation) -> Self {
        self.iter()
            .map(|(dir, val)| (dir.rotate(rotation), val.clone()))
            .collect()
    }
}

impl Rotatable for DynamicImage {
    fn rotate(&self, rotation: Rotation) -> Self {
        match rotation {
            Rotation::None => self.clone(),
            Rotation::Clockwise90 => self.rotate90(),
            Rotation::Clockwise180 => self.rotate180(),
            Rotation::Counterclockwise90 => self.rotate270(),
        }
    }
}
