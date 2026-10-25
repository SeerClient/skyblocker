//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.ClientPacketListenerMixin by fabric2seer.
//! @Mixin(net.minecraft.client.multiplayer.ClientPacketListener)

#[allow(unused_imports)]
use crate::prelude::*;

/// @Inject skyblocker$onEntityTrackerUpdate
///   java: @Inject(method = "handleSetEntityData", at = @At("TAIL"))
/// target: net.minecraft.client.multiplayer.ClientPacketListener#handleSetEntityData  [exists]
/// Rust: net/minecraft/client/multiplayer/client_packet_listener.rs:7021 fn handle_set_entity_data(&mut self, packet: &ClientboundSetEntityDataPacket)
fn skyblocker_on_entity_tracker_update(ci: &mut CallbackInfo<'_>) {
    let Some(packet) = compat::detach_ref(ci.arg::<mcrust::net::minecraft::network::protocol::game::clientbound_set_entity_data_packet::ClientboundSetEntityDataPacket>(0)) else { return; };
    // `@Local(name = "entity")` is `:639`'s `this.level.getEntity(packet.id())`; at TAIL the handler has let it go.
    mcrust::mcrust::modding::game::with_entity(packet.id(), |entity| {
        let entity: &mcrust::net::minecraft::client::multiplayer::client_level::StoredEntity = &*entity;
        let Some(armor_stand_entity) = entity.as_any().and_then(|a| a.downcast_ref::<mcrust::net::minecraft::world::entity::decoration::armor_stand::ArmorStand>()) else {
            return;
        };
        crate::de::hysky::skyblocker::skyblock::slayers::slayer_manager::SlayerManager::check_slayer_boss(armor_stand_entity);
        crate::de::hysky::skyblocker::skyblock::slayers::boss::voidgloom::beacon_highlighter::BeaconHighlighter::on_entity_spawn(armor_stand_entity);
        crate::de::hysky::skyblocker::skyblock::slayers::boss::demonlord::fire_pillar_announcer::FirePillarAnnouncer::check_fire_pillar(entity);
        // java: CorpseFinder.checkIfCorpse(armorStandEntity) - the ArmorStand overload
        crate::de::hysky::skyblocker::skyblock::dwarven::corpse_finder::CorpseFinder::check_if_corpse_armor_stand(armor_stand_entity);
        crate::de::hysky::skyblocker::skyblock::health_bars::HealthBars::health_bar(armor_stand_entity);
        crate::de::hysky::skyblocker::skyblock::fishing::sea_creature_tracker::SeaCreatureTracker::on_entity_spawn(armor_stand_entity);
        crate::de::hysky::skyblocker::skyblock::fishing::fishing_helper::FishingHelper::check_if_fish_was_caught(armor_stand_entity);
        crate::de::hysky::skyblocker::skyblock::foraging::galatea::tree_break_progress_hud::TreeBreakProgressHud::on_entity_update(armor_stand_entity);
        crate::de::hysky::skyblocker::skyblock::hunting::lasso_hud::LassoHud::on_entity_update(armor_stand_entity);
        // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            crate::de::hysky::skyblocker::skyblock::compact_damage::CompactDamage::compact_damage(armor_stand_entity);
        }));
        if let Err(e) = __result {
            logger().error(&format!("[Skyblocker Compact Damage] Failed to compact damage number: {}", compat::panic_message(&e)));
        }
        crate::de::hysky::skyblocker::skyblock::fishing::fishing_hook_display_helper::FishingHookDisplayHelper::on_armor_stand_spawn(armor_stand_entity);
    });
}

/// @Inject skyblocker$onEntityAttach
///   java: @Inject(method = "handleEntityLinkPacket", at = @At("TAIL"))
/// target: net.minecraft.client.multiplayer.ClientPacketListener#handleEntityLinkPacket  [exists]
/// Rust: net/minecraft/client/multiplayer/client_packet_listener.rs:6707 fn handle_entity_link_packet(&mut self, packet: &crate::net::minecraft::network::protocol::game::clientbound_set_entity_link_packet::ClientboundSetEntityLinkPacket)
fn skyblocker_on_entity_attach(ci: &mut CallbackInfo<'_>) {
    let Some(this) = compat::detach(ci.this_mut::<mcrust::net::minecraft::client::multiplayer::client_packet_listener::ClientPacketListener>()) else { return; };
    let Some(packet) = compat::detach_ref(ci.arg::<mcrust::net::minecraft::network::protocol::game::clientbound_set_entity_link_packet::ClientboundSetEntityLinkPacket>(0)) else { return; };
    crate::de::hysky::skyblocker::skyblock::hunting::lasso_hud::LassoHud::on_entity_attach(&packet);
}

