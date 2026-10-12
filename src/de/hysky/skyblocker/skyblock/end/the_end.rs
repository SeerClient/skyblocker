//! Ported from de.hysky.skyblocker.skyblock.end.TheEnd by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::events::skyblock_events;
use crate::de::hysky::skyblocker::skyblock::end::end_hud_widget::EndHudWidget;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element_based_widget::ElementBasedWidget;
use crate::de::hysky::skyblocker::utils::color_utils::ColorUtils;
use crate::de::hysky::skyblocker::utils::data::profiled_data::ProfiledData;
use crate::de::hysky::skyblocker::utils::render::level_render_extraction_callback;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_collector::PrimitiveCollector;
use crate::de::hysky::skyblocker::utils::render::renderable::Renderable;
use crate::de::hysky::skyblocker::utils::waypoint::waypoint::Type;
use crate::de::hysky::skyblocker::utils::waypoint::waypoint::Waypoint;
use crate::de::hysky::skyblocker::utils::waypoint::waypoint::WaypointBase;
use crate::entry::skyblocker_mod;
use mcrust::com::mojang::serialization::codec;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::com::mojang::serialization::codecs::record_codec_builder;
use mcrust::java::text::decimal_format_symbols::Locale;
use mcrust::java::util::uuid::Uuid;
use mcrust::net::minecraft::client::multiplayer::client_level::ClientLevel;
use mcrust::net::minecraft::core::block_pos::BlockPos;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::world::entity::decoration::armor_stand::ArmorStand;
use mcrust::net::minecraft::world::entity::entity::Entity;
use mcrust::net::minecraft::world::entity::monster::ender_man::EnderMan;
use mcrust::net::minecraft::world::item::dye_color::DyeColor;
use mcrust::net::minecraft::world::level::block::blocks;
use mcrust::net::minecraft::world::level::block::state::block_state::BlockState;
use crate::de::hysky::skyblocker::utils::data::json_data::JsonData as _;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final Path FILE = SkyblockerMod.CONFIG_DIR.resolve("end.json");
pub static FILE: compat::MainThread<std::path::PathBuf> = compat::MainThread::new(|| skyblocker_mod::CONFIG_DIR.get_mut().join("end.json"));
/// java: private static final Pattern END_STONE_PROTECTOR_TREMOR = Pattern.compile("^You feel a tremor from beneath the earth!$");
pub static END_STONE_PROTECTOR_TREMOR: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("^You feel a tremor from beneath the earth!$"));
/// java: private static final Pattern END_STONE_PROTECTOR_RISES = Pattern.compile("^The ground begins to shake as an End Stone Protector rises from below!$");
pub static END_STONE_PROTECTOR_RISES: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("^The ground begins to shake as an End Stone Protector rises from below!$"));
/// java: private static final Pattern END_STONE_PROTECTOR_FIGHT_STARTS = Pattern.compile("^BEWARE - An (?:Endstone|End Stone) Protector has risen!$");
pub static END_STONE_PROTECTOR_FIGHT_STARTS: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("^BEWARE - An (?:Endstone|End Stone) Protector has risen!$"));
/// java: private static final Pattern SPECIAL_ZEALOT_SPAWNED = Pattern.compile("^A special Zealot has spawned nearby!$");
pub static SPECIAL_ZEALOT_SPAWNED: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("^A special Zealot has spawned nearby!$"));
/// java: private static final List<ProtectorLocation> PROTECTOR_LOCATIONS = List.of( new ProtectorLocation(-649, -219, Component.translatable("skyblocker.end.hud.protectorLocations.left")), new ProtectorLocati
pub static PROTECTOR_LOCATIONS: compat::MainThread<Vec<ProtectorLocation>> = compat::MainThread::new(|| vec![ProtectorLocation::new(-649, -219, component::translatable("skyblocker.end.hud.protectorLocations.left")), ProtectorLocation::new(-644, -269, component::translatable("skyblocker.end.hud.protectorLocations.front")), ProtectorLocation::new(-689, -273, component::translatable("skyblocker.end.hud.protectorLocations.center")), ProtectorLocation::new(-727, -284, component::translatable("skyblocker.end.hud.protectorLocations.back")), ProtectorLocation::new(-639, -328, component::translatable("skyblocker.end.hud.protectorLocations.rightFront")), ProtectorLocation::new(-678, -332, component::translatable("skyblocker.end.hud.protectorLocations.rightBack"))]);
/// java: private static final Set<UUID> HIT_ZEALOTS = new ObjectOpenHashSet<>();
pub static HIT_ZEALOTS: compat::MainThread<std::collections::HashSet<Uuid>> = compat::MainThread::new(|| std::collections::HashSet::new());
/// java: public static final ProfiledData<EndStats> PROFILES_STATS = new ProfiledData<>(FILE, EndStats.CODEC);
pub static PROFILES_STATS: compat::MainThread<compat::Shared<ProfiledData<EndStats>>> = compat::MainThread::new(|| ProfiledData::new(FILE.get_mut().clone(), END_STATS_CODEC.get_mut().clone()));
/// java: public static @Nullable ProtectorLocation currentProtectorLocation = null;
pub static CURRENT_PROTECTOR_LOCATION: compat::MainThread<Option<ProtectorLocation>> = compat::MainThread::new(|| None);
/// java: public static int stage = 0;
pub static STAGE: LazyLock<Mutex<i32>> = LazyLock::new(|| Mutex::new(0));
/// java: private static final Codec<EndStats> CODEC = RecordCodecBuilder.create(instance -> instance.group( Codec.INT.fieldOf("totalZealotKills").forGetter(EndStats::totalZealotKills), Codec.INT.fieldOf("zealo
pub static END_STATS_CODEC: compat::MainThread<CodecRef<EndStats>> = compat::MainThread::new(|| record_codec_builder::create(record_codec_builder::apply3(record_codec_builder::of(|o: &EndStats| o.total_zealot_kills(), codec::field_of(mcrust::com::mojang::serialization::codec::int_codec(), "totalZealotKills")), record_codec_builder::of(|o: &EndStats| o.zealots_since_last_eye(), codec::field_of(mcrust::com::mojang::serialization::codec::int_codec(), "zealotsSinceLastEye")), record_codec_builder::of(|o: &EndStats| o.eyes(), codec::field_of(mcrust::com::mojang::serialization::codec::int_codec(), "eyes")), move |__a0, __a1, __a2| EndStats::new(__a0.clone(), __a1.clone(), __a2.clone()))));
/// java: public static final Supplier<EndStats> EMPTY = () -> new EndStats(0, 0, 0);
pub static END_STATS_EMPTY: compat::MainThread<Arc<dyn Fn() -> EndStats>> = compat::MainThread::new(|| Arc::new(move || EndStats::new(0, 0, 0)));

