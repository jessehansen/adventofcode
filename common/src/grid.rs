use itertools::Itertools;
use std::fmt;
use std::ops::{Index, IndexMut};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Point2D {
    pub x: usize,
    pub y: usize,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Bounds2D {
    pub width: usize,
    pub height: usize,
}

impl Point2D {
    fn bounded_relatives<T>(&self, bounds: Bounds2D, deltas: T) -> impl Iterator<Item = Point2D>
    where
        T: IntoIterator<Item = (i32, i32)>,
    {
        let x = self.x as i32;
        let y = self.y as i32;
        let width = bounds.width as i32;
        let height = bounds.height as i32;

        deltas
            .into_iter()
            .map(move |(dx, dy)| (x + dx, y + dy))
            .filter(move |(x, y)| *x >= 0 && *x <= width - 1 && *y >= 0 && *y <= height - 1)
            .map(|(x, y)| Point2D {
                x: x as usize,
                y: y as usize,
            })
    }

    pub fn cardinal_neighbors(&self, bounds: Bounds2D) -> impl Iterator<Item = Point2D> {
        self.bounded_relatives(bounds, [(-1, 0), (1, 0), (0, -1), (0, 1)])
    }

    pub fn neighbors(&self, bounds: Bounds2D) -> impl Iterator<Item = Point2D> {
        self.bounded_relatives(
            bounds,
            [
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, -1),
                (0, 1),
                (1, -1),
                (1, 0),
                (1, 1),
            ],
        )
    }
}

impl Bounds2D {
    pub fn iter_vertical(&self) -> impl Iterator<Item = Point2D> {
        (0..self.width)
            .cartesian_product(0..self.height)
            .map(|(x, y)| Point2D { x, y })
    }

    pub fn iter_horizontal(&self) -> impl Iterator<Item = Point2D> {
        (0..self.height)
            .cartesian_product(0..self.width)
            .map(|(y, x)| Point2D { x, y })
    }
}

#[derive(Debug)]
pub struct Grid2D<T> {
    pub data: Vec<Vec<T>>,
    pub bounds: Bounds2D,
}

impl<T> Grid2D<T> {
    pub fn new<I1, I2>(data: I2) -> Grid2D<T>
    where
        I1: IntoIterator<Item = T>,
        I2: IntoIterator<Item = I1>,
    {
        let data: Vec<Vec<T>> = data.into_iter().map(|x| x.into_iter().collect()).collect();
        let bounds = Bounds2D {
            width: data[0].len(),
            height: data.len(),
        };
        Grid2D { data, bounds }
    }

    pub fn rows(&self) -> impl Iterator<Item = impl Iterator<Item = &T>> {
        (0..self.bounds.height)
            .map(move |row| (0..self.bounds.width).map(move |col| &self.data[row][col]))
    }

    pub fn cols(&self) -> impl Iterator<Item = impl Iterator<Item = &T>> {
        (0..self.bounds.width)
            .map(move |col| (0..self.bounds.height).map(move |row| &self.data[row][col]))
    }

    pub fn enumerate_rows(&self) -> impl Iterator<Item = (usize, impl Iterator<Item = &T>)> {
        self.rows().enumerate()
    }

    pub fn enumerate_cols(&self) -> impl Iterator<Item = (usize, impl Iterator<Item = &T>)> {
        self.cols().enumerate()
    }

    pub fn iter_vertical(&self) -> impl Iterator<Item = (Point2D, &T)> {
        self.bounds.iter_vertical().map(|pt| (pt, &self[pt]))
    }

    pub fn iter_horizontal(&self) -> impl Iterator<Item = (Point2D, &T)> {
        self.bounds.iter_horizontal().map(|pt| (pt, &self[pt]))
    }

    pub fn transform<F>(&mut self, mut f: F)
    where
        F: FnMut((Point2D, &T)) -> T,
    {
        self.bounds.iter_horizontal().for_each(|pt| {
            self[pt] = f((pt, &self[pt]));
        });
    }
}

impl<T> Index<Point2D> for Grid2D<T> {
    type Output = T;

    fn index(&self, point: Point2D) -> &Self::Output {
        if point.x >= self.bounds.width || point.y >= self.bounds.height {
            panic!("index out of bounds");
        }

        &self.data[point.y][point.x]
    }
}

impl<T> IndexMut<Point2D> for Grid2D<T> {
    fn index_mut(&mut self, point: Point2D) -> &mut Self::Output {
        if point.x >= self.bounds.width || point.y >= self.bounds.height {
            panic!("index out of bounds");
        }

        &mut self.data[point.y][point.x]
    }
}

impl<T, I1> FromIterator<I1> for Grid2D<T>
where
    I1: IntoIterator<Item = T>,
{
    fn from_iter<I2>(iter: I2) -> Self
    where
        I2: IntoIterator<Item = I1>,
    {
        Grid2D::new(iter)
    }
}

impl<T> Clone for Grid2D<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Grid2D {
            data: self.data.clone(),
            bounds: self.bounds.clone(),
        }
    }
}

