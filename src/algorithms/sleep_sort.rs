//! What is an eternity anyway?
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

#[cfg(test)]
mod tests {
    use crate::sleep_sort;

    #[test]
    fn case_1() {
        let mut data: Vec<u32> = vec![5, 4, 3, 2, 1];
        sleep_sort(&mut data);
        assert_eq!(data, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn case_2() {
        let mut data: Vec<u32> = vec![];
        sleep_sort(&mut data);
        assert_eq!(data, []);
    }

    #[test]
    fn case_3() {
        let mut data: Vec<u32> = vec![1, 2, 3, 4, 5];
        sleep_sort(&mut data);
        assert_eq!(data, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn case_4() {
        let mut data: Vec<u32> = vec![1, 2, 3, 2, 1];
        sleep_sort(&mut data);
        assert_eq!(data, [1, 1, 2, 2, 3]);
    }
}

/// Each item in the vector is assigned to a new thread. Each thread will sleep for
/// a number of seconds determined by the value of the item. Once a thread wakes
/// up it will push the assigned item to a new vector which will be returned once all
/// threads are done. Whenever that will be.
/// Be aware that due to quirks with sleeping behavior and short delays when spawning the
/// individual threads, the result is unreliable. In case of doubt just run it again.
///
/// Fun Fact: The maxumim value for a u64 in rust is 18 446 744 073 709 551 615. So a thread could
/// potentially sleep for 18 446 744 073 709 551 615 seconds or roughly about 584 942 417 355 years.

pub trait SleepAmount {
    fn sleep_amount(&self) -> u64;
}


impl<T: Into<u64> + Copy> SleepAmount for T {
    fn sleep_amount(&self) -> u64 {
        (*self).into()
    }
}

pub fn sleep_sort<T: SleepAmount + Send + Sync>(arr: &mut [T]) {
    let sorted_vec_arc = Mutex::new(Vec::with_capacity(arr.len()));
    
    thread::scope(|s| {
        for (i, item) in arr.iter().enumerate() {
            let handle = &sorted_vec_arc;
            s.spawn(move || {
                thread::sleep(Duration::from_secs(item.sleep_amount()));
                
                handle
                    .lock()
                    .expect("sleep panicked")
                    .push(i)
            });
        }
    });

    let mut indices = sorted_vec_arc.into_inner().expect("sorting panicked");
    
    assert_eq!(indices.len(), arr.len());
    
    for i in 0..arr.len() {
        let mut index = indices[i];
        while index < i {
            index = indices[index];
        }
        indices[i] = index;
        arr.swap(i, index);
    }
}
