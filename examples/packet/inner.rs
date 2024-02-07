use rustix::event::{poll, PollFd, PollFlags};
use rustix::fd::OwnedFd;
use rustix::mm::{mmap, munmap, MapFlags, ProtFlags};
use rustix::net::{
    bind_link, eth,
    netdevice::name_to_index,
    packet::{PacketHeader2, PacketReq, PacketReqAny, PacketStatus, SocketAddrLink},
    send, socket_with,
    sockopt::{set_packet_rx_ring, set_packet_tx_ring, set_packet_version, PacketVersion},
    AddressFamily, SendFlags, SocketFlags, SocketType,
};
use std::{cell::Cell, collections::VecDeque, env, ffi::c_void, io, ptr, slice, str};

#[derive(Debug)]
pub struct Socket {
    fd: OwnedFd,
    block_size: usize,
    block_count: usize,
    frame_size: usize,
    frame_count: usize,
    rx: Cell<*mut c_void>,
    tx: Cell<*mut c_void>,
}

impl Socket {
    fn new(
        name: &str,
        block_size: usize,
        block_count: usize,
        frame_size: usize,
    ) -> io::Result<Self> {
        let family = AddressFamily::PACKET;
        let type_ = SocketType::RAW;
        let flags = SocketFlags::empty();
        let fd = socket_with(family, type_, flags, None)?;

        let index = name_to_index(&fd, name)?;

        set_packet_version(&fd, PacketVersion::V2)?;

        let frame_count = (block_size * block_count) / frame_size;
        let req = PacketReq {
            block_size: block_size as u32,
            block_nr: block_count as u32,
            frame_size: frame_size as u32,
            frame_nr: frame_count as u32,
        };

        let req = PacketReqAny::V2(req);
        set_packet_rx_ring(&fd, &req)?;
        set_packet_tx_ring(&fd, &req)?;

        let addr = SocketAddrLink::new(eth::ALL, index);
        bind_link(&fd, &addr)?;

        let rx = unsafe {
            mmap(
                ptr::null_mut(),
                block_size * block_count * 2,
                ProtFlags::READ | ProtFlags::WRITE,
                MapFlags::SHARED,
                &fd,
                0,
            )
        }?;
        let tx = unsafe { rx.add(block_size * block_count) };

        Ok(Self {
            fd,
            block_size,
            block_count,
            frame_size,
            frame_count,
            rx: Cell::new(rx),
            tx: Cell::new(tx),
        })
    }

    /// Returns a reader object for receiving packets.
    pub fn reader(&self) -> Reader<'_> {
        assert!(!self.rx.get().is_null());
        Reader {
            socket: self,
            // Take ring pointer.
            ring: self.rx.replace(ptr::null_mut()),
        }
    }

    /// Returns a writer object for transmitting packets.
    pub fn writer(&self) -> Writer<'_> {
        assert!(!self.tx.get().is_null());
        Writer {
            socket: self,
            // Take ring pointer.
            ring: self.tx.replace(ptr::null_mut()),
        }
    }

    /// Flushes the transmit buffer.
    pub fn flush(&self) -> io::Result<()> {
        send(&self.fd, &[], SendFlags::empty())?;
        Ok(())
    }
}

impl Drop for Socket {
    fn drop(&mut self) {
        debug_assert!(!self.rx.get().is_null());
        debug_assert!(!self.tx.get().is_null());
        unsafe {
            let _ = munmap(self.rx.get(), self.block_size * self.block_count * 2);
        }
    }
}

/// TODO
#[derive(Debug)]
pub struct Packet<'r> {
    header: &'r mut PacketHeader2,
}

impl<'r> Packet<'r> {
    pub fn payload(&self) -> &[u8] {
        let ptr = self.header.payload_rx();
        let len = self.header.len as usize;
        unsafe { slice::from_raw_parts(ptr, len) }
    }
}

impl<'r> Drop for Packet<'r> {
    fn drop(&mut self) {
        self.header.status = PacketStatus::empty();
    }
}

/// TODO
#[derive(Debug)]
pub struct Slot<'w> {
    header: &'w mut PacketHeader2,
}

impl<'w> Slot<'w> {
    pub fn write(&mut self, payload: &[u8]) {
        let ptr = self.header.payload_tx();
        // TODO verify length
        let len = payload.len();
        unsafe {
            ptr.copy_from_nonoverlapping(payload.as_ptr(), len);
            self.header.len = len as u32;
        }
    }
}

impl<'w> Drop for Slot<'w> {
    fn drop(&mut self) {
        self.header.status = PacketStatus::SEND_REQUEST;
    }
}

/// A reader object for receiving packets.
#[derive(Debug)]
pub struct Reader<'s> {
    socket: &'s Socket,
    ring: *mut c_void, // Owned
}

