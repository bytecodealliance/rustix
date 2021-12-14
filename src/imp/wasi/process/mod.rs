pub(crate) mod types;
pub(crate) mod syscalls;

#[inline]
pub(crate) fn page_size() -> usize {
    // WebAssembly pages are always this size.
    65536
}
