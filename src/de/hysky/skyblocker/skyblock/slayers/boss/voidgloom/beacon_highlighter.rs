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
        // TODO(fabric2seer): ClientReceiveMessageEvents.ALLOW_GAME.register: no SeerClient equivalent yet
        //   java: ClientReceiveMessageEvents.ALLOW_GAME.register(BeaconHighlighter::onMessage);
    }

    /// java: private static void reset()
    pub fn reset() {
        compat::set_state(&BEACON_THROWN, false);
        BEACON_ENTITY_PATHS.get_mut().clear();
        BEACONS.get_mut().clear();
    }

    /// java: @SuppressWarnings("ConstantValue") public static <T> void onThrowBeacon(SyncedDataHolder dataHolder, SynchedEntityData.DataItem<T> dataItem, SynchedEntityData.DataValue<T> newValue)
    pub fn on_throw_beacon<T: 'static>(data_holder: &dyn SyncedDataHolder, data_item: &DataItem, new_value: &DataValue) {
        // TODO(fabric2seer): instanceof an interface (`Entity`): Rust cannot test for a trait at run time
        //   java: if (Utils.isInTheEnd() && (SkyblockerConfigManager.get().slayers.endermanSlayer.enableYangGlyphsNotification || SkyblockerConfigManager.get().slayers.endermanSlayer.highlightBeacons)
        //   java: 				&& dataHolder instanceof Entity entity && SlayerManager.isSelectedBoss(entity.getUUID())
        //   java: 				&& dataItem.getAccessor() == EnderManAccessor.getDATA_CARRY_STATE()
        //   java: 				&& dataItem.getValue() instanceof Optional<?> value && value.isPresent() && value.get() instanceof BlockState state && state.is(Blocks.BEACON)
        //   java: 				&& ((Optional<?>) newValue.value()).isEmpty()) {
        //   java: 			if (SkyblockerConfigManager.get().slayers.endermanSlayer.highlightBeacons) {
        //   java: 				beaconThrown = true;
        //   java: 			}
        //   java:
        //   java: 			if (SkyblockerConfigManager.get().slayers.endermanSlayer.enableYangGlyphsNotification) {
        //   java: 				CLIENT.gui.hud.setTimes(5, 20, 10);
        //   java: 				CLIENT.gui.hud.setTitle(Component.literal("Yang Glyph!").withStyle(ChatFormatting.RED));
        //   java: 				CLIENT.player.playSound(SoundEvents.NOTE_BLOCK_PLING.value(), 100f, 0.1f);
        //   java: 			}
        //   java: 		}
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
        for beacon_entity_path in BEACON_ENTITY_PATHS.get_mut().iter().map(|(k, v)| (k.clone(), v.clone())) {
            // TODO(fabric2seer): `ClientLevel.getEntity` has no Rust method in the port (looked for `get_entity` in ClientLevel and its supertypes)
            // rust: let mut entity: Option<Box<dyn Entity>> = Some(mcrust::mcrust::modding::game::level().unwrap().get_entity(&beacon_entity_path.0));
            let mut entity: Box<dyn Entity> = todo!("fabric2seer: the initialiser of `entity` (TODO above)");
            if true /* java: entity != null: never null here */ && !(beacon_entity_path.1[beacon_entity_path.1.len() - 1] == entity.position()) {
                beacon_entity_path.1.push(entity.get_eye_position());
            }
        }
    }

    /// java: private static void onBlockStateUpdate(BlockPos pos, @Nullable BlockState oldState, BlockState newState)
    pub fn on_block_state_update(pos: &BlockPos, mut old_state: Option<Arc<BlockState>>, new_state: &Arc<BlockState>) {
        if Utils::is_in_the_end() && SlayerManager::is_fighting_slayer_type(SlayerType::Voidgloom) && SkyblockerConfigManager::get().slayers.enderman_slayer.highlight_beacons {
            // TODO(fabric2seer): Map.removeLong(..)
            //   java: BEACONS.removeLong(pos);
            if new_state.is_block(&blocks::blocks().get("beacon")) {
                BEACONS.get_mut().insert(pos.immutable(), compat::millis());
                compat::set_state(&BEACON_THROWN, false);
            } else if old_state.is_none() || old_state.as_ref().unwrap().is_block(&blocks::blocks().get("beacon")) {
                let mut closest_path: Vec<Vec3> = BEACON_ENTITY_PATHS.get_mut().values().cloned().min_by(|a, b| { let key = |path| (path[path.len() - 1]).distance_to_sqr_vec(&vec3::at_center_of(pos)); key(a).total_cmp(&key(b)) }).filter(|path| ((path[path.len() - 1]).distance_to_sqr_vec(&vec3::at_center_of(pos)) < (4 as f64))).unwrap();
                BEACON_ENTITY_PATHS.get_mut().retain(|_, __v| !(|path| (path == closest_path))(__v));
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
            // TODO(fabric2seer): Map.object2LongEntrySet(..)
            //   java: for (Object2LongMap.Entry<BlockPos> beacon : BEACONS.object2LongEntrySet()) {
            //   java: 				collector.submitFilledBox(beacon.getKey(), RED_COLOR_COMPONENTS, 0.6f, true);
            //   java:
            //   java: 				long elapsed = System.currentTimeMillis() - beacon.getLongValue();
            //   java: 				float remainingSec = (BEACON_DURATION_MS - elapsed) / 1000f;
            //   java: 				if (remainingSec >= 0) {
            //   java: 					Component text = Component.literal(String.format("%.1fs", remainingSec)).withStyle(ChatFormatting.AQUA);
            //   java: 					collector.submitText(text, Vec3.atCenterOf(beacon.getKey().above()), 3, true);
            //   java: 				}
            //   java: 			}
        }
    }

}
