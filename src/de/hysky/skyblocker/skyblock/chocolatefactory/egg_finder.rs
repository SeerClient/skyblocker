//! Ported from de.hysky.skyblocker.skyblock.chocolatefactory.EggFinder by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::events::skyblock_events;
use crate::de::hysky::skyblocker::utils::color_utils::ColorUtils;
use crate::de::hysky::skyblocker::utils::constants;
use crate::de::hysky::skyblocker::utils::item_utils::ItemUtils;
use crate::de::hysky::skyblocker::utils::location::Location;
use crate::de::hysky::skyblocker::utils::render::level_render_extraction_callback;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_collector::PrimitiveCollector;
use crate::de::hysky::skyblocker::utils::render::renderable::Renderable;
use crate::de::hysky::skyblocker::utils::time::skyblock_time;
use crate::de::hysky::skyblocker::utils::time::skyblock_time::Season;
use crate::de::hysky::skyblocker::utils::waypoint::waypoint::Waypoint;
use crate::de::hysky::skyblocker::utils::waypoint::waypoint::WaypointBase;
use crate::de::hysky::skyblocker::utils::ws::message::egg_waypoint_message::EggWaypointMessage;
use crate::de::hysky::skyblocker::utils::ws::message::message::Message;
use crate::de::hysky::skyblocker::utils::ws::service::Service;
use crate::de::hysky::skyblocker::utils::ws::ws_message_handler::WsMessageHandler;
use crate::de::hysky::skyblocker::utils::ws::ws_state_manager::WsStateManager;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::core::block_pos::BlockPos;
use mcrust::net::minecraft::network::chat::click_event::ClickEvent;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::hover_event::HoverEvent;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::world::entity::decoration::armor_stand::ArmorStand;
use mcrust::net::minecraft::world::phys::aabb::AABB;
use mcrust::net::minecraft::network::chat::component::Component as _;
use mcrust::net::minecraft::world::entity::item_owner::ItemOwner as _;
use mcrust::net::minecraft::world::level::entity_getter::EntityGetter as _;

/// java: private static final Pattern EGG_FOUND_PATTERN = Pattern.compile("^(?:HOPPITY'S HUNT You found a Chocolate|You have already collected this Chocolate) (Breakfast|Lunch|Dinner|Brunch|Déjeuner|Supper) Eg
pub static EGG_FOUND_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("^(?:HOPPITY'S HUNT You found a Chocolate|You have already collected this Chocolate) (Breakfast|Lunch|Dinner|Brunch|Déjeuner|Supper) Egg"));
/// java: private static final Pattern NO_EGGS_PATTERN = Pattern.compile("^There are no hidden Chocolate Rabbit Eggs nearby! Try again later!$");
pub static NO_EGGS_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("^There are no hidden Chocolate Rabbit Eggs nearby! Try again later!$"));
/// java: private static final Set<Location> LOCATIONS = Set.of( Location.BACKWATER_BAYOU, Location.CRIMSON_ISLE, Location.CRYSTAL_HOLLOWS, Location.DEEP_CAVERNS, Location.DUNGEON_HUB, Location.DWARVEN_MINES, L
pub static LOCATIONS: LazyLock<std::collections::HashSet<Location>> = LazyLock::new(|| std::collections::HashSet::from([Location::BackwaterBayou, Location::CrimsonIsle, Location::CrystalHollows, Location::DeepCaverns, Location::DungeonHub, Location::DwarvenMines, Location::Galatea, Location::GoldMine, Location::Hub, Location::LotusAtoll, Location::SpidersDen, Location::TheEnd, Location::TheFarmingIsland, Location::ThePark, Location::TorrhusCanyon]));
// TODO(fabric2seer): `.get(..)` on a AtomicReference
//   java: private static boolean isSpring = SkyblockTime.skyblockSeason.get() == SkyblockTime.Season.SPRING;
/// java: private static boolean isSpring = SkyblockTime.skyblockSeason.get() == SkyblockTime.Season.SPRING;
pub static IS_SPRING: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(todo!("fabric2seer: the initialiser of `isSpring` - see the TODO above")));
// TODO(fabric2seer): `from_values` takes 2 argument(s) in the port (`values: Vec<E>, serialized_name: impl Fn(&E) -> String + Send + Sync + Copy + 'static`), Java passes 1
//   java: public static final Codec<EggType> CODEC = StringRepresentable.fromValues(EggType::values);
/// java: public static final Codec<EggType> CODEC = StringRepresentable.fromValues(EggType::values);
pub static EGG_TYPE_CODEC: compat::MainThread<CodecRef<EggType>> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `CODEC` - see the TODO above"));
// TODO(fabric2seer): static call ObjectImmutableList.of(..)
//   java: public static final ObjectImmutableList<EggType> entries = ObjectImmutableList.of(EggType.values());
/// java: public static final ObjectImmutableList<EggType> entries = ObjectImmutableList.of(EggType.values());
pub static EGG_TYPE_ENTRIES: LazyLock<Vec<EggType>> = LazyLock::new(|| todo!("fabric2seer: the initialiser of `entries` - see the TODO above"));

