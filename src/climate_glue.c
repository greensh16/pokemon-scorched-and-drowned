// Glue between the C game and the Rust climate library (climate-rs).
// Script specials and any C-side wrappers live here, so the rest of the
// codebase only ever touches this file, which keeps upstream merges easy.

#include "global.h"
#include "event_data.h"
#include "constants/vars.h"
#include "climate.h"

// Phase 0 round trip, called from a map script with `special Climate_Phase0Special`.
//   1. C -> Rust: Climate_Ping() must return 42.
//   2. Rust -> C: Climate_ReadVarPlusOne reads VAR_TEMP_1 via VarGet and adds 1.
// The script sets VAR_TEMP_1 to 41, so a working build shows 42.
// A broken ping shows 0 instead.
void Climate_Phase0Special(void)
{
    if (Climate_Ping() != 42)
    {
        gSpecialVar_Result = 0;
        return;
    }
    gSpecialVar_Result = Climate_ReadVarPlusOne(VAR_TEMP_1);
}
