//! Because bogo sort is to efficient, right?
use rand::Rng;

use crate::bogo_sort::{bogo_sort, is_sorted};

#[cfg(test)]
mod tests {
    use crate::bogobogo_sort;

    #[test]
    fn case_1() {
        let mut data = vec![5, 4, 3, 2, 1];
        bogobogo_sort(&mut data);
        assert_eq!(data, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn case_2() {
        let mut data: Vec<i32> = vec![];
        bogobogo_sort(&mut data);
        assert_eq!(data, []);
    }

    #[test]
    fn case_3() {
        let mut data = vec![1, 2, 3, 4, 5];
        bogobogo_sort(&mut data);
        assert_eq!(data, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn case_4() {
        let mut data = vec![1, 2, 3, 2, 1];
        bogobogo_sort(&mut data);
        assert_eq!(data, [1, 1, 2, 2, 3]);
    }
}

/// Sorts subarrays of increasing length 2, 3, 4, etc. using bogo sort.
/// Should at any point any one of these subarrays not be sorted
/// on the first try, the entire process will be restarted.
pub fn bogobogo_sort<T: Ord + Copy>(arr: &mut [T]) {
    let length = arr.len();
    if length <= 1 {
        return;
    }

    let mut index = 2;
    let mut rng = rand::thread_rng();
    while index < length {
        bogo_sort(&mut arr[0..index]);
        index += 1;
        if !is_sorted(&arr[0..index]) {
            for i in 0..arr.len() {
                let j = rng.gen_range(0..arr.len());
                arr.swap(i, j);
            }
            index = 2;
        }
    }
}