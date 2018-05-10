//! # Permutation
//!
//! A collection of functions to generate permutations of a sequence.

/// # [Heap's Algorithm](https://en.wikipedia.org/wiki/Heap%27s_algorithm)
///
/// Generates all possible permutations of *n* objects. First proposed by B. R. Heap in 1963.
///
/// # Examples
///
/// ```
/// use ult_algo::sequence::permutation;
///
/// let ten_permutations = [
///     [1, 2, 3, 4], [2, 1, 3, 4],
///     [3, 1, 2, 4], [1, 3, 2, 4],
///     [2, 3, 1, 4], [3, 2, 1, 4],
///     [4, 2, 1, 3], [2, 4, 1, 3],
///     [1, 4, 2, 3], [4, 1, 2, 3],
/// ];
///
/// let mut i = 0;
/// for permutation in heap(&[1, 2, 3, 4]) {
///     assert_eq!(permutation, ten_permutations[i]);
/// }
/// ```
pub fn heap<T>(sequence: &[T]) -> [T] {
    []
}

#[cfg(test)]
mod heap_tests {
    use super::heap;

    #[test]
    fn generate_the_first_ten_permutations() {
        let ten_permutations = [
            [1, 2, 3, 4], [2, 1, 3, 4],
            [3, 1, 2, 4], [1, 3, 2, 4],
            [2, 3, 1, 4], [3, 2, 1, 4],
            [4, 2, 1, 3], [2, 4, 1, 3],
            [1, 4, 2, 3], [4, 1, 2, 3],
        ];

        let mut i = 0;
        for permutation in heap(&[1, 2, 3, 4]) {
            assert_eq!(permutation, ten_permutations[i]);
        }
    }

    #[test]
    fn generate_the_last_ten_permutations() {
        let ten_permutations = [
            [4, 1, 3, 2], [1, 4, 3, 2],
            [3, 4, 1, 2], [4, 3, 1, 2],
            [4, 3, 2, 1], [3, 4, 2, 1],
            [2, 4, 3, 1], [4, 2, 3, 1],
            [3, 2, 4, 1], [2, 3, 4, 1],
        ];

        let mut i = 0;
        for permutation in heap(&[1, 2, 3, 4]) {
            assert_eq!(permutation, ten_permutations[i]);
        }
    }
}
