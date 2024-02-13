use not_empty::NonEmptySlice;

fn main() {
    let mut array = [1, 2, 3];
    let s = NonEmptySlice::new_mut(&mut array).unwrap();
    let k = 1;
    let get_pivot = |s: &mut NonEmptySlice<i32>| s.len().get() / 2;

    unsafe { alg_quickselect::quickselect_unchecked(s, k, get_pivot) };
}

// Choosing the `get_pivot` function depends on how you want to select the pivot element during each iteration of the Quickselect algorithm. The choice can affect the performance and efficiency of the algorithm.
//
// Here are a few strategies for choosing a `get_pivot` function:
//
// 1. **Median of Three**: You can select the pivot as the median of three randomly chosen elements from the slice. This strategy can help avoid worst-case performance scenarios.
//
// 2. **Random Pivot**: Simply select a random index as the pivot. This approach can provide good average-case performance and is easy to implement.
//
// 3. **Middle Element**: Choose the middle element of the slice as the pivot. This is a straightforward strategy and can work well for many cases.
//
// 4. **Fixed Pivot**: Always choose a fixed index as the pivot. For example, you could always choose the first, last, or middle element of the slice. This can simplify the implementation but may lead to suboptimal performance in certain cases.
//
// 5. **Median of Medians**: This is a more complex strategy that aims to select a pivot that is close to the true median of the slice. It involves recursively dividing the slice into smaller parts and selecting a pivot based on the medians of those parts.
//
// The choice of `get_pivot` depends on factors such as the size of the input slice, the distribution of elements, and the desired performance characteristics. It's often a good idea to experiment with different strategies and measure their performance to find the most suitable one for your specific use case.