/// java: public class EggFinder
#[derive(Clone)]
pub struct EggFinder {}

impl EggFinder {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        client_play_connection_events::JOIN.register(Arc::new(|listener: &mut ClientPacketListener| {
            EggFinder::clear_eggs();
        }));
        skyblock_events::LOCATION_CHANGE.register(Arc::new(move |__a0| { EggFinder::handle_location_change(__a0); }));
        // TODO(fabric2seer): ClientReceiveMessageEvents.ALLOW_GAME.register: no SeerClient equivalent yet
        //   java: ClientReceiveMessageEvents.ALLOW_GAME.register(EggFinder::onChatMessage);
        level_render_extraction_callback::EVENT.register(Arc::new(move |__a0| { EggFinder::extract_rendering(&__a0); }));
        skyblock_time::HOUR_CHANGE.register(Arc::new(move |hour| {
            if !(compat::get_state(&IS_SPRING)) {
                return;
            }
            // TODO(fabric2seer): `.get(..)` on a AtomicReference
            //   java: int dayNumber = SkyblockTime.skyblockMonth.get().ordinal() * 31 + SkyblockTime.skyblockDay.get();
            let mut day_number: i32 = todo!("fabric2seer: the initialiser of `dayNumber` (TODO above)");
            let mut is_odd: bool = (day_number.wrapping_rem(2) == 1);
            for r#type in EGG_TYPE_ENTRIES.iter().copied() {
                if hour == (r#type).fields().reset_hour && is_odd == (r#type).fields().odd_day {
                    (r#type).fields().collected = false;
                    (r#type).fields().prev_egg = (r#type).fields().egg.clone();
                    (r#type).fields().egg = None;
                }
            }
        }));
        skyblock_time::SEASON_CHANGE.register(Arc::new(move |season| {
            compat::set_state(&IS_SPRING, (season == Season::Spring));
            if !(compat::get_state(&IS_SPRING)) {
                EggFinder::clear_eggs();
            }
        }));
        client_command_registration_callback::EVENT.register(Arc::new(
            |dispatcher: &mut client_command_registration_callback::ClientCommandDispatcher| {
                // TODO(fabric2seer): a command tree this tool cannot read (arguments, redirects, requires) - SeerClient commands receive the words after the name in `ctx.args`
                //   java: dispatcher.register(literal(SkyblockerMod.NAMESPACE).then(literal("eggFinder").then(literal("shareLocation").then(argument("eggType", EggTypeArgumentType.eggType())
                //   java: 					.executes(context -> {
                //   java: 						EggType eggType = context.getArgument("eggType", EggType.class);
                //   java: 						if (eggType.egg == null) {
                //   java: 							context.getSource().sendError(Constants.PREFIX.get().append(Component.translatable("skyblocker.helpers.hoppitysHunt.unableToShareEgg").withStyle(style -> style.withColor(ChatFormatting.RED))));
                //   java: 							return Command.SINGLE_SUCCESS;
                //   java: 						}
                //   java: 						MessageScheduler.INSTANCE.sendMessageAfterCooldown("[Skyblocker] Chocolate %s Egg found at %s".formatted(eggType.name, eggType.egg.pos.toShortString()), false);
                //   java: 						return Command.SINGLE_SUCCESS;
                //   java: 					})))));
                if !(crate::de::hysky::skyblocker::debug::debug::Debug::debug_enabled()) {
                    return;
                }
                // TODO(fabric2seer): a command tree this tool cannot read (arguments, redirects, requires) - SeerClient commands receive the words after the name in `ctx.args`
                //   java: dispatcher.register(literal(SkyblockerMod.NAMESPACE).then(literal("eggFinder").then(literal("resetFoundStatus")
                //   java: 					.executes(_ -> {
                //   java: 						for (EggType type : EggType.entries) {
                //   java: 							type.collected = false;
                //   java: 							if (type.egg != null) type.egg.setMissing();
                //   java: 						}
                //   java: 						return Command.SINGLE_SUCCESS;
                //   java: 					}))));
                // TODO(fabric2seer): a command tree this tool cannot read (arguments, redirects, requires) - SeerClient commands receive the words after the name in `ctx.args`
                //   java: dispatcher.register(literal(SkyblockerMod.NAMESPACE).then(literal("eggFinder").then(literal("clearWaypoints")
                //   java: 					.executes(_ -> {
                //   java: 						clearEggs();
                //   java: 						return Command.SINGLE_SUCCESS;
                //   java: 					}))));
            },
        ));
    }

    /// java: public static void onWebsocketMessage(EggWaypointMessage message)
    pub fn on_websocket_message(message: &EggWaypointMessage) {
        let mut egg_type: EggType = message.egg_type();
        egg_type.fields().egg = Egg::new(message.coordinates(), egg_type);
        egg_type.on_egg_received();
    }

    /// java: private static void clearEggs()
    pub fn clear_eggs() {
        for r#type in EGG_TYPE_ENTRIES.iter().copied() {
            (r#type).fields().egg = None;
            (r#type).fields().prev_egg = None;
        }
    }

    /// java: private static void handleLocationChange(Location location)
    pub fn handle_location_change(mut location: Location) {
        EggFinder::clear_eggs();
        if !(compat::get_state(&IS_SPRING)) || !(LOCATIONS.contains(&location)) || !(SkyblockerConfigManager::get().helpers.chocolate_factory.enable_egg_finder) {
            return;
        }
        WsStateManager::subscribe_island(Service::EggWaypoints, &None);
    }

    /// java: public static boolean checkIfEgg(ArmorStand armorStand, EggType eggType)
    pub fn check_if_egg(armor_stand: &ArmorStand, mut egg_type: EggType) -> bool {
        if armor_stand.base.base.has_custom_name() || !armor_stand.is_invisible_field() || armor_stand.show_base_plate() {
            return false;
        }
        return EggFinder::handle_armor_stand(armor_stand, egg_type);
    }

    /// java: private static boolean handleArmorStand(ArmorStand armorStand, EggType eggType)
    pub fn handle_armor_stand(armor_stand: &ArmorStand, mut egg_type: EggType) -> bool {
        for item_stack in ItemUtils::get_armor(armor_stand) {
            let mut texture: Option<String> = ItemUtils::get_head_texture_optional(&item_stack);
            if texture.is_none() {
                continue;
            }
            if texture.clone().unwrap() == egg_type.fields().texture {
                return true;
            }
        }
        return false;
    }

    /// java: private static void extractRendering(PrimitiveCollector collector)
    pub fn extract_rendering(collector: &compat::Shared<dyn PrimitiveCollector>) {
        if !(compat::get_state(&IS_SPRING)) || !(SkyblockerConfigManager::get().helpers.chocolate_factory.enable_egg_finder) {
            return;
        }
        for r#type in EGG_TYPE_ENTRIES.iter().copied() {
            let mut egg: Option<compat::Shared<Egg>> = Some((r#type).fields().egg.clone());
            if let Some(egg) = &egg {
                ({ let __f = compat::dispatch(egg, |__t| __t.extract_rendering()); __f(&compat::view(egg, |a| a as compat::Shared<dyn Renderable>), collector) });
            }
        }
    }

    /// java: @SuppressWarnings("SameReturnValue") private static boolean onChatMessage(Component text, boolean overlay)
    pub fn on_chat_message(text: &MutableComponent, mut overlay: bool) -> bool {
        if overlay || !(compat::get_state(&IS_SPRING)) || !(SkyblockerConfigManager::get().helpers.chocolate_factory.enable_egg_finder) {
            return true;
        }
        let mut matcher: compat::regex::Matcher = NO_EGGS_PATTERN.matcher(&text.get_string());
        if matcher.matches() {
            for r#type in EGG_TYPE_ENTRIES.iter().copied() {
                let mut egg: Option<compat::Shared<Egg>> = Some((r#type).fields().egg.clone());
                let Some(mut egg) = egg.clone() else {
                    continue;
                };
                (r#type).fields().collected = true;
                ({ let __f = compat::dispatch(&egg, |__t| __t.set_found()); __f(&compat::view(&egg, |a| a as compat::Shared<dyn Waypoint>)) });
            }
            return true;
        }
        matcher.use_pattern(&EGG_FOUND_PATTERN);
        if !matcher.find() {
            return true;
        }
        // java: try {..} catch (IllegalArgumentException e) - the body runs inline: a Rust body does not throw checked exceptions (a panic in it propagates)
        {
            let Some(mut egg_type) = EggType::get_type_by_name(matcher.group(1).as_deref().unwrap()) else {
                return true;
            };
            egg_type.fields().collected = true;
            if true /* java: egg_type.fields().egg != null: never null here */ {
                ({ let __r = &egg_type.fields().egg; let __f = compat::dispatch(__r, |__t| __t.set_found()); __f(&compat::view(__r, |a| a as compat::Shared<dyn Waypoint>)) });
                return true;
            }
            logger().info("[Skyblocker Egg Finder] Discovered a new egg!");
            let client = compat::minecraft();
            if mcrust::mcrust::modding::game::with_player(|_| ()).is_none() || mcrust::mcrust::modding::game::level().is_none() {
                return true;
            }
            // TODO(fabric2seer): the port's `get_entities_of_class` ports this Java method with other parameters (base_class, except, bb, selector for Java's base_class, bb, selector)
            // TODO(fabric2seer): `get_entities_of_class` takes 4 argument(s) in the port (`base_class: crate::net::minecraft::world::level::entity::entity_type_test::EntityClass, except: Option<i32>, bb: &AABB, selector: &dyn Fn(&(dyn Entity + Send + Sync)) -> bool`), Java passes 3
            // TODO(fabric2seer): a lambda for a parameter the port does not type as a closure (`Predicate<? super T>`)
            // rust: let mut entities: Vec<ArmorStand> = mcrust::mcrust::modding::game::level().unwrap().get_entities_of_class(std::any::TypeId::of::<ArmorStand>(), Some(AABB::of_size(&mcrust::mcrust::modding::game::with_player(|player| player.position()).unwrap(), (4.0f32) as f64, (4.0f32) as f64, (4.0f32) as f64)), |entity| EggFinder::check_if_egg(&entity, egg_type));
            let mut entities: Vec<ArmorStand> = todo!("fabric2seer: the initialiser of `entities` (TODO above)");
            if (entities.len() as i32) != 1 {
                return true;
            }
            egg_type.fields().egg = Egg::new(entities[0].base.base.block_position().above_n(2), egg_type);
            ({ let __r = &egg_type.fields().egg; let __f = compat::dispatch(__r, |__t| __t.set_found()); __f(&compat::view(__r, |a| a as compat::Shared<dyn Waypoint>)) });
            egg_type.send_egg_message();
            if { let __r = &egg_type.fields().egg; let __f = compat::dispatch(__r, |__t| __t.equals()); __f(&compat::view(__r, |a| a as compat::Shared<dyn Waypoint>), &egg_type.fields().prev_egg) } {
                logger().info("[Skyblocker Egg Finder] Not sharing this egg to the WebSocket - matches previous location");
                return true;
            }
            WsMessageHandler::send_location_message(Service::EggWaypoints, &(compat::shared(EggWaypointMessage::new(egg_type, compat::read(&egg_type.fields().egg, |__t| __t.pos.clone()).unwrap(), None)) as compat::Shared<dyn Message<compat::Object>>));
        }
        // TODO(fabric2seer): the `catch (IllegalArgumentException e)` block is not reached in Rust - the try body returns, breaks or continues, so it cannot run inside `catch_unwind`
        //   java: {
        //   java: 			LOGGER.error("[Skyblocker Egg Finder] Failed to process an egg!", e);
        //   java: 		}
        return true;
    }

}

/// java: @SuppressWarnings("DataFlowIssue") //Removes that pesky "unboxing of Integer might cause NPE" warning when we already know it's not null public enum EggType implements StringRepresentable
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum EggType {
    Breakfast,
    Lunch,
    Dinner,
    Brunch,
    Dejeuner,
    Supper,
}

impl EggType {
    pub const VALUES: [EggType; 6] = [EggType::Breakfast, EggType::Lunch, EggType::Dinner, EggType::Brunch, EggType::Dejeuner, EggType::Supper];

    /// java: values()
    pub fn values() -> Vec<EggType> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            EggType::Breakfast => "BREAKFAST",
            EggType::Lunch => "LUNCH",
            EggType::Dinner => "DINNER",
            EggType::Brunch => "BRUNCH",
            EggType::Dejeuner => "DEJEUNER",
            EggType::Supper => "SUPPER",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> EggType {
        match name {
            "BREAKFAST" => EggType::Breakfast,
            "LUNCH" => EggType::Lunch,
            "DINNER" => EggType::Dinner,
            "BRUNCH" => EggType::Brunch,
            "DEJEUNER" => EggType::Dejeuner,
            "SUPPER" => EggType::Supper,
            _ => panic!("IllegalArgumentException: No enum constant EggType.{name}"),
        }
    }

    /// the constant's constructor data (Java keeps it in the constant's fields)
    pub fn fields(self) -> &'static EggTypeFields {
        static FIELDS: LazyLock<Vec<EggTypeFields>> = LazyLock::new(|| vec![
            EggTypeFields::new("Breakfast".to_string(), mcrust::net::minecraft::network::chat::text_color::TextColor::GOLD.get_value(), 7, "ewogICJ0aW1lc3RhbXAiIDogMTcxMTQ2MjY3MzE0OSwKICAicHJvZmlsZUlkIiA6ICJiN2I4ZTlhZjEwZGE0NjFmOTY2YTQxM2RmOWJiM2U4OCIsCiAgInByb2ZpbGVOYW1lIiA6ICJBbmFiYW5hbmFZZzciLAogICJzaWduYXR1cmVSZXF1aXJlZCIgOiB0cnVlLAogICJ0ZXh0dXJlcyIgOiB7CiAgICAiU0tJTiIgOiB7CiAgICAgICJ1cmwiIDogImh0dHA6Ly90ZXh0dXJlcy5taW5lY3JhZnQubmV0L3RleHR1cmUvYTQ5MzMzZDg1YjhhMzE1ZDAzMzZlYjJkZjM3ZDhhNzE0Y2EyNGM1MWI4YzYwNzRmMWI1YjkyN2RlYjUxNmMyNCIKICAgIH0KICB9Cn0".to_string(), true),
            EggTypeFields::new("Lunch".to_string(), mcrust::net::minecraft::network::chat::text_color::TextColor::BLUE.get_value(), 14, "ewogICJ0aW1lc3RhbXAiIDogMTcxMTQ2MjU2ODExMiwKICAicHJvZmlsZUlkIiA6ICI3NzUwYzFhNTM5M2Q0ZWQ0Yjc2NmQ4ZGUwOWY4MjU0NiIsCiAgInByb2ZpbGVOYW1lIiA6ICJSZWVkcmVsIiwKICAic2lnbmF0dXJlUmVxdWlyZWQiIDogdHJ1ZSwKICAidGV4dHVyZXMiIDogewogICAgIlNLSU4iIDogewogICAgICAidXJsIiA6ICJodHRwOi8vdGV4dHVyZXMubWluZWNyYWZ0Lm5ldC90ZXh0dXJlLzdhZTZkMmQzMWQ4MTY3YmNhZjk1MjkzYjY4YTRhY2Q4NzJkNjZlNzUxZGI1YTM0ZjJjYmM2NzY2YTAzNTZkMGEiCiAgICB9CiAgfQp9".to_string(), true),
            EggTypeFields::new("Dinner".to_string(), mcrust::net::minecraft::network::chat::text_color::TextColor::GREEN.get_value(), 21, "ewogICJ0aW1lc3RhbXAiIDogMTcxMTQ2MjY0OTcwMSwKICAicHJvZmlsZUlkIiA6ICI3NGEwMzQxNWY1OTI0ZTA4YjMyMGM2MmU1NGE3ZjJhYiIsCiAgInByb2ZpbGVOYW1lIiA6ICJNZXp6aXIiLAogICJzaWduYXR1cmVSZXF1aXJlZCIgOiB0cnVlLAogICJ0ZXh0dXJlcyIgOiB7CiAgICAiU0tJTiIgOiB7CiAgICAgICJ1cmwiIDogImh0dHA6Ly90ZXh0dXJlcy5taW5lY3JhZnQubmV0L3RleHR1cmUvZTVlMzYxNjU4MTlmZDI4NTBmOTg1NTJlZGNkNzYzZmY5ODYzMTMxMTkyODNjMTI2YWNlMGM0Y2M0OTVlNzZhOCIKICAgIH0KICB9Cn0".to_string(), true),
            todo!("BRUNCH"),
            todo!("DEJEUNER"),
            todo!("SUPPER"),
        ]);
        &FIELDS[self as usize]
    }

    /// java: public void onEggReceived()
    pub fn on_egg_received(&mut self) {
        if self.fields().collected {
            ({ let __r = &self.fields().egg; let __f = compat::dispatch(__r, |__t| __t.set_found()); __f(&compat::view(__r, |a| a as compat::Shared<dyn Waypoint>)) });
        } else if SkyblockerConfigManager::get().helpers.chocolate_factory.send_egg_found_messages {
            self.send_egg_message();
        }
    }

    /// java: public void sendEggMessage()
    pub fn send_egg_message(&self) {
        let mut egg_name: MutableComponent = (component::translatable_args("skyblocker.helpers.hoppitysHunt.chocolateEgg", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of(self.fields().name.clone())])).with_color(self.fields().color);
        mcrust::mcrust::modding::game::with_player(|player| { script_host::send_feedback(&{ let (__q0,) = (|style| (style.with_click_event(Some(ClickEvent::new_run_command(format!("/skyblocker eggFinder shareLocation {}", compat::jstring(&self)))))).with_hover_event(Some(HoverEvent::new_show_text(mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::translatable("skyblocker.helpers.hoppitysHunt.shareEggPrompt").with_style_format(ChatFormatting::Green))))),); (constants::PREFIX.get_mut())().append(component::translatable_args("skyblocker.helpers.hoppitysHunt.newEggDiscovered", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(egg_name.clone()), mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of(self.fields().egg.lock().borrow().pos.as_ref().unwrap().as_vec3i().to_short_string())])).with_style_fn(__q0) }.to_string()); });
    }

    /// java: @Override public String toString()
    pub fn to_string(&self) -> String {
        // TODO(fabric2seer): static call WordUtils.capitalizeFully(..)
        //   java: return WordUtils.capitalizeFully(this.name());
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public String getSerializedName()
    pub fn get_serialized_name(&self) -> String {
        return self.fields().name.clone();
    }

    /// java: public static @Nullable EggType getTypeByName(String eggType)
    pub fn get_type_by_name(egg_type: &str) -> Option<EggType> {
        for r#type in EGG_TYPE_ENTRIES.iter().copied() {
            if (r#type).fields().name == egg_type {
                return Some(r#type);
            }
        }
        logger().info(&format!("[Skyblocker Egg Finder] Failed to get egg type for egg '{}'", egg_type));
        return None;
    }

}

impl std::fmt::Display for EggType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", EggType::to_string(*self))
    }
}

