//! Ported from de.hysky.skyblocker.skyblock.StatusBarTracker by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::fancybars::fancy_status_bars;
use crate::de::hysky::skyblocker::skyblock::fancybars::fancy_status_bars::FancyStatusBars;
use crate::de::hysky::skyblocker::skyblock::fancybars::status_bar_type::StatusBarType;
use crate::de::hysky::skyblocker::skyblock::item::pet_info::PetInfo;
use crate::de::hysky::skyblocker::skyblock::pet_cache::PetCache;
use crate::de::hysky::skyblocker::utils::item_utils::ItemUtils;
use crate::de::hysky::skyblocker::utils::location::Location;
use crate::de::hysky::skyblocker::utils::regex_utils::RegexUtils;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler::Scheduler;
use crate::de::hysky::skyblocker::utils::sky_block_icons;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::net::minecraft::chat_formatting;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::world::entity::player::player::Player;
use mcrust::net::minecraft::world::interaction_hand::InteractionHand;
use mcrust::net::minecraft::world::interaction_result::InteractionResult;
use mcrust::net::minecraft::world::level::level::Level;
use crate::de::hysky::skyblocker::injected::skyblocker_stack::SkyblockerStack as _;
use mcrust::net::minecraft::network::chat::component::Component as _;
use mcrust::net::minecraft::world::entity::living_entity::LivingEntity as _;

/// java: private static final Pattern STATUS_PATTERN = Pattern.compile("(?<status>.+?)(?: {2,}|$)");
pub static STATUS_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("(?<status>.+?)(?: {2,}|$)"));
/// java: private static final Pattern RIFT_TIME_STATUS = Pattern.compile(String.format("(?:[\\d,]+m)?[\\d,]+s[ф%s] Left", SkyBlockIcons.RIFT_TIME));
pub static RIFT_TIME_STATUS: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile(&format!("(?:[\\d,]+m)?[\\d,]+s[ф{}] Left", sky_block_icons::RIFT_TIME)));
/// java: private static final Pattern HEALTH_STATUS = Pattern.compile(String.format("(?<health>[\\d,]+)/(?<max>[\\d,]+)[❤%s](?<healing>\\+([\\d,]+)[▁-▆])?", SkyBlockIcons.HEALTH));
pub static HEALTH_STATUS: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile(&format!("(?<health>[\\d,]+)/(?<max>[\\d,]+)[❤{}](?<healing>\\+([\\d,]+)[▁-▆])?", sky_block_icons::HEALTH)));
/// java: private static final Pattern VITALITY_STATUS = Pattern.compile(String.format("(?<vitality>[\\d,]+)/(?<max>[\\d,]+)%s", SkyBlockIcons.VITALITY));
pub static VITALITY_STATUS: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile(&format!("(?<vitality>[\\d,]+)/(?<max>[\\d,]+){}", sky_block_icons::VITALITY)));
/// java: private static final Pattern HEALING = Pattern.compile(String.format("(?:§[\\da-z])*[❤%s]", SkyBlockIcons.HEALTH));
pub static HEALING: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile(&format!("(?:§[\\da-z])*[❤{}]", sky_block_icons::HEALTH)));
/// java: private static final Pattern DEFENSE_STATUS = Pattern.compile(String.format("(?<defense>[\\d,]+)[❈%s]( Defense)?", SkyBlockIcons.DEFENSE));
pub static DEFENSE_STATUS: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile(&format!("(?<defense>[\\d,]+)[❈{}]( Defense)?", sky_block_icons::DEFENSE)));
/// java: private static final Pattern MANA_USE = Pattern.compile("-([\\d,]+) Mana \\(.*?\\)");
pub static MANA_USE: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("-([\\d,]+) Mana \\(.*?\\)"));
/// java: private static final Pattern MANA_STATUS = Pattern.compile(String.format("(?<mana>[\\d,]+)/(?<max>[\\d,]+)[✎%s] ?(?:Mana|(?<overflow>[\\d,]+)[ʬ%s])?", SkyBlockIcons.MANA, SkyBlockIcons.OVERFLOW_MANA))
pub static MANA_STATUS: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile(&format!("(?<mana>[\\d,]+)/(?<max>[\\d,]+)[✎{}] ?(?:Mana|(?<overflow>[\\d,]+)[ʬ{}])?", *sky_block_icons::MANA, sky_block_icons::OVERFLOW_MANA)));
/// java: private static Component lastMessage = Component.empty();
pub static LAST_MESSAGE: compat::MainThread<MutableComponent> = compat::MainThread::new(|| component::empty());
/// java: private static Component lastReturn = Component.empty();
pub static LAST_RETURN: compat::MainThread<MutableComponent> = compat::MainThread::new(|| component::empty());
/// java: private static long lastMessageTime = 0;
pub static LAST_MESSAGE_TIME: LazyLock<Mutex<i64>> = LazyLock::new(|| Mutex::new(((0) as i64) as i64));
/// java: private static Resource health = new Resource(100, 100, 0);
pub static HEALTH: LazyLock<Mutex<Resource>> = LazyLock::new(|| Mutex::new(Resource::new(100, 100, 0)));
/// java: private static final EstimatedResource vitality = new EstimatedResource(new Resource(100, 100, 0));
pub static VITALITY: LazyLock<Mutex<compat::Shared<EstimatedResource>>> = LazyLock::new(|| Mutex::new(EstimatedResource::new(Resource::new(100, 100, 0))));
/// java: private static final EstimatedResource mana = new EstimatedResource(new Resource(100, 100, 0));
pub static MANA: LazyLock<Mutex<compat::Shared<EstimatedResource>>> = LazyLock::new(|| Mutex::new(EstimatedResource::new(Resource::new(100, 100, 0))));
/// java: private static Resource speed = new Resource(100, 400, 0);
pub static SPEED: LazyLock<Mutex<Resource>> = LazyLock::new(|| Mutex::new(Resource::new(100, 400, 0)));
/// java: private static Resource air = new Resource(100, 300, 0);
pub static AIR: LazyLock<Mutex<Resource>> = LazyLock::new(|| Mutex::new(Resource::new(100, 300, 0)));
/// java: private static int defense = 0;
pub static DEFENSE: LazyLock<Mutex<i32>> = LazyLock::new(|| Mutex::new(0));
/// java: private static int absorption = 0;
pub static ABSORPTION: LazyLock<Mutex<i32>> = LazyLock::new(|| Mutex::new(0));
/// java: private static int ticks;
pub static TICKS: LazyLock<Mutex<i32>> = LazyLock::new(|| Mutex::new(0));

