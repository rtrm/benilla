//! Display entries for the reserved custom spell-ID range (60000-60999, see ARCHITECTURE.md)
//! that `Spell.dbc` was never going to carry, since these abilities only exist on our own server
//! fork. Without an entry here, [`benilla_formats::SpellCatalog::get`] returns `None` for them and
//! the spellbook's add-gate (`ui_spellbook.rs`'s `build_book`) silently drops them - a learned
//! spell with no DBC row is otherwise invisible no matter how correctly the server taught it.
//!
//! This earns these spells a name and icon; the dedicated "Pets" tab grouping and the
//! click-to-cast override (ARCHITECTURE.md build-order steps 2-3) are implemented separately, in
//! `ui_spellbook.rs`'s `build_book` and `benilla-ui`'s `pickup_spell`, both keyed off
//! [`COMPANION_SPELL_RANGE`].

use benilla_formats::{SpellCatalog, SpellDisplay};

/// The reserved custom spell-id range (ARCHITECTURE.md) for companion vanity pets: server-only
/// abilities with no `Spell.dbc` row. The spellbook's "Pets" tab (`ui_spellbook.rs`) and its
/// click-to-cast override (`benilla-ui`'s `pickup_spell`, which duplicates this same range as a
/// literal, since that crate has no dependency on this one) both key off it.
pub(crate) const COMPANION_SPELL_RANGE: std::ops::RangeInclusive<u32> = 60000..=60999;

/// `TARGET_UNIT_CASTER` (`cast_target.rs::cast_target_mask`): clears the explicit-target-required
/// bit, so clicking the spell fires immediately with no reticle - matching the server's own
/// `effectImplicitTargetA1 = 1` on these spells (see `server/sql/migrations/20261004120000_world.sql`).
const TARGET_UNIT_CASTER: u32 = 1;

/// `97`, this build's own stand-in for `SPELL_EFFECT_SUMMON_CRITTER`: no client-side logic branches
/// on it today (benilla has no companion-pet concept yet - ARCHITECTURE.md step 4), it is set only
/// so a display row's `effects[0]` isn't silently wrong if something comes to depend on it later.
const EFFECT_SUMMON_CRITTER: u32 = 97;

/// `SpellVisual.dbc` id for every "Teach Companion" spell's cast - the trainer-style effect around
/// the player when the item teaches the pet (requested after live testing). Not invented: it's the
/// real, standard "you learned a spell from a trainer" visual, confirmed by querying every stock
/// `LEARN_SPELL` spell_template row and finding this value on 1228 of them - the trainer-side
/// wrapper every ordinary class spell (Frostbolt, Polymorph, Frost Armor, ...) uses when learned.
/// As with the summon/dismiss sound, the server's own `spellVisual1` column
/// (`server/sql/migrations/20261005233000_world.sql`) does not reach the client over the wire by
/// itself - this needs its own client-side catalog row too, below.
const TEACH_VISUAL: u32 = 222;

/// Every "Teach Companion" spell id: 60001 (the pilot) plus the contiguous 60067..=60135 block
/// `20261005230630_world.sql` added one per migrated item. These never reach the spellbook - they
/// are cast directly by the item's on-use effect and never land in `character_spell` - so unlike
/// [`COMPANIONS`] they get no name or icon, only enough of a display row for the cast-visual system
/// (`creature_anim::spell_visual`) to resolve [`TEACH_VISUAL`] by spell id.
fn teach_spell_ids() -> impl Iterator<Item = u32> {
    std::iter::once(60001).chain(60067..=60135)
}

/// One companion's catalog row. This range has no DBC source to generate from, so these were
/// generated from the live world DB instead (not hand-typed): every stock item whose on-use spell
/// is `SPELL_EFFECT_SUMMON_CRITTER`, resolved through the exact same progressive patch/build
/// selection the server itself uses (`item_template.patch` against the live `WowPatch` config,
/// `spell_template.build` against `SUPPORTED_CLIENT_BUILD`) so this list matches what a 1.12.1
/// character can actually obtain - not stale historical rows or later-build stubs the DB still
/// carries. `icon`/`visual` are each pet's own real values (the item's display icon, the stock
/// spell's own `SpellVisual.dbc` id), not reused from Black Tabby's.
struct Companion {
    id: u32,
    name: &'static str,
    /// MPQ path, without extension - [`SpellDisplay::icon`]'s own convention. Confirmed to exist
    /// in the 5875 client (`cargo run -p benilla-formats --example list_chain -- petcarrier`
    /// found `Interface\Icons\INV_Box_PetCarrier_01.blp`), the same carrier icon the item itself
    /// already shows, rather than left blank or guessed.
    icon: &'static str,
    /// `SpellVisual.dbc` id (`SpellDisplay::visual`): the cast visual/sound. The server's own
    /// `spellVisual1` column does NOT reach the client over the wire (only the spell id does) -
    /// a synthetic entry's `visual` is the only thing that actually drives sound, confirmed live
    /// (setting the server column alone did nothing). 353 is spell 10675 "Summon Maine Coon"'s
    /// own real client-side value (`Spell.dbc`, read via `load_spell_catalog` directly, not
    /// assumed from the server DB, though the two did agree) - this item's stock summon spell
    /// before milestone 1 repointed it, so it's already the right sound, not a guess.
    visual: u32,
}

