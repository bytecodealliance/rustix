use rustix::process::Resource;
#[cfg(not(target_os = "haiku"))] // No `Core` on Haiku.
use rustix::process::Rlimit;

#[test]
fn test_getrlimit() {
    let lim = rustix::process::getrlimit(Resource::Stack);
    assert_ne!(lim.current, Some(0));
    assert_ne!(lim.maximum, Some(0));
}

#[cfg(not(target_os = "haiku"))] // No `Core` on Haiku.
#[test]
fn test_setrlimit() {
    let old = rustix::process::getrlimit(Resource::Core);
    let new = Rlimit {
        current: Some(0),
        maximum: Some(4096),
    };
    assert_ne!(old, new);
    rustix::process::setrlimit(Resource::Core, new).unwrap();

    let lim = rustix::process::getrlimit(Resource::Core);
    assert_eq!(lim, new);

    #[cfg(linux_kernel)]
    {
        let new = Rlimit {
            current: Some(0),
            maximum: Some(0),
        };

        let first = rustix::process::getrlimit(Resource::Core);

        let old = match rustix::process::prlimit(None, Resource::Core, new) {
            Ok(rlimit) => rlimit,
            Err(rustix::io::Errno::NOSYS) => return,
            Err(err) => panic!("{:?}", err),
        };

        assert_eq!(first, old);

        let other = Rlimit {
            current: Some(0),
            maximum: Some(0),
        };

        let again =
            rustix::process::prlimit(Some(rustix::process::getpid()), Resource::Core, other)
                .unwrap();

        assert_eq!(again, new);
    }
}
