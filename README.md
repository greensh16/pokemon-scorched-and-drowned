# Pokémon: Scorched and Drowned

*A story-driven Pokémon Emerald romhack about a warming crisis that can't be undone.*

Hoenn has always been a region of drought and flood: Groudon raises the land and Kyogre raises the sea. **Scorched and Drowned** turns that conflict into climate change. The player arrives in Hoenn during its hottest summer on record and travels a region that gets measurably worse as the story goes on.

The player can't save Hoenn. They can only decide how much of it is lost, and with their Pokémon, what gets protected.

> **Status:** early development. Phase 0, the toolchain proof, is complete. Nothing is playable yet beyond the base game. See [Roadmap](#roadmap).

---

## Vision

### Pillars
- **Climate is the world state, not a cutscene.** One warming level, from 0 to 5, changes maps, weather, wild Pokémon and dialogue across the whole region.
- **Grounded in real science.** Range shifts, coral bleaching, sea-level rise, drought and bushfire follow real mechanisms. Professor Birch is a climate scientist who explains each one.
- **No restoration, only adaptation.** As in the real world, some damage is locked in. The player's choices decide how bad things get and which towns adapt, never whether Hoenn goes back to how it was.
- **Pokémon are the source of hope.** The story is grim but not hopeless. People and Pokémon adapt together, and every ending keeps what the player saved.
- **Built on Emerald, not around it.** It reuses Emerald's maps, battles and weather, and spends the effort on story and systems, not new art.

### Tone
Grim and grounded. Losses are permanent, and wins are about what gets saved, not what gets restored. The villains have real motives, and no one is a cartoon polluter.

---

## The story

The game has five acts that follow Emerald's route order, so the existing maps and progression gates carry over. Each act pushes warming up a level. Mitigation can slow the rise but never reverse it.

| Act | Region | What happens |
| --- | --- | --- |
| **1. The hottest summer** | Littleroot → Rustboro | You move to Hoenn during a record heatwave. Birch runs a climate field station and gives you a Climate Logger along with your Pokédex. |
| **2. Two answers** | Dewford → Lavaridge | Magma Industries drills under Mt Chimney. The Aqua Collective sabotages it and pushes radical ocean geoengineering. Both claim to be saving Hoenn. |
| **3. The data war** | Fortree → Lilycove | Aqua seizes the Weather Institute for its forecasting data. Fortree's forest burns in drought. |
| **4. Tipping points** | Mossdeep → Sootopolis | Groudon and Kyogre awaken, bringing permanent drought and permanent storms. Sootopolis floods. Warming level 2 is now locked in. |
| **5. Aftermath** | Sky Pillar → League | Rayquaza can stop the legendaries, but not the warming already locked in. The League run happens in a Hoenn that won't recover. |

### Factions
- **Magma Industries** (CEO Maxie): growth and energy security at any cost.
- **The Aqua Collective** (Archie): cool the planet now, whatever the side effects.
- **Devon Corporation**: funds both sides' research. Steven Stone's arc is about forcing it to change.

### Gym towns
Each gym town faces one local impact and offers one mitigation or adaptation quest: Rustboro's quarry, Dewford's failing reef, coal-powered Mauville, Fortree's canopy fires, Sootopolis's managed retreat, and more. Earning every town's trust is what calms Rayquaza.

### Endings
The final warming level decides the ending. There is no restoration ending.

| Ending | Final warming | Hoenn |
| --- | --- | --- |
| **Adaptation** (best) | 2–3 | Sea walls, relocated towns, managed retreat from Sootopolis. It survives, diminished. |
| **Collapse** (likely) | 4 | Coastal routes stay flooded, Fortree's forest is gone, and some towns are abandoned. |
| **Overshoot** (worst) | 5 | Drought and storms never lift, and the playable map shrinks. |

---

## Hope through Pokémon

Every grim system has a Pokémon-driven answer:

- **Pokémon do the adaptation work.** Field moves become rescue and rebuilding: Cut clears firebreaks, Surf ferries evacuees, Strength builds sea walls, Grass types replant burnt forest, and Water types fight fires.
- **The Refuge.** The Safari Zone becomes a climate refuge. Species pushed out of their range can be rescued and relocated there, so *catching them all* becomes *saving them all*.
- **Resilient species.** Castform, the weather Pokémon, is the game's mascot. A Corsola nursery in Dewford fights reef bleaching.
- **Every ending keeps something.** Even in Overshoot, the epilogue shows the towns you protected and the species in the Refuge.

---

## Climate systems

| System | What it does |
| --- | --- |
| **Warming level** | A single saved level, from 0 to 5, calculated at each act boundary from story pressure and mitigation points. Each act has a minimum level, so locked-in damage stays locked in. |
| **Map variants** | Affected maps have up to three layouts (normal, warm and extreme): flooded beaches, dried rainforest, a drowned Sootopolis. |
| **Range shifts** | Wild encounters move with warming. Cold-adapted species leave Shoal Cave, and tropical species spread north. |
| **Weather** | Heatwaves and storms get more frequent as warming rises, using Emerald's own weather effects. |
| **Reef bleaching** | Dive areas lose their colour, and their Pokémon change. |
| **Reactive dialogue** | Key NPCs react to how bad things have got. |
| **Climate Logger** (v2) | A PokéNav screen showing the warming meter, reef and forest health, and Birch's explanations. |

---

## How it's built

- **Base:** [pokeemerald-expansion](https://github.com/rh-hideout/pokeemerald-expansion), a C decompilation of Pokémon Emerald with modern features. It handles everything the original game does.
- **Climate systems:** a Rust `no_std` static library in [`climate-rs/`](climate-rs/), built for the GBA (`thumbv4t-none-eabi`) and linked into the ROM.
- **A narrow interface between them:** C calls a handful of `Climate_*` functions that take and return plain numbers. C owns all saved state, and Rust holds none of its own, so saves stay standard Emerald save data.
- **Glue:** all the C-side wiring lives in [`src/climate_glue.c`](src/climate_glue.c) to keep upstream merges simple.
- **Testing:** the climate logic is unit-tested on the host with `cargo test`. The ROM is tested in mGBA.

```
scorched-and-drowned/
├── climate-rs/            # Rust climate library (logic.rs, ffi.rs)
├── climate.mk             # Make rules for the Rust build
├── src/climate_glue.c     # C-side script specials and wrappers
├── include/climate.h      # C header for the Rust functions
└── …                      # pokeemerald-expansion
```

---

## Building

1. Set up pokeemerald-expansion's toolchain, devkitARM included, by following [INSTALL.md](INSTALL.md).
2. Install nightly Rust, which the tier-3 GBA target needs:
   ```bash
   rustup toolchain install nightly --component rust-src
   ```
3. Build and test:
   ```bash
   make climate-test                 # Rust unit tests on the host
   make -j$(sysctl -n hw.ncpu)       # or -j$(nproc) on Linux; builds Rust + C
   make climate-check                # checks Rust keeps no RAM state
   ```
4. Open `pokeemerald.gba` in [mGBA](https://mgba.io).

---

## Roadmap

| Phase | Goal | Status |
| --- | --- | --- |
| **0. Toolchain proof** | Rust code runs inside the ROM, with calls both ways between C and Rust | ✅ Done |
| **1. MVP vertical slice** | Littleroot → Petalburg: the warming system, one flooded beach variant, range-shifted encounters and rewritten early dialogue | Next |
| **2. v1: full story** | All five acts and eight gym arcs on existing maps, quests, weather and three endings | Planned |
| **3. v2: depth and polish** | Reef bleaching, forest states, the Refuge, the Climate Logger, custom sprites | Planned |
| **4. v3: real data** | Warming effects drawn from real sea-level and ocean-temperature projections | Stretch |

Scope rule: no new art until Phase 2 is finished.

---

## Distribution

- This repository contains **source code only**. It contains no ROM and never will.
- Releases will be `.bps` patches, applied to your own legally dumped Pokémon Emerald (US) ROM.
- This is a free, non-commercial fan project. Pokémon is © Nintendo, Creatures Inc. and GAME FREAK inc.

---

## Credits

Based off RHH's pokeemerald-expansion 1.17.2 https://github.com/rh-hideout/pokeemerald-expansion/

- [pret](https://github.com/pret/pokeemerald) for the pokeemerald decompilation
- [Rom Hacking Hideout](https://github.com/rh-hideout/pokeemerald-expansion) and all [pokeemerald-expansion contributors](CREDITS.md)
- Tools: Porymap, Poryscript, Porytiles, mGBA, Flips

For the base engine's documentation, see the [pokeemerald-expansion docs](https://rh-hideout.github.io/pokeemerald-expansion/).
