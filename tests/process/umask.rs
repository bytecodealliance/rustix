#[cfg(feature = "fs")]
#[test]
fn test_umask() {
    use rustix::fs::Mode;

    let tmp = Mode::RWXO | Mode::RWXG;
    let old = rustix::process::umask(tmp);
    let new = rustix::process::umask(old);
    assert_eq!(tmp, new);
}
