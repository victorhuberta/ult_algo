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
///     [1, 4, 2, 3], [4, 1, 2, 3]
/// ];
///
/// let sequence = vec![1, 2, 3, 4];
/// let gen = permutation::HeapGen::new(sequence);
/// for (i, permutation) in gen.take(10).enumerate() {
///     assert_eq!(permutation, ten_permutations[i]);
/// }
/// ```
///
/// # Gotchas
///
/// * Order of generated permutations is not preserved
/// * It consumes the vector; we can optionally clone the vector first
/// * It clones its internal representation for every iteration
#[derive(Debug)]
pub struct HeapGen<T: Clone> {
    /// Last generated permutation
    last_permutation: Vec<T>,
    /// Storage for swap indexes
    swaps: Vec<usize>,
    /// Last position of a permutation (to be swapped with elements indexed by self.swaps)
    n: usize,
    /// Count the number of iterations
    count: usize
}

impl<T: Clone> HeapGen<T> {
    pub fn new(sequence: Vec<T>) -> HeapGen<T> {
        let len = sequence.len();
        HeapGen {
            last_permutation: sequence,
            swaps: vec![0; len],
            n: 0,
            count: 0
        }
    }
}

impl<T: Clone> Iterator for HeapGen<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        // Return without swapping only on the first call.
        if self.count == 1 {
            return Some(self.last_permutation.to_vec());
        }

        if self.n < self.last_permutation.len() {
            let counter = self.swaps[self.n];
            if counter < self.n {
                // Swap two elements based on n.
                if self.n % 2 == 0 {
                    self.last_permutation.swap(0, self.n);
                } else {
                    self.last_permutation.swap(counter, self.n);
                };

                // Prepare for the next permutation.
                self.swaps[self.n] += 1;
                self.n = 0;
                Some(self.last_permutation.to_vec())
            } else {
                // We are not done. Let's call next() again.
                self.swaps[self.n] = 0;
                self.n += 1;
                self.next()
            }
        } else {
            // Reset state so the iteration may continue.
            for counter in self.swaps.iter_mut() {
                *counter = 0;
            }
            self.n = 0;
            self.count = 0;
            None
        }
    }
}

#[cfg(test)]
mod heap_tests {
    use super::HeapGen;

    #[test]
    fn generate_correct_number_of_permutations() {
        let sequence = vec![1, 2, 3, 4];
        assert_eq!(HeapGen::new(sequence).count(), 24);
    }

    #[test]
    fn generate_the_first_ten_permutations() {
        let ten_permutations = [
            [1, 2, 3, 4], [2, 1, 3, 4],
            [3, 1, 2, 4], [1, 3, 2, 4],
            [2, 3, 1, 4], [3, 2, 1, 4],
            [4, 2, 1, 3], [2, 4, 1, 3],
            [1, 4, 2, 3], [4, 1, 2, 3]
        ];

        let sequence = vec![1, 2, 3, 4];
        for (i, permutation) in HeapGen::new(sequence).take(10).enumerate() {
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
            [3, 2, 4, 1], [2, 3, 4, 1]
        ];

        let sequence = vec![1, 2, 3, 4];
        for (i, permutation) in HeapGen::new(sequence).skip(14).enumerate() {
            assert_eq!(permutation, ten_permutations[i]);
        }
    }

    #[test]
    fn generate_unique_permutations() {
        let sequence = vec![1, 2, 3, 4];
        let mut permutations: Vec<Vec<usize>> = HeapGen::new(sequence).collect();
        permutations.sort_unstable();
        permutations.dedup();
        assert_eq!(permutations.len(), 24);
    }

    #[test]
    fn regenerate_permutations() {
        let sequence = vec![1, 2, 3, 4];
        let mut gen = HeapGen::new(sequence).skip(24);
        assert_eq!(gen.next(), None);

        let mut permutations: Vec<Vec<usize>> = gen.collect();
        permutations.sort_unstable();
        permutations.dedup();
        assert_eq!(permutations.len(), 24);
    }
}
