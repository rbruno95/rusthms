use rusthms::algorithms::math::sum::sum_first_n_natural_numbers;

#[test]
pub fn it_sum_first_n_natural_numbers() {
    assert_eq!(0, sum_first_n_natural_numbers(0));
    assert_eq!(1, sum_first_n_natural_numbers(1));
    assert_eq!(3, sum_first_n_natural_numbers(2));
    assert_eq!(6, sum_first_n_natural_numbers(3));
    assert_eq!(10, sum_first_n_natural_numbers(4));
    assert_eq!(15, sum_first_n_natural_numbers(5));
    assert_eq!(21, sum_first_n_natural_numbers(6));
    assert_eq!(28, sum_first_n_natural_numbers(7));
    assert_eq!(36, sum_first_n_natural_numbers(8));
    assert_eq!(45, sum_first_n_natural_numbers(9));
    assert_eq!(55, sum_first_n_natural_numbers(10));
    assert_eq!(30628, sum_first_n_natural_numbers(247));
    assert_eq!(761995, sum_first_n_natural_numbers(1234));
    assert_eq!(59631757185, sum_first_n_natural_numbers(345345));
    assert_eq!(500000500000, sum_first_n_natural_numbers(1000000));
}
