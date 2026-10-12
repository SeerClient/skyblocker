//! Ported from de.hysky.skyblocker.skyblock.events.EventNotifications by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::configs::event_notifications_config::Criterion;
use crate::de::hysky::skyblocker::config::configs::event_notifications_config::EventConfig;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::events::skyblock_events;
use crate::de::hysky::skyblocker::skyblock::events::event_toast::EventToast;
use crate::de::hysky::skyblocker::skyblock::events::event_toast::EventToastBase;
use crate::de::hysky::skyblocker::skyblock::events::event_toast::EventToastAdapter;
use crate::de::hysky::skyblocker::skyblock::events::jacob_event_toast::JacobEventToast;
use crate::de::hysky::skyblocker::skyblock::tabhud::util::ico;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler::Scheduler;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use crate::entry::skyblocker_mod;
use mcrust::com::mojang::brigadier::builder::literal_argument_builder::LiteralArgumentBuilder;
use mcrust::com::mojang::datafixers::util::either::Either;
use mcrust::com::mojang::serialization::codec;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::com::mojang::serialization::codecs::record_codec_builder;
use mcrust::java::lang::throwable::Throwable;
use mcrust::java::util::concurrent::completable_future::CompletableFuture;
use mcrust::net::minecraft::client::resources::sounds::simple_sound_instance::SimpleSoundInstance;
use mcrust::net::minecraft::sounds::sound_event::SoundEvent;
use mcrust::net::minecraft::world::item::items;
use mcrust::com::mojang::serialization::decoder::Decoder as _;

/// java: public static final String JACOBS = "Jacob's Farming Contest";
pub const JACOBS: &str = "Jacob's Farming Contest";
/// java: public static final String MAYOR_JERRY = "Mayor Jerry";
pub const MAYOR_JERRY: &str = "Mayor Jerry";
/// java: public static final EventNotificationsConfig.EventConfig DEFAULT_REMINDERS = new EventNotificationsConfig.EventConfig();
pub static DEFAULT_REMINDERS: LazyLock<compat::Shared<EventConfig>> = LazyLock::new(|| EventConfig::new());
/// java: public static final Map<String, FlexibleItemStack> eventIcons = Map.ofEntries( Map.entry("Dark Auction", new FlexibleItemStack(Items.NETHER_BRICK)), Map.entry("Bonus Fishing Festival", new FlexibleIte
pub static EVENT_ICONS: compat::MainThread<std::collections::HashMap<String, compat::Shared<FlexibleItemStack>>> = compat::MainThread::new(|| std::collections::HashMap::from([("Dark Auction".to_string(), FlexibleItemStack::new2(items::items().get("nether_brick"))), ("Bonus Fishing Festival".to_string(), FlexibleItemStack::new2(items::items().get("fishing_rod"))), ("Bonus Mining Fiesta".to_string(), FlexibleItemStack::new2(items::items().get("iron_pickaxe"))), (JACOBS.to_string(), FlexibleItemStack::new2(items::items().get("iron_hoe"))), ("New Year Celebration".to_string(), FlexibleItemStack::new2(items::items().get("cake"))), ("Election Over!".to_string(), FlexibleItemStack::new2(items::items().get("jukebox"))), ("Election Booth Opens".to_string(), FlexibleItemStack::new2(items::items().get("jukebox"))), (MAYOR_JERRY.to_string(), FlexibleItemStack::new2(items::items().get("villager_spawn_egg"))), ("Spooky Festival".to_string(), FlexibleItemStack::new2(items::items().get("jack_o_lantern"))), ("Season of Jerry".to_string(), FlexibleItemStack::new2(items::items().get("snowball"))), ("Jerry's Workshop Opens".to_string(), FlexibleItemStack::new2(items::items().get("snow_block"))), ("Cult of the Fallen Star".to_string(), ico::NETHER_STAR.get_mut().clone()), ("Traveling Zoo".to_string(), FlexibleItemStack::new2(items::items().get("hay_block")))]));
/// java: private static final FlexibleItemStack FALLBACK_ICON = new FlexibleItemStack(Items.PAPER);
pub static FALLBACK_ICON: compat::MainThread<compat::Shared<FlexibleItemStack>> = compat::MainThread::new(|| FlexibleItemStack::new2(items::items().get("paper")));
/// java: private static long currentTime = System.currentTimeMillis() / 1000;
pub static CURRENT_TIME: LazyLock<Mutex<i64>> = LazyLock::new(|| Mutex::new((compat::millis()).wrapping_div(1000 as i64)));
/// java: private static final Map<String, LinkedList<SkyblockEvent>> events = new ConcurrentHashMap<>();
pub static EVENTS: compat::MainThread<std::collections::HashMap<String, Vec<SkyblockEvent>>> = compat::MainThread::new(|| std::collections::HashMap::new());
// TODO(fabric2seer): `PrimitiveCodec.listOf` has no Rust method in the port (looked for `list_of` in PrimitiveCodec and its supertypes)
//   java: private static final Codec<SkyblockEvent> CODEC = RecordCodecBuilder.create(instance -> instance.group(
//   java: 				Codec.LONG.fieldOf("timestamp").forGetter(SkyblockEvent::start),
//   java: 				Codec.INT.fieldOf("duration").forGetter(SkyblockEvent::duration),
//   java: 				Codec.STRING.fieldOf("event").forGetter(SkyblockEvent::event),
//   java: 				Codec.either(Codec.STRING.listOf(), JerryPerks.CODEC)
//   java: 						.fieldOf("extras").forGetter(SkyblockEvent::extras),
//   java: 				Codec.STRING.fieldOf("location").forGetter(SkyblockEvent::warpCommand)
//   java: 		).apply(instance, SkyblockEvent::new));
/// java: private static final Codec<SkyblockEvent> CODEC = RecordCodecBuilder.create(instance -> instance.group( Codec.LONG.fieldOf("timestamp").forGetter(SkyblockEvent::start), Codec.INT.fieldOf("duration").f
pub static SKYBLOCK_EVENT_CODEC: compat::MainThread<CodecRef<SkyblockEvent>> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `CODEC` - see the TODO above"));
/// java: public static final Codec<List<SkyblockEvent>> LIST_CODEC = CODEC.listOf();
pub static SKYBLOCK_EVENT_LIST_CODEC: compat::MainThread<CodecRef<Vec<SkyblockEvent>>> = compat::MainThread::new(|| codec::list(SKYBLOCK_EVENT_CODEC.get_mut().clone()));
// TODO(fabric2seer): `PrimitiveCodec.listOf` has no Rust method in the port (looked for `list_of` in PrimitiveCodec and its supertypes)
//   java: public static final Codec<JerryPerks> CODEC = RecordCodecBuilder.create(instance -> instance.group(
//   java: 				Codec.STRING.fieldOf("name").forGetter(JerryPerks::mayorName),
//   java: 				Codec.STRING.listOf().fieldOf("perks").forGetter(JerryPerks::perks)
//   java: 		).apply(instance, JerryPerks::new));
/// java: public static final Codec<JerryPerks> CODEC = RecordCodecBuilder.create(instance -> instance.group( Codec.STRING.fieldOf("name").forGetter(JerryPerks::mayorName), Codec.STRING.listOf().fieldOf("perks"
pub static JERRY_PERKS_CODEC: compat::MainThread<CodecRef<JerryPerks>> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `CODEC` - see the TODO above"));

