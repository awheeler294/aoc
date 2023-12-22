use std::{
    cmp::{max, min},
    ops::{Div, Mul, Rem},
};

pub fn least_common_multiple<T>(a: T, b: T) -> T
where
    T: Ord + Rem<Output = T> + Mul<Output = T> + Div<Output = T> + Default + Copy,
{
    a * b / greatest_common_denominator(a, b)
}

pub fn greatest_common_denominator<T>(a: T, b: T) -> T
where
    T: Ord + Rem<Output = T> + Mul<Output = T> + Div<Output = T> + Default + Copy,
{
    let (mut min, mut max) = (min(a, b), max(a, b));

    loop {
        let remainder = max % min;
        if remainder == T::default() {
            return min;
        }

        max = min;
        min = remainder;
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_lcm() {
        assert_eq!(least_common_multiple(15, 20), 60);
    }
}