/// java: public class TheEnd
#[derive(Clone)]
pub struct TheEnd {}

impl TheEnd {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        // TODO(fabric2seer): AttackEntityCallback.EVENT.register: no SeerClient equivalent yet
        //   java: AttackEntityCallback.EVENT.register((_, _, _, entity, _) -> {
        //   java: 			if (entity instanceof EnderMan enderman && isZealot(enderman)) {
        //   java: 				HIT_ZEALOTS.add(enderman.getUUID());
        //   java: 			}
        //   java: 			return InteractionResult.PASS;
        //   java: 		});
        // TODO(fabric2seer): ClientChunkEvents.CHUNK_LOAD.register: no SeerClient equivalent yet
        //   java: ClientChunkEvents.CHUNK_LOAD.register((world, chunk) -> {
        //   java: 			Area area = Utils.getArea();
        //   java: 			if (Utils.isInTheEnd() || area.equals(Area.TheEnd.THE_END) || area.equals(Area.TheEnd.DRAGONS_NEST)) {
        //   java: 				ChunkPos pos = chunk.getPos();
        //   java: 				AABB box = new AABB(pos.getMinBlockX(), 4, pos.getMinBlockZ(), pos.getMaxBlockX() + 1, 5, pos.getMaxBlockZ() + 1);
        //   java: 				for (ProtectorLocation protectorLocation : PROTECTOR_LOCATIONS) {
        //   java: 					if (box.contains(protectorLocation.x(), 4, protectorLocation.z())) {
        //   java: 						if (isProtectorHere(world, protectorLocation)) break;
        //   java: 					}
        //   java: 				}
        //   java: 			}
        //   java: 		});
        skyblock_events::PROFILE_CHANGE.register(Arc::new(move |_, _| { ({ let __r = &EndHudWidget::get_instance(); let __f = compat::dispatch(__r, |__t| __t.update()); __f(&compat::view(__r, |a| a as compat::Shared<dyn ElementBasedWidget>)) }); }));
        skyblock_events::LOCATION_CHANGE.register(Arc::new(move |_| {
            TheEnd::reset_location();
            HIT_ZEALOTS.get_mut().clear();
        }));
        // TODO(fabric2seer): ClientReceiveMessageEvents.ALLOW_GAME.register: no SeerClient equivalent yet
        //   java: ClientReceiveMessageEvents.ALLOW_GAME.register((message, overlay) -> {
        //   java: 			if (!Utils.isInTheEnd() || overlay) return true;
        //   java: 			onMessage(message.getString());
        //   java: 			return true;
        //   java: 		});
        level_render_extraction_callback::EVENT.register(Arc::new(move |__a0| { TheEnd::extract_rendering(&__a0); }));
        PROFILES_STATS.get_mut().lock().borrow_mut().init();
    }

    /// java: private static void onMessage(String text)
    pub fn on_message(text: &str) {
        if END_STONE_PROTECTOR_TREMOR.matcher(text).matches() {
            if compat::get_state(&STAGE) == 0 {
                TheEnd::check_all_protector_locations();
            } else {
                compat::set_state(&STAGE, (compat::get_state(&STAGE)).wrapping_add(1));
            }
        } else if END_STONE_PROTECTOR_RISES.matcher(text).matches() {
            if CURRENT_PROTECTOR_LOCATION.get_mut().is_none() {
                TheEnd::check_all_protector_locations();
            }
            compat::set_state(&STAGE, 5);
        } else if END_STONE_PROTECTOR_FIGHT_STARTS.matcher(text).matches() {
            TheEnd::reset_location();
        } else if SPECIAL_ZEALOT_SPAWNED.matcher(text).matches() {
            TheEnd::on_special_zealot();
        }
        ({ let __r = &EndHudWidget::get_instance(); let __f = compat::dispatch(__r, |__t| __t.update()); __f(&compat::view(__r, |a| a as compat::Shared<dyn ElementBasedWidget>)) });
    }

    /// java: private static void checkAllProtectorLocations()
    pub fn check_all_protector_locations() {
        let Some(mut world) = mcrust::mcrust::modding::game::level() else {
            return;
        };
        for protector_location in PROTECTOR_LOCATIONS.get_mut().iter() {
            if !(world.has_chunk(protector_location.x().wrapping_shr((4) as u32), protector_location.z().wrapping_shr((4) as u32))) {
                continue;
            }
            if TheEnd::is_protector_here(&world, protector_location.clone()) {
                break;
            }
        }
    }

    /// java: private static boolean isProtectorHere(ClientLevel world, ProtectorLocation protectorLocation)
    pub fn is_protector_here(world: &ClientLevel, mut protector_location: ProtectorLocation) -> bool {
        for i in 0..5 {
            let mut state: Arc<BlockState> = world.get_block_state_typed(&BlockPos::new(protector_location.x, i.wrapping_add(5), protector_location.z));
            if state.is_block(&blocks::blocks().get("player_wall_head")) || state.is_block(&blocks::blocks().get("player_head")) {
                compat::set_state(&STAGE, i.wrapping_add(1));
                *CURRENT_PROTECTOR_LOCATION.get_mut() = Some(protector_location.clone());
                ({ let __r = &EndHudWidget::get_instance(); let __f = compat::dispatch(__r, |__t| __t.update()); __f(&compat::view(__r, |a| a as compat::Shared<dyn ElementBasedWidget>)) });
                return true;
            }
        }
        return false;
    }

    /// java: private static void resetLocation()
    pub fn reset_location() {
        compat::set_state(&STAGE, 0);
        *CURRENT_PROTECTOR_LOCATION.get_mut() = None;
    }

    /// java: public static void onEntityDeath(Entity entity)
    pub fn on_entity_death(entity: &dyn Entity) {
        // TODO(fabric2seer): an `instanceof` pattern under `||` / `!`
        //   java: if (!(entity instanceof EnderMan enderman) || !isZealot(enderman)) return;
        // TODO(fabric2seer): the condition: unknown name `enderman`
        //   java: HIT_ZEALOTS.contains(..)
        if todo!("fabric2seer: the condition (TODO above)") {
            let mut stats: EndStats = ProfiledData::compute_if_absent(PROFILES_STATS.get_mut(), END_STATS_EMPTY.get_mut().clone());
            ProfiledData::put(PROFILES_STATS.get_mut(), EndStats::new(stats.total_zealot_kills().wrapping_add(1), stats.zealots_since_last_eye().wrapping_add(1), stats.eyes()));
            // TODO(fabric2seer): unknown name `enderman`
            //   java: HIT_ZEALOTS.remove(enderman.getUUID());
            ({ let __r = &EndHudWidget::get_instance(); let __f = compat::dispatch(__r, |__t| __t.update()); __f(&compat::view(__r, |a| a as compat::Shared<dyn ElementBasedWidget>)) });
        }
    }

    /// java: public static void onSpecialZealot()
    pub fn on_special_zealot() {
        let mut stats: EndStats = ProfiledData::compute_if_absent(PROFILES_STATS.get_mut(), END_STATS_EMPTY.get_mut().clone());
        ProfiledData::put(PROFILES_STATS.get_mut(), EndStats::new(stats.total_zealot_kills(), 0, stats.eyes().wrapping_add(1)));
    }

    /// java: public static boolean isZealot(EnderMan enderman)
    pub fn is_zealot(enderman: &EnderMan) -> bool {
        if compat::component_of(enderman.get_name()).get_string().to_lowercase().contains("zealot") {
            return true;
        }
        debug_assert!(mcrust::mcrust::modding::game::level().is_some());
        // TODO(fabric2seer): `.getName(..)` on a T
        //   java: List<ArmorStand> entities = CLIENT.level.getEntitiesOfClass(
        //   java: 				ArmorStand.class,
        //   java: 				enderman.getDimensions(enderman.getPose()).makeBoundingBox(enderman.position()).inflate(1),
        //   java: 				armorStandEntity -> armorStandEntity.getName().getString().toLowerCase(Locale.ENGLISH).contains("zealot"));
        let mut entities: Vec<ArmorStand> = todo!("fabric2seer: the initialiser of `entities` (TODO above)");
        return !entities.is_empty();
    }

    /// java: public static boolean isSpecialZealot(EnderMan enderman)
    pub fn is_special_zealot(enderman: &EnderMan) -> bool {
        let mut carried_block: Option<Arc<BlockState>> = enderman.get_carried_block();
        return (carried_block.is_some() && carried_block.as_ref().unwrap().is_block(&blocks::blocks().get("end_portal_frame")) && TheEnd::is_zealot(enderman));
    }

    /// java: private static void extractRendering(PrimitiveCollector collector)
    pub fn extract_rendering(collector: &compat::Shared<dyn PrimitiveCollector>) {
        if !(SkyblockerConfigManager::get().other_locations.end.waypoint) {
            return;
        }
        if CURRENT_PROTECTOR_LOCATION.get_mut().is_none() || compat::get_state(&STAGE) != 5 {
            return;
        }
        ({ let __r = &CURRENT_PROTECTOR_LOCATION.get_mut().as_ref().unwrap().waypoint(); let __f = compat::dispatch(__r, |__t| __t.extract_rendering()); __f(&compat::view(__r, |a| a as compat::Shared<dyn Renderable>), collector) });
    }

}

