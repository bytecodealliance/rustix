#[test]
fn test_proc_funcs() {
    let _maps = rustix::io::proc_self_maps().unwrap();
    let _status = rustix::io::proc_self_status().unwrap();
    let _pagemap = rustix::io::proc_self_pagemap().unwrap();
}
