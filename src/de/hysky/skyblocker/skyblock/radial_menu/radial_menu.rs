//! Ported from de.hysky.skyblocker.skyblock.radialMenu.RadialMenu by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config::SkyblockerConfig;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: public abstract class RadialMenu
#[derive(Clone)]
pub struct RadialMenuBase {}

impl RadialMenuBase {
    /// java: the implicit no-argument constructor (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_in<__R>(__wrap: impl FnOnce(Self) -> (compat::Shared<dyn RadialMenu>, __R)) -> (compat::Shared<dyn RadialMenu>, __R) {
        let this = Self {
        };
        let (__h, __r) = __wrap(this);
        let __this = __h;
        (__this, __r)
    }

}

/// java: the virtual methods of RadialMenu (implemented by RadialMenuBase and every subclass)
pub trait RadialMenu: compat::DynAny {
    fn radial_menu_base(&self) -> &RadialMenuBase;
    fn radial_menu_base_mut(&mut self) -> &mut RadialMenuBase;
    fn as_any(&self) -> &dyn std::any::Any;

    /// java: protected abstract Component getTitle(Component title); (dispatch: the body for this class)
    fn get_title(&self) -> fn(&compat::Shared<dyn RadialMenu>, &MutableComponent) -> MutableComponent;

    /// java: protected abstract boolean titleMatches(String title); (dispatch: the body for this class)
    fn title_matches(&self) -> fn(&compat::Shared<dyn RadialMenu>, &str) -> bool;

    /// java: protected abstract boolean itemMatches(int slotId, ItemStack stack); (dispatch: the body for this class)
    fn item_matches(&self) -> fn(&compat::Shared<dyn RadialMenu>, i32, &mcrust::net::minecraft::world::item::item_stack::ItemStack) -> bool;

    /// java: public abstract String getConfigId(); (dispatch: the body for this class)
    fn get_config_id(&self) -> fn(&compat::Shared<dyn RadialMenu>) -> String;

    /// java: public abstract int remapClickSlotButton(int originalButton, int slotId); (dispatch: the body for this class)
    fn remap_click_slot_button(&self) -> fn(&compat::Shared<dyn RadialMenu>, i32, i32) -> i32;

    /// java: public abstract int clickSlotOffset(int slotId); (dispatch: the body for this class)
    fn click_slot_offset(&self) -> fn(&compat::Shared<dyn RadialMenu>, i32) -> i32;

    /// java: public abstract String[] getNavigationItemNames(); (dispatch: the body for this class)
    fn get_navigation_item_names(&self) -> fn(&compat::Shared<dyn RadialMenu>) -> Vec<String>;

    /// java: protected boolean getEnabled() (dispatch: the body for this class)
    fn get_enabled(&self) -> fn(&compat::Shared<dyn RadialMenu>) -> bool {
        <dyn RadialMenu>::get_enabled_body
    }

    /// java: public Option<Boolean> getOption(SkyblockerConfig config) (dispatch: the body for this class)
    fn get_option(&self) -> fn(&compat::Shared<dyn RadialMenu>, &compat::Shared<SkyblockerConfig>) -> Arc<dyn dandelion::net::azureaaron::dandelion::api::option::Option<bool>> {
        <dyn RadialMenu>::get_option_body
    }

}

impl dyn RadialMenu {
    /// java: protected boolean getEnabled()
    pub fn get_enabled_body(__this: &compat::Shared<dyn RadialMenu>) -> bool {
        return (SkyblockerConfigManager::get().ui_and_visuals.radial_menu.enabled && SkyblockerConfigManager::get().ui_and_visuals.radial_menu.enabled_menus.get(&({ let __f = compat::dispatch(__this, |__t| __t.get_config_id()); __f(__this) })).cloned().unwrap_or_else(|| false));
    }

    /// java: public Option<Boolean> getOption(SkyblockerConfig config)
    pub fn get_option_body(__this: &compat::Shared<dyn RadialMenu>, config: &compat::Shared<SkyblockerConfig>) -> Arc<dyn dandelion::net::azureaaron::dandelion::api::option::Option<bool>> {
        use dandelion::net::azureaaron::dandelion::api::option::{self, Builder as _};
        // java: return Option.<Boolean>createBuilder()
        //       .name(Component.translatable("skyblocker.config.uiAndVisuals.radialMenu." + getConfigId()))
        //       .binding(false,
        //               () -> config.uiAndVisuals.radialMenu.enabledMenus.getOrDefault(getConfigId(), false),
        //               newValue -> config.uiAndVisuals.radialMenu.enabledMenus.put(getConfigId(), newValue.booleanValue()))
        //       .controller(ConfigUtils.createBooleanController())
        //       .build();
        let config_id: String = { let __f = compat::dispatch(__this, |__t| __t.get_config_id()); __f(__this) };
        let (get_id, get_config) = (config_id.clone(), config.clone());
        let (set_id, set_config) = (config_id.clone(), config.clone());
        return option::create_builder::<bool>()
            .name(mcrust::net::minecraft::network::chat::component::translatable(&format!("skyblocker.config.uiAndVisuals.radialMenu.{}", config_id)))
            .binding(
                false,
                move || compat::read(&get_config, |c| c.ui_and_visuals.radial_menu.enabled_menus.get(&get_id).cloned().unwrap_or(false)),
                move |new_value: bool| compat::write(&set_config, |c| {
                    c.ui_and_visuals.radial_menu.enabled_menus.insert(set_id.clone(), new_value);
                }),
            )
            .controller(crate::de::hysky::skyblocker::config::config_utils::ConfigUtils::create_boolean_controller())
            .build();
    }

}
