//! Ported from de.hysky.skyblocker.utils.mayor.MayorUtils by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::events::skyblock_events;
use crate::de::hysky::skyblocker::utils::mayor::mayor;
use crate::de::hysky::skyblocker::utils::mayor::mayor::Mayor;
use crate::de::hysky::skyblocker::utils::mayor::minister;
use crate::de::hysky::skyblocker::utils::mayor::minister::Minister;
use crate::de::hysky::skyblocker::utils::render::render_helper::RenderHelper;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler::Scheduler;
use crate::de::hysky::skyblocker::utils::time::skyblock_time::SkyblockTime;
use crate::entry::skyblocker_mod;
use mcrust::com::mojang::serialization::codec;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::com::mojang::serialization::codecs::record_codec_builder;
use mcrust::java::util::concurrent::completable_future::CompletableFuture;
use mcrust::com::mojang::serialization::decoder::Decoder as _;

/// java: private static Mayor mayor = Mayor.EMPTY;
pub static MAYOR: LazyLock<Mutex<Mayor>> = LazyLock::new(|| Mutex::new(mayor::EMPTY.clone()));
/// java: private static Minister minister = Minister.EMPTY;
pub static MINISTER: LazyLock<Mutex<Minister>> = LazyLock::new(|| Mutex::new(minister::EMPTY.clone()));
/// java: private static List<PerkOverride> mayorPerkOverrides = List.of();
pub static MAYOR_PERK_OVERRIDES: LazyLock<Mutex<Vec<PerkOverride>>> = LazyLock::new(|| Mutex::new(vec![]));
/// java: private static boolean mayorTickScheduled = false;
pub static MAYOR_TICK_SCHEDULED: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));
/// java: private static int mayorTickRetryAttempts = 0;
pub static MAYOR_TICK_RETRY_ATTEMPTS: LazyLock<Mutex<i32>> = LazyLock::new(|| Mutex::new(0));
/// java: private static final Codec<PerkOverride> CODEC = RecordCodecBuilder.create(instance -> instance.group( Codec.STRING.fieldOf("perk").forGetter(PerkOverride::perk), Codec.LONG.fieldOf("from").forGetter(
/// (read on the executor's thread by `loadMayorPerkOverrides`: a codec is `Send + Sync`, so a `LazyLock`)
pub static PERK_OVERRIDE_CODEC: LazyLock<CodecRef<PerkOverride>> = LazyLock::new(|| record_codec_builder::create(record_codec_builder::apply3(record_codec_builder::of(|o: &PerkOverride| o.perk(), codec::field_of(mcrust::com::mojang::serialization::codec::string_codec(), "perk")), record_codec_builder::of(|o: &PerkOverride| o.from(), codec::field_of(mcrust::com::mojang::serialization::codec::long_codec(), "from")), record_codec_builder::of(|o: &PerkOverride| o.to(), codec::field_of(mcrust::com::mojang::serialization::codec::long_codec(), "to")), move |__a0, __a1, __a2| PerkOverride::new(__a0.clone(), __a1.clone(), __a2.clone()))));
/// java: private static final Codec<List<PerkOverride>> LIST_CODEC = CODEC.listOf();
/// (read on the executor's thread: a `LazyLock`, see `PERK_OVERRIDE_CODEC`)
pub static PERK_OVERRIDE_LIST_CODEC: LazyLock<CodecRef<Vec<PerkOverride>>> = LazyLock::new(|| codec::list(PERK_OVERRIDE_CODEC.clone()));

/// java: public class MayorUtils
#[derive(Clone)]
pub struct MayorUtils {}

impl MayorUtils {
    /// java: private MayorUtils()
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static List<String> getActivePerks()
    pub fn get_active_perks() -> Vec<String> {
        let mut mayor_perks = compat::lock(&MAYOR).perks().into_iter().map(|__a0| __a0.name());
        let mut minister_perk = vec![compat::lock(&MINISTER).perk().name()].into_iter();
        let mut overridden_mayor_perks = compat::lock(&MAYOR_PERK_OVERRIDES).iter().cloned().filter(|__a0| __a0.is_active()).map(|__a0| __a0.perk()).collect::<Vec<String>>().into_iter();
        return mayor_perks.chain(minister_perk).chain(overridden_mayor_perks).collect::<Vec<_>>();
    }

