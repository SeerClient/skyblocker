//! Ported from de.hysky.skyblocker.skyblock.crimson.dojo.ForceTestHelper by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_collector::PrimitiveCollector;
use mcrust::java::text::decimal_format::DecimalFormat;
use mcrust::net::minecraft::world::entity::entity::Entity;
use mcrust::net::minecraft::world::entity::monster::zombie::zombie::Zombie;

// TODO(fabric2seer): `new` takes 2 argument(s) in the port (`pattern: &str, symbols: DecimalFormatSymbols`), Java passes 1
//   java: private static final DecimalFormat FORMATTER = new DecimalFormat("0.0");
/// java: private static final DecimalFormat FORMATTER = new DecimalFormat("0.0");
pub static FORMATTER: compat::MainThread<DecimalFormat> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `FORMATTER` - see the TODO above"));
/// java: private static final int ZOMBIE_LIFE_TIME = 10100;
pub const ZOMBIE_LIFE_TIME: i32 = 10100;
/// java: private static final Object2LongOpenHashMap<Zombie> zombies = new Object2LongOpenHashMap<>();
pub static ZOMBIES: compat::MainThread<std::collections::HashMap<Zombie, i64>> = compat::MainThread::new(|| std::collections::HashMap::new());

/// java: public class ForceTestHelper
#[derive(Clone)]
pub struct ForceTestHelper {}

impl ForceTestHelper {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: protected static void reset()
    pub fn reset() {
        ZOMBIES.get_mut().clear();
    }

    /// java: protected static boolean shouldGlow(String name)
    pub fn should_glow(name: &str) -> bool {
        return name.contains("-");
    }

    /// java: protected static int getColor()
    pub fn get_color() -> i32 {
        // TODO(fabric2seer): `.getRGB(..)` on a value of unknown type
        //   java: return Color.RED.getRGB();
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: protected static void onEntitySpawn(Entity entity)
    pub fn on_entity_spawn(entity: &dyn Entity) {
        if let Some(zombie) = entity.as_any().unwrap_or(&() as &dyn std::any::Any).downcast_ref::<Zombie>() {
            // TODO(fabric2seer): the mod keeps the game's `Zombie` (Java holds a reference to the live object); the port's `Zombie` is not a value to copy and the port lends the mod no handle to it
            // rust: ZOMBIES.get_mut().insert(zombie.clone(), (compat::millis()).wrapping_add(ZOMBIE_LIFE_TIME as i64));
        }
    }

    /// java: protected static void onEntityAttacked(Entity entity)
    pub fn on_entity_attacked(entity: &dyn Entity) {
        if let Some(zombie) = entity.as_any().unwrap_or(&() as &dyn std::any::Any).downcast_ref::<Zombie>() {
            if ZOMBIES.get_mut().contains_key(zombie) {
                // TODO(fabric2seer): the mod keeps the game's `Zombie` (Java holds a reference to the live object); the port's `Zombie` is not a value to copy and the port lends the mod no handle to it
                // rust: ZOMBIES.get_mut().insert(zombie.clone(), (compat::millis()).wrapping_add(ZOMBIE_LIFE_TIME as i64));
            }
        }
    }

    /// java: protected static void onEntityDespawn(Entity entity)
    pub fn on_entity_despawn(entity: &dyn Entity) {
        if let Some(zombie) = entity.as_any().unwrap_or(&() as &dyn std::any::Any).downcast_ref::<Zombie>() {
            // TODO(fabric2seer): Map.removeLong(..)
            //   java: zombies.removeLong(zombie);
        }
    }

    /// java: protected static void extractRendering(PrimitiveCollector collector)
    pub fn extract_rendering(collector: &compat::Shared<dyn PrimitiveCollector>) {
        let mut current_time: i64 = compat::millis();
        // TODO(fabric2seer): Map.object2LongEntrySet(..)
        //   java: for (Object2LongMap.Entry<Zombie> zombie : zombies.object2LongEntrySet()) {
        //   java: 			float secondsTime = Math.max((zombie.getLongValue() - currentTime) / 1000f, 0);
        //   java:
        //   java: 			MutableComponent text = Component.literal(FORMATTER.format(secondsTime));
        //   java: 			if (secondsTime > 1) {
        //   java: 				text = text.withStyle(ChatFormatting.GREEN);
        //   java: 			} else if (secondsTime > 0) {
        //   java: 				text = text.withStyle(ChatFormatting.YELLOW);
        //   java: 			} else {
        //   java: 				text = text.withStyle(ChatFormatting.RED);
        //   java: 			}
        //   java:
        //   java: 			Vec3 labelPos = zombie.getKey().getEyePosition(RenderHelper.getTickCounter().getGameTimeDeltaPartialTick(false));
        //   java: 			collector.submitText(text, labelPos, 1.5f, true);
        //   java: 		}
    }

}
