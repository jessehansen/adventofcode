use anyhow::*;
use itertools::Itertools;
use std::cmp::{max, min, Eq, Ord, Ordering, PartialEq};
use std::collections::BinaryHeap;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::{Index, IndexMut};
use std::str::FromStr;

use crate::wrap_parse_error;

// contains helpers for grids and unsigned points
// coordinates are laid out like this
// +------------------------  y = 0
// |
// |
// |    *
// |
// |
// x = 0
// The * is at (x=4, y=3)

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Point2D {
    pub x: usize,
    pub y: usize,
}

#[inline]
pub fn pt(x: usize, y: usize) -> Point2D {
    Point2D { x, y }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Bounds2D {
    pub width: usize,
    pub height: usize,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Direction {
    Up,
    Left,
    Right,
    Down,
}

impl Point2D {
    pub const ORIGIN: Point2D = Point2D { x: 0, y: 0 };

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
            .filter(move |(x, y)| *x >= 0 && *x < width && *y >= 0 && *y < height)
            .map(|(x, y)| pt(x as usize, y as usize))
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

    pub fn left(&self) -> Option<Point2D> {
        if self.x > 0 {
            Some(pt(self.x - 1, self.y))
        } else {
            None
        }
    }

    pub fn right(&self, width: usize) -> Option<Point2D> {
        if self.x < width - 1 {
            Some(pt(self.x + 1, self.y))
        } else {
            None
        }
    }

    pub fn right_unbounded(&self) -> Point2D {
        pt(self.x + 1, self.y)
    }

    pub fn up(&self) -> Option<Point2D> {
        if self.y > 0 {
            Some(pt(self.x, self.y - 1))
        } else {
            None
        }
    }

    pub fn down(&self, height: usize) -> Option<Point2D> {
        if self.y < height - 1 {
            Some(pt(self.x, self.y + 1))
        } else {
            None
        }
    }

    pub fn down_unbounded(&self) -> Point2D {
        pt(self.x, self.y + 1)
    }

    pub fn cardinal_neighbor(&self, direction: Direction, bounds: Bounds2D) -> Option<Point2D> {
        match direction {
            Direction::Left => self.left(),
            Direction::Right => self.right(bounds.width),
            Direction::Up => self.up(),
            Direction::Down => self.down(bounds.height),
        }
    }

    pub fn manhattan_distance(&self, other: Point2D) -> usize {
        max(self.x, other.x) - min(self.x, other.x) + max(self.y, other.y) - min(self.y, other.y)
    }

    pub fn to(&self, other: &Point2D) -> impl Iterator<Item = Point2D> {
        let min_x = min(self.x, other.x);
        let min_y = min(self.y, other.y);
        let max_x = max(self.x, other.x);
        let max_y = max(self.y, other.y);
        (min_x..=max_x)
            .cartesian_product(min_y..=max_y)
            .map(|(x, y)| pt(x, y))
    }
}

impl FromStr for Point2D {
    type Err = Error;

    fn from_str(input: &str) -> Result<Point2D> {
        let mut parts = input.split(',').map(|x| x.parse());

        let x = parts.next().ok_or_else(|| anyhow!("missing x value"))??;
        let y = parts.next().ok_or_else(|| anyhow!("missing y value"))??;

        if parts.next().is_some() {
            bail!("received extra coordinates for Point2D");
        }

        Ok(pt(x, y))
    }
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{}", self.x, self.y,)
    }
}

impl Bounds2D {
    pub fn iter_vertical(&self) -> impl Iterator<Item = Point2D> {
        (0..self.width)
            .cartesian_product(0..self.height)
            .map(|(x, y)| pt(x, y))
    }

    pub fn iter_horizontal(&self) -> impl Iterator<Item = Point2D> {
        (0..self.height)
            .cartesian_product(0..self.width)
            .map(|(y, x)| pt(x, y))
    }

    pub fn iter_horizontal_rev(&self) -> impl Iterator<Item = Point2D> {
        (0..self.height)
            .rev()
            .cartesian_product((0..self.width).rev())
            .map(move |(y, x)| pt(x, y))
    }
    pub fn bottom_right(&self) -> Point2D {
        pt(self.width - 1, self.height - 1)
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
        self.bounds
            .iter_vertical()
            .map(|pt| (pt, &self.data[pt.y][pt.x]))
    }

    pub fn iter_horizontal(&self) -> impl Iterator<Item = (Point2D, &T)> {
        self.bounds
            .iter_horizontal()
            .map(|pt| (pt, &self.data[pt.y][pt.x]))
    }

    pub fn cardinal_neighbors(&self, pt: Point2D) -> impl Iterator<Item = (Point2D, &T)> {
        pt.cardinal_neighbors(self.bounds)
            .map(|pt| (pt, &self.data[pt.y][pt.x]))
    }

    pub fn neighbors(&self, pt: Point2D) -> impl Iterator<Item = (Point2D, &T)> {
        pt.neighbors(self.bounds)
            .map(|pt| (pt, &self.data[pt.y][pt.x]))
    }

    pub fn transform<F>(&mut self, mut f: F)
    where
        F: FnMut((Point2D, &T)) -> T,
    {
        self.bounds.iter_horizontal().for_each(|pt| {
            self[pt] = f((pt, &self[pt]));
        });
    }

    pub fn transform_neighbors<F>(&mut self, pt: Point2D, mut f: F)
    where
        F: FnMut((Point2D, &T)) -> T,
    {
        pt.neighbors(self.bounds)
            .for_each(|pt| self[pt] = f((pt, &self.data[pt.y][pt.x])));
    }

    pub fn transform_cardinal_neighbors<F>(&mut self, pt: Point2D, mut f: F)
    where
        F: FnMut((Point2D, &T)) -> T,
    {
        pt.cardinal_neighbors(self.bounds)
            .for_each(|pt| self[pt] = f((pt, &self.data[pt.y][pt.x])));
    }

    pub fn bottom_right(&self) -> &T {
        let pt = self.bounds.bottom_right();
        &self.data[pt.y][pt.x]
    }
}

// basically a reverse sorter for T, with the location along for the ride
#[derive(Copy, Clone, Eq, PartialEq)]
struct ShortestPathState<T> {
    distance: T,
    pt: Point2D,
}

impl<T> Ord for ShortestPathState<T>
where
    T: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .distance
            .cmp(&self.distance)
            .then_with(|| self.pt.cmp(&other.pt))
    }
}

