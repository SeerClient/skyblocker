//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.ClientPacketListenerMixin by fabric2seer.
//! @Mixin(net.minecraft.client.multiplayer.ClientPacketListener)

#[allow(unused_imports)]
use crate::prelude::*;

/// @Inject skyblocker$onEntityTrackerUpdate
///   java: @Inject(method = "handleSetEntityData", at = @At("TAIL"))
/// target: net.minecraft.client.multiplayer.ClientPacketListener#handleSetEntityData  [exists]
/// Rust: net/minecraft/client/multiplayer/client_packet_listener.rs:7021 fn handle_set_entity_data(&mut self, packet: &ClientboundSetEntityDataPacket)
fn skyblocker_on_entity_tracker_update(ci: &mut CallbackInfo<'_>) {
    let Some(this) = compat::detach(ci.this_mut::<mcrust::net::minecraft::client::multiplayer::client_packet_listener::ClientPacketListener>()) else { return; };
    let Some(packet) = compat::detach_ref(ci.arg::<mcrust::net::minecraft::network::protocol::game::clientbound_set_entity_data_packet::ClientboundSetEntityDataPacket>(0)) else { return; };
    let Some(armor_stand_entity) = nil.as_any().downcast_ref::<mcrust::net::minecraft::world::entity::decoration::armor_stand::ArmorStand>() else {
        return;
    };
    crate::de::hysky::skyblocker::skyblock::slayers::slayer_manager::SlayerManager::check_slayer_boss(armor_stand_entity);
    crate::de::hysky::skyblocker::skyblock::slayers::boss::voidgloom::beacon_highlighter::BeaconHighlighter::on_entity_spawn(armor_stand_entity);
    crate::de::hysky::skyblocker::skyblock::slayers::boss::demonlord::fire_pillar_announcer::FirePillarAnnouncer::check_fire_pillar(&nil);
    crate::de::hysky::skyblocker::skyblock::dwarven::corpse_finder::CorpseFinder::check_if_corpse_armor_stand(armor_stand_entity);
    // TODO(fabric2seer): the mod keeps the game's `ArmorStand` (Java holds a reference to the live object); the port's `ArmorStand` is not a value to copy and the port lends the mod no handle to it
    // rust: crate::de::hysky::skyblocker::skyblock::health_bars::HealthBars::health_bar(armor_stand_entity.clone());
    // TODO(fabric2seer): the mod keeps the game's `ArmorStand` (Java holds a reference to the live object); the port's `ArmorStand` is not a value to copy and the port lends the mod no handle to it
    // rust: crate::de::hysky::skyblocker::skyblock::fishing::sea_creature_tracker::SeaCreatureTracker::on_entity_spawn(armor_stand_entity.clone());
    crate::de::hysky::skyblocker::skyblock::fishing::fishing_helper::FishingHelper::check_if_fish_was_caught(armor_stand_entity);
    // TODO(fabric2seer): the mod keeps the game's `ArmorStand` (Java holds a reference to the live object); the port's `ArmorStand` is not a value to copy and the port lends the mod no handle to it
    // rust: crate::de::hysky::skyblocker::skyblock::foraging::galatea::tree_break_progress_hud::TreeBreakProgressHud::on_entity_update(armor_stand_entity.clone());
    crate::de::hysky::skyblocker::skyblock::hunting::lasso_hud::LassoHud::on_entity_update(armor_stand_entity);
    // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
    let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        crate::de::hysky::skyblocker::skyblock::compact_damage::CompactDamage::compact_damage(armor_stand_entity);
    }));
    if let Err(e) = __result {
        // TODO(fabric2seer): `.error(..)` on a Logger
        //   java: LOGGER.error("[Skyblocker Compact Damage] Failed to compact damage number", e);
    }
    // TODO(fabric2seer): the mod keeps the game's `ArmorStand` (Java holds a reference to the live object); the port's `ArmorStand` is not a value to copy and the port lends the mod no handle to it
    // rust: crate::de::hysky::skyblocker::skyblock::fishing::fishing_hook_display_helper::FishingHookDisplayHelper::on_armor_stand_spawn(armor_stand_entity.clone());
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
fn skyblocker_before_teleport(ci: &mut CallbackInfo<'_>) {
    // TODO(fabric2seer): `beforeTeleport`: this argument is not visible to the injector (see the notes above the handler)
    //   java: beforeTeleport.set(minecraft.player.blockPosition().immutable());
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
    // TODO(fabric2seer): unknown name `minecraft`
    //   java: TeleportMaze.INSTANCE.onTeleport(minecraft, beforeTeleport.get(), minecraft.player.blockPosition().immutable());
}

