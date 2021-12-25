use rustix::{process, io};

#[test]
fn test_posix_spawn() {
    let (read_pipe, write_pipe) = io::pipe().unwrap();
    let message = "posix_spawn works";
    let env_vars: &[&str] = &[];
    let mut config = process::SpawnConfig::default();
    let stdout = unsafe { io::stdout() };

    config.add_dup2_action(&write_pipe, &stdout);

    let pid = process::posix_spawn(
        "/usr/bin/echo",
        &["echo", "-n", message],
        env_vars,
        &config,
    ).unwrap();

    // ensure reading from the pipe ends when the child finishes writing
    core::mem::drop(write_pipe);
    let mut buf = [0; 32];

    let len = io::read(&read_pipe, &mut buf).unwrap();
    let output = std::str::from_utf8(&buf[..len]).unwrap();
    assert_eq!(output, message);
    process::waitpid(Some(pid), process::WaitOptions::empty()).unwrap();
}