/// java: public class EventNotifications
#[derive(Clone)]
pub struct EventNotifications {}

impl EventNotifications {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        ({ let __r = scheduler::INSTANCE.get_mut(); let __f = compat::dispatch(__r, |__t| __t.schedule_cyclic()); __f(__r, Arc::new(move || { EventNotifications::time_update(); }), 20) });
        skyblock_events::JOIN.register(Arc::new(move || { EventNotifications::refresh_events(); }));
    }

    /// java: public static LiteralArgumentBuilder<FabricClientCommandSource> debugToasts()
    pub fn debug_toasts() -> LiteralArgumentBuilder<crate::unresolved::FabricClientCommandSource> {
        // TODO(fabric2seer): a command builder outside dispatcher.register(..)
        //   java: return ClientCommands.literal("toasts").then(
        //   java: 				ClientCommands.argument("time", IntegerArgumentType.integer(0)).then(
        //   java: 						ClientCommands.argument("duration", IntegerArgumentType.integer(0)).then(
        //   java: 								ClientCommands.argument("jacob", BoolArgumentType.bool()).executes(context -> {
        //   java: 									long time = System.currentTimeMillis() / 1000 + context.getArgument("time", int.class);
        //   java: 									int duration = context.getArgument("duration", int.class);
        //   java: 									if (context.getArgument("jacob", Boolean.class)) {
        //   java: 										Minecraft.getInstance().gui.toastManager().addToast(
        //   java: 												new JacobEventToast(
        //   java: 														time,
        //   java: 														time + duration,
        //   java: 														"Jacob's farming contest",
        //   java: 														List.of("Cactus", "Cocoa Beans", "Pumpkin")
        //   java: 												)
        //   java: 										);
        //   java: 									} else {
        //   java: 										Minecraft.getInstance().gui.toastManager().addToast(
        //   java: 												new EventToast(
        //   java: 														time,
        //   java: 														time + duration,
        //   java: 														"Jacob's or something idk",
        //   java: 														FALLBACK_ICON
        //   java: 												)
        //   java: 										);
        //   java: 									}
        //   java: 									return 0;
        //   java: 								})
        //   java: 						)
        //   java: 				)
        //   java: 		);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: public static Map<String, LinkedList<SkyblockEvent>> getEvents()
    pub fn get_events() -> std::collections::HashMap<String, Vec<SkyblockEvent>> {
        return EVENTS.get_mut().clone();
    }

    /// java: public static void refreshEvents()
    pub fn refresh_events() {
        // CompletableFuture.supplyAsync(.., VIRTUAL_THREAD_EXECUTOR): the download, on the executor
        compat::executor(Box::new(move || {
            let response: Option<String> = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| crate::de::hysky::skyblocker::utils::http::Http::send_get_request("https://hysky.de/api/calendar"))) {
                Ok(response) => Some(response),
                Err(e) => {
                    logger().error(&format!("[Skyblocker] Failed to download events list {}", compat::panic_message(&e)));
                    None // java: return null
                }
            };
            // .thenAcceptAsync(response -> {..}, Minecraft.getInstance()).exceptionally(EventNotifications::itBorked)
            mcrust::mcrust::modding::game::execute(move || {
                let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| EventNotifications::accept_events(response)));
                if let Err(e) = result {
                    EventNotifications::it_borked(&Throwable::new(Some(compat::panic_message(&e))));
                }
            });
        }));
    }

    /// `refreshEvents`' `thenAcceptAsync` body, on the render thread.
    fn accept_events(response: Option<String>) {
        EVENTS.get_mut().clear();
        let Some(response) = response else {
            logger().error("[Skyblocker] Failed to get events list");
            return;
        };
        let mut parsed_events: Vec<SkyblockEvent> = (SKYBLOCK_EVENT_LIST_CODEC.get_mut().parse(&mcrust::com::mojang::serialization::json_ops::JsonOps::instance(), &compat::json_to_port(&serde_json::from_str::<serde_json::Value>(&response).expect("JsonSyntaxException")))).get_partial_or_throw().unwrap();
        for event in parsed_events.iter() {
            if event.start().wrapping_add(event.duration() as i64) < compat::get_state(&CURRENT_TIME) {
                continue;
            }
            (EVENTS.get_mut().entry(event.event()).or_insert_with(Vec::new)).push(event.clone());
        }
        for entry in EVENTS.get_mut().values_mut() {
            // Comparator.comparingLong(SkyblockEvent::start) (List.sort is stable, as `sort_by_key` is)
            entry.sort_by_key(|event| event.start());
        }
        // the lambda runs before `update` returns: the keys it reads are taken first
        let keys: Vec<String> = EVENTS.get().keys().cloned().collect();
        SkyblockerConfigManager::update(Arc::new(move |mut config| {
            for s in keys.iter() {
                config.event_notifications.events.entry(s.clone()).or_insert_with(|| DEFAULT_REMINDERS.clone());
            }
        }));
    }

    /// java: private static Void itBorked(Throwable throwable)
    pub fn it_borked(throwable: &Throwable) -> Option<()> {
        logger().error(&format!("[Skyblocker] Event loading borked, sowwy :("));
        return None;
    }

    /// java: private static void timeUpdate()
    pub fn time_update() {
        let mut new_time: i64 = (compat::millis()).wrapping_div(1000 as i64);
        // java: for (Map.Entry<String, LinkedList<SkyblockEvent>> entry : events.entrySet())
        // (the map's keys first: the body polls the entry's OWN list, and a toast / sound below may run code that
        // reads the map)
        let event_keys: Vec<String> = EVENTS.get().keys().cloned().collect();
        for event_key in event_keys {
            // java: LinkedList<SkyblockEvent> nextEvents = entry.getValue(); SkyblockEvent skyblockEvent = nextEvents.peekFirst();
            //   if (skyblockEvent == null) continue;
            //   // Remove finished event
            //   if (newTime > skyblockEvent.start() + skyblockEvent.duration()) { nextEvents.pollFirst(); skyblockEvent = nextEvents.peekFirst(); if (skyblockEvent == null) continue; }
            let skyblock_event: SkyblockEvent = {
                let events = EVENTS.get_mut();
                let Some(next_events) = events.get_mut(&event_key) else { continue };
                let Some(first) = next_events.first().cloned() else { continue };
                if new_time > first.start().wrapping_add(first.duration() as i64) {
                    next_events.remove(0);
                    match next_events.first().cloned() {
                        Some(next) => next,
                        None => continue,
                    }
                } else {
                    first
                }
            };
            if !(EventNotifications::criterion_met()) {
                continue;
            }
            let mut event_name: String = event_key.clone();
            let mut config: compat::Shared<EventConfig> = SkyblockerConfigManager::get().event_notifications.events.get(&event_name).cloned().unwrap_or_else(|| DEFAULT_REMINDERS.clone());
            if !(compat::read(&config, |__t| __t.enabled)) {
                continue;
            }
            for reminder_time in compat::read(&config, |__t| __t.reminder_times.clone()) {
                if new_time.wrapping_add(reminder_time as i64) < skyblock_event.start() || (compat::get_state(&CURRENT_TIME)).wrapping_add(reminder_time as i64) >= skyblock_event.start() {
                    continue;
                }
                let instance = compat::minecraft();
                if event_name == JACOBS && skyblock_event.extras().left_value().is_some() {
                    instance.gui.toast_manager().borrow_mut().add_toast(Box::new(EventToastAdapter(JacobEventToast::new(skyblock_event.start(), skyblock_event.start().wrapping_add(skyblock_event.duration() as i64), event_name.clone(), skyblock_event.extras().left_value().unwrap()) as compat::Shared<dyn EventToast>)));
                } else {
                    instance.gui.toast_manager().borrow_mut().add_toast(Box::new(EventToastAdapter(EventToastBase::new(skyblock_event.start(), skyblock_event.start().wrapping_add(skyblock_event.duration() as i64), event_name.clone(), EVENT_ICONS.get_mut().get(&event_name).cloned().unwrap_or_else(|| FALLBACK_ICON.get_mut().clone())) as compat::Shared<dyn EventToast>)));
                }
                let mut sound_event: Option<SoundEvent> = SkyblockerConfigManager::get().event_notifications.reminder_sound.get_sound_event();
                if let Some(sound_event) = &sound_event {
                    // TODO(fabric2seer): `play` takes 4 argument(s) in the port (`_al: &OpenAl, library: &mut Library, options: &OptionsData, instance: Box<dyn SoundInstance>`), Java passes 1
                    // rust: instance.get_sound_manager().play(&SimpleSoundInstance::for_ui_with_volume(sound_event, 1.0f32, 1.0f32));
                }
                break;
            }
        }
        compat::set_state(&CURRENT_TIME, new_time);
    }

    /// java: private static boolean criterionMet()
    pub fn criterion_met() -> bool {
        return match SkyblockerConfigManager::get().event_notifications.criterion {
            Criterion::None => false,
            Criterion::Skyblock => Utils::is_on_skyblock(),
            Criterion::Hypixel => Utils::is_on_hypixel(),
            Criterion::Everywhere => true,
            _ => unreachable!("switch expression: no case matched"),
        };
    }

}

