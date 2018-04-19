//! # Search
//!
//! A collection of functions to search for a value from a sequence/function.

/// # [Ternary Search](https://en.wikipedia.org/wiki/ternary)
///
/// Finds the maximum of a [unimodal](https://en.wikipedia.org/wiki/Unimodality#Unimodal_function) function.
///
/// # Examples
///
/// ```
/// use ult_algo::sequence::search;
///
/// let local_maximum = search::ternary(|x| x % 5.0, 25.4, 30.1, 0.05);
/// assert_eq!(local_maximum as u64, 29);
/// ```
///
/// # Panics
///
/// Case #1: Absolute precision is smaller than 1e-14
pub fn ternary<F>(f: F, mut left: f64, mut right: f64, absolute_precision: f64) -> f64
    where F: Fn(f64) -> f64
{
    // Ensure that the loop always ends.
    if absolute_precision < 1e-14 {
        panic!("absolute precision is too small");
    }
    loop {
        if (right-left).abs() < absolute_precision {
            return (right+left)/2f64 // found local maximum
        }

        // Move each value 1/3 higher or lower. Both values converge at the end.
        let left_third = left + (right-left)/3f64;
        let right_third = right - (right-left)/3f64;

        if f(left_third) < f(right_third) {
            left = left_third;
        } else {
            right = right_third;
        }
    }
}

#[cfg(test)]
mod ternary_tests {
    use super::ternary;

    #[test]
    fn receives_mod_function() {
        assert_eq!(ternary(|x| x % 5.0, 25.4, 30.1, 0.05) as u64, 29);
    }

    #[test]
    fn receives_power_function() {
        assert_eq!(ternary(|x| x.powf(x), 25.4, 30.1, 0.00001).ceil(), 31.0);
    }

    #[test]
    fn receives_smaller_right() {
        assert_eq!(ternary(|x| x % 5.0, 30.1, 25.4, 0.05) as u64, 29);
    }

    #[test]
    fn receives_negative_left_or_right() {
        assert_eq!(ternary(|x| x % 5.0, 30.1, -25.4, 0.05) as u64, 14);
    }

    #[test]
    fn receives_negative_left_and_right() {
        assert_eq!(ternary(|x| x % 5.0, -30.1, -25.4, 0.05) as u64, 14);
    }

    #[test]
    #[should_panic(expected = "absolute precision is too small")]
    fn receives_very_small_abs_precision() {
        ternary(|x| x % 5.0, 30.1, 25.4, 1e-15);
    }
}
