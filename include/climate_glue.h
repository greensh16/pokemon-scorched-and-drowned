#ifndef GUARD_CLIMATE_GLUE_H
#define GUARD_CLIMATE_GLUE_H

// C-side climate hooks (src/climate_glue.c), called from game code.
// The Rust functions themselves are declared in climate.h.

// Species for a wild encounter on the current map after climate range shifts.
u16 Climate_ShiftWildSpecies(u16 species);

#endif // GUARD_CLIMATE_GLUE_H
