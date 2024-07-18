//! Are you feeling lucky today?
use rand::Rng;

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

pub fn bogo_sort<T: Ord + Copy>(arr: &mut [T]) {
    let mut rng = rand::thread_rng();
    while !is_sorted(arr) {
        for i in 0..arr.len() {
            let j = rng.gen_range(0..arr.len());
            arr.swap(i, j);
        }
    }
}

fn is_sorted<T: Ord>(arr: &[T]) -> bool {
    if arr.len() <= 1 {
        return true;
    }
    for i in 0..arr.len() - 1 {
        if arr[i] > arr[i + 1] {
            return false;
        }
    }
    return true;
}