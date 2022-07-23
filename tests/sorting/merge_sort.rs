use rusthms::algorithms::sorting::merge_sort::merge_sort;

#[test]
pub fn it_sorts_an_array_of_integers_using_merge_sort() {
    let mut arr = vec![5, 4, 3, 2, 1];
    let sorted = vec![1, 2, 3, 4, 5];

    assert_eq!(merge_sort(&mut arr), sorted);

    arr = vec![1, 2, 3, 4, 5];
    assert_eq!(merge_sort(&mut arr), sorted);

    arr = vec![2, 1, 3, 5, 4];
    assert_eq!(merge_sort(&mut arr), sorted);
}