impl<T> PartialOrd for ShortestPathState<T>
where
    T: Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Grid2D<T>
where
    T: Default + Ord + Copy + std::ops::Add<Output = T>,
{
    // Dijkstra’s algorithm
    pub fn shortest_path(&self) -> T {
        let mut dist = Grid2D::<Option<T>>::new_constant(self.bounds, None);
        dist[Point2D::ORIGIN] = Default::default();

        let mut heap = BinaryHeap::new();
        heap.push(ShortestPathState {
            distance: Default::default(),
            pt: Point2D::ORIGIN,
        });

        // using a binary heap means we're always looking at the next best unvisited node
        while let Some(ShortestPathState { distance, pt }) = heap.pop() {
            if pt == self.bounds.bottom_right() {
                return distance;
            }

            // if we're already above the previous best to this point, don't bother continuing
            match dist[pt] {
                Some(prev_dist) if distance > prev_dist => {
                    continue;
                }
                _ => (),
            }

            for (pt, dist_there) in self.cardinal_neighbors(pt) {
                let dist_next = distance + *dist_there;

                // if this is the best distance calculated so far, push it on the heap
                match dist[pt] {
                    Some(prev_dist) if dist_next >= prev_dist => (),
                    _ => {
                        heap.push(ShortestPathState {
                            distance: dist_next,
                            pt,
                        });
                        dist[pt] = Some(dist_next);
                    }
                }
            }
        }

        Default::default()
    }
}

impl<T> Grid2D<T>
where
    T: Copy,
{
    pub fn new_constant(bounds: Bounds2D, value: T) -> Grid2D<T> {
        let data: Vec<Vec<T>> = vec![vec![value; bounds.width]; bounds.height];
        Grid2D { data, bounds }
    }
}

