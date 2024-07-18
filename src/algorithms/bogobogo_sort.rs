//! Because bogo sort is to efficient, right?
use rand::Rng;

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

/// Sorts all except the last element of an array recursively.
/// If the last element is not the largest the array will be randomized
/// and the process will be repeated.
pub fn bogobogo_sort<T: Ord + Copy>(arr: &mut [T]) {
    let length = arr.len();
    if length <= 1 {
        return;
    }

    let mut rng = rand::thread_rng();
    loop {
        bogobogo_sort(&mut arr[0..length - 1]);
        if arr[length - 2] <= arr[length - 1] {
            return;
        } else {
            for i in 0..arr.len() {
                let j = rng.gen_range(0..arr.len());
                arr.swap(i, j);
            }
        }
    }
}