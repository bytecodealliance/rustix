#[test]
fn test_dir_read_from() {
    let t = rustix::fs::openat(
        rustix::fs::CWD,
        rustix::cstr!("."),
        rustix::fs::OFlags::RDONLY | rustix::fs::OFlags::CLOEXEC,
        rustix::fs::Mode::empty(),
    )
    .unwrap();

    let mut dir = rustix::fs::Dir::read_from(&t).unwrap();

    let _file = rustix::fs::openat(
        &t,
        rustix::cstr!("Cargo.toml"),
        rustix::fs::OFlags::RDONLY | rustix::fs::OFlags::CLOEXEC,
        rustix::fs::Mode::empty(),
    )
    .unwrap();

    // Read the directory entries. We use `while let Some(entry)` so that we
    // don't consume the `Dir` so that we can run more tests on it.
    let mut saw_dot = false;
    let mut saw_dotdot = false;
    let mut saw_cargo_toml = false;
    while let Some(entry) = dir.read() {
        let entry = entry.unwrap();
        if entry.file_name() == rustix::cstr!(".") {
            saw_dot = true;
        } else if entry.file_name() == rustix::cstr!("..") {
            saw_dotdot = true;
        } else if entry.file_name() == rustix::cstr!("Cargo.toml") {
            saw_cargo_toml = true;
        }
    }
    assert!(saw_dot);
    assert!(saw_dotdot);
    assert!(saw_cargo_toml);

    // Rewind the directory so we can iterate over the entries again.
    dir.rewind();

    // For what comes next, we don't need `mut` anymore.
    let dir = dir;

    // Read the directory entries, again. This time we use `for entry in dir`.
    let mut saw_dot = false;
    let mut saw_dotdot = false;
    let mut saw_cargo_toml = false;
    for entry in dir {
        let entry = entry.unwrap();
        if entry.file_name() == rustix::cstr!(".") {
            saw_dot = true;
        } else if entry.file_name() == rustix::cstr!("..") {
            saw_dotdot = true;
        } else if entry.file_name() == rustix::cstr!("Cargo.toml") {
            saw_cargo_toml = true;
        }
    }
    assert!(saw_dot);
    assert!(saw_dotdot);
    assert!(saw_cargo_toml);
}

#[cfg(any(
    linux_like,
    solarish,
    target_os = "fuchsia",
    target_os = "hermit",
    target_os = "openbsd",
    target_os = "redox"
))]
#[test]
fn test_dir_seek() {
    use std::io::Write as _;

    let tempdir = tempfile::tempdir().unwrap();

    // Create many files so that we exhaust the readdir buffer at least once.
    let count = 500;
    let prefix = "file_with_a_very_long_name_to_make_sure_that_we_fill_up_the_buffer";
    let test_string = "This is a test string.";
    let mut filenames = Vec::<String>::with_capacity(count);
    for i in 0..count {
        let filename = format!("{}-{}.txt", prefix, i);
        let mut file = std::fs::File::create(tempdir.path().join(&filename)).unwrap();
        filenames.push(filename);
        file.write_all(test_string.as_bytes()).unwrap();
    }

    let t = rustix::fs::open(
        tempdir.path(),
        rustix::fs::OFlags::RDONLY | rustix::fs::OFlags::CLOEXEC,
        rustix::fs::Mode::empty(),
    )
    .unwrap();

    let mut dir = rustix::fs::Dir::read_from(&t).unwrap();

    // Read the first half of directory entries and record offset
    for _ in 0..count / 2 {
        dir.read().unwrap().unwrap();
    }
    let offset: i64 = dir.read().unwrap().unwrap().offset();

    // Read the rest of the directory entries and record the names
    let mut entries = Vec::new();
    while let Some(entry) = dir.read() {
        let entry = entry.unwrap();
        entries.push(entry.file_name().to_string_lossy().into_owned());
    }
    assert!(entries.len() >= count / 2);

    // Seek to the stored position. On 64-bit platforms we can `seek`.
    // On 32-bit platforms, `seek` isn't supported so rewind and scan.
    #[cfg(target_pointer_width = "64")]
    {
        dir.seek(offset).unwrap();
    }
    #[cfg(target_pointer_width = "32")]
    {
        dir.rewind();
        while let Some(entry) = dir.read() {
            let entry = entry.unwrap();
            if entry.offset() == offset {
                break;
            }
        }
    }

    // Confirm that we're getting the same results as before
    let mut entries2 = Vec::new();
    while let Some(entry) = dir.read() {
        let entry = entry.unwrap();
        entries2.push(entry.file_name().to_string_lossy().into_owned());
    }

    assert_eq!(entries, entries2);
}