/// @Inject skyblocker$onItemPickup
///   java: @Inject(method = "handleTakeItemEntity", at = @At(value = "INVOKE", target = "Lnet/minecraft/world/entity/item/ItemEntity;getItem()Lnet/minecraft/world/item/ItemStack;"))
/// target: net.minecraft.world.entity.item.ItemEntity#getItem  [exists]
/// within: net.minecraft.client.multiplayer.ClientPacketListener#handleTakeItemEntity
/// Rust: net/minecraft/world/entity/item/item_entity.rs:228 fn get_item(&self)
/// note: @Inject at INVOKE: a call-site injector (`hook::call_site`) - `ci` is the CALL's (its arguments; cancel() skips that call, not the rest of the method)
fn skyblocker_on_item_pickup(ci: &mut CallbackInfo<'_>) {
    // TODO(fabric2seer): `itemEntity`: this argument is not visible to the injector (see the notes above the handler)
    //   java: DungeonManager.onItemPickup(itemEntity);
}

/// @WrapWithCondition skyblocker$cancelEntityPassengersWarning
///   java: @WrapWithCondition(method = "handleSetEntityPassengersPacket", at = @At(value = "INVOKE", target = "Lorg/slf4j/Logger;warn(Ljava/lang/String;)V"))
/// target: org.slf4j.Logger#warn(Ljava/lang/String;)V  [class not in JavaSrc]
/// within: net.minecraft.client.multiplayer.ClientPacketListener#handleSetEntityPassengersPacket
/// note: class not in JavaSrc: org.slf4j.Logger has no file in JavaSrc/src - a remapping problem, or a class this Minecraft version does not have
fn skyblocker_cancel_entity_passengers_warning(ci: &CallbackInfo<'_>) -> bool {
    let Some(this) = compat::detach(ci.this_mut::<Logger>()) else { return true; };
    return !(crate::de::hysky::skyblocker::utils::utils::Utils::is_on_hypixel());
}

/// @ModifyExpressionValue skyblocker$onEntityDeath
///   java: @ModifyExpressionValue(method = "handleEntityEvent", at = @At(value = "INVOKE", target = "Lnet/minecraft/network/protocol/game/ClientboundEntityEventPacket;getEntity(Lnet/minecraft/world/level/Level;)Lnet/minecraft/world/entity/Entity;"))
/// target: net.minecraft.network.protocol.game.ClientboundEntityEventPacket#getEntity  [exists]
/// within: net.minecraft.client.multiplayer.ClientPacketListener#handleEntityEvent
/// Rust: net/minecraft/network/protocol/game/clientbound_entity_event_packet.rs:92 fn get_entity_id(&self)
fn skyblocker_on_entity_death(v: Box<dyn mcrust::net::minecraft::world::entity::entity::Entity>) -> Box<dyn mcrust::net::minecraft::world::entity::entity::Entity> {
    let mut entity = v;
    // TODO(fabric2seer): the condition: `packet`: this argument is not visible to the injector (see the notes above the handler)
    //   java: packet.getEventId() == EntityEvent.DEATH
    if todo!("fabric2seer: the condition (TODO above)") {
        crate::de::hysky::skyblocker::skyblock::dungeon::dungeon_score::DungeonScore::handle_entity_death(&entity);
        crate::de::hysky::skyblocker::skyblock::end::the_end::TheEnd::on_entity_death(&entity);
    }
    return entity;
}

