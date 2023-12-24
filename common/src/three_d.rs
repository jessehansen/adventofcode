use std::cmp::{max, min};
use std::fmt;
use std::str::FromStr;

use anyhow::*;
use itertools::Itertools;

use crate::WrappedPatternParsable;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Point3D {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

impl Point3D {
    pub fn to(&self, other: &Point3D) -> impl Iterator<Item = Point3D> {
        let min_x = min(self.x, other.x);
        let min_y = min(self.y, other.y);
        let min_z = min(self.z, other.z);
        let max_x = max(self.x, other.x);
        let max_y = max(self.y, other.y);
        let max_z = max(self.z, other.z);

        (min_x..=max_x)
            .cartesian_product(min_y..=max_y)
            .cartesian_product(min_z..=max_z)
            .map(|((x, y), z)| Point3D { x, y, z })
    }

    pub fn shift_z_down(&self) -> Point3D {
        Point3D {
            x: self.x,
            y: self.y,
            z: self.z - 1,
        }
    }

    pub fn to_f64_vec(&self) -> Vec<f64> {
        vec![self.x as f64, self.y as f64, self.z as f64]
    }
}

impl FromStr for Point3D {
    type Err = Error;

    fn from_str(point: &str) -> Result<Self> {
        let parsed: Vec<usize> = point.parse_split(',')?;
        parsed.try_into()
    }
}

impl fmt::Display for Point3D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{},{}", self.x, self.y, self.z)
    }
}

impl fmt::Debug for Point3D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl TryFrom<Vec<usize>> for Point3D {
    type Error = Error;

    fn try_from(value: Vec<usize>) -> std::prelude::v1::Result<Self, Self::Error> {
        if value.len() != 3 {
            bail!("invalid value, must have len 3");
        }
        Ok(Point3D {
            x: value[0],
            y: value[1],
            z: value[2],
        })
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Cuboid {
    pub origin: Point3D,
    pub terminex: Point3D,
}

impl Cuboid {
    pub fn points(&self) -> impl Iterator<Item = Point3D> {
        self.origin.to(&self.terminex)
    }

    pub fn min_z(&self) -> usize {
        min(self.origin.z, self.terminex.z)
    }

    pub fn bottom_layer(&self) -> impl Iterator<Item = Point3D> {
        let min_z = self.min_z();
        Point3D {
            x: self.origin.x,
            y: self.origin.y,
            z: min_z,
        }
        .to(&Point3D {
            x: self.terminex.x,
            y: self.terminex.y,
            z: min_z,
        })
    }

    pub fn contains(&self, pt: &Point3D) -> bool {
        let min_x = min(self.origin.x, self.terminex.x);
        let max_x = max(self.origin.x, self.terminex.x);
        if !(min_x..=max_x).contains(&pt.x) {
            return false;
        }
        let min_y = min(self.origin.y, self.terminex.y);
        let max_y = max(self.origin.y, self.terminex.y);
        if !(min_y..=max_y).contains(&pt.y) {
            return false;
        }
        let min_z = min(self.origin.z, self.terminex.z);
        let max_z = max(self.origin.z, self.terminex.z);
        (min_z..=max_z).contains(&pt.z)
    }

    pub fn shift_down(&self) -> Cuboid {
        Cuboid {
            origin: self.origin.shift_z_down(),
            terminex: self.terminex.shift_z_down(),
        }
    }
}

impl From<(Point3D, Point3D)> for Cuboid {
    fn from((origin, terminex): (Point3D, Point3D)) -> Self {
        // origin should always be <= terminex
        let (origin, terminex) = if origin > terminex {
            (terminex, origin)
        } else {
            (origin, terminex)
        };
        Cuboid { origin, terminex }
    }
}

#[derive(Clone, Copy, Default, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct IVector3D {
    pub dx: i64,
    pub dy: i64,
    pub dz: i64,
}

impl IVector3D {
    pub fn to_f64_vec(&self) -> Vec<f64> {
        vec![self.dx as f64, self.dy as f64, self.dz as f64]
    }
}

impl FromStr for IVector3D {
    type Err = Error;

    fn from_str(point: &str) -> Result<Self> {
        let parsed: Vec<i64> = point.parse_split(',')?;
        parsed.try_into()
    }
}

impl fmt::Display for IVector3D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{},{}", self.dx, self.dy, self.dz)
    }
}

impl fmt::Debug for IVector3D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl TryFrom<Vec<i64>> for IVector3D {
    type Error = Error;

    fn try_from(value: Vec<i64>) -> std::prelude::v1::Result<Self, Self::Error> {
        if value.len() != 3 {
            bail!("invalid value, must have len 3");
        }
        Ok(IVector3D {
            dx: value[0],
            dy: value[1],
            dz: value[2],
        })
    }
}