/// java: public class StatusBarTracker
#[derive(Clone)]
pub struct StatusBarTracker {}

impl StatusBarTracker {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        // java: ClientReceiveMessageEvents.ALLOW_GAME.register(StatusBarTracker::allowOverlayMessage);
        compat::register_allow_game(|text, overlay| StatusBarTracker::allow_overlay_message(text, overlay));
        // java: ClientReceiveMessageEvents.MODIFY_GAME.register(StatusBarTracker::onOverlayMessage);
        compat::register_modify_game(|text, overlay| StatusBarTracker::on_overlay_message(text.clone(), overlay));
        // java: UseItemCallback.EVENT.register(StatusBarTracker::interactItem);
        mcrust::mcrust::modding::events::use_item_callback::register(|player, world, hand| StatusBarTracker::interact_item(player, world, &hand));
        ({ let __r = scheduler::INSTANCE.get_mut(); let __f = compat::dispatch(__r, |__t| __t.schedule_cyclic()); __f(__r, Arc::new(move || { StatusBarTracker::tick(); }), 1) });
    }

    /// java: public static Resource getHealth()
    pub fn get_health() -> Resource {
        return compat::get_state(&HEALTH);
    }

    /// java: public static EstimatedResource getVitality()
    pub fn get_vitality() -> compat::Shared<EstimatedResource> {
        return compat::get_state(&VITALITY);
    }

    /// java: public static EstimatedResource getMana()
    pub fn get_mana() -> compat::Shared<EstimatedResource> {
        return compat::get_state(&MANA);
    }

    /// java: public static int getDefense()
    pub fn get_defense() -> i32 {
        return compat::get_state(&DEFENSE);
    }

    /// java: public static Resource getSpeed()
    pub fn get_speed() -> Resource {
        return compat::get_state(&SPEED);
    }

    /// java: public static Resource getAir()
    pub fn get_air() -> Resource {
        return compat::get_state(&AIR);
    }

    /// java: private static void tick()
    pub fn tick() {
        if mcrust::mcrust::modding::game::with_player(|_| ()).is_none() || !(Utils::is_on_skyblock()) {
            return;
        }
        compat::set_state(&TICKS, compat::get_state(&TICKS) + 1);
        // one guard for the whole statement: two `compat::lock(&HEALTH)` temporaries in one
        // expression both live to its end, and the second waits for the first (a self-deadlock)
        #[allow(unused_mut)] let mut __guard_health = compat::lock(&HEALTH);
        let (__value, __max) = ((*__guard_health).value, (*__guard_health).max);
        drop(__guard_health); // released BEFORE the call: update_health_2 locks HEALTH itself
        StatusBarTracker::update_health_2(__value, __max);
        StatusBarTracker::update_speed();
        StatusBarTracker::update_air();
        EstimatedResource::tick(&compat::get_state(&MANA));
        EstimatedResource::tick(&compat::get_state(&VITALITY));
    }

    /// java: @SuppressWarnings("SameReturnValue") private static InteractionResult interactItem(Player player, Level world, InteractionHand hand)
    pub fn interact_item(player: &dyn Player, world: &Level, hand: &InteractionHand) -> InteractionResult {
        // java: if (client.player == null) return InteractionResult.PASS;
        //       ItemStack handStack = client.player.getMainHandItem();
        // (the callback is handed the client's player - the game holds it while it asks, so it is read through the
        // argument rather than `game::with_player`, which would wait on that hold)
        use mcrust::net::minecraft::world::entity::player::player::Player as _;
        let mut hand_stack: mcrust::net::minecraft::world::item::item_stack::ItemStack = player.player().base.get_item_in_hand(mcrust::net::minecraft::world::interaction_hand::InteractionHand::MainHand).clone();
        let mut mana_cost: i32 = 0;
        for ability in hand_stack.skyblocker_get_abilities() {
            if ability.activation().is_right_click() {
                mana_cost = ability.mana_cost().unwrap_or(0);
                break;
            }
        }
        if mana_cost > 0 && mana_cost <= EstimatedResource::resource(&compat::get_state(&MANA)).value() {
            { let __v = Resource::new(((EstimatedResource::resource(&compat::get_state(&MANA)).value()).wrapping_sub(mana_cost)).max(0), EstimatedResource::resource(&compat::get_state(&MANA)).max(), EstimatedResource::resource(&compat::get_state(&MANA)).overflow()); compat::write(&compat::get_state(&MANA), |__t| __t.resource = __v); }
        }
        return InteractionResult::Pass;
    }

    /// java: @VisibleForTesting protected static boolean allowOverlayMessage(Component text, boolean overlay)
    pub fn allow_overlay_message(text: &MutableComponent, mut overlay: bool) -> bool {
        return (!overlay || !((StatusBarTracker::on_overlay_message(text.clone(), overlay)).get_string().is_empty()));
    }

    /// java: private static Component onOverlayMessage(Component text, boolean overlay)
    pub fn on_overlay_message(mut text: MutableComponent, mut overlay: bool) -> MutableComponent {
        if !overlay || !(Utils::is_on_skyblock()) {
            return text;
        }
        let mut now: i64 = compat::millis();
        if *LAST_MESSAGE.get() == text && (compat::get_state(&LAST_MESSAGE_TIME)).wrapping_add(337 as i64) > now {
            return LAST_RETURN.get_mut().clone();
        }
        *LAST_MESSAGE.get_mut() = text.clone();
        compat::set_state(&LAST_MESSAGE_TIME, now);
        let mut stringified: String = text.get_string();
        // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`; the body's
        // `return lastReturn = Component.literal(returned)` is the closure's `Some`
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| -> Option<MutableComponent> {
            let mut returned: String = StatusBarTracker::update(&stringified, SkyblockerConfigManager::get().chat.hide_mana);
            if FancyStatusBars::is_enabled(fancy_status_bars::INSTANCE.get_mut()) && !(stringified == returned) {
                return Some(component::literal(&returned));
            }
            None
        }));
        match __result {
            Ok(Some(returned)) => {
                //   java: return lastReturn = Component.literal(returned);
                *LAST_RETURN.get_mut() = returned.clone();
                return returned;
            }
            Ok(None) => {}
            Err(e) => {
                let mut stripped: String = chat_formatting::strip_formatting(&stringified);
                logger().error(&format!("[Skyblocker Status Bar Tracker] Failed to update status bars! Content: '{}' {}", stripped, compat::panic_message(&e)));
            }
        }
        //   java: return lastReturn = text;
        *LAST_RETURN.get_mut() = text.clone();
        return text;
    }

    /// java: @VisibleForTesting protected static String update(String actionBar, boolean filterManaUse)
    pub fn update(action_bar: &str, mut filter_mana_use: bool) -> String {
        let mut statuses: compat::regex::Matcher = STATUS_PATTERN.matcher(action_bar);
        let mut output = String::new();
        // the matcher's append position (`Matcher.appendReplacement` / `appendTail`, see `append_replacement`)
        let mut append_pos: i32 = 0;
        while statuses.find() {
            let mut status: compat::regex::Matcher;
            if Utils::is_in_the_rift() {
                status = RIFT_TIME_STATUS.matcher(&chat_formatting::strip_formatting(statuses.group_named("status").as_deref().unwrap()));
                if FancyStatusBars::is_experience_fancy_bar_enabled(fancy_status_bars::INSTANCE.get_mut()) && status.find() {
                    //   java: statuses.appendReplacement(output, "");
                    append_replacement(&mut output, action_bar, &mut append_pos, &statuses, "");
                }
            } else {
                status = HEALTH_STATUS.matcher(&chat_formatting::strip_formatting(statuses.group_named("status").as_deref().unwrap()));
                if status.find() {
                    StatusBarTracker::update_health(&status);
                    if FancyStatusBars::is_health_fancy_bar_enabled(fancy_status_bars::INSTANCE.get_mut()) {
                        if status.group_named("healing").is_none() {
                            //   java: statuses.appendReplacement(output, "");
                            append_replacement(&mut output, action_bar, &mut append_pos, &statuses, "");
                        } else {
                            status = HEALING.matcher(statuses.group(0).as_deref().unwrap());
                            if !status.find() {
                                continue;
                            }
                            if !(status.group(0).unwrap().starts_with("§")) {
                                { output.push_str("§c"); };
                            }
                            //   java: statuses.appendReplacement(output, statuses.group().substring(status.start()));
                            append_replacement(&mut output, action_bar, &mut append_pos, &statuses, &compat::substring(&statuses.group(0).unwrap(), status.start(0)));
                        }
                    } else {
                        //   java: statuses.appendReplacement(output, "$0");
                        append_replacement(&mut output, action_bar, &mut append_pos, &statuses, "$0");
                    }
                } else if status.use_pattern(&DEFENSE_STATUS).find() {
                    compat::set_state(&DEFENSE, RegexUtils::parse_int_from_matcher_matcher_string(&status, "defense"));
                    if FancyStatusBars::is_health_fancy_bar_enabled(fancy_status_bars::INSTANCE.get_mut()) {
                        //   java: statuses.appendReplacement(output, "");
                        append_replacement(&mut output, action_bar, &mut append_pos, &statuses, "");
                    } else {
                        //   java: statuses.appendReplacement(output, "$0");
                        append_replacement(&mut output, action_bar, &mut append_pos, &statuses, "$0");
                    }
                } else if status.use_pattern(&VITALITY_STATUS).find() {
                    StatusBarTracker::update_vitality(&status);
                    if FancyStatusBars::is_bar_enabled(fancy_status_bars::INSTANCE.get_mut(), StatusBarType::Vitality) {
                        //   java: statuses.appendReplacement(output, "");
                        append_replacement(&mut output, action_bar, &mut append_pos, &statuses, "");
                    } else {
                        //   java: statuses.appendReplacement(output, "$0");
                        append_replacement(&mut output, action_bar, &mut append_pos, &statuses, "$0");
                    }
                }
            }
            if status.use_pattern(&MANA_USE).find() {
                if filter_mana_use {
                    //   java: statuses.appendReplacement(output, "");
                    append_replacement(&mut output, action_bar, &mut append_pos, &statuses, "");
                }
            } else if status.use_pattern(&MANA_STATUS).find() {
                StatusBarTracker::update_mana(&status);
                if FancyStatusBars::is_bar_enabled(fancy_status_bars::INSTANCE.get_mut(), StatusBarType::Intelligence) {
                    //   java: statuses.appendReplacement(output, "");
                    append_replacement(&mut output, action_bar, &mut append_pos, &statuses, "");
                } else {
                    //   java: statuses.appendReplacement(output, "$0");
                    append_replacement(&mut output, action_bar, &mut append_pos, &statuses, "$0");
                }
            }
        }
        //   java: return statuses.appendTail(output).toString().trim();
        append_tail(&mut output, action_bar, append_pos);
        return compat::trim(&output);
    }

    /// java: private static void updateHealth(Matcher matcher)
    pub fn update_health(matcher: &compat::regex::Matcher) {
        let mut health: i32 = RegexUtils::parse_int_from_matcher_matcher_string(matcher, "health");
        let mut max: i32 = RegexUtils::parse_int_from_matcher_matcher_string(matcher, "max");
        if crate::de::hysky::skyblocker::debug::debug::Debug::is_test_environment() || mcrust::mcrust::modding::game::with_player(|_| ()).is_none() || mcrust::mcrust::modding::game::with_player(|player| player.base.base.base.get_health()).unwrap() == mcrust::mcrust::modding::game::with_player(|player| player.base.base.base.get_max_health()).unwrap() {
            compat::set_state(&ABSORPTION, 0i32.max(health.wrapping_sub(max)));
        } else {
            compat::set_state(&ABSORPTION, ((((health as f32) - ((mcrust::mcrust::modding::game::with_player(|player| player.base.base.base.get_health()).unwrap() * (max as f32)) / mcrust::mcrust::modding::game::with_player(|player| player.base.base.base.get_max_health()).unwrap()))) as i32));
        }
        StatusBarTracker::update_health_2(health, max);
    }

    /// java: private static void updateHealth(int value, int max)
    pub fn update_health_2(mut value: i32, mut max: i32) {
        if !(crate::de::hysky::skyblocker::debug::debug::Debug::is_test_environment()) && mcrust::mcrust::modding::game::with_player(|_| ()).is_some() {
            value = ((((mcrust::mcrust::modding::game::with_player(|player| player.base.base.base.get_health()).unwrap() * (max as f32)) / mcrust::mcrust::modding::game::with_player(|player| player.base.base.base.get_max_health()).unwrap())) as i32);
        }
        compat::set_state(&HEALTH, Resource::new(value.min(max), max, compat::get_state(&ABSORPTION)));
    }

    /// java: private static void updateVitality(Matcher m)
    pub fn update_vitality(m: &compat::regex::Matcher) {
        if !(SkyblockerConfigManager::get().ui_and_visuals.bars.has_seen_vitality_at_least_once) {
            SkyblockerConfigManager::update_only(Arc::new(move |mut config| { config.ui_and_visuals.bars.has_seen_vitality_at_least_once = true; }));
            FancyStatusBars::make_vitality_visible(fancy_status_bars::INSTANCE.get_mut());
        }
        EstimatedResource::update(&compat::get_state(&VITALITY), Resource::new(RegexUtils::parse_int_from_matcher_matcher_string(m, "vitality"), RegexUtils::parse_int_from_matcher_matcher_string(m, "max"), 0));
    }

    /// java: private static void updateMana(Matcher m)
    pub fn update_mana(m: &compat::regex::Matcher) {
        let mut mana: i32 = RegexUtils::parse_int_from_matcher_matcher_string(m, "mana");
        let mut max: i32 = RegexUtils::parse_int_from_matcher_matcher_string(m, "max");
        let mut overflow: i32 = (if m.group_named("overflow").is_none() { 0 } else { RegexUtils::parse_int_from_matcher_matcher_string(m, "overflow") });
        EstimatedResource::update(&compat::get_state(&MANA), Resource::new(mana, max, overflow));
    }

    /// java: private static void updateSpeed()
    pub fn update_speed() {
        debug_assert!(mcrust::mcrust::modding::game::with_player(|_| ()).is_some());
        let mut value: i32 = (((if mcrust::mcrust::modding::game::with_player(|player| player.base.base.base.base.get_shared_flag(3)).unwrap_or(false) { ((mcrust::mcrust::modding::game::with_player(|player| player.get_speed()).unwrap() / 1.3f32) * (1000 as f32)) } else { (mcrust::mcrust::modding::game::with_player(|player| player.get_speed()).unwrap() * (1000 as f32)) })) as i32);
        let mut max: i32 = 400;
        if mcrust::mcrust::modding::game::with_player(|player| compat::component_of(player.get_main_hand_item().get_hover_name()).get_string().contains("Cactus Knife")).unwrap_or(false) && Utils::get_location() == Location::Garden {
            max = 500;
        }
        let mut armor: Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack> = mcrust::mcrust::modding::game::with_player(|player| ItemUtils::get_armor(player)).unwrap();
        let mut young_drag_count: i32 = 0;
        for armor_piece in armor.iter() {
            if compat::component_of(armor_piece.get_hover_name()).get_string().contains("Racing Helmet") {
                max = 500;
            } else if compat::component_of(armor_piece.get_hover_name()).get_string().contains("Young Dragon") {
                young_drag_count += 1;
            }
        }
        if young_drag_count == 4 {
            max = 500;
        }
        let mut pet: Option<PetInfo> = PetCache::get_current_pet();
        if let Some(pet) = &pet {
            if (pet.r#type()).contains("BLACK_CAT") {
                max = 500;
            } else if (pet.r#type()).contains("SNAIL") {
                max = 100;
            }
        }
        compat::set_state(&SPEED, Resource::new(value, max, 0));
    }

    /// java: private static void updateAir()
    pub fn update_air() {
        debug_assert!(mcrust::mcrust::modding::game::with_player(|_| ()).is_some());
        let mut max: i32 = mcrust::mcrust::modding::game::with_player(|player| player.base.base.base.base.get_max_air_supply()).unwrap();
        let mut value: i32 = (mcrust::mcrust::modding::game::with_player(|player| player.base.base.base.base.get_air_supply()).unwrap()).clamp(0, max);
        compat::set_state(&AIR, Resource::new(value, max, 0));
    }

}

