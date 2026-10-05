//! Display entries for the reserved custom spell-ID range (60000-60999, see ARCHITECTURE.md)
//! that `Spell.dbc` was never going to carry, since these abilities only exist on our own server
//! fork. Without an entry here, [`benilla_formats::SpellCatalog::get`] returns `None` for them and
//! the spellbook's add-gate (`ui_spellbook.rs`'s `build_book`) silently drops them - a learned
//! spell with no DBC row is otherwise invisible no matter how correctly the server taught it.
//!
//! This is the milestone-1 minimum: it earns these spells a name, icon-less button and a slot in
//! the General tab, cast by dragging to an action bar like any other spell (standard spellbook
//! behavior - `PickupSpell`, not a direct cast). The dedicated "Pets" tab with click-to-cast
//! (ARCHITECTURE.md build-order steps 2-3) is follow-up work, not this.

use benilla_formats::{SpellCatalog, SpellDisplay};

/// `TARGET_UNIT_CASTER` (`cast_target.rs::cast_target_mask`): clears the explicit-target-required
/// bit, so clicking the spell fires immediately with no reticle - matching the server's own
/// `effectImplicitTargetA1 = 1` on these spells (see `server/sql/migrations/20261004120000_world.sql`).
const TARGET_UNIT_CASTER: u32 = 1;

/// `97`, this build's own stand-in for `SPELL_EFFECT_SUMMON_CRITTER`: no client-side logic branches
/// on it today (benilla has no companion-pet concept yet - ARCHITECTURE.md step 4), it is set only
/// so a display row's `effects[0]` isn't silently wrong if something comes to depend on it later.
const EFFECT_SUMMON_CRITTER: u32 = 97;

/// One companion's catalog row: the student developing this range by hand, one row per entry.
struct Companion {
    id: u32,
    name: &'static str,
}

const COMPANIONS: &[Companion] = &[Companion {
    id: 60002,
    name: "Summon Companion: Black Tabby",
}];

/// Installs every reserved-range display row into `catalog`, called once after `Spell.dbc` loads
/// (`ui_action::load_spells`). The Teach spell (60001) is deliberately not given a row: it is cast
/// directly by the item's on-use effect (`Spell::EffectLearnSpell`, server-side) and never lands in
/// `character_spell`, so it never reaches the spellbook's known-spell list to begin with.
pub(crate) fn install(catalog: &mut SpellCatalog) {
    for companion in COMPANIONS {
        catalog.insert(
            companion.id,
            SpellDisplay {
                id: companion.id,
                name: companion.name.to_string(),
                effects: [EFFECT_SUMMON_CRITTER, 0, 0],
                implicit_target_a1: TARGET_UNIT_CASTER,
                // `targets: 0` plus the implicit-target arm above together ask for no target at
                // all (`cast_target_mask`) - icon stays `None` (the doc'd "not in SpellIcon.dbc"
                // case other code already handles), casting_time_index 0 resolves to instant via
                // the same "missing row reads 0" fallback `Spells::cast_time_unclamped_ms` uses.
                ..Default::default()
            },
        );
    }
}
