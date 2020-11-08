use match_any::match_any;

#[test]
fn match_any_result() {
    let result: Result<i32, i32> = Err(1);
    let option: Option<i32> = match_any!(result, Ok(i) | Err(i) => Some(i));
    assert_eq!(option, Some(1));
}
