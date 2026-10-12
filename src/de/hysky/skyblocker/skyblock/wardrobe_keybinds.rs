//! Ported from de.hysky.skyblocker.skyblock.WardrobeKeybinds by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::item::slottext::simple_slot_text_adder::SimpleSlotTextAdder;
use crate::de::hysky::skyblocker::skyblock::item::slottext::simple_slot_text_adder::SimpleSlotTextAdderBase;
use crate::de::hysky::skyblocker::skyblock::item::slottext::slot_text;
use crate::de::hysky::skyblocker::skyblock::item::slottext::slot_text::SlotText;
use crate::de::hysky::skyblocker::skyblock::loadout_keybinds;
use crate::de::hysky::skyblocker::utils::container::container_matcher::ContainerMatcher;
use crate::de::hysky::skyblocker::utils::container::regex_container_matcher::RegexContainerMatcher;
use crate::de::hysky::skyblocker::utils::container::regex_container_matcher::RegexContainerMatcherBase;
use crate::de::hysky::skyblocker::utils::container::slot_text_adder::ConfigInformation;
use crate::de::hysky::skyblocker::utils::container::slot_text_adder::SlotTextAdder;
use mcrust::net::minecraft::client::gui::screens::inventory::abstract_container_screen::AbstractContainerScreen;
use mcrust::net::minecraft::client::key_mapping;
use mcrust::net::minecraft::world::inventory::slot::Slot;
use mcrust::net::minecraft::client::gui::screens::inventory::abstract_container_screen::ContainerScreenHost as _;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: public static final WardrobeKeybinds INSTANCE = new WardrobeKeybinds();
pub static INSTANCE: compat::MainThread<compat::Shared<WardrobeKeybinds>> = compat::MainThread::new(|| WardrobeKeybinds::new());

/// java: public class WardrobeKeybinds extends SimpleSlotTextAdder
#[derive(Clone)]
pub struct WardrobeKeybinds {
    /// java: extends SimpleSlotTextAdder
    pub base: SimpleSlotTextAdderBase,
}

impl std::ops::Deref for WardrobeKeybinds {
    type Target = SimpleSlotTextAdderBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for WardrobeKeybinds {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl WardrobeKeybinds {
    /// java: private WardrobeKeybinds()
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = SimpleSlotTextAdderBase::new_in("^(?:\\(\\d+/\\d+\\) )?(Armor Sets|Equipment Sets)$".to_string(), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
            });
            ((__h.clone() as compat::Shared<dyn SimpleSlotTextAdder>), __h)
        });
        __this
    }

    /// java: @Init public static void init()
    pub fn init() {
        screen_events::AFTER_INIT.register(Arc::new(|screen: &mut Screen, width: i32, height: i32| {
            // TODO(fabric2seer): an `instanceof` pattern under `||` / `!`
            //   java: if (!(screen instanceof AbstractContainerScreen<?> containerScreen) || !INSTANCE.test(containerScreen) || !INSTANCE.isEnabled() || client.gameMode == null) return;
            screen_keyboard_events::ALLOW_KEY_PRESS.register(Arc::new(|action: i32, key_event: &KeyEvent| {
                // TODO(fabric2seer): `client`: Fabric passes the Minecraft here; this SeerClient callback does not (take it from ClientTickEvents or a hook)
                //   java: return allowInput(client, containerScreen, keybinding -> keybinding.matches(keyInput));
                true
            }));
            screen_mouse_events::ALLOW_MOUSE_CLICK.register(Arc::new(|screen: &mut Screen, (mouse_x, mouse_y): (i32, i32), pressed: bool| {
                // TODO(fabric2seer): `client`: Fabric passes the Minecraft here; this SeerClient callback does not (take it from ClientTickEvents or a hook)
                //   java: return allowInput(client, containerScreen, keybinding -> keybinding.matchesMouse(click));
                true
            }));
        }));
    }

    /// java: private static boolean allowInput(Minecraft minecraft, AbstractContainerScreen<?> containerScreen, Predicate<KeyMapping> predicate)
    pub fn allow_input(minecraft: &mut mcrust::net::minecraft::client::minecraft::Minecraft, container_screen: &AbstractContainerScreen, predicate: Arc<dyn Fn(mcrust::net::minecraft::client::key_mapping::KeyMapping) -> bool>) -> bool {
        if false /* java: minecraft.game_mode == null: never null here */ || mcrust::mcrust::modding::game::with_player(|_| ()).is_none() {
            return true;
        }
        let mut found: bool = false;
        let mut i: i32;
        {
            i = 0;
            while i < (loadout_keybinds::LOADOUT_KEY_MAPPINGS.get_mut().len() as i32) {
                if (predicate)(loadout_keybinds::LOADOUT_KEY_MAPPINGS.get_mut()[(i) as usize].clone()) {
                    found = true;
                    break;
                }
                i += 1;
            }
        }
        if !found {
            return true;
        }
        i = i.wrapping_add(9i32.wrapping_mul(4));
        // TODO(fabric2seer): `getSlot` on a Object
        //   java: ItemStack itemStack = containerScreen.getMenu().getSlot(i).getItem();
        let mut item_stack: mcrust::net::minecraft::world::item::item_stack::ItemStack = todo!("fabric2seer: the initialiser of `itemStack` (TODO above)");
        if !(item_stack.is_item(mcrust::net::minecraft::world::item::items::dye().gray())) && !(item_stack.is_item(mcrust::net::minecraft::world::item::items::dye().lime())) {
            return true;
        }
        // TODO(fabric2seer): field `containerId` of a Object
        //   java: minecraft.gameMode.handleContainerInput(containerScreen.getMenu().containerId, i, ContainerUtils.getContainerClickButton(InputConstants.MOUSE_BUTTON_LEFT), ContainerInput.PICKUP, minecraft.player);
        return false;
    }

}

