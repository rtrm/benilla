//! Display entries for the reserved mount spell-ID range (61000-61999, see ARCHITECTURE.md), the
//! same "no Spell.dbc row" problem `synthetic_spells.rs` solves for companion pets, for mounts.
//! Deliberately a sibling module, not a generalization of that one: a mount's Summon spell is a
//! full clone of the original stock spell (speed, level requirement and flavor text all carry
//! real per-mount variation a shared template would flatten), where a companion pet's is built
//! from a uniform template - the generation needed to differ, so the data structures do too.

use benilla_formats::{SpellCatalog, SpellDisplay};

/// The reserved mount spell-id range (ARCHITECTURE.md), parallel to `synthetic_spells`'s
/// `COMPANION_SPELL_RANGE` but for mounts - kept in its own block (not interleaved with pets') so
/// the client can tell a "Pets" entry from a "Mounts" entry with a plain id-range check, the same
/// way it tells either apart from an ordinary DBC-backed spell.
pub(crate) const MOUNT_SPELL_RANGE: std::ops::RangeInclusive<u32> = 61000..=61999;

/// `TARGET_UNIT_CASTER`, as `synthetic_spells::TARGET_UNIT_CASTER` - self-cast, no target.
const TARGET_UNIT_CASTER: u32 = 1;

/// `6`, `SPELL_EFFECT_APPLY_AURA`: every mount spell's real effect1 (the Mounted aura, 78, plus a
/// speed-increase aura, both applied this way). Set for data-correctness parity with the real
/// cloned spells below; no client-side logic branches on a display row's `effects` today.
const EFFECT_APPLY_AURA: u32 = 6;

/// `SpellVisual.dbc` id for every "Teach Mount" spell's cast: the same real trainer-learn visual
/// `synthetic_spells::TEACH_VISUAL` uses for companion pets (confirmed there against live stock
/// spell_template data) - one fix, applied from the start here rather than rediscovered.
const TEACH_VISUAL: u32 = 222;

/// Every "Teach Mount" spell id: the contiguous 61107..=61218 block
/// `20261006003000_world.sql` added one per migrated mount item. Same reasoning as
/// `synthetic_spells::teach_spell_ids`: never learned, never in the spellbook, but still need a
/// catalog row so the cast-visual system can resolve [`TEACH_VISUAL`] by spell id.
fn teach_spell_ids() -> impl Iterator<Item = u32> {
    61107..=61218
}

/// One mount's catalog row. Unlike `synthetic_spells::Companion`, no `icon`/`visual` doc repeats
/// the "confirmed real, not guessed" story per-field - see the migration's own header
/// (`20261006003000_world.sql`) for how every value below was resolved: icon from the item's own
/// `ItemDisplayInfo.dbc` row, visual from the stock spell's own real `Spell.dbc` value (verified
/// per mount-type, not copied from one example - horses, wolves, rams, raptors, mechanostriders,
/// cats and kodos each have their own real summon visual).
struct Mount {
    id: u32,
    name: &'static str,
    icon: &'static str,
    visual: u32,
}

