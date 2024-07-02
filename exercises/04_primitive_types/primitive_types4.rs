// primitive_types4.rs
//
// Get a slice out of Array a where the ??? is so that the test passes.
//
// Execute `rustlings hint primitive_types4` or use the `hint` watch subcommand
// for a hint.

fn slice(arr: &[i8; 5], start: usize, end: usize) -> &[i8] {
    return &arr[start..end];
}

#[test]
fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];

    let nice_slice = slice(&a, 1, 4);

    assert_eq!([2, 3, 4], nice_slice)
}