impl SimpleSlotTextAdder for WardrobeKeybinds {
    fn simple_slot_text_adder_base(&self) -> &SimpleSlotTextAdderBase {
        &self.base
    }
    fn simple_slot_text_adder_base_mut(&mut self) -> &mut SimpleSlotTextAdderBase {
        &mut self.base
    }

}

impl RegexContainerMatcher for WardrobeKeybinds {
    fn regex_container_matcher_base(&self) -> &RegexContainerMatcherBase {
        &self.base.base
    }
    fn regex_container_matcher_base_mut(&mut self) -> &mut RegexContainerMatcherBase {
        &mut self.base.base
    }

}

impl ContainerMatcher for WardrobeKeybinds {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public boolean isEnabled() (dispatch: the body for this class)
    fn is_enabled(&self) -> fn(&compat::Shared<dyn ContainerMatcher>) -> bool {
        WardrobeKeybinds::is_enabled_dyn
    }

    /// java: @Override public boolean test(Screen screen) (dispatch: the body for this class)
    fn test(&self) -> fn(&compat::Shared<dyn ContainerMatcher>, &mcrust::net::minecraft::client::gui::screens::screen::Screen) -> bool {
        <dyn RegexContainerMatcher>::test_from::<Self>
    }

}

impl SlotTextAdder for WardrobeKeybinds {
    /// java: @Override public List<SlotText> getText(@Nullable Slot slot, ItemStack stack, int slotId) (dispatch: the body for this class)
    fn get_text(&self) -> fn(&compat::Shared<dyn SlotTextAdder>, Option<Slot>, &mcrust::net::minecraft::world::item::item_stack::ItemStack, i32) -> Vec<SlotText> {
        WardrobeKeybinds::get_text_dyn
    }

    /// java: @Override public @Nullable ConfigInformation getConfigInformation() (dispatch: the body for this class)
    fn get_config_information(&self) -> fn(&compat::Shared<dyn SlotTextAdder>) -> Option<ConfigInformation> {
        <dyn SimpleSlotTextAdder>::get_config_information_from::<Self>
    }

}

impl WardrobeKeybinds {
    /// java: @Override public boolean isEnabled()
    pub fn is_enabled_impl(__this: &compat::Shared<Self>) -> bool {
        return SkyblockerConfigManager::get().helpers.enable_wardrobe_helper;
    }

    /// WardrobeKeybinds.isEnabled as the dispatch of ContainerMatcher.isEnabled
    pub fn is_enabled_dyn(__this: &compat::Shared<dyn ContainerMatcher>) -> bool {
        Self::is_enabled_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public List<SlotText> getText(@Nullable Slot slot, ItemStack stack, int slotId)
    pub fn get_text_impl(__this: &compat::Shared<Self>, mut slot: Option<Slot>, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, mut slot_id: i32) -> Vec<SlotText> {
        if !(stack.is_item(mcrust::net::minecraft::world::item::items::dye().gray())) && !(stack.is_item(mcrust::net::minecraft::world::item::items::dye().lime())) {
            return vec![];
        }
        if !(slot_id >= 36 && slot_id <= 44) {
            return vec![];
        }
        let mut loadout_index: i32 = (slot_id.wrapping_sub(35)).wrapping_sub(1);
        // TODO(fabric2seer): `get_translated_key_message` takes 2 argument(s) in the port (`name: &str, key_name: &dyn Fn(i32, i32) -> Option<String>`), Java passes 0
        // rust: return SlotText::bottom_left_list(&compat::component_of(key_mapping::get_translated_key_message()).plain_copy().with_color(slot_text::MID_BLUE));
        return todo!("fabric2seer: see the TODO above");
    }

    /// WardrobeKeybinds.getText as the dispatch of SlotTextAdder.getText
    pub fn get_text_dyn(__this: &compat::Shared<dyn SlotTextAdder>, __p0: Option<Slot>, __p1: &mcrust::net::minecraft::world::item::item_stack::ItemStack, __p2: i32) -> Vec<SlotText> {
        Self::get_text_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2)
    }

}
