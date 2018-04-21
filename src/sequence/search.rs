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

    while (right-left).abs() >= absolute_precision {
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
    (right+left)/2f64 // found local maximum
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

#[macro_export]
macro_rules! binary_predecessor {
    ($sequence:expr, $val:expr) => {
        {
            let predecessor_rank = binary(&$sequence, &$val, true).unwrap() as isize - 1;
            if predecessor_rank >= 0 {
                Some(predecessor_rank)
            } else {
                None
            }
        }
    };
}

#[macro_export]
macro_rules! binary_successor {
    ($sequence:expr, $val:expr) => {
        {
            if let Some(val_idx) = binary(&$sequence, &$val, false) {
                let successor_rank = val_idx + 1;
                if successor_rank < $sequence.len() { Some(successor_rank) } else { None }
            } else {
                None
            }
        }
    };
}

#[macro_export]
macro_rules! binary_nearest_neighbor {
    ($sequence:expr, $val:expr) => {
        {
            let target_rank = binary(&$sequence, &$val, true).unwrap() as isize;
            let (predecessor_rank, mut predecessor_diff) = (target_rank-1, 0);
            let (successor_rank, mut successor_diff) = (target_rank+1, 0);
            None
        }
    }
}

pub fn binary<T: PartialOrd + PartialEq>(sequence: &[T], val: &T, rank: bool) -> Option<usize> {
    let (mut left, mut right) = (0, sequence.len() as isize - 1);

    while left <= right {
        let m = ((left+right) as f64 / 2f64).floor() as usize;
        if sequence[m] < *val {
            left = (m+1) as isize;
        } else if sequence[m] > *val {
            right = m as isize - 1;
        } else {
            return Some(m);
        }
    }
    if rank { Some(left as usize) } else { None }
}

#[cfg(test)]
mod binary_tests {
    use super::*;

    #[test]
    fn receives_integer_sequence() {
        let sequence: Vec<u32> = (0..100).collect();
        assert_eq!(binary(&sequence, &87, false), Some(87));
    }

    #[test]
    fn receives_char_sequence() {
        let sequence: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
        assert_eq!(binary(&sequence, &'g', false), Some(6));
    }

    #[test]
    fn receives_empty_sequence() {
        let sequence = vec![];
        assert_eq!(binary(&sequence, &1, false), None);
    }

    #[test]
    fn finds_non_existent_item() {
        let sequence: Vec<u32> = (0..100).collect();
        assert_eq!(binary(&sequence, &100, false), None);
    }

    #[test]
    fn returns_rank() {
        let sequence: Vec<u32> = (0..100).collect();
        assert_eq!(binary(&sequence, &87, true), Some(87));
    }

    #[test]
    fn returns_rank_for_non_existent_positive_item() {
        let sequence: Vec<u32> = (0..100).collect();
        assert_eq!(binary(&sequence, &500, true), Some(100));
    }

    #[test]
    fn returns_rank_for_non_existent_negative_item() {
        let sequence: Vec<i32> = (0..100).collect();
        assert_eq!(binary(&sequence, &-200, true), Some(0));
    }

    #[test]
    fn finds_predecessor() {
        let sequence: Vec<u32> = (0..100).collect();
        assert_eq!(binary_predecessor!(sequence, 56), Some(55));
    }

    #[test]
    fn finds_non_existent_predecessor() {
        let sequence: Vec<u32> = (0..100).collect();
        assert_eq!(binary_predecessor!(sequence, 0), None);
    }

    #[test]
    fn finds_predecessor_with_non_existent_positive_item() {
        let sequence: Vec<u32> = (0..100).collect();
        assert_eq!(binary_predecessor!(sequence, 500), Some(99));
    }

    #[test]
    fn finds_predecessor_with_non_existent_negative_item() {
        let sequence: Vec<i32> = (0..100).collect();
        assert_eq!(binary_predecessor!(sequence, -200), None);
    }

    #[test]
    fn finds_successor() {
        let sequence: Vec<u32> = (0..100).collect();
        assert_eq!(binary_successor!(sequence, 56), Some(57));
    }

    #[test]
    fn finds_non_existent_successor() {
        let sequence: Vec<u32> = (0..100).collect();
        assert_eq!(binary_successor!(sequence, 99), None);
    }

    #[test]
    fn finds_successor_with_non_existent_positive_item() {
        let sequence: Vec<u32> = (0..100).collect();
        assert_eq!(binary_successor!(sequence, 500), None);
    }

    #[test]
    fn finds_successor_with_non_existent_negative_item() {
        let sequence: Vec<i32> = (0..100).collect();
        assert_eq!(binary_successor!(sequence, -100), None);
    }

    #[test]
    fn finds_nearest_neighbor_returns_successor() {
        let sequence = vec![10, 20, 50, 60, 70, 75, 100];
        assert_eq!(binary_nearest_neighbor!(sequence, 50), Some(3));
    }

    #[test]
    fn finds_nearest_neighbor_returns_predecessor() {
        let sequence = vec![10, 20, 50, 60, 70, 75, 100];
        assert_eq!(binary_nearest_neighbor!(sequence, 75), Some(4));
    }

    #[test]
    fn finds_nearest_neighbor_with_equal_distance() {
        let sequence: Vec<u32> = (0..100).collect();
        assert_eq!(binary_successor!(sequence, 56), Some(55));
    }
}
