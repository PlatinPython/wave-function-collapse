use std::collections::HashMap;

use image::DynamicImage;

use crate::prototype::direction::Direction;
use crate::prototype::rotation::{Rotatable, Rotation};

pub(crate) type Sockets = HashMap<Direction, bool>;
type NeighborLists<'a> = HashMap<Direction, Vec<&'a Prototype<'a>>>;

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct Prototype<'a> {
    image: DynamicImage,
    rotation: Rotation,
    sockets: Sockets,
    neighbor_lists: NeighborLists<'a>,
}

impl<'a> Prototype<'a> {
    pub(crate) fn new(image: &DynamicImage, sockets: Sockets) -> Vec<Self> {
        if sockets.keys().len() != 4 {
            panic!("All directions should be defined.");
        }
        let prototype = Self {
            image: image.clone(),
            rotation: Rotation::None,
            sockets,
            neighbor_lists: HashMap::with_capacity(4),
        };
        let prototype90 = prototype.rotate(Rotation::Clockwise90);
        let prototype180 = prototype.rotate(Rotation::Clockwise180);
        let prototype270 = prototype.rotate(Rotation::Counterclockwise90);
        let mut prototypes = vec![prototype, prototype90, prototype180, prototype270];
        let mut duplicates = vec![];
        for i in &prototypes {
            for j in &prototypes {
                if i == j {
                    continue;
                }
                if i.equals(j) && !(duplicates.contains(&i) || duplicates.contains(&j)) {
                    duplicates.push(j);
                }
            }
        }
        let mut filtered_indices: Vec<usize> = prototypes
            .iter()
            .enumerate()
            .filter_map(|(i, val)| {
                if duplicates.contains(&val) {
                    Some(i)
                } else {
                    None
                }
            })
            .collect();
        filtered_indices.reverse();
        for i in filtered_indices {
            prototypes.remove(i);
        }
        prototypes
    }

    #[allow(unused)]
    pub(crate) fn add_neighbor(&mut self, direction: Direction, neighbor: &'a Prototype) {
        if let Some(neighbor_list) = self.neighbor_lists.get_mut(&direction) {
            neighbor_list.push(neighbor);
        }
    }

    pub(crate) fn image(&self) -> &DynamicImage {
        &self.image
    }

    pub(crate) fn matches(&self, other: &Self, direction: Direction) -> bool {
        self.sockets[&direction] == other.sockets[&direction.opposite()]
    }

    fn equals(&self, other: &Self) -> bool {
        self == other
            || (self.sockets == other.sockets && self.neighbor_lists == other.neighbor_lists)
    }
}

impl<'a> Rotatable for Prototype<'a> {
    fn rotate(&self, rotation: Rotation) -> Self {
        Self {
            image: self.image.rotate(rotation),
            rotation: self.rotation.rotate(rotation),
            sockets: self.sockets.rotate(rotation),
            neighbor_lists: self.neighbor_lists.rotate(rotation),
        }
    }
}

pub(crate) mod direction;
pub(crate) mod rotation;
