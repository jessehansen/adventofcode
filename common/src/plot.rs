use std::cmp::{Eq, Ord, PartialEq};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::Add;

use crate::{Bounds2D, Direction, Point2D};

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

    pub fn cardinal_neighbors(&self) -> Vec<Self> {
        vec![self.up(), self.down(), self.left(), self.right()]
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

    pub fn move_to(&self, direction: Direction, distance: i32) -> IPoint2D {
        use Direction::*;
        match direction {
            Left => self.move_by(-distance, 0),
            Right => self.move_by(distance, 0),
            Up => self.move_by(0, distance),
            Down => self.move_by(0, -distance),
        }
    }

    pub fn manhattan_distance(&self, other: IPoint2D) -> i32 {
        (other.x - self.x).abs() + (other.y - self.y).abs()
    }

    pub fn points_within_manhattan_distance(
        &self,
        distance: i32,
    ) -> impl Iterator<Item = IPoint2D> {
        let mut points = vec![];
        for dx in -distance..=distance {
            let y_allowed = distance - dx.abs();
            for dy in -y_allowed..=y_allowed {
                points.push(IPoint2D {
                    x: self.x + dx,
                    y: self.y + dy,
                });
            }
        }
        points.into_iter()
    }

    pub fn map_infinite_to_template_bounds(&self, template_bounds: &Bounds2D) -> (Self, Self) {
        let width: i32 = template_bounds.width.try_into().unwrap();
        let height: i32 = template_bounds.height.try_into().unwrap();
        let x = self.x.rem_euclid(width);
        let y = self.y.rem_euclid(height);
        (
            Self { x, y },
            Self {
                x: self.x - x,
                y: self.y - y,
            },
        )
    }
}

impl Add for IPoint2D {
    type Output = IPoint2D;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl Add for &IPoint2D {
    type Output = IPoint2D;

    fn add(self, rhs: Self) -> Self::Output {
        IPoint2D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl Add<IPoint2D> for &IPoint2D {
    type Output = IPoint2D;

    fn add(self, rhs: IPoint2D) -> Self::Output {
        IPoint2D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
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

// NOTE: This is useful for accepting usize points where integer points are required (ex: shoelace
// formula), but beware - this switches the meaning of "Up"
impl TryFrom<&Point2D> for IPoint2D {
    type Error = anyhow::Error;

    fn try_from(value: &Point2D) -> Result<Self, Self::Error> {
        Ok(IPoint2D {
            x: value.x.try_into()?,
            y: value.y.try_into()?,
        })
    }
}

impl TryFrom<Point2D> for IPoint2D {
    type Error = anyhow::Error;

    fn try_from(value: Point2D) -> Result<Self, Self::Error> {
        IPoint2D::try_from(&value)
    }
}

impl TryInto<Point2D> for IPoint2D {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<Point2D, Self::Error> {
        Ok(Point2D {
            x: self.x.try_into()?,
            y: self.y.try_into()?,
        })
    }
}

#[inline]
pub fn ipt(x: i32, y: i32) -> IPoint2D {
    IPoint2D { x, y }
}

// infinite points: moving left from 0 wraps to bounds

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct InfinitePoint2D {
    pub coord: Point2D,
    pub template_coord: IPoint2D,
}

impl Hash for InfinitePoint2D {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.coord.hash(state);
        self.template_coord.hash(state);
    }
}

impl InfinitePoint2D {
    pub fn new(pt: Point2D) -> Self {
        Self {
            coord: pt,
            template_coord: IPoint2D::ORIGIN,
        }
    }

    pub fn cardinal_neighbors(&self, bounds: &Bounds2D) -> Vec<InfinitePoint2D> {
        vec![
            self.up(bounds),
            self.left(bounds),
            self.right(bounds),
            self.down(bounds),
        ]
    }

    pub fn up(&self, bounds: &Bounds2D) -> Self {
        if let Some(up) = self.coord.up() {
            Self {
                coord: up,
                template_coord: self.template_coord,
            }
        } else {
            Self {
                coord: Point2D {
                    x: self.coord.x,
                    y: bounds.height - 1,
                },
                template_coord: self.template_coord.up(),
            }
        }
    }

    pub fn left(&self, bounds: &Bounds2D) -> Self {
        if let Some(left) = self.coord.left() {
            Self {
                coord: left,
                template_coord: self.template_coord,
            }
        } else {
            Self {
                coord: Point2D {
                    x: bounds.width - 1,
                    y: self.coord.y,
                },
                template_coord: self.template_coord.left(),
            }
        }
    }

    pub fn down(&self, bounds: &Bounds2D) -> Self {
        if let Some(down) = self.coord.down(bounds.height) {
            Self {
                coord: down,
                template_coord: self.template_coord,
            }
        } else {
            Self {
                coord: Point2D {
                    x: self.coord.x,
                    y: 0,
                },
                template_coord: self.template_coord.down(),
            }
        }
    }

    pub fn right(&self, bounds: &Bounds2D) -> Self {
        if let Some(right) = self.coord.right(bounds.width) {
            Self {
                coord: right,
                template_coord: self.template_coord,
            }
        } else {
            Self {
                coord: Point2D {
                    x: 0,
                    y: self.coord.y,
                },
                template_coord: self.template_coord.right(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_points_within_manhattan_distance() {
        let point = ipt(2, 3);
        let points: Vec<IPoint2D> = point.points_within_manhattan_distance(3).collect();
        assert_eq!(
            points,
            vec![
                ipt(-1, 3),
                ipt(0, 2),
                ipt(0, 3),
                ipt(0, 4),
                ipt(1, 1),
                ipt(1, 2),
                ipt(1, 3),
                ipt(1, 4),
                ipt(1, 5),
                ipt(2, 0),
                ipt(2, 1),
                ipt(2, 2),
                ipt(2, 3),
                ipt(2, 4),
                ipt(2, 5),
                ipt(2, 6),
                ipt(3, 1),
                ipt(3, 2),
                ipt(3, 3),
                ipt(3, 4),
                ipt(3, 5),
                ipt(4, 2),
                ipt(4, 3),
                ipt(4, 4),
                ipt(5, 3),
            ]
        );

        for pt in points {
            assert!(pt.manhattan_distance(point) <= 3);
        }
    }

    #[test]
    fn test_map_infinite_within_template_bounds() {
        let bounds = Bounds2D {
            width: 10,
            height: 10,
        };
        assert_eq!(
            (ipt(1, 1), ipt(0, 0)),
            ipt(1, 1).map_infinite_to_template_bounds(&bounds)
        );
        assert_eq!(
            (ipt(1, 1), ipt(10, 0)),
            ipt(11, 1).map_infinite_to_template_bounds(&bounds)
        );
        assert_eq!(
            (ipt(1, 1), ipt(10, 10)),
            ipt(11, 11).map_infinite_to_template_bounds(&bounds)
        );
        assert_eq!(
            (ipt(1, 1), ipt(-10, -10)),
            ipt(-9, -9).map_infinite_to_template_bounds(&bounds)
        );
    }
}
