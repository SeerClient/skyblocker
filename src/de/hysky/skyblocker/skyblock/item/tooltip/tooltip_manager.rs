//! Ported from de.hysky.skyblocker.skyblock.item.tooltip.TooltipManager by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::bazaar::bazaar_order_tracker;
use crate::de::hysky::skyblocker::skyblock::bazaar::reorder_helper::ReorderHelper;
use crate::de::hysky::skyblocker::skyblock::chocolatefactory::chocolate_factory_solver;
use crate::de::hysky::skyblocker::skyblock::dungeon::croesus_profit;
use crate::de::hysky::skyblocker::skyblock::dwarven::fossil::fossil_solver::FossilSolver;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::adders::accessory_tooltip::AccessoryTooltip;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::adders::avg_bin_tooltip::AvgBinTooltip;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::adders::bazaar_price_tooltip::BazaarPriceTooltip;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::adders::bits_helper;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::adders::color_tooltip::ColorTooltip;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::adders::craft_price_tooltip::CraftPriceTooltip;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::adders::date_calculator_tooltip::DateCalculatorTooltip;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::adders::dungeon_quality_tooltip::DungeonQualityTooltip;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::adders::essence_shop_price::EssenceShopPrice;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::adders::estimated_item_value_tooltip::EstimatedItemValueTooltip;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::adders::evolving_item_progress_tooltip::EvolvingItemProgressTooltip;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::adders::george_price_tooltip::GeorgePriceTooltip;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::adders::hunting_box_price_tooltip::HuntingBoxPriceTooltip;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::adders::l_bin_tooltip::LBinTooltip;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::adders::line_smoothener::LineSmoothener;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::adders::motes_tooltip::MotesTooltip;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::adders::museum_tooltip::MuseumTooltip;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::adders::npc_price_tooltip::NpcPriceTooltip;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::adders::obtained_date_tooltip::ObtainedDateTooltip;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::adders::stacking_enchant_progress_tooltip::StackingEnchantProgressTooltip;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::adders::supercraft_reminder::SupercraftReminder;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::adders::true_hex_display::TrueHexDisplay;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::adders::true_hex_dye_screen_display::TrueHexDyeScreenDisplay;
use crate::de::hysky::skyblocker::utils::container::container_matcher::ContainerMatcher;
use crate::de::hysky::skyblocker::utils::container::tooltip_adder::TooltipAdder;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::net::minecraft::client::gui::screens::inventory::abstract_container_screen::AbstractContainerScreen;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::world::inventory::slot::Slot;

/// java: private static final TooltipAdder[] adders = new TooltipAdder[]{ new LineSmoothener(), // Applies before anything else new TrueHexDisplay(), new TrueHexDyeScreenDisplay(), new SupercraftReminder(), Ch
pub static ADDERS: compat::MainThread<Vec<compat::Shared<dyn TooltipAdder>>> = compat::MainThread::new(|| vec![(LineSmoothener::new() as compat::Shared<dyn TooltipAdder>), (TrueHexDisplay::new() as compat::Shared<dyn TooltipAdder>), (TrueHexDyeScreenDisplay::new() as compat::Shared<dyn TooltipAdder>), (SupercraftReminder::new() as compat::Shared<dyn TooltipAdder>), (chocolate_factory_solver::INSTANCE.get_mut().clone() as compat::Shared<dyn TooltipAdder>), (bits_helper::INSTANCE.get_mut().clone() as compat::Shared<dyn TooltipAdder>), (FossilSolver::new() as compat::Shared<dyn TooltipAdder>), (ReorderHelper::new() as compat::Shared<dyn TooltipAdder>), (bazaar_order_tracker::INSTANCE.get_mut().clone() as compat::Shared<dyn TooltipAdder>), (StackingEnchantProgressTooltip::new(0) as compat::Shared<dyn TooltipAdder>), (EvolvingItemProgressTooltip::new(0) as compat::Shared<dyn TooltipAdder>), (NpcPriceTooltip::new(1) as compat::Shared<dyn TooltipAdder>), (GeorgePriceTooltip::new(2) as compat::Shared<dyn TooltipAdder>), (BazaarPriceTooltip::new(3) as compat::Shared<dyn TooltipAdder>), (LBinTooltip::new(4) as compat::Shared<dyn TooltipAdder>), (AvgBinTooltip::new(5) as compat::Shared<dyn TooltipAdder>), (EssenceShopPrice::new(6) as compat::Shared<dyn TooltipAdder>), (CraftPriceTooltip::new(7) as compat::Shared<dyn TooltipAdder>), (EstimatedItemValueTooltip::new(8) as compat::Shared<dyn TooltipAdder>), (DungeonQualityTooltip::new(9) as compat::Shared<dyn TooltipAdder>), (MotesTooltip::new(10) as compat::Shared<dyn TooltipAdder>), (ObtainedDateTooltip::new(11) as compat::Shared<dyn TooltipAdder>), (MuseumTooltip::new(12) as compat::Shared<dyn TooltipAdder>), (ColorTooltip::new(13) as compat::Shared<dyn TooltipAdder>), (AccessoryTooltip::new(14) as compat::Shared<dyn TooltipAdder>), (DateCalculatorTooltip::new(15) as compat::Shared<dyn TooltipAdder>), (HuntingBoxPriceTooltip::new(16) as compat::Shared<dyn TooltipAdder>), (croesus_profit::INSTANCE.get_mut().clone() as compat::Shared<dyn TooltipAdder>)]);
/// java: private static List<TooltipAdder> currentScreenAdders = new ArrayList<>();
pub static CURRENT_SCREEN_ADDERS: compat::MainThread<Vec<compat::Shared<dyn TooltipAdder>>> = compat::MainThread::new(|| Vec::new());