impl<'s> Reader<'s> {
    /// Returns an iterator over received packets.
    /// The iterator blocks until at least one packet is received.
    ///
    /// # Lifetimes
    ///
    /// - `'s`: The lifetime of the socket.
    /// - `'r`: The lifetime of the received packets.
    pub fn wait<'r>(&'r mut self) -> io::Result<ReadIter<'s, 'r>>
    where
        's: 'r,
    {
        let flags = PollFlags::IN | PollFlags::RDNORM | PollFlags::ERR;
        let pfd = PollFd::new(&self.socket.fd, flags);
        let pfd = &mut [pfd];
        let n = poll(pfd, -1)?;
        assert_eq!(n, 1);
        Ok(ReadIter {
            reader: self,
            index: 0,
        })
    }
}

impl<'s> Drop for Reader<'s> {
    fn drop(&mut self) {
        // Give back ring pointer.
        self.socket.rx.set(self.ring);
    }
}

/// A writer object for transmitting packets.
#[derive(Debug)]
pub struct Writer<'s> {
    socket: &'s Socket,
    ring: *mut c_void, // Owned
}

impl<'s> Writer<'s> {
    /// Returns an iterator over available slots for transmitting packets.
    /// The iterator blocks until at least one slot is available.
    ///
    /// # Lifetimes
    ///
    /// - `'s`: The lifetime of the socket.
    /// - `'w`: The lifetime of the slots.
    pub fn wait<'w>(&'w mut self) -> io::Result<WriteIter<'s, 'w>>
    where
        's: 'w,
    {
        let flags = PollFlags::OUT | PollFlags::WRNORM | PollFlags::ERR;
        let pfd = PollFd::new(&self.socket.fd, flags);
        let pfd = &mut [pfd];
        let n = poll(pfd, -1)?;
        assert_eq!(n, 1);
        Ok(WriteIter {
            writer: self,
            index: 0,
        })
    }
}

impl<'s> Drop for Writer<'s> {
    fn drop(&mut self) {
        // Give back ring pointer.
        self.socket.tx.set(self.ring);
    }
}

/// An iterator over received packets.
#[derive(Debug)]
pub struct ReadIter<'s, 'r> {
    reader: &'r mut Reader<'s>,
    index: usize,
}

impl<'s, 'r> Iterator for ReadIter<'s, 'r> {
    type Item = Packet<'r>;

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < self.reader.socket.frame_count {
            let base = unsafe {
                self.reader
                    .ring
                    .add(self.index * self.reader.socket.frame_size)
            };
            self.index += 1;

            if let Some(header) = unsafe { PacketHeader2::from_rx_ptr(base) } {
                return Some(Packet { header });
            }
        }
        None
    }
}

/// An iterator over available slots for transmitting packets.
#[derive(Debug)]
pub struct WriteIter<'s, 'w> {
    writer: &'w mut Writer<'s>,
    index: usize,
}

impl<'s, 'w> Iterator for WriteIter<'s, 'w> {
    type Item = Slot<'w>;

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < self.writer.socket.frame_count {
            let base = unsafe {
                self.writer
                    .ring
                    .add(self.index * self.writer.socket.frame_size)
            };
            self.index += 1;

            if let Some(header) = unsafe { PacketHeader2::from_tx_ptr(base) } {
                return Some(Slot { header });
            }
        }
        None
    }
}

// ECHO server
fn server(socket: Socket, mut count: usize) -> io::Result<()> {
    let mut reader = socket.reader();
    let mut writer = socket.writer();

    while count > 0 {
        let mut queue = VecDeque::new();

        for packet in reader.wait()? {
            queue.push_back(packet);
        }

        while let Some(packet) = queue.pop_front() {
            let mut iter = writer.wait()?.take(count);
            while let Some(mut slot) = iter.next() {
                let mut payload = packet.payload().to_vec();
                assert_eq!(payload[12..14], [0x08, 0x00]);
                payload.swap(14, 15);

                slot.write(&payload);
                drop(slot);
                count -= 1;
            }
            drop(packet);
        }

        socket.flush()?;
    }

    Ok(())
}

// ECHO client
fn client(socket: Socket, mut count: usize) -> io::Result<()> {
    let mut reader = socket.reader();
    let mut writer = socket.writer();

    while count > 0 {
        let mut iter = writer.wait()?.take(count);
        while let Some(mut slot) = iter.next() {
            let payload = &[
                0xff, 0xff, 0xff, 0xff, 0xff, 0xff, // Destination
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Source
                0x08, 0x00, // Type (IPv4, but not really)
                0x13, 0x37, // Payload (some value)
            ];

            slot.write(payload);
            drop(slot);
            count -= 1;
        }

        socket.flush()?;

        for packet in reader.wait()? {
            assert_eq!(packet.payload()[14..16], [0x37, 0x13]);
        }
    }

    Ok(())
}

pub fn main() -> io::Result<()> {
    let mut args = env::args().skip(1);
    let name = args.next().expect("name");
    let mode = args.next().expect("mode");
    let count = args.next().expect("count");

    let socket = Socket::new(&name, 4096, 4, 2048)?;
    let count = count.parse().unwrap();

    match mode.as_str() {
        "server" => server(socket, count),
        "client" => client(socket, count),
        _ => panic!("invalid mode"),
    }
}
