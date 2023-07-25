use crate::prelude::*;

#[macro_export]
macro_rules! not_empty {
    ($x:expr, $ev:path) => {
        if $x.is_empty() {
            return Err($ev(f!("{} is required", stringify!($x))));
        }
    };
    ($x:expr, $ev:path, $msg:literal) => {
        if $x.is_empty() {
            return Err($ev(String::from($msg)));
        }
    };
}

#[macro_export]
macro_rules! positive {
    ($x:expr, $ev:path) => {
        if $x <= 0 {
            return Err($ev(f!("{} must be greater than zero", stringify!($x))));
        }
    };
    ($x:expr, $ev:path, $msg:literal) => {
        if $x <= 0 {
            return Err($ev(String::from($msg)));
        }
    };
}

#[macro_export]
macro_rules! uuid {
    ($x:expr, $ev:path) => {
        use std::str::FromStr;
        use uuid::Uuid;
        if Uuid::from_str(&$x).is_err() {
            return Err($ev(f!("{} must be a valid uuid", stringify!($x))));
        }
    };
    ($x:expr, $ev:path, $msg:literal) => {
        use std::str::FromStr;
        use uuid::Uuid;
        if Uuid::from_str($x).is_err() {
            return Err($ev(String::from($msg)));
        }
    };
}

#[macro_export]
macro_rules! ok_or_err {
    ($x:expr, $ev:path) => {
        $x.as_ref()
            .ok_or($ev(f!("{} is required", stringify!($x))))?
    };
    ($x:expr, $ev:path, $msg:literal) => {
        $x.as_ref().ok_or($ev(String::from($msg)))
    };
}

pub use {not_empty, ok_or_err, positive, uuid};