/// @Inject skyblocker$onEntityEquip
///   java: @Inject(method = "handleSetEquipment", at = @At(value = "TAIL"))
/// target: net.minecraft.client.multiplayer.ClientPacketListener#handleSetEquipment  [exists]
/// Rust: net/minecraft/client/multiplayer/client_packet_listener.rs:5062 fn handle_set_equipment(&mut self, packet: &ClientboundSetEquipmentPacket)
fn skyblocker_on_entity_equip(ci: &mut CallbackInfo<'_>) {
    let Some(this) = compat::detach(ci.this_mut::<mcrust::net::minecraft::client::multiplayer::client_packet_listener::ClientPacketListener>()) else { return; };
    let Some(packet) = compat::detach_ref(ci.arg::<mcrust::net::minecraft::network::protocol::game::clientbound_set_equipment_packet::ClientboundSetEquipmentPacket>(0)) else { return; };
    crate::de::hysky::skyblocker::skyblock::dwarven::corpse_finder::CorpseFinder::check_if_corpse(&nil);
}

/// @Inject skyblocker$updatePlayerListFooter
///   java: @Inject(method = "handleTabListCustomisation", at = @At(value = "INVOKE", target = "Lnet/minecraft/client/gui/components/PlayerTabOverlay;setFooter(Lnet/minecraft/network/chat/Component;)V"))
/// target: net.minecraft.client.gui.components.PlayerTabOverlay#setFooter  [exists]
/// within: net.minecraft.client.multiplayer.ClientPacketListener#handleTabListCustomisation
/// Rust: net/minecraft/client/gui/components/player_tab_overlay.rs:1726 fn set_footer(&mut self, footer: Option<crate::net::minecraft::network::chat::formatted_text::FormattedText>)
/// note: @Inject at INVOKE: a call-site injector (`hook::call_site`) - `ci` is the CALL's (its arguments; cancel() skips that call, not the rest of the method)
fn skyblocker_update_player_list_footer(ci: &mut CallbackInfo<'_>) {
    // TODO(fabric2seer): `packet`: this argument is not visible to the injector (see the notes above the handler)
    //   java: PlayerListManager.updateFooter(packet.footer());
}

/// @Inject skyblocker$onListChange
///   java: @Inject(method = "handlePlayerInfoUpdate", at = @At("HEAD"))
/// target: net.minecraft.client.multiplayer.ClientPacketListener#handlePlayerInfoUpdate  [exists]
/// Rust: net/minecraft/client/multiplayer/client_packet_listener.rs:9334 fn handle_player_info_update(&mut self, packet: &super::super::super::network::protocol::game::clientbound_player_info_update_packet::ClientboundPlayerInfoUpdatePacket)
fn skyblocker_on_list_change(ci: &mut CallbackInfo<'_>) {
    let Some(this) = compat::detach(ci.this_mut::<mcrust::net::minecraft::client::multiplayer::client_packet_listener::ClientPacketListener>()) else { return; };
    compat::set_state(&crate::classes::player_list_manager::SHOULD_UPDATE_NEXT_TICK, true);
}

/// @WrapWithCondition skyblocker$cancelPlayerListWarning
///   java: @WrapWithCondition(method = "handlePlayerInfoUpdate", at = @At(value = "INVOKE", target = "Lorg/slf4j/Logger;warn(Ljava/lang/String;Ljava/lang/Object;Ljava/lang/Object;)V"))
/// target: org.slf4j.Logger#warn(Ljava/lang/String;Ljava/lang/Object;Ljava/lang/Object;)V  [class not in JavaSrc]
/// within: net.minecraft.client.multiplayer.ClientPacketListener#handlePlayerInfoUpdate
/// note: class not in JavaSrc: org.slf4j.Logger has no file in JavaSrc/src - a remapping problem, or a class this Minecraft version does not have
fn skyblocker_cancel_player_list_warning(ci: &CallbackInfo<'_>) -> bool {
    let Some(this) = compat::detach(ci.this_mut::<Logger>()) else { return true; };
    return !(crate::de::hysky::skyblocker::utils::utils::Utils::is_on_hypixel());
}