const COMPANIONS: &[Companion] = &[
    Companion {
        id: 60002,
        name: "Summon Companion: Black Tabby",
        icon: "Interface\\Icons\\INV_Box_PetCarrier_01",
        visual: 353,
    },
    Companion {
        id: 60003,
        name: "Summon Companion: Mechanical Squirrel",
        icon: "Interface\\Icons\\INV_Crate_01",
        visual: 353,
    },
    Companion {
        id: 60004,
        name: "Summon Companion: Bombay",
        icon: "Interface\\Icons\\INV_Box_PetCarrier_01",
        visual: 353,
    },
    Companion {
        id: 60005,
        name: "Summon Companion: Cornish Rex",
        icon: "Interface\\Icons\\INV_Box_PetCarrier_01",
        visual: 353,
    },
    Companion {
        id: 60006,
        name: "Summon Companion: Orange Tabby",
        icon: "Interface\\Icons\\INV_Box_PetCarrier_01",
        visual: 353,
    },
    Companion {
        id: 60007,
        name: "Summon Companion: Siamese",
        icon: "Interface\\Icons\\INV_Box_PetCarrier_01",
        visual: 353,
    },
    Companion {
        id: 60008,
        name: "Summon Companion: Silver Tabby",
        icon: "Interface\\Icons\\INV_Box_PetCarrier_01",
        visual: 353,
    },
    Companion {
        id: 60009,
        name: "Summon Companion: White Kitten",
        icon: "Interface\\Icons\\INV_Box_PetCarrier_01",
        visual: 353,
    },
    Companion {
        id: 60010,
        name: "Summon Companion: Cockatiel",
        icon: "Interface\\Icons\\Spell_Nature_ForceOfNature",
        visual: 215,
    },
    Companion {
        id: 60011,
        name: "Summon Companion: Hyacinth Macaw",
        icon: "Interface\\Icons\\Spell_Nature_ForceOfNature",
        visual: 215,
    },
    Companion {
        id: 60012,
        name: "Summon Companion: Green Wing Macaw",
        icon: "Interface\\Icons\\Spell_Nature_ForceOfNature",
        visual: 215,
    },
    Companion {
        id: 60013,
        name: "Summon Companion: Senegal",
        icon: "Interface\\Icons\\Spell_Nature_ForceOfNature",
        visual: 215,
    },
    Companion {
        id: 60014,
        name: "Summon Companion: Ancona",
        icon: "Interface\\Icons\\INV_Crate_02",
        visual: 353,
    },
    Companion {
        id: 60015,
        name: "Summon Companion: Cockroach",
        icon: "Interface\\Icons\\Spell_Shadow_CarrionSwarm",
        visual: 353,
    },
    Companion {
        id: 60016,
        name: "Summon Companion: Dark Whelpling",
        icon: "Interface\\Icons\\INV_Misc_Head_Dragon_01",
        visual: 215,
    },
    Companion {
        id: 60017,
        name: "Summon Companion: Crimson Whelpling",
        icon: "Interface\\Icons\\INV_Misc_Head_Dragon_01",
        visual: 215,
    },
    Companion {
        id: 60018,
        name: "Summon Companion: Emerald Whelpling",
        icon: "Interface\\Icons\\INV_Misc_Head_Dragon_01",
        visual: 215,
    },
    Companion {
        id: 60019,
        name: "Summon Companion: Wood Frog",
        icon: "Interface\\Icons\\INV_Crate_02",
        visual: 353,
    },
    Companion {
        id: 60020,
        name: "Summon Companion: Tree Frog",
        icon: "Interface\\Icons\\INV_Crate_02",
        visual: 353,
    },
    Companion {
        id: 60021,
        name: "Summon Companion: Hawk Owl",
        icon: "Interface\\Icons\\Ability_EyeOfTheOwl",
        visual: 215,
    },
    Companion {
        id: 60022,
        name: "Summon Companion: Great Horned Owl",
        icon: "Interface\\Icons\\Ability_EyeOfTheOwl",
        visual: 215,
    },
    Companion {
        id: 60023,
        name: "Summon Companion: Prairie Dog",
        icon: "Interface\\Icons\\Ability_Hunter_BeastCall",
        visual: 353,
    },
    Companion {
        id: 60024,
        name: "Summon Companion: Snowshoe Rabbit",
        icon: "Interface\\Icons\\INV_Crate_02",
        visual: 353,
    },
    Companion {
        id: 60025,
        name: "Summon Companion: Black Kingsnake",
        icon: "Interface\\Icons\\Spell_Nature_GuardianWard",
        visual: 353,
    },
    Companion {
        id: 60026,
        name: "Summon Companion: Brown Snake",
        icon: "Interface\\Icons\\Spell_Nature_GuardianWard",
        visual: 353,
    },
    Companion {
        id: 60027,
        name: "Summon Companion: Crimson Snake",
        icon: "Interface\\Icons\\Spell_Nature_GuardianWard",
        visual: 353,
    },
    Companion {
        id: 60028,
        name: "Summon Companion: Mechanical Chicken",
        icon: "Interface\\Icons\\Spell_Magic_PolymorphChicken",
        visual: 353,
    },
    Companion {
        id: 60029,
        name: "Summon Companion: Farm Chicken",
        icon: "Interface\\Icons\\INV_Egg_02",
        visual: 353,
    },
    Companion {
        id: 60030,
        name: "Summon Companion: Bomb",
        icon: "Interface\\Icons\\INV_Misc_Bomb_04",
        visual: 353,
    },
    Companion {
        id: 60031,
        name: "Summon Companion: Robot",
        icon: "Interface\\Icons\\INV_Misc_Idol_02",
        visual: 353,
    },
    Companion {
        id: 60032,
        name: "Summon Companion: Sprite Darter Hatchling",
        icon: "Interface\\Icons\\INV_Egg_02",
        visual: 353,
    },
    Companion {
        id: 60033,
        name: "Summon Companion: Corrupted Kitten",
        icon: "Interface\\Icons\\INV_Box_PetCarrier_01",
        visual: 353,
    },
    Companion {
        id: 60034,
        name: "Summon Companion: Worg Pup",
        icon: "Interface\\Icons\\INV_Box_PetCarrier_01",
        visual: 353,
    },
    Companion {
        id: 60035,
        name: "Summon Companion: Smolderweb",
        icon: "Interface\\Icons\\INV_Box_Birdcage_01",
        visual: 353,
    },
    Companion {
        id: 60036,
        name: "Summon Companion: Blood Parrot",
        icon: "Interface\\Icons\\INV_Helmet_66",
        visual: 215,
    },
    Companion {
        id: 60037,
        name: "Summon Companion: Panda",
        icon: "Interface\\Icons\\INV_Belt_05",
        visual: 353,
    },
    Companion {
        id: 60038,
        name: "Summon Companion: Diablo",
        icon: "Interface\\Icons\\INV_DiabloStone",
        visual: 353,
    },
    Companion {
        id: 60039,
        name: "Summon Companion: Zergling",
        icon: "Interface\\Icons\\Spell_Shadow_SummonFelHunter",
        visual: 353,
    },
    Companion {
        id: 60040,
        name: "Summon Companion: Lifelike Toad",
        icon: "Interface\\Icons\\INV_Misc_MonsterHead_03",
        visual: 353,
    },
    Companion {
        id: 60041,
        name: "Summon Companion: Ar'lia",
        icon: "Interface\\Icons\\INV_Jewelry_Ring_25",
        visual: 335,
    },
    Companion {
        id: 60042,
        name: "Summon Companion: Orcish Orphan",
        icon: "Interface\\Icons\\Ability_Hunter_BeastCall",
        visual: 6744,
    },
    Companion {
        id: 60043,
        name: "Summon Companion: Human Orphan",
        icon: "Interface\\Icons\\Ability_Hunter_BeastCall",
        visual: 6744,
    },
    Companion {
        id: 60044,
        name: "Summon Companion: Albino Snapjaw",
        icon: "Interface\\Icons\\INV_Egg_03",
        visual: 353,
    },
    Companion {
        id: 60045,
        name: "Summon Companion: Loggerhead Snapjaw",
        icon: "Interface\\Icons\\INV_Egg_02",
        visual: 353,
    },
    Companion {
        id: 60046,
        name: "Summon Companion: Olive Snapjaw",
        icon: "Interface\\Icons\\INV_Egg_02",
        visual: 353,
    },
    Companion {
        id: 60047,
        name: "Summon Companion: Leatherback Snapjaw",
        icon: "Interface\\Icons\\INV_Egg_03",
        visual: 353,
    },
    Companion {
        id: 60048,
        name: "Summon Companion: Hawksbill Snapjaw",
        icon: "Interface\\Icons\\INV_Egg_03",
        visual: 353,
    },
    Companion {
        id: 60049,
        name: "Summon Companion: Tiny Red Dragon",
        icon: "Interface\\Icons\\INV_Misc_Orb_05",
        visual: 353,
    },
    Companion {
        id: 60050,
        name: "Summon Companion: Tiny Green Dragon",
        icon: "Interface\\Icons\\INV_Misc_Orb_01",
        visual: 353,
    },
    Companion {
        id: 60051,
        name: "Summon Companion: Jubling",
        icon: "Interface\\Icons\\INV_Egg_04",
        visual: 353,
    },
    Companion {
        id: 60052,
        name: "Summon Companion: Murky",
        icon: "Interface\\Icons\\INV_Egg_03",
        visual: 353,
    },
    Companion {
        id: 60053,
        name: "Summon Companion: Murki",
        icon: "Interface\\Icons\\INV_Egg_03",
        visual: 353,
    },
    Companion {
        id: 60054,
        name: "Summon Companion: Disgusting Oozeling",
        icon: "Interface\\Icons\\Ability_Creature_Poison_05",
        visual: 353,
    },
    Companion {
        id: 60055,
        name: "Summon Companion: Baby Shark",
        icon: "Interface\\Icons\\INV_Drink_19",
        visual: 353,
    },
    Companion {
        id: 60056,
        name: "Summon Companion: Tranquil Mechanical Yeti",
        icon: "Interface\\Icons\\Ability_Hunter_Pet_Gorilla",
        visual: 353,
    },
    Companion {
        id: 60057,
        name: "Summon Companion: Gurky",
        icon: "Interface\\Icons\\INV_Egg_03",
        visual: 353,
    },
    Companion {
        id: 60058,
        name: "Summon Companion: Peddlefeet",
        icon: "Interface\\Icons\\INV_Ammo_Arrow_02",
        visual: 0,
    },
    Companion {
        id: 60059,
        name: "Summon Companion: Terky",
        icon: "Interface\\Icons\\INV_Egg_03",
        visual: 353,
    },
    Companion {
        id: 60060,
        name: "Summon Companion: Poley",
        icon: "Interface\\Icons\\INV_Belt_09",
        visual: 353,
    },
    Companion {
        id: 60061,
        name: "Summon Companion: Speedy",
        icon: "Interface\\Icons\\INV_Crate_03",
        visual: 353,
    },
    Companion {
        id: 60062,
        name: "Summon Companion: Mr. Wiggles",
        icon: "Interface\\Icons\\INV_Belt_25",
        visual: 353,
    },
    Companion {
        id: 60063,
        name: "Summon Companion: Whiskers",
        icon: "Interface\\Icons\\INV_Box_PetCarrier_01",
        visual: 353,
    },
    Companion {
        id: 60064,
        name: "Summon Companion: Spirit of Summer",
        icon: "Interface\\Icons\\INV_Potion_33",
        visual: 353,
    },
    Companion {
        id: 60065,
        name: "Summon Companion: White Tiger Cub",
        icon: "Interface\\Icons\\INV_Misc_Head_Tiger_01",
        visual: 215,
    },
    Companion {
        id: 60066,
        name: "Summon Companion: Hippogryph Hatchling",
        icon: "Interface\\Icons\\INV_Egg_02",
        visual: 215,
    },
];

