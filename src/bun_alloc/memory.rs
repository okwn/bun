//! Basic utilities for working with memory and objects.

use crate::AllocError;

#[inline]
pub fn create<T>(value: T) -> Result<Box<T>, AllocError> {
    // PERF(port): Zig `allocator.create` is fallible; Rust `Box::new` aborts on OOM.
    // If fallible allocation is required, swap to `Box::try_new` (nightly
    // `allocator_api`) or a manual `alloc::alloc` + `ptr::write` pair.
    Ok(Box::new(value))
}

#[inline]
pub fn destroy<T>(ptr: Box<T>) {
    drop(ptr);
}

#[inline]
pub fn init_default<T: Default>() -> T {
    T::default()
}

/// Rebase a slice from one memory buffer to another buffer.
///
/// Given a slice which points into a memory buffer with base `old_base`, return a
/// slice which points to the same offset in a new memory buffer with base `new_base`,
/// preserving the length of the slice.
///
/// ```text
/// const old_base = [6]u8{};
/// assert(@ptrToInt(&old_base) == 0x32);
///
///            0x32 0x33 0x34 0x35 0x36 0x37
/// old_base |????|????|????|????|????|????|
///                    ^
///                    |<-- slice --->|
///
/// const new_base = [6]u8{};
/// assert(@ptrToInt(&new_base) == 0x74);
/// const output = rebaseSlice(slice, old_base, new_base)
///
///            0x74 0x75 0x76 0x77 0x78 0x79
/// new_base |????|????|????|????|????|????|
///                    ^
///                    |<-- output -->|
/// ```
///
/// # Safety
/// - `slice` must point into the allocation starting at `old_base`.
/// - `new_base` must point to a valid allocation of at least
///   `(slice.as_ptr() - old_base) + slice.len()` bytes.
/// - The returned lifetime `'a` must not outlive the allocation at `new_base`.
pub unsafe fn rebase_slice<'a>(slice: &[u8], old_base: *const u8, new_base: *const u8) -> &'a [u8] {
    let offset = (slice.as_ptr() as usize) - (old_base as usize);
    // SAFETY: caller contract above guarantees `new_base.add(offset)` is in-bounds for
    // `slice.len()` bytes.
    unsafe { core::slice::from_raw_parts(new_base.add(offset), slice.len()) }
}

pub fn drop_sentinel(mut buf: Vec<u8>) -> Result<Box<[u8]>, AllocError> {
    debug_assert_eq!(buf.last().copied(), Some(0), "buffer is not NUL-terminated");
    buf.pop();
    Ok(buf.into_boxed_slice())
}

// ported from: src/bun_alloc/memory.zig
