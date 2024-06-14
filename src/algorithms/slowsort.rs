//! Take your time. Get some coffee, start a family.

#[cfg(test)]
mod tests {
    use crate::slowsort;

    #[test]
    fn case_1() {
        let mut data = vec![5, 4, 3, 2, 1];
        slowsort(&mut data);
        assert_eq!(data, [1, 2, 3, 4, 5])
    }

    #[test]
    fn case_2() {
        let mut data: Vec<i32> = vec![];
        slowsort(&mut data);
        assert_eq!(data, [])
    }

    #[test]
    fn case_3() {
        let mut data = vec![1, 2, 3, 4, 5];
        slowsort(&mut data);
        assert_eq!(data, [1, 2, 3, 4, 5])
    }

    #[test]
    fn case_4() {
        let mut data = vec![1, 2, 3, 2, 1];
        slowsort(&mut data);
        assert_eq!(data, [1, 1, 2, 2, 3])
    }
}


/// Splits the vector into two parts in the middle. Each half is then sorted
/// recursively. Then the last element from both halfs are compared and the
/// largest will be moved to the end of the vector. The procedure is then
/// repeated for the rest of the vector excluding the last element.
pub fn slowsort<T: Ord + Copy>(arr: &mut Vec<T>) {
    if arr.len() <= 0 {
        return
    }
    _slowsort(arr, 0, arr.len() - 1);
}

fn _slowsort<T: Ord + Copy>(arr: &mut Vec<T>, start: usize, end: usize) {
    if end <= start {
        return
    }

    let middle = (start + end) / 2;

    _slowsort(arr, start, middle);
    _slowsort(arr, middle + 1, end);

    if arr[middle] > arr[end] {
        let temp = arr[middle];
        arr[middle] = arr[end];
        arr[end] = temp;
    }

    _slowsort(arr, start, end - 1);
}
