//! Range shifts: wild Pokémon move as Hoenn warms.
//!
//! When a wild encounter is generated, C passes the species, the map's climate zone,
//! the warming level and a random number. Rust returns the species to actually create:
//! the same one, or a replacement that fits the changed habitat. Replacement keeps
//! the slot's level, so encounter balance stays close to the original tables.
//!
//! Every rule is grounded in a real climate-driven change:
//! - drying ponds and spreading dunes (Lotad → Trapinch)
//! - heat-stressed caterpillars giving way to heat-loving cicadas (Wurmple → Nincada)
//! - sensitive species replaced by generalists (Ralts → Poochyena)
//! - invasive crayfish taking over warm fresh water (Marill, Goldeen → Corphish)
//! - coastal species moving inland as the sea rises (Taillow, Poochyena → Wingull)
//! - jellyfish blooms in warmer seas (Magikarp, Wailmer → Tentacool)

use crate::species::*;
use crate::warming::MAX_WARMING;

/// Climate zones. Values are shared with C (`include/climate.h`); append only.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClimateZone {
    /// No climate rules: encounters are never changed.
    None = 0,
    /// Inland grassland and ponds: Routes 101 and 102.
    Grassland = 1,
    /// Coast and shallow sea: Routes 103 and 104.
    Coast = 2,
}

impl ClimateZone {
    pub fn from_id(id: u8) -> ClimateZone {
        match id {
            1 => ClimateZone::Grassland,
            2 => ClimateZone::Coast,
            _ => ClimateZone::None,
        }
    }

    const fn bit(self) -> u8 {
        1 << (self as u8)
    }
}

const GRASSLAND: u8 = ClimateZone::Grassland.bit();
const COAST: u8 = ClimateZone::Coast.bit();
const ANY_ZONE: u8 = GRASSLAND | COAST;

/// One range-shift rule: from `min_warming`, `chance` percent of `from` encounters in
/// `zones` become `to`. For the same species and zone, the rule with the highest
/// `min_warming` that applies wins, so later rows make the shift stronger.
struct Rule {
    from: u16,
    to: u16,
    zones: u8,
    min_warming: u8,
    chance: u8,
}

const fn rule(from: u16, to: u16, zones: u8, min_warming: u8, chance: u8) -> Rule {
    Rule { from, to, zones, min_warming, chance }
}

#[rustfmt::skip]
const RULES: &[Rule] = &[
    // Ponds dry out and the grassland turns sandy.
    rule(LOTAD,     TRAPINCH,  GRASSLAND, 2, 50),
    rule(LOTAD,     TRAPINCH,  GRASSLAND, 3, 100),
    rule(ZIGZAGOON, TRAPINCH,  GRASSLAND, 4, 25),
    // Heat stress: caterpillars decline, cicadas thrive.
    rule(WURMPLE,   NINCADA,   ANY_ZONE,  2, 30),
    rule(WURMPLE,   NINCADA,   ANY_ZONE,  4, 70),
    // Sensitive species vanish; generalists fill the gap.
    rule(RALTS,     POOCHYENA, GRASSLAND, 3, 100),
    // Invasive crayfish take over warming fresh water.
    rule(MARILL,    CORPHISH,  ANY_ZONE,  2, 50),
    rule(MARILL,    CORPHISH,  ANY_ZONE,  4, 100),
    rule(GOLDEEN,   CORPHISH,  ANY_ZONE,  3, 50),
    // Rising seas push coastal birds inland.
    rule(TAILLOW,   WINGULL,   COAST,     3, 50),
    rule(POOCHYENA, WINGULL,   COAST,     3, 30),
    // Jellyfish blooms in warmer water; whales shift their range away.
    rule(MAGIKARP,  TENTACOOL, COAST,     2, 30),
    rule(MAGIKARP,  TENTACOOL, COAST,     4, 60),
    rule(WAILMER,   TENTACOOL, COAST,     3, 50),
];

