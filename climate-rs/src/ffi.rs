//! The C ABI. Every function the C game calls is declared here and nowhere else.
//! Keep these thin: read inputs, call `logic`, return a primitive.

use crate::logic;
use crate::variants::{self, ClimateMap};
use crate::warming;

/// Climate map ids, shared with C (`include/climate.h`). Must match `variants::ClimateMap`.
pub const CLIMATE_MAP_NONE: u8 = ClimateMap::None as u8;
pub const CLIMATE_MAP_ROUTE104_BEACH: u8 = ClimateMap::Route104Beach as u8;

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

/// New warming level (0–5) for `act`, from the saved pressure, mitigation and previous level.
/// Called at act boundaries; the caller stores the result in VAR_CLIMATE_WARMING.
#[no_mangle]
pub extern "C" fn Climate_ComputeWarming(act: u16, pressure: u16, mitigation: u16, previous: u16) -> u8 {
    warming::compute_warming(act, pressure, mitigation, previous)
}

/// Layout tier for a climate map at a warming level: 0 normal, 1 warm, 2 extreme.
/// Unknown map ids return 0 (keep the default layout).
#[no_mangle]
pub extern "C" fn Climate_MapVariant(climate_map: u8, warming: u16) -> u8 {
    variants::map_variant(ClimateMap::from_id(climate_map), warming)
}

/// Ending for a final warming level: 0 Adaptation, 1 Collapse, 2 Overshoot.
#[no_mangle]
pub extern "C" fn Climate_Ending(warming: u16) -> u8 {
    warming::ending(warming) as u8
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

    #[test]
    fn compute_warming_via_ffi() {
        assert_eq!(Climate_ComputeWarming(5, 0, 0, 0), 4);
        assert_eq!(Climate_ComputeWarming(5, 0, 255, 0), 2);
    }

    #[test]
    fn map_variant_via_ffi() {
        assert_eq!(Climate_MapVariant(CLIMATE_MAP_ROUTE104_BEACH, 2), 0);
        assert_eq!(Climate_MapVariant(CLIMATE_MAP_ROUTE104_BEACH, 3), 1);
        assert_eq!(Climate_MapVariant(CLIMATE_MAP_NONE, 5), 0);
        assert_eq!(Climate_MapVariant(250, 5), 0);
    }

    #[test]
    fn ending_via_ffi() {
        assert_eq!(Climate_Ending(3), 0);
        assert_eq!(Climate_Ending(4), 1);
        assert_eq!(Climate_Ending(5), 2);
    }

    #[test]
    fn climate_map_ids_are_stable() {
        // C hard-codes these in include/climate.h. Renumbering breaks the ROM.
        assert_eq!(CLIMATE_MAP_NONE, 0);
        assert_eq!(CLIMATE_MAP_ROUTE104_BEACH, 1);
    }
}
