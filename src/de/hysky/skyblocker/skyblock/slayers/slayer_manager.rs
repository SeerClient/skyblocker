//! Ported from de.hysky.skyblocker.skyblock.slayers.SlayerManager by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::configs::slayers_config::HighlightSlayerEntities;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::events::skyblock_events;
use crate::de::hysky::skyblocker::skyblock::slayers::boss::vampire::mania_indicator::ManiaIndicator;
use crate::de::hysky::skyblocker::skyblock::slayers::boss::vampire::stake_indicator::StakeIndicator;
use crate::de::hysky::skyblocker::skyblock::slayers::boss::vampire::twin_claws_indicator::TwinClawsIndicator;
use crate::de::hysky::skyblocker::skyblock::slayers::slayer_tier::SlayerTier;
use crate::de::hysky::skyblocker::skyblock::slayers::slayer_type;
use crate::de::hysky::skyblocker::skyblock::slayers::slayer_type::SlayerType;
use crate::de::hysky::skyblocker::utils::data::profiled_data::ProfiledData;
use crate::de::hysky::skyblocker::utils::mayor::mayor_utils::MayorUtils;
use crate::de::hysky::skyblocker::utils::render::title::title::Title;
use crate::de::hysky::skyblocker::utils::render::title::title_container::TitleContainer;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler::Scheduler;
use crate::de::hysky::skyblocker::utils::utils;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use crate::entry::skyblocker_mod;
use mcrust::com::mojang::serialization::codec;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::com::mojang::serialization::codecs::record_codec_builder;
use mcrust::java::util::uuid::Uuid;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::world::entity::decoration::armor_stand::ArmorStand;
use mcrust::net::minecraft::world::entity::entity::Entity;
use mcrust::net::minecraft::world::entity::entity_type::EntityType;
use mcrust::net::minecraft::world::entity::player::player::Player;
use mcrust::net::minecraft::world::interaction_hand::InteractionHand;
use mcrust::net::minecraft::world::interaction_result::InteractionResult;
use mcrust::net::minecraft::world::level::level::Level;
use mcrust::net::minecraft::world::phys::entity_hit_result::EntityHitResult;
use crate::de::hysky::skyblocker::utils::data::json_data::JsonData as _;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final Pattern SLAYER_PATTERN = Pattern.compile("\\b(Revenant Horror|Atoned Horror|Tarantula Broodfather|Conjoined Brood|Sven Packmaster|Voidgloom Seraph|Inferno Demonlord|Riftstalker Bl
pub static SLAYER_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("\\b(Revenant Horror|Atoned Horror|Tarantula Broodfather|Conjoined Brood|Sven Packmaster|Voidgloom Seraph|Inferno Demonlord|Riftstalker Bloodfiend|Bloodfiend)(?:\\s+(V|IV|III|II|I))?\\b"));
/// java: private static final Pattern XP_NEEDED_PATTERN = Pattern.compile("\\s*(Wolf|Zombie|Spider|Enderman|Blaze|Vampire) Slayer LVL ([0-9]) - (?:Next LVL in ([\\d,]+) XP!|LVL MAXED OUT!)\\s*");
pub static XP_NEEDED_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("\\s*(Wolf|Zombie|Spider|Enderman|Blaze|Vampire) Slayer LVL ([0-9]) - (?:Next LVL in ([\\d,]+) XP!|LVL MAXED OUT!)\\s*"));
/// java: private static final Pattern LVL_UP_PATTERN = Pattern.compile("\\s*LVL UP! ➜ (Wolf|Zombie|Spider|Enderman|Blaze|Vampire) Slayer LVL [1-9]\\s*");
pub static LVL_UP_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("\\s*LVL UP! ➜ (Wolf|Zombie|Spider|Enderman|Blaze|Vampire) Slayer LVL [1-9]\\s*"));
/// java: private static final Title BOSS_SPAWN = new Title(Component.translatable("skyblocker.slayer.bossSpawnAlert").withStyle(ChatFormatting.RED));
pub static BOSS_SPAWN: compat::MainThread<compat::Shared<Title>> = compat::MainThread::new(|| Title::new_1(component::translatable("skyblocker.slayer.bossSpawnAlert").with_style_format(ChatFormatting::Red)));
/// java: private static final Title MINIBOSS_SPAWN = new Title(Component.translatable("skyblocker.slayer.miniBossSpawnAlert").withStyle(ChatFormatting.RED));
pub static MINIBOSS_SPAWN: compat::MainThread<compat::Shared<Title>> = compat::MainThread::new(|| Title::new_1(component::translatable("skyblocker.slayer.miniBossSpawnAlert").with_style_format(ChatFormatting::Red)));
/// java: private static final Path FILE = SkyblockerMod.CONFIG_DIR.resolve("slayers_data.json");
pub static FILE: compat::MainThread<std::path::PathBuf> = compat::MainThread::new(|| skyblocker_mod::CONFIG_DIR.get_mut().join("slayers_data.json"));
/// java: private static final ProfiledData<Object2ObjectOpenHashMap<SlayerType, SlayerInfo>> SLAYERS_DATA = new ProfiledData<>(FILE, SlayerInfo.CODEC);
pub static SLAYERS_DATA: compat::MainThread<compat::Shared<ProfiledData<std::collections::HashMap<SlayerType, SlayerInfo>>>> = compat::MainThread::new(|| ProfiledData::new(FILE.get_mut().clone(), SLAYER_INFO_CODEC.get_mut().clone()));
// TODO(fabric2seer): Collections.newSetFromMap
//   java: private static final Set<Entity> ENTITIES_CACHE = Collections.newSetFromMap(new WeakHashMap<>());
/// java: private static final Set<Entity> ENTITIES_CACHE = Collections.newSetFromMap(new WeakHashMap<>());
pub static ENTITIES_CACHE: compat::MainThread<std::collections::HashSet<Box<dyn Entity>>> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `ENTITIES_CACHE` - see the TODO above"));
/// java: private static @Nullable SlayerQuest slayerQuest;
pub static SLAYER_QUEST: compat::MainThread<Option<compat::Shared<SlayerQuest>>> = compat::MainThread::new(|| None);
/// java: private static @Nullable BossFight bossFight;
pub static BOSS_FIGHT: compat::MainThread<Option<compat::Shared<BossFight>>> = compat::MainThread::new(|| None);
/// java: private static float slayerExpBuff = 1.0f;
pub static SLAYER_EXP_BUFF: LazyLock<Mutex<f32>> = LazyLock::new(|| Mutex::new(1.0f32));
/// java: private static final Codec<SlayerInfo> SLAYER_CODEC = RecordCodecBuilder.create(instance -> instance.group( Codec.INT.fieldOf("level").forGetter(SlayerInfo::level), Codec.INT.fieldOf("xpRemaining").fo
pub static SLAYER_INFO_SLAYER_CODEC: compat::MainThread<CodecRef<SlayerInfo>> = compat::MainThread::new(|| record_codec_builder::create(record_codec_builder::apply2(record_codec_builder::of(|o: &SlayerInfo| o.level(), codec::field_of(mcrust::com::mojang::serialization::codec::int_codec(), "level")), record_codec_builder::of(|o: &SlayerInfo| o.xp_remaining(), codec::field_of(mcrust::com::mojang::serialization::codec::int_codec(), "xpRemaining")), move |__a0, __a1| SlayerInfo::new(__a0.clone(), __a1.clone()))));
// TODO(fabric2seer): static call Function.identity(..)
//   java: private static final Codec<Object2ObjectOpenHashMap<SlayerType, SlayerInfo>> CODEC = Codec.unboundedMap(SlayerType.CODEC, SLAYER_CODEC)
//   java: 				.xmap(Object2ObjectOpenHashMap::new, Function.identity());
/// java: private static final Codec<Object2ObjectOpenHashMap<SlayerType, SlayerInfo>> CODEC = Codec.unboundedMap(SlayerType.CODEC, SLAYER_CODEC) .xmap(Object2ObjectOpenHashMap::new, Function.identity());
pub static SLAYER_INFO_CODEC: compat::MainThread<CodecRef<std::collections::HashMap<SlayerType, SlayerInfo>>> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `CODEC` - see the TODO above"));

