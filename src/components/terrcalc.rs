// basically directly ported from code by Lamadile
pub struct TerrStats {
    dmg_min: u16,
    dmg_max: u16,
    attack: f64,
    health: u32,
    defense: f64
}
const HP_BASE_STATS: [TerrStats; 12] = [
    TerrStats { dmg_min: 1000, dmg_max: 1500, attack: 0.5, health: 300000, defense: 0.1 },
    TerrStats { dmg_min: 1400, dmg_max: 2100, attack: 0.75, health: 450000, defense: 0.4 },
    TerrStats { dmg_min: 1800, dmg_max: 2700, attack: 1.0, health: 600000, defense: 0.55 },
    TerrStats { dmg_min: 2200, dmg_max: 3300, attack: 1.25, health: 750000, defense: 0.625 },
    TerrStats { dmg_min: 2600, dmg_max: 3900, attack: 1.6, health: 960000, defense: 0.7 },
    TerrStats { dmg_min: 3000, dmg_max: 4500, attack: 2.0, health: 1200000, defense: 0.75 },
    TerrStats { dmg_min: 3400, dmg_max: 5100, attack: 2.5, health: 1500000, defense: 0.79 },
    TerrStats { dmg_min: 3800, dmg_max: 5700, attack: 3.0, health: 1860000, defense: 0.82 },
    TerrStats { dmg_min: 4200, dmg_max: 6300, attack: 3.6, health: 2220000, defense: 0.84 },
    TerrStats { dmg_min: 4600, dmg_max: 6900, attack: 3.8, health: 2580000, defense: 0.86 },
    TerrStats { dmg_min: 5000, dmg_max: 7500, attack: 4.2, health: 2940000, defense: 0.88 },
    TerrStats { dmg_min: 5400, dmg_max: 8100, attack: 4.7, health: 3300000, defense: 0.9 }
];

pub fn get_terr_stats(lv_dmg: u8, lv_atk: u8, lv_hp: u8, lv_def: u8) -> TerrStats {
    let fallback_data = &HP_BASE_STATS[HP_BASE_STATS.len() -1]; // last element (max) used as fallback
    let dmg_min = HP_BASE_STATS.get(lv_dmg as usize)
        .unwrap_or(fallback_data)
        .dmg_min;
    let dmg_max = HP_BASE_STATS.get(lv_dmg as usize)
        .unwrap_or(fallback_data)
        .dmg_max;
    let atk = HP_BASE_STATS.get(lv_atk as usize)
        .unwrap_or(fallback_data)
        .attack;
    let hp = HP_BASE_STATS.get(lv_hp as usize)
        .unwrap_or(fallback_data)
        .health;
    let def = HP_BASE_STATS.get(lv_def as usize)
        .unwrap_or(fallback_data)
        .defense;

    TerrStats {
        dmg_min,
        dmg_max,
        attack: atk,
        health: hp,
        defense: def
    }
}