/// @Inject skyblocker$onPlaySound
///   java: @Inject(method = "handleSoundEvent", at = @At(value = "INVOKE", target = "Lnet/minecraft/network/protocol/PacketUtils;ensureRunningOnSameThread(Lnet/minecraft/network/protocol/Packet;Lnet/minecraft/network/PacketListener;Lnet/minecraft/network/PacketProcessor;)V", shift = At.Shift.AFTER))
/// target: net.minecraft.network.protocol.PacketUtils#ensureRunningOnSameThread(Lnet/minecraft/network/protocol/Packet;Lnet/minecraft/network/PacketListener;Lnet/minecraft/network/PacketProcessor;)V  [exists]
/// within: net.minecraft.client.multiplayer.ClientPacketListener#handleSoundEvent
/// Rust: net/minecraft/network/protocol/packet_utils.rs:53 fn ensure_running_on_same_thread(packet: Box<dyn Packet>, packet_processor: &mut PacketProcessor)
/// note: @Inject at INVOKE: a call-site injector (`hook::call_site`) - `ci` is the CALL's (its arguments; cancel() skips that call, not the rest of the method)
fn skyblocker_on_play_sound(ci: &mut CallbackInfo<'_>) {
    // TODO(fabric2seer): `packet`: this argument is not visible to the injector (see the notes above the handler)
    //   java: PlaySoundEvents.FROM_SERVER.invoker().onPlaySoundFromServer(packet);
}

/// @WrapWithCondition skyblocker$dropBadlionPacketWarnings
///   java: @WrapWithCondition(method = "handleUnknownCustomPayload", at = @At(value = "INVOKE", target = "Lorg/slf4j/Logger;warn(Ljava/lang/String;Ljava/lang/Object;)V"))
/// target: org.slf4j.Logger#warn(Ljava/lang/String;Ljava/lang/Object;)V  [class not in JavaSrc]
/// within: net.minecraft.client.multiplayer.ClientPacketListener#handleUnknownCustomPayload
/// note: class not in JavaSrc: org.slf4j.Logger has no file in JavaSrc/src - a remapping problem, or a class this Minecraft version does not have
fn skyblocker_drop_badlion_packet_warnings(ci: &CallbackInfo<'_>) -> bool {
    let Some(this) = compat::detach(ci.this_mut::<Logger>()) else { return true; };
    // TODO(fabric2seer): `identifier`: this argument is not visible to the injector (see the notes above the handler)
    //   java: return !(Utils.isOnHypixel() && ((Identifier) identifier).getNamespace().equals("badlion"));
    todo!("fabric2seer: the value of this function is in a TODO above")
}

/// @WrapWithCondition skyblocker$cancelUnknownScoreboardObjectiveWarnings
///   java: @WrapWithCondition(method = {"handleSetScore", "handleResetScore"}, at = @At(value = "INVOKE", target = "Lorg/slf4j/Logger;warn(Ljava/lang/String;Ljava/lang/Object;)V"), require = 2)
/// target: org.slf4j.Logger#warn(Ljava/lang/String;Ljava/lang/Object;)V  [class not in JavaSrc]
/// within: net.minecraft.client.multiplayer.ClientPacketListener#handleSetScore
/// note: class not in JavaSrc: org.slf4j.Logger has no file in JavaSrc/src - a remapping problem, or a class this Minecraft version does not have
fn skyblocker_cancel_unknown_scoreboard_objective_warnings(ci: &CallbackInfo<'_>) -> bool {
    let Some(this) = compat::detach(ci.this_mut::<Logger>()) else { return true; };
    return !(crate::de::hysky::skyblocker::utils::utils::Utils::is_on_hypixel());
}

/// @WrapWithCondition skyblocker$cancelUnknownScoreboardObjectiveWarnings
///   java: @WrapWithCondition(method = {"handleSetScore", "handleResetScore"}, at = @At(value = "INVOKE", target = "Lorg/slf4j/Logger;warn(Ljava/lang/String;Ljava/lang/Object;)V"), require = 2)
/// target: org.slf4j.Logger#warn(Ljava/lang/String;Ljava/lang/Object;)V  [class not in JavaSrc]
/// within: net.minecraft.client.multiplayer.ClientPacketListener#handleResetScore
/// note: class not in JavaSrc: org.slf4j.Logger has no file in JavaSrc/src - a remapping problem, or a class this Minecraft version does not have
fn skyblocker_cancel_unknown_scoreboard_objective_warnings_1(ci: &CallbackInfo<'_>) -> bool {
    let Some(this) = compat::detach(ci.this_mut::<Logger>()) else { return true; };
    return !(crate::de::hysky::skyblocker::utils::utils::Utils::is_on_hypixel());
}

