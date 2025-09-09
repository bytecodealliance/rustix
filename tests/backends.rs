//! A test that compiles the test crates in the `test-crates` directories in
//! various configurations, including backend configurations, and tests that
//! they behave as expected.

use std::process::{Command, Stdio};

#[test]
#[ignore] // TODO: re-enable until tempfile is updated
fn test_backends() {
    // Pick an arbitrary platform where linux_raw is enabled by default and
    // ensure that the use-default crate uses it.
    #[cfg(all(target_os = "linux", target_arch = "aarch64"))]
    {
        assert!(
            has_dependency(
                "test-crates/use-default",
                &[],
                &[],
                &["RUSTFLAGS"],
                "linux-raw-sys"
            ),
            "use-default does not depend on linux-raw-sys"
        );
        assert!(
            !has_dependency("test-crates/use-default", &[], &[], &["RUSTFLAGS"], "libc"),
            "use-default depends on libc"
        );
        // If the libc backend is explicitly requested (by a cfg), check that
        // it's used.
        assert!(
            has_dependency(
                "test-crates/use-default",
                &[],
                &[("RUSTFLAGS", "--cfg=rustix_use_libc")],
                &[],
                "libc"
            ),
            "use-default with `RUSTFLAGS=--cfg=use-libc` doesn't depend on libc"
        );
        // If the libc backend is explicitly requested (by a feature flag),
        // check that it's used.
        assert!(
            has_dependency(
                "test-crates/use-default",
                &["--features=rustix/use-libc"],
                &[],
                &[],
                "libc"
            ),
            "use-default with `--features=use-libc` doesn't depend on libc"
        );
    }

    // Rustix's use-libc-auxv feature calls into libc, but does not use the
    // libc crate to do so. Pick an arbitrary platform where linux_raw is
    // enabled by default and ensure that the use-libc-auxv crate uses it,
    // and does not use the libc crate.
    #[cfg(all(target_os = "linux", target_arch = "aarch64"))]
    {
        assert!(
            !has_dependency(
                "test-crates/use-libc-auxv",
                &[],
                &[],
                &["RUSTFLAGS"],
                "libc"
            ),
            "use-libc-auxv depends on libc"
        );

        assert!(
            has_dependency(
                "test-crates/use-libc-auxv",
                &[],
                &[],
                &["RUSTFLAGS"],
                "linux-raw-sys"
            ),
            "use-libc-auxv does not depend on linux-raw-sys"
        );
    }

    #[cfg(windows)]
    let libc_dep = "windows-sys";
    #[cfg(any(unix, target_os = "wasi"))]
    let libc_dep = "libc";

    // Test the use-libc crate, which enables the "use-libc" cargo feature.
    assert!(
        has_dependency("test-crates/use-libc", &[], &[], &["RUSTFLAGS"], libc_dep),
        "use-libc doesn't depend on {}",
        libc_dep
    );

    // Test the use-default crate with `--cfg=rustix_use_libc`.
    assert!(
        has_dependency(
            "test-crates/use-default",
            &[],
            &[("RUSTFLAGS", "--cfg=rustix_use_libc")],
            &[],
            libc_dep
        ),
        "use-default with --cfg=rustix_use_libc does not depend on {}",
        libc_dep
    );

    // Test the use-default crate with `--features=rustix/use-libc`.
    assert!(
        has_dependency(
            "test-crates/use-default",
            &["--features=rustix/use-libc"],
            &[],
            &[],
            libc_dep
        ),
        "use-default with --features=rustix/use-libc does not depend on {}",
        libc_dep
    );

    // Test that the windows crate does not depend on libc.
    #[cfg(windows)]
    assert!(
        !has_dependency("test-crates/use-default", &[], &[], &[], "libc"),
        "use-default depends on libc on windows",
    );
}

/// Test whether the crate at directory `dir` has a dependency on `dependency`,
/// setting the environment variables `envs` and unsetting the environment
/// variables `remove_envs` when running `cargo`.
fn has_dependency(
    dir: &str,
    args: &[&str],
    envs: &[(&str, &str)],
    remove_envs: &[&str],
    dependency: &str,
) -> bool {
    let mut command = Command::new("cargo");

    command
        .arg("tree")
        .arg("--quiet")
        .arg("--edges=normal")
        .arg(format!("--invert={}", dependency))
        .current_dir(dir)
        .stderr(Stdio::inherit());

    command.args(args);
    for (key, value) in envs {
        command.env(key, value);
    }
    for key in remove_envs {
        command.env_remove(key);
    }

    let child = command.output().unwrap();

    // `cargo tree --invert=foo` can fail in two different ways: it exits with
    // a non-zero status if the dependency is not present in the Cargo.toml
    // configuration, and it exists with a zero status and prints nothing if
    // the dependency is present but optional and not enabled. So we check for
    // both here.
    child.status.success() && !child.stdout.is_empty()
}
