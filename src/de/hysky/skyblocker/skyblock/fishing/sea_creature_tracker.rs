//! Ported from de.hysky.skyblocker.skyblock.fishing.SeaCreatureTracker by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::fishing::fishing_hud_widget::FishingHudWidget;
use crate::de::hysky::skyblocker::skyblock::fishing::sea_creature::SeaCreature;
use crate::de::hysky::skyblocker::skyblock::item::skyblock_item_rarity::SkyblockItemRarity;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::widget_manager::WidgetManager;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::hud_widget::HudWidget;
use crate::de::hysky::skyblocker::utils::render::title::title::Title;
use crate::de::hysky::skyblocker::utils::render::title::title_container::TitleContainer;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler::Scheduler;
use crate::de::hysky::skyblocker::utils::time::skyblock_time::SkyblockTime;
use mcrust::net::minecraft::chat_formatting;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::multiplayer::client_level::ClientLevel;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::world::entity::decoration::armor_stand::ArmorStand;
use mcrust::net::minecraft::world::entity::entity::Entity;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final Pattern DOUBLE_HOOK_PATTERN = Pattern.compile("Double Hook!(?: Woot woot!)?");
pub static DOUBLE_HOOK_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("Double Hook!(?: Woot woot!)?"));
/// java: public static final int SEA_CREATURE_CAP = 10;
pub const SEA_CREATURE_CAP: i32 = 10;
/// java: private static final SequencedMap<Entity, LiveSeaCreature> seaCreatures = new LinkedHashMap<>();
/// (keyed by the entity's id: the level keeps the entity; Java's key is the object, compared by identity)
pub static SEA_CREATURES: compat::MainThread<indexmap::IndexMap<i32, LiveSeaCreature>> = compat::MainThread::new(|| indexmap::IndexMap::new());
/// java: private static @Nullable SeaCreature lastCatch;
pub static LAST_CATCH: LazyLock<Mutex<Option<SeaCreature>>> = LazyLock::new(|| Mutex::new(None));
/// java: private static boolean doubleHook = false;
pub static DOUBLE_HOOK: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));

/// java: public class SeaCreatureTracker
#[derive(Clone)]
pub struct SeaCreatureTracker {}

