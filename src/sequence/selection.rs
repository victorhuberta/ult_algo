use rand;
use rand::Rng;

pub fn quick_min<T: PartialOrd>(list: &mut [T], k: usize) -> &T {
    if k >= list.len() {
        panic!("k={} should be smaller than list's length", k);
    }

    loop {
        if list.len() == 1 {
            return &list[0];
        }

        let mut rng = rand::thread_rng();
        let pivot_idx = rng.gen_range(0, list.len());
        let pivot_idx = partition(list, pivot_idx);

        if k == pivot_idx {
            return &list[k];
        } else if k < pivot_idx {
            return quick_min(&mut list[..pivot_idx], k);
        } else {
            return quick_min(&mut list[pivot_idx..], k-pivot_idx);
        }
    }
}

#[cfg(test)]
mod quick_tests {
    use super::*;

    #[test]
    fn receives_integer_vector_slice() {
        let mut list = vec![10, -30, -2, 5, 7, 0];
        assert_eq!(*quick_min(&mut list[..], 3), 5);
    }

    #[test]
    fn receives_char_vector_slice() {
        let mut list = vec!['z', 'b', 'e', 'y', 'm', 'k'];
        assert_eq!(*quick_min(&mut list[..], 1), 'e');
    }

    #[test]
    fn receives_partial_slice() {
        let mut list = vec![10, -30, 5, -2, 7, 0];
        assert_eq!(*quick_min(&mut list[1..5], 2), 5);
    }

    #[test]
    #[should_panic(expected = "k=6 should be smaller than list's length")]
    fn receives_invalid_k() {
        let mut list = vec![10, -30, -2, 5, 7, 0];
        quick_min(&mut list[..], 6);
    }
}

fn partition<T: PartialOrd>(list: &mut [T], pivot_idx: usize) -> usize {
    let last_idx = list.len()-1;
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
