//! Ported from de.hysky.skyblocker.utils.hoveredItem.HoveredItemStackUtils by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::hovered_item::hovered_item_stack_provider::HoveredItemStackProvider;

/// java: public class HoveredItemStackUtils
#[derive(Clone)]
pub struct HoveredItemStackUtils {}

impl HoveredItemStackUtils {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static @Nullable HoveredItemStackProvider getProvider(Screen screen)
    pub fn get_provider(screen: &mcrust::net::minecraft::client::gui::screens::screen::Screen) -> Option<compat::Shared<dyn HoveredItemStackProvider>> {
        // java: if (screen instanceof HoveredItemStackProvider provider) return provider;
        //       else if (screen.getFocused() instanceof HoveredItemStackProvider provider) return provider;
        // rust: port gap - the port hands this method the `Screen` husk; neither it nor its focused child (an index into
        // its widget list) can be asked whether the mod object behind it implements the mod's own interface
        // (`HoveredItemStackProvider`: SkyblockInventoryScreen, AccessoriesHelperWidget, ResultButton, MuseumManager,
        // InventoryWidget). GAPS_batchA.md
        let _ = screen;
        return None;
    }

    /// java: public static @Nullable ItemStack getHoveredItemStack(Screen screen)
    pub fn get_hovered_item_stack(screen: &mcrust::net::minecraft::client::gui::screens::screen::Screen) -> Option<mcrust::net::minecraft::world::item::item_stack::ItemStack> {
        let Some(mut provider) = HoveredItemStackUtils::get_provider(screen) else {
            return None;
        };
        return ({ let __f = compat::dispatch(&provider, |__t| __t.get_focused_item()); __f(&provider) });
    }

}
