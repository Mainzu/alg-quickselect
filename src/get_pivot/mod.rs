//! TODO: brief
//!
//! # How to choose the pivot
//!
//! The pivot is used to divide the input slice into two smaller parts, with elements
//! less than the pivot on one side and elements greater than the pivot on the other.
//!
//! ## Guideline
//!
//! The ideal pivot selection, in any case, is one that always correctly chooses the k-th
//! smallest element itself on the first try every time. If this strategy exists,
//! then there is no need to use Quickselect. In practice, however, this is often not the case
//! and only a guess can be made. In this case, Quickselect can then utilize these gueses
//! to find the k-th smallest element.
//!
//! 1. If possible, the strategy should choose a pivot closest to the *value*
//! of the k-th smallest element to eliminate the largest amount of potential candidates.
//! 2. Avoid doing too much work. Compute spent selecting the pivot is compute not spent
//! doing more iterations.
//! 3. If the first goal is not possible, instead aim to select a pivot that
//! splits the remaining values evenly into two halves.
//!
//! ## Invariants
//!
//! This crate already ensures the following preconditions:
//! - The slice received by `get_pivot` is never empty (as per the signature).
//! - For each invocation of `get_pivot` since this function was called,
//! the given slice will always be shorter by at least 1 element.
//!
//! The user must ensure the following postcondition:
//! - The output index is valid for the given slice.
//!
//! ## Examples
//!
//! ```rust, no_run
//! # use not_empty::NonEmptySlice;
//! fn middle_index<T>(s: &mut NonEmptySlice<T>) -> usize {
//!     s.len().get() / 2
//! }
//! fn first_index<T>(_: &mut NonEmptySlice<T>) -> usize {
//!     0
//! }
//! fn last_index<T>(s: &mut NonEmptySlice<T>) -> usize {
//!     s.len().get() - 1
//! }
//! ```
//!
//! Another common approach is to use random index. Though this approach is theoretically
//! as fast or slower than the simple approach. The random strategy is **not implemented**
//! in this crate due to the extra dependency required.
//!
//! ```rust, ignore
//! # use not_empty::NonEmptySlice;
//! use rand::{Rng, thread_rng};
//!
//! let mut rng = thread_rng();
//! let random = |s: &mut NonEmptySlice<_>| rng.gen_range(0..s.len().get());
//!
//! // OR
//!
//! fn random<T>(s: &mut NonEmptySlice<T>) -> usize {
//!     thread_rng().gen_range(0..s.len().get())
//! }
//! ```
//!
// TODO: Add more examples

use not_empty::NonEmptySlice;

/// ```rust, ignore
/// s.len().get() / 2
/// ```
#[inline]
pub fn middle_index<T>(s: &mut NonEmptySlice<T>) -> usize {
    s.len().get() / 2
}
/// ```rust, ignore
/// 0
/// ```
#[inline]
pub fn first_index<T>(_: &mut NonEmptySlice<T>) -> usize {
    0
}
/// ```rust, ignore
/// s.len().get() - 1
/// ```
#[inline]
pub fn last_index<T>(s: &mut NonEmptySlice<T>) -> usize {
    s.len().get() - 1
}
