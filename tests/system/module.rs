#[cfg(feature = "fs")]
#[test]
fn test_kernel_module() {
    use rustix::cstr;
    use rustix::system::*;

    let _ = init_module(&[], cstr!(""));
    let _ = finit_module(rustix::fs::CWD, cstr!(""), 0);
    let _ = delete_module(cstr!(""), 0);
}
