//! Packet MMAP.
#![allow(unsafe_code)]

use super::{AddressFamily, Protocol};
use crate::backend::c;
use bitflags::bitflags;
use core::{
    ffi::c_void,
    fmt,
    mem::{align_of, size_of, MaybeUninit},
    slice,
};

/// A type for holding raw integer packet types.
pub type RawPacketType = u8;

/// `PACKET_*` constants.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub struct PacketType(pub(crate) RawPacketType);

#[rustfmt::skip]
impl PacketType {
    /// `PACKET_HOST`
    pub const HOST: Self = Self(c::PACKET_HOST as _);

    /// `PACKET_BROADCAST`
    pub const BROADCAST: Self = Self(c::PACKET_BROADCAST as _);

    /// `PACKET_MULTICAST`
    pub const MULTICAST: Self = Self(c::PACKET_MULTICAST as _);

    /// `PACKET_OTHERHOST`
    pub const OTHERHOST: Self = Self(c::PACKET_OTHERHOST as _);

    /// `PACKET_OUTGOING`
    pub const OUTGOING: Self = Self(c::PACKET_OUTGOING as _);
}

// TODO maybe separate RX and TX flags (and other flags)
bitflags! {
    /// `TP_STATUS_*` constants.
    ///
    /// `TP_STATUS_KERNEL` == 0
    /// `TP_STATUS_AVAILABLE` == 0
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
    pub struct PacketStatus: u32 {
        /// `TP_STATUS_USER`
        const USER = bitcast!(c::TP_STATUS_USER);
        /// `TP_STATUS_COPY`
        const COPY = bitcast!(c::TP_STATUS_COPY);
        /// `TP_STATUS_LOSING`
        const LOSING = bitcast!(c::TP_STATUS_LOSING);
        /// `TP_STATUS_CSUMNOTREADY`
        const CSUMNOTREADY = bitcast!(c::TP_STATUS_CSUMNOTREADY);
        /// `TP_STATUS_VLAN_VALID`
        const VLAN_VALID = bitcast!(c::TP_STATUS_VLAN_VALID);
        /// `TP_STATUS_BLK_TMO`
        const BLK_TMO = bitcast!(c::TP_STATUS_BLK_TMO);
        /// `TP_STATUS_VLAN_TPID_VALID`
        const VLAN_TPID_VALID = bitcast!(c::TP_STATUS_VLAN_TPID_VALID);
        /// `TP_STATUS_CSUM_VALID`
        const CSUM_VALID = bitcast!(c::TP_STATUS_CSUM_VALID);
        /// `TP_STATUS_GSO_TCP`
        const GSO_TCP = bitcast!(c::TP_STATUS_GSO_TCP);

        /// `TP_STATUS_SEND_REQUEST`
        const SEND_REQUEST = bitcast!(c::TP_STATUS_SEND_REQUEST);
        /// `TP_STATUS_SENDING`
        const SENDING = bitcast!(c::TP_STATUS_SENDING);
        /// `TP_STATUS_WRONG_FORMAT`
        const WRONG_FORMAT = bitcast!(c::TP_STATUS_WRONG_FORMAT);

        /// `TP_STATUS_TS_SOFTWARE`
        const TS_SOFTWARE = bitcast!(c::TP_STATUS_TS_SOFTWARE);
        /// `TP_STATUS_TS_SYS_HARDWARE`
        const TS_SYS_HARDWARE = bitcast!(c::TP_STATUS_TS_SYS_HARDWARE);
        /// `TP_STATUS_TS_RAW_HARDWARE`
        const TS_RAW_HARDWARE = bitcast!(c::TP_STATUS_TS_RAW_HARDWARE);

        /// <https://docs.rs/bitflags/*/bitflags/#externally-defined-flags>
        const _ = !0;
    }
}

/// `struct sockaddr_ll`
#[repr(C)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash)]
#[allow(missing_docs)]
pub struct SocketAddrLink {
    pub family: u16,
    pub protocol: u16,
    pub ifindex: i32,
    pub hatype: u16,
    pub pkttype: u8,
    pub halen: u8,
    pub addr: [u8; 8],
}

impl SocketAddrLink {
    /// Constructs a new link-layer socket address.
    pub const fn new(protocol: Protocol, index: u32) -> Self {
        let protocol = protocol.as_raw().get();
        debug_assert!(protocol <= u16::MAX as u32);
        debug_assert!(index <= i32::MAX as u32);
        Self {
            family: AddressFamily::PACKET.as_raw(),
            protocol: protocol as _,
            ifindex: index as _,
            hatype: 0,
            pkttype: 0,
            halen: 0,
            addr: [0; 8],
        }
    }
}

