#[macro_export]
macro_rules! match_any {
    ( $match_expr:expr , $( $pat:pat )|+ => $expr:expr ) => {
        match $match_expr {
            $(
                $pat => $expr,
            )+
        }
    };
}
