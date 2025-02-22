use rustix::fs::FileTypeExt as _;
use rustix::io;
use rustix::termios::{isatty, ttyname};
use std::fs::File;

#[test]
fn test_ttyname_ok() {
    let file = match File::open("/dev/stdin") {
        Ok(file) => file,
        Err(err) if err.kind() == std::io::ErrorKind::PermissionDenied => return,
        Err(err) => panic!("{:?}", err),
    };
    if isatty(&file) {
        let name = ttyname(&file, Vec::new()).unwrap().into_string().unwrap();
        assert!(name.starts_with("/dev/"));
        assert!(!name.ends_with('/'));
        assert!(std::fs::metadata(&name)
            .unwrap()
            .file_type()
            .is_char_device());
    }
}

#[test]
fn test_ttyname_not_tty() {
    let file = File::open("Cargo.toml").unwrap();
    assert_eq!(ttyname(&file, Vec::new()).unwrap_err(), io::Errno::NOTTY);

    let file = File::open("/dev/null").unwrap();
    assert_eq!(ttyname(&file, Vec::new()).unwrap_err(), io::Errno::NOTTY);
}
