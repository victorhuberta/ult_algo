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
/// let mut sequence = vec![1, 2, 3, 4];
/// let mut i = 0;
/// for permutation in HeapGen::new(&mut sequence) {
///     assert_eq!(permutation, ten_permutations[i]);
/// }
/// ```
#[derive(Debug)]
pub struct HeapGen<'a, T: 'a> {
    last_permutation: &'a mut Vec<T>,
    interchanges: Vec<usize>,
    i: usize
}

impl<'a, T> HeapGen<'a, T> {
    pub fn new(sequence: &'a mut Vec<T>) -> HeapGen<'a, T> {
        HeapGen {
            last_permutation: sequence,
            interchanges: vec![0; sequence.len()],
            i: 0
        }
    }
}

impl<'a, T> Iterator for HeapGen<'a, T> {
    type Item = &'a mut Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i < self.last_permutation.len() {
            let counter = self.interchanges[self.i];
            if counter < self.i {
                // Figure out which element to swap into the last position.
                let item_idx = if self.i % 2 == 0 {
                    0
                } else {
                    counter
                };

                // Swap two elements.
                let tmp = self.last_permutation[item_idx];
                self.last_permutation[item_idx] = self.last_permutation[self.i];
                self.last_permutation[self.i] = tmp;

                // Prepare for the next permutation.
                self.interchanges[self.i] += 1;
                self.i = 0;
                Some(self.last_permutation)
            } else {
                // We are not done. Let's call next() again.
                self.interchanges[self.i] = 0;
                self.i += 1;
                self.next()
            }
        } else {
            None
        }
    }
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

        let mut sequence = vec![1, 2, 3, 4];
        let mut i = 0;
        for permutation in HeapGen::new(&mut sequence) {
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

        let mut sequence = vec![1, 2, 3, 4];
        let mut i = 0;
        for permutation in HeapGen::new(&mut sequence) {
            assert_eq!(permutation, ten_permutations[i]);
        }
    }
}
