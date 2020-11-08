/// Matches an expression to any of the patterns and executes the same expression arm for any match.
///
/// # Examples
///
/// ```
/// use match_any::match_any;
/// let result: Result<i64, i32> = Err(42);
/// let option: Option<i64> = match_any!(result, Ok(i) | Err(i) => Some(i.into()));
/// assert_eq!(option, Some(42));
/// ```
///
/// # Macro Expansion
///
/// ```
/// use match_any::match_any;
/// let result: Result<i32, i32> = Err(42);
/// match_any!(result, Ok(i) | Err(i) => Some(i));
/// ```
/// expands to
/// ```
/// let result: Result<i32, i32> = Err(42);
/// match result { Ok(i) => Some(i), Err(i) => Some(i) };
/// ```
#[macro_export]
macro_rules! match_any {
    ( $expr:expr , $( $( $pat:pat )|+ => $expr_arm:expr ),+ ) => {
        match $expr {
            $(
                $( $pat => $expr_arm, )+
            )+
        }
    };
}
