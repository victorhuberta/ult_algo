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
/// * Order of generated permutations is not preserved across regenerations
/// * It consumes the vector; we can optionally clone the vector first
/// * It clones its internal representation for every iteration
pub struct HeapGen<T: Clone> {
    /// Last generated permutation
    last_permutation: Vec<T>,
    /// Storage for swap indexes
    swaps: Vec<usize>,
    /// Last position of a permutation (to be swapped with elements indexed by self.swaps)
    n: usize,
    /// Number of iterations
    count: usize,
}

impl<T: Clone> HeapGen<T> {
    pub fn new(sequence: Vec<T>) -> HeapGen<T> {
        HeapGen {
            swaps: vec![0; sequence.len()],
            last_permutation: sequence,
            n: 0,
            count: 0,
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
            [1, 2, 3, 4],
            [2, 1, 3, 4],
            [3, 1, 2, 4],
            [1, 3, 2, 4],
            [2, 3, 1, 4],
            [3, 2, 1, 4],
            [4, 2, 1, 3],
            [2, 4, 1, 3],
            [1, 4, 2, 3],
            [4, 1, 2, 3],
        ];

        let sequence = vec![1, 2, 3, 4];
        for (i, permutation) in HeapGen::new(sequence).take(10).enumerate() {
            assert_eq!(permutation, ten_permutations[i]);
        }
    }

    #[test]
    fn generate_the_last_ten_permutations() {
        let ten_permutations = [
            [4, 1, 3, 2],
            [1, 4, 3, 2],
            [3, 4, 1, 2],
            [4, 3, 1, 2],
            [4, 3, 2, 1],
            [3, 4, 2, 1],
            [2, 4, 3, 1],
            [4, 2, 3, 1],
            [3, 2, 4, 1],
            [2, 3, 4, 1],
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

/// # [Steinhaus–Johnson–Trotter Algorithm](https://en.wikipedia.org/wiki/Steinhaus%E2%80%93Johnson%E2%80%93Trotter_algorithm)
///
/// Generates all the permutations of *n* elements.
/// Each permutation differs from the previous permutation by swapping two adjacent elements.
/// This algoritm is named after Hugo Steinhaus, Selmer M. Johnson, and Hale F. Trotter.
/// Shimon Even provided an improvement to its running time, which is implemented here.
///
/// # Examples
///
/// ```
/// use ult_algo::sequence::permutation;
///
/// let ten_permutations = [
///     [1, 2, 3, 4], [1, 2, 4, 3],
///     [1, 4, 2, 3], [4, 1, 2, 3],
///     [4, 1, 3, 2], [1, 4, 3, 2],
///     [1, 3, 4, 2], [1, 3, 2, 4],
///     [3, 1, 2, 4], [3, 1, 4, 2]
/// ];
///
/// let sequence = vec![1, 2, 3, 4];
/// let gen = permutation::SJTEven::new(sequence);
/// for (i, permutation) in gen.take(10).enumerate() {
///     assert_eq!(permutation, ten_permutations[i]);
/// }
/// ```
///
/// # Gotchas
///
/// * Order of generated permutations is not preserved across regenerations
/// * It consumes the vector; we can optionally clone the vector first
/// * It clones its internal representation for every iteration
pub struct SJTEven<T: Clone + PartialOrd> {
    /// Last generated permutation
    last_permutation: Vec<T>,
    /// Direction of every element (0 = stay, +1 = move right, -1 = move left)
    directions: Vec<i8>,
    /// Number of iterations
    count: usize,
}

impl<T: Clone + PartialOrd> SJTEven<T> {
    pub fn new(sequence: Vec<T>) -> SJTEven<T> {
        SJTEven {
            // [0, -1, -1, -1, ...]
            directions: sequence
                .iter()
                .enumerate()
                .map(|(i, _)| if i == 0 { 0 } else { -1 })
                .collect(),
            last_permutation: sequence,
            count: 0,
        }
    }
}

impl<T: Clone + PartialOrd> Iterator for SJTEven<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count == 1 {
            // Return the sequence itself as the first permutation.
            return Some(self.last_permutation.to_vec());
        }

        // Find the largest/max element which has nonzero direction.
        let mut max_i = 0;
        let mut is_marked = false;
        for (i, x) in self.last_permutation.iter().enumerate() {
            if self.directions[i] != 0 {
                is_marked = true;
                if self.directions[max_i] == 0 || *x > self.last_permutation[max_i] {
                    max_i = i;
                }
            }
        }

        // If none of the elements is marked with a direction, all permutations have been generated.
        if !is_marked {
            // Reset state so it may regenerate all permutations.
            self.directions[0] = 0;
            for direction in self.directions.iter_mut().skip(1) {
                *direction = -1;
            }
            self.count = 0;
            return None;
        }

        // Swap the chosen element with the next element in its direction.
        let old_max_i = max_i;
        max_i = (max_i as isize + self.directions[max_i] as isize) as usize;
        self.last_permutation.swap(max_i, old_max_i);
        self.directions.swap(max_i, old_max_i);

        // If the chosen element is at the first or last position,
        // or the next element in its direction is larger than itself,
        // set its direction to zero (stop moving it).
        let last_i = self.last_permutation.len() - 1;
        let next_i = (max_i as isize + self.directions[max_i] as isize) as usize;
        if max_i == 0
            || max_i == last_i
            || self.last_permutation[next_i] > self.last_permutation[max_i]
        {
            self.directions[max_i] = 0;
        }

        // Find elements greater than the chosen element.
        // Each element's direction is marked based on its position
        // in relation to the chosen element.
        for (i, x) in self.last_permutation.iter().enumerate() {
            if *x > self.last_permutation[max_i] {
                self.directions[i] = if i < max_i { 1 } else { -1 };
            }
        }
        Some(self.last_permutation.to_vec())
    }
}

#[cfg(test)]
mod sjt_tests {
    use super::SJTEven;

