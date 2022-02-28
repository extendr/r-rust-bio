use extendr_api::prelude::*;

pub mod gff;
// use crate::io::gff::get_gff_metadata;
use crate::io::gff::get_gff_metadata;

extendr_module! {
    mod io;
    use gff;
}
