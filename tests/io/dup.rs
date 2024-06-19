#[cfg(feature = "fs")]
#[test]
fn test_dup() {
    let file = std::fs::File::open("Cargo.toml").unwrap();
    let alt = rustix::io::dup(&file).unwrap();

    // Initial position is 0.
    assert_eq!(
        rustix::fs::seek(&file, rustix::fs::SeekFrom::Current(0)),
        Ok(0)
    );
    assert_eq!(
        rustix::fs::seek(&alt, rustix::fs::SeekFrom::Current(0)),
        Ok(0)
    );

    let mut buf = [0_u8; 4];
    assert_eq!(rustix::io::read(&file, &mut buf), Ok(4));

    // Both positions updated.
    assert_eq!(
        rustix::fs::seek(&file, rustix::fs::SeekFrom::Current(0)),
        Ok(4)
    );
    assert_eq!(
        rustix::fs::seek(&alt, rustix::fs::SeekFrom::Current(0)),
        Ok(4)
    );

    assert_eq!(rustix::io::read(&alt, &mut buf), Ok(4));

    // Both positions updated.
    assert_eq!(
        rustix::fs::seek(&file, rustix::fs::SeekFrom::Current(0)),
        Ok(8)
    );
    assert_eq!(
        rustix::fs::seek(&alt, rustix::fs::SeekFrom::Current(0)),
        Ok(8)
    );

    drop(file);

    assert_eq!(rustix::io::read(&alt, &mut buf), Ok(4));

    assert_eq!(
        rustix::fs::seek(&alt, rustix::fs::SeekFrom::Current(0)),
        Ok(12)
    );
}
