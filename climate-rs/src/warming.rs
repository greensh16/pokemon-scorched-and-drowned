//! The warming level: the single number (0–5) that drives the whole climate system.
//!
//! The level is recomputed at each act boundary from three saved inputs:
//! - **pressure**: story-driven push upward (villain actions, failed quests), in points
//! - **mitigation**: points earned from mitigation quests, which slow the climb
//! - **previous**: the level before this act, because warming never goes back down
//!
//! The rules that make Hoenn's story what it is:
//! - Each act raises the base level by one. With no mitigation and no extra pressure,
//!   Act 5 ends at 4 (Collapse, the "likely" ending).
//! - Each act has a floor. From Act 4, level 2 is locked in whatever the player does,
//!   so there is no restoration ending: Adaptation (2–3) is the best possible outcome.
//! - Mitigation can slow the rise but never reverse it: the result is never below
//!   the previous level.

/// Highest warming level.
pub const MAX_WARMING: u8 = 5;

/// Highest act. Act 0 is the prologue, before Act 1 formally starts.
pub const MAX_ACT: u8 = 5;

/// Pressure or mitigation points that add up to one warming level.
/// A gym-town mitigation quest is worth one level; side quests are worth a quarter.
pub const POINTS_PER_LEVEL: u16 = 32;

/// Base warming at the start of each act, before pressure and mitigation (index = act).
const BASE_FOR_ACT: [u8; MAX_ACT as usize + 1] = [0, 0, 1, 2, 3, 4];

/// Locked-in minimum for each act (index = act). Act 4 locks in level 2.
const FLOOR_FOR_ACT: [u8; MAX_ACT as usize + 1] = [0, 0, 0, 1, 2, 2];

/// Computes the warming level for `act`.
///
/// `warming = max(previous, floor, clamp(base + pressure/32 − mitigation/32, 0, 5))`
pub fn compute_warming(act: u16, pressure: u16, mitigation: u16, previous: u16) -> u8 {
    let act = act.min(MAX_ACT as u16) as usize;
    let push = (pressure / POINTS_PER_LEVEL) as i32;
    let relief = (mitigation / POINTS_PER_LEVEL) as i32;
    let raw = BASE_FOR_ACT[act] as i32 + push - relief;
    let level = raw.clamp(0, MAX_WARMING as i32) as u8;
    let previous = previous.min(MAX_WARMING as u16) as u8;
    level.max(FLOOR_FOR_ACT[act]).max(previous)
}

/// The three endings, chosen by the final warming level.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ending {
    /// Warming 2–3 (or lower via debug): sea walls, managed retreat, Hoenn survives diminished.
    Adaptation = 0,
    /// Warming 4: flooded coasts, lost forest, abandoned towns.
    Collapse = 1,
    /// Warming 5: drought and storms never lift; the playable map shrinks.
    Overshoot = 2,
}

/// Picks the ending from the final warming level.
pub fn ending(warming: u16) -> Ending {
    match warming {
        0..=3 => Ending::Adaptation,
        4 => Ending::Collapse,
        _ => Ending::Overshoot,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prologue_and_act_one_start_cool() {
        assert_eq!(compute_warming(0, 0, 0, 0), 0);
        assert_eq!(compute_warming(1, 0, 0, 0), 0);
    }

    #[test]
    fn each_act_adds_a_level_without_intervention() {
        let mut level = 0;
        let mut seen = vec![];
        for act in 1..=5 {
            level = compute_warming(act, 0, 0, level as u16);
            seen.push(level);
        }
        assert_eq!(seen, vec![0, 1, 2, 3, 4]);
        assert_eq!(ending(level as u16), Ending::Collapse, "doing nothing gives the likely ending");
    }

    #[test]
    fn pressure_pushes_into_overshoot() {
        assert_eq!(compute_warming(5, POINTS_PER_LEVEL, 0, 0), 5);
        assert_eq!(ending(5), Ending::Overshoot);
    }

    #[test]
    fn mitigation_slows_but_floor_holds() {
        // Two gym mitigation quests (Mauville, Lavaridge) take two levels off Act 5...
        assert_eq!(compute_warming(5, 0, 2 * POINTS_PER_LEVEL, 0), 2);
        // ...and no amount of mitigation goes below the locked-in floor.
        assert_eq!(compute_warming(5, 0, 255, 0), 2);
        assert_eq!(compute_warming(4, 0, 255, 0), 2);
        assert_eq!(compute_warming(3, 0, 255, 0), 1);
    }

    #[test]
    fn warming_never_goes_back_down() {
        // Previous level 4, then lots of mitigation: still 4.
        assert_eq!(compute_warming(5, 0, 255, 4), 4);
    }

    #[test]
    fn best_case_is_adaptation_never_restoration() {
        for pressure in [0u16, 16, 31] {
            for mitigation in 0..=255u16 {
                let level = compute_warming(5, pressure, mitigation, 0);
                assert!(level >= 2, "no restoration: level {level}");
            }
        }
    }

    #[test]
    fn out_of_range_inputs_are_clamped() {
        assert_eq!(compute_warming(99, 0, 0, 0), compute_warming(5, 0, 0, 0));
        assert_eq!(compute_warming(5, u16::MAX, 0, 0), MAX_WARMING);
        assert_eq!(compute_warming(0, 0, 0, 99), MAX_WARMING);
    }

    #[test]
    fn always_within_range() {
        for act in 0..=6u16 {
            for pressure in (0..=255u16).step_by(8) {
                for mitigation in (0..=255u16).step_by(8) {
                    for previous in 0..=6u16 {
                        let level = compute_warming(act, pressure, mitigation, previous);
                        assert!(level <= MAX_WARMING);
                    }
                }
            }
        }
    }

    #[test]
    fn endings_by_level() {
        assert_eq!(ending(2), Ending::Adaptation);
        assert_eq!(ending(3), Ending::Adaptation);
        assert_eq!(ending(4), Ending::Collapse);
        assert_eq!(ending(5), Ending::Overshoot);
    }
}
