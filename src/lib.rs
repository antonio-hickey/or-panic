#![cfg_attr(feature = "no_std", no_std)]

use core::fmt::{Debug, Display};

/// The prelude: a collection of commonly-used types, traits, and functions.
///
/// Importing the prelude brings the most ergonomically important items
/// into scope:
///
/// ```
/// use or_panic::prelude::*;
/// ```
///
/// This is purely for convenience—everything in the prelude is also
/// available through their regular module paths.
pub mod prelude {
    pub use crate::{OptionOrPanic, ResultOrPanic};
}

pub trait OptionOrPanic<T> {
    /// Unwrap the [`Option`] or panic with a message.
    ///
    /// Results:
    /// `Some(T)` -> `T`
    /// [`None`] -> [`panic`] with custom message
    #[track_caller]
    fn or_panic<M: Display>(self, msg: M) -> T;
}

impl<T> OptionOrPanic<T> for Option<T> {
    #[track_caller]
    fn or_panic<M: Display>(self, msg: M) -> T {
        self.unwrap_or_else(|| panic!("{msg}"))
    }
}

pub trait ResultOrPanic<T> {
    /// Unwrap the [`Result`] or panic with a message.
    ///
    /// Results:
    /// Ok(T) -> `T`
    /// Err(E) -> [`panic`] with custom message
    ///
    /// NOTE: The error type of your [`Result`] must implement [`Debug`].
    #[track_caller]
    fn or_panic<M: Display>(self, msg: M) -> T;
}

impl<T, E: Debug> ResultOrPanic<T> for Result<T, E> {
    #[track_caller]
    fn or_panic<M: Display>(self, msg: M) -> T {
        self.unwrap_or_else(|err| panic!("{msg}.\nCaused by: {err:?}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn option_or_panic_returns_value() {
        let x = Some(42).or_panic("should not panic");
        assert_eq!(x, 42);
    }

    #[test]
    #[should_panic(expected = "option is missing")]
    fn option_or_panic_panics_on_none() {
        let x: Option<i32> = None;
        x.or_panic("option is missing");
    }

    #[test]
    #[should_panic(expected = "error 123")]
    fn option_or_panic_handles_display_types() {
        let x: Option<i32> = None;
        let code = 123;
        x.or_panic(format!("error {code}"));
    }

    #[test]
    fn result_or_panic_returns_ok_value() {
        let r: Result<i32, &str> = Ok(7);
        let x = r.or_panic("no panic expected");
        assert_eq!(x, 7);
    }

    #[test]
    #[should_panic(expected = "explicit failure.\nCaused by: \"boom\"")]
    fn result_or_panic_panics_on_err() {
        let r: Result<i32, &str> = Err("boom");
        r.or_panic("explicit failure");
    }

    #[test]
    #[should_panic(expected = "Bad stuff.\nCaused by: 404")]
    fn result_or_panic_works_with_display_formatting() {
        let r: Result<i32, i32> = Err(404);
        r.or_panic("Bad stuff");
    }

    #[test]
    fn result_or_panic_allows_non_string_message_types() {
        let r: Result<&str, &str> = Ok("yay");
        let msg_int = 999; // i32 implements Display

        let v = r.or_panic(msg_int);
        assert_eq!(v, "yay");
    }
}