/// java: public class SlayerManager
#[derive(Clone)]
pub struct SlayerManager {}

impl SlayerManager {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        SLAYERS_DATA.get_mut().lock().borrow_mut().load();
        // TODO(fabric2seer): ClientReceiveMessageEvents.ALLOW_GAME.register: no SeerClient equivalent yet
        //   java: ClientReceiveMessageEvents.ALLOW_GAME.register(SlayerManager::onChatMessage);
        skyblock_events::MAYOR_CHANGE.register(Arc::new(move || { SlayerManager::on_mayor_change(); }));
        skyblock_events::PROFILE_CHANGE.register(Arc::new(move |_, _| { *SLAYER_QUEST.get_mut() = None; }));
        client_play_connection_events::JOIN.register(Arc::new(|listener: &mut ClientPacketListener| {
            *BOSS_FIGHT.get_mut() = None;
        }));
        // TODO(fabric2seer): AttackEntityCallback.EVENT.register: no SeerClient equivalent yet
        //   java: AttackEntityCallback.EVENT.register(SlayerManager::onEntityAttack);
        ({ let __r = scheduler::INSTANCE.get_mut(); let __f = compat::dispatch(__r, |__t| __t.schedule_cyclic()); __f(__r, Arc::new(move || { TwinClawsIndicator::update_ice(); }), SkyblockerConfigManager::get().slayers.vampire_slayer.holy_ice_update_frequency) });
        ({ let __r = scheduler::INSTANCE.get_mut(); let __f = compat::dispatch(__r, |__t| __t.schedule_cyclic()); __f(__r, Arc::new(move || { ManiaIndicator::update_mania(); }), SkyblockerConfigManager::get().slayers.vampire_slayer.mania_update_frequency) });
        ({ let __r = scheduler::INSTANCE.get_mut(); let __f = compat::dispatch(__r, |__t| __t.schedule_cyclic()); __f(__r, Arc::new(move || { StakeIndicator::update_stake(); }), SkyblockerConfigManager::get().slayers.vampire_slayer.steak_stake_update_frequency) });
    }

    /// java: private static void onMayorChange()
    pub fn on_mayor_change() {
        compat::set_state(&SLAYER_EXP_BUFF, 1.0f32);
        if MayorUtils::get_active_perks().iter().any(|e| *e == "Work Smarter") {
            compat::set_state(&SLAYER_EXP_BUFF, (compat::get_state(&SLAYER_EXP_BUFF) * 1.5f32));
        }
        if MayorUtils::get_active_perks().iter().any(|e| *e == "Slayer XP Buff") {
            compat::set_state(&SLAYER_EXP_BUFF, (compat::get_state(&SLAYER_EXP_BUFF) * 1.25f32));
        }
    }

    /// java: @SuppressWarnings("SameReturnValue") private static boolean onChatMessage(Component text, boolean overlay)
    pub fn on_chat_message(text: &MutableComponent, mut overlay: bool) -> bool {
        if overlay || !(Utils::is_on_skyblock()) {
            return true;
        }
        let mut message: String = text.get_string();
        // TODO(fabric2seer): a constant `case` among pattern cases
        //   java: switch (message.stripLeading()) {
        //   java: 			case "Your Slayer Quest has been cancelled!", "SLAYER QUEST FAILED!" -> {
        //   java: 				slayerQuest = null;
        //   java: 				CallMaddox.onSlayerFailed();
        //   java: 				return true;
        //   java: 			}
        //   java: 			case "SLAYER QUEST STARTED!" -> {
        //   java: 				return true;
        //   java: 			}
        //   java: 			case "NICE! SLAYER BOSS SLAIN!", "SLAYER QUEST COMPLETE!" -> {
        //   java: 				if (slayerQuest != null) {
        //   java: 					SlayerTimer.sendMessage();
        //   java: 					CallMaddox.onBossKilled();
        //   java: 				}
        //   java: 				return true;
        //   java: 			}
        //   java: 			case "YOU COCOONED YOUR SLAYER BOSS" -> {
        //   java: 				if (slayerQuest != null) SlayerTimer.sendMessage();
        //   java: 				return true;
        //   java: 			}
        //   java: 			case String s when s.startsWith("SLAYER MINI-BOSS") -> {
        //   java: 				if (SkyblockerConfigManager.get().slayers.miniBossSpawnAlert && !SkyblockerConfigManager.get().slayers.alertOtherMinibosses) {
        //   java: 					TitleContainer.addTitleAndPlaySound(MINIBOSS_SPAWN, 20);
        //   java: 				}
        //   java: 			}
        //   java: 			default -> {}
        //   java: 		}
        if SLAYER_QUEST.get_mut().is_some() {
            let mut matcher_next_lvl: compat::regex::Matcher = XP_NEEDED_PATTERN.matcher(&message);
            if matcher_next_lvl.matches() {
                if message.contains("LVL MAXED OUT") {
                    let mut level: i32 = (if message.contains("Vampire") { 5 } else { 9 });
                    SlayerQuest::update(SLAYER_QUEST.get_mut().as_ref().unwrap(), level, -1, true);
                } else {
                    let mut xp_remaining_str: Option<String> = matcher_next_lvl.group(3);
                    if let Some(xp_remaining_str) = &xp_remaining_str {
                        let mut level: i32 = matcher_next_lvl.group(2).unwrap().trim().parse::<i32>().expect("NumberFormatException");
                        let mut xp_remaining: i32 = (compat::trim(&(xp_remaining_str.replace(",", "")))).trim().parse::<i32>().expect("NumberFormatException");
                        SlayerQuest::update(SLAYER_QUEST.get_mut().as_ref().unwrap(), level, xp_remaining, true);
                    }
                }
            } else if LVL_UP_PATTERN.matcher(&message).matches() {
                let mut level: i32 = (compat::regex::replace_all(&message, "(\\d+).+", "$1")).trim().parse::<i32>().expect("NumberFormatException");
                SlayerQuest::update(SLAYER_QUEST.get_mut().as_ref().unwrap(), level, -1, true);
            }
        }
        return true;
    }

    /// java: @SuppressWarnings("SameReturnValue") private static InteractionResult onEntityAttack(Player player, Level level, InteractionHand interactionHand, Entity entity, @Nullable EntityHitResult entityHitResu
    pub fn on_entity_attack(player: &dyn Player, level: &Level, interaction_hand: &InteractionHand, mut entity: Box<dyn Entity>, mut entity_hit_result: Option<EntityHitResult>) -> InteractionResult {
        if ENTITIES_CACHE.get_mut().contains(&entity) {
            return InteractionResult::Pass;
        }
        ENTITIES_CACHE.get_mut().insert(entity.clone());
        if !SlayerType::values().iter().copied().any(|slayer| std::ptr::addr_eq(std::sync::Arc::as_ptr(&slayer.fields().mob_type), std::sync::Arc::as_ptr(&entity.get_type().unwrap()))) {
            return InteractionResult::Pass;
        }
        let mut boss: Option<compat::Shared<BossFight>> = SlayerManager::find_boss(entity.clone());
        if let Some(boss) = &boss {
            if !(SlayerManager::is_slayer_armor_stand_alive()) || compat::read(boss, |__t| __t.player_boss) && BOSS_FIGHT.get_mut().is_some() && !std::ptr::eq(&compat::read(BOSS_FIGHT.get_mut().as_ref().unwrap(), |__t| __t.armor_stand.clone()), &compat::read(boss, |__t| __t.armor_stand.clone())) {
                *BOSS_FIGHT.get_mut() = Some(boss.clone());
                ENTITIES_CACHE.get_mut().clear();
                ENTITIES_CACHE.get_mut().insert(entity.clone());
            }
        }
        return InteractionResult::Pass;
    }

    /// java: public static void checkSlayerQuest()
    pub fn check_slayer_quest() {
        let mut active: bool = false;
        let mut boss_spawned: bool = false;
        for line in compat::get_state(&utils::STRING_SCOREBOARD) {
            let mut matcher: compat::regex::Matcher = SLAYER_PATTERN.matcher(&line);
            if matcher.find() {
                let mut boss_name: String = matcher.group(1).unwrap();
                let mut boss_tier: String = matcher.group(2).unwrap();
                if SLAYER_QUEST.get_mut().is_none() || !(boss_name == SLAYER_QUEST.get_mut().as_ref().unwrap().lock().borrow().slayer_type.fields().boss_name) || !(boss_tier == SLAYER_QUEST.get_mut().as_ref().unwrap().lock().borrow().slayer_tier.name()) {
                    let Some(mut slayer_type) = SlayerType::from_boss_name(&boss_name) else {
                        return;
                    };
                    // TODO(fabric2seer): no overload of `valueOf` takes 1 argument(s)
                    //   java: slayerQuest = new SlayerQuest(slayerType, SlayerTier.valueOf(bossTier));
                }
                active = true;
            } else if line == "Slay the boss!" {
                boss_spawned = true;
            }
        }
        if SLAYER_QUEST.get_mut().is_some() {
            { let __v = active; compat::write(SLAYER_QUEST.get_mut().as_ref().unwrap(), |__t| __t.active = __v); }
            if active && boss_spawned {
                SlayerQuest::on_boss_spawn(SLAYER_QUEST.get_mut().as_ref().unwrap());
            }
            { let __v = boss_spawned; compat::write(SLAYER_QUEST.get_mut().as_ref().unwrap(), |__t| __t.boss_spawned = __v); }
        }
    }

    /// java: public static void checkSlayerBoss(ArmorStand armorStand)
    pub fn check_slayer_boss(armor_stand: &ArmorStand) {
        if !armor_stand.base.base.has_custom_name() || !(SlayerManager::is_in_slayer_quest()) || SlayerManager::is_slayer_armor_stand_alive() {
            return;
        }
        let mut entity_name: String = compat::component_of(armor_stand.get_name()).get_string();
        // TODO(fabric2seer): the mod keeps the game's `ArmorStand` (Java holds a reference to the live object); the port's `ArmorStand` is not a value to copy and the port lends the mod no handle to it
        // rust: let mut boss: Option<compat::Shared<BossFight>> = SlayerManager::find_boss(armor_stand.clone());
        let mut boss: compat::Shared<BossFight> = todo!("fabric2seer: the initialiser of `boss` (TODO above)");
        if true /* java: boss != null: never null here */ {
            *BOSS_FIGHT.get_mut() = Some(boss.clone());
            SlayerQuest::on_boss_spawn(SLAYER_QUEST.get_mut().as_ref().unwrap());
            if !(compat::read(SLAYER_QUEST.get_mut().as_ref().unwrap(), |__t| __t.sent_boss_alert)) {
                BossFight::alert();
                { let __v = true; compat::write(SLAYER_QUEST.get_mut().as_ref().unwrap(), |__t| __t.sent_boss_alert = __v); }
            }
            return;
        }
        debug_assert!(mcrust::mcrust::modding::game::with_player(|_| ()).is_some());
        if mcrust::mcrust::modding::game::with_player(|player| armor_stand.base.base.closer_than(player, (15) as f64)).unwrap_or(false) && SLAYER_QUEST.get_mut().as_ref().unwrap().lock().borrow().slayer_type.is_miniboss(&entity_name, compat::read(SLAYER_QUEST.get_mut().as_ref().unwrap(), |__t| __t.slayer_tier)) {
            // TODO(fabric2seer): the mod keeps the game's `ArmorStand` (Java holds a reference to the live object); the port's `ArmorStand` is not a value to copy and the port lends the mod no handle to it
            // rust: SlayerQuest::on_miniboss_spawn(SLAYER_QUEST.get_mut().as_ref().unwrap(), armor_stand.clone());
        }
    }

    /// java: private static @Nullable BossFight findBoss(Entity entity)
    pub fn find_boss(mut entity: Box<dyn Entity>) -> Option<compat::Shared<BossFight>> {
        let mut username: String = compat::minecraft().get_user().get_name().to_string();
        let mut boss_stand: Option<ArmorStand> = None;
        let mut boss: Option<Box<dyn Entity>> = None;
        let mut slayer_type: Option<SlayerType> = None;
        let mut slayer_tier: Option<SlayerTier> = None;
        let mut player_boss: bool = false;
        if let Some(armor_stand) = entity.as_any().unwrap_or(&() as &dyn std::any::Any).downcast_ref::<ArmorStand>() {
            let mut entity_name: String = compat::component_of(armor_stand.get_name()).get_string();
            let mut entity_matcher: compat::regex::Matcher = SLAYER_PATTERN.matcher(&entity_name);
            let mut is_player_stand: bool = entity_name.contains(&username);
            let mut is_boss_stand: bool = (entity_matcher.find() && SlayerManager::is_valid_boss_armor_stand(armor_stand));
            if !is_boss_stand && !is_player_stand {
                return None;
            }
            for stand in SlayerManager::get_entity_armor_stands(&entity, 1.5f32) {
                let mut stand_name: String = compat::component_of(stand.get_name()).get_string();
                if is_boss_stand && stand_name.contains(&username) {
                    let mut boss_name: String = entity_matcher.group(1).unwrap();
                    slayer_type = SlayerType::from_boss_name(&boss_name);
                    slayer_tier = Some(SlayerTier::value_of(entity_matcher.group(2), &boss_name));
                    // TODO(fabric2seer): the mod keeps the game's `ArmorStand` (Java holds a reference to the live object); the port's `ArmorStand` is not a value to copy and the port lends the mod no handle to it
                    // rust: boss_stand = Some(armor_stand.clone());
                    player_boss = true;
                    break;
                }
                if is_player_stand {
                    let mut matcher: compat::regex::Matcher = SLAYER_PATTERN.matcher(&stand_name);
                    if matcher.find() && SlayerManager::is_valid_boss_armor_stand(&stand) {
                        let mut boss_name: String = matcher.group(1).unwrap();
                        slayer_type = SlayerType::from_boss_name(&boss_name);
                        slayer_tier = Some(SlayerTier::value_of(matcher.group(2), &boss_name));
                        // TODO(fabric2seer): the mod keeps the game's `ArmorStand` (Java holds a reference to the live object); the port's `ArmorStand` is not a value to copy and the port lends the mod no handle to it
                        // rust: boss_stand = Some(stand.clone());
                        player_boss = true;
                        break;
                    }
                }
            }
            boss = (if slayer_type.is_some() { SlayerManager::find_closest_mob_entity(&slayer_type.as_ref().unwrap().fields().mob_type, armor_stand) } else { None });
        } else {
            boss = Some(entity.clone());
            for stand in SlayerManager::get_entity_armor_stands(&entity, 1.5f32) {
                let mut stand_name: String = compat::component_of(stand.get_name()).get_string();
                let mut matcher: compat::regex::Matcher = SLAYER_PATTERN.matcher(&stand_name);
                if matcher.find() && SlayerManager::is_valid_boss_armor_stand(&stand) {
                    // TODO(fabric2seer): the mod keeps the game's `ArmorStand` (Java holds a reference to the live object); the port's `ArmorStand` is not a value to copy and the port lends the mod no handle to it
                    // rust: boss_stand = Some(stand.clone());
                    let mut boss_name: String = matcher.group(1).unwrap();
                    slayer_type = SlayerType::from_boss_name(&boss_name);
                    slayer_tier = Some(SlayerTier::value_of(matcher.group(2), &boss_name));
                } else if stand_name.contains(&username) {
                    player_boss = true;
                }
            }
        }
        return (if boss_stand.is_some() && boss.is_some() && slayer_type.is_some() { Some(BossFight::new(boss_stand.clone().unwrap(), boss.clone().unwrap(), player_boss, slayer_type.clone().unwrap(), slayer_tier.clone().unwrap())) } else { None });
    }

    /// java: public static List<ArmorStand> getEntityArmorStands(Entity entity, float expandY)
    pub fn get_entity_armor_stands(entity: &dyn Entity, mut expand_y: f32) -> Vec<ArmorStand> {
        // TODO(fabric2seer): `Entity.level` has no Rust method in the port (looked for `level` in Entity and its supertypes)
        // TODO(fabric2seer): `Level.getEntities` has no Rust method in the port (looked for `get_entities` in Level and its supertypes)
        // TODO(fabric2seer): the mod keeps the game's `ArmorStand` (Java holds a reference to the live object); the port's `ArmorStand` is not a value to copy and the port lends the mod no handle to it
        // rust: return entity.level().get_entities(entity, &entity.get_bounding_box().inflate3((0.1f32) as f64, (expand_y) as f64, (0.1f32) as f64), &Arc::new(move |x| (x.as_any().unwrap_or(&() as &dyn std::any::Any).is::<ArmorStand>() && x.has_custom_name()))).into_iter().map(|e| e.as_any().downcast_ref::<ArmorStand>().unwrap().clone()).collect::<Vec<_>>();
        return todo!("fabric2seer: see the TODO above");
    }

    /// java: private static @Nullable <T extends Entity> T findClosestMobEntity(EntityType<T> entityType, ArmorStand armorStand)
    pub fn find_closest_mob_entity<T: 'static>(entity_type: &Arc<EntityType>, armor_stand: &ArmorStand) -> Option<T> {
        // TODO(fabric2seer): `.isAlive(..)` on a T
        //   java: List<T> mobEntities = armorStand.level().getEntities(entityType, armorStand.getBoundingBox().inflate(0, 1.5f, 0), Entity::isAlive);
        let mut mob_entities: Vec<T> = todo!("fabric2seer: the initialiser of `mobEntities` (TODO above)");
        mob_entities.sort_by(|a, b| { let key = |__a0| armor_stand.base.base.distance_to_sqr_entity(__a0); key(a).total_cmp(&key(b)) });
        // TODO(fabric2seer): field `tickCount` of a T
        //   java: return switch (mobEntities.size()) {
        //   java: 			case 0 -> null;
        //   java: 			case 1 -> mobEntities.getFirst();
        //   java: 			default -> mobEntities.stream()
        //   java: 					.min(Comparator.comparingInt(entity -> Math.abs(entity.tickCount - armorStand.tickCount)))
        //   java: 					.get();
        //   java: 		};
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: public static boolean shouldGlow(Entity entity, SlayersConfig.HighlightSlayerEntities highlightType)
    pub fn should_glow(entity: &dyn Entity, mut highlight_type: HighlightSlayerEntities) -> bool {
        if SkyblockerConfigManager::get().slayers.highlight_minis == highlight_type && SlayerManager::is_in_slayer_quest() && SLAYER_QUEST.get_mut().as_ref().unwrap().lock().borrow().minibosses.iter().any(|e| *e == entity.clone()) {
            return true;
        }
        return (SkyblockerConfigManager::get().slayers.highlight_bosses == highlight_type && SlayerManager::is_selected_boss(&compat::uuid_of(entity.get_uuid())));
    }

    /// java: public static @Nullable SlayerQuest getSlayerQuest()
    pub fn get_slayer_quest() -> Option<compat::Shared<SlayerQuest>> {
        return SLAYER_QUEST.get_mut().clone();
    }

    /// java: public static @Nullable BossFight getBossFight()
    pub fn get_boss_fight() -> Option<compat::Shared<BossFight>> {
        return BOSS_FIGHT.get_mut().clone();
    }

    /// java: public static boolean isInSlayerQuest()
    pub fn is_in_slayer_quest() -> bool {
        return (SLAYER_QUEST.get_mut().is_some() && compat::read(SLAYER_QUEST.get_mut().as_ref().unwrap(), |__t| __t.active));
    }

    /// java: public static boolean isFightingSlayer()
    pub fn is_fighting_slayer() -> bool {
        return (BOSS_FIGHT.get_mut().is_some() && BOSS_FIGHT.get_mut().as_ref().unwrap().lock().borrow().boss.is_alive());
    }

    /// java: public static boolean isFightingOwnedSlayer()
    pub fn is_fighting_owned_slayer() -> bool {
        return (SlayerManager::is_fighting_slayer() && compat::read(BOSS_FIGHT.get_mut().as_ref().unwrap(), |__t| __t.player_boss));
    }

    /// java: public static boolean isFightingSlayerType(SlayerType slayerType)
    pub fn is_fighting_slayer_type(mut slayer_type: SlayerType) -> bool {
        return (BOSS_FIGHT.get_mut().is_some() && compat::read(BOSS_FIGHT.get_mut().as_ref().unwrap(), |__t| __t.slayer_type) == slayer_type);
    }

    /// java: public static boolean isSelectedBoss(UUID uuid)
    pub fn is_selected_boss(uuid: &Uuid) -> bool {
        return (BOSS_FIGHT.get_mut().is_some() && compat::uuid_of(BOSS_FIGHT.get_mut().as_ref().unwrap().lock().borrow().boss.get_uuid()) == uuid);
    }

    /// java: public static @Nullable ArmorStand getSlayerArmorStand()
    pub fn get_slayer_armor_stand() -> Option<ArmorStand> {
        return (if SlayerManager::is_fighting_slayer() { Some(compat::read(BOSS_FIGHT.get_mut().as_ref().unwrap(), |__t| __t.armor_stand.clone())) } else { None });
    }

    /// java: public static boolean isSlayerArmorStandAlive()
    pub fn is_slayer_armor_stand_alive() -> bool {
        return (SlayerManager::is_fighting_owned_slayer() && SlayerManager::is_valid_boss_armor_stand(&compat::read(BOSS_FIGHT.get_mut().as_ref().unwrap(), |__t| __t.armor_stand.clone())));
    }

    /// java: public static boolean isValidBossArmorStand(ArmorStand armorStandEntity)
    pub fn is_valid_boss_armor_stand(armor_stand_entity: &ArmorStand) -> bool {
        return (armor_stand_entity.base.is_alive() && !(compat::component_of(armor_stand_entity.get_name()).get_string().ends_with(" 0❤")));
    }

}

