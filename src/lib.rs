/// Matches an expression to any of the patterns and executes the same expression arm for any match.
///
/// This macro allows you to use the same expression arm for different types
/// by creating the same match arm for each of the patterns separated by `|`.
/// A standard match statement only allows such patterns for the same type:
///
/// ```compile_fail
/// let result: Result<i64, i32> = Err(42);
/// let int: i64 = match result { Ok(i) | Err(i) => i.into() }; // does not compile!
/// ```
///
/// ```
/// # use match_any::match_any;
/// let result: Result<i64, i32> = Err(42);
/// let int: i64 = match_any!(result, Ok(i) | Err(i) => i.into()); // compiles just fine
/// assert_eq!(int, 42);
/// ```
///
/// # Examples
///
/// ```
/// use match_any::match_any;
///
/// enum Id { U8(u8), I16(i16), I32(i32) }
/// use Id::*;
///
/// let id = Id::I16(-2);
/// let id: i32 = match_any!(id, U8(x) | I16(x) | I32(x) => x.into());
/// assert_eq!(id, -2);
/// ```
///
/// With multiple match arms:
///
/// ```
/// use core::convert::TryFrom;
/// use match_any::match_any;
///
/// enum Id { U8(u8), I16(i16), I32(i32), U64(u64) }
/// use Id::*;
///
/// let id = Id::I32(-3);
/// let id: i32 = match_any!(id,
///     U8(a) | I16(a) | I32(a) => a.into(),
///     U64(b) => i32::try_from(b).unwrap_or(0)
/// );
/// assert_eq!(id, -3);
/// ```
///
/// # Macro Expansion
///
/// ```
/// # use match_any::match_any;
/// let result: Result<i32, i32> = Err(42);
/// match_any!(result, Ok(i) | Err(i) => Some(i));
/// ```
/// expands to
/// ```
/// let result: Result<i32, i32> = Err(42);
/// match result { Ok(i) => Some(i), Err(i) => Some(i) };
/// ```
///
/// # Enum Dispatch
///
/// Similarly to the [enum_dispatch crate](https://crates.io/crates/enum_dispatch),
/// this macro can be used to implement "enum dispatch" as an alternative to dynamic dispatch.
/// The major difference between the enum_dispatch crate and this macro is,
/// that enum_dispatch provides a _procedural_ macro, while this is a _declarative_ macro.
/// This allows enum_dispatch to reduce the boilerplate code a lot more than match_any.
/// However IDE support should be a bit better with match_any.
///
/// ## Enum Dispatch Example
///
/// ```
/// use match_any::match_any;
///
/// trait IntId {
///     fn int_id(&self) -> i32;
/// }
///
/// impl IntId for u64 {
///     fn int_id(&self) -> i32 { 64 }
/// }
///
/// impl IntId for u32 {
///     fn int_id(&self) -> i32 { 32 }
/// }
///
/// enum IntIdKind { U64(u64), U32(u32) }
///
/// impl IntId for IntIdKind {
///     fn int_id(&self) -> i32 {
///         use IntIdKind::*;
///         match_any!(self, U64(i) | U32(i) => i.int_id())
///     }
/// }
///
/// let int_id_kind = IntIdKind::U32(0);
/// assert_eq!(int_id_kind.int_id(), 32); // enum dispatch
/// let int_id_box: Box<dyn IntId> = Box::new(0_u32);
/// assert_eq!(int_id_box.int_id(), 32); // dynamic dispatch
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
