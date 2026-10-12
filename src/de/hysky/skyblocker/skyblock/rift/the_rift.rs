//! Ported from de.hysky.skyblocker.skyblock.rift.TheRift by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::rift::effigy_waypoints::EffigyWaypoints;
use crate::de::hysky::skyblocker::skyblock::rift::enigma_souls::EnigmaSouls;
use crate::de::hysky::skyblocker::skyblock::rift::mirrorverse_waypoints::MirrorverseWaypoints;
use crate::de::hysky::skyblocker::utils::render::level_render_extraction_callback;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler::Scheduler;

/// java: public class TheRift
#[derive(Clone)]
pub struct TheRift {}

impl TheRift {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        level_render_extraction_callback::EVENT.register(Arc::new(move |__a0| { MirrorverseWaypoints::extract_rendering(&__a0); }));
        level_render_extraction_callback::EVENT.register(Arc::new(move |__a0| { EffigyWaypoints::extract_rendering(&__a0); }));
        level_render_extraction_callback::EVENT.register(Arc::new(move |__a0| { EnigmaSouls::extract_rendering(&__a0); }));
        // TODO(fabric2seer): no SeerClient event yet: hook RETURN of Minecraft's constructor (`net.minecraft.client.Minecraft#<init>`: exists)
        //   java: ClientLifecycleEvents.CLIENT_STARTED.register(MirrorverseWaypoints::load);
        // TODO(fabric2seer): no SeerClient event yet: hook RETURN of Minecraft's constructor (`net.minecraft.client.Minecraft#<init>`: exists)
        //   java: ClientLifecycleEvents.CLIENT_STARTED.register(EnigmaSouls::load);
        // TODO(fabric2seer): no SeerClient event yet: hook HEAD of Minecraft#stop (`net.minecraft.client.Minecraft#stop`: exists)
        //   java: ClientLifecycleEvents.CLIENT_STOPPING.register(EnigmaSouls::save);
        // TODO(fabric2seer): ClientReceiveMessageEvents.ALLOW_GAME.register: no SeerClient equivalent yet
        //   java: ClientReceiveMessageEvents.ALLOW_GAME.register(EnigmaSouls::onMessage);
        client_command_registration_callback::EVENT.register(Arc::new(
            |dispatcher: &mut client_command_registration_callback::ClientCommandDispatcher| {
                // TODO(fabric2seer): a command tree this tool cannot read (arguments, redirects, requires) - SeerClient commands receive the words after the name in `ctx.args`
                //   java: dispatcher.register(literal(SkyblockerMod.NAMESPACE)
                //   java: 				.then(literal("rift")
                //   java: 						.then(literal("enigmaSouls")
                //   java: 								.then(literal("markAllFound").executes(context -> {
                //   java: 									SOUL_WAYPOINTS.values().forEach(Waypoint::setFound);
                //   java: 									context.getSource().sendFeedback(Constants.PREFIX.get().append(Component.translatable("skyblocker.rift.enigmaSouls.markAllFound")));
                //   java:
                //   java: 									return Command.SINGLE_SUCCESS;
                //   java: 								}))
                //   java: 								.then(literal("markAllMissing").executes(context -> {
                //   java: 									SOUL_WAYPOINTS.values().forEach(Waypoint::setMissing);
                //   java: 									context.getSource().sendFeedback(Constants.PREFIX.get().append(Component.translatable("skyblocker.rift.enigmaSouls.markAllMissing")));
                //   java:
                //   java: 									return Command.SINGLE_SUCCESS;
                //   java: 								}))
                //   java: 								.then(literal("markClosestFound").executes(context -> {
                //   java: 									markClosestSoul(true);
                //   java: 									context.getSource().sendFeedback(Constants.PREFIX.get().append(Component.translatable("skyblocker.rift.enigmaSouls.markClosestFound")));
                //   java:
                //   java: 									return Command.SINGLE_SUCCESS;
                //   java: 								}))
                //   java: 								.then(literal("markClosestMissing").executes(context -> {
                //   java: 									markClosestSoul(false);
                //   java: 									context.getSource().sendFeedback(Constants.PREFIX.get().append(Component.translatable("skyblocker.rift.enigmaSouls.markClosestMissing")));
                //   java:
                //   java: 									return Command.SINGLE_SUCCESS;
                //   java: 								})))));
            },
        ));
        ({ let __r = scheduler::INSTANCE.get_mut(); let __f = compat::dispatch(__r, |__t| __t.schedule_cyclic()); __f(__r, Arc::new(move || { EffigyWaypoints::update_effigies(); }), SkyblockerConfigManager::get().slayers.vampire_slayer.effigy_update_frequency) });
    }

}
