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
                // TODO(fabric2seer): a command tree this tool cannot read (arguments, redirects, requires) - SeerClient commands receive the words after the name in `ctx.args`
                //   java: dispatcher.register(..);
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
        // TODO(fabric2seer): `.thenRunAsync(..)` on a value of unknown type
        //   java: CompletableFuture.supplyAsync(() -> {
        //   java: 			try (Http.ApiResponse response = Http.sendCacheableGetRequest("https://hysky.de/api/skyblock/election", null)) {
        //   java: 				if (!response.ok()) {
        //   java: 					throw new RuntimeException("Received bad http response: " + response.statusCode() + " " + response.content());
        //   java: 				}
        //   java:
        //   java: 				JsonObject json = JsonParser.parseString(response.content()).getAsJsonObject();
        //   java:
        //   java: 				if (!json.get("success").getAsBoolean()) {
        //   java: 					//Can't find a more appropriate exception to throw here.
        //   java: 					throw new RuntimeException("Request failed!");
        //   java: 				}
        //   java:
        //   java: 				JsonObject mayorObject = json.getAsJsonObject("mayor");
        //   java:
        //   java: 				if (mayorObject == null) {
        //   java: 					throw new RuntimeException("No mayor object found in response!");
        //   java: 				}
        //   java:
        //   java: 				return mayorObject;
        //   java: 			} catch (Exception e) {
        //   java: 				throw new RuntimeException(e); //Wrap the exception to be handled by the exceptionally block
        //   java: 			}
        //   java: 		}, SkyblockerMod.VIRTUAL_THREAD_EXECUTOR).exceptionally(throwable -> {
        //   java: 			LOGGER.error("[Skyblocker] Failed to get mayor status!", throwable.getCause());
        //   java: 			if (mayorTickRetryAttempts < 5) {
        //   java: 				int minutes = 5 << mayorTickRetryAttempts; //5, 10, 20, 40, 80 minutes
        //   java: 				mayorTickRetryAttempts++;
        //   java:
        //   java: 				LOGGER.warn("[Skyblocker] Retrying in {} minutes.", minutes);
        //   java: 				RenderHelper.runOnRenderThread(() -> {
        //   java: 					Scheduler.INSTANCE.schedule(MayorUtils::tickMayorCache, minutes * 60 * 20);
        //   java: 				});
        //   java: 			} else {
        //   java: 				LOGGER.warn("[Skyblocker] Failed to get mayor status after 5 retries! Stopping further retries until next reboot.");
        //   java: 			}
        //   java: 			return new JsonObject(); //Have to return a value for the thenAccept block.
        //   java: 		}).thenAccept(result -> {
        //   java: 			if (!result.isEmpty()) {
        //   java: 				try {
        //   java: 					mayor = Mayor.CODEC.parse(JsonOps.INSTANCE, result)
        //   java: 							.setPartial(Mayor.EMPTY)
        //   java: 							.resultOrPartial(error -> LOGGER.warn("[Skyblocker] Failed to parse mayor status from the API response. Error: {}", error))
        //   java: 							.get();
        //   java: 				} catch (Exception e) {
        //   java: 					LOGGER.warn("[Skyblocker] Failed to parse mayor status from the API response.", e);
        //   java: 					mayor = Mayor.EMPTY;
        //   java: 				}
        //   java:
        //   java: 				try {
        //   java: 					JsonObject ministerObject = result.getAsJsonObject("minister");
        //   java:
        //   java: 					// Check if ministerObject is not null stops NPE caused by Derpy
        //   java: 					if (ministerObject != null) {
        //   java: 						minister = Minister.CODEC.parse(JsonOps.INSTANCE, ministerObject)
        //   java: 								.setPartial(Minister.EMPTY)
        //   java: 								.resultOrPartial(error -> LOGGER.warn("[Skyblocker] Failed to parse minister status from the API response. Error: {}", error))
        //   java: 								.get();
        //   java: 					} else {
        //   java: 						LOGGER.info("[Skyblocker] No minister data found for the current mayor.");
        //   java: 						minister = Minister.EMPTY;
        //   java: 					}
        //   java: 				} catch (Exception e) {
        //   java: 					LOGGER.warn("[Skyblocker] Failed to parse minister status from the API response.", e);
        //   java: 					minister = Minister.EMPTY;
        //   java: 				}
        //   java: 				LOGGER.info("[Skyblocker] Mayor set to {}, minister set to {}.", mayor, minister);
        //   java: 			}
        //   java: 		}).thenRunAsync(() -> {
        //   java: 			scheduleMayorTick(); //Ends up as a cyclic task with finer control over scheduled time
        //   java: 			SkyblockEvents.MAYOR_CHANGE.invoker().onMayorChange();
        //   java: 		}, Minecraft.getInstance());
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
        }, std::sync::Arc::new(|__t: Box<dyn FnOnce() + Send>| mcrust::mcrust::modding::game::execute(__t)));
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