/// java: public class TooltipManager
#[derive(Clone)]
pub struct TooltipManager {}

impl TooltipManager {
    /// java: private TooltipManager()
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        item_tooltip_callback::EVENT.register(Arc::new(|stack: &ItemStack, _context: &TooltipContext<'_>, flag: TooltipFlag, lines: &mut Vec<FormattedText>| {
            // `lines` is the tooltip's `List<Component>`: the adders edit it as components and it goes back as the
            // port's `FormattedText` lines.
            let mut components: Vec<MutableComponent> = lines.drain(..).map(compat::component_of).collect();
            if compat::container_screen_open() {
                // addToTooltip(((AbstractContainerScreenAccessor) handledScreen).getFocusedSlot(), stack, lines);
                // rust: port gap - `AbstractContainerScreen.hoveredSlot` is not readable by a mod here: the screen object is
                // not published and the tooltip is built under the game's player lock (`extract_render_state_tail`), so
                // neither the screen nor `player.containerMenu` answer. GAPS_batchB.md.
                TooltipManager::add_to_tooltip(None, &stack, &mut components);
            } else {
                TooltipManager::add_to_tooltip(None, &stack, &mut components);
            }
            lines.extend(components.into_iter().map(mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from));
        }));
        screen_events::AFTER_INIT.register(Arc::new(|screen: &mut Screen, width: i32, height: i32| {
            TooltipManager::on_screen_change(&screen);
            // TODO(fabric2seer): ScreenEvents.remove(..).register: no SeerClient equivalent yet
            //   java: ScreenEvents.remove(screen).register(_ -> currentScreenAdders = List.of());
        }));
    }

    /// java: private static void onScreenChange(Screen screen)
    pub fn on_screen_change(screen: &mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        *CURRENT_SCREEN_ADDERS.get_mut() = compat::sorted_by(ADDERS.get_mut().iter().cloned().filter(|__a0| ({ let __f = compat::dispatch(__a0, |__t| __t.is_enabled()); __f(&compat::view(__a0, |a| a as compat::Shared<dyn ContainerMatcher>)) })).filter(|adder| ({ let __f = compat::dispatch(adder, |__t| __t.test()); __f(&compat::view(adder, |a| a as compat::Shared<dyn ContainerMatcher>), screen) })), |a, b| { let key = |__a0: &compat::Shared<dyn TooltipAdder>| ({ let __f = compat::dispatch(__a0, |__t| __t.get_priority()); __f(__a0) }); key(a).cmp(&key(b)) }).collect::<Vec<_>>();
    }

    /// java: private static List<Component> addToTooltip(@Nullable Slot focusedSlot, ItemStack stack, List<Component> lines)
    /// `lines` is edited in place (Java's list); the returned list is the same lines.
    pub fn add_to_tooltip(mut focused_slot: Option<Slot>, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, lines: &mut Vec<MutableComponent>) -> Vec<MutableComponent> {
        if !(Utils::is_on_skyblock()) {
            return lines.clone();
        }
        let adders = CURRENT_SCREEN_ADDERS.get().clone();
        for adder in adders.iter() {
            ({ let __f = compat::dispatch(adder, |__t| __t.add_to_tooltip()); __f(adder, focused_slot.clone(), stack, lines) });
        }
        return lines.clone();
    }

}