/// @WrapWithCondition skyblocker$cancelTeamWarning
///   java: @WrapWithCondition(method = "handleSetPlayerTeamPacket", at = @At(value = "INVOKE", target = "Lorg/slf4j/Logger;warn(Ljava/lang/String;[Ljava/lang/Object;)V"))
/// target: org.slf4j.Logger#warn(Ljava/lang/String;[Ljava/lang/Object;)V  [class not in JavaSrc]
/// within: net.minecraft.client.multiplayer.ClientPacketListener#handleSetPlayerTeamPacket
/// note: class not in JavaSrc: org.slf4j.Logger has no file in JavaSrc/src - a remapping problem, or a class this Minecraft version does not have
fn skyblocker_cancel_team_warning(ci: &CallbackInfo<'_>) -> bool {
    let Some(this) = compat::detach(ci.this_mut::<Logger>()) else { return true; };
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
    if crate::de::hysky::skyblocker::utils::utils::Utils::is_on_skyblock() && options.predictive && !(compat::get_state(&crate::de::hysky::skyblocker::skyblock::teleport::predictive_smooth_aote::TELEPORT_DISABLED)) && options.enable_weird_transmission || options.enable_ether_transmission || options.enable_instant_transmission || options.enable_sinrecall_transmission || options.enable_wither_impact {
        return true;
    }
    return original;
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    report(hook::register_injector("net.minecraft.client.multiplayer.ClientPacketListener#handleSetEntityData", InjectOptions::at(At::Tail).kind(InjectorKind::Inject).priority(1000), Arc::new(skyblocker_on_entity_tracker_update)));
    report(hook::register_injector("net.minecraft.client.multiplayer.ClientPacketListener#handleEntityLinkPacket", InjectOptions::at(At::Tail).kind(InjectorKind::Inject).priority(1000), Arc::new(skyblocker_on_entity_attach)));
    report(hook::call_site("net.minecraft.client.multiplayer.ClientPacketListener#handleMovePlayer", "net.minecraft.network.protocol.PacketUtils#ensureRunningOnSameThread(Lnet/minecraft/network/protocol/Packet;Lnet/minecraft/network/PacketListener;Lnet/minecraft/network/PacketProcessor;)V").priority(1000).inject(Shift::After, skyblocker_before_teleport));
    report(hook::register_injector("net.minecraft.client.multiplayer.ClientPacketListener#handleMovePlayer", InjectOptions::at(At::Return).kind(InjectorKind::Inject).priority(1000), Arc::new(skyblocker_on_teleport)));
    report(hook::call_site("net.minecraft.client.multiplayer.ClientPacketListener#handleTakeItemEntity", "net.minecraft.world.entity.item.ItemEntity#getItem").priority(1000).inject(Shift::Before, skyblocker_on_item_pickup));
    // TODO(fabric2seer): not registered - class not in JavaSrc: org.slf4j.Logger has no file in JavaSrc/src - a remapping problem, or a class this Minecraft version does not have
    // report(hook::call_site(`net.minecraft.client.multiplayer.ClientPacketListener#handleSetEntityPassengersPacket`, `org.slf4j.Logger#warn(Ljava/lang/String;)V`).priority(1000).wrap_with_condition(skyblocker_cancel_entity_passengers_warning));
    report(hook::call_site("net.minecraft.client.multiplayer.ClientPacketListener#handleEntityEvent", "net.minecraft.network.protocol.game.ClientboundEntityEventPacket#getEntity").priority(1000).modify_expression_value::<Box<dyn mcrust::net::minecraft::world::entity::entity::Entity>>(skyblocker_on_entity_death));
    report(hook::register_injector("net.minecraft.client.multiplayer.ClientPacketListener#handleSetEquipment", InjectOptions::at(At::Tail).kind(InjectorKind::Inject).priority(1000), Arc::new(skyblocker_on_entity_equip)));
    report(hook::call_site("net.minecraft.client.multiplayer.ClientPacketListener#handleTabListCustomisation", "net.minecraft.client.gui.components.PlayerTabOverlay#setFooter").priority(1000).inject(Shift::Before, skyblocker_update_player_list_footer));
    report(hook::register_injector("net.minecraft.client.multiplayer.ClientPacketListener#handlePlayerInfoUpdate", InjectOptions::at(At::Head).kind(InjectorKind::Inject).priority(1000), Arc::new(skyblocker_on_list_change)));
    // TODO(fabric2seer): not registered - class not in JavaSrc: org.slf4j.Logger has no file in JavaSrc/src - a remapping problem, or a class this Minecraft version does not have
    // report(hook::call_site(`net.minecraft.client.multiplayer.ClientPacketListener#handlePlayerInfoUpdate`, `org.slf4j.Logger#warn(Ljava/lang/String;Ljava/lang/Object;Ljava/lang/Object;)V`).priority(1000).wrap_with_condition(skyblocker_cancel_player_list_warning));
    report(hook::call_site("net.minecraft.client.multiplayer.ClientPacketListener#handleSoundEvent", "net.minecraft.network.protocol.PacketUtils#ensureRunningOnSameThread(Lnet/minecraft/network/protocol/Packet;Lnet/minecraft/network/PacketListener;Lnet/minecraft/network/PacketProcessor;)V").priority(1000).inject(Shift::After, skyblocker_on_play_sound));
    // TODO(fabric2seer): not registered - class not in JavaSrc: org.slf4j.Logger has no file in JavaSrc/src - a remapping problem, or a class this Minecraft version does not have
    // report(hook::call_site(`net.minecraft.client.multiplayer.ClientPacketListener#handleUnknownCustomPayload`, `org.slf4j.Logger#warn(Ljava/lang/String;Ljava/lang/Object;)V`).priority(1000).wrap_with_condition(skyblocker_drop_badlion_packet_warnings));
    // TODO(fabric2seer): not registered - class not in JavaSrc: org.slf4j.Logger has no file in JavaSrc/src - a remapping problem, or a class this Minecraft version does not have
    // report(hook::call_site(`net.minecraft.client.multiplayer.ClientPacketListener#handleSetScore`, `org.slf4j.Logger#warn(Ljava/lang/String;Ljava/lang/Object;)V`).priority(1000).wrap_with_condition(skyblocker_cancel_unknown_scoreboard_objective_warnings));
    // TODO(fabric2seer): not registered - class not in JavaSrc: org.slf4j.Logger has no file in JavaSrc/src - a remapping problem, or a class this Minecraft version does not have
    // report(hook::call_site(`net.minecraft.client.multiplayer.ClientPacketListener#handleResetScore`, `org.slf4j.Logger#warn(Ljava/lang/String;Ljava/lang/Object;)V`).priority(1000).wrap_with_condition(skyblocker_cancel_unknown_scoreboard_objective_warnings_1));
    // TODO(fabric2seer): not registered - class not in JavaSrc: org.slf4j.Logger has no file in JavaSrc/src - a remapping problem, or a class this Minecraft version does not have
    // report(hook::call_site(`net.minecraft.client.multiplayer.ClientPacketListener#handleSetPlayerTeamPacket`, `org.slf4j.Logger#warn(Ljava/lang/String;[Ljava/lang/Object;)V`).priority(1000).wrap_with_condition(skyblocker_cancel_team_warning));
    report(hook::register_injector("net.minecraft.client.multiplayer.ClientPacketListener#handleParticleEvent", InjectOptions::at(At::Return).kind(InjectorKind::Inject).priority(1000), Arc::new(skyblocker_on_particle)));
    report(hook::register_injector("net.minecraft.client.multiplayer.ClientPacketListener#handleMapItemData", InjectOptions::at(At::Return).kind(InjectorKind::Inject).priority(1000), Arc::new(skyblocker_on_map_item_data)));
    report(hook::register_injector("net.minecraft.client.multiplayer.ClientPacketListener#handleSetTime", InjectOptions::at(At::Return).kind(InjectorKind::Inject).priority(1000), Arc::new(skyblocker_on_set_time)));
    report(hook::call_site("net.minecraft.client.multiplayer.ClientPacketListener#tick", "net.minecraft.client.gui.components.DebugScreenOverlay#showNetworkCharts").priority(1000).modify_expression_value::<bool>(should_show_packet_size_and_ping_charts));
}
