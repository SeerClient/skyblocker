//! Ported from de.hysky.skyblocker.skyblock.item.HotbarSlotLock by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use mcrust::net::minecraft::client::key_mapping;
use mcrust::net::minecraft::client::player::local_player::LocalPlayer;

/// java: public static KeyMapping hotbarSlotLock;
pub static HOTBAR_SLOT_LOCK: compat::MainThread<Option<mcrust::net::minecraft::client::key_mapping::KeyMapping>> = compat::MainThread::new(|| None);

/// java: public class HotbarSlotLock
#[derive(Clone)]
pub struct HotbarSlotLock {}

impl HotbarSlotLock {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        key_binding_helper::register_key_binding(KeyMapping::new("key.skyblocker.hotbarSlotLock", 72, Category::Misc));
    }

    /// java: public static boolean isLocked(int slot)
    pub fn is_locked(mut slot: i32) -> bool {
        return SkyblockerConfigManager::get().general.locked_slots.iter().any(|e| *e == slot);
    }

    /// java: public static void handleInputEvents(LocalPlayer player)
    pub fn handle_input_events(player: &LocalPlayer) {
        // the port's key mappings are reached by name
        while key_mapping::consume_click("key.skyblocker.hotbarSlotLock") {
            let mut selected: i32 = player.base.base.get_inventory().get_selected_slot();
            SkyblockerConfigManager::update(Arc::new(move |mut config| {
                let locked_slots = &mut config.general.locked_slots;
                if !(locked_slots.iter().any(|e| *e == selected)) {
                    locked_slots.push(selected);
                } else {
                    compat::remove_item(&mut *locked_slots, &selected);
                }
            }));
        }
    }

}