/// java: public record Resource(int value, int max, int overflow)
#[derive(Clone, Debug, PartialEq)]
pub struct Resource {
    /// java: int value
    pub value: i32,
    /// java: int max
    pub max: i32,
    /// java: int overflow
    pub overflow: i32,
}

impl Resource {
    /// java: the canonical record constructor
    pub fn new(mut value: i32, mut max: i32, mut overflow: i32) -> Self {
        Self { value, max, overflow }
    }

    /// java: the record accessor `value()`
    pub fn value(&self) -> i32 {
        self.value
    }

    /// java: the record accessor `max()`
    pub fn max(&self) -> i32 {
        self.max
    }

    /// java: the record accessor `overflow()`
    pub fn overflow(&self) -> i32 {
        self.overflow
    }

}

/// java: public static class EstimatedResource
#[derive(Clone)]
pub struct EstimatedResource {
    /// java: private Resource resource;
    pub resource: Resource,
    /// java: private int perSecond;
    pub per_second: i32,
    /// java: private int lastTick;
    pub last_tick: i32,
    /// java: private int lastValue;
    pub last_value: i32,
}

impl EstimatedResource {
    /// java: public EstimatedResource(Resource baseValue)
    pub fn new(mut base_value: Resource) -> compat::Shared<Self> {
        let __init_resource = base_value.clone();
        let __this = compat::shared(Self {
            resource: __init_resource,
            per_second: 0,
            last_tick: 0,
            last_value: 0,
        });
        __this
    }

