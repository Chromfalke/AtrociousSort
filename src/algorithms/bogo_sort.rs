//! Are you feeling lucky today?
use rand::seq::SliceRandom;

#[cfg(test)]
mod tests {
    use crate::bogo_sort;

    #[test]
    fn case_1() {
        let mut data = vec![5, 4, 3, 2, 1];
        bogo_sort(&mut data);
        assert_eq!(data, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn case_2() {
        let mut data: Vec<i32> = vec![];
        bogo_sort(&mut data);
        assert_eq!(data, []);
    }

    #[test]
    fn case_3() {
        let mut data = vec![1, 2, 3, 4, 5];
        bogo_sort(&mut data);
        assert_eq!(data, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn case_4() {
        let mut data = vec![1, 2, 3, 2, 1];
        bogo_sort(&mut data);
        assert_eq!(data, [1, 1, 2, 2, 3]);
    }
}

/// Creates a random permutation of the array until one happens to be sorted.
/// The longer the array the longer of a brake you can take while you wait for
/// it to be sorted.
pub fn bogo_sort<T: PartialOrd>(arr: &mut [T]) {
    let mut rng = rand::rng();
    while !arr.is_sorted() {
        arr.shuffle(&mut rng)
    }
}