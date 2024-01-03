use std::{
    cmp::{max, min},
    ops::{Range, Rem},
};

use crate::IPoint2D;

pub fn least_common_multiple(nums: &[usize]) -> usize {
    let mut result = 1;
    for &num in nums {
        result = num * result / greatest_common_denominator(num, result);
    }
    result
}

pub fn greatest_common_denominator<T>(a: T, b: T) -> T
where
    T: Default + Copy + Eq + Rem<Output = T>,
{
    if b == Default::default() {
        return a;
    }

    greatest_common_denominator(b, a % b)
}

// Shoelace formula.  https://en.wikipedia.org/wiki/Shoelace_formula
//
// There are two implementations because most won't need 64-bit values,
// but some do
pub fn shoelace_loop_area(vertices: Vec<IPoint2D>) -> i32 {
    let mut area: i32 = 0;
    let len = vertices.len();
    // there's probably a smarter windowing function I could do here, but I'm tired
    for i in 0..len {
        let j = (i + 1) % len;
        area += vertices[i].x * vertices[j].y - vertices[j].x * vertices[i].y;
    }

    area.abs() / 2
}

pub fn shoelace_loop_area_64(vertices: Vec<IPoint2D>) -> i64 {
    let mut area: i64 = 0;
    let len = vertices.len();
    for i in 0..len {
        let j = (i + 1) % len;
        area += i64::from(vertices[i].x) * i64::from(vertices[j].y)
            - i64::from(vertices[j].x) * i64::from(vertices[i].y);
    }

    area.abs() / 2
}

pub trait RangeOperations {
    type RangeOutput;

    fn intersect(&self, other: &Self) -> Option<Self::RangeOutput>;
    fn offset(&self, delta: usize) -> Self::RangeOutput;
    fn difference(&self, other: &Self) -> Vec<Self::RangeOutput>;
}

impl RangeOperations for Range<usize> {
    type RangeOutput = Self;

    fn intersect(&self, other: &Self) -> Option<Self::RangeOutput> {
        if self.start < other.end && self.end > other.start {
            Some((max(self.start, other.start))..(min(self.end, other.end)))
        } else {
            None
        }
    }

    fn offset(&self, delta: usize) -> Self::RangeOutput {
        (self.start + delta)..(self.end + delta)
    }

    fn difference(&self, other: &Self) -> Vec<Self::RangeOutput> {
        if self.start >= other.end || self.end <= other.start {
            // no overlap
            vec![self.clone()]
        } else {
            let mut diff = vec![];
            if self.start < other.start {
                diff.push((self.start)..(other.start));
            }
            if self.end > other.end {
                diff.push((other.end)..(self.end))
            }
            diff
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn range_difference_tests() {
        assert_eq!(vec![61..73], (60..73).difference(&(60..61)));
        assert_eq!(vec![10..14, 17..20], (10..20).difference(&(14..17)));
        assert_eq!(vec![90..95], (90..100).difference(&(95..110)));
        assert_eq!(vec![1..10], (1..10).difference(&(95..110)));
        assert_eq!(vec![91..100], (91..100).difference(&(1..10)));
    }
}
