//! Ported from de.hysky.skyblocker.skyblock.item.tooltip.ItemTooltip by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::adders::craft_price_tooltip::CraftPriceTooltip;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::info::data_tooltip_info_type::DataTooltipInfoType;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::info::tooltip_info_type;
use crate::de::hysky::skyblocker::utils::constants;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler::Scheduler;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::java::text::decimal_format_symbols::Locale;
use mcrust::java::util::concurrent::completable_future::CompletableFuture;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: public static final Supplier<GeneralConfig.ItemTooltip> config = () -> SkyblockerConfigManager.get().general.itemTooltip;
pub static CONFIG: compat::MainThread<Arc<dyn Fn() -> compat::Shared<crate::de::hysky::skyblocker::config::configs::general_config::ItemTooltip>>> = compat::MainThread::new(|| Arc::new(move || SkyblockerConfigManager::get().general.item_tooltip.clone()));
/// java: private static volatile boolean sentNullWarning = false;
pub static SENT_NULL_WARNING: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));
/// java: public static int minute = 0;
pub static MINUTE: LazyLock<Mutex<i32>> = LazyLock::new(|| Mutex::new(0));

/// java: public class ItemTooltip
#[derive(Clone)]
pub struct ItemTooltip {}

impl ItemTooltip {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Deprecated(since = "1.22.0", forRemoval = true) public static String getNeuName(String id, String apiId)
    pub fn get_neu_name(id: &str, api_id: &str) -> String {
        logger().error("[Skyblocker Item Tooltip] ItemTooltip.getNeuName is deprecated and will not work. Use ItemStack#getNeuName instead.");
        return "".to_string();
    }

    /// java: public static void nullWarning()
    pub fn null_warning() {
        if !(compat::get_state(&SENT_NULL_WARNING)) && mcrust::mcrust::modding::game::with_player(|_| ()).is_some() {
            logger().warn(&(constants::PREFIX.get_mut())().append(component::translatable("skyblocker.itemTooltip.nullMessage").with_style_format(ChatFormatting::Red)).get_string());
            compat::set_state(&SENT_NULL_WARNING, true);
        }
    }

    /// java: public static Component getCoinsMessage(double price, int count)
    pub fn get_coins_message(mut price: f64, mut count: i32) -> MutableComponent {
        return ItemTooltip::get_coins_message_3(price, count, false);
    }

    /// java: public static Component getCoinsMessage(double price, int count, boolean preCounted)
    pub fn get_coins_message_3(mut price: f64, mut count: i32, mut pre_counted: bool) -> MutableComponent {
        let mut price_string: String = format!("%1$,.1f");
        if count == 1 {
            return (component::literal(&format!("{} Coins", price_string))).with_style_format(ChatFormatting::DarkAqua);
        }
        let mut price_string_total: String = format!("%1$,.1f");
        return (component::literal(&format!("{} Coins ", price_string_total))).with_style_format(ChatFormatting::DarkAqua).append((component::literal(&format!("({} each)", price_string))).with_style_format(ChatFormatting::Gray));
    }

    /// java: @Init public static void init()
    pub fn init() {
        ({ let __r = scheduler::INSTANCE.get_mut(); let __f = compat::dispatch(__r, |__t| __t.schedule_cyclic_3()); __f(__r, Arc::new(move || {
            if !(Utils::is_on_skyblock()) && 0 < compat::get_state(&MINUTE) {
                compat::set_state(&SENT_NULL_WARNING, false);
                return;
            }
            // TODO(fabric2seer): the condition: ++/-- inside an expression
            //   java: ++minute % 60 == 0
            if todo!("fabric2seer: the condition (TODO above)") {
                compat::set_state(&SENT_NULL_WARNING, false);
            }
            // TODO(fabric2seer): `.cast(..)` on a Class
            //   java: CompletableFuture.allOf(Stream.of(
            //   java: 							TooltipInfoType.NPC,
            //   java: 							TooltipInfoType.BAZAAR,
            //   java: 							TooltipInfoType.LOWEST_BINS,
            //   java: 							TooltipInfoType.ONE_DAY_AVERAGE,
            //   java: 							TooltipInfoType.THREE_DAY_AVERAGE,
            //   java: 							TooltipInfoType.MOTES,
            //   java: 							TooltipInfoType.MUSEUM,
            //   java: 							TooltipInfoType.COLOR,
            //   java: 							TooltipInfoType.ACCESSORIES,
            //   java: 							TooltipInfoType.GEORGE)
            //   java: 					.map(DataTooltipInfoType.class::cast)
            //   java: 					.map(DataTooltipInfoType::downloadIfEnabled)
            //   java: 					.toArray(CompletableFuture[]::new)
            //   java: 			).thenRunAsync(ItemPriceUpdateEvent.ON_PRICE_UPDATE.invoker()::onPriceUpdate, Minecraft.getInstance()
            //   java: 			).exceptionally(e -> {
            //   java: 				LOGGER.error("[Skyblocker] Encountered unknown error while downloading tooltip data", e);
            //   java: 				return null;
            //   java: 			});
            CraftPriceTooltip::clear_cache();
        }), 1200, true) });
    }

}