/// Species to create for a wild encounter of `species` in `zone` at `warming`.
/// `roll` is any random number (C passes `Random()`); only `roll % 100` is used.
pub fn shift_encounter(species: u16, zone: ClimateZone, warming: u16, roll: u16) -> u16 {
    if zone == ClimateZone::None {
        return species;
    }
    let warming = warming.min(MAX_WARMING as u16) as u8;
    let best = RULES
        .iter()
        .filter(|r| r.from == species && r.zones & zone.bit() != 0 && warming >= r.min_warming)
        .max_by_key(|r| r.min_warming);
    match best {
        Some(r) if (roll % 100) < r.chance as u16 => r.to,
        _ => species,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Share of 100 evenly spread rolls that get replaced.
    fn replaced_pct(species: u16, zone: ClimateZone, warming: u16) -> usize {
        (0..100u16).filter(|&roll| shift_encounter(species, zone, warming, roll) != species).count()
    }

    #[test]
    fn species_numbers_come_from_the_game() {
        assert_eq!(POOCHYENA, 261);
        assert_eq!(LOTAD, 270);
        assert_eq!(CORPHISH, 341);
    }

    #[test]
    fn nothing_changes_at_warming_0_or_1() {
        for r in RULES {
            for zone in [ClimateZone::Grassland, ClimateZone::Coast] {
                for w in 0..=1 {
                    assert_eq!(replaced_pct(r.from, zone, w), 0);
                }
            }
        }
    }

    #[test]
    fn no_zone_means_no_change() {
        for w in 0..=5 {
            assert_eq!(replaced_pct(LOTAD, ClimateZone::None, w), 0);
            assert_eq!(replaced_pct(MAGIKARP, ClimateZone::None, w), 0);
        }
    }

    #[test]
    fn lotad_disappears_as_ponds_dry() {
        assert_eq!(replaced_pct(LOTAD, ClimateZone::Grassland, 1), 0);
        assert_eq!(replaced_pct(LOTAD, ClimateZone::Grassland, 2), 50);
        assert_eq!(replaced_pct(LOTAD, ClimateZone::Grassland, 3), 100);
        assert_eq!(shift_encounter(LOTAD, ClimateZone::Grassland, 3, 0), TRAPINCH);
    }

    #[test]
    fn stronger_rule_wins_at_higher_warming() {
        assert_eq!(replaced_pct(WURMPLE, ClimateZone::Grassland, 2), 30);
        assert_eq!(replaced_pct(WURMPLE, ClimateZone::Grassland, 3), 30);
        assert_eq!(replaced_pct(WURMPLE, ClimateZone::Grassland, 4), 70);
        assert_eq!(replaced_pct(MARILL, ClimateZone::Coast, 5), 100);
    }

    #[test]
    fn coastal_rules_stay_on_the_coast() {
        assert_eq!(replaced_pct(POOCHYENA, ClimateZone::Grassland, 5), 0);
        assert_eq!(replaced_pct(POOCHYENA, ClimateZone::Coast, 3), 30);
        assert_eq!(replaced_pct(MAGIKARP, ClimateZone::Grassland, 5), 0);
    }

    #[test]
    fn unlisted_species_never_change() {
        for w in 0..=5 {
            assert_eq!(replaced_pct(SEEDOT, ClimateZone::Grassland, w), 0);
            assert_eq!(replaced_pct(WINGULL, ClimateZone::Coast, w), 0);
        }
    }

    #[test]
    fn warming_above_max_is_clamped() {
        assert_eq!(
            replaced_pct(LOTAD, ClimateZone::Grassland, u16::MAX),
            replaced_pct(LOTAD, ClimateZone::Grassland, 5)
        );
    }

    #[test]
    fn rules_are_sane() {
        for r in RULES {
            assert_ne!(r.from, r.to, "rule replaces a species with itself");
            assert!(r.from != 0 && r.to != 0, "rule uses SPECIES_NONE");
            assert!(r.chance > 0 && r.chance <= 100);
            assert!(r.min_warming >= 2 && r.min_warming <= MAX_WARMING, "shifts start from warming 2");
            assert!(r.zones != 0 && r.zones & !ANY_ZONE == 0);
            // Replacements aren't themselves replaced in the same zone (no chains).
            for other in RULES {
                assert!(!(other.from == r.to && other.zones & r.zones != 0), "chained rule");
            }
        }
    }

    #[test]
    fn shift_never_decreases_with_warming() {
        for r in RULES {
            for zone in [ClimateZone::Grassland, ClimateZone::Coast] {
                let mut last = 0;
                for w in 0..=5 {
                    let pct = replaced_pct(r.from, zone, w);
                    assert!(pct >= last, "{} shifts less at higher warming", r.from);
                    last = pct;
                }
            }
        }
    }
}
