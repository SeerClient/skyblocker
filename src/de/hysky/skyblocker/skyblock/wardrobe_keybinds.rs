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

/// Whether the screen `AFTER_INIT` last saw is a container screen this adder matches and is enabled for (Fabric's
/// per-screen listener registration, see `init`).
pub static SCREEN_ACCEPTED: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));
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
        // Fabric registers the two input listeners on the screen instance (`ScreenKeyboardEvents.allowKeyPress(screen)`),
        // so they live as long as that screen; the port's input events are global, so they are registered once and
        // answer for the screen `AFTER_INIT` last accepted (`SCREEN_ACCEPTED`, reset by every screen's init).
        screen_events::AFTER_INIT.register(Arc::new(|screen: &mut Screen, width: i32, height: i32| {
            //   java: if (!(screen instanceof AbstractContainerScreen<?> containerScreen) || !INSTANCE.test(containerScreen) || !INSTANCE.isEnabled() || client.gameMode == null) return;
            let is_container_screen = matches!(compat::minecraft().gui.screen_kind, mcrust::net::minecraft::client::gui::gui::ScreenKind::Container);
            let accepted = is_container_screen && {
                let __r = &compat::view(INSTANCE.get(), |a| a as compat::Shared<dyn ContainerMatcher>);
                let __test = compat::dispatch(__r, |__t| __t.test());
                let __enabled = compat::dispatch(__r, |__t| __t.is_enabled());
                __test(__r, screen) && __enabled(__r)
            };
            compat::set_state(&SCREEN_ACCEPTED, accepted);
        }));
        //   java: ScreenKeyboardEvents.allowKeyPress(containerScreen).register((_, keyInput) -> allowInput(client, containerScreen, keybinding -> keybinding.matches(keyInput)));
        screen_keyboard_events::ALLOW_KEY_PRESS.register(Arc::new(|action: i32, key_event: &KeyEvent| {
            if action == 0 || !compat::get_state(&SCREEN_ACCEPTED) {
                return true;
            }
            let key_event = *key_event;
            WardrobeKeybinds::allow_input(Arc::new(move |keybinding: mcrust::net::minecraft::client::key_mapping::KeyMapping| key_mapping::matches(keybinding.get_name(), &key_event)))
        }));
        //   java: ScreenMouseEvents.allowMouseClick(containerScreen).register((_, click) -> allowInput(client, containerScreen, keybinding -> keybinding.matchesMouse(click)));
        // rust: port gap - `screen_mouse_events::ALLOW_MOUSE_CLICK` is not handed the button (`MouseButtonEvent`), so
        // `keybinding.matchesMouse(click)` cannot be asked: a loadout bound to a mouse button does nothing (GAPS_K)
    }

    /// java: private static boolean allowInput(Minecraft minecraft, AbstractContainerScreen<?> containerScreen, Predicate<KeyMapping> predicate)
    ///
    /// (`minecraft` is `compat::minecraft()`, and `containerScreen.getMenu()` is the player's open menu, which the screen
    /// was opened over)
    pub fn allow_input(predicate: Arc<dyn Fn(mcrust::net::minecraft::client::key_mapping::KeyMapping) -> bool>) -> bool {
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
        //   java: ItemStack itemStack = containerScreen.getMenu().getSlot(i).getItem();
        let Some((container_id, mut item_stack)) = compat::with_open_menu(|menu, containers| (menu.container_id, menu.slots[i as usize].get_item(containers).duplicate())) else {
            return true;
        };
        if !(item_stack.is_item(mcrust::net::minecraft::world::item::items::dye().gray())) && !(item_stack.is_item(mcrust::net::minecraft::world::item::items::dye().lime())) {
            return true;
        }
        //   java: minecraft.gameMode.handleContainerInput(containerScreen.getMenu().containerId, i, ContainerUtils.getContainerClickButton(InputConstants.MOUSE_BUTTON_LEFT), ContainerInput.PICKUP, minecraft.player);
        handle_container_input(container_id, i, crate::de::hysky::skyblocker::utils::container_utils::ContainerUtils::get_container_click_button(0), mcrust::net::minecraft::world::inventory::container_input::ContainerInput::Pickup);
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
        //   java: return SlotText.bottomLeftList(LoadoutKeybinds.LOADOUT_KEY_MAPPINGS[loadoutIndex].getTranslatedKeyMessage().plainCopy().withColor(SlotText.MID_BLUE));
        let name: &'static str = loadout_keybinds::LOADOUT_KEY_MAPPINGS.get()[loadout_index as usize].get_name();
        let message = key_mapping::get_translated_key_message(name, &|key: i32, _scancode: i32| compat::minecraft().key_names.get(&key).cloned());
        return SlotText::bottom_left_list(&compat::component_of(message).plain_copy().with_color(slot_text::MID_BLUE));
    }

    /// WardrobeKeybinds.getText as the dispatch of SlotTextAdder.getText
    pub fn get_text_dyn(__this: &compat::Shared<dyn SlotTextAdder>, __p0: Option<Slot>, __p1: &mcrust::net::minecraft::world::item::item_stack::ItemStack, __p2: i32) -> Vec<SlotText> {
        Self::get_text_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2)
    }

}

/// `minecraft.gameMode.handleContainerInput(containerId, slotId, buttonNum, containerInput, minecraft.player)`: the
/// port's `handleContainerInput:443-481` over the player (its `:445-447` check of the container id is made here - the
/// port's body is handed the open menu's own id), then `:480`'s send.
fn handle_container_input(container_id: i32, slot_id: i32, button_num: i32, container_input: mcrust::net::minecraft::world::inventory::container_input::ContainerInput) {
    use mcrust::net::minecraft::client::multiplayer::multi_player_game_mode;
    use mcrust::net::minecraft::world::entity::player::player::Player as _;
    let packet = mcrust::mcrust::modding::game::with_player(|player| {
        let open_container_id = player.player().container_menu_container_id();
        if container_id != open_container_id {
            logger().warn(&format!("Ignoring click in mismatching container. Click in {}, player has {}.", container_id, open_container_id));
            return None;
        }
        multi_player_game_mode::handle_container_input_on_player(&mut compat::minecraft().game_mode, player, slot_id, button_num, container_input)
    })
    .flatten();
    multi_player_game_mode::send_container_click(packet, container_input, slot_id, button_num);
}