#[rustfmt::skip]
impl fmt::Display for SocketAddrLink {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let [a, b, c, d, e, f, g, h] = self.addr;
        match self.halen {
            0 => write!(fmt, "empty"),
            1 => write!(fmt, "{a:02x}"),
            2 => write!(fmt, "{a:02x}:{b:02x}"),
            3 => write!(fmt, "{a:02x}:{b:02x}:{c:02x}"),
            4 => write!(fmt, "{a:02x}:{b:02x}:{c:02x}:{d:02x}"),
            5 => write!(fmt, "{a:02x}:{b:02x}:{c:02x}:{d:02x}:{e:02x}"),
            6 => write!(fmt, "{a:02x}:{b:02x}:{c:02x}:{d:02x}:{e:02x}:{f:02x}"),
            7 => write!(fmt, "{a:02x}:{b:02x}:{c:02x}:{d:02x}:{e:02x}:{f:02x}:{g:02x}"),
            8 => write!(fmt, "{a:02x}:{b:02x}:{c:02x}:{d:02x}:{e:02x}:{f:02x}:{g:02x}:{h:02x}"),
            _ => unimplemented!("halen > 8"),
        }
    }
}

/// `TPACKET_ALIGN`
pub const fn align(x: usize) -> usize {
    let v = c::TPACKET_ALIGNMENT as usize;
    (x + v - 1) & !(v - 1)
}

/// `TPACKET_HDRLEN`
pub const PACKET_HEADER_LEN: usize =
    align(size_of::<c::tpacket_hdr>()) + size_of::<c::sockaddr_ll>();
/// `TPACKET2_HDRLEN`
pub const PACKET_HEADER2_LEN: usize =
    align(size_of::<c::tpacket2_hdr>()) + size_of::<c::sockaddr_ll>();
/// `TPACKET3_HDRLEN`
pub const PACKET_HEADER3_LEN: usize =
    align(size_of::<c::tpacket3_hdr>()) + size_of::<c::sockaddr_ll>();

/// `struct tpacket_hdr`
#[repr(C)]
#[allow(missing_docs)]
pub struct PacketHeader {
    pub status: u64,
    pub len: u32,
    pub snaplen: u32,
    pub mac: u16,
    pub net: u16,
    pub sec: u32,
    pub usec: u32,
    _private: (),
}

impl PacketHeader {
    // TODO
}

/// `struct tpacket2_hdr`
#[repr(C)]
#[allow(missing_docs)]
pub struct PacketHeader2 {
    pub status: PacketStatus,
    pub len: u32,
    pub snaplen: u32,
    pub mac: u16,
    pub net: u16,
    pub sec: u32,
    pub nsec: u32,
    pub vlan_tci: u16,
    pub vlan_tpid: u16,
    padding: [u8; 4],
    _private: (),
}

impl PacketHeader2 {
    /// TODO
    ///
    /// # Safety
    ///
    /// The pointer must be properly aligned and non-null and point to
    /// a valid `tpacket2_hdr` structure inside a `RX` ring buffer.
    pub unsafe fn from_rx_ptr<'a>(ptr: *mut c_void) -> Option<&'a mut Self> {
        assert_eq!(ptr.align_offset(c::TPACKET_ALIGNMENT as _), 0);
        // TODO or should we return None?
        debug_assert!(!ptr.is_null());
        // First read the status field without creating a reference.
        let status = {
            let ptr = ptr as *const u32;
            PacketStatus::from_bits_truncate(ptr.read())
        };
        if status.contains(PacketStatus::USER) {
            Some(&mut *(ptr as *mut Self))
        } else {
            None
        }
    }

    /// TODO
    ///
    /// # Safety
    ///
    /// The pointer must be properly aligned and non-null and point to
    /// a valid `tpacket2_hdr` structure inside a `TX` ring buffer.
    pub unsafe fn from_tx_ptr<'a>(ptr: *mut c_void) -> Option<&'a mut Self> {
        assert_eq!(ptr.align_offset(c::TPACKET_ALIGNMENT as _), 0);
        // TODO or should we return None?
        debug_assert!(!ptr.is_null());
        // First read the status field without creating a reference.
        let status = {
            let ptr = ptr as *const u32;
            PacketStatus::from_bits_truncate(ptr.read())
        };
        if status.is_empty() {
            // Available
            Some(&mut *(ptr as *mut Self))
        } else {
            None
        }
    }