/// @Inject skyblocker$beforeTeleport
///   java: @Inject(method = "handleMovePlayer", at = @At(value = "INVOKE", target = "Lnet/minecraft/network/protocol/PacketUtils;ensureRunningOnSameThread(Lnet/minecraft/network/protocol/Packet;Lnet/minecraft/network/PacketListener;Lnet/minecraft/network/PacketProcessor;)V", shift = At.Shift.AFTER))
/// target: net.minecraft.network.protocol.PacketUtils#ensureRunningOnSameThread(Lnet/minecraft/network/protocol/Packet;Lnet/minecraft/network/PacketListener;Lnet/minecraft/network/PacketProcessor;)V  [exists]
/// within: net.minecraft.client.multiplayer.ClientPacketListener#handleMovePlayer
/// Rust: net/minecraft/network/protocol/packet_utils.rs:53 fn ensure_running_on_same_thread(packet: Box<dyn Packet>, packet_processor: &mut PacketProcessor)
/// note: @Inject at INVOKE: a call-site injector (`hook::call_site`) - `ci` is the CALL's (its arguments; cancel() skips that call, not the rest of the method)
/// `@Share("playerBeforeTeleportBlockPos") LocalRef<BlockPos>` - one value per run of `handleMovePlayer`, shared by
/// the two injectors below (both run on the render thread, one run at a time).
static PLAYER_BEFORE_TELEPORT_BLOCK_POS: LazyLock<Mutex<Option<mcrust::net::minecraft::core::block_pos::BlockPos>>> = LazyLock::new(|| Mutex::new(None));

/// `@At(value = "INVOKE", target = "PacketUtils.ensureRunningOnSameThread", shift = AFTER)`: the port's handlers
/// reach that check through `ClientCommonPacketListenerImpl::ensure_running_on_same_thread[_for]`, which call the
/// unhooked `_borrowed` / `_for` forms, so that call site never fires (GAPS_batchD). "After the check passed" is
/// the handler's HEAD on the render thread: on any other thread the check throws and the handler re-runs there.
fn after_ensure_running_on_same_thread() -> bool {
    mcrust::com::mojang::blaze3d::systems::render_system::is_on_render_thread()
}

fn skyblocker_before_teleport(ci: &mut CallbackInfo<'_>) {
    if !after_ensure_running_on_same_thread() {
        return;
    }
    compat::set_state(&PLAYER_BEFORE_TELEPORT_BLOCK_POS, mcrust::mcrust::modding::game::with_player(|player| player.base.base.base.base.block_position().immutable()));
    crate::de::hysky::skyblocker::skyblock::teleport::responsive_smooth_aote::ResponsiveSmoothAOTE::player_going_to_teleport();
}

/// @Inject skyblocker$onTeleport
///   java: @Inject(method = "handleMovePlayer", at = @At(value = "RETURN"))
/// target: net.minecraft.client.multiplayer.ClientPacketListener#handleMovePlayer  [exists]
/// Rust: net/minecraft/client/multiplayer/client_packet_listener.rs:4246 fn handle_move_player(&mut self, packet: &ClientboundPlayerPositionPacket)
fn skyblocker_on_teleport(ci: &mut CallbackInfo<'_>) {
    let Some(this) = compat::detach(ci.this_mut::<mcrust::net::minecraft::client::multiplayer::client_packet_listener::ClientPacketListener>()) else { return; };
    let Some(packet) = compat::detach_ref(ci.arg::<mcrust::net::minecraft::network::protocol::game::clientbound_player_position_packet::ClientboundPlayerPositionPacket>(0)) else { return; };
    crate::de::hysky::skyblocker::skyblock::teleport::predictive_smooth_aote::PredictiveSmoothAOTE::player_teleported();
    let before_teleport = compat::get_state(&PLAYER_BEFORE_TELEPORT_BLOCK_POS);
    let after_teleport = mcrust::mcrust::modding::game::with_player(|player| player.base.base.base.base.block_position().immutable());
    if let (Some(from), Some(to)) = (before_teleport, after_teleport) {
        crate::de::hysky::skyblocker::skyblock::dungeon::puzzle::teleport_maze::TeleportMaze::on_teleport(crate::de::hysky::skyblocker::skyblock::dungeon::puzzle::teleport_maze::INSTANCE.get(), compat::minecraft(), &from, &to);
    }
}

