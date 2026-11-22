//! Pure game logic. No FFI, no unsafe, no hardware: everything here is
//! tested on the host with `cargo test`.

/// Fixed value returned by `Climate_Ping`. Proves C can call Rust.
pub const PING: u16 = 42;

/// Adds one without wrapping. Used by the Phase 0 round-trip test.
pub fn plus_one(value: u16) -> u16 {
    value.saturating_add(1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ping_is_42() {
        assert_eq!(PING, 42);
    }

    #[test]
    fn plus_one_adds() {
        assert_eq!(plus_one(41), 42);
    }

    #[test]
    fn plus_one_saturates() {
        assert_eq!(plus_one(u16::MAX), u16::MAX);
    }
}
