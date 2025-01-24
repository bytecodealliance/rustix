use rustix::net::netlink::{self, SocketAddrNetlink};
use rustix::net::{
    bind, getsockname, recvfrom, sendto, socket_with, AddressFamily, RecvFlags, SendFlags,
    SocketAddrAny, SocketFlags, SocketType,
};

#[test]
fn encode_decode() {
    let orig = SocketAddrNetlink::new(0x1234_5678, 0x9abc_def0);
    let encoded = SocketAddrAny::from(orig);
    let decoded = SocketAddrNetlink::try_from(encoded).unwrap();
    assert_eq!(decoded, orig);
}

#[test]
fn test_bind_kobject_uevent() {
    let server = socket_with(
        AddressFamily::NETLINK,
        SocketType::RAW,
        SocketFlags::CLOEXEC,
        Some(netlink::KOBJECT_UEVENT),
    )
    .unwrap();

    bind(&server, &SocketAddrNetlink::new(0, 1)).unwrap();
}

#[test]
#[cfg_attr(
    not(any(target_arch = "x86", target_arch = "x86_64")),
    ignore = "qemu used in CI does not support NETLINK_USERSOCK"
)]
fn test_usersock() {
    let server = socket_with(
        AddressFamily::NETLINK,
        SocketType::RAW,
        SocketFlags::CLOEXEC,
        Some(netlink::USERSOCK),
    )
    .unwrap();

    bind(&server, &SocketAddrNetlink::new(0, 0)).unwrap();
    let addr = getsockname(&server).unwrap();
    let addr = SocketAddrNetlink::try_from(addr).unwrap();

    let client = socket_with(
        AddressFamily::NETLINK,
        SocketType::RAW,
        SocketFlags::CLOEXEC,
        Some(netlink::USERSOCK),
    )
    .unwrap();

    let data = b"ABCDEF";

    sendto(client, data, SendFlags::empty(), &addr).unwrap();

    let mut buffer = [0_u8; 4096];
    let (len, actual, src) = recvfrom(&server, &mut buffer, RecvFlags::empty()).unwrap();

    assert_eq!(&buffer[..len], data);
    assert_eq!(len, actual);
    let src = SocketAddrNetlink::try_from(src.unwrap()).unwrap();
    assert_eq!(src.groups(), 0);
}
