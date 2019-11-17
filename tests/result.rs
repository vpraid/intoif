use into_option::IntoResult;

#[test]
fn ok_on_zero() {
    assert_eq!(Ok(0), 0.ok_if(|x| *x == 0, "omae"))
}

#[test]
fn err_on_zero() {
    assert_eq!(Err("wa"), 0.err_if(|x| *x == 0, "wa"))
}

#[test]
fn ok_on_empty_string() {
    assert_eq!(
        Ok("".to_string()),
        "".to_string().ok_if(String::is_empty, "mou"))
}

#[test]
fn err_on_empty_string() {
    assert_eq!(
        Err("shindeiru"),
        "".to_string().err_if(String::is_empty, "shindeiru"))
}
