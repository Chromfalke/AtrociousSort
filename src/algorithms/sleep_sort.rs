//! What is an eternity anyway?

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[cfg(test)]
mod tests {
    use crate::sleep_sort;

    #[test]
    fn case_1() {
        let mut data: Vec<u32> = vec![5, 4, 3, 2, 1];
        let sorted = sleep_sort(&mut data);
        assert_eq!(sorted, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn case_2() {
        let mut data: Vec<u32> = vec![];
        let sorted = sleep_sort(&mut data);
        assert_eq!(sorted, []);
    }

    #[test]
    fn case_3() {
        let mut data: Vec<u32> = vec![1, 2, 3, 4, 5];
        let sorted = sleep_sort(&mut data);
        assert_eq!(sorted, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn case_4() {
        let mut data: Vec<u32> = vec![1, 2, 3, 2, 1];
        let sorted = sleep_sort(&mut data);
        assert_eq!(sorted, [1, 1, 2, 2, 3]);
    }
}

/// Each item in the vector is assigned to a new thread. Each thread will sleep for
/// a number of milliseconds determined by the value of the item. Once a thread wakes
/// up it will push the assigned item to a new vector which will be returned once all
/// threads are done. Whenever that will be.
/// 
/// Fun Fact: The maxumim value for a u64 in rust is 18 446 744 073 709 551 615. So a thread could
/// potentially sleep for 18 446 744 073 709 551 615 milliseconds or about 584 554 531 years.
pub fn sleep_sort<T: Into<u64> + Send + Sync + Copy>(arr: &[T]) -> Vec<T> {
    let sorted_vec_arc = Arc::new(Mutex::new(Vec::<T>::new()));
    thread::scope(|s| {
        for item in arr.iter() {
            let target = sorted_vec_arc.clone();
            s.spawn(move || {
                thread::sleep(Duration::from_millis(Into::<u64>::into(*item)));
                let mut binding_target = target.lock();
                let reference = binding_target.as_deref_mut().unwrap();
                reference.push(*item);
            });
        }
    });

    let binding = sorted_vec_arc.lock();
    let sorted_vec = binding.as_deref().unwrap();
    return sorted_vec.to_vec()
}