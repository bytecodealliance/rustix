use rustix::io::read;
use rustix::stdio::stdin;

fn main() {
    let buf = Vec::new();
    let _x: Vec<u8> = read(stdin(), buf).unwrap();

    let mut buf = Vec::new();
    let _x: usize = read(stdin(), &mut buf).unwrap();

    let mut buf = [0, 0, 0];
    let _x: usize = read(stdin(), &mut buf).unwrap();

    // Why doesn't this work? This is reduced from src/fs/inotify.rs line 177.
    struct Wrapper<'a>(&'a mut [u8]);
    impl<'a> Wrapper<'a> {
        fn read(&mut self) {
            let _x: usize = read(stdin(), self.0).unwrap();
        }
    }
    let mut buf = Vec::new();
    let mut wrapper = Wrapper(&mut buf);
    wrapper.read();

    // Why does this get two error messages?
    let mut buf = [0, 0, 0];
    let _x = read(stdin(), buf).unwrap();
}
