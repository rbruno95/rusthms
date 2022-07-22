use rusthms::algorithms::math::sum;

#[test]
pub fn it_sum_first_n_natural_numbers() {
    assert_eq!(1, sum::sum_first_n_natural_numbers(1));
    assert_eq!(3, sum::sum_first_n_natural_numbers(2));
    assert_eq!(6, sum::sum_first_n_natural_numbers(3));
    assert_eq!(10, sum::sum_first_n_natural_numbers(4));
    assert_eq!(15, sum::sum_first_n_natural_numbers(5));
    assert_eq!(21, sum::sum_first_n_natural_numbers(6));
    assert_eq!(28, sum::sum_first_n_natural_numbers(7));
    assert_eq!(36, sum::sum_first_n_natural_numbers(8));
    assert_eq!(45, sum::sum_first_n_natural_numbers(9));
    assert_eq!(55, sum::sum_first_n_natural_numbers(10));
}
