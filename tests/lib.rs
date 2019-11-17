use into_option::IntoOption;

#[test]
fn none_on_zero() {
    assert_eq!(None, 0.none_if(|x| *x == 0))
}

#[test]
fn some_on_zero() {
    assert_eq!(Some(0), 0.some_if(|x| *x == 0))
}
