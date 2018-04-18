//! # Match
//!
//! A collection of functions to approximately or exactly match a pattern over a sequence.

use bit_vec::BitVec;
use std::cmp;

/// # [Bitap Algorithm](https://en.wikipedia.org/wiki/Bitap_algorithm)
///
/// Exactly matches a pattern over the given sequence using bitwise operations.
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
pub fn bitap<T: PartialEq>(sequence: &[T], pattern: &[T]) -> i64 {
    let (pat_len, seq_len) = (pattern.len(), sequence.len());
    if pat_len == 0 {
        return 0; // empty pattern matches everything
    } else if pat_len > seq_len {
        return -1; // longer pattern matches nothing
    }

    let mut bit_arr = BitVec::from_elem(pat_len+1, false); // init bit array
    bit_arr.set(0, true);
    for i in 0..seq_len {
        // Update the bit array.
        for k in (1..pat_len+1).rev() {
            let prev_bit = bit_arr[k-1];
            bit_arr.set(k, prev_bit & (sequence[i] == pattern[k-1]));
        }

        if bit_arr[pat_len] {
            return (i-pat_len+1) as i64; // found a match
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

/// # [Levenshtein Edit Distance](https://en.wikipedia.org/wiki/Levenshtein_distance)
///
/// Calculates the minimum edit distance between two sequences.
///
/// # Examples
///
/// ```
/// use ult_algo::sequence::match_;
///
/// let source: Vec<char> = "sitting".chars().collect();
/// let target: Vec<char> = "kitten".chars().collect();
/// assert_eq!(match_::levenshtein_distance(&source, &target), 3);
/// ```
pub fn levenshtein_distance<T: PartialEq>(source: &[T], target: &[T]) -> u64 {
    let (m, n) = (source.len()+1, target.len()+1);
    // distances[i][j] holds the edit distance for the first i source chars and j target chars;
    // the distances matrix has the size of m*n.
    let mut distances = vec![vec![0u64; n]; m];

    // Source prefixes can be transformed into empty string by dropping all chars.
    for i in 1..m {
        distances[i][0] = i as u64;
    }
    // Empty string can be transformed into target prefixes by inserting every char.
    for j in 1..n {
        distances[0][j] = j as u64;
    }

    for j in 1..n {
        for i in 1..m {
            // i-2 because m = source.len()+1 (the same explains j-2).
            let substitution_cost = if source[i-1] == target[j-1] { 0 } else { 1 };

            // Find the minimum of 3 different edit operation costs.
            distances[i][j] = cmp::min(
                distances[i-1][j] + 1, // deletion
                cmp::min(
                    distances[i][j-1] + 1, // insertion
                    distances[i-1][j-1] + substitution_cost // substitution
                )
            );
        }
    }
    distances[m-1][n-1] // the last element is the min. edit distance
}

#[cfg(test)]
mod levenshtein_distance_tests {
    use super::levenshtein_distance;

    #[test]
    fn receives_longer_source() {
        let source: Vec<char> = "sitting".chars().collect();
        let target: Vec<char> = "kitten".chars().collect();
        assert_eq!(levenshtein_distance(&source, &target), 3);
    }

    #[test]
    fn receives_longer_target() {
        let source: Vec<char> = "kite".chars().collect();
        let target: Vec<char> = "sitting".chars().collect();
        assert_eq!(levenshtein_distance(&source, &target), 5);
    }

    #[test]
    fn receives_integer_vectors() {
        let source: Vec<u64> = (1..50).collect();
        let target: Vec<u64> = (4..40).collect();
        assert_eq!(levenshtein_distance(&source, &target), 13);
    }

    #[test]
    fn receives_empty_source() {
        let source: Vec<char> = vec![];
        let target: Vec<char> = "sitting".chars().collect();
        assert_eq!(levenshtein_distance(&source, &target), target.len() as u64);
    }

    #[test]
    fn receives_empty_target() {
        let source: Vec<char> = "sitting".chars().collect();
        let target: Vec<char> = vec![];
        assert_eq!(levenshtein_distance(&source, &target), source.len() as u64);
    }
}
