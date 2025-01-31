use rustix::buffer::extend;
use rustix::io::read;
use rustix::stdio::stdin;
use std::mem::MaybeUninit;

fn main() {
    let mut buf = vec![0_u8; 3];
    let _x: () = read(stdin(), extend(&mut buf)).unwrap();
    let _x: usize = read(stdin(), &mut buf).unwrap();
    let _x: usize = read(stdin(), &mut *buf).unwrap();
    let _x: usize = read(stdin(), &mut buf[..]).unwrap();
    let _x: usize = read(stdin(), &mut (*buf)[..]).unwrap();

    let mut buf = [0, 0, 0];
    let _x: usize = read(stdin(), &mut buf).unwrap();
    let _x: usize = read(stdin(), &mut buf[..]).unwrap();

    let mut buf = [
        MaybeUninit::uninit(),
        MaybeUninit::uninit(),
        MaybeUninit::uninit(),
    ];
    let _x: (&mut [u8], &mut [MaybeUninit<u8>]) = read(stdin(), &mut buf).unwrap();
    let _x: (&mut [u8], &mut [MaybeUninit<u8>]) = read(stdin(), &mut buf[..]).unwrap();

    // This is reduced from src/fs/inotify.rs line 177.
    struct Wrapper<'a>(&'a mut [u8]);
    impl<'a> Wrapper<'a> {
        fn read(&mut self) {
            // Ideally we'd write this.
            //let _x: usize = read(stdin(), self.0).unwrap();
            // But we need to write this instead.
            let _x: usize = read(stdin(), &mut *self.0).unwrap();
        }
    }
    let mut buf = vec![0_u8; 3];
    let mut wrapper = Wrapper(&mut buf);
    wrapper.read();

    // Why does this get two error messages?
    //let mut buf = [0, 0, 0];
    //let _x = read(stdin(), buf).unwrap();
}
