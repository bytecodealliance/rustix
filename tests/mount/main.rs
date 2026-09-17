//! Tests for [`rustix::mount`].

#![cfg(feature = "mount")]
#![cfg(linux_kernel)]

use core::mem::MaybeUninit;
use linux_raw_sys::general::{__NR_listmount, mnt_id_req, LISTMOUNT_REVERSE, MNT_ID_REQ_SIZE_VER0};
use rustix::io::Errno;
use rustix::mount::{listmount, ListMountFlags, LISTMOUNT_ROOT};

fn raw_listmount_page(
    root_mount_id: u64,
    last_mount_id: u64,
    mount_ids: &mut [u64],
    flags: u32,
) -> Result<usize, i32> {
    let request = mnt_id_req {
        size: MNT_ID_REQ_SIZE_VER0,
        spare: 0,
        mnt_id: root_mount_id,
        param: last_mount_id,
        mnt_ns_id: 0,
    };
    let result = unsafe {
        libc::syscall(
            __NR_listmount as libc::c_long,
            core::ptr::addr_of!(request),
            mount_ids.as_mut_ptr(),
            mount_ids.len() as libc::size_t,
            flags as libc::c_uint,
        )
    };
    if result == -1 {
        Err(libc_errno::errno().0)
    } else {
        Ok(result as usize)
    }
}

fn raw_listmount_all(flags: u32) -> Option<Vec<u64>> {
    let mut mount_ids = Vec::new();
    let mut last_mount_id = 0;

    for _ in 0..4096 {
        let mut page = [0_u64; 64];
        let count = match raw_listmount_page(LISTMOUNT_ROOT, last_mount_id, &mut page, flags) {
            Err(libc::ENOSYS | libc::EPERM) => return None,
            Err(libc::EINVAL) if flags == LISTMOUNT_REVERSE => return None,
            Err(err) => panic!(
                "raw listmount oracle failed: {}",
                std::io::Error::from_raw_os_error(err)
            ),
            Ok(count) => count,
        };
        assert!(count <= page.len());
        if count == 0 {
            return Some(mount_ids);
        }
        assert!(page[..count].iter().all(|mount_id| *mount_id != 0));
        last_mount_id = page[count - 1];
        mount_ids.extend_from_slice(&page[..count]);
    }

    panic!("raw listmount oracle did not terminate");
}

fn first_mount_id() -> Option<u64> {
    let mut mount_ids = [0_u64; 1];
    match listmount(LISTMOUNT_ROOT, 0, &mut mount_ids, ListMountFlags::empty()) {
        // Older kernels and syscall sandboxes may require privilege even for
        // the caller's current mount namespace.
        Err(Errno::NOSYS) | Err(Errno::PERM) => None,
        Ok(1) => {
            assert_ne!(mount_ids[0], 0);
            Some(mount_ids[0])
        }
        Ok(count) => panic!("the current mount namespace returned {count} root mounts"),
        Err(err) => panic!("listmount failed for the current mount namespace: {err}"),
    }
}

#[test]
fn listmount_current_namespace_and_initialized_prefix() {
    for _ in 0..3 {
        let Some(expected) = raw_listmount_all(0) else {
            return;
        };
        assert!(!expected.is_empty());

        let mut exact_mount_ids = vec![MaybeUninit::<u64>::uninit(); expected.len()];
        let (exact_initialized, exact_remainder) = listmount(
            LISTMOUNT_ROOT,
            0,
            &mut exact_mount_ids,
            ListMountFlags::empty(),
        )
        .unwrap();
        let mut slack_mount_ids = vec![MaybeUninit::<u64>::uninit(); expected.len() + 1];
        let (slack_initialized, slack_remainder) = listmount(
            LISTMOUNT_ROOT,
            0,
            &mut slack_mount_ids,
            ListMountFlags::empty(),
        )
        .unwrap();
        let Some(after) = raw_listmount_all(0) else {
            return;
        };
        if after != expected {
            continue;
        }

        assert_eq!(exact_initialized.len(), expected.len());
        assert_eq!(exact_initialized, expected);
        assert!(exact_remainder.is_empty());
        assert_eq!(slack_initialized.len(), expected.len());
        assert_eq!(slack_initialized, expected);
        assert_eq!(slack_remainder.len(), 1);
        return;
    }

    panic!("the current mount namespace did not stabilize for the length oracle");
}

#[test]
fn listmount_reverse_matches_the_raw_kernel_order() {
    for _ in 0..3 {
        let Some(forward) = raw_listmount_all(0) else {
            return;
        };
        if forward.len() < 2 {
            return;
        }
        let Some(reverse) = raw_listmount_all(LISTMOUNT_REVERSE) else {
            return;
        };
        if !reverse.iter().eq(forward.iter().rev()) {
            continue;
        }

        let mut mount_ids = vec![MaybeUninit::<u64>::uninit(); reverse.len() + 1];
        let (initialized, remainder) =
            listmount(LISTMOUNT_ROOT, 0, &mut mount_ids, ListMountFlags::REVERSE).unwrap();
        let Some(after) = raw_listmount_all(LISTMOUNT_REVERSE) else {
            return;
        };
        if after != reverse {
            continue;
        }

        assert_eq!(initialized.len(), reverse.len());
        assert_eq!(initialized, reverse);
        assert_eq!(remainder.len(), 1);
        return;
    }

    panic!("the current mount namespace did not stabilize for the reverse-order oracle");
}

#[test]
fn listmount_cursor_progresses() {
    let Some(first) = first_mount_id() else {
        return;
    };

    let mut next_mount_id = [0_u64; 1];
    let count = listmount(
        LISTMOUNT_ROOT,
        first,
        &mut next_mount_id,
        ListMountFlags::empty(),
    )
    .unwrap();
    assert!(count <= 1);
    if count == 1 {
        assert_ne!(next_mount_id[0], 0);
        assert_ne!(next_mount_id[0], first);
    }
}

#[test]
fn listmount_rejects_invalid_root_and_flags() {
    if first_mount_id().is_none() {
        return;
    }

    let mut mount_ids = [0_u64; 1];
    assert!(listmount(0, 0, &mut mount_ids, ListMountFlags::empty()).is_err());

    let unknown = ListMountFlags::from_bits_retain(1_u32 << 31);
    assert_eq!(
        listmount(LISTMOUNT_ROOT, 0, &mut mount_ids, unknown),
        Err(Errno::INVAL)
    );
}