impl<T> Grid2D<T>
where
    T: Copy + Clone,
{
    pub fn grow_y(&self, by: usize, fill: T) -> Grid2D<T> {
        let mut data: Vec<Vec<T>> = self.data.clone();
        for _ in 0..by {
            data.push(vec![fill; self.bounds.width]);
        }
        Grid2D {
            data,
            bounds: Bounds2D {
                width: self.bounds.width,
                height: self.bounds.height + by,
            },
        }
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
            bounds: self.bounds,
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
                    .map(|x| format!("{x}"))
                    .collect::<Vec<String>>()
                    .join(""))
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

impl<T> Grid2D<T>
where
    T: fmt::Display,
{
    pub fn to_string_with_cell_width(&self, width: usize) -> String {
        self.data
            .iter()
            .map(|row| {
                row.iter()
                    .map(|x| format!("{x:width$}"))
                    .collect::<Vec<String>>()
                    .join("")
            })
            .collect::<Vec<String>>()
            .join("\n")
    }
}

impl<T> Grid2D<T> {
    pub fn to_string_format_cell<F>(&self, formatter: F) -> String
    where
        F: Copy + FnMut(&T) -> String,
    {
        self.data
            .iter()
            .map(move |row| row.iter().map(formatter).collect::<Vec<String>>().join(""))
            .collect::<Vec<String>>()
            .join("\n")
    }
}

impl<T> Grid2D<T>
where
    T: FromStr + Default,
    <T as FromStr>::Err: std::fmt::Display,
{
    // this is a special case where each grid item is only represented by a single character
    pub fn from_char_str(input: &str) -> Result<Grid2D<T>> {
        input
            .lines()
            .map(|x| -> Result<Vec<T>> {
                x.chars()
                    .map(|x| -> Result<T> { Ok(wrap_parse_error(x.to_string().parse())?) })
                    .collect()
            })
            .collect()
    }

    pub fn from_delimited_str(input: &str, delimiter: &str) -> Result<Grid2D<T>> {
        input
            .lines()
            .map(|x| -> Result<Vec<T>> {
                x.split(delimiter)
                    .map(|x| Ok(wrap_parse_error(x.to_string().parse())?))
                    .collect()
            })
            .collect()
    }
}

impl<T> Hash for Grid2D<T>
where
    T: Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.data.hash(state);
    }
}

impl<T> PartialEq for Grid2D<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl<T> Eq for Grid2D<T> where T: Eq {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_neighbors_middle_of_grid() {
        let point = pt(2, 3);
        let bounds = Bounds2D {
            width: 5,
            height: 10,
        };
        let points: Vec<Point2D> = point.cardinal_neighbors(bounds).collect();
        assert_eq!(points, vec![pt(1, 3), pt(3, 3), pt(2, 2), pt(2, 4)]);
    }

    #[test]
    fn point_neighbors_edge_of_grid() {
        let point = pt(0, 3);
        let bounds = Bounds2D {
            width: 5,
            height: 10,
        };
        let points: Vec<Point2D> = point.cardinal_neighbors(bounds).collect();
        assert_eq!(points, vec![pt(1, 3), pt(0, 2), pt(0, 4)]);

        let point = pt(4, 3);
        let points: Vec<Point2D> = point.cardinal_neighbors(bounds).collect();
        assert_eq!(points, vec![pt(3, 3), pt(4, 2), pt(4, 4)]);

        let point = pt(2, 0);
        let points: Vec<Point2D> = point.cardinal_neighbors(bounds).collect();
        assert_eq!(points, vec![pt(1, 0), pt(3, 0), pt(2, 1)]);

        let point = pt(2, 9);
        let points: Vec<Point2D> = point.cardinal_neighbors(bounds).collect();
        assert_eq!(points, vec![pt(1, 9), pt(3, 9), pt(2, 8)]);

        let point = pt(0, 0);
        let points: Vec<Point2D> = point.cardinal_neighbors(bounds).collect();
        assert_eq!(points, vec![pt(1, 0), pt(0, 1)]);
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
            vec![pt(0, 0), pt(0, 1), pt(0, 2), pt(1, 0), pt(1, 1), pt(1, 2)]
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
            vec![pt(0, 0), pt(1, 0), pt(0, 1), pt(1, 1), pt(0, 2), pt(1, 2)]
        );
    }

    fn sample_grid() -> Grid2D<u32> {
        vec![vec![1, 2, 3], vec![4, 5, 6]].into_iter().collect()
    }

    #[test]
    fn grid_index() {
        let grid = sample_grid();

        let value: u32 = grid[pt(2, 1)];
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
                (pt(0, 0), &1),
                (pt(1, 0), &2),
                (pt(2, 0), &3),
                (pt(0, 1), &4),
                (pt(1, 1), &5),
                (pt(2, 1), &6),
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
                (pt(0, 0), &1),
                (pt(0, 1), &4),
                (pt(1, 0), &2),
                (pt(1, 1), &5),
                (pt(2, 0), &3),
                (pt(2, 1), &6),
            ]
        );
    }
}
