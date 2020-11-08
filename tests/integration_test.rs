use match_any::match_any;

#[test]
fn match_any_result() {
    let result: Result<i32, i64> = Err(1);
    let option: Option<i64> = match_any!(result, Ok(i) | Err(i) => Some(i.into()));
    assert_eq!(option, Some(1));
}

#[test]
fn unnecessary_match_any_ok() {
    let a = 30;
    let x = match_any!(a, ref b => b);
    assert_eq!(*x, a);
}
