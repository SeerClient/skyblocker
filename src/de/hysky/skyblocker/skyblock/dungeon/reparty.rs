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
use mcrust::com::mojang::brigadier::builder::argument_builder::ArgumentBuilder as _;

/// java: private static final int BASE_DELAY = 10;
pub const BASE_DELAY: i32 = 10;

/// The one `Reparty` (`ChatMessageListener.init` makes it): the `/reparty` command and the party-info packet reach it
/// here (`this::executeCommand`, `this::onPacket`).
pub static INSTANCE: compat::MainThread<Option<compat::Shared<Reparty>>> = compat::MainThread::new(|| None);

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
        *INSTANCE.get_mut() = Some(__this.clone());
        // java: HypixelPacketEvents.PARTY_INFO.register(this::onPacket);
        crate::net::azureaaron::hmapi::events::hypixel_packet_events::PARTY_INFO.get_mut().register(Arc::new(|packet: &crate::unresolved::HypixelS2CPacket| {
            if let Some(this) = INSTANCE.get().clone() {
                Reparty::on_packet(&this, packet);
            }
        }));
        client_command_registration_callback::EVENT.register(Arc::new(
            |dispatcher: &mut client_command_registration_callback::ClientCommandDispatcher| {
                // java: dispatcher.register(ClientCommands.literal("reparty").executes(this::executeCommand));
                // java: dispatcher.register(ClientCommands.literal("rp").executes(this::executeCommand));
                for name in ["reparty", "rp"] {
                    let mut node = client_command_registration_callback::literal(name);
                    node.executes(Arc::new(|source: &CommandContext<client_command_registration_callback::FabricClientCommandSource>| {
                        Ok(match INSTANCE.get().clone() { Some(this) => Reparty::execute_command(&this, source), None => 0 })
                    }));
                    dispatcher.dispatcher().register(&node);
                }
            },
        ));
        __this
    }

    /// java: private int executeCommand(CommandContext<FabricClientCommandSource> source)
    pub fn execute_command(__this: &compat::Shared<Self>, _source: &CommandContext<client_command_registration_callback::FabricClientCommandSource>) -> i32 {
        if !(Utils::is_on_skyblock()) || compat::read(__this, |__t| __t.repartying) || mcrust::mcrust::modding::game::with_player(|_| ()).is_none() {
            return 0;
        }
        { let __v = true; compat::write(__this, |__t| __t.repartying = __v); }
        // java: HypixelNetworking.sendPartyInfoC2SPacket(2);
        crate::net::azureaaron::hmapi::network::hypixel_networking::HypixelNetworking::send_party_info_c2s_packet(2);
        return 1;
    }

    /// java: private void onPacket(HypixelS2CPacket packet)
    pub fn on_packet(__this: &compat::Shared<Self>, packet: &crate::unresolved::HypixelS2CPacket) {
        use crate::net::azureaaron::hmapi::data::party::party_role::PartyRole;
        use crate::net::azureaaron::hmapi::network::packet::v2::s2c::party_info_s2c_packet::PartyInfoS2CPacket;
        let repartying = compat::read(__this, |__t| __t.repartying);
        match packet {
            // java: case PartyInfoS2CPacket(var inParty, var members) when this.repartying -> {..}
            crate::unresolved::HypixelS2CPacket::PartyInfo(info) if repartying => {
                // java: UUID ourUuid = Objects.requireNonNull(CLIENT.getUser().getProfileId());
                let our_uuid: String = compat::minecraft().get_user().get_profile_id().replace("-", "").to_lowercase();
                let our_role = info.members.as_ref().and_then(|members| members.iter().find(|(uuid, _)| uuid.to_string().replace("-", "") == our_uuid).map(|(_, role)| *role));
                if info.in_party && info.members.is_some() && our_role == Some(PartyRole::Leader) {
                    Reparty::send_command(__this, "/p disband", 1);
                    let mut count: i32 = 0;
                    for (uuid, role) in info.members.as_ref().unwrap().iter() {
                        // Don't invite ourselves
                        if *role != PartyRole::Leader {
                            count += 1;
                            Reparty::send_command(__this, &format!("/p {}", uuid), count + 2);
                        }
                    }
                    // java: Scheduler.INSTANCE.schedule(() -> this.repartying = false, count * BASE_DELAY);
                    let __this_1 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
                    let __r = scheduler::INSTANCE.get_mut();
                    let __f = compat::dispatch(__r, |__t| __t.schedule());
                    __f(__r, Arc::new(move || { if let Some(this) = __this_1.get().upgrade() { compat::write(&this, |__t| __t.repartying = false); } }), count.wrapping_mul(BASE_DELAY));
                } else {
                    // java: CLIENT.player.sendSystemMessage(Constants.PREFIX.get().append(Component.translatable("skyblocker.reparty.notInPartyOrNotLeader")));
                    let message = (crate::de::hysky::skyblocker::utils::constants::PREFIX.get())().append(mcrust::net::minecraft::network::chat::component::translatable("skyblocker.reparty.notInPartyOrNotLeader"));
                    compat::batch_w_send_feedback(&message);
                    compat::write(__this, |__t| __t.repartying = false);
                }
            }
            // java: case ErrorS2CPacket(var id, var error) when id.equals(PartyInfoS2CPacket.ID) && this.repartying -> {..}
            crate::unresolved::HypixelS2CPacket::Error(error) if error.id == PartyInfoS2CPacket::ID && repartying => {
                let message = (crate::de::hysky::skyblocker::utils::constants::PREFIX.get())().append(mcrust::net::minecraft::network::chat::component::translatable("skyblocker.reparty.error"));
                compat::batch_w_send_feedback(&message);
                logger().error(&format!("[Skyblocker Reparty] The party info packet returned an unexpected error! {:?}", error.reason));
                compat::write(__this, |__t| __t.repartying = false);
            }
            // java: default -> {} //Do nothing
            _ => {}
        }
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
