//! Is a capitalist spy hiding in your data?

#[cfg(test)]
mod tests {
    use crate::stalinsort;

    #[test]
    fn case_1() {
        let mut data = vec![5, 4, 3, 2, 1];
        stalinsort(&mut data);
        assert_eq!(data, [5]);
    }

    #[test]
    fn case_2() {
        let mut data: Vec<i32> = vec![];
        stalinsort(&mut data);
        assert_eq!(data, []);
    }

    #[test]
    fn case_3() {
        let mut data = vec![1, 2, 3, 4, 5];
        stalinsort(&mut data);
        assert_eq!(data, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn case_4() {
        let mut data = vec![1, 2, 3, 2, 1];
        stalinsort(&mut data);
        assert_eq!(data, [1, 2, 3]);
    }
}

/// Iterates over the vector and removes all elements that are out of order.
/// If an element in the vector is out of order it is most likely a filthy capitalist
/// saboteur keeping your honest and hard working data from being in order.
/// The given vector will be modified in place.
pub fn stalinsort<T: Ord>(arr: &mut Vec<T>) {
    let old_len = arr.len();
    let mut new_len = old_len;
    let mut index = 1;
    
    while index < new_len {
        arr.swap(index, (old_len - new_len) + index);
        
        match arr[index] < arr[index - 1] { 
            false => index += 1,
            true => new_len -= 1,
        }
    }
    
    arr.truncate(new_len)
}
