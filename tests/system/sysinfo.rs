#[test]
#[allow(unused_comparisons)]
fn test_sysinfo() {
    let sysinfo: rustix::system::Sysinfo = rustix::system::sysinfo();

    // Values can vary, but we can test a few simple things.
    assert!(sysinfo.uptime >= 0);
    assert!(sysinfo.totalram > 0);
    assert!(sysinfo.procs > 0);
}
