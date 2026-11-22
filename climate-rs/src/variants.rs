//! Map variants: which version of a map to load at the current warming level.
//!
//! Each climate-affected map has up to three layouts, all drawn in Porymap:
//! tier 0 (normal), tier 1 (warm) and tier 2 (extreme). Rust only decides the tier;
//! the map's on-transition script turns the tier into a `setmaplayoutindex`.
//!
//! Maps are identified by a small climate map id (see `ClimateMap`), not by
//! pokeemerald's map group/number, so this crate doesn't depend on generated C
//! constants. `src/climate_glue.c` translates the current map into a climate map id.

use crate::warming::MAX_WARMING;

/// Climate-affected maps. Values are shared with C via `include/climate.h`;
/// append new maps at the end and never renumber.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClimateMap {
    None = 0,
    /// Route 104's southern beach, below Mr. Briney's cottage. Sea-level rise.
    Route104Beach = 1,
}

impl ClimateMap {
    pub fn from_id(id: u8) -> ClimateMap {
        match id {
            1 => ClimateMap::Route104Beach,
            _ => ClimateMap::None,
        }
    }
}

/// Never reached: use for maps that have no tier-2 layout yet.
const NEVER: u8 = MAX_WARMING + 1;

/// Warming level at which each tier starts, for one map.
struct Thresholds {
    warm_at: u8,
    extreme_at: u8,
}

fn thresholds(map: ClimateMap) -> Option<Thresholds> {
    match map {
        ClimateMap::None => None,
        // Flooded from warming 3 (Act 4 with no mitigation). No extreme layout yet.
        ClimateMap::Route104Beach => Some(Thresholds { warm_at: 3, extreme_at: NEVER }),
    }
}

/// Tier for `map` at `warming`: 0 = normal, 1 = warm, 2 = extreme.
/// Unknown maps always get tier 0, so the default layout is kept.
pub fn map_variant(map: ClimateMap, warming: u16) -> u8 {
    let Some(t) = thresholds(map) else { return 0 };
    let warming = warming.min(MAX_WARMING as u16) as u8;
    if warming >= t.extreme_at {
        2
    } else if warming >= t.warm_at {
        1
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unknown_maps_keep_default_layout() {
        for w in 0..=5 {
            assert_eq!(map_variant(ClimateMap::None, w), 0);
            assert_eq!(map_variant(ClimateMap::from_id(200), w), 0);
        }
    }

    #[test]
    fn route104_floods_from_warming_3() {
        let tiers: Vec<u8> = (0..=5).map(|w| map_variant(ClimateMap::Route104Beach, w)).collect();
        assert_eq!(tiers, vec![0, 0, 0, 1, 1, 1]);
    }

    #[test]
    fn high_warming_is_clamped() {
        assert_eq!(map_variant(ClimateMap::Route104Beach, u16::MAX), 1);
    }

    #[test]
    fn ids_round_trip() {
        assert_eq!(ClimateMap::from_id(ClimateMap::Route104Beach as u8), ClimateMap::Route104Beach);
        assert_eq!(ClimateMap::from_id(0), ClimateMap::None);
    }

    #[test]
    fn tiers_never_go_down_as_warming_rises() {
        for id in 0..=1u8 {
            let map = ClimateMap::from_id(id);
            let mut last = 0;
            for w in 0..=5 {
                let tier = map_variant(map, w);
                assert!(tier >= last);
                last = tier;
            }
        }
    }
}