const MOUNTS: &[Mount] = &[
    Mount {
        id: 61000,
        name: "Summon Mount: Brown Horse",
        icon: "Interface\\Icons\\INV_Scroll_08",
        visual: 1703,
    },
    Mount {
        id: 61001,
        name: "Summon Mount: Gray Wolf",
        icon: "Interface\\Icons\\Ability_Mount_WhiteDireWolf",
        visual: 1709,
    },
    Mount {
        id: 61002,
        name: "Summon Mount: White Stallion",
        icon: "Interface\\Icons\\Ability_Mount_RidingHorse",
        visual: 1703,
    },
    Mount {
        id: 61003,
        name: "Summon Mount: Black Stallion",
        icon: "Interface\\Icons\\Ability_Mount_RidingHorse",
        visual: 1703,
    },
    Mount {
        id: 61004,
        name: "Summon Mount: Palamino Stallion",
        icon: "Interface\\Icons\\Ability_Mount_RidingHorse",
        visual: 1703,
    },
    Mount {
        id: 61005,
        name: "Summon Mount: Pinto Horse",
        icon: "Interface\\Icons\\Ability_Mount_RidingHorse",
        visual: 1703,
    },
    Mount {
        id: 61006,
        name: "Summon Mount: Black Wolf",
        icon: "Interface\\Icons\\Ability_Mount_BlackDireWolf",
        visual: 1709,
    },
    Mount {
        id: 61007,
        name: "Summon Mount: Red Wolf",
        icon: "Interface\\Icons\\Ability_Mount_BlackDireWolf",
        visual: 1709,
    },
    Mount {
        id: 61008,
        name: "Summon Mount: Large Timber Wolf",
        icon: "Interface\\Icons\\Ability_Mount_BlackDireWolf",
        visual: 1709,
    },
    Mount {
        id: 61009,
        name: "Summon Mount: Winter Wolf",
        icon: "Interface\\Icons\\Ability_Mount_WhiteDireWolf",
        visual: 1709,
    },
    Mount {
        id: 61010,
        name: "Summon Mount: Riding Gryphon",
        icon: "Interface\\Icons\\Ability_Mount_Undeadhorse",
        visual: 0,
    },
    Mount {
        id: 61011,
        name: "Summon Mount: Chestnut Mare",
        icon: "Interface\\Icons\\Ability_Mount_RidingHorse",
        visual: 1703,
    },
    Mount {
        id: 61012,
        name: "Summon Mount: Dire Wolf",
        icon: "Interface\\Icons\\Ability_Mount_WhiteDireWolf",
        visual: 1709,
    },
    Mount {
        id: 61013,
        name: "Summon Mount: Brown Wolf",
        icon: "Interface\\Icons\\Ability_Mount_BlackDireWolf",
        visual: 1709,
    },
    Mount {
        id: 61014,
        name: "Summon Mount: Gray Ram",
        icon: "Interface\\Icons\\Ability_Mount_MountainRam",
        visual: 1704,
    },
    Mount {
        id: 61015,
        name: "Summon Mount: Black Ram",
        icon: "Interface\\Icons\\Ability_Mount_MountainRam",
        visual: 1704,
    },
    Mount {
        id: 61016,
        name: "Summon Mount: Blue Ram",
        icon: "Interface\\Icons\\Ability_Mount_MountainRam",
        visual: 1704,
    },
    Mount {
        id: 61017,
        name: "Summon Mount: White Ram",
        icon: "Interface\\Icons\\Ability_Mount_MountainRam",
        visual: 1704,
    },
    Mount {
        id: 61018,
        name: "Summon Mount: Brown Ram",
        icon: "Interface\\Icons\\Ability_Mount_MountainRam",
        visual: 1704,
    },
    Mount {
        id: 61019,
        name: "Summon Mount: Striped Frostsaber",
        icon: "Interface\\Icons\\Ability_Mount_WhiteTiger",
        visual: 1708,
    },
    Mount {
        id: 61020,
        name: "Summon Mount: Emerald Raptor",
        icon: "Interface\\Icons\\Ability_Mount_Raptor",
        visual: 1705,
    },
    Mount {
        id: 61021,
        name: "Summon Mount: Skeletal Horse",
        icon: "Interface\\Icons\\Ability_Mount_Undeadhorse",
        visual: 1706,
    },
    Mount {
        id: 61022,
        name: "Summon Mount: Panther",
        icon: "Interface\\Icons\\Ability_Mount_BlackPanther",
        visual: 1708,
    },
    Mount {
        id: 61023,
        name: "Summon Mount: Leopard",
        icon: "Interface\\Icons\\Ability_Mount_JungleTiger",
        visual: 1708,
    },
    Mount {
        id: 61024,
        name: "Summon Mount: Spotted Frostsaber",
        icon: "Interface\\Icons\\Ability_Mount_WhiteTiger",
        visual: 1708,
    },
    Mount {
        id: 61025,
        name: "Summon Mount: Tiger",
        icon: "Interface\\Icons\\Ability_Mount_JungleTiger",
        visual: 1708,
    },
    Mount {
        id: 61026,
        name: "Summon Mount: Spotted Panther",
        icon: "Interface\\Icons\\Ability_Mount_BlackPanther",
        visual: 1708,
    },
    Mount {
        id: 61027,
        name: "Summon Mount: Striped Nightsaber",
        icon: "Interface\\Icons\\Ability_Mount_BlackPanther",
        visual: 1708,
    },
    Mount {
        id: 61028,
        name: "Summon Mount: Ivory Raptor",
        icon: "Interface\\Icons\\Ability_Mount_Raptor",
        visual: 1705,
    },
    Mount {
        id: 61029,
        name: "Summon Mount: Turquoise Raptor",
        icon: "Interface\\Icons\\Ability_Mount_Raptor",
        visual: 1705,
    },
    Mount {
        id: 61030,
        name: "Summon Mount: Obsidian Raptor",
        icon: "Interface\\Icons\\Ability_Mount_Raptor",
        visual: 1705,
    },
    Mount {
        id: 61031,
        name: "Summon Mount: Violet Raptor",
        icon: "Interface\\Icons\\Ability_Mount_Raptor",
        visual: 1705,
    },
    Mount {
        id: 61032,
        name: "Summon Mount: Red Mechanostrider",
        icon: "Interface\\Icons\\Ability_Mount_MechaStrider",
        visual: 1707,
    },
    Mount {
        id: 61033,
        name: "Summon Mount: Blue Mechanostrider",
        icon: "Interface\\Icons\\Ability_Mount_MechaStrider",
        visual: 1707,
    },
    Mount {
        id: 61034,
        name: "Summon Mount: White Mechanostrider",
        icon: "Interface\\Icons\\Ability_Mount_MechaStrider",
        visual: 1707,
    },
    Mount {
        id: 61035,
        name: "Summon Mount: Nightsaber",
        icon: "Interface\\Icons\\Ability_Mount_WhiteTiger",
        visual: 1708,
    },
    Mount {
        id: 61036,
        name: "Summon Mount: Frostsaber",
        icon: "Interface\\Icons\\Ability_Mount_WhiteTiger",
        visual: 1708,
    },
    Mount {
        id: 61037,
        name: "Summon Mount: Primal Leopard",
        icon: "Interface\\Icons\\Ability_Mount_JungleTiger",
        visual: 1708,
    },
    Mount {
        id: 61038,
        name: "Summon Mount: Tawny Sabercat",
        icon: "Interface\\Icons\\Ability_Mount_JungleTiger",
        visual: 1708,
    },
    Mount {
        id: 61039,
        name: "Summon Mount: Golden Sabercat",
        icon: "Interface\\Icons\\Ability_Mount_JungleTiger",
        visual: 1708,
    },
    Mount {
        id: 61040,
        name: "Summon Mount: Red Wolf (Alt)",
        icon: "Interface\\Icons\\Ability_Mount_BlackDireWolf",
        visual: 1709,
    },
    Mount {
        id: 61041,
        name: "Summon Mount: Arctic Wolf",
        icon: "Interface\\Icons\\Ability_Mount_WhiteDireWolf",
        visual: 1709,
    },
    Mount {
        id: 61042,
        name: "Summon Mount: Palomino Stallion",
        icon: "Interface\\Icons\\Ability_Mount_RidingHorse",
        visual: 1703,
    },
    Mount {
        id: 61043,
        name: "Summon Mount: Swift White Stallion",
        icon: "Interface\\Icons\\Ability_Mount_RidingHorse",
        visual: 1703,
    },
    Mount {
        id: 61044,
        name: "Summon Mount: Mottled Red Raptor",
        icon: "Interface\\Icons\\Ability_Mount_Raptor",
        visual: 1705,
    },
    Mount {
        id: 61045,
        name: "Summon Mount: Winterspring Frostsaber",
        icon: "Interface\\Icons\\Ability_Mount_PinkTiger",
        visual: 1708,
    },
    Mount {
        id: 61046,
        name: "Summon Mount: Swift Ivory Raptor",
        icon: "Interface\\Icons\\Ability_Mount_Raptor",
        visual: 1705,
    },
    Mount {
        id: 61047,
        name: "Summon Mount: Green Mechanostrider",
        icon: "Interface\\Icons\\Ability_Mount_MechaStrider",
        visual: 1707,
    },
    Mount {
        id: 61048,
        name: "Summon Mount: Unpainted Mechanostrider",
        icon: "Interface\\Icons\\Ability_Mount_MechaStrider",
        visual: 1707,
    },
    Mount {
        id: 61049,
        name: "Summon Mount: Purple Mechanostrider",
        icon: "Interface\\Icons\\Ability_Mount_MechaStrider",
        visual: 1707,
    },
    Mount {
        id: 61050,
        name: "Summon Mount: Red & Blue Mechanostrider",
        icon: "Interface\\Icons\\Ability_Mount_MechaStrider",
        visual: 1707,
    },
    Mount {
        id: 61051,
        name: "Summon Mount: Fluorescent Green Mechanostrider",
        icon: "Interface\\Icons\\Ability_Mount_MechaStrider",
        visual: 1707,
    },
    Mount {
        id: 61052,
        name: "Summon Mount: Icy Blue Mechanostrider",
        icon: "Interface\\Icons\\Ability_Mount_MechaStrider",
        visual: 1707,
    },
    Mount {
        id: 61053,
        name: "Summon Mount: Frost Ram",
        icon: "Interface\\Icons\\Ability_Mount_MountainRam",
        visual: 1704,
    },
    Mount {
        id: 61054,
        name: "Summon Mount: Swift Black Ram",
        icon: "Interface\\Icons\\Ability_Mount_MountainRam",
        visual: 1704,
    },
    Mount {
        id: 61055,
        name: "Summon Mount: Red Skeletal Horse",
        icon: "Interface\\Icons\\Ability_Mount_Undeadhorse",
        visual: 1706,
    },
    Mount {
        id: 61056,
        name: "Summon Mount: Blue Skeletal Horse",
        icon: "Interface\\Icons\\Ability_Mount_Undeadhorse",
        visual: 1706,
    },
    Mount {
        id: 61057,
        name: "Summon Mount: Brown Skeletal Horse",
        icon: "Interface\\Icons\\Ability_Mount_Undeadhorse",
        visual: 1706,
    },
    Mount {
        id: 61058,
        name: "Summon Mount: Green Skeletal Warhorse",
        icon: "Interface\\Icons\\Ability_Mount_Undeadhorse",
        visual: 1706,
    },
    Mount {
        id: 61059,
        name: "Summon Mount: Deathcharger",
        icon: "Interface\\Icons\\Ability_Mount_Undeadhorse",
        visual: 1706,
    },
    Mount {
        id: 61060,
        name: "Summon Mount: Riding Kodo",
        icon: "Interface\\Icons\\Ability_Mount_Undeadhorse",
        visual: 1703,
    },
    Mount {
        id: 61061,
        name: "Summon Mount: Gray Kodo",
        icon: "Interface\\Icons\\Ability_Mount_Kodo_01",
        visual: 5160,
    },
    Mount {
        id: 61062,
        name: "Summon Mount: Brown Kodo",
        icon: "Interface\\Icons\\Ability_Mount_Kodo_03",
        visual: 5160,
    },
    Mount {
        id: 61063,
        name: "Summon Mount: Green Kodo",
        icon: "Interface\\Icons\\Ability_Mount_Kodo_02",
        visual: 5160,
    },
    Mount {
        id: 61064,
        name: "Summon Mount: Teal Kodo",
        icon: "Interface\\Icons\\Ability_Mount_Kodo_02",
        visual: 5160,
    },
    Mount {
        id: 61065,
        name: "Summon Mount: Black War Steed",
        icon: "Interface\\Icons\\Ability_Mount_NightmareHorse",
        visual: 1703,
    },
    Mount {
        id: 61066,
        name: "Summon Mount: Black War Kodo",
        icon: "Interface\\Icons\\Ability_Mount_Kodo_03",
        visual: 5160,
    },
    Mount {
        id: 61067,
        name: "Summon Mount: Black Battlestrider",
        icon: "Interface\\Icons\\Ability_Mount_MechaStrider",
        visual: 1707,
    },
    Mount {
        id: 61068,
        name: "Summon Mount: Black War Ram",
        icon: "Interface\\Icons\\Ability_Mount_MountainRam",
        visual: 1704,
    },
    Mount {
        id: 61069,
        name: "Summon Mount: Black War Raptor",
        icon: "Interface\\Icons\\Ability_Mount_Raptor",
        visual: 1705,
    },
    Mount {
        id: 61070,
        name: "Summon Mount: Red Skeletal Warhorse",
        icon: "Interface\\Icons\\Ability_Mount_Undeadhorse",
        visual: 1706,
    },
    Mount {
        id: 61071,
        name: "Summon Mount: Black War Tiger",
        icon: "Interface\\Icons\\Ability_Mount_BlackPanther",
        visual: 1708,
    },
    Mount {
        id: 61072,
        name: "Summon Mount: Black War Wolf",
        icon: "Interface\\Icons\\Ability_Mount_BlackDireWolf",
        visual: 1709,
    },
    Mount {
        id: 61073,
        name: "Summon Mount: Swift Mistsaber",
        icon: "Interface\\Icons\\Ability_Mount_WhiteTiger",
        visual: 1708,
    },
    Mount {
        id: 61074,
        name: "Summon Mount: Swift Dawnsaber",
        icon: "Interface\\Icons\\Ability_Mount_WhiteTiger",
        visual: 1708,
    },
    Mount {
        id: 61075,
        name: "Summon Mount: Swift Frostsaber",
        icon: "Interface\\Icons\\Ability_Mount_WhiteTiger",
        visual: 1708,
    },
    Mount {
        id: 61076,
        name: "Summon Mount: Swift Yellow Mechanostrider",
        icon: "Interface\\Icons\\Ability_Mount_MechaStrider",
        visual: 1707,
    },
    Mount {
        id: 61077,
        name: "Summon Mount: Swift White Mechanostrider",
        icon: "Interface\\Icons\\Ability_Mount_MechaStrider",
        visual: 1707,
    },
    Mount {
        id: 61078,
        name: "Summon Mount: Swift Green Mechanostrider",
        icon: "Interface\\Icons\\Ability_Mount_MechaStrider",
        visual: 1707,
    },
    Mount {
        id: 61079,
        name: "Summon Mount: Swift Palomino",
        icon: "Interface\\Icons\\Ability_Mount_RidingHorse",
        visual: 1703,
    },
    Mount {
        id: 61080,
        name: "Summon Mount: Swift White Steed",
        icon: "Interface\\Icons\\Ability_Mount_RidingHorse",
        visual: 1703,
    },
    Mount {
        id: 61081,
        name: "Summon Mount: Swift Brown Steed",
        icon: "Interface\\Icons\\Ability_Mount_RidingHorse",
        visual: 1703,
    },
    Mount {
        id: 61082,
        name: "Summon Mount: Swift Brown Ram",
        icon: "Interface\\Icons\\Ability_Mount_MountainRam",
        visual: 1704,
    },
    Mount {
        id: 61083,
        name: "Summon Mount: Swift Gray Ram",
        icon: "Interface\\Icons\\Ability_Mount_MountainRam",
        visual: 1704,
    },
    Mount {
        id: 61084,
        name: "Summon Mount: Swift White Ram",
        icon: "Interface\\Icons\\Ability_Mount_MountainRam",
        visual: 1704,
    },
    Mount {
        id: 61085,
        name: "Summon Mount: Swift Blue Raptor",
        icon: "Interface\\Icons\\Ability_Mount_Raptor",
        visual: 1705,
    },
    Mount {
        id: 61086,
        name: "Summon Mount: Swift Olive Raptor",
        icon: "Interface\\Icons\\Ability_Mount_Raptor",
        visual: 1705,
    },
    Mount {
        id: 61087,
        name: "Summon Mount: Swift Orange Raptor",
        icon: "Interface\\Icons\\Ability_Mount_Raptor",
        visual: 1705,
    },
    Mount {
        id: 61088,
        name: "Summon Mount: Purple Skeletal Warhorse",
        icon: "Interface\\Icons\\Ability_Mount_Undeadhorse",
        visual: 1706,
    },
    Mount {
        id: 61089,
        name: "Summon Mount: Great White Kodo",
        icon: "Interface\\Icons\\Ability_Mount_Kodo_01",
        visual: 5160,
    },
    Mount {
        id: 61090,
        name: "Summon Mount: Great Gray Kodo",
        icon: "Interface\\Icons\\Ability_Mount_Kodo_01",
        visual: 5160,
    },
    Mount {
        id: 61091,
        name: "Summon Mount: Great Brown Kodo",
        icon: "Interface\\Icons\\Ability_Mount_Kodo_03",
        visual: 5160,
    },
    Mount {
        id: 61092,
        name: "Summon Mount: Swift Brown Wolf",
        icon: "Interface\\Icons\\Ability_Mount_BlackDireWolf",
        visual: 1709,
    },
    Mount {
        id: 61093,
        name: "Summon Mount: Swift Timber Wolf",
        icon: "Interface\\Icons\\Ability_Mount_WhiteDireWolf",
        visual: 1709,
    },
    Mount {
        id: 61094,
        name: "Summon Mount: Swift Gray Wolf",
        icon: "Interface\\Icons\\Ability_Mount_WhiteDireWolf",
        visual: 1709,
    },
    Mount {
        id: 61095,
        name: "Summon Mount: Swift Stormsaber",
        icon: "Interface\\Icons\\Ability_Mount_WhiteTiger",
        visual: 1708,
    },
    Mount {
        id: 61096,
        name: "Summon Mount: Frostwolf Howler",
        icon: "Interface\\Icons\\INV_Misc_Horn_01",
        visual: 1709,
    },
    Mount {
        id: 61097,
        name: "Summon Mount: Stormpike Battle Charger",
        icon: "Interface\\Icons\\Ability_Mount_MountainRam",
        visual: 1704,
    },
    Mount {
        id: 61098,
        name: "Summon Mount: Swift Razzashi Raptor",
        icon: "Interface\\Icons\\Ability_Mount_Raptor",
        visual: 1705,
    },
    Mount {
        id: 61099,
        name: "Summon Mount: Swift Zulian Tiger",
        icon: "Interface\\Icons\\Ability_Mount_JungleTiger",
        visual: 1708,
    },
    Mount {
        id: 61100,
        name: "Summon Mount: Reindeer",
        icon: "Interface\\Icons\\Ability_Mount_Undeadhorse",
        visual: 1704,
    },
    Mount {
        id: 61101,
        name: "Summon Mount: Blue Qiraji Battle Tank",
        icon: "Interface\\Icons\\INV_Misc_QirajiCrystal_04",
        visual: 7456,
    },
    Mount {
        id: 61102,
        name: "Summon Mount: Red Qiraji Battle Tank",
        icon: "Interface\\Icons\\INV_Misc_QirajiCrystal_02",
        visual: 7456,
    },
    Mount {
        id: 61103,
        name: "Summon Mount: Yellow Qiraji Battle Tank",
        icon: "Interface\\Icons\\INV_Misc_QirajiCrystal_01",
        visual: 7456,
    },
    Mount {
        id: 61104,
        name: "Summon Mount: Green Qiraji Battle Tank",
        icon: "Interface\\Icons\\INV_Misc_QirajiCrystal_03",
        visual: 7456,
    },
    Mount {
        id: 61105,
        name: "Summon Mount: Skeletal Steed",
        icon: "Interface\\Icons\\Ability_Mount_Undeadhorse",
        visual: 1706,
    },
    Mount {
        id: 61106,
        name: "Summon Mount: Riding Turtle",
        icon: "Interface\\Icons\\Ability_Hunter_Pet_Turtle",
        visual: 5160,
    },
];

/// Installs every reserved mount-range display row into `catalog`, called once after `Spell.dbc`
/// loads (`ui_action::load_spells`), the same place and same pattern as
/// `synthetic_spells::install`.
pub(crate) fn install(catalog: &mut SpellCatalog) {
    for id in teach_spell_ids() {
        catalog.insert(
            id,
            SpellDisplay {
                id,
                visual: TEACH_VISUAL,
                ..Default::default()
            },
        );
    }
    for mount in MOUNTS {
        debug_assert!(
            MOUNT_SPELL_RANGE.contains(&mount.id),
            "{} is outside MOUNT_SPELL_RANGE - the Mounts tab and click-to-cast would both miss it",
            mount.id
        );
        catalog.insert(
            mount.id,
            SpellDisplay {
                id: mount.id,
                name: mount.name.to_string(),
                icon: Some(mount.icon.to_string()),
                visual: mount.visual,
                effects: [EFFECT_APPLY_AURA, EFFECT_APPLY_AURA, 0],
                implicit_target_a1: TARGET_UNIT_CASTER,
                // As `synthetic_spells::install`: `targets: 0` plus the implicit-target arm above
                // ask for no target at all, casting_time_index 0 resolves to instant.
                ..Default::default()
            },
        );
    }
}
