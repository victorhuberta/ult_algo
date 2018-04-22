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
macro_rules! binary {
    ($sequence:expr, $val:expr) => {
        binary(&$sequence, &$val).index
    };
}

#[macro_export]
macro_rules! binary_rank {
    ($sequence:expr, $val:expr) => {
        binary(&$sequence, &$val).rank
    };
}

#[macro_export]
macro_rules! binary_predecessor {
    ($sequence:expr, $val:expr) => {
        {
            let predecessor_idx = binary(&$sequence, &$val).rank as isize - 1;
            if predecessor_idx >= 0 { Some(predecessor_idx) } else { None }
        }
    };
}

#[macro_export]
macro_rules! binary_successor {
    ($sequence:expr, $val:expr) => {
        {
            if $sequence.len() == 0 {
                return None;
            }
            let result = binary(&$sequence, &$val);
            if result.index == None && result.rank == 0 { // target is out of range and smaller
                Some(0)
            } else if let Some(target_idx) = result.index {
                let successor_idx = target_idx + 1;
                if successor_idx < $sequence.len() { Some(successor_idx) } else { None }
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
            if $sequence.len() == 0 {
                return None;
            }
            let result = binary(&$sequence, &$val);
            // If target is out of range and smaller, return first index.
            // If target is the first item, return second index.
            if result.rank == 0 {
                return if result.index == None { Some(0) } else { Some(1) };
            }

            let predecessor_idx = result.rank as isize - 1;
            let successor_idx = result.rank + 1;
            // If target is in range, then compare diffs and return that with a larger diff.
            if predecessor_idx >= 0 && successor_idx < $sequence.len() {
                let predecessor_idx = predecessor_idx as usize; // required for indexing
                let predecessor_diff = $sequence[result.rank] - $sequence[predecessor_idx];
                let successor_diff = $sequence[successor_idx] - $sequence[result.rank];
                if predecessor_diff >= successor_diff {
                    return Some(predecessor_idx);
                } else {
                    return Some(successor_idx);
                }
            } else {
                // If target is the last item, return second-to-last index.
                // If target is out of range and larger, return last index.
                return Some(predecessor_idx);
            }
        }
    }
}

pub fn binary<T: PartialOrd + PartialEq>(sequence: &[T], val: &T) -> BinarySearchResult {
    let (mut left, mut right) = (0, sequence.len() as isize - 1);

    while left <= right {
        let m = ((left+right) as f64 / 2f64).floor() as usize;
        if sequence[m] < *val {
            left = (m+1) as isize;
        } else if sequence[m] > *val {
            right = m as isize - 1;
        } else {
            return BinarySearchResult::new(Some(m), m);
        }
    }
    BinarySearchResult::new(None, left as usize)
}

pub struct BinarySearchResult {
    index: Option<usize>,
    rank: usize
}

impl BinarySearchResult {
    pub fn new(index: Option<usize>, rank: usize) -> BinarySearchResult {
        if let Some(index) = index {
            if rank > index {
                panic!("rank should be less than or equal to index");
            }
        }
        BinarySearchResult { index, rank }
    }
}

#[cfg(test)]
mod binary_tests {
    use super::*;

    #[test]
    fn receives_integer_sequence() {
        let sequence: Vec<u32> = (0..100).collect();
        let result = binary(&sequence, &87);
        assert_eq!(result.index.unwrap(), 87);
        assert_eq!(result.rank, 87);
    }

    #[test]
    fn receives_char_sequence() {
        let sequence: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
        let result = binary(&sequence, &'g');
        assert_eq!(result.index.unwrap(), 6);
        assert_eq!(result.rank, 6);
    }

    #[test]
    fn receives_empty_sequence() {
        let sequence = vec![];
        let result = binary(&sequence, &1);
        assert_eq!(result.index, None);
        assert_eq!(result.rank, 0);
    }

    #[test]
    fn finds_non_existent_large_item() {
        let sequence: Vec<u32> = (0..100).collect();
        let result = binary(&sequence, &100);
        assert_eq!(result.index, None);
        assert_eq!(result.rank, 100);
    }

    #[test]
    fn finds_non_existent_small_item() {
        let sequence: Vec<i32> = (0..100).collect();
        let result = binary(&sequence, &-200);
        assert_eq!(result.index, None);
        assert_eq!(result.rank, 0);
    }

    #[test]
    fn finds_predecessor() {
        let sequence: Vec<u32> = (0..100).collect();
        assert_eq!(binary_predecessor!(sequence, 56).unwrap(), 55);
    }

    #[test]
    fn finds_non_existent_predecessor() {
        let sequence: Vec<u32> = (0..100).collect();
        assert_eq!(binary_predecessor!(sequence, 0), None);
    }

    #[test]
    fn finds_predecessor_with_non_existent_out_of_range_larger_item() {
        let sequence: Vec<u32> = (0..100).collect();
        assert_eq!(binary_predecessor!(sequence, 500).unwrap(), 99);
    }

    #[test]
    fn finds_predecessor_with_non_existent_out_of_range_smaller_item() {
        let sequence: Vec<i32> = (0..100).collect();
        assert_eq!(binary_predecessor!(sequence, -200), None);
    }

    #[test]
    fn finds_predecessor_with_non_existent_in_range_item() {
        let sequence = vec![1, 4, 5, 10, 30, 50, 80, 90];
        assert_eq!(binary_predecessor!(sequence, 40).unwrap(), 4);
    }

    #[test]
    fn finds_successor() {
        let sequence: Vec<u32> = (0..100).collect();
        assert_eq!(binary_successor!(sequence, 56).unwrap(), 57);
    }

    #[test]
    fn finds_non_existent_successor() {
        let sequence: Vec<u32> = (0..100).collect();
        assert_eq!(binary_successor!(sequence, 99), None);
    }

    #[test]
    fn finds_successor_with_non_existent_out_of_range_larger_item() {
        let sequence: Vec<u32> = (0..100).collect();
        assert_eq!(binary_successor!(sequence, 500), None);
    }

    #[test]
    fn finds_successor_with_non_existent_out_of_range_smaller_item() {
        let sequence: Vec<i32> = (0..100).collect();
        assert_eq!(binary_successor!(sequence, -100).unwrap(), 0);
    }

    #[test]
    fn finds_successor_with_non_existent_in_range_item() {
        let sequence = vec![1, 4, 5, 10, 30, 50, 80, 90];
        assert_eq!(binary_successor!(sequence, 40).unwrap(), 5);
    }

    #[test]
    fn finds_nearest_neighbor_returns_successor() {
        let sequence = vec![10, 20, 50, 60, 70, 75, 100];
        assert_eq!(binary_nearest_neighbor!(sequence, 50).unwrap(), 3);
    }

    #[test]
    fn finds_nearest_neighbor_returns_predecessor() {
        let sequence = vec![10, 20, 50, 60, 70, 75, 100];
        assert_eq!(binary_nearest_neighbor!(sequence, 75).unwrap(), 4);
    }

    #[test]
    fn finds_nearest_neighbor_with_equal_distance() {
        let sequence: Vec<u32> = (0..100).collect();
        assert_eq!(binary_nearest_neighbor!(sequence, 56).unwrap(), 55);
    }

    #[test]
    fn finds_nearest_neighbor_with_non_existent_in_range_item() {
        let sequence = vec![10, 20, 50, 60, 70, 75, 100];
        assert_eq!(binary_nearest_neighbor!(sequence, 76).unwrap(), 5);
    }

    #[test]
    fn finds_nearest_neighbor_with_non_existent_out_of_range_smaller_item() {
        let sequence = vec![10, 20, 50, 60, 70, 75, 100];
        assert_eq!(binary_nearest_neighbor!(sequence, 5).unwrap(), 0);
    }

    #[test]
    fn finds_nearest_neighbor_with_non_existent_out_of_range_larger_item() {
        let sequence = vec![10, 20, 50, 60, 70, 75, 100];
        assert_eq!(binary_nearest_neighbor!(sequence, 106).unwrap(), 6);
    }
}
