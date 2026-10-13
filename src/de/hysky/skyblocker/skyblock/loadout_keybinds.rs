//! Ported from de.hysky.skyblocker.skyblock.LoadoutKeybinds by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::item::slottext::simple_slot_text_adder::SimpleSlotTextAdder;
use crate::de::hysky::skyblocker::skyblock::item::slottext::simple_slot_text_adder::SimpleSlotTextAdderBase;
use crate::de::hysky::skyblocker::skyblock::item::slottext::slot_text;
use crate::de::hysky::skyblocker::skyblock::item::slottext::slot_text::SlotText;
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
/// java: public static final LoadoutKeybinds INSTANCE = new LoadoutKeybinds();
pub static INSTANCE: compat::MainThread<compat::Shared<LoadoutKeybinds>> = compat::MainThread::new(|| LoadoutKeybinds::new());
//   java: public static final KeyMapping LOADOUT_1 = KeyMappingHelper.registerKeyMapping(new KeyMapping("key.skyblocker.loadout.01", InputConstants.KEY_1, SkyblockerMod.KEYBINDING_CATEGORY));
/// java: public static final KeyMapping LOADOUT_1 = KeyMappingHelper.registerKeyMapping(new KeyMapping("key.skyblocker.loadout.01", InputConstants.KEY_1, SkyblockerMod.KEYBINDING_CATEGORY));
pub static LOADOUT_1: compat::MainThread<mcrust::net::minecraft::client::key_mapping::KeyMapping> = compat::MainThread::new(|| key_binding_helper::register_key_binding(KeyMapping::new("key.skyblocker.loadout.01", 49 /* InputConstants.KEY_1 */, *crate::entry::skyblocker_mod::KEYBINDING_CATEGORY.get())));
//   java: public static final KeyMapping LOADOUT_2 = KeyMappingHelper.registerKeyMapping(new KeyMapping("key.skyblocker.loadout.02", InputConstants.KEY_2, SkyblockerMod.KEYBINDING_CATEGORY));
/// java: public static final KeyMapping LOADOUT_2 = KeyMappingHelper.registerKeyMapping(new KeyMapping("key.skyblocker.loadout.02", InputConstants.KEY_2, SkyblockerMod.KEYBINDING_CATEGORY));
pub static LOADOUT_2: compat::MainThread<mcrust::net::minecraft::client::key_mapping::KeyMapping> = compat::MainThread::new(|| key_binding_helper::register_key_binding(KeyMapping::new("key.skyblocker.loadout.02", 50 /* InputConstants.KEY_2 */, *crate::entry::skyblocker_mod::KEYBINDING_CATEGORY.get())));
//   java: public static final KeyMapping LOADOUT_3 = KeyMappingHelper.registerKeyMapping(new KeyMapping("key.skyblocker.loadout.03", InputConstants.KEY_3, SkyblockerMod.KEYBINDING_CATEGORY));
/// java: public static final KeyMapping LOADOUT_3 = KeyMappingHelper.registerKeyMapping(new KeyMapping("key.skyblocker.loadout.03", InputConstants.KEY_3, SkyblockerMod.KEYBINDING_CATEGORY));
pub static LOADOUT_3: compat::MainThread<mcrust::net::minecraft::client::key_mapping::KeyMapping> = compat::MainThread::new(|| key_binding_helper::register_key_binding(KeyMapping::new("key.skyblocker.loadout.03", 51 /* InputConstants.KEY_3 */, *crate::entry::skyblocker_mod::KEYBINDING_CATEGORY.get())));
//   java: public static final KeyMapping LOADOUT_4 = KeyMappingHelper.registerKeyMapping(new KeyMapping("key.skyblocker.loadout.04", InputConstants.KEY_4, SkyblockerMod.KEYBINDING_CATEGORY));
/// java: public static final KeyMapping LOADOUT_4 = KeyMappingHelper.registerKeyMapping(new KeyMapping("key.skyblocker.loadout.04", InputConstants.KEY_4, SkyblockerMod.KEYBINDING_CATEGORY));
pub static LOADOUT_4: compat::MainThread<mcrust::net::minecraft::client::key_mapping::KeyMapping> = compat::MainThread::new(|| key_binding_helper::register_key_binding(KeyMapping::new("key.skyblocker.loadout.04", 52 /* InputConstants.KEY_4 */, *crate::entry::skyblocker_mod::KEYBINDING_CATEGORY.get())));
//   java: public static final KeyMapping LOADOUT_5 = KeyMappingHelper.registerKeyMapping(new KeyMapping("key.skyblocker.loadout.05", InputConstants.KEY_5, SkyblockerMod.KEYBINDING_CATEGORY));
/// java: public static final KeyMapping LOADOUT_5 = KeyMappingHelper.registerKeyMapping(new KeyMapping("key.skyblocker.loadout.05", InputConstants.KEY_5, SkyblockerMod.KEYBINDING_CATEGORY));
pub static LOADOUT_5: compat::MainThread<mcrust::net::minecraft::client::key_mapping::KeyMapping> = compat::MainThread::new(|| key_binding_helper::register_key_binding(KeyMapping::new("key.skyblocker.loadout.05", 53 /* InputConstants.KEY_5 */, *crate::entry::skyblocker_mod::KEYBINDING_CATEGORY.get())));
//   java: public static final KeyMapping LOADOUT_6 = KeyMappingHelper.registerKeyMapping(new KeyMapping("key.skyblocker.loadout.06", InputConstants.KEY_6, SkyblockerMod.KEYBINDING_CATEGORY));
/// java: public static final KeyMapping LOADOUT_6 = KeyMappingHelper.registerKeyMapping(new KeyMapping("key.skyblocker.loadout.06", InputConstants.KEY_6, SkyblockerMod.KEYBINDING_CATEGORY));
pub static LOADOUT_6: compat::MainThread<mcrust::net::minecraft::client::key_mapping::KeyMapping> = compat::MainThread::new(|| key_binding_helper::register_key_binding(KeyMapping::new("key.skyblocker.loadout.06", 54 /* InputConstants.KEY_6 */, *crate::entry::skyblocker_mod::KEYBINDING_CATEGORY.get())));
//   java: public static final KeyMapping LOADOUT_7 = KeyMappingHelper.registerKeyMapping(new KeyMapping("key.skyblocker.loadout.07", InputConstants.KEY_7, SkyblockerMod.KEYBINDING_CATEGORY));
/// java: public static final KeyMapping LOADOUT_7 = KeyMappingHelper.registerKeyMapping(new KeyMapping("key.skyblocker.loadout.07", InputConstants.KEY_7, SkyblockerMod.KEYBINDING_CATEGORY));
pub static LOADOUT_7: compat::MainThread<mcrust::net::minecraft::client::key_mapping::KeyMapping> = compat::MainThread::new(|| key_binding_helper::register_key_binding(KeyMapping::new("key.skyblocker.loadout.07", 55 /* InputConstants.KEY_7 */, *crate::entry::skyblocker_mod::KEYBINDING_CATEGORY.get())));
//   java: public static final KeyMapping LOADOUT_8 = KeyMappingHelper.registerKeyMapping(new KeyMapping("key.skyblocker.loadout.08", InputConstants.KEY_8, SkyblockerMod.KEYBINDING_CATEGORY));
/// java: public static final KeyMapping LOADOUT_8 = KeyMappingHelper.registerKeyMapping(new KeyMapping("key.skyblocker.loadout.08", InputConstants.KEY_8, SkyblockerMod.KEYBINDING_CATEGORY));
pub static LOADOUT_8: compat::MainThread<mcrust::net::minecraft::client::key_mapping::KeyMapping> = compat::MainThread::new(|| key_binding_helper::register_key_binding(KeyMapping::new("key.skyblocker.loadout.08", 56 /* InputConstants.KEY_8 */, *crate::entry::skyblocker_mod::KEYBINDING_CATEGORY.get())));
//   java: public static final KeyMapping LOADOUT_9 = KeyMappingHelper.registerKeyMapping(new KeyMapping("key.skyblocker.loadout.09", InputConstants.KEY_9, SkyblockerMod.KEYBINDING_CATEGORY));
/// java: public static final KeyMapping LOADOUT_9 = KeyMappingHelper.registerKeyMapping(new KeyMapping("key.skyblocker.loadout.09", InputConstants.KEY_9, SkyblockerMod.KEYBINDING_CATEGORY));
pub static LOADOUT_9: compat::MainThread<mcrust::net::minecraft::client::key_mapping::KeyMapping> = compat::MainThread::new(|| key_binding_helper::register_key_binding(KeyMapping::new("key.skyblocker.loadout.09", 57 /* InputConstants.KEY_9 */, *crate::entry::skyblocker_mod::KEYBINDING_CATEGORY.get())));
//   java: public static final KeyMapping LOADOUT_10 = KeyMappingHelper.registerKeyMapping(new KeyMapping("key.skyblocker.loadout.10", InputConstants.KEY_0, SkyblockerMod.KEYBINDING_CATEGORY));
/// java: public static final KeyMapping LOADOUT_10 = KeyMappingHelper.registerKeyMapping(new KeyMapping("key.skyblocker.loadout.10", InputConstants.KEY_0, SkyblockerMod.KEYBINDING_CATEGORY));
pub static LOADOUT_10: compat::MainThread<mcrust::net::minecraft::client::key_mapping::KeyMapping> = compat::MainThread::new(|| key_binding_helper::register_key_binding(KeyMapping::new("key.skyblocker.loadout.10", 48 /* InputConstants.KEY_0 */, *crate::entry::skyblocker_mod::KEYBINDING_CATEGORY.get())));
//   java: public static final KeyMapping LOADOUT_11 = KeyMappingHelper.registerKeyMapping(new KeyMapping("key.skyblocker.loadout.11", InputConstants.KEY_MINUS, SkyblockerMod.KEYBINDING_CATEGORY));
/// java: public static final KeyMapping LOADOUT_11 = KeyMappingHelper.registerKeyMapping(new KeyMapping("key.skyblocker.loadout.11", InputConstants.KEY_MINUS, SkyblockerMod.KEYBINDING_CATEGORY));
pub static LOADOUT_11: compat::MainThread<mcrust::net::minecraft::client::key_mapping::KeyMapping> = compat::MainThread::new(|| key_binding_helper::register_key_binding(KeyMapping::new("key.skyblocker.loadout.11", 45 /* InputConstants.KEY_MINUS */, *crate::entry::skyblocker_mod::KEYBINDING_CATEGORY.get())));
//   java: public static final KeyMapping LOADOUT_12 = KeyMappingHelper.registerKeyMapping(new KeyMapping("key.skyblocker.loadout.12", InputConstants.KEY_EQUALS, SkyblockerMod.KEYBINDING_CATEGORY));
/// java: public static final KeyMapping LOADOUT_12 = KeyMappingHelper.registerKeyMapping(new KeyMapping("key.skyblocker.loadout.12", InputConstants.KEY_EQUALS, SkyblockerMod.KEYBINDING_CATEGORY));
pub static LOADOUT_12: compat::MainThread<mcrust::net::minecraft::client::key_mapping::KeyMapping> = compat::MainThread::new(|| key_binding_helper::register_key_binding(KeyMapping::new("key.skyblocker.loadout.12", 61 /* InputConstants.KEY_EQUALS */, *crate::entry::skyblocker_mod::KEYBINDING_CATEGORY.get())));
/// java: protected static final KeyMapping[] LOADOUT_KEY_MAPPINGS = { LOADOUT_1, LOADOUT_2, LOADOUT_3, LOADOUT_4, LOADOUT_5, LOADOUT_6, LOADOUT_7, LOADOUT_8, LOADOUT_9, LOADOUT_10, LOADOUT_11, LOADOUT_12 };
pub static LOADOUT_KEY_MAPPINGS: compat::MainThread<Vec<mcrust::net::minecraft::client::key_mapping::KeyMapping>> = compat::MainThread::new(|| vec![LOADOUT_1.get_mut().clone(), LOADOUT_2.get_mut().clone(), LOADOUT_3.get_mut().clone(), LOADOUT_4.get_mut().clone(), LOADOUT_5.get_mut().clone(), LOADOUT_6.get_mut().clone(), LOADOUT_7.get_mut().clone(), LOADOUT_8.get_mut().clone(), LOADOUT_9.get_mut().clone(), LOADOUT_10.get_mut().clone(), LOADOUT_11.get_mut().clone(), LOADOUT_12.get_mut().clone()]);