#[test]
fn test_dir_new() {
    let t = rustix::fs::openat(
        rustix::fs::CWD,
        rustix::cstr!("."),
        rustix::fs::OFlags::RDONLY | rustix::fs::OFlags::CLOEXEC,
        rustix::fs::Mode::empty(),
    )
    .unwrap();

    let _file = rustix::fs::openat(
        &t,
        rustix::cstr!("Cargo.toml"),
        rustix::fs::OFlags::RDONLY | rustix::fs::OFlags::CLOEXEC,
        rustix::fs::Mode::empty(),
    )
    .unwrap();

    let mut dir = rustix::fs::Dir::new(t).unwrap();

    // Read the directory entries. We use `while let Some(entry)` so that we
    // don't consume the `Dir` so that we can run more tests on it.
    let mut saw_dot = false;
    let mut saw_dotdot = false;
    let mut saw_cargo_toml = false;
    while let Some(entry) = dir.read() {
        let entry = entry.unwrap();
        if entry.file_name() == rustix::cstr!(".") {
            saw_dot = true;
        } else if entry.file_name() == rustix::cstr!("..") {
            saw_dotdot = true;
        } else if entry.file_name() == rustix::cstr!("Cargo.toml") {
            saw_cargo_toml = true;
        }
    }
    assert!(saw_dot);
    assert!(saw_dotdot);
    assert!(saw_cargo_toml);

    // Rewind the directory so we can iterate over the entries again.
    dir.rewind();

    // For what comes next, we don't need `mut` anymore.
    let dir = dir;

    // Read the directory entries, again. This time we use `for entry in dir`.
    let mut saw_dot = false;
    let mut saw_dotdot = false;
    let mut saw_cargo_toml = false;
    for entry in dir {
        let entry = entry.unwrap();
        if entry.file_name() == rustix::cstr!(".") {
            saw_dot = true;
        } else if entry.file_name() == rustix::cstr!("..") {
            saw_dotdot = true;
        } else if entry.file_name() == rustix::cstr!("Cargo.toml") {
            saw_cargo_toml = true;
        }
    }
    assert!(saw_dot);
    assert!(saw_dotdot);
    assert!(saw_cargo_toml);
}

// Test that `Dir` silently stops iterating if the directory has been removed.
//
// Except on FreeBSD and macOS, where apparently `readdir` just keeps reading.
#[cfg_attr(any(apple, freebsdlike), ignore)]
#[test]
fn dir_iterator_handles_dir_removal() {
    // create a dir, keep the FD, then delete the dir
    let tmp = tempfile::tempdir().unwrap();
    let fd = rustix::fs::openat(
        rustix::fs::CWD,
        tmp.path(),
        rustix::fs::OFlags::RDONLY | rustix::fs::OFlags::CLOEXEC,
        rustix::fs::Mode::empty(),
    )
    .unwrap();

    // Drop the `TempDir`, which deletes the directory.
    drop(tmp);

    let mut dir = rustix::fs::Dir::read_from(&fd).unwrap();
    assert!(dir.next().is_none());
}

// Like `dir_iterator_handles_dir_removal`, but close the directory after
// `Dir::read_from`.
#[cfg_attr(any(apple, freebsdlike, target_os = "cygwin"), ignore)]
#[test]
fn dir_iterator_handles_dir_removal_after_open() {
    // create a dir, keep the FD, then delete the dir
    let tmp = tempfile::tempdir().unwrap();
    let fd = rustix::fs::openat(
        rustix::fs::CWD,
        tmp.path(),
        rustix::fs::OFlags::RDONLY | rustix::fs::OFlags::CLOEXEC,
        rustix::fs::Mode::empty(),
    )
    .unwrap();

    let mut dir = rustix::fs::Dir::read_from(&fd).unwrap();

    // Drop the `TempDir`, which deletes the directory.
    drop(tmp);

    assert!(dir.next().is_none());
}
