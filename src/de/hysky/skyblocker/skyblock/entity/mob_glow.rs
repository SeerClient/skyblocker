//! Ported from de.hysky.skyblocker.skyblock.entity.MobGlow by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::dungeon::livid_color::LividColor;
use crate::de::hysky::skyblocker::skyblock::entity::mob_glow_adder::MobGlowAdder;
use crate::net::azureaaron::renderchest::api::custom_glow_callback;
use crate::net::azureaaron::renderchest::api::glow_constants::GlowConstants;
use mcrust::net::minecraft::client::renderer::entity::state::entity_render_state::EntityRenderState;
use mcrust::net::minecraft::util::argb;
use mcrust::net::minecraft::world::entity::decoration::armor_stand::ArmorStand;
use mcrust::net::minecraft::world::entity::entity::Entity;
use mcrust::net::minecraft::world::level::level::Level;
use mcrust::net::minecraft::world::phys::aabb::AABB;
use mcrust::net::minecraft::network::chat::component::Component as _;
use mcrust::net::minecraft::world::level::entity_getter::EntityGetter as _;

/// java: public static final int NO_GLOW = GlowConstants.NO_GLOW;
pub static NO_GLOW: LazyLock<i32> = LazyLock::new(|| GlowConstants::NO_GLOW);
/// java: private static final List<MobGlowAdder> ADDERS = new ArrayList<>();
pub static ADDERS: compat::MainThread<Vec<compat::Shared<dyn MobGlowAdder>>> = compat::MainThread::new(|| Vec::new());
/// java: private static final Object2IntMap<Entity> CACHE = new Object2IntOpenHashMap<>();
///
/// Keyed by entity id (Java's key is the live entity, compared by identity).
pub static CACHE: compat::MainThread<std::collections::HashMap<i32, i32>> = compat::MainThread::new(|| std::collections::HashMap::new());

/// java: public class MobGlow
#[derive(Clone)]
pub struct MobGlow {}

