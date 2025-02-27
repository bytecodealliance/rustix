//! Fixing errors related to the `Buffer` trait.
//!
//! This example demonstrates some error messages that can come from using the
//! `Buffer` trait and how to fix them.

fn main() {
    error_buffer_wrapper();
    error_retry_closure();
    error_retry_indirect_closure();
    error_empty_slice();
    error_array_by_value();
}

fn error_buffer_wrapper() {
    use rustix::io::read;

    // This is reduced from src/fs/inotify.rs line 177.
    struct Wrapper<'a>(&'a mut [u8]);
    impl<'a> Wrapper<'a> {
        fn read(&mut self) {
            let input = std::fs::File::open("Cargo.toml").unwrap();

            // Ideally we'd write this, but it gets:
            // "cannot move out of `self` which is behind a mutable reference".
            /*
            let _x: usize = read(&input, self.0).unwrap();
            */

            // The fix: add `&mut *`.
            let _x: usize = read(&input, &mut *self.0).unwrap();
        }
    }
    let mut buf = vec![0_u8; 3];
    let mut wrapper = Wrapper(&mut buf);
    wrapper.read();
}

fn error_retry_closure() {
    use rustix::buffer::Buffer;
    use rustix::io;
    use rustix::io::retry_on_intr;

    fn b<B: Buffer<u8>>(_: B) -> Result<(), io::Errno> {
        Ok(())
    }

    let mut event_buf = [0; 4];

    // Ideally we'd write this, but it gets:
    // "cannot move out of `event_buf`, a captured variable in an `FnMut` closure".
    /*
    let event_buf_slice = event_buf.as_mut_slice();
    retry_on_intr(|| b(event_buf_slice)).unwrap();
    */

    // The fix: Add `&mut *`.
    let event_buf_slice = event_buf.as_mut_slice();
    retry_on_intr(|| b(&mut *event_buf_slice)).unwrap();
}

fn error_retry_indirect_closure() {
    use rustix::buffer::Buffer;
    use rustix::io;
    let flag = true;

    // This is reduced from the xattr crate, src/sys/linux_macos.rs line 119.

    // Ideally we'd write this, but it gets:
    // "borrowed data escapes outside of closure".
    /*
    let func = if flag {
        f
    } else {
        g
    };
    let _vec = allocate_loop(|buf| func(buf)).unwrap();
    */

    // The fix: Move `func` to inside the closure.
    let _vec = allocate_loop(|buf| {
        let func = if flag { f } else { g };
        func(buf)
    })
    .unwrap();

    fn allocate_loop<F: FnMut(&mut [u8]) -> io::Result<usize>>(mut f: F) -> io::Result<Vec<u8>> {
        let mut vec: Vec<u8> = Vec::new();
        loop {
            let ret = f(&mut [])?;
            vec.resize(ret, 0);

            match f(&mut vec) {
                Ok(size) => {
                    vec.truncate(size);
                    vec.shrink_to_fit();
                    return Ok(vec);
                }

                Err(io::Errno::RANGE) => continue,
                Err(err) => return Err(err),
            }
        }
    }
    fn f<B: Buffer<u8>>(_: B) -> Result<usize, io::Errno> {
        Ok(0)
    }
    fn g<B: Buffer<u8>>(_: B) -> Result<usize, io::Errno> {
        Ok(0)
    }
}

fn error_empty_slice() {
    use rustix::io::read;
    let input = std::fs::File::open("Cargo.toml").unwrap();

    // Functions that take `&mut [u8]` can be passed `&mut []`, but with
    // `Buffer` passing `&mut []` gets:
    // "type annotations needed".
    /*
    let _x = read(&input, &mut []).unwrap();
    */

    // The fix: make the element type explicit.
    let _x = read(&input, &mut [0_u8; 0]).unwrap();
}

fn error_array_by_value() {
    use rustix::io::read;
    let input = std::fs::File::open("Cargo.toml").unwrap();

    // This code is erroneously attempts to pass a buffer by value, but it
    // confusingly gets two error messages:
    // "the trait bound `[{integer}; 3]: Buffer<u8>` is not satisfied", and
    // "the trait bound `[{integer}; 3]: buffer::private::Sealed<u8>` is not satisfied".
    /*
    let mut buf = [0, 0, 0];
    let _x = read(&input, buf).unwrap();
    */

    // The fix: pass the buffer by reference.
    let mut buf = [0, 0, 0];
    let _x = read(&input, &mut buf).unwrap();
}