    /// TODO
    pub fn addr(&self) -> &SocketAddrLink {
        let ptr = self as *const Self as *const u8;
        unsafe {
            let ptr = ptr.add(size_of::<c::tpacket2_hdr>());
            let ptr = ptr.add(ptr.align_offset(c::TPACKET_ALIGNMENT as _));
            &*(ptr as *const SocketAddrLink)
        }
    }

    /// TODO
    pub fn addr_mut(&mut self) -> &mut SocketAddrLink {
        let ptr = self as *mut Self as *mut u8;
        unsafe {
            let ptr = ptr.add(size_of::<c::tpacket2_hdr>());
            let ptr = ptr.add(ptr.align_offset(c::TPACKET_ALIGNMENT as _));
            &mut *(ptr as *mut SocketAddrLink)
        }
    }

    //pub unsafe fn mac(&self) -> Option<&[u8]> {
    //    // FIXME if using DGRAM, the mac header is not present
    //    if self.mac == 0 {
    //        return None;
    //    }
    //    debug_assert!(
    //        self.mac >= (size_of::<c::tpacket2_hdr>() + size_of::<c::sockaddr_ll>()) as _
    //    );

    //    // TODO might want to add other hardware types
    //    let len = match self.addr().hatype {
    //        // TODO use ARPHRD_* constants
    //        1 => c::ETH_HLEN, // Ethernet
    //        _ => unimplemented!("unsupported hardware type"),
    //    };
    //    debug_assert!(len <= (self.net - self.mac).into());

    //    let ptr = self.as_ptr() as *const u8;
    //    unsafe {
    //        // XXX do we just assume that `tp_mac` is valid?
    //        let ptr = ptr.add(self.mac as _);
    //        let ptr = ptr.add(ptr.align_offset(c::TPACKET_ALIGNMENT as _));
    //        Some(slice::from_raw_parts(ptr, len as _))
    //    }
    //}

    /// TODO
    pub fn payload_rx(&self) -> *const u8 {
        let ptr = self as *const Self as *const u8;
        unsafe {
            let ptr = ptr.add(self.mac as _);
            ptr
        }
    }

    /// TODO
    pub fn payload_tx(&mut self) -> *mut u8 {
        let ptr = self as *mut Self as *mut u8;
        unsafe {
            let ptr = ptr.add(size_of::<c::tpacket2_hdr>());
            let ptr = ptr.add(ptr.align_offset(c::TPACKET_ALIGNMENT as _));
            ptr
        }
    }
}

impl fmt::Debug for PacketHeader2 {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("PacketHeader2")
            .field("status", &self.status)
            .field("len", &self.len)
            .field("snaplen", &self.snaplen)
            .field("mac", &self.mac)
            .field("net", &self.net)
            .field("sec", &self.sec)
            .field("nsec", &self.nsec)
            .field("vlan_tci", &self.vlan_tci)
            .field("vlan_tpid", &self.vlan_tpid)
            .finish()
    }
}

/// `struct tpacket3_hdr`
#[repr(C)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[allow(missing_docs)]
pub struct PacketHeader3 {
    pub next_offset: u32,
    pub sec: u32,
    pub nsec: u32,
    pub snaplen: u32,
    pub len: u32,
    pub status: u32,
    pub mac: u16,
    pub net: u16,
    pub inner: PacketHeader3Inner,
    pub padding: [u8; 8],
}

impl PacketHeader3 {
    // TODO
}

/// `struct tpacket_hdr_variant1`
#[repr(C)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[allow(missing_docs)]
pub struct PacketHeader3Inner {
    pub rxhash: u32,
    pub vlan_tci: u32,
    pub vlan_tpid: u16,
    pub padding: u16,
}

/// `struct tpacket_req`
#[repr(C)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[allow(missing_docs)]
pub struct PacketReq {
    pub block_size: u32,
    pub block_nr: u32,
    pub frame_size: u32,
    pub frame_nr: u32,
}

/// `struct tpacket_req3`
#[repr(C)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[allow(missing_docs)]
pub struct PacketReq3 {
    pub block_size: u32,
    pub block_nr: u32,
    pub frame_size: u32,
    pub frame_nr: u32,
    pub retire_blk_tov: u32,
    pub sizeof_priv: u32,
    pub feature_req_word: u32,
}

/// Packet MMAP settings for use with [`set_packet_rx_ring`] and [`set_packet_tx_ring`].
#[repr(C)]
#[allow(missing_docs)]
pub enum PacketReqAny {
    V1(PacketReq),
    V2(PacketReq),
    V3(PacketReq3),
}

