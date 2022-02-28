// We need to forward routine registration from C to Rust
// to avoid the linker removing the static library.

void R_init_r_rust_bio_extendr(void *dll);

void R_init_r_rust_bio(void *dll) {
    R_init_r_rust_bio_extendr(dll);
}
