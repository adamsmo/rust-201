pub fn split<T: Copy + Default, const SIZE: usize, const SPLIT_AT: usize>(
    arr: [T; SIZE],
) -> ([T; SPLIT_AT], [T; SIZE - SPLIT_AT]) {
    let mut first = [T::default(); SPLIT_AT];
    let mut rest = [T::default(); SIZE - SPLIT_AT];

    for i in 0..SIZE {
        if i < SPLIT_AT {
            first[i] = arr[i];
        } else {
            rest[i - SPLIT_AT] = arr[i];
        }
    }

    //first and rest are fixed size arrays
    (first, rest)
}

#[test]
fn check_split() {
    let array = [5, 6, 42, 24];
    // specify split via tubo fish
    let (first, rest) = split::<_, _, 2>(array);
    assert_eq!(first, [5, 6]);
    assert_eq!(rest, [42, 24]);

    let array = [5, 6, 42, 24];
    // infer split from deconstructing tuple
    let (first, rest): (_, [i32; 3]) = split(array);
    assert_eq!(first, [5]);
    assert_eq!(rest, [6, 42, 24]);

    let array = [5, 6, 42, 24];
    // infer split from usage in assertions
    let (first, rest) = split(array);
    assert_eq!(first, [5]);
    assert_eq!(rest, [6, 42, 24]);
}
