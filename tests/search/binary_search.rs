use rusthms::algorithms::search::binary_search::binary_search_iterative;
use rusthms::algorithms::search::binary_search::binary_search_recusrive;

#[test]
pub fn it_find_number_in_sorted_array_recursive_binary_search() {
    fn assert_tests(tests: Vec<(i32, i32)>, arr: Vec<i32>) {
        let size = (arr.len() - 1) as i32;
        for &(expect, target) in &tests {
            let result = binary_search_recusrive(&arr, target, 0, size);
            assert_eq!(expect, result);
        }
    }

    // sorted array
    let mut arr = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // tests set
    let mut tests: Vec<(i32, i32)> = vec![(-1, 0), (0, 1), (4, 5), (5, 6)];

    assert_tests(tests, arr);

    arr = vec![7];
    tests = vec![(-1, 0), (-1, 3), (-1, 5), (-1, 20), (0, 7), (-1, 99)];

    assert_tests(tests, arr);

    arr = vec![
        45, 78, 90, 100, 150, 200, 300, 400, 500, 600, 700, 800, 900, 1000,
    ];
    tests = vec![
        (-1, 0),
        (0, 45),
        (1, 78),
        (2, 90),
        (3, 100),
        (4, 150),
        (5, 200),
        (6, 300),
        (7, 400),
        (8, 500),
        (9, 600),
        (10, 700),
        (11, 800),
        (12, 900),
        (13, 1000),
        (-1, 434),
        (-1, 1001),
    ];

    assert_tests(tests, arr);
}

#[test]
pub fn it_find_number_in_sorted_array_iterative_binary_search() {
    fn assert_tests(tests: Vec<(i32, i32)>, arr: Vec<i32>) {
        for &(expect, target) in &tests {
            let result = binary_search_iterative(&arr, target);
            assert_eq!(expect, result);
        }
    }

    // sorted array
    let mut arr = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // tests set
    let mut tests: Vec<(i32, i32)> = vec![(-1, 0), (0, 1), (4, 5), (5, 6)];

    assert_tests(tests, arr);

    arr = vec![7];
    tests = vec![(-1, 0), (-1, 3), (-1, 5), (-1, 20), (0, 7), (-1, 99)];

    assert_tests(tests, arr);

    arr = vec![
        45, 78, 90, 100, 150, 200, 300, 400, 500, 600, 700, 800, 900, 1000,
    ];
    tests = vec![
        (-1, 0),
        (0, 45),
        (1, 78),
        (2, 90),
        (3, 100),
        (4, 150),
        (5, 200),
        (6, 300),
        (7, 400),
        (8, 500),
        (9, 600),
        (10, 700),
        (11, 800),
        (12, 900),
        (13, 1000),
        (-1, 434),
        (-1, 1001),
    ];

    assert_tests(tests, arr);
}
