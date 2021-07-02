#![cfg_attr(target_os = "wasi", feature(wasi_ext))]

mod path {
    mod arg;
    mod dec_int;
}
