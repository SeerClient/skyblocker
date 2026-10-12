//! Ported from de.hysky.skyblocker.utils.chat.ChatMessageListener by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::barn::call_trevor::CallTrevor;
use crate::de::hysky::skyblocker::skyblock::barn::hungry_hiker::HungryHiker;
use crate::de::hysky::skyblocker::skyblock::barn::treasure_hunter::TreasureHunter;
use crate::de::hysky::skyblocker::skyblock::chat::filters::ability_filter::AbilityFilter;
use crate::de::hysky::skyblocker::skyblock::chat::filters::ad_filter::AdFilter;
use crate::de::hysky::skyblocker::skyblock::chat::filters::aote_filter::AoteFilter;
use crate::de::hysky::skyblocker::skyblock::chat::filters::autopet_filter::AutopetFilter;
use crate::de::hysky::skyblocker::skyblock::chat::filters::combo_filter::ComboFilter;
use crate::de::hysky::skyblocker::skyblock::chat::filters::critter_capture_filter::CritterCaptureFilter;
use crate::de::hysky::skyblocker::skyblock::chat::filters::death_filter::DeathFilter;
use crate::de::hysky::skyblocker::skyblock::chat::filters::dungeon_breaker_filter::DungeonBreakerFilter;
use crate::de::hysky::skyblocker::skyblock::chat::filters::heal_filter::HealFilter;
use crate::de::hysky::skyblocker::skyblock::chat::filters::implosion_filter::ImplosionFilter;
use crate::de::hysky::skyblocker::skyblock::chat::filters::lottery_filter::LotteryFilter;
use crate::de::hysky::skyblocker::skyblock::chat::filters::mimic_filter::MimicFilter;
use crate::de::hysky::skyblocker::skyblock::chat::filters::molten_wave_filter::MoltenWaveFilter;
use crate::de::hysky::skyblocker::skyblock::chat::filters::rare_crop_filter::RareCropFilter;
use crate::de::hysky::skyblocker::skyblock::chat::filters::show_off_filter::ShowOffFilter;
use crate::de::hysky::skyblocker::skyblock::chat::filters::sky_mall_filter::SkyMallFilter;
use crate::de::hysky::skyblocker::skyblock::chat::filters::spirit_sceptre_filter::SpiritSceptreFilter;
use crate::de::hysky::skyblocker::skyblock::chat::filters::teleport_pad_filter::TeleportPadFilter;
use crate::de::hysky::skyblocker::skyblock::dungeon::reparty::Reparty;
use crate::de::hysky::skyblocker::skyblock::dwarven::call_mismyla::CallMismyla;
use crate::de::hysky::skyblocker::skyblock::dwarven::fetchur::Fetchur;
use crate::de::hysky::skyblocker::skyblock::dwarven::fossil::fossil_muncher::FossilMuncher;
use crate::de::hysky::skyblocker::skyblock::dwarven::puzzler::Puzzler;
use crate::de::hysky::skyblocker::skyblock::dwarven::redial_on_bad_signal::RedialOnBadSignal;
use crate::de::hysky::skyblocker::skyblock::foraging::galatea::sweep_details_listener::SweepDetailsListener;
use crate::de::hysky::skyblocker::skyblock::slayers::boss::demonlord::hellion_shield_filter::HellionShieldFilter;
use crate::de::hysky::skyblocker::skyblock::slayers::features::slayer_miniboss_spawn_filter::SlayerMinibossSpawnFilter;
use crate::de::hysky::skyblocker::utils::chat::chat_filter_result::ChatFilterResult;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

// TODO(fabric2seer): Fabric API `EventFactory.createArrayBacked` has no SeerClient equivalent
//   java: Event<ChatMessageListener> EVENT = EventFactory.createArrayBacked(ChatMessageListener.class,
//   java: 			listeners -> (message, asString) -> {
//   java: 				for (ChatMessageListener listener : listeners) {
//   java: 					ChatFilterResult result = listener.onMessage(message, asString);
//   java: 					if (result != ChatFilterResult.PASS) return result;
//   java: 				}
//   java: 				return ChatFilterResult.PASS;
//   java: 			});
/// java: Event<ChatMessageListener> EVENT = EventFactory.createArrayBacked(ChatMessageListener.class, listeners -> (message, asString) -> { for (ChatMessageListener listener : listeners) { ChatFilterResult res
pub static EVENT: compat::MainThread<crate::unresolved::Event> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `EVENT` - see the TODO above"));

