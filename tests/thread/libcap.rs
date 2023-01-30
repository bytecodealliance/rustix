use rustix::thread;

#[test]
fn libcap() {
    thread::set_capabilities(None, thread::capabilities(None).unwrap()).unwrap();
}
