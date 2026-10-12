//! Ported from de.hysky.skyblocker.skyblock.crimson.kuudra.ArrowPoisonWarning by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::configs::crimson_isle_config::Kuudra;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::crimson::kuudra::kuudra;
use crate::de::hysky::skyblocker::skyblock::crimson::kuudra::kuudra_phase::KuudraPhase;
use crate::de::hysky::skyblocker::utils::render::title::title::Title;
use crate::de::hysky::skyblocker::utils::render::title::title_container::TitleContainer;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::world::entity::player::inventory::Inventory;
use mcrust::net::minecraft::world::item::item::Item;
use crate::de::hysky::skyblocker::injected::skyblocker_stack::SkyblockerStack as _;
use mcrust::net::minecraft::world::container::Container as _;

/// java: private static final Supplier<CrimsonIsleConfig.Kuudra> CONFIG = () -> SkyblockerConfigManager.get().crimsonIsle.kuudra;
pub static CONFIG: compat::MainThread<Arc<dyn Fn() -> compat::Shared<Kuudra>>> = compat::MainThread::new(|| Arc::new(move || SkyblockerConfigManager::get().crimson_isle.kuudra.clone()));
/// java: private static final int THREE_SECONDS = 20 * 3;
pub const THREE_SECONDS: i32 = 20i32.wrapping_mul(3);
/// java: private static final Title NONE_TITLE = new Title(Component.translatable("skyblocker.crimson.kuudra.noArrowPoison").withStyle(ChatFormatting.GREEN));
pub static NONE_TITLE: compat::MainThread<compat::Shared<Title>> = compat::MainThread::new(|| Title::new_1(component::translatable("skyblocker.crimson.kuudra.noArrowPoison").with_style_format(ChatFormatting::Green)));
/// java: private static final Title LOW_TITLE = new Title(Component.translatable("skyblocker.crimson.kuudra.lowArrowPoison").withStyle(ChatFormatting.GREEN));
pub static LOW_TITLE: compat::MainThread<compat::Shared<Title>> = compat::MainThread::new(|| Title::new_1(component::translatable("skyblocker.crimson.kuudra.lowArrowPoison").with_style_format(ChatFormatting::Green)));

/// java: public class ArrowPoisonWarning
#[derive(Clone)]
pub struct ArrowPoisonWarning {}

impl ArrowPoisonWarning {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static void tryWarn(int newSlot)
    pub fn try_warn(mut new_slot: i32) {
        if Utils::is_in_kuudra() && compat::read(&(CONFIG.get_mut())(), |__t| __t.no_arrow_poison_warning) && compat::get_state(&kuudra::PHASE) == KuudraPhase::Dps && new_slot != 8 {
            let client = compat::minecraft();
            // client.player.getInventory(), read through the player
            let scan: Option<(bool, i32)> = mcrust::mcrust::modding::game::with_player(|player| {
                let inventory = player.base.base.get_inventory();
                let held_item = inventory.get_item(new_slot);
                // `heldItem.getItem() instanceof BowItem`: the one `BowItem` is the registered `bow`
                if !std::sync::Arc::ptr_eq(&held_item.get_item(), &mcrust::net::minecraft::world::item::items::items().get("bow")) {
                    return None;
                }
                let mut has_toxic_arrow_poison: bool = false;
                let mut arrow_poison_amount: i32 = 0;
                for i in 0..inventory.get_container_size() {
                    let stack = inventory.get_item(i);
                    let mut item_id: String = stack.get_skyblock_id();
                    if item_id == "TOXIC_ARROW_POISON" {
                        has_toxic_arrow_poison = true;
                        arrow_poison_amount = arrow_poison_amount.wrapping_add(stack.get_count());
                    }
                }
                Some((has_toxic_arrow_poison, arrow_poison_amount))
            }).flatten();
            if let Some((has_toxic_arrow_poison, arrow_poison_amount)) = scan {
                if !has_toxic_arrow_poison {
                    TitleContainer::add_title_and_play_sound_2(NONE_TITLE.get_mut(), THREE_SECONDS);
                } else if arrow_poison_amount < compat::read(&(CONFIG.get_mut())(), |__t| __t.arrow_poison_threshold) {
                    TitleContainer::add_title_and_play_sound_2(LOW_TITLE.get_mut(), THREE_SECONDS);
                }
            }
        }
    }

}