/// @Inject skyblocker$onItemPickup
///   java: @Inject(method = "handleTakeItemEntity", at = @At(value = "INVOKE", target = "Lnet/minecraft/world/entity/item/ItemEntity;getItem()Lnet/minecraft/world/item/ItemStack;"))
/// target: net.minecraft.world.entity.item.ItemEntity#getItem  [exists]
/// within: net.minecraft.client.multiplayer.ClientPacketListener#handleTakeItemEntity
/// Rust: net/minecraft/world/entity/item/item_entity.rs:228 fn get_item(&self)
/// note: @Inject at INVOKE: a call-site injector (`hook::call_site`) - `ci` is the CALL's (its arguments; cancel() skips that call, not the rest of the method)
fn skyblocker_on_item_pickup(ci: &mut CallbackInfo<'_>) {
    // `@Local(name = "itemEntity")` is the receiver of the `getItem()` call this injector sits on.
    let Some(item_entity) = compat::detach_ref(ci.this::<mcrust::net::minecraft::world::entity::item::item_entity::ItemEntity>()) else { return; };
    crate::de::hysky::skyblocker::skyblock::dungeon::secrets::dungeon_manager::DungeonManager::on_item_pickup(item_entity);
}

/// @WrapWithCondition skyblocker$cancelEntityPassengersWarning
///   java: @WrapWithCondition(method = "handleSetEntityPassengersPacket", at = @At(value = "INVOKE", target = "Lorg/slf4j/Logger;warn(Ljava/lang/String;)V"))
/// target: org.slf4j.Logger#warn(Ljava/lang/String;)V  [exists: org/slf4j/logger.rs]
/// within: net.minecraft.client.multiplayer.ClientPacketListener#handleSetEntityPassengersPacket
fn skyblocker_cancel_entity_passengers_warning(ci: &CallbackInfo<'_>) -> bool {
    return !(crate::de::hysky::skyblocker::utils::utils::Utils::is_on_hypixel());
}

/// @ModifyExpressionValue skyblocker$onEntityDeath
///   java: @ModifyExpressionValue(method = "handleEntityEvent", at = @At(value = "INVOKE", target = "Lnet/minecraft/network/protocol/game/ClientboundEntityEventPacket;getEntity(Lnet/minecraft/world/level/Level;)Lnet/minecraft/world/entity/Entity;"))
/// target: net.minecraft.network.protocol.game.ClientboundEntityEventPacket#getEntity  [exists]
/// within: net.minecraft.client.multiplayer.ClientPacketListener#handleEntityEvent
/// Rust: net/minecraft/network/protocol/game/clientbound_entity_event_packet.rs:92 fn get_entity_id(&self)
fn skyblocker_on_entity_death(ci: &mut CallbackInfo<'_>) {
    // The port's `getEntity(Level)` answers the entity ID (`ClientboundEntityEventPacket::get_entity_id`) and the
    // handler hands the value back unchanged, so it runs after the call: `packet` is the call's receiver.
    let Some(packet) = compat::detach_ref(ci.this::<mcrust::net::minecraft::network::protocol::game::clientbound_entity_event_packet::ClientboundEntityEventPacket>()) else { return; };
    let Some(entity_id) = ci.return_value::<i32>().copied() else { return; };
    if packet.get_event_id() == mcrust::net::minecraft::world::entity::entity_event::DEATH {
        mcrust::mcrust::modding::game::with_entity(entity_id, |entity| {
            let entity: &mcrust::net::minecraft::client::multiplayer::client_level::StoredEntity = &*entity;
            crate::de::hysky::skyblocker::skyblock::dungeon::dungeon_score::DungeonScore::handle_entity_death(entity);
            crate::de::hysky::skyblocker::skyblock::end::the_end::TheEnd::on_entity_death(entity);
        });
    }
}

