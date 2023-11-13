use rustix::io;
use rustix::process::*;

#[test]
fn test_parent_process_death_signal() {
    dbg!(parent_process_death_signal().unwrap());
}

#[test]
fn test_trace_status() {
    dbg!(trace_status(None).unwrap());
}

#[test]
fn test_reaper_status() {
    match set_reaper_status(false).unwrap_err() {
        io::Errno::INVAL => (),
        io::Errno::PERM => return, // FreeBSD 12 doesn't support this
        err => panic!("{:?}", err),
    };
    set_reaper_status(true).unwrap();
    let status_while_acq = dbg!(get_reaper_status(None).unwrap());
    set_reaper_status(false).unwrap();
    let status_while_rel = dbg!(get_reaper_status(None).unwrap());
    assert!(status_while_acq.flags.contains(ReaperStatusFlags::OWNED));
    assert!(!status_while_rel.flags.contains(ReaperStatusFlags::OWNED));
}

#[test]
fn test_reaper_pids() {
    dbg!(get_reaper_pids(None).unwrap());
}

#[test]
fn test_trapcap() {
    assert_eq!(trap_cap_behavior(None).unwrap(), TrapCapBehavior::Disable);
    set_trap_cap_behavior(None, TrapCapBehavior::Enable).unwrap();
    assert_eq!(trap_cap_behavior(None).unwrap(), TrapCapBehavior::Enable);
    set_trap_cap_behavior(None, TrapCapBehavior::Disable).unwrap();
    assert_eq!(trap_cap_behavior(None).unwrap(), TrapCapBehavior::Disable);
}

#[test]
fn test_no_new_privs() {
    match no_new_privs(None) {
        Ok(flag) => assert!(!flag),
        Err(io::Errno::INVAL) => return, // FreeBSD 12 doesn't support this
        Err(err) => panic!("{:?}", err),
    };
    set_no_new_privs(None).unwrap();
    assert!(no_new_privs(None).unwrap());
    // No going back but, well, we're not gonna execute SUID binaries from the
    // test suite.
}