/// the constructor data of each EggType constant
pub struct EggTypeFields {
    pub name: String,
    pub color: i32,
    pub reset_hour: i32,
    pub texture: String,
    pub odd_day: bool,
    pub collected: bool,
    pub egg: Option<compat::Shared<Egg>>,
    pub prev_egg: Option<compat::Shared<Egg>>,
}

impl EggTypeFields {
    fn new(mut name: String, mut color: i32, mut reset_hour: i32, mut texture: String, mut odd_day: bool) -> Self {
        let mut this = Self {
            name: Default::default(),
            color: 0,
            reset_hour: 0,
            texture: Default::default(),
            odd_day: false,
            collected: false,
            egg: None,
            prev_egg: None,
        };
        this.name = name.clone();
        this.color = color;
        this.reset_hour = reset_hour;
        this.texture = texture.clone();
        this.odd_day = odd_day;
        this
    }

}

/// java: static class Egg extends Waypoint
#[derive(Clone)]
pub struct Egg {
    /// java: extends Waypoint
    pub base: WaypointBase,
}

impl std::ops::Deref for Egg {
    type Target = WaypointBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for Egg {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl Egg {
    /// java: Egg(BlockPos pos, EggType eggType)
    pub fn new(mut pos: BlockPos, mut egg_type: EggType) -> compat::Shared<Self> {
        let (_, __this) = WaypointBase::new5_in(pos.clone(), SkyblockerConfigManager::get().helpers.chocolate_factory.waypoint_type, ColorUtils::get_float_components(egg_type.fields().color), SkyblockerConfigManager::get().helpers.chocolate_factory.show_through_walls, move |__base| {
            let __h = compat::shared(Self {
                base: __base,
            });
            ((__h.clone() as compat::Shared<dyn Waypoint>), __h)
        });
        __this
    }

}

impl Waypoint for Egg {
    fn waypoint_base(&self) -> &WaypointBase {
        &self.base
    }
    fn waypoint_base_mut(&mut self) -> &mut WaypointBase {
        &mut self.base
    }

}

impl Renderable for Egg {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void extractRendering(PrimitiveCollector collector) (dispatch: the body for this class)
    fn extract_rendering(&self) -> fn(&compat::Shared<dyn Renderable>, &compat::Shared<dyn PrimitiveCollector>) {
        <dyn Waypoint>::extract_rendering_from::<Self>
    }

}