/// @Inject skyblocker$onEntityEquip
///   java: @Inject(method = "handleSetEquipment", at = @At(value = "TAIL"))
/// target: net.minecraft.client.multiplayer.ClientPacketListener#handleSetEquipment  [exists]
/// Rust: net/minecraft/client/multiplayer/client_packet_listener.rs:5062 fn handle_set_equipment(&mut self, packet: &ClientboundSetEquipmentPacket)
fn skyblocker_on_entity_equip(ci: &mut CallbackInfo<'_>) {
    let Some(this) = compat::detach(ci.this_mut::<mcrust::net::minecraft::client::multiplayer::client_packet_listener::ClientPacketListener>()) else { return; };
    let Some(packet) = compat::detach_ref(ci.arg::<mcrust::net::minecraft::network::protocol::game::clientbound_set_equipment_packet::ClientboundSetEquipmentPacket>(0)) else { return; };
    // `@Local Entity entity` is `this.level.getEntity(packet.getEntity())`; at TAIL the handler has let it go.
    mcrust::mcrust::modding::game::with_entity(packet.get_entity(), |entity| {
        crate::de::hysky::skyblocker::skyblock::dwarven::corpse_finder::CorpseFinder::check_if_corpse(&*entity);
    });
}

/// @Inject skyblocker$updatePlayerListFooter
///   java: @Inject(method = "handleTabListCustomisation", at = @At(value = "INVOKE", target = "Lnet/minecraft/client/gui/components/PlayerTabOverlay;setFooter(Lnet/minecraft/network/chat/Component;)V"))
/// target: net.minecraft.client.gui.components.PlayerTabOverlay#setFooter  [exists]
/// within: net.minecraft.client.multiplayer.ClientPacketListener#handleTabListCustomisation
/// Rust: net/minecraft/client/gui/components/player_tab_overlay.rs:1726 fn set_footer(&mut self, footer: Option<crate::net::minecraft::network::chat::formatted_text::FormattedText>)
/// note: @Inject at INVOKE: a call-site injector (`hook::call_site`) - `ci` is the CALL's (its arguments; cancel() skips that call, not the rest of the method)
/// The port's handler parks `:1980-1981`'s `setHeader` / `setFooter` for the client loop
/// (`PlayPendingAction::TabListCustomisation`), so the `setFooter` call site never runs inside it; this injector is
/// registered at the handler's RETURN instead, which is where Java's `setFooter` statement sits (`:1981`, its last).
fn skyblocker_update_player_list_footer(ci: &mut CallbackInfo<'_>) {
    let Some(packet) = compat::detach_ref(ci.arg::<mcrust::net::minecraft::network::protocol::game::clientbound_tab_list_packet::ClientboundTabListPacket>(0)) else { return; };
    crate::de::hysky::skyblocker::skyblock::tabhud::util::player_list_manager::PlayerListManager::update_footer(Some(compat::component_of(packet.footer().clone())));
}

/// @Inject skyblocker$onListChange
///   java: @Inject(method = "handlePlayerInfoUpdate", at = @At("HEAD"))
/// target: net.minecraft.client.multiplayer.ClientPacketListener#handlePlayerInfoUpdate  [exists]
/// Rust: net/minecraft/client/multiplayer/client_packet_listener.rs:9334 fn handle_player_info_update(&mut self, packet: &super::super::super::network::protocol::game::clientbound_player_info_update_packet::ClientboundPlayerInfoUpdatePacket)
fn skyblocker_on_list_change(ci: &mut CallbackInfo<'_>) {
    let Some(this) = compat::detach(ci.this_mut::<mcrust::net::minecraft::client::multiplayer::client_packet_listener::ClientPacketListener>()) else { return; };
    compat::set_state(&crate::de::hysky::skyblocker::skyblock::tabhud::util::player_list_manager::SHOULD_UPDATE_NEXT_TICK, true);
}

