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
pub fn slowsort<T: PartialOrd>(arr: &mut [T]) {
    // this is correct as a slices maximum length is isize::MAX
    // note this eliminates the top check and makes the code slightly faster
    slowsort_inner(arr, 0, arr.len() as isize - 1);
}

#[inline]
fn slowsort_inner<T: PartialOrd>(arr: &mut [T], start: isize, end: isize) {
    if start >= end {
        return
    }
    
    let middle = (start + end) / 2;

    slowsort_inner(arr, start, middle);
    slowsort_inner(arr, middle + 1, end);

    let (mid, last) = (middle as usize, end as usize);
    if arr[mid] > arr[last] {
        arr.swap(mid, last);
    }

    slowsort_inner(arr, start, end - 1);
}