    /// java: private void update(Resource newValue)
    pub fn update(__this: &compat::Shared<Self>, mut new_value: Resource) {
        { let __v = new_value.clone(); compat::write(__this, |__t| __t.resource = __v); }
        if __this.lock().borrow().resource.value() != __this.lock().borrow().resource.max() && compat::read(__this, |__t| __t.last_value) < __this.lock().borrow().resource.value() {
            { let __v = ((__this.lock().borrow().resource.value()).wrapping_sub(compat::read(__this, |__t| __t.last_value))).max(0); compat::write(__this, |__t| __t.per_second = __v); }
        }
        if compat::read(__this, |__t| __t.last_value) != __this.lock().borrow().resource.value() || __this.lock().borrow().resource.value() == __this.lock().borrow().resource.max() {
            { let __v = compat::get_state(&TICKS); compat::write(__this, |__t| __t.last_tick = __v); }
        }
        { let __v = __this.lock().borrow().resource.value(); compat::write(__this, |__t| __t.last_value = __v); }
    }

    /// java: private void tick()
    pub fn tick(__this: &compat::Shared<Self>) {
        if (compat::get_state(&TICKS)).wrapping_sub(compat::read(__this, |__t| __t.last_tick)) > 0 && ((compat::get_state(&TICKS)).wrapping_sub(compat::read(__this, |__t| __t.last_tick))).wrapping_rem(20) == 0 {
            { let __v = Resource::new(((__this.lock().borrow().resource.value()).wrapping_add(compat::read(__this, |__t| __t.per_second))).min(__this.lock().borrow().resource.max()), __this.lock().borrow().resource.max(), __this.lock().borrow().resource.overflow()); compat::write(__this, |__t| __t.resource = __v); }
        }
    }

