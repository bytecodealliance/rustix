use rustix::event::{epoll, Timespec};
use std::time::Instant;

#[test]
fn epoll_timeout() {
    let epoll_fd = epoll::create(epoll::CreateFlags::CLOEXEC).unwrap();

    let start = Instant::now();
    let mut events = Vec::with_capacity(1);
    epoll::wait(
        &epoll_fd,
        &mut events,
        Some(&Timespec {
            tv_sec: 0,
            tv_nsec: 1_000_000,
        }),
    )
    .unwrap();

    let duration = start.elapsed();

    assert!(
        duration.as_secs() > 0 || (duration.as_secs() == 0 && duration.subsec_nanos() >= 1_000_000)
    );
}