impl MobGlow {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        // java: ClientTickEvents.END_LEVEL_TICK.register(_ -> clearCache());
        // (Fabric fires it at the end of `ClientLevel.tick`, inside `Minecraft.tick` and only while a level exists. The
        // port runs `ClientLevel.tick`'s statements inline in `Minecraft.tick` - there is no hooked `ClientLevel#tick` -
        // so the listener runs at the end of that same client tick, with the same level guard; nothing renders between
        // the two points, which is where the cache is read.)
        mcrust::mcrust::modding::events::client_tick_events::END_CLIENT_TICK.register(Arc::new(|_client: &mut mcrust::net::minecraft::client::minecraft::Minecraft| {
            if mcrust::mcrust::modding::game::level().is_some() {
                MobGlow::clear_cache();
            }
        }));
        // java: CustomGlowCallback.EVENT.register(MobGlow::applyCustomGlow);
        custom_glow_callback::EVENT.register(Arc::new(|entity: &dyn Entity, state: &EntityRenderState| MobGlow::apply_custom_glow(entity, state)));
    }

    /// java: private static int applyCustomGlow(Entity entity, EntityRenderState state)
    pub fn apply_custom_glow(entity: &dyn Entity, state: &EntityRenderState) -> i32 {
        let mut allow_glow_in_livid: bool = LividColor::allow_glow();
        let mut custom_glow: bool = MobGlow::has_or_compute_mob_glow(entity);
        let mut allow_glow: bool = (allow_glow_in_livid && state.appears_glowing() || custom_glow);
        if allow_glow && custom_glow {
            return argb::opaque(MobGlow::get_mob_glow(entity));
        } else if !allow_glow {
            // java: return GlowConstants.REMOVE_GLOW;
            return GlowConstants::REMOVE_GLOW;
        }
        // java: return GlowConstants.NO_GLOW;
        return GlowConstants::NO_GLOW;
    }

    /// java: protected static void registerGlowAdder(MobGlowAdder adder)
    pub fn register_glow_adder(mut adder: compat::Shared<dyn MobGlowAdder>) {
        ADDERS.get_mut().push(adder.clone());
    }

    /// java: public static boolean hasOrComputeMobGlow(Entity entity)
    pub fn has_or_compute_mob_glow(entity: &dyn Entity) -> bool {
        if CACHE.get_mut().contains_key(&entity.base().get_id()) {
            return true;
        }
        let mut color: i32 = MobGlow::compute_mob_glow(entity);
        if color != *NO_GLOW {
            CACHE.get_mut().insert(entity.base().get_id(), color);
            return true;
        }
        return false;
    }

    /// java: public static int getMobGlow(Entity entity)
    pub fn get_mob_glow(entity: &dyn Entity) -> i32 {
        // Object2IntMap.getInt: 0 for an absent key
        return CACHE.get_mut().get(&entity.base().get_id()).copied().unwrap_or_default();
    }

    /// java: public static int getMobGlowOrDefault(Entity entity, int defaultColor)
    pub fn get_mob_glow_or_default(entity: &dyn Entity, mut default_color: i32) -> i32 {
        return CACHE.get_mut().get(&entity.base().get_id()).cloned().unwrap_or_else(|| default_color);
    }

    /// java: public static void clearCache()
    pub fn clear_cache() {
        CACHE.get_mut().clear();
    }

    /// java: private static int computeMobGlow(Entity entity)
    pub fn compute_mob_glow(entity: &dyn Entity) -> i32 {
        for adder in ADDERS.get_mut().iter() {
            if { let __f = compat::dispatch(adder, |__t| __t.is_enabled()); __f(adder) } {
                let mut glow_colour: i32 = ({ let __f = compat::dispatch(adder, |__t| __t.compute_colour()); __f(adder, entity) });
                if glow_colour != *NO_GLOW {
                    return glow_colour;
                }
            }
        }
        return *NO_GLOW;
    }

    /// java: public static String getArmorStandName(Entity entity)
    pub fn get_armor_stand_name(entity: &dyn Entity) -> String {
        let mut armor_stands: Vec<i32> = MobGlow::get_armor_stands(entity);
        if armor_stands.is_empty() {
            return "".to_string();
        }
        return mcrust::mcrust::modding::game::with_entity(armor_stands[0], |e| e.get_name().get_string()).unwrap_or_default();
    }

    /// java: public static List<ArmorStand> getArmorStands(Entity entity)
    ///
    /// The stands by entity id (read them through `game::with_entity`). `entity` is lent by a caller that may hold
    /// it, so the sweep leaves it out and it is checked here: it answers for itself through its own reference.
    pub fn get_armor_stands(entity: &dyn Entity) -> Vec<i32> {
        let Some(level) = mcrust::mcrust::modding::game::level() else {
            return Vec::new();
        };
        return MobGlow::get_armor_stands_except(&level, &entity.base().get_bounding_box(), Some(entity));
    }

    /// java: public static List<ArmorStand> getArmorStands(Level world, AABB box)
    pub fn get_armor_stands_2(world: &Level, r#box: &AABB) -> Vec<i32> {
        return MobGlow::get_armor_stands_except(world, r#box, None);
    }

    /// `world.getEntitiesOfClass(ArmorStand.class, box.inflate(0, 2, 0), EntitySelector.ENTITY_NOT_BEING_RIDDEN)`, with
    /// `held` (an entity the caller holds) tested through its own reference instead of the level's lock.
    fn get_armor_stands_except(world: &Level, r#box: &AABB, held: Option<&dyn Entity>) -> Vec<i32> {
        use mcrust::net::minecraft::world::level::entity_getter::EntityGetter as _;
        let bounds = (r#box).inflate3((0) as f64, (2) as f64, (0) as f64);
        let is_stand = |e: &dyn Entity| e.as_any().is_some_and(|a| a.is::<ArmorStand>());
        let mut stands: Vec<i32> = Vec::new();
        if let Some(held) = held {
            // the held entity is in the box it was asked about; ENTITY_NOT_BEING_RIDDEN = alive, no passengers, not riding
            let alive = match held.as_living_entity() {
                Some(living) => living.living().is_alive(),
                None => held.base().is_alive(),
            };
            if is_stand(held) && alive && !held.base().is_vehicle() && !held.base().is_passenger() {
                stands.push(held.base().get_id());
            }
        }
        let found = world.get_entities(held.map(|h| h.base().get_id()), &bounds, &|e| is_stand(e) && mcrust::net::minecraft::world::entity::entity_selector::entity_not_being_ridden(e));
        stands.extend(found.iter().map(|stand| mcrust::mcrust::modding::game::lock_entity(&**stand, "mob_glow.rs:getArmorStands").base().get_id()));
        return stands;
    }

    /// java: public static String getArmorStandNameTrick(Entity entity)
    pub fn get_armor_stand_name_trick(entity: &dyn Entity) -> String {
        // entity.level().getEntity(entity.getId() + 1)
        let possible_name_tag = mcrust::mcrust::modding::game::with_entity(entity.base().get_id().wrapping_add(1), |e| (e.as_any().is_some_and(|a| a.is::<ArmorStand>()), e.get_plain_text_name()));
        return (if let Some((true, name)) = possible_name_tag { name } else { "ERR NO ARMR STAND".to_string() });
    }

}