/// java: public class LoadoutKeybinds extends SimpleSlotTextAdder
#[derive(Clone)]
pub struct LoadoutKeybinds {
    /// java: extends SimpleSlotTextAdder
    pub base: SimpleSlotTextAdderBase,
}

impl std::ops::Deref for LoadoutKeybinds {
    type Target = SimpleSlotTextAdderBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for LoadoutKeybinds {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl LoadoutKeybinds {
    /// java: private LoadoutKeybinds()
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = SimpleSlotTextAdderBase::new_in("^(?:\\(\\d+/\\d+\\) )?(Loadouts)$".to_string(), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
            });
            ((__h.clone() as compat::Shared<dyn SimpleSlotTextAdder>), __h)
        });
        __this
    }

    /// java: @Init public static void init()
    pub fn init() {
        // the class's static initialiser runs before its first static method: the twelve `registerKeyMapping` calls
        let _ = LOADOUT_KEY_MAPPINGS.get();
        // Fabric registers the two input listeners on the screen instance (`ScreenKeyboardEvents.allowKeyPress(screen)`),
        // so they live as long as that screen; the port's input events are global, so they are registered once and
        // answer for the screen `AFTER_INIT` last accepted (`SCREEN_ACCEPTED`, reset by every screen's init).
        screen_events::AFTER_INIT.register(Arc::new(|screen: &mut Screen, width: i32, height: i32| {
            //   java: if (!(screen instanceof AbstractContainerScreen<?> handledScreen) || !INSTANCE.test(handledScreen) || !INSTANCE.isEnabled() || client.gameMode == null) return;
            let is_container_screen = matches!(compat::minecraft().gui.screen_kind, mcrust::net::minecraft::client::gui::gui::ScreenKind::Container);
            let accepted = is_container_screen && {
                let __r = &compat::view(INSTANCE.get(), |a| a as compat::Shared<dyn ContainerMatcher>);
                let __test = compat::dispatch(__r, |__t| __t.test());
                let __enabled = compat::dispatch(__r, |__t| __t.is_enabled());
                __test(__r, screen) && __enabled(__r)
            };
            compat::set_state(&SCREEN_ACCEPTED, accepted);
        }));
        //   java: ScreenKeyboardEvents.allowKeyPress(handledScreen).register((_, keyInput) -> allowInput(client, handledScreen, keybinding -> keybinding.matches(keyInput)));
        screen_keyboard_events::ALLOW_KEY_PRESS.register(Arc::new(|action: i32, key_event: &KeyEvent| {
            if action == 0 || !compat::get_state(&SCREEN_ACCEPTED) {
                return true;
            }
            let key_event = *key_event;
            LoadoutKeybinds::allow_input(Arc::new(move |keybinding: mcrust::net::minecraft::client::key_mapping::KeyMapping| key_mapping::matches(keybinding.get_name(), &key_event)))
        }));
        //   java: ScreenMouseEvents.allowMouseClick(handledScreen).register((_, click) -> allowInput(client, handledScreen, keybinding -> keybinding.matchesMouse(click)));
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
        let mut loadout_index: i32;
        {
            loadout_index = 0;
            while loadout_index < (LOADOUT_KEY_MAPPINGS.get_mut().len() as i32) {
                if (predicate)(LOADOUT_KEY_MAPPINGS.get_mut()[(loadout_index) as usize].clone()) {
                    found = true;
                    break;
                }
                loadout_index += 1;
            }
        }
        if !found {
            return true;
        }
        let mut slot_index: i32 = LoadoutKeybinds::loadout_index_to_slot_id(loadout_index);
        //   java: ItemStack stack = containerScreen.getMenu().getSlot(slotIndex).getItem();
        let Some((container_id, mut stack)) = compat::with_open_menu(|menu, containers| (menu.container_id, menu.slots[slot_index as usize].get_item(containers).duplicate())) else {
            return true;
        };
        if LoadoutKeybinds::is_item_invalid(&stack) {
            return true;
        }
        //   java: minecraft.gameMode.handleContainerInput(containerScreen.getMenu().containerId, slotIndex, ContainerUtils.getContainerClickButton(InputConstants.MOUSE_BUTTON_LEFT), ContainerInput.PICKUP, minecraft.player);
        handle_container_input(container_id, slot_index, crate::de::hysky::skyblocker::utils::container_utils::ContainerUtils::get_container_click_button(0), mcrust::net::minecraft::world::inventory::container_input::ContainerInput::Pickup);
        return false;
    }

    /// java: private static int loadoutIndexToSlotId(int loadoutIndex)
    pub fn loadout_index_to_slot_id(mut loadout_index: i32) -> i32 {
        return match loadout_index {
            0 => 14,
            1 => 15,
            2 => 16,
            3 => 23,
            4 => 24,
            5 => 25,
            6 => 32,
            7 => 33,
            8 => 34,
            9 => 41,
            10 => 42,
            11 => 43,
            _ => {
                panic!("IllegalArgumentException: {}", format!("Illegal loadoutIndex {}", loadout_index).to_string());
            }
        };
    }

    /// java: private static int slotIdToLoadoutIndex(int slotId)
    pub fn slot_id_to_loadout_index(mut slot_id: i32) -> i32 {
        return match slot_id {
            14 => 0,
            15 => 1,
            16 => 2,
            23 => 3,
            24 => 4,
            25 => 5,
            32 => 6,
            33 => 7,
            34 => 8,
            41 => 9,
            42 => 10,
            43 => 11,
            _ => -1,
        };
    }

    /// java: private static boolean isItemInvalid(ItemStack stack)
    pub fn is_item_invalid(stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack) -> bool {
        return (stack.is_item(mcrust::net::minecraft::world::item::items::dye().gray()) || stack.is_item(mcrust::net::minecraft::world::item::items::dye().red()) || stack.is_item(mcrust::net::minecraft::world::item::items::stained_glass_pane().black()) || stack.is_empty());
    }

}

