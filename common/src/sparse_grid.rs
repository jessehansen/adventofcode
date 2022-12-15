use crate::{pt, Bounds2D, Point2D};
use std::collections::HashMap;
use std::ops::Index;

#[derive(Debug)]
pub struct SparseGrid2D<T> {
    data: HashMap<Point2D, T>,
}

impl<T> FromIterator<(Point2D, T)> for SparseGrid2D<T> {
    fn from_iter<TIter: IntoIterator<Item = (Point2D, T)>>(iter: TIter) -> Self {
        SparseGrid2D {
            data: iter.into_iter().collect(),
        }
    }
}

impl<T> SparseGrid2D<T> {
    pub fn points(&self) -> impl Iterator<Item = &Point2D> {
        self.data.keys()
    }

    pub fn bottom_right(&self) -> Option<Point2D> {
        match self.data.len() {
            0 => None,
            _ => Some(pt(
                self.points().map(|Point2D { x, y: _ }| *x).max().unwrap(),
                self.points().map(|Point2D { x: _, y }| *y).max().unwrap(),
            )),
        }
    }

    pub fn bounds(&self) -> Option<Bounds2D> {
        self.bottom_right().map(|Point2D { x, y }| Bounds2D {
            width: x + 1,
            height: y + 1,
        })
    }

    pub fn get(&self, pt: &Point2D) -> Option<&T> {
        self.data.get(pt)
    }

    pub fn set(&mut self, pt: Point2D, value: T) -> Option<T> {
        self.data.insert(pt, value)
    }
}

impl<T> Clone for SparseGrid2D<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        SparseGrid2D {
            data: self.data.clone(),
        }
    }
}

impl<T> IntoIterator for SparseGrid2D<T> {
    type Item = (Point2D, T);
    type IntoIter = std::collections::hash_map::IntoIter<Point2D, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<T> Index<&Point2D> for SparseGrid2D<T> {
    type Output = T;

    fn index(&self, point: &Point2D) -> &T {
        &self.data[point]
    }
}
