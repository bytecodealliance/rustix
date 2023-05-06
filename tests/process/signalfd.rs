//! Tests for signalfd

use rustix::io::{poll, PollFd, PollFlags};
use rustix::process::{signalfd_create, signalfd_modify, SigSet, Signal, SignalfdFlags};

use std::process::Command;
use std::slice::from_mut;

#[test]
fn test_signalfd() {
    // Create a new signalfd.
    let mut set = SigSet::new();
    set.add(Signal::Child);
    let fd = signalfd_create(&set, SignalfdFlags::empty()).unwrap();

    // Polling the signalfd should not yield any events.
    let mut pollfd = PollFd::new(&fd, PollFlags::IN);
    poll(from_mut(&mut pollfd), 0).unwrap();
    assert!(pollfd.revents().is_empty());

    // Spawn a child process and let it complete.
    Command::new("true").spawn().unwrap().wait().unwrap();

    // We should have received a `Signal::Child` event.
    poll(from_mut(&mut pollfd), 0).unwrap();
    assert!(pollfd.revents().contains(PollFlags::IN));

    // Modify the signalfd to stop listening for `Signal::Child`.
    signalfd_modify(&fd, &SigSet::new(), SignalfdFlags::empty()).unwrap();

    // Spawn another child process and let it complete.
    Command::new("true").spawn().unwrap().wait().unwrap();

    // We should not have received a `Signal::Child` event.
    pollfd.clear_revents();
    poll(from_mut(&mut pollfd), 0).unwrap();
    assert!(!pollfd.revents().contains(PollFlags::IN));
}
