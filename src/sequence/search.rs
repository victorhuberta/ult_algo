//! # Search
//!
//! A collection of functions to search for a value from a sequence/function.

/// Brings all sequence search types and functions required by macros into scope.
#[macro_export]
macro_rules! include_sequence_search {
    () => {
        use ult_algo::sequence::search::{SearchTarget, ternary};
    };
}

/// # [Ternary Maximum](https://en.wikipedia.org/wiki/ternary)
///
/// Finds the maximum of a
///  [unimodal](https://en.wikipedia.org/wiki/Unimodality#Unimodal_function) function.
///
/// # Examples
///
/// ```
/// #[macro_use]
/// extern crate ult_algo;
///
/// include_sequence_search!();
///
/// fn main() {
///     let local_maximum = ternary_max!(|x| x % 5.0, 25.4, 30.1, 0.05);
///     assert_eq!(local_maximum, 29.990194395991274);
/// }
/// ```
///
/// # Panics
///
/// Case 1: Absolute precision is smaller than 1e-14
#[macro_export]
macro_rules! ternary_max {
    ($f:expr, $left:expr, $right:expr, $absolute_precision:expr) => {
        ternary(SearchTarget::Maximum, $f, $left, $right, $absolute_precision)
    };
}

/// # [Ternary Minimum](https://en.wikipedia.org/wiki/ternary)
///
/// Finds the minimum of a
///  [unimodal](https://en.wikipedia.org/wiki/Unimodality#Unimodal_function) function.
///
/// # Examples
///
/// ```
/// #[macro_use]
/// extern crate ult_algo;
///
/// include_sequence_search!();
///
/// fn main() {
///     let local_minimum = ternary_min!(|x| x % 5.0, 25.4, 30.1, 0.05);
///     assert_eq!(local_minimum, 25.41811226457876);
/// }
/// ```
///
/// # Panics
///
/// Case 1: Absolute precision is smaller than 1e-14
#[macro_export]
macro_rules! ternary_min {
    ($f:expr, $left:expr, $right:expr, $absolute_precision:expr) => {
        ternary(SearchTarget::Minimum, $f, $left, $right, $absolute_precision)
    };
}

/// # [Ternary](https://en.wikipedia.org/wiki/ternary)
///
/// Finds the minimum or maximum of a
///  [unimodal](https://en.wikipedia.org/wiki/Unimodality#Unimodal_function) function.
///
/// # Examples
///
/// ```
/// use ult_algo::sequence::search;
///
/// let search_target = search::SearchTarget::Maximum;
/// let local_maximum = search::ternary(search_target, |x| x % 5.0, 25.4, 30.1, 0.05);
/// assert_eq!(local_maximum, 29.990194395991274);
/// ```
///
/// # Panics
///
/// Case 1: Absolute precision is smaller than 1e-14
pub fn ternary<F>(
    search_target: SearchTarget,
    f: F,
    mut left: f64,
    mut right: f64,
    absolute_precision: f64
) -> f64
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

        // Continue based on minimum or maximum search.
        let result_comparison = match search_target {
            SearchTarget::Minimum => f(left_third) > f(right_third),
            SearchTarget::Maximum => f(left_third) < f(right_third)
        };
        if result_comparison {
            left = left_third;
        } else {
            right = right_third;
        }
    }
}

/// Enumerates the kinds of a search target.
pub enum SearchTarget {
    Minimum,
    Maximum
}

#[cfg(test)]
mod ternary_tests {
    use super::*;

    #[test]
    fn finds_max_and_receives_mod_function() {
        let search_target = SearchTarget::Maximum;
        assert_eq!(ternary(search_target, |x| x % 5.0, 25.4, 30.1, 0.05), 29.990194395991274);
    }

    #[test]
    fn finds_max_and_receives_power_function() {
        let search_target = SearchTarget::Maximum;
        assert_eq!(ternary(search_target, |x| x.powf(x), 25.4, 30.1, 0.00001), 30.099996368748634);
    }

    #[test]
    fn finds_max_and_receives_smaller_right() {
        let search_target = SearchTarget::Maximum;
        assert_eq!(ternary(search_target, |x| x % 5.0, 30.1, 25.4, 0.05), 29.990194395991274);
    }

    #[test]
    fn finds_max_and_receives_negative_left_or_right() {
        let search_target = SearchTarget::Maximum;
        assert_eq!(ternary(search_target, |x| x % 5.0, 30.1, -25.4, 0.05), 14.983253353180297);
    }

    #[test]
    fn finds_max_and_receives_negative_left_and_right() {
        let search_target = SearchTarget::Maximum;
        assert_eq!(ternary(search_target, |x| x % 5.0, -30.1, -25.4, 0.05), -25.41811226457876);
    }

    #[test]
    fn finds_min_and_receives_power_function() {
        let search_target = SearchTarget::Minimum;
        assert_eq!(ternary(search_target, |x| x.powf(x), 25.4, 30.1, 0.00001), 25.400003631251366);
    }

    #[test]
    fn finds_min_and_receives_smaller_right() {
        let search_target = SearchTarget::Minimum;
        assert_eq!(ternary(search_target, |x| x % 5.0, 30.1, 25.4, 0.05), 25.41811226457876);
    }

    #[test]
    fn finds_min_and_receives_negative_left_or_right() {
        let search_target = SearchTarget::Minimum;
        assert_eq!(ternary(search_target, |x| x % 5.0, 30.1, -25.4, 0.05), -9.985704278536492);
    }

    #[test]
    fn finds_min_and_receives_negative_left_and_right() {
        let search_target = SearchTarget::Minimum;
        assert_eq!(ternary(search_target, |x| x % 5.0, -30.1, -25.4, 0.05), -29.990194395991274);
    }

    #[test]
    #[should_panic(expected = "absolute precision is too small")]
    fn receives_very_small_abs_precision() {
        let search_target = SearchTarget::Maximum;
        ternary(search_target, |x| x % 5.0, 30.1, 25.4, 1e-15);
    }

    #[test]
    fn use_ternary_max_macro() {
        assert_eq!(ternary_max!(|x| x % 5.0, 25.4, 30.1, 0.05), 29.990194395991274);
    }

    #[test]
    fn use_ternary_min_macro() {
        assert_eq!(ternary_min!(|x| x.powf(x), 25.4, 30.1, 0.00001), 25.400003631251366);
    }
}