    /// java: @Init public static void init()
    pub fn init() {
        skyblock_events::JOIN.register(Arc::new(move || {
            if !(compat::get_state(&MAYOR_TICK_SCHEDULED)) {
                MayorUtils::tick_mayor_cache();
                MayorUtils::schedule_mayor_tick();
                compat::set_state(&MAYOR_TICK_SCHEDULED, true);
            }
        }));
        client_command_registration_callback::EVENT.register(Arc::new(
            |dispatcher: &mut client_command_registration_callback::ClientCommandDispatcher| {
                //   java: dispatcher.register(ClientCommands.literal(SkyblockerMod.NAMESPACE).then(ClientCommands.literal("updatePerkOverrides").executes(context -> {
                //   java:     loadMayorPerkOverrides().thenAcceptAsync(result -> { if (minecraft.player != null) sendSystemMessage(PREFIX + (result ? ..success : ..failed)); }, minecraft);
                //   java:     return Command.SINGLE_SUCCESS; })));
                use mcrust::com::mojang::brigadier::builder::argument_builder::ArgumentBuilder as _;
                let mut update = client_command_registration_callback::literal("updatePerkOverrides");
                update.executes(Arc::new(|_context: &mcrust::com::mojang::brigadier::context::command_context::CommandContext<client_command_registration_callback::FabricClientCommandSource>| {
                    MayorUtils::load_mayor_perk_overrides().then_accept(|result: &bool| {
                        let result: bool = *result;
                        crate::compat::execute(move || {
                            if mcrust::mcrust::modding::game::with_player(|_| ()).is_some() {
                                let key = if result { "skyblocker.mayor.perkOverrides.update.success" } else { "skyblocker.mayor.perkOverrides.update.failed" };
                                compat::batch_w_send_feedback(&(crate::de::hysky::skyblocker::utils::constants::PREFIX.get())().append(mcrust::net::minecraft::network::chat::component::translatable(key)));
                            }
                        });
                    });
                    Ok(1)
                }));
                let mut root = client_command_registration_callback::literal(skyblocker_mod::NAMESPACE);
                root.then(&update);
                dispatcher.dispatcher().register(&root);
            },
        ));
    }

    /// java: private static void scheduleMayorTick()
    pub fn schedule_mayor_tick() {
        let mut current_year_millis: i64 = (SkyblockTime::get_skyblock_millis()).wrapping_rem(446400000);
        let mut millis_until_next_mayor_change: i64 = (if current_year_millis > 105600000 { (446400000i64.wrapping_sub(current_year_millis)).wrapping_add(105600000) } else { 105600000i64.wrapping_sub(current_year_millis) });
        ({ let __r = scheduler::INSTANCE.get_mut(); let __f = compat::dispatch(__r, |__t| __t.schedule()); __f(__r, Arc::new(move || { MayorUtils::tick_mayor_cache(); }), ((millis_until_next_mayor_change.wrapping_div(50 as i64)) as i32).wrapping_add((5i32.wrapping_mul(60)).wrapping_mul(20))) });
        ({ let __r = scheduler::INSTANCE.get_mut(); let __f = compat::dispatch(__r, |__t| __t.schedule()); __f(__r, Arc::new(move || {
            compat::set_state(&MAYOR, mayor::EMPTY.clone());
            compat::set_state(&MINISTER, minister::EMPTY.clone());
            logger().info(&format!("[Skyblocker] Mayor set to {}, minister set to {}.", compat::jstring(&compat::get_state(&MAYOR)), compat::jstring(&compat::get_state(&MINISTER))));
            (skyblock_events::mayor_change_invoker())();
        }), ((millis_until_next_mayor_change.wrapping_div(50 as i64)) as i32)) });
    }

