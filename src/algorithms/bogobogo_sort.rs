//! Because bogo sort is to efficient, right?
use rand::seq::SliceRandom;

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
pub fn bogobogo_sort<T: PartialOrd>(arr: &mut [T]) {
    let mut index = 2;
    let mut rng = rand::rng();
    while index <= arr.len() {
        let slice = &mut arr[..index];
        slice.shuffle(&mut rng);
        if !slice.is_sorted() {
            // with the increment afterwards the index = 2
            index = 1
        }
        
        index += 1
    }
}