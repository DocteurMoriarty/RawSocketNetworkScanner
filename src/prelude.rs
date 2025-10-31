//! Prelude global du projet no_std
//! Définit des alias et importations communes.

#![allow(unused_imports)]
pub use alloc::{
    vec,
    vec::Vec as VecNoStd,
    string::{
        String as StringNoStd,
        ToString as ToStringNoStd
    },
    collections::BTreeMap,
    vec::Vec,
    boxed::Box
};

pub use core::{
    result::Result,
    fmt
};
