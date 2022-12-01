use rustix::process::*;

#[test]
fn test_parent_process_death_signal() {
    dbg!(parent_process_death_signal().unwrap());
}

#[test]
fn test_trace_status() {
    dbg!(trace_status(None).unwrap());
}
