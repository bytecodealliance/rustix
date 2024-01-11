#[cfg(all(feature = "mm", feature = "fs"))]
#[test]
fn test_mbind() {
    let size = 8192;

    unsafe {
        let vaddr = rustix::mm::mmap_anonymous(
            std::ptr::null_mut(),
            size,
            rustix::mm::ProtFlags::READ | rustix::mm::ProtFlags::WRITE,
            rustix::mm::MapFlags::PRIVATE,
        )
        .unwrap();

        vaddr.cast::<usize>().write(100);

        let mask = &[1];
        rustix::numa::mbind(
            vaddr,
            size,
            rustix::numa::Mode::BIND | rustix::numa::Mode::STATIC_NODES,
            mask,
            rustix::numa::ModeFlags::empty(),
        )
        .unwrap();

        rustix::numa::get_mempolicy_node(vaddr).unwrap();

        match rustix::numa::get_mempolicy_next_node() {
            Err(rustix::io::Errno::INVAL) => (),
            _ => panic!(
                "rustix::numa::get_mempolicy_next_node() should return EINVAL for MPOL_DEFAULT"
            ),
        }

        rustix::numa::set_mempolicy(rustix::numa::Mode::INTERLEAVE, mask).unwrap();

        rustix::numa::get_mempolicy_next_node().unwrap();
    }
}