    /// java: public boolean isEstimated()
    pub fn is_estimated(__this: &compat::Shared<Self>) -> bool {
        return ((compat::get_state(&TICKS)).wrapping_sub(compat::read(__this, |__t| __t.last_tick)) > 30);
    }

    /// java: public Resource resource()
    pub fn resource(__this: &compat::Shared<Self>) -> Resource {
        return compat::read(__this, |__t| __t.resource.clone());
    }

    /// java: public int value()
    pub fn value(__this: &compat::Shared<Self>) -> i32 {
        return __this.lock().borrow().resource.value();
    }

    /// java: public int max()
    pub fn max(__this: &compat::Shared<Self>) -> i32 {
        return __this.lock().borrow().resource.max();
    }

    /// java: public int overflow()
    pub fn overflow(__this: &compat::Shared<Self>) -> i32 {
        return __this.lock().borrow().resource.overflow();
    }

}

/// `matcher.appendReplacement(sb, replacement)` (`java.util.regex.Matcher:1001-1034`): the input from the append
/// position to the match's start, then the replacement - `$n` / `${name}` are the match's groups, `\x` is `x` - and
/// the append position moves to the match's end. `compat::regex::Matcher` keeps no append position, so the caller
/// keeps it (`append_pos`, a char index into `input`, as the matcher's `start` / `end` are).
fn append_replacement(output: &mut String, input: &str, append_pos: &mut i32, matcher: &compat::regex::Matcher, replacement: &str) {
    let start = matcher.start(0);
    output.push_str(&compat::substring2(input, *append_pos, start));
    let chars: Vec<char> = replacement.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        if c == '\\' {
            i += 1;
            if i < chars.len() {
                output.push(chars[i]);
                i += 1;
            }
        } else if c == '$' {
            i += 1;
            if i < chars.len() && chars[i] == '{' {
                i += 1;
                let mut name = String::new();
                while i < chars.len() && chars[i] != '}' {
                    name.push(chars[i]);
                    i += 1;
                }
                i += 1;
                if let Some(group) = matcher.group_named(&name) {
                    output.push_str(&group);
                }
            } else {
                // the first digit, then more digits while the number stays a group of the pattern
                let mut group = chars.get(i).and_then(|d| d.to_digit(10)).expect("IllegalArgumentException: Illegal group reference") as i32;
                i += 1;
                while let Some(d) = chars.get(i).and_then(|d| d.to_digit(10)) {
                    let next = group * 10 + d as i32;
                    if next > matcher.group_count() {
                        break;
                    }
                    group = next;
                    i += 1;
                }
                if let Some(text) = matcher.group(group) {
                    output.push_str(&text);
                }
            }
        } else {
            output.push(c);
            i += 1;
        }
    }
    *append_pos = matcher.end(0);
}

/// `matcher.appendTail(sb)`: the input from the append position to its end.
fn append_tail(output: &mut String, input: &str, append_pos: i32) {
    output.push_str(&compat::substring(input, append_pos));
}
