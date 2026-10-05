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

/// One companion's catalog row, hand-written one per entry (this range has no DBC source to
/// generate from).
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

const COMPANIONS: &[Companion] = &[Companion {
    id: 60002,
    name: "Summon Companion: Black Tabby",
    icon: "Interface\\Icons\\INV_Box_PetCarrier_01",
    visual: 353,
}];

/// Installs every reserved-range display row into `catalog`, called once after `Spell.dbc` loads
/// (`ui_action::load_spells`). The Teach spell (60001) is deliberately not given a row: it is cast
/// directly by the item's on-use effect (`Spell::EffectLearnSpell`, server-side) and never lands in
/// `character_spell`, so it never reaches the spellbook's known-spell list to begin with.
pub(crate) fn install(catalog: &mut SpellCatalog) {
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
