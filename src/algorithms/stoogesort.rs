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
pub fn stoogesort<T: PartialOrd>(arr: &mut [T]) {
    // this is correct as a slices maximum length is isize::MAX
    // and same as slow sort removes one additional check to make things slightly faster
    _stoogesort(arr, 0, arr.len() as isize -1);
}


#[inline]
fn _stoogesort<T: PartialOrd>(arr: &mut [T], start: isize, end: isize) {
    if start >= end { 
        return
    }
    
    let (first, last) = (start as usize, end as usize);
    if arr[first] > arr[last] {
        arr.swap(first, last);
    }
    
    let third = (end - start + 1) / 3;
    
    // if there are less than 2 elements in this range return
    if third <= 0 { 
        return
    }
    
    _stoogesort(arr, start, end - third);
    _stoogesort(arr, start + third, end);
    _stoogesort(arr, start, end - third);
}