/// `struct tpacket_stats`
#[repr(C)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[allow(missing_docs)]
pub struct PacketStats {
    pub packets: u32,
    pub drops: u32,
}

/// `struct tpacket_stats_v3`
#[repr(C)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[allow(missing_docs)]
pub struct PacketStats3 {
    pub packets: u32,
    pub drops: u32,
    pub freeze_q_cnt: u32,
}

/// Packet MMAP stats.
#[repr(C)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[allow(missing_docs)]
pub enum PacketStatsAny {
    V1(PacketStats),
    V2(PacketStats),
    V3(PacketStats3),
}

/// `struct tpacket_auxdata`
#[repr(C)]
#[allow(missing_docs)]
pub struct PacketAuxData {
    pub status: u32,
    pub len: u32,
    pub snaplen: u32,
    pub mac: u16,
    pub net: u16,
    pub vlan_tci: u16,
    pub vlan_tpid: u16,
}

#[test]
fn if_packet_layouts() {
    check_renamed_type!(SocketAddrLink, sockaddr_ll);
    check_renamed_struct_renamed_field!(SocketAddrLink, sockaddr_ll, family, sll_family);
    check_renamed_struct_renamed_field!(SocketAddrLink, sockaddr_ll, protocol, sll_protocol);
    check_renamed_struct_renamed_field!(SocketAddrLink, sockaddr_ll, ifindex, sll_ifindex);
    check_renamed_struct_renamed_field!(SocketAddrLink, sockaddr_ll, hatype, sll_hatype);
    check_renamed_struct_renamed_field!(SocketAddrLink, sockaddr_ll, pkttype, sll_pkttype);
    check_renamed_struct_renamed_field!(SocketAddrLink, sockaddr_ll, halen, sll_halen);
    check_renamed_struct_renamed_field!(SocketAddrLink, sockaddr_ll, addr, __bindgen_anon_1);

    check_renamed_type!(PacketHeader, tpacket_hdr);
    check_renamed_struct_renamed_field!(PacketHeader, tpacket_hdr, status, tp_status);
    check_renamed_struct_renamed_field!(PacketHeader, tpacket_hdr, len, tp_len);
    check_renamed_struct_renamed_field!(PacketHeader, tpacket_hdr, snaplen, tp_snaplen);
    check_renamed_struct_renamed_field!(PacketHeader, tpacket_hdr, mac, tp_mac);
    check_renamed_struct_renamed_field!(PacketHeader, tpacket_hdr, net, tp_net);
    check_renamed_struct_renamed_field!(PacketHeader, tpacket_hdr, sec, tp_sec);
    check_renamed_struct_renamed_field!(PacketHeader, tpacket_hdr, usec, tp_usec);

    check_renamed_type!(PacketHeader2, tpacket2_hdr);
    check_renamed_struct_renamed_field!(PacketHeader2, tpacket2_hdr, status, tp_status);
    check_renamed_struct_renamed_field!(PacketHeader2, tpacket2_hdr, len, tp_len);
    check_renamed_struct_renamed_field!(PacketHeader2, tpacket2_hdr, snaplen, tp_snaplen);
    check_renamed_struct_renamed_field!(PacketHeader2, tpacket2_hdr, mac, tp_mac);
    check_renamed_struct_renamed_field!(PacketHeader2, tpacket2_hdr, net, tp_net);
    check_renamed_struct_renamed_field!(PacketHeader2, tpacket2_hdr, sec, tp_sec);
    check_renamed_struct_renamed_field!(PacketHeader2, tpacket2_hdr, nsec, tp_nsec);
    check_renamed_struct_renamed_field!(PacketHeader2, tpacket2_hdr, vlan_tci, tp_vlan_tci);
    check_renamed_struct_renamed_field!(PacketHeader2, tpacket2_hdr, vlan_tpid, tp_vlan_tpid);
    check_renamed_struct_renamed_field!(PacketHeader2, tpacket2_hdr, padding, tp_padding);

    check_renamed_type!(PacketHeader3, tpacket3_hdr);
    check_renamed_struct_renamed_field!(PacketHeader3, tpacket3_hdr, next_offset, tp_next_offset);
    check_renamed_struct_renamed_field!(PacketHeader3, tpacket3_hdr, sec, tp_sec);
    check_renamed_struct_renamed_field!(PacketHeader3, tpacket3_hdr, nsec, tp_nsec);
    check_renamed_struct_renamed_field!(PacketHeader3, tpacket3_hdr, snaplen, tp_snaplen);
    check_renamed_struct_renamed_field!(PacketHeader3, tpacket3_hdr, len, tp_len);
    check_renamed_struct_renamed_field!(PacketHeader3, tpacket3_hdr, status, tp_status);
    check_renamed_struct_renamed_field!(PacketHeader3, tpacket3_hdr, mac, tp_mac);
    check_renamed_struct_renamed_field!(PacketHeader3, tpacket3_hdr, net, tp_net);
    check_renamed_struct_renamed_field!(PacketHeader3, tpacket3_hdr, inner, __bindgen_anon_1);
    check_renamed_struct_renamed_field!(PacketHeader3, tpacket3_hdr, padding, tp_padding);

    check_renamed_type!(PacketHeader3Inner, tpacket_hdr_variant1);
    check_renamed_struct_renamed_field!(
        PacketHeader3Inner,
        tpacket_hdr_variant1,
        rxhash,
        tp_rxhash
    );
    check_renamed_struct_renamed_field!(
        PacketHeader3Inner,
        tpacket_hdr_variant1,
        vlan_tci,
        tp_vlan_tci
    );
    check_renamed_struct_renamed_field!(
        PacketHeader3Inner,
        tpacket_hdr_variant1,
        vlan_tpid,
        tp_vlan_tpid
    );
    check_renamed_struct_renamed_field!(
        PacketHeader3Inner,
        tpacket_hdr_variant1,
        padding,
        tp_padding
    );

    check_renamed_type!(PacketReq, tpacket_req);
    check_renamed_struct_renamed_field!(PacketReq, tpacket_req, block_size, tp_block_size);
    check_renamed_struct_renamed_field!(PacketReq, tpacket_req, block_nr, tp_block_nr);
    check_renamed_struct_renamed_field!(PacketReq, tpacket_req, frame_size, tp_frame_size);
    check_renamed_struct_renamed_field!(PacketReq, tpacket_req, frame_nr, tp_frame_nr);

    check_renamed_type!(PacketReq3, tpacket_req3);
    check_renamed_struct_renamed_field!(PacketReq3, tpacket_req3, block_size, tp_block_size);
    check_renamed_struct_renamed_field!(PacketReq3, tpacket_req3, block_nr, tp_block_nr);
    check_renamed_struct_renamed_field!(PacketReq3, tpacket_req3, frame_size, tp_frame_size);
    check_renamed_struct_renamed_field!(PacketReq3, tpacket_req3, frame_nr, tp_frame_nr);
    check_renamed_struct_renamed_field!(
        PacketReq3,
        tpacket_req3,
        retire_blk_tov,
        tp_retire_blk_tov
    );
    check_renamed_struct_renamed_field!(PacketReq3, tpacket_req3, sizeof_priv, tp_sizeof_priv);
    check_renamed_struct_renamed_field!(
        PacketReq3,
        tpacket_req3,
        feature_req_word,
        tp_feature_req_word
    );

    check_renamed_type!(PacketStats, tpacket_stats);
    check_renamed_struct_renamed_field!(PacketStats, tpacket_stats, packets, tp_packets);
    check_renamed_struct_renamed_field!(PacketStats, tpacket_stats, drops, tp_drops);

    check_renamed_type!(PacketStats3, tpacket_stats_v3);
    check_renamed_struct_renamed_field!(PacketStats3, tpacket_stats_v3, packets, tp_packets);
    check_renamed_struct_renamed_field!(PacketStats3, tpacket_stats_v3, drops, tp_drops);
    check_renamed_struct_renamed_field!(
        PacketStats3,
        tpacket_stats_v3,
        freeze_q_cnt,
        tp_freeze_q_cnt
    );

    check_renamed_type!(PacketAuxData, tpacket_auxdata);
    check_renamed_struct_renamed_field!(PacketAuxData, tpacket_auxdata, status, tp_status);
    check_renamed_struct_renamed_field!(PacketAuxData, tpacket_auxdata, len, tp_len);
    check_renamed_struct_renamed_field!(PacketAuxData, tpacket_auxdata, snaplen, tp_snaplen);
    check_renamed_struct_renamed_field!(PacketAuxData, tpacket_auxdata, mac, tp_mac);
    check_renamed_struct_renamed_field!(PacketAuxData, tpacket_auxdata, net, tp_net);
    check_renamed_struct_renamed_field!(PacketAuxData, tpacket_auxdata, vlan_tci, tp_vlan_tci);
    check_renamed_struct_renamed_field!(PacketAuxData, tpacket_auxdata, vlan_tpid, tp_vlan_tpid);
}
