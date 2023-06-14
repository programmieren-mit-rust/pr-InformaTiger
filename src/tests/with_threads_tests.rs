use crate::with_threads::{divide_data, take_every_nth_value};

#[test]
fn test_divide_data() {
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];

    let divided_into_2_parts = divide_data(&data, 2);
    let divided_into_3_parts = divide_data(&data, 3);
    let divided_into_4_parts = divide_data(&data, 4);

    assert_eq!(
        divided_into_2_parts,
        vec![vec![1, 3, 5, 7, 9, 11], vec![2, 4, 6, 8, 10, 12]]
    );
    assert_eq!(
        divided_into_3_parts,
        vec![vec![1, 4, 7, 10], vec![2, 5, 8, 11], vec![3, 6, 9, 12]]
    );
    assert_eq!(
        divided_into_4_parts,
        vec![
            vec![1, 5, 9],
            vec![2, 6, 10],
            vec![3, 7, 11],
            vec![4, 8, 12]
        ]
    );
}

#[test]
fn test_take_every_nth_value() {
    let data: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
    let n = 4;

    let nth_from0 = take_every_nth_value(&data, n, 0);
    let nth_from1 = take_every_nth_value(&data, n, 1);
    let nth_from2 = take_every_nth_value(&data, n, 2);
    let nth_from3 = take_every_nth_value(&data, n, 3);

    assert_eq!(nth_from0, vec![1, 5, 9]);
    assert_eq!(nth_from1, vec![2, 6, 10]);
    assert_eq!(nth_from2, vec![3, 7, 11]);
    assert_eq!(nth_from3, vec![4, 8, 12]);
}
