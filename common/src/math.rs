use std::ops::Rem;

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
