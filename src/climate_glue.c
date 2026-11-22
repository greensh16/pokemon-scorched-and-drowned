// Glue between the C game and the Rust climate library (climate-rs).
// Script specials and any C-side wrappers live here, so the rest of the
// codebase only ever touches this file, which keeps upstream merges easy.

#include "global.h"
#include "event_data.h"
#include "constants/maps.h"
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

// ---------------------------------------------------------------------------
// Phase 1: warming level and map variants
// ---------------------------------------------------------------------------

// Translates a pokeemerald map into a climate map id (see variants.rs).
// Add new climate-affected maps here and in Rust's ClimateMap.
static u8 GetClimateMapId(u8 mapGroup, u8 mapNum)
{
    if (mapGroup == MAP_GROUP(MAP_ROUTE104) && mapNum == MAP_NUM(MAP_ROUTE104))
        return CLIMATE_MAP_ROUTE104_BEACH;
    return CLIMATE_MAP_NONE;
}

// `special Climate_GetMapVariant` from a map's ON_TRANSITION script.
// VAR_RESULT = layout tier for the map being entered: 0 normal, 1 warm, 2 extreme.
void Climate_GetMapVariant(void)
{
    u8 climateMap = GetClimateMapId(gSaveBlock1Ptr->location.mapGroup, gSaveBlock1Ptr->location.mapNum);
    gSpecialVar_Result = Climate_MapVariant(climateMap, VarGet(VAR_CLIMATE_WARMING));
}

// `special Climate_UpdateWarming` at an act boundary, after VAR_CLIMATE_ACT is set.
// Stores the new level in VAR_CLIMATE_WARMING and VAR_RESULT.
void Climate_UpdateWarming(void)
{
    u8 level = Climate_ComputeWarming(VarGet(VAR_CLIMATE_ACT),
                                      VarGet(VAR_CLIMATE_PRESSURE),
                                      VarGet(VAR_CLIMATE_MITIGATION),
                                      VarGet(VAR_CLIMATE_WARMING));
    VarSet(VAR_CLIMATE_WARMING, level);
    gSpecialVar_Result = level;
}

// `special Climate_AdvanceAct`: moves to the next act (max 5) and updates warming.
void Climate_AdvanceAct(void)
{
    u16 act = VarGet(VAR_CLIMATE_ACT);
    if (act < 5)
        VarSet(VAR_CLIMATE_ACT, act + 1);
    Climate_UpdateWarming();
}

// `special Climate_GetEnding` after the League: VAR_RESULT = 0 Adaptation, 1 Collapse, 2 Overshoot.
void Climate_GetEnding(void)
{
    gSpecialVar_Result = Climate_Ending(VarGet(VAR_CLIMATE_WARMING));
}
