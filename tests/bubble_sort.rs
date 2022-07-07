use rusthms::algorithms::bubble_sort::sort;

#[test]
fn it_sort_array() {
    //declaring unsorted array of len 5
    let mut arr = [5, 4, 3, 2, 1];

    //calling bubble sort function on unsorted array
    sort(&mut arr);

    //asserting that the array is sorted
    assert_eq!([1, 2, 3, 4, 5], arr);
}