/// java: public static class BossFight
pub struct BossFight {
    /// java: public final ArmorStand armorStand;
    pub armor_stand: ArmorStand,
    /// java: public final Entity boss;
    pub boss: Box<dyn Entity>,
    /// java: public final boolean playerBoss;
    pub player_boss: bool,
    /// java: public final SlayerType slayerType;
    pub slayer_type: SlayerType,
    /// java: public final SlayerTier slayerTier;
    pub slayer_tier: SlayerTier,
}

impl BossFight {
    /// java: private BossFight(ArmorStand armorStand, Entity boss, boolean playerBoss, SlayerType slayerType, SlayerTier slayerTier)
    pub fn new(mut armor_stand: ArmorStand, mut boss: Box<dyn Entity>, mut player_boss: bool, mut slayer_type: SlayerType, mut slayer_tier: SlayerTier) -> compat::Shared<Self> {
        // TODO(fabric2seer): the mod keeps the game's `ArmorStand` (Java holds a reference to the live object); the port's `ArmorStand` is not a value to copy and the port lends the mod no handle to it
        //   java: this.armorStand = armorStand;
        let __init_boss = boss.clone();
        let __init_player_boss = player_boss;
        let __init_slayer_type = slayer_type;
        let __init_slayer_tier = slayer_tier;
        let __this = compat::shared(Self {
            armor_stand: todo!("fabric2seer: the field value (TODO above)"),
            boss: __init_boss,
            player_boss: __init_player_boss,
            slayer_type: __init_slayer_type,
            slayer_tier: __init_slayer_tier,
        });
        __this
    }