impl SeaCreatureTracker {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        // java: ClientReceiveMessageEvents.ALLOW_GAME.register(SeaCreatureTracker::onChatMessage);
        compat::register_allow_game(|text, overlay| SeaCreatureTracker::on_chat_message(text, overlay));
        // java: ClientEntityEvents.ENTITY_UNLOAD.register(SeaCreatureTracker::onEntityDespawn);
        mcrust::mcrust::modding::events::client_entity_events::register_entity_unload(|entity, client_world| SeaCreatureTracker::on_entity_despawn(entity, client_world));
    }

    /// java: private static void onEntityDespawn(Entity entity, ClientLevel clientWorld)
    pub fn on_entity_despawn(entity: &dyn Entity, client_world: &ClientLevel) {
        SEA_CREATURES.get_mut().shift_remove(&entity.base().get_id());
    }

    /// java: public static void onEntitySpawn(ArmorStand armorStand)
    pub fn on_entity_spawn(armor_stand: &ArmorStand) {
        if !armor_stand.is_invisible_field() || !armor_stand.base.base.has_custom_name() || !armor_stand.base.base.is_custom_name_visible() || compat::get_state(&LAST_CATCH).is_none() {
            return;
        }
        if compat::component_of(armor_stand.get_name()).get_string().contains(&compat::lock(&LAST_CATCH).unwrap().fields().name) {
            let __id = armor_stand.base.base.get_id();
            SEA_CREATURES.get_mut().insert(__id, LiveSeaCreature::new(compat::get_state(&LAST_CATCH).unwrap(), __id, compat::millis()));
            if compat::get_state(&DOUBLE_HOOK) {
                compat::set_state(&DOUBLE_HOOK, false);
            } else {
                compat::set_state(&LAST_CATCH, None);
            }
            SeaCreatureTracker::check_cap_notification();
            SeaCreatureTracker::check_rarity_notification();
            ({ let __r = scheduler::INSTANCE.get_mut(); let __f = compat::dispatch(__r, |__t| __t.schedule()); __f(__r, Arc::new(move || { SeaCreatureTracker::check_timer_notification(); }), (SkyblockerConfigManager::get().helpers.fishing.timer_length).wrapping_mul(20)) });
        }
    }

    /// java: private static void checkTimerNotification()
    pub fn check_timer_notification() {
        if !(SkyblockerConfigManager::get().helpers.fishing.sea_creature_timer_notification) || !(SeaCreatureTracker::is_creatures_alive()) {
            return;
        }
        if (((SkyblockerConfigManager::get().helpers.fishing.timer_length as i64).wrapping_mul(1000)).wrapping_sub(SeaCreatureTracker::get_oldest_sea_creature_age())).wrapping_abs() < (100 as i64) {
            TitleContainer::add_title_2(&Title::new_1(component::translatable("skyblocker.config.helpers.fishing.seaCreatureTimerNotification.notification").with_style_format(ChatFormatting::Red)), 60);
            if mcrust::mcrust::modding::game::with_player(|_| ()).is_none() {
                return;
            }
            mcrust::mcrust::modding::game::with_player(|player| { player.play_sound(mcrust::net::minecraft::sounds::sound_events::arrow_hit_player().clone(), 100.0f32, 0.1f32); });
        }
    }

    /// java: private static void checkRarityNotification()
    pub fn check_rarity_notification() {
        let mut rarity_threshold: SkyblockItemRarity = SkyblockerConfigManager::get().helpers.fishing.minimum_notification_rarity;
        if rarity_threshold == SkyblockItemRarity::Unknown {
            return;
        }
        // java: SeaCreature lastCreature = seaCreatures.sequencedValues().getLast().seaCreature;
        let mut last_creature: SeaCreature = SEA_CREATURES.get().values().last().expect("NoSuchElementException").sea_creature;
        let mut last_creature_rarity: SkyblockItemRarity = last_creature.fields().rarity;
        if compat::ord_int(last_creature_rarity.cmp(&rarity_threshold)) >= 0 {
            TitleContainer::add_title_2(&Title::new_1(component::literal(&last_creature.fields().name).with_color(last_creature_rarity.fields().color)), 60);
            if mcrust::mcrust::modding::game::with_player(|_| ()).is_none() {
                return;
            }
            mcrust::mcrust::modding::game::with_player(|player| { player.play_sound(mcrust::net::minecraft::sounds::sound_events::arrow_hit_player().clone(), 100.0f32, 0.1f32); });
        }
    }

    /// java: private static void checkCapNotification()
    pub fn check_cap_notification() {
        if !(SkyblockerConfigManager::get().helpers.fishing.sea_creature_cap_notification) {
            return;
        }
        if SeaCreatureTracker::sea_creature_count() == SEA_CREATURE_CAP {
            TitleContainer::add_title_2(&Title::new_1(component::translatable("skyblocker.config.helpers.fishing.seaCreatureCapNotification.notification").with_style_format(ChatFormatting::Red)), 60);
            if mcrust::mcrust::modding::game::with_player(|_| ()).is_none() {
                return;
            }
            mcrust::mcrust::modding::game::with_player(|player| { player.play_sound(mcrust::net::minecraft::sounds::sound_events::arrow_hit_player().clone(), 100.0f32, 0.1f32); });
        }
    }

    /// java: @SuppressWarnings("SameReturnValue") private static boolean onChatMessage(Component text, boolean overlay)
    pub fn on_chat_message(text: &MutableComponent, mut overlay: bool) -> bool {
        if !(WidgetManager::is_widget_in_current_screen(&(FishingHudWidget::get_instance() as compat::Shared<dyn HudWidget>))) || overlay {
            return true;
        }
        let mut message: String = chat_formatting::strip_formatting(&text.get_string());
        if DOUBLE_HOOK_PATTERN.matcher(&message).find() {
            compat::set_state(&DOUBLE_HOOK, true);
            return true;
        }
        for creature in SeaCreature::values() {
            if creature.fields().chat_message == message {
                compat::set_state(&LAST_CATCH, Some(creature));
                break;
            }
        }
        return true;
    }

    /// java: protected static long getOldestSeaCreatureAge()
    pub fn get_oldest_sea_creature_age() -> i64 {
        // java: return System.currentTimeMillis() - seaCreatures.sequencedValues().getFirst().spawnTime;
        return compat::millis().wrapping_sub(SEA_CREATURES.get().values().next().expect("NoSuchElementException").spawn_time);
    }

    /// java: protected static Boolean isCreaturesAlive()
    pub fn is_creatures_alive() -> bool {
        return !(SEA_CREATURES.get_mut().is_empty());
    }

    /// java: protected static ObjectFloatPair<Component> getTimerText(long currentTime)
    pub fn get_timer_text(mut current_time: i64) -> (MutableComponent, f32) {
        let mut max_time: i64 = (SkyblockerConfigManager::get().helpers.fishing.timer_length as i64).wrapping_mul(1000);
        let mut time: MutableComponent = SkyblockTime::format_time((((max_time.wrapping_sub(current_time) as f32) / 1000.0f32)) as f64);
        let mut percentage: f32 = ((((max_time.wrapping_sub(current_time)) as f32) / (max_time as f32)) * (100 as f32));
        return (time.clone(), percentage);
    }

    /// java: protected static int seaCreatureCount()
    pub fn sea_creature_count() -> i32 {
        return (SEA_CREATURES.get_mut().len() as i32);
    }

}

/// java: record LiveSeaCreature(SeaCreature seaCreature, Entity entity, Long spawnTime)
pub struct LiveSeaCreature {
    /// java: SeaCreature seaCreature
    pub sea_creature: SeaCreature,
    /// java: Entity entity (the entity's id)
    pub entity: i32,
    /// java: Long spawnTime
    pub spawn_time: i64,
}

impl LiveSeaCreature {
    /// java: the canonical record constructor
    pub fn new(mut sea_creature: SeaCreature, mut entity: i32, mut spawn_time: i64) -> Self {
        Self { sea_creature, entity, spawn_time }
    }

    /// java: the record accessor `seaCreature()`
    pub fn sea_creature(&self) -> SeaCreature {
        self.sea_creature
    }

    /// java: the record accessor `entity()`
    pub fn entity(&self) -> i32 {
        self.entity
    }

    /// java: the record accessor `spawnTime()`
    pub fn spawn_time(&self) -> i64 {
        self.spawn_time
    }

}