/// @WrapWithCondition skyblocker$cancelPlayerListWarning
///   java: @WrapWithCondition(method = "handlePlayerInfoUpdate", at = @At(value = "INVOKE", target = "Lorg/slf4j/Logger;warn(Ljava/lang/String;Ljava/lang/Object;Ljava/lang/Object;)V"))
/// target: org.slf4j.Logger#warn(Ljava/lang/String;Ljava/lang/Object;Ljava/lang/Object;)V  [exists: org/slf4j/logger.rs]
/// within: net.minecraft.client.multiplayer.ClientPacketListener#handlePlayerInfoUpdate
fn skyblocker_cancel_player_list_warning(ci: &CallbackInfo<'_>) -> bool {
    return !(crate::de::hysky::skyblocker::utils::utils::Utils::is_on_hypixel());
}

/// @Inject skyblocker$onPlaySound
///   java: @Inject(method = "handleSoundEvent", at = @At(value = "INVOKE", target = "Lnet/minecraft/network/protocol/PacketUtils;ensureRunningOnSameThread(Lnet/minecraft/network/protocol/Packet;Lnet/minecraft/network/PacketListener;Lnet/minecraft/network/PacketProcessor;)V", shift = At.Shift.AFTER))
/// target: net.minecraft.network.protocol.PacketUtils#ensureRunningOnSameThread(Lnet/minecraft/network/protocol/Packet;Lnet/minecraft/network/PacketListener;Lnet/minecraft/network/PacketProcessor;)V  [exists]
/// within: net.minecraft.client.multiplayer.ClientPacketListener#handleSoundEvent
/// Rust: net/minecraft/network/protocol/packet_utils.rs:53 fn ensure_running_on_same_thread(packet: Box<dyn Packet>, packet_processor: &mut PacketProcessor)
/// note: @Inject at INVOKE: a call-site injector (`hook::call_site`) - `ci` is the CALL's (its arguments; cancel() skips that call, not the rest of the method)
fn skyblocker_on_play_sound(ci: &mut CallbackInfo<'_>) {
    if !after_ensure_running_on_same_thread() {
        return;
    }
    let Some(packet) = compat::detach_ref(ci.arg::<mcrust::net::minecraft::network::protocol::game::clientbound_sound_packet::ClientboundSoundPacket>(0)) else { return; };
    (crate::de::hysky::skyblocker::events::play_sound_events::from_server_invoker())(packet.clone());
}

/// @WrapWithCondition skyblocker$dropBadlionPacketWarnings
///   java: @WrapWithCondition(method = "handleUnknownCustomPayload", at = @At(value = "INVOKE", target = "Lorg/slf4j/Logger;warn(Ljava/lang/String;Ljava/lang/Object;)V"))
/// target: org.slf4j.Logger#warn(Ljava/lang/String;Ljava/lang/Object;)V  [exists: org/slf4j/logger.rs]
/// within: net.minecraft.client.multiplayer.ClientPacketListener#handleUnknownCustomPayload
///
/// `handleUnknownCustomPayload:2198-2200` is that one `LOGGER.warn` call and nothing else; the port's body is the
/// warning (client_packet_listener.rs:2199, a `warn_once`, not a hookable `Logger` call). So "skip the warn" is "skip
/// the body": a HEAD injector that cancels when the condition says not to log. `identifier` is `payload.type().id()`,
/// the port method's `id` argument (0).
fn skyblocker_drop_badlion_packet_warnings(ci: &CallbackInfo<'_>) -> bool {
    let Some(identifier) = ci.arg::<str>(0) else { return true; };
    let namespace: String = mcrust::net::minecraft::resources::identifier::Identifier::try_parse(identifier).map(|identifier| identifier.namespace().to_string()).unwrap_or_default();
    //   java: return !(Utils.isOnHypixel() && ((Identifier) identifier).getNamespace().equals("badlion"));
    return !(crate::de::hysky::skyblocker::utils::utils::Utils::is_on_hypixel() && namespace == "badlion");
}

/// HEAD of `handleUnknownCustomPayload`: the `@WrapWithCondition` above over the method's only statement.
fn skyblocker_drop_badlion_packet_warnings_at_head(ci: &mut CallbackInfo<'_>) {
    if !skyblocker_drop_badlion_packet_warnings(ci) {
        ci.cancel();
    }
}

