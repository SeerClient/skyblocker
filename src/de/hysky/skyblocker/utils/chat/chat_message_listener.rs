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

/// java: Event<ChatMessageListener> EVENT = EventFactory.createArrayBacked(ChatMessageListener.class, listeners -> (message, asString) -> { for (ChatMessageListener listener : listeners) { ChatFilterResult result = listener.onMessage(message, asString); if (result != ChatFilterResult.PASS) return result; } return ChatFilterResult.PASS; });
///
/// Fabric's `Event` is the API's `events::Event` (listeners by phase, then registration); the array-backed invoker is
/// `invoker` below. The listeners are the mod's render-thread objects, so the event lives in a render-thread cell.
pub static EVENT: compat::MainThread<mcrust::mcrust::modding::events::Event<dyn Fn(&MutableComponent, &str) -> ChatFilterResult>> = compat::MainThread::new(|| mcrust::mcrust::modding::events::Event::new());

/// `EVENT.invoker().onMessage(message, asString)`: the first listener's answer that is not `PASS`, else `PASS`.
pub fn invoker(message: &MutableComponent, as_string: &str) -> ChatFilterResult {
    for listener in EVENT.get().listeners() {
        let result: ChatFilterResult = listener(message, as_string);
        if result != ChatFilterResult::Pass {
            return result;
        }
    }
    return ChatFilterResult::Pass;
}

/// java: @FunctionalInterface public interface ChatMessageListener
pub trait ChatMessageListener: compat::DynAny {
    fn as_any(&self) -> &dyn std::any::Any;
    /// java: ChatFilterResult onMessage(Component message, String asString); (dispatch: the body for this class)
    fn on_message(&self) -> fn(&compat::Shared<dyn ChatMessageListener>, &MutableComponent, &str) -> ChatFilterResult;

}

/// java: @SuppressWarnings("incomplete-switch") @Init static void init()
pub fn init() {
    let mut listeners: Vec<compat::Shared<dyn ChatMessageListener>> = vec![(Fetchur::new() as compat::Shared<dyn ChatMessageListener>), (Puzzler::new() as compat::Shared<dyn ChatMessageListener>), (FossilMuncher::new() as compat::Shared<dyn ChatMessageListener>), (Reparty::new() as compat::Shared<dyn ChatMessageListener>), (TreasureHunter::new() as compat::Shared<dyn ChatMessageListener>), (HungryHiker::new() as compat::Shared<dyn ChatMessageListener>), (SweepDetailsListener::new() as compat::Shared<dyn ChatMessageListener>), (CallTrevor::new() as compat::Shared<dyn ChatMessageListener>), (CallMismyla::new() as compat::Shared<dyn ChatMessageListener>), (RedialOnBadSignal::new() as compat::Shared<dyn ChatMessageListener>), (AbilityFilter::new() as compat::Shared<dyn ChatMessageListener>), (AdFilter::new() as compat::Shared<dyn ChatMessageListener>), (AoteFilter::new() as compat::Shared<dyn ChatMessageListener>), (ComboFilter::new() as compat::Shared<dyn ChatMessageListener>), (CritterCaptureFilter::new() as compat::Shared<dyn ChatMessageListener>), (HealFilter::new() as compat::Shared<dyn ChatMessageListener>), (ImplosionFilter::new() as compat::Shared<dyn ChatMessageListener>), (SpiritSceptreFilter::new() as compat::Shared<dyn ChatMessageListener>), (MoltenWaveFilter::new() as compat::Shared<dyn ChatMessageListener>), (RareCropFilter::new() as compat::Shared<dyn ChatMessageListener>), (TeleportPadFilter::new() as compat::Shared<dyn ChatMessageListener>), (AutopetFilter::new() as compat::Shared<dyn ChatMessageListener>), (ShowOffFilter::new() as compat::Shared<dyn ChatMessageListener>), (SkyMallFilter::new() as compat::Shared<dyn ChatMessageListener>), (LotteryFilter::new() as compat::Shared<dyn ChatMessageListener>), (MimicFilter::new() as compat::Shared<dyn ChatMessageListener>), (DeathFilter::new() as compat::Shared<dyn ChatMessageListener>), (DungeonBreakerFilter::new() as compat::Shared<dyn ChatMessageListener>), (SlayerMinibossSpawnFilter::new() as compat::Shared<dyn ChatMessageListener>), (HellionShieldFilter::new() as compat::Shared<dyn ChatMessageListener>)];
    // Register all listeners to EVENT
    for listener in listeners.iter() {
        //   java: EVENT.register(listener);
        let listener = listener.clone();
        EVENT.get().register(Arc::new(move |message: &MutableComponent, as_string: &str| { let __f = compat::dispatch(&listener, |__t| __t.on_message()); __f(&listener, message, as_string) }));
    }
    // Register EVENT to ClientReceiveMessageEvents.ALLOW_GAME from fabric api
    // rust: port gap - `ClientReceiveMessageEvents.ALLOW_GAME` has no SeerClient event, and the port's game-message
    // apply (`client_packet_listener::apply_system_chat`) is not a hooked function (GAPS_batchG L2, GAPS_M #16): the
    // listener below (`allow_game`) is the registered lambda's body, not registered.
    let _ = allow_game;
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

/// The `ClientReceiveMessageEvents.ALLOW_GAME` listener `init` registers: `false` drops the game message.
pub fn allow_game(message: &MutableComponent, overlay: bool) -> bool {
    use mcrust::net::minecraft::network::chat::component::Component as _;
    if !crate::de::hysky::skyblocker::utils::utils::Utils::is_on_skyblock() {
        return true;
    }
    //   java: ChatFilterResult result = EVENT.invoker().onMessage(message, ChatFormatting.stripFormatting(message.getString()));
    let result: ChatFilterResult = invoker(message, &mcrust::net::minecraft::chat_formatting::strip_formatting(&message.get_string()));
    match result {
        ChatFilterResult::ActionBar => {
            if overlay {
                return true;
            }
            //   java: LocalPlayer player = Minecraft.getInstance().player; if (player != null) { player.sendOverlayMessage(message); return false; }
            // `LocalPlayer.sendOverlayMessage` is `minecraft.gui.hud.setOverlayMessage(message, false)`.
            if mcrust::mcrust::modding::game::with_player(|_| ()).is_some() {
                mcrust::net::minecraft::client::gui::hud::set_overlay_message(&mut compat::minecraft().gui.hud, mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(message.clone()), false);
                return false;
            }
        }
        ChatFilterResult::Toast => {
            //   java: Minecraft.getInstance().gui.toastManager().addToast(new BasicToast(message, (long) (SkyblockerConfigManager.get().chat.toastDisplayDuration * 1000L), null));
            compat::minecraft().gui.toast_manager().borrow_mut().add_toast(Box::new(crate::de::hysky::skyblocker::utils::render::gui::basic_toast::BasicToast::new_3(message.clone(), (crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().chat.toast_display_duration * 1000.0f32) as i64, None)));
            return false;
        }
        ChatFilterResult::Filter => {
            return false;
        }
        ChatFilterResult::Pass => {}
    }
    return true;
}
