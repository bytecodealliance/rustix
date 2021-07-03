use criterion::{criterion_group, criterion_main, Criterion};

fn simple_statat(c: &mut Criterion) {
    use posish::fs::{cwd, statat, AtFlags};

    c.bench_function("simple statat", |b| {
        b.iter(|| {
            statat(&cwd(), "/", AtFlags::empty()).unwrap();
        })
    });
}

fn simple_statat_libc(c: &mut Criterion) {
    c.bench_function("simple statat libc", |b| {
        b.iter(|| {
            let mut s = std::mem::MaybeUninit::<libc::stat>::uninit();
            unsafe {
                assert_eq!(
                    libc::fstatat(
                        libc::AT_FDCWD,
                        std::ffi::CString::new("/").unwrap().as_c_str().as_ptr() as _,
                        s.as_mut_ptr(),
                        0
                    ),
                    0
                );
            }
        })
    });
}

fn simple_statat_libc_cstr(c: &mut Criterion) {
    c.bench_function("simple statat libc cstr", |b| {
        b.iter(|| {
            let mut s = std::mem::MaybeUninit::<libc::stat>::uninit();
            unsafe {
                assert_eq!(
                    libc::fstatat(
                        libc::AT_FDCWD,
                        cstr::cstr!("/").as_ptr() as _,
                        s.as_mut_ptr(),
                        0
                    ),
                    0
                );
            }
        })
    });
}

fn simple_statat_cstr(c: &mut Criterion) {
    use posish::fs::{cwd, statat, AtFlags};

    c.bench_function("simple statat cstr", |b| {
        b.iter(|| {
            statat(&cwd(), cstr::cstr!("/"), AtFlags::empty()).unwrap();
        })
    });
}

fn simple_clock_gettime(c: &mut Criterion) {
    use posish::time::{clock_gettime, ClockId};

    c.bench_function("simple clock_gettime", |b| {
        b.iter(|| {
            let _ = clock_gettime(ClockId::Monotonic);
        })
    });
}

fn simple_clock_gettime_libc(c: &mut Criterion) {
    c.bench_function("simple clock_gettime libc", |b| {
        b.iter(|| {
            let mut s = std::mem::MaybeUninit::<libc::timespec>::uninit();
            unsafe {
                assert_eq!(
                    libc::clock_gettime(libc::CLOCK_MONOTONIC, s.as_mut_ptr()),
                    0
                );
                let _ = s.assume_init();
            }
        })
    });
}

fn simple_getpid(c: &mut Criterion) {
    use posish::process::getpid;

    c.bench_function("simple getpid", |b| {
        b.iter(|| {
            let _ = getpid();
        })
    });
}

fn simple_getpid_libc(c: &mut Criterion) {
    c.bench_function("simple getpid libc", |b| {
        b.iter(|| unsafe {
            let _ = libc::getpid();
        })
    });
}

criterion_group!(
    benches,
    simple_statat,
    simple_statat_libc,
    simple_statat_libc_cstr,
    simple_statat_cstr,
    simple_clock_gettime,
    simple_clock_gettime_libc,
    simple_getpid,
    simple_getpid_libc
);
criterion_main!(benches);
