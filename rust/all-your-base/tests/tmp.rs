/*
use allyourbase as ayb;

#[test]
fn always() {
    assert_eq!(Ok(1), ayb::from_base_to_decimal(&[1], 2));
    assert_eq!(Ok(5), ayb::from_base_to_decimal(&[1, 0, 1], 2));
    assert_eq!(Ok(5), ayb::from_base_to_decimal(&[5], 10));
    assert_eq!(Ok(42), ayb::from_base_to_decimal(&[1, 0, 1, 0, 1, 0], 2));
    assert_eq!(Ok(42), ayb::from_base_to_decimal(&[1, 1, 2, 0], 3));
    assert_eq!(Ok(42), ayb::from_base_to_decimal(&[2, 10], 16));
    // assert_eq!(Ok(43), ayb::from_base_to_decimal(&[3,46, 60], 97));
    assert_eq!(Ok(442), ayb::from_base_to_decimal(&[4, 4, 2], 10));
}

#[test]
fn dtd() {
    assert_eq!(
        Ok(vec![1, 0, 1, 0, 1, 0]),
        ayb::decimal_to_digits_base(42, 2)
    );
    assert_eq!(Ok(vec![2, 10]), ayb::decimal_to_digits_base(42, 16));
    assert_eq!(Ok(vec![1]), ayb::decimal_to_digits_base(1, 10));
    assert_eq!(Ok(vec![1, 0, 1]), ayb::decimal_to_digits_base(5, 2));
    assert_eq!(Ok(vec![4, 2]), ayb::decimal_to_digits_base(42, 10));
    assert_eq!(Ok(vec![1, 1, 2, 0]), ayb::decimal_to_digits_base(42, 3));
    assert_eq!(Ok(vec![4, 4, 2,]), ayb::decimal_to_digits_base(442, 10));
}
*/
