use crate::prototype::rotation::Rotatable;
use crate::prototype::Rotation;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub(crate) enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub(crate) fn opposite(self) -> Self {
        use Direction::*;
        match self {
            North => South,
            East => West,
            South => North,
            West => East,
        }
    }

    pub(crate) fn clockwise(self) -> Self {
        use Direction::*;
        match self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }

    pub(crate) fn counterclockwise(self) -> Self {
        use Direction::*;
        match self {
            North => West,
            East => North,
            South => East,
            West => South,
        }
    }
}

impl Rotatable for Direction {
    fn rotate(&self, rotation: Rotation) -> Self {
        match rotation {
            Rotation::None => *self,
            Rotation::Clockwise90 => self.clockwise(),
            Rotation::Clockwise180 => self.opposite(),
            Rotation::Counterclockwise90 => self.counterclockwise(),
        }
    }
}