/// java: public record EndStats(int totalZealotKills, int zealotsSinceLastEye, int eyes)
#[derive(Clone, Debug, PartialEq)]
pub struct EndStats {
    /// java: int totalZealotKills
    pub total_zealot_kills: i32,
    /// java: int zealotsSinceLastEye
    pub zealots_since_last_eye: i32,
    /// java: int eyes
    pub eyes: i32,
}

impl EndStats {
    /// java: the canonical record constructor
    pub fn new(mut total_zealot_kills: i32, mut zealots_since_last_eye: i32, mut eyes: i32) -> Self {
        Self { total_zealot_kills, zealots_since_last_eye, eyes }
    }

    /// java: the record accessor `totalZealotKills()`
    pub fn total_zealot_kills(&self) -> i32 {
        self.total_zealot_kills
    }

    /// java: the record accessor `zealotsSinceLastEye()`
    pub fn zealots_since_last_eye(&self) -> i32 {
        self.zealots_since_last_eye
    }

    /// java: the record accessor `eyes()`
    pub fn eyes(&self) -> i32 {
        self.eyes
    }

}

/// java: public record ProtectorLocation(int x, int z, Component name, Waypoint waypoint)
#[derive(Clone, Debug)]
pub struct ProtectorLocation {
    /// java: int x
    pub x: i32,
    /// java: int z
    pub z: i32,
    /// java: Component name
    pub name: MutableComponent,
    /// java: Waypoint waypoint
    pub waypoint: compat::Shared<dyn Waypoint>,
}

impl ProtectorLocation {
    /// java: public ProtectorLocation(int x, int z, Component name)
    pub fn new(mut x: i32, mut z: i32, mut name: MutableComponent) -> Self {
        let mut this = Self::new(x, z, name.clone(), (WaypointBase::new(BlockPos::new(x, 0, z), Type::Waypoint, ColorUtils::get_float_components_dye_color(&DyeColor::Magenta)) as compat::Shared<dyn Waypoint>));
        this
    }

    /// java: the canonical record constructor
    pub fn new(mut x: i32, mut z: i32, mut name: MutableComponent, mut waypoint: compat::Shared<dyn Waypoint>) -> Self {
        Self { x, z, name, waypoint }
    }

    /// java: the record accessor `x()`
    pub fn x(&self) -> i32 {
        self.x
    }

    /// java: the record accessor `z()`
    pub fn z(&self) -> i32 {
        self.z
    }

    /// java: the record accessor `name()`
    pub fn name(&self) -> MutableComponent {
        self.name.clone()
    }

    /// java: the record accessor `waypoint()`
    pub fn waypoint(&self) -> compat::Shared<dyn Waypoint> {
        self.waypoint.clone()
    }

}