    /// java: private static void alert()
    pub fn alert() {
        if SkyblockerConfigManager::get().slayers.boss_spawn_alert {
            TitleContainer::add_title_and_play_sound_2(BOSS_SPAWN.get_mut(), 20);
        }
    }

    /// java: public static void remove()
    pub fn remove() {
        *BOSS_FIGHT.get_mut() = None;
        ENTITIES_CACHE.get_mut().clear();
    }

}

/// java: public static class SlayerQuest
pub struct SlayerQuest {
    /// java: public final SlayerType slayerType;
    pub slayer_type: SlayerType,
    /// java: public final SlayerTier slayerTier;
    pub slayer_tier: SlayerTier,
    /// java: public final List<ArmorStand> minibossesArmorStand = new ArrayList<>();
    pub minibosses_armor_stand: Vec<ArmorStand>,
    /// java: public final List<Entity> minibosses = new ArrayList<>();
    pub minibosses: Vec<Box<dyn Entity>>,
    /// java: public boolean active = true;
    pub active: bool,
    /// java: public int level;
    pub level: i32,
    /// java: public int xpRemaining;
    pub xp_remaining: i32,
    /// java: public int bossesNeeded;
    pub bosses_needed: i32,
    /// java: public @Nullable Instant bossSpawnTime;
    pub boss_spawn_time: Option<std::time::SystemTime>,
    /// java: public @Nullable Instant bossDeathTime;
    pub boss_death_time: Option<std::time::SystemTime>,
    /// java: public boolean bossSpawned;
    pub boss_spawned: bool,
    /// java: public boolean sentBossAlert;
    pub sent_boss_alert: bool,
}

