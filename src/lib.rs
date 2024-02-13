#![feature(slice_swap_unchecked)]
#![no_std]
// lints
#![deny(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]
#![deny(rustdoc::invalid_rust_codeblocks)]
#![deny(rustdoc::missing_crate_level_docs)]
#![warn(rustdoc::invalid_codeblock_attributes)]
//! TODO: crate-level docs

use core::cmp::Ordering;

use not_empty::NonEmptySlice;

// pub mod variantions;
pub mod get_pivot;

/// Partitions the given mutable slice `s` around a pivot element selected at `pivot_index`.
///
/// The function rearranges the elements of the slice such that all elements less than or equal to
/// the pivot element are placed before it, and all elements greater than it are placed after it.
/// The pivot element is moved to its final sorted position.
///
/// # Safety
///
/// `pivot_index` MUST be less than the length of the slice `s`.
///
/// # Note
///
/// This is a private function, do not expose it to the public API.
unsafe fn partition_unchecked<T: Ord>(s: &mut NonEmptySlice<T>, pivot_index: usize) -> usize {
    debug_assert!(pivot_index < s.len().get());

    let last_index = s.len().get() - 1;
    unsafe { s.swap_unchecked(pivot_index, last_index) };

    let mut i = 0;
    for j in 0..last_index {
        if s[j] <= s[last_index] {
            unsafe { s.swap_unchecked(i, j) };
            i += 1;
        }
    }
    unsafe { s.swap_unchecked(i, last_index) };
    i
}

/// Finds the k-th smallest element in an unsorted, non-empty slice
/// using the [Quickselect algorithm](https://en.wikipedia.org/wiki/Quickselect).
///
/// Works by iteratively partitioning the input slice into smaller parts,
/// discarding one side of the pivot element until the k-th smallest element is found.
///
/// See the [`get_pivot`] module for more information on the parameter of the same name.
///
/// # Panics
///
/// Panics if the specified value of `k` is out of bounds for the given slice `s`.
/// Additionally, panics if the index returned by `get_pivot` is out of bounds
/// for the input slice passed to it.
///
/// # Examples
///
/// ```
/// use not_empty::NonEmptySlice;
/// use alg_quickselect::quickselect;
///
/// let mut arr = [4, 2, 5, 1, 3];
/// let mut s = NonEmptySlice::new_mut(&mut arr).unwrap(); // 1 to 5
/// let k = 2; // Find the 3rd smallest element
///
/// let result = quickselect(s, k, |slice| slice.len().get() / 2);
/// assert_eq!(result, &mut 3);
/// ```
pub fn quickselect<T: Ord>(
    mut s: &mut NonEmptySlice<T>,
    mut k: usize,
    mut get_pivot: impl FnMut(&mut NonEmptySlice<T>) -> usize,
) -> &mut T {
    if k >= s.len().get() {
        panic!(
            "index out of bounds: the len is {len} but the index is {idx}",
            len = s.len().get(),
            idx = k,
        );
    }

    loop {
        let pivot_index = get_pivot(s);
        if pivot_index >= s.len().get() {
            panic!(
                "invalid pivot: index out of bounds: the len is {len} but the index is {idx}",
                len = s.len().get(),
                idx = pivot_index
            )
        }
        let pivot_index = unsafe { partition_unchecked(s, pivot_index) };
        // 0 <= pivot_index < s.len()
        // because if pivot_index >= s.len(), partition would have panicked

        match pivot_index.cmp(&k) {
            Ordering::Equal => return unsafe { s.get_unchecked_mut(k) }, // 0 <= k < s.len()
            Ordering::Less => {
                // before: 0 <= k < s.len()
                k -= pivot_index + 1;
                // after:  0 <= k < s.len() - 1
                // since k is greater than pivot_index (by at least +1), this will never underflow

                // To make sure the new slice is not empty
                // Safety condition: pivot_index + 1 < s.len()
                s = unsafe { NonEmptySlice::new_mut_unchecked(&mut s[pivot_index + 1..]) };
                // transitively: pivot_index < k < s.len()
                // therefore, pivot_index < s.len() - 1
                // and pivot_index + 1 < s.len()
                // (using k before because pivot_index does not change)
            }
            Ordering::Greater => {
                // Safety condition: 0 < pivot_index <= s.len()
                s = unsafe { NonEmptySlice::new_mut_unchecked(&mut s[..pivot_index]) };
                // Safe because
                // 1. pivot_index < s.len()
                // 2. pivot_index != 0 because pivot_index > k >= 0, so pivot_index is at least 1
            }
        }
    }
}

/// Unsafe version of [`quickselect`]. It does not perform bounds checks
/// nor panic when indices are out-of-bounds.
///
/// # Safety
///
/// The same invariants as stated by the [panic section](quickselect#panics)
/// of the safe version must be upheld. However, instead of panicking,
/// violating these conditions is undefined behavior.
pub unsafe fn quickselect_unchecked<T: Ord>(
    mut s: &mut NonEmptySlice<T>,
    mut k: usize,
    mut get_pivot: impl FnMut(&mut NonEmptySlice<T>) -> usize,
) -> &mut T {
    debug_assert!(k < s.len().get());

    loop {
        let pivot_index = get_pivot(s);
        debug_assert!(pivot_index < s.len().get());
        let pivot_index = unsafe { partition_unchecked(s, pivot_index) };
        debug_assert!(pivot_index < s.len().get());

        match pivot_index.cmp(&k) {
            Ordering::Equal => return unsafe { s.get_unchecked_mut(k) },
            Ordering::Less => {
                debug_assert!(k < s.len().get());
                k -= pivot_index + 1;
                debug_assert!(k < s.len().get() - 1);

                debug_assert!(pivot_index + 1 < s.len().get());
                s = unsafe { NonEmptySlice::new_mut_unchecked(&mut s[pivot_index + 1..]) };
            }
            Ordering::Greater => {
                debug_assert!(pivot_index < s.len().get());
                debug_assert!(pivot_index > 0);
                s = unsafe { NonEmptySlice::new_mut_unchecked(&mut s[..pivot_index]) };
            }
        }
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn it_works() {}
}
