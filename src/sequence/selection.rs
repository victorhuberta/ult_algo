//! # Selection
//!
//! A collection of functions to select an item from a sequence.

use rand;
use rand::Rng;

/// # [Quickselect](https://en.wikipedia.org/wiki/Quickselect)
///
/// Selects the k-th smallest element in an unordered slice, where 0 <= k < list.len().
///
/// # Examples
///
/// In this example, we pass in the slice [-30, 5, -2, 7] and k = 2,
/// which means we want to select the 3rd smallest element in that slice.
///
/// ```
/// use ult_algo::sequence::selection;
///
/// let mut list = vec![10, -30, 5, -2, 7, 0];
/// let third_smallest = selection::quick_smallest(&mut list[1..5], 2);
/// assert_eq!(*third_smallest, 5);
/// ```
///
/// # Panics
///
/// * k is larger than or equal to the list's length
pub fn quick_smallest<T: PartialOrd>(list: &mut [T], k: usize) -> &T {
    if k >= list.len() {
        panic!("k={} should be smaller than list's length", k);
    }

    while list.len() != 1 {
        // Randomly choose a pivot and partitions the list accordingly.
        let mut rng = rand::thread_rng();
        let pivot_idx = rng.gen_range(0, list.len());
        let pivot_idx = partition(list, pivot_idx);

        if k == pivot_idx {
            return &list[k]; // found it!
        } else if k < pivot_idx {
            return quick_smallest(&mut list[..pivot_idx], k); // take the left side
        } else {
            // Take the right side and transform k to fit the new slice.
            return quick_smallest(&mut list[pivot_idx + 1..], k - pivot_idx - 1);
        }
    }
    &list[0] // only one possibility
}

#[cfg(test)]
mod quick_tests {
    use super::*;

    #[test]
    fn receives_integer_vector_slice() {
        let mut list = vec![10, -30, -2, 5, 7, 0];
        assert_eq!(*quick_smallest(&mut list[..], 3), 5);
    }

    #[test]
    fn receives_char_vector_slice() {
        let mut list = vec!['z', 'b', 'e', 'y', 'm', 'k'];
        assert_eq!(*quick_smallest(&mut list[..], 1), 'e');
    }

    #[test]
    fn receives_partial_slice() {
        let mut list = vec![10, -30, 5, -2, 7, 0];
        assert_eq!(*quick_smallest(&mut list[1..5], 2), 5);
    }

    #[test]
    #[should_panic(expected = "k=6 should be smaller than list's length")]
    fn receives_invalid_k() {
        let mut list = vec![10, -30, -2, 5, 7, 0];
        quick_smallest(&mut list[..], 6);
    }
}

/// Partitions a list based on the chosen pivot. Left side items are smaller than the pivot,
/// while right side items are larger than or equal to the pivot.
fn partition<T: PartialOrd>(list: &mut [T], pivot_idx: usize) -> usize {
    let last_idx = list.len() - 1;
    let mut store_idx = 0;

    list.swap(pivot_idx, last_idx); // move pivot to end

    for i in 0..last_idx {
        // Compare with pivot value.
        if list[i] < list[last_idx] {
            list.swap(store_idx, i); // put it into the left side of pivot
            store_idx += 1;
        }
    }
    list.swap(store_idx, last_idx); // move pivot to its final sorted place
    store_idx
}