impl SimpleSlotTextAdder for LoadoutKeybinds {
    fn simple_slot_text_adder_base(&self) -> &SimpleSlotTextAdderBase {
        &self.base
    }
    fn simple_slot_text_adder_base_mut(&mut self) -> &mut SimpleSlotTextAdderBase {
        &mut self.base
    }

}

impl RegexContainerMatcher for LoadoutKeybinds {
    fn regex_container_matcher_base(&self) -> &RegexContainerMatcherBase {
        &self.base.base
    }
    fn regex_container_matcher_base_mut(&mut self) -> &mut RegexContainerMatcherBase {
        &mut self.base.base
    }

}

impl ContainerMatcher for LoadoutKeybinds {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public boolean isEnabled() (dispatch: the body for this class)
    fn is_enabled(&self) -> fn(&compat::Shared<dyn ContainerMatcher>) -> bool {
        LoadoutKeybinds::is_enabled_dyn
    }

    /// java: @Override public boolean test(Screen screen) (dispatch: the body for this class)
    fn test(&self) -> fn(&compat::Shared<dyn ContainerMatcher>, &mcrust::net::minecraft::client::gui::screens::screen::Screen) -> bool {
        <dyn RegexContainerMatcher>::test_from::<Self>
    }

}

impl SlotTextAdder for LoadoutKeybinds {
    /// java: @Override public List<SlotText> getText(@Nullable Slot slot, ItemStack stack, int slotId) (dispatch: the body for this class)
    fn get_text(&self) -> fn(&compat::Shared<dyn SlotTextAdder>, Option<Slot>, &mcrust::net::minecraft::world::item::item_stack::ItemStack, i32) -> Vec<SlotText> {
        LoadoutKeybinds::get_text_dyn
    }