/// @WrapWithCondition skyblocker$cancelUnknownScoreboardObjectiveWarnings
///   java: @WrapWithCondition(method = {"handleSetScore", "handleResetScore"}, at = @At(value = "INVOKE", target = "Lorg/slf4j/Logger;warn(Ljava/lang/String;Ljava/lang/Object;)V"), require = 2)
/// target: org.slf4j.Logger#warn(Ljava/lang/String;Ljava/lang/Object;)V  [exists: org/slf4j/logger.rs]
/// within: net.minecraft.client.multiplayer.ClientPacketListener#handleSetScore
fn skyblocker_cancel_unknown_scoreboard_objective_warnings(ci: &CallbackInfo<'_>) -> bool {
    return !(crate::de::hysky::skyblocker::utils::utils::Utils::is_on_hypixel());
}

/// @WrapWithCondition skyblocker$cancelUnknownScoreboardObjectiveWarnings
///   java: @WrapWithCondition(method = {"handleSetScore", "handleResetScore"}, at = @At(value = "INVOKE", target = "Lorg/slf4j/Logger;warn(Ljava/lang/String;Ljava/lang/Object;)V"), require = 2)
/// target: org.slf4j.Logger#warn(Ljava/lang/String;Ljava/lang/Object;)V  [exists: org/slf4j/logger.rs]
/// within: net.minecraft.client.multiplayer.ClientPacketListener#handleResetScore
fn skyblocker_cancel_unknown_scoreboard_objective_warnings_1(ci: &CallbackInfo<'_>) -> bool {
    return !(crate::de::hysky::skyblocker::utils::utils::Utils::is_on_hypixel());
}

/// @WrapWithCondition skyblocker$cancelTeamWarning
///   java: @WrapWithCondition(method = "handleSetPlayerTeamPacket", at = @At(value = "INVOKE", target = "Lorg/slf4j/Logger;warn(Ljava/lang/String;[Ljava/lang/Object;)V"))
/// target: org.slf4j.Logger#warn(Ljava/lang/String;[Ljava/lang/Object;)V  [exists: org/slf4j/logger.rs]
/// within: net.minecraft.client.multiplayer.ClientPacketListener#handleSetPlayerTeamPacket
fn skyblocker_cancel_team_warning(ci: &CallbackInfo<'_>) -> bool {
    return !(crate::de::hysky::skyblocker::utils::utils::Utils::is_on_hypixel());
}

/// @Inject skyblocker$onParticle
///   java: @Inject(method = "handleParticleEvent", at = @At("RETURN"))
/// target: net.minecraft.client.multiplayer.ClientPacketListener#handleParticleEvent  [exists]
/// Rust: net/minecraft/client/multiplayer/client_packet_listener.rs:8662 fn handle_particle_event(&mut self, packet: &crate::net::minecraft::network::protocol::game::clientbound_level_particles_packet::ClientboundLevelParticlesPacket)
fn skyblocker_on_particle(ci: &mut CallbackInfo<'_>) {
    let Some(this) = compat::detach(ci.this_mut::<mcrust::net::minecraft::client::multiplayer::client_packet_listener::ClientPacketListener>()) else { return; };
    let Some(packet) = compat::detach_ref(ci.arg::<mcrust::net::minecraft::network::protocol::game::clientbound_level_particles_packet::ClientboundLevelParticlesPacket>(0)) else { return; };
    (crate::de::hysky::skyblocker::events::particle_events::from_server_invoker())(packet.clone());
}

/// @Inject skyblocker$onMapItemData
///   java: @Inject(method = "handleMapItemData", at = @At("RETURN"))
/// target: net.minecraft.client.multiplayer.ClientPacketListener#handleMapItemData  [exists]
/// Rust: net/minecraft/client/multiplayer/client_packet_listener.rs:8128 fn handle_map_item_data(&mut self, packet: &ClientboundMapItemDataPacket)
fn skyblocker_on_map_item_data(ci: &mut CallbackInfo<'_>) {
    let Some(this) = compat::detach(ci.this_mut::<mcrust::net::minecraft::client::multiplayer::client_packet_listener::ClientPacketListener>()) else { return; };
    let Some(packet) = compat::detach_ref(ci.arg::<mcrust::net::minecraft::network::protocol::game::clientbound_map_item_data_packet::ClientboundMapItemDataPacket>(0)) else { return; };
    crate::de::hysky::skyblocker::skyblock::dungeon::dungeon_map_texture::DungeonMapTexture::on_map_item_data_update(&packet.map_id(), packet.color_patch().is_some());
}