/// Installs every reserved-range display row into `catalog`, called once after `Spell.dbc` loads
/// (`ui_action::load_spells`). Teach spells get a visual-only row ([`teach_spell_ids`]'s doc): they
/// are cast directly by the item's on-use effect (`Spell::EffectLearnSpell`, server-side) and never
/// land in `character_spell`, so they never reach the spellbook's known-spell list regardless of
/// having a catalog row - only [`in_spellbook`](benilla_formats::SpellDisplay::in_spellbook)-gated,
/// already-known spells ever show up there.
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
    for companion in COMPANIONS {
        debug_assert!(
            COMPANION_SPELL_RANGE.contains(&companion.id),
            "{} is outside COMPANION_SPELL_RANGE - the Pets tab and click-to-cast would both miss it",
            companion.id
        );
        catalog.insert(
            companion.id,
            SpellDisplay {
                id: companion.id,
                name: companion.name.to_string(),
                icon: Some(companion.icon.to_string()),
                visual: companion.visual,
                effects: [EFFECT_SUMMON_CRITTER, 0, 0],
                implicit_target_a1: TARGET_UNIT_CASTER,
                // `targets: 0` plus the implicit-target arm above together ask for no target at
                // all (`cast_target_mask`) - casting_time_index 0 resolves to instant via the
                // same "missing row reads 0" fallback `Spells::cast_time_unclamped_ms` uses.
                ..Default::default()
            },
        );
    }
}