impl<T> fmt::Display for Grid2D<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.data
                .iter()
                .map(|row| row
                    .iter()
                    .map(|x| format!("{}", x))
                    .collect::<Vec<String>>()
                    .join(""))
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_neighbors_middle_of_grid() {
        let point = Point2D { x: 2, y: 3 };
        let bounds = Bounds2D {
            width: 5,
            height: 10,
        };
        let points: Vec<Point2D> = point.neighbors(bounds).collect();
        assert_eq!(
            points,
            vec![
                Point2D { x: 1, y: 3 },
                Point2D { x: 3, y: 3 },
                Point2D { x: 2, y: 2 },
                Point2D { x: 2, y: 4 }
            ]
        );
    }

    #[test]
    fn point_neighbors_edge_of_grid() {
        let point = Point2D { x: 0, y: 3 };
        let bounds = Bounds2D {
            width: 5,
            height: 10,
        };
        let points: Vec<Point2D> = point.neighbors(bounds).collect();
        assert_eq!(
            points,
            vec![
                Point2D { x: 1, y: 3 },
                Point2D { x: 0, y: 2 },
                Point2D { x: 0, y: 4 }
            ]
        );

        let point = Point2D { x: 4, y: 3 };
        let points: Vec<Point2D> = point.neighbors(bounds).collect();
        assert_eq!(
            points,
            vec![
                Point2D { x: 3, y: 3 },
                Point2D { x: 4, y: 2 },
                Point2D { x: 4, y: 4 }
            ]
        );

        let point = Point2D { x: 2, y: 0 };
        let points: Vec<Point2D> = point.neighbors(bounds).collect();
        assert_eq!(
            points,
            vec![
                Point2D { x: 1, y: 0 },
                Point2D { x: 3, y: 0 },
                Point2D { x: 2, y: 1 }
            ]
        );

        let point = Point2D { x: 2, y: 9 };
        let points: Vec<Point2D> = point.neighbors(bounds).collect();
        assert_eq!(
            points,
            vec![
                Point2D { x: 1, y: 9 },
                Point2D { x: 3, y: 9 },
                Point2D { x: 2, y: 8 }
            ]
        );

        let point = Point2D { x: 0, y: 0 };
        let points: Vec<Point2D> = point.neighbors(bounds).collect();
        assert_eq!(points, vec![Point2D { x: 1, y: 0 }, Point2D { x: 0, y: 1 }]);
    }

    #[test]
    fn bounds_iter_vertical() {
        let bounds = Bounds2D {
            width: 2,
            height: 3,
        };

        let points: Vec<Point2D> = bounds.iter_vertical().collect();
        assert_eq!(
            points,
            vec![
                Point2D { x: 0, y: 0 },
                Point2D { x: 0, y: 1 },
                Point2D { x: 0, y: 2 },
                Point2D { x: 1, y: 0 },
                Point2D { x: 1, y: 1 },
                Point2D { x: 1, y: 2 }
            ]
        );
    }

    #[test]
    fn bounds_iter_horizontal() {
        let bounds = Bounds2D {
            width: 2,
            height: 3,
        };

        let points: Vec<Point2D> = bounds.iter_horizontal().collect();
        assert_eq!(
            points,
            vec![
                Point2D { x: 0, y: 0 },
                Point2D { x: 1, y: 0 },
                Point2D { x: 0, y: 1 },
                Point2D { x: 1, y: 1 },
                Point2D { x: 0, y: 2 },
                Point2D { x: 1, y: 2 }
            ]
        );
    }

    fn sample_grid() -> Grid2D<u32> {
        vec![vec![1, 2, 3], vec![4, 5, 6]].into_iter().collect()
    }

    #[test]
    fn grid_index() {
        let grid = sample_grid();

        let value: u32 = grid[Point2D { x: 2, y: 1 }];
        assert_eq!(value, 6_u32);
    }

    #[test]
    fn grid_enumerate_rows() {
        let grid = sample_grid();

        let rows: Vec<(usize, Vec<&u32>)> = grid
            .enumerate_rows()
            .map(|(pos, row)| (pos, row.collect()))
            .collect();

        assert_eq!(rows, vec![(0, vec![&1, &2, &3]), (1, vec![&4, &5, &6]),]);
    }

    #[test]
    fn grid_enumerate_cols() {
        let grid = sample_grid();

        let cols: Vec<(usize, Vec<&u32>)> = grid
            .enumerate_cols()
            .map(|(pos, col)| (pos, col.collect()))
            .collect();

        assert_eq!(
            cols,
            vec![(0, vec![&1, &4]), (1, vec![&2, &5]), (2, vec![&3, &6]),]
        );
    }

    #[test]
    fn grid_iter_horizontal() {
        let grid = sample_grid();

        let result: Vec<(Point2D, &u32)> = grid.iter_horizontal().collect();

        assert_eq!(
            result,
            vec![
                (Point2D { x: 0, y: 0 }, &1),
                (Point2D { x: 1, y: 0 }, &2),
                (Point2D { x: 2, y: 0 }, &3),
                (Point2D { x: 0, y: 1 }, &4),
                (Point2D { x: 1, y: 1 }, &5),
                (Point2D { x: 2, y: 1 }, &6),
            ]
        );
    }

    #[test]
    fn grid_iter_vertical() {
        let grid = sample_grid();

        let result: Vec<(Point2D, &u32)> = grid.iter_vertical().collect();

        assert_eq!(
            result,
            vec![
                (Point2D { x: 0, y: 0 }, &1),
                (Point2D { x: 0, y: 1 }, &4),
                (Point2D { x: 1, y: 0 }, &2),
                (Point2D { x: 1, y: 1 }, &5),
                (Point2D { x: 2, y: 0 }, &3),
                (Point2D { x: 2, y: 1 }, &6),
            ]
        );
    }
}
