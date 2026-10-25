//! Ported from de.hysky.skyblocker.skyblock.slayers.boss.voidgloom.BeaconHighlighter by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::events::world_events;
use crate::de::hysky::skyblocker::skyblock::slayers::slayer_manager::SlayerManager;
use crate::de::hysky::skyblocker::skyblock::slayers::slayer_type::SlayerType;
use crate::de::hysky::skyblocker::utils::render::level_render_extraction_callback;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_collector::PrimitiveCollector;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler::Scheduler;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::java::util::uuid::Uuid;
use mcrust::net::minecraft::core::block_pos::BlockPos;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::network::syncher::synced_data_holder::SyncedDataHolder;
use mcrust::net::minecraft::network::syncher::synched_entity_data::DataItem;
use mcrust::net::minecraft::network::syncher::synched_entity_data::DataValue;
use mcrust::net::minecraft::world::entity::decoration::armor_stand::ArmorStand;
use mcrust::net::minecraft::world::entity::entity::Entity;
use mcrust::net::minecraft::world::entity::equipment_slot::EquipmentSlot;
use mcrust::net::minecraft::world::item::items;
use mcrust::net::minecraft::world::level::block::blocks;
use mcrust::net::minecraft::world::level::block::state::block_state::BlockState;
use mcrust::net::minecraft::world::phys::vec3;
use mcrust::net::minecraft::world::phys::vec3::Vec3;
use mcrust::net::minecraft::network::chat::component::Component as _;
use mcrust::net::minecraft::world::entity::living_entity::LivingEntity as _;
use mcrust::net::minecraft::world::level::entity::level_entity_getter::LevelEntityGetter as _;

/// java: private static final float[] RED_COLOR_COMPONENTS = { 1.0f, 0.0f, 0.0f };
pub static RED_COLOR_COMPONENTS: LazyLock<Vec<f32>> = LazyLock::new(|| vec![1.0f32, 0.0f32, 0.0f32]);
/// java: private static final long BEACON_DURATION_MS = 5000L;
pub const BEACON_DURATION_MS: i64 = 5000;
/// java: private static boolean beaconThrown;
pub static BEACON_THROWN: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));
/// java: private static final Object2ObjectMap<UUID, List<Vec3>> BEACON_ENTITY_PATHS = new Object2ObjectOpenHashMap<>();
pub static BEACON_ENTITY_PATHS: compat::MainThread<std::collections::HashMap<Uuid, Vec<Vec3>>> = compat::MainThread::new(|| std::collections::HashMap::new());
/// java: private static final Object2LongOpenHashMap<BlockPos> BEACONS = new Object2LongOpenHashMap<>();
pub static BEACONS: compat::MainThread<std::collections::HashMap<BlockPos, i64>> = compat::MainThread::new(|| std::collections::HashMap::new());

/// java: public class BeaconHighlighter
#[derive(Clone)]
pub struct BeaconHighlighter {}