    #[test]
    fn generate_correct_number_of_permutations() {
        let sequence = vec![1, 2, 3, 4];
        assert_eq!(SJTEven::new(sequence).count(), 24);
    }

    #[test]
    fn generate_the_first_ten_permutations() {
        let ten_permutations = [
            [1, 2, 3, 4],
            [1, 2, 4, 3],
            [1, 4, 2, 3],
            [4, 1, 2, 3],
            [4, 1, 3, 2],
            [1, 4, 3, 2],
            [1, 3, 4, 2],
            [1, 3, 2, 4],
            [3, 1, 2, 4],
            [3, 1, 4, 2],
        ];

        let sequence = vec![1, 2, 3, 4];
        for (i, permutation) in SJTEven::new(sequence).take(10).enumerate() {
            assert_eq!(permutation, ten_permutations[i]);
        }
    }

    #[test]
    fn generate_the_last_ten_permutations() {
        let ten_permutations = [
            [3, 2, 4, 1],
            [3, 2, 1, 4],
            [2, 3, 1, 4],
            [2, 3, 4, 1],
            [2, 4, 3, 1],
            [4, 2, 3, 1],
            [4, 2, 1, 3],
            [2, 4, 1, 3],
            [2, 1, 4, 3],
            [2, 1, 3, 4],
        ];

        let sequence = vec![1, 2, 3, 4];
        for (i, permutation) in SJTEven::new(sequence).skip(14).enumerate() {
            assert_eq!(permutation, ten_permutations[i]);
        }
    }

    #[test]
    fn generate_unique_permutations() {
        let sequence = vec![1, 2, 3, 4];
        let mut permutations: Vec<Vec<usize>> = SJTEven::new(sequence).collect();
        permutations.sort_unstable();
        permutations.dedup();
        assert_eq!(permutations.len(), 24);
    }

    #[test]
    fn regenerate_permutations() {
        let sequence = vec![1, 2, 3, 4];
        let mut gen = SJTEven::new(sequence).skip(24);
        assert_eq!(gen.next(), None);

        let mut permutations: Vec<Vec<usize>> = gen.collect();
        permutations.sort_unstable();
        permutations.dedup();
        assert_eq!(permutations.len(), 24);
    }
}
