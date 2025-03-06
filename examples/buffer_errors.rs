//! Fixing errors related to the `Buffer` trait.
//!
//! This example demonstrates some error messages that can come from using the
//! `Buffer` trait and how to fix them.

fn main() {
    error_buffer_wrapper();
    error_retry_closure();
    error_retry_closure_uninit();
    error_retry_indirect_closure();
    error_empty_slice();
    error_array_by_value();
}

fn error_buffer_wrapper() {
    use rustix::buffer::Buffer;

    fn read<B: Buffer<u8>>(_: B) {}

    // This is reduced from src/fs/inotify.rs line 177.
    struct Wrapper<'a>(&'a mut [u8]);
    impl<'a> Wrapper<'a> {
        fn read(&mut self) {
            // Ideally we'd write this, but it gets:
            // "cannot move out of `self` which is behind a mutable reference".
            /*
            read(self.0);
            */

            // The fix: add `&mut *`.
            read(&mut *self.0);
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

    fn b<B: Buffer<u8>>(b: B) -> io::Result<B::Output> {
        unsafe { Ok(b.assume_init(0)) }
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

fn error_retry_closure_uninit() {
    use rustix::buffer::Buffer;
    use rustix::io;
    use std::mem::MaybeUninit;

    fn b<B: Buffer<u8>>(b: B) -> io::Result<B::Output> {
        unsafe { Ok(b.assume_init(0)) }
    }

    let mut event_buf = [MaybeUninit::<u8>::uninit(); 4];

    // It's tempting to write this, but it gets:
    // "captured variable cannot escape `FnMut` closure body".
    /*
    rustix::io::retry_on_intr(|| b(&mut event_buf)).unwrap();
    */

    // The fix: Don't use `retry_on_intr`, unfortunately.
    loop {
        match b(&mut event_buf) {
            Ok((_init, _unini)) => break,
            Err(io::Errno::INTR) => continue,
            Err(err) => Err(err).unwrap(),
        }
    }
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
    use rustix::buffer::Buffer;

    fn read<B: Buffer<u8>>(_: B) {}

    // Functions that take `&mut [u8]` can be passed `&mut []`, but with
    // `Buffer` passing `&mut []` gets:
    // "type annotations needed".
    /*
    read(&mut []);
    */

    // The fix: make the element type explicit.
    read(&mut [0_u8; 0]);
}

fn error_array_by_value() {
    use rustix::buffer::Buffer;

    fn read<B: Buffer<u8>>(b: B) -> B::Output {
        unsafe { b.assume_init(0) }
    }

    // This code is erroneously attempts to pass a buffer by value, but it
    // confusingly gets two error messages:
    // "the trait bound `[{integer}; 3]: Buffer<u8>` is not satisfied", and
    // "the trait bound `[{integer}; 3]: buffer::private::Sealed<u8>` is not satisfied".
    /*
    let mut buf = [0, 0, 0];
    read(buf);
    */

    // The fix: pass the buffer by reference.
    let mut buf = [0, 0, 0];
    read(&mut buf);
}
