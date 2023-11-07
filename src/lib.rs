#![allow(dead_code)]

use errors::GitErrors;

pub type GitResult<T> = Result<T, GitErrors>;

/// TODO
pub mod auth_type;
/// TODO
pub mod defaults;
pub mod errors;
/// TODO
pub mod opt;
/// TODO
pub mod repository;

// #![ warn( rust_2018_idioms ) ]
// #![ warn( missing_debug_implementations ) ]
// #![ warn( missing_docs ) ]

// #![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]