impl SlayerQuest {
    /// java: private SlayerQuest(SlayerType slayerType, SlayerTier slayerTier)
    pub fn new(mut slayer_type: SlayerType, mut slayer_tier: SlayerTier) -> compat::Shared<Self> {
        let __init_slayer_type = slayer_type;
        let __init_slayer_tier = slayer_tier;
        let mut slayers_data = ProfiledData::get(SLAYERS_DATA.get_mut());
        let __this = compat::shared(Self {
            slayer_type: __init_slayer_type,
            slayer_tier: __init_slayer_tier,
            minibosses_armor_stand: Vec::new(),
            minibosses: Vec::new(),
            active: true,
            level: 0,
            xp_remaining: 0,
            bosses_needed: 0,
            boss_spawn_time: None,
            boss_death_time: None,
            boss_spawned: false,
            sent_boss_alert: false,
        });
        // TODO(fabric2seer): the condition: `.containsKey(..)` on a T
        //   java: slayersData != null && slayersData.containsKey(slayerType)
        if todo!("fabric2seer: the condition (TODO above)") {
            // TODO(fabric2seer): `.get(..)` on a T
            //   java: SlayerInfo slayerInfo = slayersData.get(slayerType);
            let mut slayer_info: SlayerInfo = todo!("fabric2seer: the initialiser of `slayerInfo` (TODO above)");
            SlayerQuest::update(&__this, slayer_info.level, slayer_info.xp_remaining, false);
        } else {
            SlayerQuest::update(&__this, -1, -1, false);
        }
        __this
    }

