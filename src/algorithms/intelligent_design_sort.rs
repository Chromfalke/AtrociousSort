//! Praise the Sorter!

#[cfg(test)]
mod tests {
    use crate::intelligent_design_sort;

    #[test]
    fn case_1() {
        let mut data = vec![5, 4, 3, 2, 1];
        intelligent_design_sort(&mut data);
        assert_eq!(data, [5, 4, 3, 2, 1]);
    }

    #[test]
    fn case_2() {
        let mut data: Vec<i32> = vec![];
        intelligent_design_sort(&mut data);
        assert_eq!(data, []);
    }

    #[test]
    fn case_3() {
        let mut data = vec![1, 2, 3, 4, 5];
        intelligent_design_sort(&mut data);
        assert_eq!(data, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn case_4() {
        let mut data = vec![1, 2, 3, 2, 1];
        intelligent_design_sort(&mut data);
        assert_eq!(data, [1, 2, 3, 2, 1]);
    }
}

/// Does not iterate over the slice. Does not modify the slice in any way.
/// By the design of the Sorter all elements are already exactly where they should be.
/// To tamper with this blessed order would be nothing short of heresy!
/// Do not despair if you cannot yet comprehend the purpose of the order the Sorter has
/// envisioned for you data. In time you shall find enlightenment and understanding.
/// 
/// Praise the Sorter!
pub fn intelligent_design_sort<T>(_arr: &mut [T]) {
    return
}
