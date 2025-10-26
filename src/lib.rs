
// main libs of project 
#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

#[cfg(feature = "std")]
pub mod cli;
pub mod formats;
#[cfg(feature = "std")]
pub mod sender;
pub mod structs;
pub mod parsing;
pub mod errors;
pub mod packets;
pub mod prelude;
pub mod utils;