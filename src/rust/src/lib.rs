use extendr_api::prelude::*;

pub mod io;
pub mod error;
mod test;

use crate::io::get_io_metadata;

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod rrustbio;
    use io;
}
