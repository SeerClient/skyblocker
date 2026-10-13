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
        let mut price_string: String = compat::string_format("%1$,.1f", &[(if pre_counted { price / (count as f64) } else { price }).into()]);
        if count == 1 {
            return (component::literal(&format!("{} Coins", price_string))).with_style_format(ChatFormatting::DarkAqua);
        }
        let mut price_string_total: String = compat::string_format("%1$,.1f", &[(if pre_counted { price } else { price * (count as f64) }).into()]);
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
            if { let __m = compat::get_state(&MINUTE) + 1; compat::set_state(&MINUTE, __m); __m % 60 == 0 } {
                compat::set_state(&SENT_NULL_WARNING, false);
            }
            // CompletableFuture.allOf(Stream.of(NPC, BAZAAR, LOWEST_BINS, ONE_DAY_AVERAGE, THREE_DAY_AVERAGE, MOTES, MUSEUM,
            //   COLOR, ACCESSORIES, GEORGE).map(DataTooltipInfoType::downloadIfEnabled)..)
            //   .thenRunAsync(ItemPriceUpdateEvent.ON_PRICE_UPDATE.invoker()::onPriceUpdate, Minecraft.getInstance())
            //   .exceptionally(e -> { LOGGER.error(.., e); return null; });
            let downloads = vec![
                compat::write(crate::de::hysky::skyblocker::skyblock::item::tooltip::info::tooltip_info_type::NPC.get(), |t| t.download_if_enabled()),
                compat::write(crate::de::hysky::skyblocker::skyblock::item::tooltip::info::tooltip_info_type::BAZAAR.get(), |t| t.download_if_enabled()),
                compat::write(crate::de::hysky::skyblocker::skyblock::item::tooltip::info::tooltip_info_type::LOWEST_BINS.get(), |t| t.download_if_enabled()),
                compat::write(crate::de::hysky::skyblocker::skyblock::item::tooltip::info::tooltip_info_type::ONE_DAY_AVERAGE.get(), |t| t.download_if_enabled()),
                compat::write(crate::de::hysky::skyblocker::skyblock::item::tooltip::info::tooltip_info_type::THREE_DAY_AVERAGE.get(), |t| t.download_if_enabled()),
                compat::write(crate::de::hysky::skyblocker::skyblock::item::tooltip::info::tooltip_info_type::MOTES.get(), |t| t.download_if_enabled()),
                compat::write(crate::de::hysky::skyblocker::skyblock::item::tooltip::info::tooltip_info_type::MUSEUM.get(), |t| t.download_if_enabled()),
                compat::write(crate::de::hysky::skyblocker::skyblock::item::tooltip::info::tooltip_info_type::COLOR.get(), |t| t.download_if_enabled()),
                compat::write(crate::de::hysky::skyblocker::skyblock::item::tooltip::info::tooltip_info_type::ACCESSORIES.get(), |t| t.download_if_enabled()),
                compat::write(crate::de::hysky::skyblocker::skyblock::item::tooltip::info::tooltip_info_type::GEORGE.get(), |t| t.download_if_enabled()),
            ];
            mcrust::java::util::concurrent::completable_future::CompletableFuture::all_of(&downloads)
                .then_run_async(
                    || (crate::de::hysky::skyblocker::events::item_price_update_event::on_price_update_invoker())(),
                    std::sync::Arc::new(|__t: Box<dyn FnOnce() + Send>| mcrust::mcrust::modding::game::execute(__t)),
                )
                .exceptionally(|e| {
                    logger().error(&format!("[Skyblocker] Encountered unknown error while downloading tooltip data: {}", e));
                });
            CraftPriceTooltip::clear_cache();
        }), 1200, true) });
    }

}
