//! Ported from de.hysky.skyblocker.skyblock.dungeon.Reparty by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::utils::chat::chat_filter_result::ChatFilterResult;
use crate::de::hysky::skyblocker::utils::chat::chat_message_listener::ChatMessageListener;
use crate::de::hysky::skyblocker::utils::chat::chat_pattern_listener::ChatPatternListener;
use crate::de::hysky::skyblocker::utils::chat::chat_pattern_listener::ChatPatternListenerBase;
use crate::de::hysky::skyblocker::utils::scheduler::message_scheduler;
use crate::de::hysky::skyblocker::utils::scheduler::message_scheduler::MessageScheduler;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler::Scheduler;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::com::mojang::brigadier::context::command_context::CommandContext;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: private static final int BASE_DELAY = 10;
pub const BASE_DELAY: i32 = 10;

/// java: public class Reparty extends ChatPatternListener
#[derive(Clone)]
pub struct Reparty {
    /// java: extends ChatPatternListener
    pub base: ChatPatternListenerBase,
    /// java: private boolean repartying = false;
    pub repartying: bool,
    /// java: private String partyLeader = "";
    pub party_leader: String,
}

impl std::ops::Deref for Reparty {
    type Target = ChatPatternListenerBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for Reparty {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl Reparty {
    /// java: public Reparty()
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = ChatPatternListenerBase::new_in(String::from("^(?:([\\[A-z+\\]]* )?(?<disband>.*) has disbanded .*|.*\n([\\[A-z+\\]]* )?(?<invite>.*) has invited you to join their party!\nYou have 60 seconds to accept. Click here to join!\n.*)$"), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                repartying: false,
                party_leader: "".to_string(),
            });
            ((__h.clone() as compat::Shared<dyn ChatPatternListener>), __h)
        });
        // TODO(fabric2seer): static field HypixelPacketEvents.PARTY_INFO
        //   java: HypixelPacketEvents.PARTY_INFO.register(this::onPacket);
        client_command_registration_callback::EVENT.register(Arc::new(
            |dispatcher: &mut client_command_registration_callback::ClientCommandDispatcher| {
                // TODO(fabric2seer): method reference `::executeCommand` to a method this tool cannot see
                //   java: dispatcher.register(ClientCommands.literal("reparty").executes(this::executeCommand));
                // TODO(fabric2seer): method reference `::executeCommand` to a method this tool cannot see
                //   java: dispatcher.register(ClientCommands.literal("rp").executes(this::executeCommand));
            },
        ));
        __this
    }

    /// java: private int executeCommand(CommandContext<FabricClientCommandSource> source)
    pub fn execute_command(__this: &compat::Shared<Self>, source: &CommandContext<crate::unresolved::FabricClientCommandSource>) -> i32 {
        if !(Utils::is_on_skyblock()) || compat::read(__this, |__t| __t.repartying) || mcrust::mcrust::modding::game::with_player(|_| ()).is_none() {
            return 0;
        }
        { let __v = true; compat::write(__this, |__t| __t.repartying = __v); }
        // TODO(fabric2seer): static call HypixelNetworking.sendPartyInfoC2SPacket(..)
        //   java: HypixelNetworking.sendPartyInfoC2SPacket(2);
        return 1;
    }

    /// java: private void onPacket(HypixelS2CPacket packet)
    pub fn on_packet(__this: &compat::Shared<Self>, packet: &crate::unresolved::HypixelS2CPacket) {
        // TODO(fabric2seer): not translated (record patterns are not translated)
        //   java: switch (packet) {
        //   java: 			case PartyInfoS2CPacket(var inParty, var members) when this.repartying -> {
        //   java: 				UUID ourUuid = Objects.requireNonNull(CLIENT.getUser().getProfileId());
        //   java:
        //   java: 				if (inParty && members != null && members.get(ourUuid) == PartyRole.LEADER) {
        //   java: 					sendCommand("/p disband", 1);
        //   java: 					int count = 0;
        //   java:
        //   java: 					for (Map.Entry<UUID, PartyRole> entry : members.entrySet()) {
        //   java: 						UUID uuid = entry.getKey();
        //   java: 						PartyRole role = entry.getValue();
        //   java:
        //   java: 						//Don't invite ourselves
        //   java: 						if (role != PartyRole.LEADER) sendCommand("/p " + uuid, ++count + 2);
        //   java: 					}
        //   java:
        //   java: 					Scheduler.INSTANCE.schedule(() -> this.repartying = false, count * BASE_DELAY);
        //   java: 				} else {
        //   java: 					assert CLIENT.player != null;
        //   java: 					CLIENT.player.sendSystemMessage(Constants.PREFIX.get().append(Component.translatable("skyblocker.reparty.notInPartyOrNotLeader")));
        //   java: 					this.repartying = false;
        //   java: 				}
        //   java: 			}
        //   java:
        //   java: 			case ErrorS2CPacket(var id, var error) when id.equals(PartyInfoS2CPacket.ID) && this.repartying -> {
        //   java: 				assert CLIENT.player != null;
        //   java: 				CLIENT.player.sendSystemMessage(Constants.PREFIX.get().append(Component.translatable("skyblocker.reparty.error")));
        //   java: 				LOGGER.error("[Skyblocker Reparty] The party info packet returned an unexpected error! {}", error);
        //   java:
        //   java: 				this.repartying = false;
        //   java: 			}
        //   java:
        //   java: 			default -> {} //Do nothing
        //   java: 		}
    }

    /// java: private void sendCommand(String command, int delay)
    pub fn send_command(__this: &compat::Shared<Self>, command: &str, mut delay: i32) {
        MessageScheduler::queue_message(message_scheduler::INSTANCE.get_mut(), command, true, delay.wrapping_mul(BASE_DELAY));
    }

}

