//! 

#[cfg(test)]
mod tests {
    use crate::stoogesort;

    #[test]
    fn case_1() {
        let mut data = vec![5, 4, 3, 2, 1];
        stoogesort(&mut data);
        assert_eq!(data, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn case_2() {
        let mut data: Vec<i32> = vec![];
        stoogesort(&mut data);
        assert_eq!(data, []);
    }

    #[test]
    fn case_3() {
        let mut data = vec![1, 2, 3, 4, 5];
        stoogesort(&mut data);
        assert_eq!(data, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn case_4() {
        let mut data = vec![1, 2, 3, 2, 1];
        stoogesort(&mut data);
        assert_eq!(data, [1, 1, 2, 2, 3]);
    }
}

/// Swaps the first and the last element if they are not in order.
/// It will then sort the first two thirds of the array, then the
/// second to thirds and then the first two thirds again.
pub fn stoogesort<T: Ord>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }

    _stoogesort(arr, 0, arr.len()-1);
}

fn _stoogesort<T: Ord>(arr: &mut [T], start: usize, end: usize) {
    if arr[start] > arr[end] {
        arr.swap(start, end);
    }

    if start+1 >= end {
        return;
    }

    let third = arr.len() / 3;
    _stoogesort(arr, start, end - third);
    _stoogesort(arr, start + third, end);
    _stoogesort(arr, start, end - third);
}