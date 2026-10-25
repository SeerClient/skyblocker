//! Ported from net.azureaaron.renderchest.impl.RenderStateKeys (Render Chest 1.0.3).
//!
//! The two keys are Fabric API `RenderStateDataKey`s - data a mod attaches to a render state (`FabricRenderState
//! .setData` / `getDataOrDefault`, injected into `EntityRenderState` and `LevelRenderState`). The game's
//! `EntityRenderState` / `LevelRenderState` carry no such per-state data map, so a key has nowhere to store its value;
//! the keys are kept (their names are the debug names Java gives them) for the code that reads them. See GAPS_S.md #9.

/// java: `RenderStateDataKey<T>` - a typed key with its debug name
pub struct RenderStateDataKey<T> {
    /// java: `RenderStateDataKey.create(() -> "..")` - the debug name
    pub debug_name: &'static str,
    _type: std::marker::PhantomData<fn() -> T>,
}

impl<T> RenderStateDataKey<T> {
    /// java: `RenderStateDataKey.create(Supplier<String> debugName)`
    pub const fn create(debug_name: &'static str) -> RenderStateDataKey<T> {
        RenderStateDataKey { debug_name, _type: std::marker::PhantomData }
    }
}

/// java: public static final RenderStateDataKey<Integer> ENTITY_CUSTOM_GLOW_COLOUR = RenderStateDataKey.create(() -> "Render Chest custom glow colour");
pub const ENTITY_CUSTOM_GLOW_COLOUR: RenderStateDataKey<i32> = RenderStateDataKey::create("Render Chest custom glow colour");
/// java: public static final RenderStateDataKey<Boolean> FRAME_USES_CUSTOM_GLOW = RenderStateDataKey.create(() -> "Render Chest frame uses custom glow");
pub const FRAME_USES_CUSTOM_GLOW: RenderStateDataKey<bool> = RenderStateDataKey::create("Render Chest frame uses custom glow");
