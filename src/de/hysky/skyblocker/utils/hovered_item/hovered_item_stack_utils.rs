//! Ported from de.hysky.skyblocker.utils.hoveredItem.HoveredItemStackUtils by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::hovered_item::hovered_item_stack_provider::HoveredItemStackProvider;
use crate::de::hysky::skyblocker::skyblock::accessories::accessories_helper_widget::ResultButton;
use crate::de::hysky::skyblocker::skyblock::museum::museum_manager::MuseumManager;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::widgets::inventory_widget::InventoryWidget;

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
        // `screen instanceof HoveredItemStackProvider`: of the five providers only `SkyblockInventoryScreen` is a
        // screen, and it has no game hosting yet (no `ModScreen`/`ModContainerScreen` adapter in its own file, which
        // batch T does not own) - so no live screen can be one; the arm is the focused child below.
        // `screen.getFocused() instanceof HoveredItemStackProvider`: the focused child is the game widget at
        // `get_focused()`; a mod widget carries its object in its slot (`WidgetAdapter` / `ObjectWidgetSlot`).
        let focused = screen.get_focused()?;
        let slot = screen.widget(focused).mod_widget()?;
        if let Some(o) = slot.with::<compat::WidgetAdapter<ResultButton>, _>(|a| a.object().clone()) {
            return Some(o as compat::Shared<dyn HoveredItemStackProvider>);
        }
        if let Some(o) = slot.with::<compat::WidgetAdapter<InventoryWidget>, _>(|a| a.object().clone()) {
            return Some(o as compat::Shared<dyn HoveredItemStackProvider>);
        }
        if let Some(o) = compat::batch_t_object_widget_object::<MuseumManager>(&slot) {
            return Some(o as compat::Shared<dyn HoveredItemStackProvider>);
        }
        // `AccessoriesHelperWidget` likewise has no `ModWidget` hosting in its own file (it is never added to a
        // screen: `attach_to_screen` is a recorded gap in GAPS_batchG), so it cannot be the focused child.
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
