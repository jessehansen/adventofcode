use std::cmp::{Eq, Ord, PartialEq};
use std::fmt;

use crate::Direction;

// contains helpers for graphs and signed points
// coordinates are laid out like this:Copy
//
//
//      |
//      |
//      |
//      |
// -----+----- y=0
//      |
//    * |
//      |
//      x=0
//
//  The * is at (x=-1, y = -2)

#[derive(Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct IPoint2D {
    pub x: i32,
    pub y: i32,
}

impl IPoint2D {
    pub const ORIGIN: IPoint2D = IPoint2D { x: 0, y: 0 };

    pub fn cardinal_distance(&self, other: &IPoint2D) -> (i32, i32) {
        (self.x - other.x, self.y - other.y)
    }

    pub fn up(&self) -> IPoint2D {
        IPoint2D {
            x: self.x,
            y: self.y + 1,
        }
    }

    pub fn down(&self) -> IPoint2D {
        IPoint2D {
            x: self.x,
            y: self.y - 1,
        }
    }

    pub fn left(&self) -> IPoint2D {
        IPoint2D {
            x: self.x - 1,
            y: self.y,
        }
    }

    pub fn right(&self) -> IPoint2D {
        IPoint2D {
            x: self.x + 1,
            y: self.y,
        }
    }

    pub fn mv(&self, direction: Direction) -> IPoint2D {
        match direction {
            Direction::Up => self.up(),
            Direction::Left => self.left(),
            Direction::Right => self.right(),
            Direction::Down => self.down(),
        }
    }

    pub fn move_by(&self, dx: i32, dy: i32) -> IPoint2D {
        IPoint2D {
            x: self.x + dx,
            y: self.y + dy,
        }
    }
}

impl fmt::Display for IPoint2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{}", self.x, self.y,)
    }
}

impl fmt::Debug for IPoint2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{}", self.x, self.y,)
    }
}
