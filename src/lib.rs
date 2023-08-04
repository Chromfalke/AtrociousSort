//! # Atrocious sort
//! 
//! Atrocious sort is a collection of some of the most useless sorting algorithms.
//! All algorithms can sort all types that implement the
//! [`Ord`](https://doc.rust-lang.org/stable/std/cmp/trait.Ord.html) trait.
//! 
//! Implemented algorithms can only sort in ascending order.
//! 
//! ## Algorithms
//! 
//! Corrently implemented algorithms
//! - Stalinsort
//! 
//! ## License
//! 
//! See the license file.
//! 
//! ## Examples
//! 
//! ```rust
//! use atrosious_sort::stalinsort;
//! 
//! fn main() {
//!     let mut data = vec![1, 2, 3, 2, 1];
//!     stalinsort(&mut data);
//!     assert_eq!(data, [1, 2, 3]);
//! }
//! ```

mod algorithms;

pub use algorithms::stalinsort;
