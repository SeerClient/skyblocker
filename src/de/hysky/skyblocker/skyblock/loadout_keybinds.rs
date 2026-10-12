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

/// java: public static final LoadoutKeybinds INSTANCE = new LoadoutKeybinds();
pub static INSTANCE: compat::MainThread<compat::Shared<LoadoutKeybinds>> = compat::MainThread::new(|| LoadoutKeybinds::new());
// TODO(fabric2seer): Fabric API `KeyMappingHelper.registerKeyMapping` has no SeerClient equivalent
//   java: public static final KeyMapping LOADOUT_1 = KeyMappingHelper.registerKeyMapping(new KeyMapping("key.skyblocker.loadout.01", InputConstants.KEY_1, SkyblockerMod.KEYBINDING_CATEGORY));
/// java: public static final KeyMapping LOADOUT_1 = KeyMappingHelper.registerKeyMapping(new KeyMapping("key.skyblocker.loadout.01", InputConstants.KEY_1, SkyblockerMod.KEYBINDING_CATEGORY));
pub static LOADOUT_1: compat::MainThread<mcrust::net::minecraft::client::key_mapping::KeyMapping> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `LOADOUT_1` - see the TODO above"));
// TODO(fabric2seer): Fabric API `KeyMappingHelper.registerKeyMapping` has no SeerClient equivalent
//   java: public static final KeyMapping LOADOUT_2 = KeyMappingHelper.registerKeyMapping(new KeyMapping("key.skyblocker.loadout.02", InputConstants.KEY_2, SkyblockerMod.KEYBINDING_CATEGORY));
/// java: public static final KeyMapping LOADOUT_2 = KeyMappingHelper.registerKeyMapping(new KeyMapping("key.skyblocker.loadout.02", InputConstants.KEY_2, SkyblockerMod.KEYBINDING_CATEGORY));
pub static LOADOUT_2: compat::MainThread<mcrust::net::minecraft::client::key_mapping::KeyMapping> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `LOADOUT_2` - see the TODO above"));
// TODO(fabric2seer): Fabric API `KeyMappingHelper.registerKeyMapping` has no SeerClient equivalent
//   java: public static final KeyMapping LOADOUT_3 = KeyMappingHelper.registerKeyMapping(new KeyMapping("key.skyblocker.loadout.03", InputConstants.KEY_3, SkyblockerMod.KEYBINDING_CATEGORY));
/// java: public static final KeyMapping LOADOUT_3 = KeyMappingHelper.registerKeyMapping(new KeyMapping("key.skyblocker.loadout.03", InputConstants.KEY_3, SkyblockerMod.KEYBINDING_CATEGORY));
pub static LOADOUT_3: compat::MainThread<mcrust::net::minecraft::client::key_mapping::KeyMapping> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `LOADOUT_3` - see the TODO above"));
// TODO(fabric2seer): Fabric API `KeyMappingHelper.registerKeyMapping` has no SeerClient equivalent
//   java: public static final KeyMapping LOADOUT_4 = KeyMappingHelper.registerKeyMapping(new KeyMapping("key.skyblocker.loadout.04", InputConstants.KEY_4, SkyblockerMod.KEYBINDING_CATEGORY));
/// java: public static final KeyMapping LOADOUT_4 = KeyMappingHelper.registerKeyMapping(new KeyMapping("key.skyblocker.loadout.04", InputConstants.KEY_4, SkyblockerMod.KEYBINDING_CATEGORY));
pub static LOADOUT_4: compat::MainThread<mcrust::net::minecraft::client::key_mapping::KeyMapping> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `LOADOUT_4` - see the TODO above"));
// TODO(fabric2seer): Fabric API `KeyMappingHelper.registerKeyMapping` has no SeerClient equivalent
//   java: public static final KeyMapping LOADOUT_5 = KeyMappingHelper.registerKeyMapping(new KeyMapping("key.skyblocker.loadout.05", InputConstants.KEY_5, SkyblockerMod.KEYBINDING_CATEGORY));
/// java: public static final KeyMapping LOADOUT_5 = KeyMappingHelper.registerKeyMapping(new KeyMapping("key.skyblocker.loadout.05", InputConstants.KEY_5, SkyblockerMod.KEYBINDING_CATEGORY));
pub static LOADOUT_5: compat::MainThread<mcrust::net::minecraft::client::key_mapping::KeyMapping> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `LOADOUT_5` - see the TODO above"));
// TODO(fabric2seer): Fabric API `KeyMappingHelper.registerKeyMapping` has no SeerClient equivalent
//   java: public static final KeyMapping LOADOUT_6 = KeyMappingHelper.registerKeyMapping(new KeyMapping("key.skyblocker.loadout.06", InputConstants.KEY_6, SkyblockerMod.KEYBINDING_CATEGORY));
/// java: public static final KeyMapping LOADOUT_6 = KeyMappingHelper.registerKeyMapping(new KeyMapping("key.skyblocker.loadout.06", InputConstants.KEY_6, SkyblockerMod.KEYBINDING_CATEGORY));
pub static LOADOUT_6: compat::MainThread<mcrust::net::minecraft::client::key_mapping::KeyMapping> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `LOADOUT_6` - see the TODO above"));
// TODO(fabric2seer): Fabric API `KeyMappingHelper.registerKeyMapping` has no SeerClient equivalent
//   java: public static final KeyMapping LOADOUT_7 = KeyMappingHelper.registerKeyMapping(new KeyMapping("key.skyblocker.loadout.07", InputConstants.KEY_7, SkyblockerMod.KEYBINDING_CATEGORY));
/// java: public static final KeyMapping LOADOUT_7 = KeyMappingHelper.registerKeyMapping(new KeyMapping("key.skyblocker.loadout.07", InputConstants.KEY_7, SkyblockerMod.KEYBINDING_CATEGORY));
pub static LOADOUT_7: compat::MainThread<mcrust::net::minecraft::client::key_mapping::KeyMapping> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `LOADOUT_7` - see the TODO above"));
// TODO(fabric2seer): Fabric API `KeyMappingHelper.registerKeyMapping` has no SeerClient equivalent
//   java: public static final KeyMapping LOADOUT_8 = KeyMappingHelper.registerKeyMapping(new KeyMapping("key.skyblocker.loadout.08", InputConstants.KEY_8, SkyblockerMod.KEYBINDING_CATEGORY));
/// java: public static final KeyMapping LOADOUT_8 = KeyMappingHelper.registerKeyMapping(new KeyMapping("key.skyblocker.loadout.08", InputConstants.KEY_8, SkyblockerMod.KEYBINDING_CATEGORY));
pub static LOADOUT_8: compat::MainThread<mcrust::net::minecraft::client::key_mapping::KeyMapping> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `LOADOUT_8` - see the TODO above"));
// TODO(fabric2seer): Fabric API `KeyMappingHelper.registerKeyMapping` has no SeerClient equivalent
//   java: public static final KeyMapping LOADOUT_9 = KeyMappingHelper.registerKeyMapping(new KeyMapping("key.skyblocker.loadout.09", InputConstants.KEY_9, SkyblockerMod.KEYBINDING_CATEGORY));
/// java: public static final KeyMapping LOADOUT_9 = KeyMappingHelper.registerKeyMapping(new KeyMapping("key.skyblocker.loadout.09", InputConstants.KEY_9, SkyblockerMod.KEYBINDING_CATEGORY));
pub static LOADOUT_9: compat::MainThread<mcrust::net::minecraft::client::key_mapping::KeyMapping> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `LOADOUT_9` - see the TODO above"));
// TODO(fabric2seer): Fabric API `KeyMappingHelper.registerKeyMapping` has no SeerClient equivalent
//   java: public static final KeyMapping LOADOUT_10 = KeyMappingHelper.registerKeyMapping(new KeyMapping("key.skyblocker.loadout.10", InputConstants.KEY_0, SkyblockerMod.KEYBINDING_CATEGORY));
/// java: public static final KeyMapping LOADOUT_10 = KeyMappingHelper.registerKeyMapping(new KeyMapping("key.skyblocker.loadout.10", InputConstants.KEY_0, SkyblockerMod.KEYBINDING_CATEGORY));
pub static LOADOUT_10: compat::MainThread<mcrust::net::minecraft::client::key_mapping::KeyMapping> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `LOADOUT_10` - see the TODO above"));
// TODO(fabric2seer): Fabric API `KeyMappingHelper.registerKeyMapping` has no SeerClient equivalent
//   java: public static final KeyMapping LOADOUT_11 = KeyMappingHelper.registerKeyMapping(new KeyMapping("key.skyblocker.loadout.11", InputConstants.KEY_MINUS, SkyblockerMod.KEYBINDING_CATEGORY));
/// java: public static final KeyMapping LOADOUT_11 = KeyMappingHelper.registerKeyMapping(new KeyMapping("key.skyblocker.loadout.11", InputConstants.KEY_MINUS, SkyblockerMod.KEYBINDING_CATEGORY));
pub static LOADOUT_11: compat::MainThread<mcrust::net::minecraft::client::key_mapping::KeyMapping> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `LOADOUT_11` - see the TODO above"));
// TODO(fabric2seer): Fabric API `KeyMappingHelper.registerKeyMapping` has no SeerClient equivalent
//   java: public static final KeyMapping LOADOUT_12 = KeyMappingHelper.registerKeyMapping(new KeyMapping("key.skyblocker.loadout.12", InputConstants.KEY_EQUALS, SkyblockerMod.KEYBINDING_CATEGORY));
/// java: public static final KeyMapping LOADOUT_12 = KeyMappingHelper.registerKeyMapping(new KeyMapping("key.skyblocker.loadout.12", InputConstants.KEY_EQUALS, SkyblockerMod.KEYBINDING_CATEGORY));
pub static LOADOUT_12: compat::MainThread<mcrust::net::minecraft::client::key_mapping::KeyMapping> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `LOADOUT_12` - see the TODO above"));
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
        screen_events::AFTER_INIT.register(Arc::new(|screen: &mut Screen, width: i32, height: i32| {
            // TODO(fabric2seer): an `instanceof` pattern under `||` / `!`
            //   java: if (!(screen instanceof AbstractContainerScreen<?> handledScreen) || !INSTANCE.test(handledScreen) || !INSTANCE.isEnabled() || client.gameMode == null) return;
            screen_keyboard_events::ALLOW_KEY_PRESS.register(Arc::new(|action: i32, key_event: &KeyEvent| {
                // TODO(fabric2seer): `client`: Fabric passes the Minecraft here; this SeerClient callback does not (take it from ClientTickEvents or a hook)
                //   java: return allowInput(client, handledScreen, keybinding -> keybinding.matches(keyInput));
                true
            }));
            screen_mouse_events::ALLOW_MOUSE_CLICK.register(Arc::new(|screen: &mut Screen, (mouse_x, mouse_y): (i32, i32), pressed: bool| {
                // TODO(fabric2seer): `client`: Fabric passes the Minecraft here; this SeerClient callback does not (take it from ClientTickEvents or a hook)
                //   java: return allowInput(client, handledScreen, keybinding -> keybinding.matchesMouse(click));
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
        // TODO(fabric2seer): `getSlot` on a Object
        //   java: ItemStack stack = containerScreen.getMenu().getSlot(slotIndex).getItem();
        let mut stack: mcrust::net::minecraft::world::item::item_stack::ItemStack = todo!("fabric2seer: the initialiser of `stack` (TODO above)");
        if LoadoutKeybinds::is_item_invalid(&stack) {
            return true;
        }
        // TODO(fabric2seer): field `containerId` of a Object
        //   java: minecraft.gameMode.handleContainerInput(containerScreen.getMenu().containerId, slotIndex, ContainerUtils.getContainerClickButton(InputConstants.MOUSE_BUTTON_LEFT), ContainerInput.PICKUP, minecraft.player);
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
        // TODO(fabric2seer): `get_translated_key_message` takes 2 argument(s) in the port (`name: &str, key_name: &dyn Fn(i32, i32) -> Option<String>`), Java passes 0
        // rust: return SlotText::bottom_left_list(&compat::component_of(key_mapping::get_translated_key_message()).plain_copy().with_color(slot_text::MID_BLUE));
        return todo!("fabric2seer: see the TODO above");
    }

    /// LoadoutKeybinds.getText as the dispatch of SlotTextAdder.getText
    pub fn get_text_dyn(__this: &compat::Shared<dyn SlotTextAdder>, __p0: Option<Slot>, __p1: &mcrust::net::minecraft::world::item::item_stack::ItemStack, __p2: i32) -> Vec<SlotText> {
        Self::get_text_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2)
    }

}