impl ChatPatternListener for Reparty {
    fn chat_pattern_listener_base(&self) -> &ChatPatternListenerBase {
        &self.base
    }
    fn chat_pattern_listener_base_mut(&mut self) -> &mut ChatPatternListenerBase {
        &mut self.base
    }

    /// java: @Override public ChatFilterResult state() (dispatch: the body for this class)
    fn state(&self) -> fn(&compat::Shared<dyn ChatPatternListener>) -> Option<ChatFilterResult> {
        Reparty::state_dyn
    }

    /// java: @Override public boolean onMatch(Component message, Matcher matcher) (dispatch: the body for this class)
    fn on_match(&self) -> fn(&compat::Shared<dyn ChatPatternListener>, &MutableComponent, &compat::regex::Matcher) -> bool {
        Reparty::on_match_dyn
    }

}

impl ChatMessageListener for Reparty {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public final ChatFilterResult onMessage(Component message, String asString) (dispatch: the body for this class)
    fn on_message(&self) -> fn(&compat::Shared<dyn ChatMessageListener>, &MutableComponent, &str) -> ChatFilterResult {
        <dyn ChatPatternListener>::on_message_from::<Self>
    }

}

impl Reparty {
    /// java: @Override public ChatFilterResult state()
    pub fn state_impl(__this: &compat::Shared<Self>) -> ChatFilterResult {
        return (if SkyblockerConfigManager::get().general.accept_reparty { ChatFilterResult::Filter } else { ChatFilterResult::Pass });
    }

    /// Reparty.state as the dispatch of ChatPatternListener.state
    pub fn state_dyn(__this: &compat::Shared<dyn ChatPatternListener>) -> Option<ChatFilterResult> {
        Some(Self::state_impl(&compat::view_as::<Self, _, Self>(__this, |a| a)))
    }

    /// java: @Override public boolean onMatch(Component message, Matcher matcher)
    pub fn on_match_impl(__this: &compat::Shared<Self>, message: &MutableComponent, matcher: &compat::regex::Matcher) -> bool {
        if matcher.group_named("disband").is_some() && !(matcher.group_named("disband").as_deref() == Some(&*(compat::minecraft().get_user().get_name()))) {
            { let __v = matcher.group_named("disband").unwrap(); compat::write(__this, |__t| __t.party_leader = __v); }
            let __this_1 = compat::RenderThread::new(__this.clone());
            ({ let __r = scheduler::INSTANCE.get_mut(); let __f = compat::dispatch(__r, |__t| __t.schedule()); __f(__r, Arc::new(move || { { let __v = "".to_string(); compat::write(__this_1.get(), |__t| __t.party_leader = __v); } }), 61) });
        } else if matcher.group_named("invite").is_some() && matcher.group_named("invite").as_deref() == Some(&*(compat::read(__this, |__t| __t.party_leader.clone()))) {
            let mut command: String = format!("/party accept {}", compat::read(__this, |__t| __t.party_leader.clone()));
            Reparty::send_command(__this, &command, 0);
        }
        return false;
    }

    /// Reparty.onMatch as the dispatch of ChatPatternListener.onMatch
    pub fn on_match_dyn(__this: &compat::Shared<dyn ChatPatternListener>, __p0: &MutableComponent, __p1: &compat::regex::Matcher) -> bool {
        Self::on_match_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1)
    }

}
