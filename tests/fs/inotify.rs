use rustix::fs::inotify::{self, CreateFlags, WatchFlags};
use rustix::io::Errno;
use std::fmt::Write as _;
use std::fs::{create_dir_all, remove_file, rename, File};
use std::mem::MaybeUninit;

#[test]
fn test_inotify_iter() {
    let inotify = inotify::init(CreateFlags::NONBLOCK).unwrap();
    create_dir_all("/tmp/.rustix-inotify-test").unwrap();
    inotify::add_watch(
        &inotify,
        "/tmp/.rustix-inotify-test",
        WatchFlags::ALL_EVENTS,
    )
    .unwrap();

    File::create("/tmp/.rustix-inotify-test/foo").unwrap();
    rename(
        "/tmp/.rustix-inotify-test/foo",
        "/tmp/.rustix-inotify-test/bar",
    )
    .unwrap();
    remove_file("/tmp/.rustix-inotify-test/bar").unwrap();

    let mut output = String::new();
    let mut cookie = 0;

    let mut buf = [MaybeUninit::uninit(); 512];
    let mut iter = inotify::Reader::new(inotify, &mut buf);
    loop {
        let e = match iter.next() {
            Err(Errno::WOULDBLOCK) => break,
            r => r.unwrap(),
        };

        writeln!(output, "{e:#?}").unwrap();
        if e.cookie() != 0 {
            cookie = e.cookie();
        }
    }

    let expected = format!(
        r#"Event {{
    wd: 1,
    events: ReadFlags(
        CREATE,
    ),
    cookie: 0,
    file_name: Some(
        "foo",
    ),
}}
Event {{
    wd: 1,
    events: ReadFlags(
        OPEN,
    ),
    cookie: 0,
    file_name: Some(
        "foo",
    ),
}}
Event {{
    wd: 1,
    events: ReadFlags(
        CLOSE_WRITE,
    ),
    cookie: 0,
    file_name: Some(
        "foo",
    ),
}}
Event {{
    wd: 1,
    events: ReadFlags(
        MOVED_FROM,
    ),
    cookie: {cookie},
    file_name: Some(
        "foo",
    ),
}}
Event {{
    wd: 1,
    events: ReadFlags(
        MOVED_TO,
    ),
    cookie: {cookie},
    file_name: Some(
        "bar",
    ),
}}
Event {{
    wd: 1,
    events: ReadFlags(
        DELETE,
    ),
    cookie: 0,
    file_name: Some(
        "bar",
    ),
}}
"#
    );
    assert_eq!(expected, output);
}
