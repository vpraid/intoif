use intoif::IntoOption;

#[test]
fn some_on_zero() {
    assert_eq!(Some(0), 0.some_if(|x| *x == 0))
}

#[test]
fn none_on_zero() {
    assert_eq!(None, 0.none_if(|x| *x == 0))
}

#[test]
fn some_on_empty_string() {
    assert_eq!(
        Some("".to_string()),
        "".to_string().some_if(String::is_empty))
}

#[test]
fn none_on_empty_string() {
    assert_eq!(
        None,
        "".to_string().none_if(String::is_empty))
}