/// java: @FunctionalInterface public interface ChatMessageListener
pub trait ChatMessageListener: compat::DynAny {
    fn as_any(&self) -> &dyn std::any::Any;
    /// java: ChatFilterResult onMessage(Component message, String asString); (dispatch: the body for this class)
    fn on_message(&self) -> fn(&compat::Shared<dyn ChatMessageListener>, &MutableComponent, &str) -> ChatFilterResult;

}

/// java: @SuppressWarnings("incomplete-switch") @Init static void init()
pub fn init() {
    let mut listeners: Vec<compat::Shared<dyn ChatMessageListener>> = vec![(Fetchur::new() as compat::Shared<dyn ChatMessageListener>), (Puzzler::new() as compat::Shared<dyn ChatMessageListener>), (FossilMuncher::new() as compat::Shared<dyn ChatMessageListener>), (Reparty::new() as compat::Shared<dyn ChatMessageListener>), (TreasureHunter::new() as compat::Shared<dyn ChatMessageListener>), (HungryHiker::new() as compat::Shared<dyn ChatMessageListener>), (SweepDetailsListener::new() as compat::Shared<dyn ChatMessageListener>), (CallTrevor::new() as compat::Shared<dyn ChatMessageListener>), (CallMismyla::new() as compat::Shared<dyn ChatMessageListener>), (RedialOnBadSignal::new() as compat::Shared<dyn ChatMessageListener>), (AbilityFilter::new() as compat::Shared<dyn ChatMessageListener>), (AdFilter::new() as compat::Shared<dyn ChatMessageListener>), (AoteFilter::new() as compat::Shared<dyn ChatMessageListener>), (ComboFilter::new() as compat::Shared<dyn ChatMessageListener>), (CritterCaptureFilter::new() as compat::Shared<dyn ChatMessageListener>), (HealFilter::new() as compat::Shared<dyn ChatMessageListener>), (ImplosionFilter::new() as compat::Shared<dyn ChatMessageListener>), (SpiritSceptreFilter::new() as compat::Shared<dyn ChatMessageListener>), (MoltenWaveFilter::new() as compat::Shared<dyn ChatMessageListener>), (RareCropFilter::new() as compat::Shared<dyn ChatMessageListener>), (TeleportPadFilter::new() as compat::Shared<dyn ChatMessageListener>), (AutopetFilter::new() as compat::Shared<dyn ChatMessageListener>), (ShowOffFilter::new() as compat::Shared<dyn ChatMessageListener>), (SkyMallFilter::new() as compat::Shared<dyn ChatMessageListener>), (LotteryFilter::new() as compat::Shared<dyn ChatMessageListener>), (MimicFilter::new() as compat::Shared<dyn ChatMessageListener>), (DeathFilter::new() as compat::Shared<dyn ChatMessageListener>), (DungeonBreakerFilter::new() as compat::Shared<dyn ChatMessageListener>), (SlayerMinibossSpawnFilter::new() as compat::Shared<dyn ChatMessageListener>), (HellionShieldFilter::new() as compat::Shared<dyn ChatMessageListener>)];
    for listener in listeners.iter() {
        // TODO(fabric2seer): `.register(..)` on a Event
        //   java: EVENT.register(listener);
    }
    // TODO(fabric2seer): ClientReceiveMessageEvents.ALLOW_GAME.register: no SeerClient equivalent yet
    //   java: ClientReceiveMessageEvents.ALLOW_GAME.register((message, overlay) -> {
    //   java: 			if (!Utils.isOnSkyblock()) {
    //   java: 				return true;
    //   java: 			}
    //   java:
    //   java: 			ChatFilterResult result = EVENT.invoker().onMessage(message, ChatFormatting.stripFormatting(message.getString()));
    //   java:
    //   java: 			switch (result) {
    //   java: 				case ACTION_BAR -> {
    //   java: 					if (overlay) {
    //   java: 						return true;
    //   java: 					}
    //   java:
    //   java: 					LocalPlayer player = Minecraft.getInstance().player;
    //   java:
    //   java: 					if (player != null) {
    //   java: 						player.sendOverlayMessage(message);
    //   java:
    //   java: 						return false;
    //   java: 					}
    //   java: 				}
    //   java:
    //   java: 				case TOAST -> {
    //   java: 					Minecraft.getInstance().gui.toastManager().addToast(new BasicToast(message, (long) (SkyblockerConfigManager.get().chat.toastDisplayDuration * 1000L), null));
    //   java: 					return false;
    //   java: 				}
    //   java:
    //   java: 				case FILTER -> {
    //   java: 					return false;
    //   java: 				}
    //   java: 			}
    //   java: 			return true;
    //   java: 		});
}