    /// java: private void save()
    pub fn save(__this: &compat::Shared<Self>) {
        let mut slayers = ProfiledData::compute_if_absent(SLAYERS_DATA.get_mut(), Arc::new(move || std::collections::HashMap::new()));
        // TODO(fabric2seer): `.put(..)` on a T
        //   java: slayers.put(slayerType, new SlayerInfo(level, xpRemaining));
        SLAYERS_DATA.get_mut().lock().borrow().save();
    }

    /// java: private void update(int level, int xpRemaining, boolean saveCache)
    pub fn update(__this: &compat::Shared<Self>, mut level: i32, mut xp_remaining: i32, mut save_cache: bool) {
        { let __v = false; compat::write(__this, |__t| __t.sent_boss_alert = __v); }
        { let __v = level; compat::write(__this, |__t| __t.level = __v); }
        { let __v = (if xp_remaining <= 0 && level != -1 && level < __this.lock().borrow().slayer_type.fields().max_level { (__this.lock().borrow().slayer_type.fields().level_milestones[(level.wrapping_add(1)) as usize]).wrapping_sub(__this.lock().borrow().slayer_type.fields().level_milestones[(level) as usize]) } else { xp_remaining }); compat::write(__this, |__t| __t.xp_remaining = __v); }
        if save_cache {
            SlayerQuest::save(__this);
        }
        if compat::read(__this, |__t| __t.xp_remaining) <= 0 {
            { let __v = -1; compat::write(__this, |__t| __t.bosses_needed = __v); }
        } else {
            let mut xp_per_tier: i32 = ((((__this.lock().borrow().slayer_type.fields().xp_per_tier[(__this.lock().borrow().slayer_tier.ordinal()) as usize] as f32) * compat::get_state(&SLAYER_EXP_BUFF))) as i32);
            { let __v = (((((compat::read(__this, |__t| __t.xp_remaining)) as f64) / (xp_per_tier as f64)).ceil()) as i32); compat::write(__this, |__t| __t.bosses_needed = __v); }
        }
    }

