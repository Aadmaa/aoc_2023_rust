use std::{
    cmp::{max, min},
    iter::repeat_with,
};

/// Given a list of type `J`, Returns an iterator over `Vec<Option<J>>` that
/// gives you one Vector for each item in the list.
///
/// The vector has the original list item wrapped in Some, with
/// previous and subsequent lines included as well. The number of those is determined by
/// the arguments.
///
/// # Arguments
/// * `list` - The input list or iterator
/// * `piv` - Number of prior items visible on each iteration
/// * `siv` - Number of subsequent items visible on each iteration
///
/// # Examples
/// ```
/// // Define the input list and expected output
/// let list = vec![1, 2, 3, 4];
/// let expected_windows: Vec<Vec<Option<i32>>> = vec![
///     vec![None, Some(1), Some(2), Some(3)],
///     vec![Some(1), Some(2), Some(3), Some(4)],
///     vec![Some(2), Some(3), Some(4), None],
///     vec![Some(3), Some(4), None, None],
/// ];
///
/// let result: Vec<Vec<Option<i32>>> = aoc_2023::utils::iteration::window_each(list.into_iter(), 1, 2).collect();
///
/// assert_eq!(result, expected_windows);
/// ```
pub fn window_each<I, J>(list: I, piv: usize, siv: usize) -> impl Iterator<Item = Vec<Option<J>>>
where
    I: IntoIterator<Item = J>,
    J: Clone,
{
    let none_iter_before = repeat_with(|| None).take(max(piv, 0));
    let none_iter_after = repeat_with(|| None).take(max(siv, 0));

    //    none_iter_before.map(f)
    let list_vec: Vec<Option<J>> = none_iter_before
        .chain(list.into_iter().map(Some))
        .chain(none_iter_after)
        .collect();

    let window_size = piv + 1 + siv;
    (0..list_vec.len().saturating_sub(piv + siv))
        .map(move |i| list_vec[i..min(i + window_size, list_vec.len())].to_vec())
}

pub fn padded_iterator<I, J>(list: I, piv: usize, siv: usize) -> impl Iterator<Item = Option<J>>
where
    I: IntoIterator<Item = J>,
{
    let none_iter_before = repeat_with(|| None).take(max(piv, 0));
    let none_iter_after = repeat_with(|| None).take(max(siv, 0));

    none_iter_before
        .chain(list.into_iter().map(Some))
        .chain(none_iter_after)
}


#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_window_each_gen_1() {
        // Define the input list and expected output
        let list = vec![1, 2, 3, 4];
        let expected_windows: Vec<Vec<Option<i32>>> = vec![
            vec![None, Some(1), Some(2)],
            vec![Some(1), Some(2), Some(3)],
            vec![Some(2), Some(3), Some(4)],
            vec![Some(3), Some(4), None],
        ];

        // Convert the list into an iterator, apply the window_each function, and collect the results
        let result: Vec<Vec<Option<i32>>> = window_each(list.into_iter(), 1, 1).collect();

        // Assert that the result matches the expected output
        assert_eq!(result, expected_windows);
    }

    #[test]
    fn test_window_each_gen_2() {
        // Define the input list and expected output
        let list = vec![1, 2, 3, 4];
        let expected_windows: Vec<Vec<Option<i32>>> = vec![
            vec![None, Some(1), Some(2), Some(3)],
            vec![Some(1), Some(2), Some(3), Some(4)],
            vec![Some(2), Some(3), Some(4), None],
            vec![Some(3), Some(4), None, None],
        ];

        // Convert the list into an iterator, apply the window_each function, and collect the results
        let result: Vec<Vec<Option<i32>>> = window_each(list.into_iter(), 1, 2).collect();

        // Assert that the result matches the expected output
        assert_eq!(result, expected_windows);
    }

    #[test]
    fn test_window_each_gen_3() {
        // Define the input list and expected output
        let list = vec![1, 2, 3, 4];
        let expected_windows: Vec<Vec<Option<i32>>> = vec![
            vec![None, None, Some(1), Some(2)],
            vec![None, Some(1), Some(2), Some(3)],
            vec![Some(1), Some(2), Some(3), Some(4)],
            vec![Some(2), Some(3), Some(4), None],
        ];

        // Convert the list into an iterator, apply the window_each function, and collect the results
        let result: Vec<Vec<Option<i32>>> = window_each(list.into_iter(), 2, 1).collect();

        // Assert that the result matches the expected output
        assert_eq!(result, expected_windows);
    }

    #[test]
    fn test_window_each_gen_4() {
        // Define the input list and expected output
        let list = vec![1, 2];
        let expected_windows: Vec<Vec<Option<i32>>> = vec![
            vec![None, None, Some(1), Some(2), None, None],
            vec![None, Some(1), Some(2), None, None, None],
        ];

        // Convert the list into an iterator, apply the window_each function, and collect the results
        let result: Vec<Vec<Option<i32>>> = window_each(list.into_iter(), 2, 3).collect();

        // Assert that the result matches the expected output
        assert_eq!(result, expected_windows);
    }
}
