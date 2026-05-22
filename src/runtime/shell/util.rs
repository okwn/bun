use bun_sys::Fd;

#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum OutKind {
    Stdout,
    Stderr,
}

impl OutKind {
    pub fn to_fd(self) -> Fd {
        match self {
            OutKind::Stdout => Fd::stdout(),
            OutKind::Stderr => Fd::stderr(),
        }
    }
}

pub use crate::api::bun_spawn::stdio::Stdio;

#[cfg(any(target_os = "linux", target_os = "android"))]
pub type WatchFd = core::ffi::c_int; // std.posix.fd_t
#[cfg(not(any(target_os = "linux", target_os = "android")))]
pub type WatchFd = i32;

// ported from: src/shell/util.zig