impl BeaconHighlighter {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        ({ let __r = scheduler::INSTANCE.get_mut(); let __f = compat::dispatch(__r, |__t| __t.schedule_cyclic()); __f(__r, Arc::new(move || { BeaconHighlighter::update_beacon_entities(); }), 1) });
        world_events::BLOCK_STATE_UPDATE.register(Arc::new(move |__a0, __a1, __a2| { BeaconHighlighter::on_block_state_update(&__a0, Some(__a1.clone()), &__a2); }));
        level_render_extraction_callback::EVENT.register(Arc::new(move |__a0| { BeaconHighlighter::extract_rendering(&__a0); }));
        client_play_connection_events::JOIN.register(Arc::new(|listener: &mut ClientPacketListener| {
            BeaconHighlighter::reset();
        }));
        // java: ClientReceiveMessageEvents.ALLOW_GAME.register(BeaconHighlighter::onMessage);
        compat::register_allow_game(|text, overlay| BeaconHighlighter::on_message(text, overlay));
    }

    /// java: private static void reset()
    pub fn reset() {
        compat::set_state(&BEACON_THROWN, false);
        BEACON_ENTITY_PATHS.get_mut().clear();
        BEACONS.get_mut().clear();
    }

    /// java: @SuppressWarnings("ConstantValue") public static <T> void onThrowBeacon(SyncedDataHolder dataHolder, SynchedEntityData.DataItem<T> dataItem, SynchedEntityData.DataValue<T> newValue)
    ///
    /// `dataHolder` is `SynchedEntityData.entity` as the game names it during `assignValues`
    /// (`synched_entity_data::assigning_holder()`); a client entity's data always belongs to an `Entity`, so
    /// `dataHolder instanceof Entity entity` holds and `entity.getUUID()` is `data_holder.uuid`.
    pub fn on_throw_beacon(data_holder: &mcrust::net::minecraft::network::syncher::synched_entity_data::AssigningHolder, data_item: &DataItem, new_value: &DataValue) {
        let config = SkyblockerConfigManager::get();
        if !(Utils::is_in_the_end() && (config.slayers.enderman_slayer.enable_yang_glyphs_notification || config.slayers.enderman_slayer.highlight_beacons)) {
            return;
        }
        if !SlayerManager::is_selected_boss(&compat::uuid_of(data_holder.uuid)) {
            return;
        }
        // dataItem.getAccessor() == EnderManAccessor.getDATA_CARRY_STATE()
        if data_item.get_accessor_id() != crate::mixins::ender_man_accessor::get_data_carry_state().id() {
            return;
        }
        type Carried = Option<mcrust::net::minecraft::world::level::block::block::ErasedBlockState>;
        // dataItem.getValue() instanceof Optional<?> value && value.isPresent() && value.get() instanceof BlockState state && state.is(Blocks.BEACON)
        let Some(Some(state)) = mcrust::com::mojang::serialization::cast::<Carried>(data_item.get_value()).cloned() else {
            return;
        };
        if !state.0.is_block(&blocks::blocks().get("BEACON")) {
            return;
        }
        // ((Optional<?>) newValue.value()).isEmpty()
        if !mcrust::com::mojang::serialization::cast::<Carried>(&new_value.value).is_some_and(|v| v.is_none()) {
            return;
        }
        if config.slayers.enderman_slayer.highlight_beacons {
            compat::set_state(&BEACON_THROWN, true);
        }
        if config.slayers.enderman_slayer.enable_yang_glyphs_notification {
            let client = compat::minecraft();
            // CLIENT.gui.hud.setTimes(5, 20, 10);
            let (times, title_time) = mcrust::net::minecraft::client::gui::hud::set_times(client.gui.hud.title_times, client.gui.hud.title_time, 5, 20, 10);
            client.gui.hud.title_times = times;
            client.gui.hud.title_time = title_time;
            // CLIENT.gui.hud.setTitle(Component.literal("Yang Glyph!").withStyle(ChatFormatting.RED));
            mcrust::net::minecraft::client::multiplayer::client_packet_listener::apply_set_title_text(
                &mut client.gui.hud,
                &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(
                    mcrust::net::minecraft::network::chat::component::literal("Yang Glyph!").with_style_format(mcrust::net::minecraft::chat_formatting::ChatFormatting::Red),
                ),
            );
            // CLIENT.player.playSound(SoundEvents.NOTE_BLOCK_PLING.value(), 100f, 0.1f);
            mcrust::mcrust::modding::game::with_player(|player| { mcrust::net::minecraft::world::entity::living_entity::LivingEntity::play_sound(player, mcrust::net::minecraft::sounds::sound_events::note_block_pling().clone(), 100.0f32, 0.1f32); });
        }
    }

    /// java: public static void onEntitySpawn(ArmorStand entity)
    pub fn on_entity_spawn(entity: &ArmorStand) {
        if Utils::is_in_the_end() && SlayerManager::is_fighting_slayer_type(SlayerType::Voidgloom) && compat::get_state(&BEACON_THROWN) && entity.has_item_in_slot(EquipmentSlot::Head) && entity.get_item_by_slot(EquipmentSlot::Head).is_item(&items::items().get("beacon")) {
            (BEACON_ENTITY_PATHS.get_mut().entry(compat::uuid_of(entity.base.base.get_uuid())).or_insert_with_key(|k| (|_| Vec::new())(k.clone()))).push(entity.base.base.get_eye_position());
        }
    }

    /// java: private static void updateBeaconEntities()
    pub fn update_beacon_entities() {
        if mcrust::mcrust::modding::game::level().is_none() || !(Utils::is_in_the_end()) || !(SlayerManager::is_fighting_slayer_type(SlayerType::Voidgloom)) || !(SkyblockerConfigManager::get().slayers.enderman_slayer.highlight_beacons) {
            return;
        }
        let level = mcrust::mcrust::modding::game::level().unwrap();
        let keys: Vec<Uuid> = BEACON_ENTITY_PATHS.get().keys().cloned().collect();
        for key in keys {
            // `CLIENT.level.getEntity(uuid)` = `getEntities().get(uuid)`
            let entity = level.with_entities(|getter| getter.get_by_uuid(compat::uuid_bits(&key)));
            let Some(entity) = entity else {
                continue;
            };
            if mcrust::mcrust::modding::game::is_held_here(&*entity) {
                continue;
            }
            let (position, eye_position) = {
                let guard = mcrust::mcrust::modding::game::lock_entity(&*entity, "beacon_highlighter.rs:updateBeaconEntities");
                (guard.base().position(), guard.base().get_eye_position())
            };
            let Some(path) = BEACON_ENTITY_PATHS.get_mut().get_mut(&key) else {
                continue;
            };
            if !(path[path.len() - 1] == position) {
                path.push(eye_position);
            }
        }
    }

    /// java: private static void onBlockStateUpdate(BlockPos pos, @Nullable BlockState oldState, BlockState newState)
    pub fn on_block_state_update(pos: &BlockPos, mut old_state: Option<Arc<BlockState>>, new_state: &Arc<BlockState>) {
        if Utils::is_in_the_end() && SlayerManager::is_fighting_slayer_type(SlayerType::Voidgloom) && SkyblockerConfigManager::get().slayers.enderman_slayer.highlight_beacons {
            BEACONS.get_mut().remove(pos);
            if new_state.is_block(&blocks::blocks().get("BEACON")) {
                BEACONS.get_mut().insert(pos.immutable(), compat::millis());
                compat::set_state(&BEACON_THROWN, false);
            } else if old_state.is_none() || old_state.as_ref().unwrap().is_block(&blocks::blocks().get("BEACON")) {
                let key = |path: &Vec<Vec3>| (path[path.len() - 1]).distance_to_sqr_vec(&vec3::at_center_of(pos));
                // the closest path, by its key (Java removes that list object: `path == closestPath`)
                let mut closest_path: Option<Uuid> = BEACON_ENTITY_PATHS.get().iter().min_by(|a, b| key(a.1).total_cmp(&key(b.1))).filter(|(_, path)| key(path) < (4 as f64)).map(|(uuid, _)| uuid.clone());
                if let Some(closest_path) = &closest_path {
                    BEACON_ENTITY_PATHS.get_mut().remove(closest_path);
                }
            }
        }
    }

    /// java: private static boolean onMessage(Component text, boolean overlay)
    pub fn on_message(text: &MutableComponent, mut overlay: bool) -> bool {
        if Utils::is_in_the_end() && !overlay {
            let mut message: String = text.get_string();
            if message.contains("SLAYER QUEST COMPLETE!") || message.contains("NICE! SLAYER BOSS SLAIN!") {
                BeaconHighlighter::reset();
            }
        }
        return true;
    }

    /// java: private static void extractRendering(PrimitiveCollector collector)
    pub fn extract_rendering(collector: &compat::Shared<dyn PrimitiveCollector>) {
        if Utils::is_in_the_end() && SkyblockerConfigManager::get().slayers.enderman_slayer.highlight_beacons && SlayerManager::is_fighting_slayer_type(SlayerType::Voidgloom) {
            for beacon_entity_path in BEACON_ENTITY_PATHS.get_mut().values().cloned() {
                if (beacon_entity_path.len() as i32) > 1 {
                    ({ let __f = compat::dispatch(collector, |__t| __t.submit_lines_from_points()); __f(collector, &beacon_entity_path.clone(), &RED_COLOR_COMPONENTS, 1.0f32, 10.0f32, true) });
                }
            }
            // java: for (Object2LongMap.Entry<BlockPos> beacon : BEACONS.object2LongEntrySet()) { .. }
            for (beacon_pos, placed_at) in BEACONS.get().iter().map(|(k, v)| (k.clone(), *v)) {
                ({ let __f = compat::dispatch(collector, |__t| __t.submit_filled_box()); __f(collector, &beacon_pos, &RED_COLOR_COMPONENTS, 0.6f32, true) });
                let elapsed: i64 = compat::millis().wrapping_sub(placed_at);
                let remaining_sec: f32 = (BEACON_DURATION_MS.wrapping_sub(elapsed) as f32) / 1000.0f32;
                if remaining_sec >= 0.0 {
                    let text: MutableComponent = mcrust::net::minecraft::network::chat::component::literal(&compat::string_format("%.1fs", &[remaining_sec.into()])).with_style_format(mcrust::net::minecraft::chat_formatting::ChatFormatting::Aqua);
                    ({ let __f = compat::dispatch(collector, |__t| __t.submit_text_4()); __f(collector, &text, &mcrust::net::minecraft::world::phys::vec3::at_center_of(&beacon_pos.above()), 3.0f32, true) });
                }
            }
        }
    }

}
