use rustix::process::{Resource, Rlimit};

#[test]
fn test_getrlimit() {
    let lim = rustix::process::getrlimit(Resource::Stack);
    assert_ne!(lim.current, Some(0));
    assert_ne!(lim.maximum, Some(0));
}

#[test]
fn test_setrlimit() {
    let new = Rlimit {
        current: Some(0),
        maximum: Some(0),
    };
    rustix::process::setrlimit(Resource::Core, new).unwrap();
}

#[cfg(any(target_os = "android", target_os = "linux"))]
#[test]
fn test_prlimit() {
    let new = Rlimit {
        current: Some(0),
        maximum: Some(0),
    };

    let _old = rustix::process::prlimit(None, Resource::Core, new.clone()).unwrap();

    let again =
        rustix::process::prlimit(Some(rustix::process::getpid()), Resource::Core, new.clone())
            .unwrap();

    assert_eq!(again, new);
}
