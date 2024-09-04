#![cfg_attr(
    debug_assertions,
    allow(
//        dead_code,
//        unused_imports,
//        non_snake_case,
//        unused_variables,
//        unused_mut,
//        non_camel_case_types,
        confusable_idents,
        uncommon_codepoints,
    )
)]

pub mod letter;
use letter::*;

pub mod word;
use word::*;

pub mod phrase;
//pub use phrase::*;

pub mod types;
use types::*;

pub mod macros;

//pub mod prelude;
//use prelude::*;

use std::fmt;
use std::fmt::Display;
use std::str::FromStr;

use strum_macros::Display as StrumDisplay;
use strum_macros::EnumString as StrumEnumString;

#[cfg(test)]
mod tests {
    //use super::*;
    //use crate::types::*;
    //
    //    #[test]
    //    fn test_lib() {
    //    }
}