/// java: public record SkyblockEvent(long start, int duration, String event, Either<List<String>, JerryPerks> extras, String warpCommand)
#[derive(Clone, Debug, PartialEq)]
pub struct SkyblockEvent {
    /// java: long start
    pub start: i64,
    /// java: int duration
    pub duration: i32,
    /// java: String event
    pub event: String,
    /// java: Either<List<String>, JerryPerks> extras
    pub extras: Either<Vec<String>, JerryPerks>,
    /// java: String warpCommand
    pub warp_command: String,
}

impl SkyblockEvent {
    /// java: the canonical record constructor
    pub fn new(mut start: i64, mut duration: i32, mut event: String, mut extras: Either<Vec<String>, JerryPerks>, mut warp_command: String) -> Self {
        Self { start, duration, event, extras, warp_command }
    }

    /// java: the record accessor `start()`
    pub fn start(&self) -> i64 {
        self.start
    }

    /// java: the record accessor `duration()`
    pub fn duration(&self) -> i32 {
        self.duration
    }

    /// java: the record accessor `event()`
    pub fn event(&self) -> String {
        self.event.clone()
    }

    /// java: the record accessor `extras()`
    pub fn extras(&self) -> Either<Vec<String>, JerryPerks> {
        self.extras.clone()
    }

    /// java: the record accessor `warpCommand()`
    pub fn warp_command(&self) -> String {
        self.warp_command.clone()
    }

}

/// java: public record JerryPerks(String mayorName, List<String> perks)
#[derive(Clone, Debug, PartialEq)]
pub struct JerryPerks {
    /// java: String mayorName
    pub mayor_name: String,
    /// java: List<String> perks
    pub perks: Vec<String>,
}

impl JerryPerks {
    /// java: the canonical record constructor
    pub fn new(mut mayor_name: String, mut perks: Vec<String>) -> Self {
        Self { mayor_name, perks }
    }

    /// java: the record accessor `mayorName()`
    pub fn mayor_name(&self) -> String {
        self.mayor_name.clone()
    }

    /// java: the record accessor `perks()`
    pub fn perks(&self) -> Vec<String> {
        self.perks.clone()
    }

}