    /// java: private static void tickMayorCache()
    pub fn tick_mayor_cache() {
        MayorUtils::load_mayor_perk_overrides();
        // java: CompletableFuture.supplyAsync(() -> { try (Http.ApiResponse response = Http.sendCacheableGetRequest("https://hysky.de/api/skyblock/election", null)) { .. return mayorObject; } catch (Exception e) { throw new RuntimeException(e); } }, VIRTUAL_THREAD_EXECUTOR)
        //       .exceptionally(throwable -> { LOGGER.error(..); retry; return new JsonObject(); })
        //       .thenAccept(result -> { if (!result.isEmpty()) { mayor = ..; minister = ..; LOGGER.info(..); } })
        //       .thenRunAsync(() -> { scheduleMayorTick(); SkyblockEvents.MAYOR_CHANGE.invoker().onMayorChange(); }, Minecraft.getInstance());
        // The request and the JSON parse run on the executor; the codecs are the render thread's, so the parse into `Mayor` /
        // `Minister` and the rest run there (`Minecraft.getInstance()` is the last stage's executor anyway).
        compat::executor(Box::new(|| {
            let fetched: Result<serde_json::Map<String, serde_json::Value>, String> = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| -> Result<serde_json::Map<String, serde_json::Value>, String> {
                let response = crate::de::hysky::skyblocker::utils::http::Http::send_cacheable_get_request("https://hysky.de/api/skyblock/election", None);
                if !response.ok() {
                    return Err(format!("Received bad http response: {} {}", response.status_code(), response.content()));
                }
                let json: serde_json::Value = serde_json::from_str(&response.content()).map_err(|e| e.to_string())?;
                let json = json.as_object().ok_or_else(|| "Not a JSON object".to_string())?;
                if !json.get("success").and_then(|s| s.as_bool()).unwrap_or(false) {
                    // Can't find a more appropriate exception to throw here.
                    return Err("Request failed!".to_string());
                }
                let Some(mayor_object) = json.get("mayor").and_then(|m| m.as_object()) else {
                    return Err("No mayor object found in response!".to_string());
                };
                Ok(mayor_object.clone())
            })).unwrap_or_else(|e| Err(compat::panic_message(&e)));
            crate::compat::execute(move || {
                // java: .exceptionally(throwable -> {..})
                let result: serde_json::Map<String, serde_json::Value> = match fetched {
                    Ok(result) => result,
                    Err(cause) => {
                        logger().error(&format!("[Skyblocker] Failed to get mayor status! {cause}"));
                        let attempts: i32 = compat::get_state(&MAYOR_TICK_RETRY_ATTEMPTS);
                        if attempts < 5 {
                            let minutes: i32 = 5 << attempts; // 5, 10, 20, 40, 80 minutes
                            compat::set_state(&MAYOR_TICK_RETRY_ATTEMPTS, attempts + 1);
                            logger().warn(&format!("[Skyblocker] Retrying in {} minutes.", minutes));
                            // java: RenderHelper.runOnRenderThread(() -> Scheduler.INSTANCE.schedule(MayorUtils::tickMayorCache, minutes * 60 * 20));
                            ({ let __r = scheduler::INSTANCE.get_mut(); let __f = compat::dispatch(__r, |__t| __t.schedule()); __f(__r, Arc::new(move || { MayorUtils::tick_mayor_cache(); }), minutes * 60 * 20) });
                        } else {
                            logger().warn("[Skyblocker] Failed to get mayor status after 5 retries! Stopping further retries until next reboot.");
                        }
                        serde_json::Map::new()
                    }
                };
                // java: .thenAccept(result -> {..})
                if !result.is_empty() {
                    let ops = mcrust::com::mojang::serialization::json_ops::JsonOps::instance();
                    let result_value = serde_json::Value::Object(result.clone());
                    let parsed_mayor: Mayor = match mayor::CODEC.get().parse(&ops, &compat::json_to_port(&result_value)).get_or_throw() {
                        Ok(parsed) => parsed,
                        Err(error) => {
                            logger().warn(&format!("[Skyblocker] Failed to parse mayor status from the API response. Error: {}", error));
                            mayor::EMPTY.clone()
                        }
                    };
                    compat::set_state(&MAYOR, parsed_mayor);
                    // Check if ministerObject is not null stops NPE caused by Derpy
                    let parsed_minister: Minister = match result.get("minister").and_then(|m| m.as_object()) {
                        Some(minister_object) => match minister::CODEC.get().parse(&ops, &compat::json_to_port(&serde_json::Value::Object(minister_object.clone()))).get_or_throw() {
                            Ok(parsed) => parsed,
                            Err(error) => {
                                logger().warn(&format!("[Skyblocker] Failed to parse minister status from the API response. Error: {}", error));
                                minister::EMPTY.clone()
                            }
                        },
                        None => {
                            logger().info("[Skyblocker] No minister data found for the current mayor.");
                            minister::EMPTY.clone()
                        }
                    };
                    compat::set_state(&MINISTER, parsed_minister);
                    logger().info(&format!("[Skyblocker] Mayor set to {:?}, minister set to {:?}.", compat::get_state(&MAYOR), compat::get_state(&MINISTER)));
                }
                // java: .thenRunAsync(() -> {..}, Minecraft.getInstance())
                MayorUtils::schedule_mayor_tick(); // Ends up as a cyclic task with finer control over scheduled time
                (skyblock_events::mayor_change_invoker())();
            });
        }));
    }

    /// java: private static CompletableFuture<Boolean> loadMayorPerkOverrides()
    pub fn load_mayor_perk_overrides() -> mcrust::java::util::concurrent::completable_future::CompletableFuture<bool> {
        return (CompletableFuture::supply_async(move || Ok((|| {
            // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`; the body's `return` is the closure's value
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                //   java: String response = Http.sendGetRequest("https://hysky.de/api/mayorperkoverrides");
                let mut response: String = crate::de::hysky::skyblocker::utils::http::Http::send_get_request("https://hysky.de/api/mayorperkoverrides");
                compat::set_state(&MAYOR_PERK_OVERRIDES, (PERK_OVERRIDE_LIST_CODEC.parse(&mcrust::com::mojang::serialization::json_ops::JsonOps::instance(), &compat::json_to_port(&serde_json::from_str::<serde_json::Value>(&response).expect("JsonSyntaxException")))).get_or_throw().unwrap());
                logger().info(&format!("[Skyblocker] Loaded {} mayor perk overrides.", (compat::lock(&MAYOR_PERK_OVERRIDES).len() as i32)));
                return true;
            })) {
                Ok(__v) => return __v,
                Err(e) => {
                    logger().error(&format!("[Skyblocker] Failed to load mayor perk overrides."));
                    return false;
                }
            }
        })()), compat::executor)).then_apply_async(|loaded: &bool| {
            let loaded: bool = *loaded;
            if loaded {
                (skyblock_events::mayor_change_invoker())();
            }
            return loaded;
        }, std::sync::Arc::new(|__t: Box<dyn FnOnce() + Send>| crate::compat::execute(__t)));
    }

}

/// java: private record PerkOverride(String perk, long from, long to)
#[derive(Clone, Debug, PartialEq)]
pub struct PerkOverride {
    /// java: String perk
    pub perk: String,
    /// java: long from
    pub from: i64,
    /// java: long to
    pub to: i64,
}

impl PerkOverride {
    /// java: the canonical record constructor
    pub fn new(mut perk: String, mut from: i64, mut to: i64) -> Self {
        Self { perk, from, to }
    }

    /// java: the record accessor `perk()`
    pub fn perk(&self) -> String {
        self.perk.clone()
    }

    /// java: the record accessor `from()`
    pub fn from(&self) -> i64 {
        self.from
    }

    /// java: the record accessor `to()`
    pub fn to(&self) -> i64 {
        self.to
    }

    /// java: public boolean isActive()
    pub fn is_active(&self) -> bool {
        let mut now: i64 = compat::millis();
        return (now >= self.from && now <= self.to);
    }

}