/// @Inject skyblocker$onSetTime
///   java: @Inject(method = "handleSetTime", at = @At("RETURN"))
/// target: net.minecraft.client.multiplayer.ClientPacketListener#handleSetTime  [exists]
/// Rust: net/minecraft/client/multiplayer/client_packet_listener.rs:8515 fn handle_set_time(&mut self, packet: &ClientboundSetTimePacket)
fn skyblocker_on_set_time(ci: &mut CallbackInfo<'_>) {
    let Some(this) = compat::detach(ci.this_mut::<mcrust::net::minecraft::client::multiplayer::client_packet_listener::ClientPacketListener>()) else { return; };
    crate::de::hysky::skyblocker::skyblock::dwarven::crystals_locations_manager::CrystalsLocationsManager::on_time_update();
}

/// @ModifyExpressionValue shouldShowPacketSizeAndPingCharts
///   java: @ModifyExpressionValue(method = "tick", at = @At(value = "INVOKE", target = "Lnet/minecraft/client/gui/components/DebugScreenOverlay;showNetworkCharts()Z"))
/// target: net.minecraft.client.gui.components.DebugScreenOverlay#showNetworkCharts  [exists]
/// within: net.minecraft.client.multiplayer.ClientPacketListener#tick
/// Rust: net/minecraft/client/gui/components/debug_screen_overlay.rs:624 fn show_network_charts(&self)
fn should_show_packet_size_and_ping_charts(v: bool) -> bool {
    let mut original = v;
    let options = &crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().ui_and_visuals.smooth_aote;
    if crate::de::hysky::skyblocker::utils::utils::Utils::is_on_skyblock() && options.predictive && !(compat::get_state(&crate::de::hysky::skyblocker::skyblock::teleport::predictive_smooth_aote::TELEPORT_DISABLED)) && (options.enable_weird_transmission || options.enable_ether_transmission || options.enable_instant_transmission || options.enable_sinrecall_transmission || options.enable_wither_impact) {
        return true;
    }
    return original;
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    report(hook::register_injector("net.minecraft.client.multiplayer.ClientPacketListener#handleSetEntityData", InjectOptions::at(At::Tail).kind(InjectorKind::Inject).priority(1000), Arc::new(skyblocker_on_entity_tracker_update)));
    report(hook::register_injector("net.minecraft.client.multiplayer.ClientPacketListener#handleEntityLinkPacket", InjectOptions::at(At::Tail).kind(InjectorKind::Inject).priority(1000), Arc::new(skyblocker_on_entity_attach)));
    // java: @At(INVOKE PacketUtils.ensureRunningOnSameThread, shift = AFTER) - see `after_ensure_running_on_same_thread`
    report(hook::register_injector("net.minecraft.client.multiplayer.ClientPacketListener#handleMovePlayer", InjectOptions::at(At::Head).kind(InjectorKind::Inject).priority(1000), Arc::new(skyblocker_before_teleport)));
    report(hook::register_injector("net.minecraft.client.multiplayer.ClientPacketListener#handleMovePlayer", InjectOptions::at(At::Return).kind(InjectorKind::Inject).priority(1000), Arc::new(skyblocker_on_teleport)));
    report(hook::call_site("net.minecraft.client.multiplayer.ClientPacketListener#handleTakeItemEntity", "net.minecraft.world.entity.item.ItemEntity#getItem").priority(1000).inject(Shift::Before, skyblocker_on_item_pickup));
    // `org.slf4j.Logger` is the game's `org::slf4j::logger::Logger` (its overloads are hooked by descriptor).
    report(hook::call_site("net.minecraft.client.multiplayer.ClientPacketListener#handleSetEntityPassengersPacket", "org.slf4j.Logger#warn(Ljava/lang/String;)V").priority(1000).wrap_with_condition(skyblocker_cancel_entity_passengers_warning));
    report(hook::call_site("net.minecraft.client.multiplayer.ClientPacketListener#handleEntityEvent", "net.minecraft.network.protocol.game.ClientboundEntityEventPacket#getEntity").priority(1000).inject(Shift::After, skyblocker_on_entity_death));
    report(hook::register_injector("net.minecraft.client.multiplayer.ClientPacketListener#handleSetEquipment", InjectOptions::at(At::Tail).kind(InjectorKind::Inject).priority(1000), Arc::new(skyblocker_on_entity_equip)));
    // java: @At(INVOKE PlayerTabOverlay.setFooter) - see `skyblocker_update_player_list_footer`
    report(hook::register_injector("net.minecraft.client.multiplayer.ClientPacketListener#handleTabListCustomisation", InjectOptions::at(At::Return).kind(InjectorKind::Inject).priority(1000), Arc::new(skyblocker_update_player_list_footer)));
    report(hook::register_injector("net.minecraft.client.multiplayer.ClientPacketListener#handlePlayerInfoUpdate", InjectOptions::at(At::Head).kind(InjectorKind::Inject).priority(1000), Arc::new(skyblocker_on_list_change)));
    // `org.slf4j.Logger` is the game's `org::slf4j::logger::Logger` (its overloads are hooked by descriptor).
    report(hook::call_site("net.minecraft.client.multiplayer.ClientPacketListener#handlePlayerInfoUpdate", "org.slf4j.Logger#warn(Ljava/lang/String;Ljava/lang/Object;Ljava/lang/Object;)V").priority(1000).wrap_with_condition(skyblocker_cancel_player_list_warning));
    // java: @At(INVOKE PacketUtils.ensureRunningOnSameThread, shift = AFTER) - see `after_ensure_running_on_same_thread`
    report(hook::register_injector("net.minecraft.client.multiplayer.ClientPacketListener#handleSoundEvent", InjectOptions::at(At::Head).kind(InjectorKind::Inject).priority(1000), Arc::new(skyblocker_on_play_sound)));
    // java: @WrapWithCondition on the `Logger.warn` that is handleUnknownCustomPayload's whole body - see the handler.
    report(hook::register_injector("net.minecraft.client.multiplayer.ClientPacketListener#handleUnknownCustomPayload", InjectOptions::at(At::Head).kind(InjectorKind::Inject).priority(1000), Arc::new(skyblocker_drop_badlion_packet_warnings_at_head)));
    // `org.slf4j.Logger` is the game's `org::slf4j::logger::Logger` (its overloads are hooked by descriptor).
    report(hook::call_site("net.minecraft.client.multiplayer.ClientPacketListener#handleSetScore", "org.slf4j.Logger#warn(Ljava/lang/String;Ljava/lang/Object;)V").priority(1000).wrap_with_condition(skyblocker_cancel_unknown_scoreboard_objective_warnings));
    // `org.slf4j.Logger` is the game's `org::slf4j::logger::Logger` (its overloads are hooked by descriptor).
    report(hook::call_site("net.minecraft.client.multiplayer.ClientPacketListener#handleResetScore", "org.slf4j.Logger#warn(Ljava/lang/String;Ljava/lang/Object;)V").priority(1000).wrap_with_condition(skyblocker_cancel_unknown_scoreboard_objective_warnings_1));
    // `org.slf4j.Logger` is the game's `org::slf4j::logger::Logger` (its overloads are hooked by descriptor).
    report(hook::call_site("net.minecraft.client.multiplayer.ClientPacketListener#handleSetPlayerTeamPacket", "org.slf4j.Logger#warn(Ljava/lang/String;[Ljava/lang/Object;)V").priority(1000).wrap_with_condition(skyblocker_cancel_team_warning));
    report(hook::register_injector("net.minecraft.client.multiplayer.ClientPacketListener#handleParticleEvent", InjectOptions::at(At::Return).kind(InjectorKind::Inject).priority(1000), Arc::new(skyblocker_on_particle)));
    report(hook::register_injector("net.minecraft.client.multiplayer.ClientPacketListener#handleMapItemData", InjectOptions::at(At::Return).kind(InjectorKind::Inject).priority(1000), Arc::new(skyblocker_on_map_item_data)));
    report(hook::register_injector("net.minecraft.client.multiplayer.ClientPacketListener#handleSetTime", InjectOptions::at(At::Return).kind(InjectorKind::Inject).priority(1000), Arc::new(skyblocker_on_set_time)));
    report(hook::call_site("net.minecraft.client.multiplayer.ClientPacketListener#tick", "net.minecraft.client.gui.components.DebugScreenOverlay#showNetworkCharts").priority(1000).modify_expression_value::<bool>(should_show_packet_size_and_ping_charts));
}
