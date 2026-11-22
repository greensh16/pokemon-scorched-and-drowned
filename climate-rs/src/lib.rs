//! Climate systems for Pokémon: Scorched and Drowned.
//!
//! Design rules (see the project plan, "Repo layout, build and FFI contract"):
//! - C owns all state. This crate holds no statics with state, so it adds
//!   nothing to EWRAM/IWRAM and nothing to the save file.
//! - Only primitives cross the FFI boundary (u8/u16/u32/bool).
//! - Game logic lives in pure functions in `logic`, tested on the host.
//! - `ffi` is a thin layer of `extern "C"` wrappers around `logic`.

// no_std on the GBA (target_os = "none"); normal std on the host for `cargo test`.
#![cfg_attr(target_os = "none", no_std)]

pub mod ffi;
pub mod logic;
pub mod variants;
pub mod warming;

/// On the GBA a panic hangs in place, so mGBA's debugger shows where it stopped.
/// Release builds use panic = "abort", so there is no unwinding.
#[cfg(target_os = "none")]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
