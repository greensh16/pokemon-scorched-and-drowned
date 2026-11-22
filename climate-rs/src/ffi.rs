//! The C ABI. Every function the C game calls is declared here and nowhere else.
//! Keep these thin: read inputs, call `logic`, return a primitive.

use crate::logic;

// Functions provided by the C side (pokeemerald-expansion, src/event_data.c).
extern "C" {
    fn VarGet(id: u16) -> u16;
}

/// Returns 42. Phase 0 check that C can call Rust.
#[no_mangle]
pub extern "C" fn Climate_Ping() -> u16 {
    logic::PING
}

/// Reads a script var through C's `VarGet` and returns it plus one.
/// Phase 0 check that Rust can call back into C.
#[no_mangle]
pub extern "C" fn Climate_ReadVarPlusOne(var_id: u16) -> u16 {
    // SAFETY: VarGet takes any u16 and returns a u16; it has no other preconditions.
    let value = unsafe { VarGet(var_id) };
    logic::plus_one(value)
}

/// Host-side stand-in for the C game, so the FFI layer can be tested with `cargo test`.
#[cfg(test)]
mod mock_c {
    /// Mock VarGet: every var holds its own id.
    #[no_mangle]
    extern "C" fn VarGet(id: u16) -> u16 {
        id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ping_round_trip() {
        assert_eq!(Climate_Ping(), 42);
    }

    #[test]
    fn read_var_calls_back_into_c() {
        // Mock VarGet(41) returns 41, so Rust should hand back 42.
        assert_eq!(Climate_ReadVarPlusOne(41), 42);
    }
}