    /// java: private void onBossSpawn()
    pub fn on_boss_spawn(__this: &compat::Shared<Self>) {
        if !(compat::read(__this, |__t| __t.boss_spawned)) {
            { let __v = Some(std::time::SystemTime::now()); compat::write(__this, |__t| __t.boss_spawn_time = __v); }
            { let __v = None; compat::write(__this, |__t| __t.boss_death_time = __v); }
            { let __v = true; compat::write(__this, |__t| __t.boss_spawned = __v); }
        }
    }

    /// java: private void onMinibossSpawn(ArmorStand armorStand)
    pub fn on_miniboss_spawn(__this: &compat::Shared<Self>, mut armor_stand: ArmorStand) {
        // TODO(fabric2seer): the mod keeps the game's `ArmorStand` (Java holds a reference to the live object); the port's `ArmorStand` is not a value to copy and the port lends the mod no handle to it
        if __this.lock().borrow().minibosses_armor_stand.iter().any(|e| *e == armor_stand.clone()) {
            return;
        }
        // TODO(fabric2seer): the mod keeps the game's `ArmorStand` (Java holds a reference to the live object); the port's `ArmorStand` is not a value to copy and the port lends the mod no handle to it
        // rust: let __h0 = armor_stand.clone();
        // rust: __this.lock().borrow_mut().minibosses_armor_stand.push(__h0);
        let mut miniboss_type: Arc<EntityType> = __this.lock().borrow().slayer_type.get_miniboss_type(&compat::component_of(armor_stand.get_name()).get_string());
        let Some(mut miniboss) = SlayerManager::find_closest_mob_entity(&miniboss_type, &armor_stand) else {
            return;
        };
        let __h0 = miniboss.clone();
        __this.lock().borrow_mut().minibosses.push(__h0);
        let slayers_config = &SkyblockerConfigManager::get().slayers;
        if slayers_config.mini_boss_spawn_alert && slayers_config.alert_other_minibosses {
            let mut armor_stand_name: Option<MutableComponent> = Some(armor_stand.base.base.get_custom_name().unwrap().as_any().downcast_ref::<MutableComponent>().unwrap().clone());
            let mut title: compat::Shared<Title> = (if slayers_config.show_mini_boss_name_in_alert && armor_stand_name.is_some() { Title::new_1(armor_stand_name.clone().unwrap()) } else { MINIBOSS_SPAWN.get_mut().clone() });
            TitleContainer::add_title_and_play_sound_2(&title, 20);
        }
    }

}

/// java: public record SlayerInfo(int level, int xpRemaining)
#[derive(Clone, Debug, PartialEq)]
pub struct SlayerInfo {
    /// java: int level
    pub level: i32,
    /// java: int xpRemaining
    pub xp_remaining: i32,
}

impl SlayerInfo {
    /// java: the canonical record constructor
    pub fn new(mut level: i32, mut xp_remaining: i32) -> Self {
        Self { level, xp_remaining }
    }

    /// java: the record accessor `level()`
    pub fn level(&self) -> i32 {
        self.level
    }

    /// java: the record accessor `xpRemaining()`
    pub fn xp_remaining(&self) -> i32 {
        self.xp_remaining
    }

}
