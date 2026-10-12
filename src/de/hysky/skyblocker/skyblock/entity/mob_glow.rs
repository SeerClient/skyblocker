//! Ported from de.hysky.skyblocker.skyblock.entity.MobGlow by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::dungeon::livid_color::LividColor;
use crate::de::hysky::skyblocker::skyblock::entity::mob_glow_adder::MobGlowAdder;
use mcrust::net::minecraft::client::renderer::entity::state::entity_render_state::EntityRenderState;
use mcrust::net::minecraft::util::argb;
use mcrust::net::minecraft::world::entity::decoration::armor_stand::ArmorStand;
use mcrust::net::minecraft::world::entity::entity::Entity;
use mcrust::net::minecraft::world::level::level::Level;
use mcrust::net::minecraft::world::phys::aabb::AABB;
use mcrust::net::minecraft::network::chat::component::Component as _;
use mcrust::net::minecraft::world::level::entity_getter::EntityGetter as _;

// TODO(fabric2seer): static field GlowConstants.NO_GLOW
//   java: public static final int NO_GLOW = GlowConstants.NO_GLOW;
/// java: public static final int NO_GLOW = GlowConstants.NO_GLOW;
pub static NO_GLOW: LazyLock<i32> = LazyLock::new(|| todo!("fabric2seer: the initialiser of `NO_GLOW` - see the TODO above"));
/// java: private static final List<MobGlowAdder> ADDERS = new ArrayList<>();
pub static ADDERS: compat::MainThread<Vec<compat::Shared<dyn MobGlowAdder>>> = compat::MainThread::new(|| Vec::new());
/// java: private static final Object2IntMap<Entity> CACHE = new Object2IntOpenHashMap<>();
pub static CACHE: compat::MainThread<std::collections::HashMap<Box<dyn Entity>, i32>> = compat::MainThread::new(|| std::collections::HashMap::new());

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
        // TODO(fabric2seer): no SeerClient event yet: hook RETURN of ClientLevel#tick (`net.minecraft.client.multiplayer.ClientLevel#tick`: ported but not hooked)
        //   java: ClientTickEvents.END_LEVEL_TICK.register(_ -> clearCache());
        // TODO(fabric2seer): static field CustomGlowCallback.EVENT
        //   java: CustomGlowCallback.EVENT.register(MobGlow::applyCustomGlow);
    }

    /// java: private static int applyCustomGlow(Entity entity, EntityRenderState state)
    pub fn apply_custom_glow(entity: &dyn Entity, state: &EntityRenderState) -> i32 {
        let mut allow_glow_in_livid: bool = LividColor::allow_glow();
        let mut custom_glow: bool = MobGlow::has_or_compute_mob_glow(entity.clone());
        let mut allow_glow: bool = (allow_glow_in_livid && state.appears_glowing() || custom_glow);
        if allow_glow && custom_glow {
            return argb::opaque(MobGlow::get_mob_glow(entity));
        } else if !allow_glow {
            // TODO(fabric2seer): static field GlowConstants.REMOVE_GLOW
            //   java: return GlowConstants.REMOVE_GLOW;
        }
        // TODO(fabric2seer): static field GlowConstants.NO_GLOW
        //   java: return GlowConstants.NO_GLOW;
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: protected static void registerGlowAdder(MobGlowAdder adder)
    pub fn register_glow_adder(mut adder: compat::Shared<dyn MobGlowAdder>) {
        ADDERS.get_mut().push(adder.clone());
    }

    /// java: public static boolean hasOrComputeMobGlow(Entity entity)
    pub fn has_or_compute_mob_glow(mut entity: Box<dyn Entity>) -> bool {
        if CACHE.get_mut().contains_key(&entity) {
            return true;
        }
        let mut color: i32 = MobGlow::compute_mob_glow(&entity);
        if color != *NO_GLOW {
            CACHE.get_mut().insert(entity.clone(), color);
            return true;
        }
        return false;
    }

    /// java: public static int getMobGlow(Entity entity)
    pub fn get_mob_glow(entity: &dyn Entity) -> i32 {
        return CACHE.get_mut().get(entity).copied().unwrap_or_default();
    }

    /// java: public static int getMobGlowOrDefault(Entity entity, int defaultColor)
    pub fn get_mob_glow_or_default(entity: &dyn Entity, mut default_color: i32) -> i32 {
        return CACHE.get_mut().get(entity).cloned().unwrap_or_else(|| default_color);
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
        let mut armor_stands: Vec<ArmorStand> = MobGlow::get_armor_stands(entity);
        if armor_stands.is_empty() {
            return "".to_string();
        }
        return compat::component_of(armor_stands[0].get_name()).get_string();
    }

    /// java: public static List<ArmorStand> getArmorStands(Entity entity)
    pub fn get_armor_stands(entity: &dyn Entity) -> Vec<ArmorStand> {
        // TODO(fabric2seer): `Entity.level` has no Rust method in the port (looked for `level` in Entity and its supertypes)
        // rust: return MobGlow::get_armor_stands_2(&entity.level(), &entity.get_bounding_box());
        return todo!("fabric2seer: see the TODO above");
    }

    /// java: public static List<ArmorStand> getArmorStands(Level world, AABB box)
    pub fn get_armor_stands_2(world: &Level, r#box: &AABB) -> Vec<ArmorStand> {
        // TODO(fabric2seer): static `EntitySelector.ENTITY_NOT_BEING_RIDDEN` has no Rust counterpart in the port's EntitySelector file
        // TODO(fabric2seer): the port's `get_entities_of_class` ports this Java method with other parameters (base_class, except, bb, selector for Java's base_class, bb, selector)
        // TODO(fabric2seer): `get_entities_of_class` takes 4 argument(s) in the port (`base_class: crate::net::minecraft::world::level::entity::entity_type_test::EntityClass, except: Option<i32>, bb: &AABB, selector: &dyn Fn(&(dyn Entity + Send + Sync)) -> bool`), Java passes 3
        // rust: return world.get_entities_of_class(std::any::TypeId::of::<ArmorStand>(), Some((r#box).inflate3((0) as f64, (2) as f64, (0) as f64)), &crate::unresolved::EntitySelector::ENTITY_NOT_BEING_RIDDEN);
        return todo!("fabric2seer: see the TODO above");
    }

    /// java: public static String getArmorStandNameTrick(Entity entity)
    pub fn get_armor_stand_name_trick(entity: &dyn Entity) -> String {
        // TODO(fabric2seer): `Entity.level` has no Rust method in the port (looked for `level` in Entity and its supertypes)
        // TODO(fabric2seer): `Level.getEntity` has no Rust method in the port (looked for `get_entity` in Level and its supertypes)
        // rust: let mut possible_name_tag: Box<dyn Entity> = entity.level().get_entity(entity.get_id().wrapping_add(1));
        let mut possible_name_tag: Box<dyn Entity> = todo!("fabric2seer: the initialiser of `possibleNameTag` (TODO above)");
        return (if possible_name_tag.as_any().unwrap_or(&() as &dyn std::any::Any).is::<ArmorStand>() { possible_name_tag.get_plain_text_name() } else { "ERR NO ARMR STAND".to_string() });
    }

}
