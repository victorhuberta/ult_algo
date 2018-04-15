//! # Match
//!
//! A collection of functions to approximately or exactly match a pattern over a sequence.

use bit_vec::BitVec;

/// Exactly matches a pattern over the given sequence with the bitap algorithm.
///
/// # Examples
///
/// It returns the index of the matching pattern in the sequence.
///
/// ```
/// use ult_algo::sequence::match_;
///
/// let sequence: Vec<char> = "hello, world".chars().collect();
/// let pattern: Vec<char> = "wor".chars().collect();
/// assert_eq!(match_::bitap(&sequence, &pattern), 7);
/// ```
///
/// If the pattern does not match, it returns -1.
pub fn bitap<T: PartialEq>(sequence: &[T], pattern: &[T]) -> i32 {
    let (m, n) = (pattern.len(), sequence.len());
    if m == 0 {
        return 0; // empty pattern matches everything
    } else if m > n {
        return -1; // longer pattern matches nothing
    }

    let mut r = BitVec::from_elem(m+1, false); // init bit array
    r.set(0, true);
    for i in 0..n {
        // Update the bit array.
        for k in (1..m+1).rev() {
            let prev = r[k-1];
            r.set(k, prev & (sequence[i] == pattern[k-1]));
        }

        if r[m] {
            return (i-m+1) as i32; // found a match
        }
    }
    return -1; // no match found
}

#[cfg(test)]
mod bitap_tests {
    use super::bitap;

    #[test]
    fn receives_char_vectors() {
        let sequence: Vec<char> = "hello, world".chars().collect();
        let pattern: Vec<char> = "wor".chars().collect();
        assert_eq!(bitap(&sequence, &pattern), 7);
    }

    #[test]
    fn receives_integer_vectors() {
        let sequence = vec![3, 4, 5, 7, 3, 2, 1];
        let pattern = vec![4, 5, 7, 3];
        assert_eq!(bitap(&sequence, &pattern), 1);
    }

    #[test]
    fn receives_different_types_of_sequences() {
        let sequence: Vec<char> = "hello, world".chars().collect();
        let pattern = ['w', 'o', 'r'];
        assert_eq!(bitap(&sequence, &pattern), 7);
    }

    #[test]
    fn receives_empty_pattern_returns_index_0() {
        let sequence: Vec<char> = "hello, world".chars().collect();
        let pattern = vec![];
        assert_eq!(bitap(&sequence, &pattern), 0);
    }

    #[test]
    fn receives_longer_pattern_returns_invalid_index() {
        let sequence: Vec<char> = "hello, world".chars().collect();
        let pattern: Vec<char> = "hello, world! Here I am looking at nothing".chars().collect();
        assert_eq!(bitap(&sequence, &pattern), -1);
    }

    #[test]
    fn receives_non_matching_pattern_returns_invalid_index() {
        let sequence = vec![3, 4, 5, 7, 3, 2, 1];
        let pattern = vec![4, 5, 7, 5];
        assert_eq!(bitap(&sequence, &pattern), -1);
    }
}
