//! # Atrocious sort
//! 
//! Atrocious sort is a collection of some of the most useless sorting algorithms.
//! The different algorithms depend on different traits to be implemented for the types
//! that are to be sorted. The traits are specified in the documentation of each algorithm.
//! 
//! Implemented algorithms can only sort in ascending order.
//! 
//! ## Algorithms
//! 
//! Currently implemented algorithms
//! - Stalinsort
//! - Intelligent Design Sort
//! - Sleep Sort
//! - Bogo Sort
//! - Bogobogo Sort
//! 
//! ## Examples
//! 
//! ```rust
//! extern crate atrocious_sort;
//! use atrocious_sort::stalinsort;
//! 
//! fn main() {
//!     let mut data = vec![1, 2, 3, 2, 1];
//!     stalinsort(&mut data);
//!     assert_eq!(data, [1, 2, 3]);
//! }
//! ```

mod algorithms;

pub use algorithms::stalinsort;
pub use algorithms::intelligent_design_sort;
pub use algorithms::sleep_sort;
pub use algorithms::slowsort;
pub use algorithms::bogo_sort;
pub use algorithms::bogobogo_sort;