    /// java: @Override public @Nullable ConfigInformation getConfigInformation() (dispatch: the body for this class)
    fn get_config_information(&self) -> fn(&compat::Shared<dyn SlotTextAdder>) -> Option<ConfigInformation> {
        <dyn SimpleSlotTextAdder>::get_config_information_from::<Self>
    }

}

impl LoadoutKeybinds {
    /// java: @Override public boolean isEnabled()
    pub fn is_enabled_impl(__this: &compat::Shared<Self>) -> bool {
        return SkyblockerConfigManager::get().helpers.enable_wardrobe_helper;
    }

    /// LoadoutKeybinds.isEnabled as the dispatch of ContainerMatcher.isEnabled
    pub fn is_enabled_dyn(__this: &compat::Shared<dyn ContainerMatcher>) -> bool {
        Self::is_enabled_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public List<SlotText> getText(@Nullable Slot slot, ItemStack stack, int slotId)
    pub fn get_text_impl(__this: &compat::Shared<Self>, mut slot: Option<Slot>, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, mut slot_id: i32) -> Vec<SlotText> {
        let mut loadout_index: i32 = LoadoutKeybinds::slot_id_to_loadout_index(slot_id);
        if loadout_index == -1 || LoadoutKeybinds::is_item_invalid(stack) {
            return vec![];
        }
        //   java: return SlotText.bottomLeftList(LOADOUT_KEY_MAPPINGS[loadoutIndex].getTranslatedKeyMessage().plainCopy().withColor(SlotText.MID_BLUE));
        let name: &'static str = LOADOUT_KEY_MAPPINGS.get()[loadout_index as usize].get_name();
        let message = key_mapping::get_translated_key_message(name, &|key: i32, _scancode: i32| compat::minecraft().key_names.get(&key).cloned());
        return SlotText::bottom_left_list(&compat::component_of(message).plain_copy().with_color(slot_text::MID_BLUE));
    }

    /// LoadoutKeybinds.getText as the dispatch of SlotTextAdder.getText
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
