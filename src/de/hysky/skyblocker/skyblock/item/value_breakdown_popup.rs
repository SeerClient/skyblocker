//! Ported from de.hysky.skyblocker.skyblock.item.ValueBreakdownPopup by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::formatters;
use crate::de::hysky::skyblocker::utils::networth::networth_calculator::NetworthCalculator;
use crate::de::hysky::skyblocker::utils::neu_repo_manager::NEURepoManager;
use crate::de::hysky::skyblocker::utils::render::gui::abstract_popup_screen::AbstractPopupScreen;
use crate::de::hysky::skyblocker::utils::render::gui::abstract_popup_screen::AbstractPopupScreenBase;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::mcrust::modding::screens::ModScreen;
use mcrust::mcrust::modding::screens::ScreenRenderContext;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::components::multi_line_text_widget::MultiLineTextWidget;
use mcrust::net::minecraft::client::gui::components::scrollable_layout::ScrollableLayoutElement;
use mcrust::net::minecraft::client::gui::components::scrollable_layout::ReserveStrategy;
use mcrust::net::minecraft::client::gui::font::Font;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::layouts::layout;
use mcrust::net::minecraft::client::gui::layouts::layout_element::LayoutElement;
use mcrust::net::minecraft::client::gui::layouts::linear_layout::LinearLayout;
use mcrust::net::minecraft::client::gui::layouts::spacer_element::SpacerElement;
use mcrust::net::minecraft::client::gui::screens::inventory::abstract_container_screen::AbstractContainerScreen;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::client::input::mouse_button_info::MouseButtonInfo;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::hover_event::HoverEvent;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::world::inventory::slot::Slot;
use mcrust::net::minecraft::network::chat::component::Component as _;

//   java: public static final KeyMapping KEY_BINDING = KeyMappingHelper.registerKeyMapping(new KeyMapping(
//   java: 			"key.skyblocker.valueBreadownPopup",
//   java: 			InputConstants.KEY_I,
//   java: 			SkyblockerMod.KEYBINDING_CATEGORY
//   java: 	));
/// java: public static final KeyMapping KEY_BINDING = KeyMappingHelper.registerKeyMapping(new KeyMapping( "key.skyblocker.valueBreadownPopup", InputConstants.KEY_I, SkyblockerMod.KEYBINDING_CATEGORY ));
pub static KEY_BINDING: compat::MainThread<mcrust::net::minecraft::client::key_mapping::KeyMapping> = compat::MainThread::new(|| key_binding_helper::register_key_binding(mcrust::net::minecraft::client::key_mapping::KeyMapping::new("key.skyblocker.valueBreadownPopup", mcrust::com::mojang::blaze3d::platform::input_constants::KEY_I, *crate::entry::skyblocker_mod::KEYBINDING_CATEGORY.get())));
/// java: private static final Function<String, Component> EMPTY = _ -> Component.empty();
pub static EMPTY: compat::MainThread<Arc<dyn Fn(String) -> MutableComponent>> = compat::MainThread::new(|| Arc::new(move |_| component::empty()));
/// java: private static final Function<String, Component> ITEM_NAME = s -> { NEUItem neuItem = NEURepoManager.getItemByNeuId(s); if (neuItem == null) return Component.literal(s); return TextTransformer.fromLeg
pub static ITEM_NAME: compat::MainThread<Arc<dyn Fn(String) -> MutableComponent>> = compat::MainThread::new(|| Arc::new(move |s| {
    let Some(mut neu_item) = NEURepoManager::get_item_by_neu_id(&s) else {
        return component::literal(&s);
    };
    // return TextTransformer.fromLegacy(neuItem.getDisplayName());
    return crate::de::hysky::skyblocker::utils::text_transformer::TextTransformer::from_legacy(&neu_item.get_display_name());
}));
/// java: private static final LayoutAppender EMPTY_APPENDER = (_, _, _) -> {};
pub static EMPTY_APPENDER: compat::MainThread<compat::Shared<dyn LayoutAppender>> = compat::MainThread::new(|| compat::shared(LayoutAppenderLambda(Arc::new(move |_, _, _, _| {

}))) as compat::Shared<dyn LayoutAppender>);
//   java: private static final Map<Calculation.Type, LayoutAppender> FORMATTERS = new EnumMap<>(Map.ofEntries(
//   java: 			Map.entry(Calculation.Type.STAR, new BasicListAppender(
//   java: 					Component.literal("Star Upgrades"),
//   java: 					ITEM_NAME
//   java: 			)),
//   java: 			// TODO Prestige (idk how it works)
//   java: 			Map.entry(Calculation.Type.GOD_ROLL, new BasicSingleAppender(
//   java: 					Component.literal("God Roll"),
//   java: 					ITEM_NAME
//   java: 			)),
//   java: 			Map.entry(Calculation.Type.SHEN_AUCTION, EMPTY_APPENDER),
//   java: 			Map.entry(Calculation.Type.WINNING_BID, EMPTY_APPENDER),
//   java: 			Map.entry(Calculation.Type.ENCHANTMENT, new BasicListAppender(
//   java: 					Component.literal("Enchantments"),
//   java: 					s -> {
//   java: 						String neuId = ItemRepository.getBazaarStocks().get("ENCHANTMENT_" + s);
//   java: 						if (neuId == null) return Component.literal(s);
//   java: 						NEUItem neuItem = NEURepoManager.getItemByNeuId(neuId);
//   java: 						if (neuItem == null) return Component.literal(s);
//   java: 						String enchantName = EnchantedBookUtils.getEnchantNameFromLore(neuItem.getLore());
//   java: 						return TextTransformer.fromLegacy(enchantName);
//   java: 					}
//   java: 			)),
//   java: 			Map.entry(Calculation.Type.SKIN, new BasicSingleAppender(
//   java: 					Component.literal("Skin"),
//   java: 					s -> {
//   java: 						NEUItem neuItem = NEURepoManager.getItemByNeuId(s);
//   java: 						if (neuItem == null) neuItem = NEURepoManager.getItemByNeuId("PET_SKIN_" + s);
//   java: 						if (neuItem == null) return Component.literal(s);
//   java: 						return TextTransformer.fromLegacy(neuItem.getDisplayName());
//   java: 					}
//   java: 			)),
//   java: 			Map.entry(Calculation.Type.SILEX, new BasicSingleAppender(
//   java: 					Component.literal("Silex"),
//   java: 					EMPTY
//   java: 			)),
//   java: 			Map.entry(Calculation.Type.GOLDEN_BOUNTY, new BasicSingleAppender(
//   java: 					Component.literal("Golden Bounty"),
//   java: 					EMPTY
//   java: 			)),
//   java: 			// TODO Attributes
//   java: 			Map.entry(Calculation.Type.POCKET_SACK_IN_A_SACK, new BasicSingleAppender(
//   java: 					Component.literal("Pocket Sack in a sack"),
//   java: 					EMPTY
//   java: 			)),
//   java: 			Map.entry(Calculation.Type.WOOD_SINGULARITY, new BasicSingleAppender(
//   java: 					Component.literal("Wood Singularity"),
//   java: 					EMPTY
//   java: 			)),
//   java: 			Map.entry(Calculation.Type.JALAPENO_BOOK, new BasicSingleAppender(
//   java: 					Component.literal("Jalapeno Book"),
//   java: 					EMPTY
//   java: 			)),
//   java: 			Map.entry(Calculation.Type.TRANSMISSION_TUNER, new BasicSingleAppender(
//   java: 					Component.literal("Transmission Tuner"),
//   java: 					EMPTY
//   java: 			)),
//   java: 			Map.entry(Calculation.Type.MANA_DISINTEGRATOR, new BasicSingleAppender(
//   java: 					Component.literal("Mana Disintegrator"),
//   java: 					EMPTY
//   java: 			)),
//   java: 			Map.entry(Calculation.Type.THUNDER_IN_A_BOTTLE, new BasicSingleAppender(
//   java: 					Component.literal("Thunder in a Bottle"),
//   java: 					EMPTY
//   java: 			)),
//   java: 			Map.entry(Calculation.Type.RUNE, new BasicSingleAppender(
//   java: 					Component.literal("Rune"),
//   java: 					ITEM_NAME
//   java: 			)),
//   java: 			Map.entry(Calculation.Type.FUMING_POTATO_BOOK, new BasicSingleAppender(
//   java: 					Component.literal("Fuming Potato Book"),
//   java: 					EMPTY
//   java: 			)),
//   java: 			Map.entry(Calculation.Type.HOT_POTATO_BOOK, new BasicSingleAppender(
//   java: 					Component.literal("Hot Potato Book"),
//   java: 					EMPTY
//   java: 			)),
//   java: 			Map.entry(Calculation.Type.DYE, new BasicSingleAppender(
//   java: 					Component.literal("Dye"),
//   java: 					ITEM_NAME
//   java: 			)),
//   java: 			Map.entry(Calculation.Type.ART_OF_WAR, new BasicSingleAppender(
//   java: 					Component.literal("Art of War"),
//   java: 					EMPTY
//   java: 			)),
//   java: 			Map.entry(Calculation.Type.ART_OF_PEACE, new BasicSingleAppender(
//   java: 					Component.literal("Art of Peace"),
//   java: 					EMPTY
//   java: 			)),
//   java: 			Map.entry(Calculation.Type.FARMING_FOR_DUMMIES, new BasicSingleAppender(
//   java: 					Component.literal("Farming For Dummies"),
//   java: 					EMPTY
//   java: 			)),
//   java: 			Map.entry(Calculation.Type.TALISMAN_ENRICHMENT, new BasicSingleAppender(
//   java: 					Component.literal("Talisman Enrichment"),
//   java: 					EMPTY
//   java: 			)),
//   java: 			Map.entry(Calculation.Type.RECOMBOBULATOR, new BasicSingleAppender(
//   java: 					Component.literal("Recombobulator"),
//   java: 					EMPTY
//   java: 			)),
//   java: 			Map.entry(Calculation.Type.GEMSTONE_SLOT, new BasicSingleAppender(
//   java: 					Component.literal("Gemstone Slot"),
//   java: 					EMPTY
//   java: 			)),
//   java: 			Map.entry(Calculation.Type.GEMSTONE, new BasicListAppender(
//   java: 					Component.literal("Gemstones"),
//   java: 					ITEM_NAME
//   java: 			)),
//   java: 			Map.entry(Calculation.Type.POWER_SCROLL, new BasicSingleAppender(
//   java: 					Component.literal("Power Scroll"),
//   java: 					ITEM_NAME
//   java: 			)),
//   java: 			Map.entry(Calculation.Type.REFORGE, new BasicSingleAppender(
//   java: 					Component.literal("Reforge"),
//   java: 					s -> {
//   java: 						//noinspection UnstableApiUsage
//   java: 						String neuId = ItemConstants.REFORGES.get(s);
//   java: 						if (neuId == null) return Component.literal(s);
//   java: 						return ITEM_NAME.apply(neuId);
//   java: 					}
//   java: 			)),
//   java: 			Map.entry(Calculation.Type.MASTER_STAR, new BasicListAppender(
//   java: 					Component.literal("Master Stars"),
//   java: 					ITEM_NAME
//   java: 			)),
//   java: 			Map.entry(Calculation.Type.WITHER_BLADE_SCROLL, new BasicListAppender(
//   java: 					Component.literal("Wither Blade Scrolls"),
//   java: 					ITEM_NAME
//   java: 			)),
//   java: 			Map.entry(Calculation.Type.DRILL_PART, new BasicListAppender(
//   java: 					Component.literal("Drill Parts"),
//   java: 					ITEM_NAME
//   java: 			)),
//   java: 			Map.entry(Calculation.Type.POLARVOID_BOOK, new BasicSingleAppender(
//   java: 					Component.literal("Polarvoid Book"),
//   java: 					EMPTY
//   java: 			)),
//   java: 			Map.entry(Calculation.Type.DIVAN_POWDER_COATING, new BasicSingleAppender(
//   java: 					Component.literal("Divan Powder Coating"),
//   java: 					EMPTY
//   java: 			)),
//   java: 			Map.entry(Calculation.Type.ETHERWARP_CONDUIT, new BasicSingleAppender(
//   java: 					Component.literal("Etherwarp Conduit"),
//   java: 					EMPTY
//   java: 			)),
//   java: 			Map.entry(Calculation.Type.NEW_YEAR_CAKES, new BasicSingleAppender(
//   java: 					Component.literal("New Year Cakes"),
//   java: 					EMPTY
//   java: 			)),
//   java: 			Map.entry(Calculation.Type.PET_ITEM, new BasicSingleAppender(
//   java: 					Component.literal("Pet Item"),
//   java: 					ITEM_NAME
//   java: 			))
//   java: 	));
/// java: private static final Map<Calculation.Type, LayoutAppender> FORMATTERS = new EnumMap<>(Map.ofEntries( Map.entry(Calculation.Type.STAR, new BasicListAppender( Component.literal("Star Upgrades"), ITEM_NA
pub static FORMATTERS: compat::MainThread<std::collections::HashMap<crate::net::azureaaron::networth::calculation::Type, compat::Shared<dyn LayoutAppender>>> = compat::MainThread::new(|| {
    use crate::net::azureaaron::networth::calculation::Type;
    std::collections::HashMap::from([
        (Type::Star, compat::shared(BasicListAppender::new(component::literal("Star Upgrades"), ITEM_NAME.get().clone())) as compat::Shared<dyn LayoutAppender>),
        // TODO Prestige (idk how it works)
        (Type::GodRoll, compat::shared(BasicSingleAppender::new(component::literal("God Roll"), ITEM_NAME.get().clone())) as compat::Shared<dyn LayoutAppender>),
        (Type::ShenAuction, EMPTY_APPENDER.get().clone()),
        (Type::WinningBid, EMPTY_APPENDER.get().clone()),
        (Type::Enchantment, compat::shared(BasicListAppender::new(component::literal("Enchantments"), Arc::new(|s: String| {
            let Some(neu_id) = crate::de::hysky::skyblocker::skyblock::itemlist::item_repository::ItemRepository::get_bazaar_stocks().get(&format!("ENCHANTMENT_{}", s)).cloned() else {
                return component::literal(&s);
            };
            let Some(neu_item) = NEURepoManager::get_item_by_neu_id(&neu_id) else {
                return component::literal(&s);
            };
            // return TextTransformer.fromLegacy(EnchantedBookUtils.getEnchantNameFromLore(neuItem.getLore()));
            crate::de::hysky::skyblocker::utils::text_transformer::TextTransformer::from_legacy(&crate::de::hysky::skyblocker::utils::enchanted_book_utils::EnchantedBookUtils::get_enchant_name_from_lore(&neu_item.get_lore()))
        }))) as compat::Shared<dyn LayoutAppender>),
        (Type::Skin, compat::shared(BasicSingleAppender::new(component::literal("Skin"), Arc::new(|s: String| {
            let mut neu_item = NEURepoManager::get_item_by_neu_id(&s);
            if neu_item.is_none() {
                neu_item = NEURepoManager::get_item_by_neu_id(&format!("PET_SKIN_{}", s));
            }
            let Some(neu_item) = neu_item else {
                return component::literal(&s);
            };
            // return TextTransformer.fromLegacy(neuItem.getDisplayName());
            crate::de::hysky::skyblocker::utils::text_transformer::TextTransformer::from_legacy(&neu_item.get_display_name())
        }))) as compat::Shared<dyn LayoutAppender>),
        (Type::Silex, compat::shared(BasicSingleAppender::new(component::literal("Silex"), EMPTY.get().clone())) as compat::Shared<dyn LayoutAppender>),
        (Type::GoldenBounty, compat::shared(BasicSingleAppender::new(component::literal("Golden Bounty"), EMPTY.get().clone())) as compat::Shared<dyn LayoutAppender>),
        // TODO Attributes
        (Type::PocketSackInASack, compat::shared(BasicSingleAppender::new(component::literal("Pocket Sack in a sack"), EMPTY.get().clone())) as compat::Shared<dyn LayoutAppender>),
        (Type::WoodSingularity, compat::shared(BasicSingleAppender::new(component::literal("Wood Singularity"), EMPTY.get().clone())) as compat::Shared<dyn LayoutAppender>),
        (Type::JalapenoBook, compat::shared(BasicSingleAppender::new(component::literal("Jalapeno Book"), EMPTY.get().clone())) as compat::Shared<dyn LayoutAppender>),
        (Type::TransmissionTuner, compat::shared(BasicSingleAppender::new(component::literal("Transmission Tuner"), EMPTY.get().clone())) as compat::Shared<dyn LayoutAppender>),
        (Type::ManaDisintegrator, compat::shared(BasicSingleAppender::new(component::literal("Mana Disintegrator"), EMPTY.get().clone())) as compat::Shared<dyn LayoutAppender>),
        (Type::ThunderInABottle, compat::shared(BasicSingleAppender::new(component::literal("Thunder in a Bottle"), EMPTY.get().clone())) as compat::Shared<dyn LayoutAppender>),
        (Type::Rune, compat::shared(BasicSingleAppender::new(component::literal("Rune"), ITEM_NAME.get().clone())) as compat::Shared<dyn LayoutAppender>),
        (Type::FumingPotatoBook, compat::shared(BasicSingleAppender::new(component::literal("Fuming Potato Book"), EMPTY.get().clone())) as compat::Shared<dyn LayoutAppender>),
        (Type::HotPotatoBook, compat::shared(BasicSingleAppender::new(component::literal("Hot Potato Book"), EMPTY.get().clone())) as compat::Shared<dyn LayoutAppender>),
        (Type::Dye, compat::shared(BasicSingleAppender::new(component::literal("Dye"), ITEM_NAME.get().clone())) as compat::Shared<dyn LayoutAppender>),
        (Type::ArtOfWar, compat::shared(BasicSingleAppender::new(component::literal("Art of War"), EMPTY.get().clone())) as compat::Shared<dyn LayoutAppender>),
        (Type::ArtOfPeace, compat::shared(BasicSingleAppender::new(component::literal("Art of Peace"), EMPTY.get().clone())) as compat::Shared<dyn LayoutAppender>),
        (Type::FarmingForDummies, compat::shared(BasicSingleAppender::new(component::literal("Farming For Dummies"), EMPTY.get().clone())) as compat::Shared<dyn LayoutAppender>),
        (Type::TalismanEnrichment, compat::shared(BasicSingleAppender::new(component::literal("Talisman Enrichment"), EMPTY.get().clone())) as compat::Shared<dyn LayoutAppender>),
        (Type::Recombobulator, compat::shared(BasicSingleAppender::new(component::literal("Recombobulator"), EMPTY.get().clone())) as compat::Shared<dyn LayoutAppender>),
        (Type::GemstoneSlot, compat::shared(BasicSingleAppender::new(component::literal("Gemstone Slot"), EMPTY.get().clone())) as compat::Shared<dyn LayoutAppender>),
        (Type::Gemstone, compat::shared(BasicListAppender::new(component::literal("Gemstones"), ITEM_NAME.get().clone())) as compat::Shared<dyn LayoutAppender>),
        (Type::PowerScroll, compat::shared(BasicSingleAppender::new(component::literal("Power Scroll"), ITEM_NAME.get().clone())) as compat::Shared<dyn LayoutAppender>),
        (Type::Reforge, compat::shared(BasicSingleAppender::new(component::literal("Reforge"), Arc::new(|s: String| {
            //noinspection UnstableApiUsage
            let Some(neu_id) = crate::net::azureaaron::networth::utils::item_constants::REFORGES.get(s.as_str()) else {
                return component::literal(&s);
            };
            (ITEM_NAME.get())(neu_id.to_string())
        }))) as compat::Shared<dyn LayoutAppender>),
        (Type::MasterStar, compat::shared(BasicListAppender::new(component::literal("Master Stars"), ITEM_NAME.get().clone())) as compat::Shared<dyn LayoutAppender>),
        (Type::WitherBladeScroll, compat::shared(BasicListAppender::new(component::literal("Wither Blade Scrolls"), ITEM_NAME.get().clone())) as compat::Shared<dyn LayoutAppender>),
        (Type::DrillPart, compat::shared(BasicListAppender::new(component::literal("Drill Parts"), ITEM_NAME.get().clone())) as compat::Shared<dyn LayoutAppender>),
        (Type::PolarvoidBook, compat::shared(BasicSingleAppender::new(component::literal("Polarvoid Book"), EMPTY.get().clone())) as compat::Shared<dyn LayoutAppender>),
        (Type::DivanPowderCoating, compat::shared(BasicSingleAppender::new(component::literal("Divan Powder Coating"), EMPTY.get().clone())) as compat::Shared<dyn LayoutAppender>),
        (Type::EtherwarpConduit, compat::shared(BasicSingleAppender::new(component::literal("Etherwarp Conduit"), EMPTY.get().clone())) as compat::Shared<dyn LayoutAppender>),
        (Type::NewYearCakes, compat::shared(BasicSingleAppender::new(component::literal("New Year Cakes"), EMPTY.get().clone())) as compat::Shared<dyn LayoutAppender>),
        (Type::PetItem, compat::shared(BasicSingleAppender::new(component::literal("Pet Item"), ITEM_NAME.get().clone())) as compat::Shared<dyn LayoutAppender>),
    ])
});

/// java: public class ValueBreakdownPopup extends AbstractPopupScreen
pub struct ValueBreakdownPopup {
    /// java: extends AbstractPopupScreen
    pub base: AbstractPopupScreenBase,
    /// java: private final NetworthResult networthResult;
    pub networth_result: crate::net::azureaaron::networth::networth_result::NetworthResult,
    /// java: private final EnumMap<Calculation.Type, List<Calculation>> map;
    pub map: std::collections::BTreeMap<crate::net::azureaaron::networth::calculation::Type, Vec<crate::net::azureaaron::networth::calculation::Calculation>>,
    /// java: private ScrollableLayout scrollable;
    /// (the port's `ScrollableLayout` layout is `ScrollableLayoutElement`; its scroll container is not a widget of a
    /// mod screen, so the content does not scroll - GAPS_O)
    pub scrollable: Option<ScrollableLayoutElement>,
}

impl std::ops::Deref for ValueBreakdownPopup {
    type Target = AbstractPopupScreenBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for ValueBreakdownPopup {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl ValueBreakdownPopup {
    /// java: protected ValueBreakdownPopup(Screen backgroundScreen, NetworthResult networthResult)
    pub fn new(mut background_screen: mcrust::net::minecraft::client::gui::screens::screen::Screen, mut networth_result: crate::net::azureaaron::networth::networth_result::NetworthResult) -> compat::Shared<Self> {
        let __init_networth_result = networth_result;
        //   java: for (Calculation calculation : networthResult.calculations()) map.computeIfAbsent(calculation.type(), _ -> new ArrayList<>()).add(calculation);
        // (an EnumMap iterates in ordinal order: a BTreeMap over the enum's declaration-order `Ord`)
        let mut __init_map: std::collections::BTreeMap<crate::net::azureaaron::networth::calculation::Type, Vec<crate::net::azureaaron::networth::calculation::Calculation>> = std::collections::BTreeMap::new();
        for calculation in __init_networth_result.calculations().iter() {
            __init_map.entry(calculation.r#type()).or_insert_with(Vec::new).push(calculation.clone());
        }
        //   java: super(Component.translatable("skyblocker.valueBreakdownPopup"), backgroundScreen);
        //   java: this.map = new EnumMap<>(Calculation.Type.class);
        // (open it with `game::set_screen_keeping_parent`: the screen it is opened over is its background screen, which
        // the base keeps a stand-in for - see `AbstractPopupScreenBase::new_in`)
        let (_, __this) = AbstractPopupScreenBase::new_in(component::translatable("skyblocker.valueBreakdownPopup"), background_screen, move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                networth_result: __init_networth_result,
                map: __init_map,
                scrollable: None,
            });
            ((__h.clone() as compat::Shared<dyn AbstractPopupScreen>), __h)
        });
        // (the map was filled from `networthResult.calculations()` before the struct was built, above)
        __this
    }

    /// java: @Init public static void initClass()
    /// `ScreenEvents.AFTER_INIT` + `ScreenKeyboardEvents.afterKeyPress(screen)` on an `AbstractContainerScreen`: Fabric
    /// fires `afterKeyPress` after the screen's own `keyPressed`, so the listener is a RETURN injector on
    /// `AbstractContainerScreen#keyPressed` - the one place the screen (`getFocusedSlot()`, i.e. `hoveredSlot`) is lent.
    pub fn init_class() {
        // the class's static initialiser runs when `initClass` first touches the class: the key mapping registers then
        let _ = KEY_BINDING.get();
        let result = mcrust::mcrust::modding::hook::inject("net.minecraft.client.gui.screens.inventory.AbstractContainerScreen#keyPressed", mcrust::mcrust::modding::hook::At::Return, |ci: &mut mcrust::mcrust::modding::hook::CallbackInfo<'_>| {
            // ScreenEvents.AFTER_INIT: if (!Utils.isOnSkyblock()) return; if (screen instanceof AbstractContainerScreen<?>)
            if !(Utils::is_on_skyblock()) {
                return;
            }
            let Some(key) = ci.find_arg::<KeyEvent>().cloned() else { return };
            // if (!KEY_BINDING.matches(key)) return;
            if !(mcrust::net::minecraft::client::key_mapping::matches("key.skyblocker.valueBreadownPopup", &key)) {
                return;
            }
            // Slot slot = ((AbstractContainerScreenAccessor) handledScreen).getFocusedSlot(); - the port's `hoveredSlot` is the
            // slot's index in the open menu
            let Some(hovered) = ci.this::<mcrust::net::minecraft::client::gui::screens::inventory::abstract_container_screen::AbstractContainerScreen>().and_then(|screen| screen.hovered_slot) else { return };
            let slot: Option<Slot> = compat::with_open_menu(|menu, _| (hovered >= 0).then(|| menu.slots.get(hovered as usize).cloned()).flatten()).flatten();
            // if (slot == null || !slot.hasItem()) return;
            let Some(slot) = slot else { return };
            if !(compat::slot_has_item(&slot)) {
                return;
            }
            // NetworthResult networth = NetworthCalculator.getItemNetworth(slot.getItem());
            let networth: crate::net::azureaaron::networth::networth_result::NetworthResult = crate::de::hysky::skyblocker::utils::networth::networth_calculator::NetworthCalculator::get_item_networth(&compat::slot_item(&slot));
            // if (networth.price() > 0) client.gui.setScreen(new ValueBreakdownPopup(screen, networth));
            // (the popup is opened over the container screen, which `set_screen_keeping_parent` keeps as its background)
            if networth.price() > 0.0 {
                let popup = ValueBreakdownPopup::new(mcrust::net::minecraft::client::gui::screens::screen::Screen::new(String::new()), networth);
                mcrust::mcrust::modding::game::set_screen_keeping_parent(Box::new(compat::ScreenAdapter::new(popup)));
            }
        });
        if let Err(e) = result {
            logger().error(&format!("[Skyblocker] Failed to hook the value breakdown key: {:?}", e));
        }
    }

    /// java: private static Component getCoinsText(double price)
    pub fn get_coins_text(mut price: f64) -> MutableComponent {
        return ValueBreakdownPopup::get_coins_text_2(price, ((0) as f64) as f64);
    }

    /// java: private static Component getCoinsText(double price, double totalPrice)
    pub fn get_coins_text_2(mut price: f64, mut total_price: f64) -> MutableComponent {
        let mut text: MutableComponent = (component::translatable_args("skyblocker.valueBreakdownPopup.coins", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of(formatters::FLOAT_NUMBERS.get_mut().format(price))])).with_style_format(ChatFormatting::Gold);
        if total_price > (0 as f64) {
            // text.withStyle(..) - MutableComponent.withStyle sets the text's own style
            text = text.with_style(&mcrust::net::minecraft::network::chat::style::Style::EMPTY.with_hover_event(Some(HoverEvent::new_show_text(mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::translatable_args("skyblocker.valueBreakdownPopup.totalPricePercent", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of(formatters::FLOAT_NUMBERS.get_mut().format(((price / total_price) * (100 as f64))))]))))));
        }
        return text;
    }

    /// java: private static MultiLineTextWidget createTextWidget(Component text, Font textRenderer)
    /// (the port's layouts hold a screen's widgets by index: the widget joins `screen`, the popup's own `Screen`
    /// half, and the layout gets its element - Java's `visitWidgets(this::addRenderableWidget)` marks it drawn)
    pub fn create_text_widget(screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, text: &MutableComponent, text_renderer: &Font) -> LayoutElement {
        // java: MultiLineTextWidget widget = new MultiLineTextWidget(text, textRenderer);
        let mut widget: MultiLineTextWidget = MultiLineTextWidget::new(0, 0, mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(text.clone()));
        // java: widget.setComponentClickHandler(_ -> {});
        // (the handler is the widget's press: `OnPress::ComponentClick` below)
        let (width, height) = (widget.get_width(&text_renderer.splitter()), widget.get_height(&text_renderer.splitter()));
        let index = screen.add_widget(mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget::new(
            0,
            0,
            width,
            height,
            text.get_string(),
            mcrust::net::minecraft::client::gui::components::abstract_widget::WidgetKind::PlainMultiLineText(Box::new(widget)),
            mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress::ComponentClick(mcrust::net::minecraft::client::gui::components::abstract_widget::StyleClickLambda::new(|_| {})),
        ));
        screen.mark_renderable(index);
        return LayoutElement::widget(index, width, height);
    }

    /// java: @Override public void extractRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_render_state(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        ();
        gui_graphics_extractor::centered_text(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of(compat::read(__this, |__t| __t.title.clone())), (compat::read(__this, |__t| __t.width)).wrapping_div(2), 15, mcrust::net::minecraft::util::common_colors::WHITE);
    }

}

impl AbstractPopupScreen for ValueBreakdownPopup {
    fn abstract_popup_screen_base(&self) -> &AbstractPopupScreenBase {
        &self.base
    }
    fn abstract_popup_screen_base_mut(&mut self) -> &mut AbstractPopupScreenBase {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override protected void init() (dispatch: the body for this class)
    fn init(&self) -> fn(&compat::Shared<dyn AbstractPopupScreen>) {
        ValueBreakdownPopup::init_dyn
    }

    /// java: @Override protected void repositionElements() (dispatch: the body for this class)
    fn reposition_elements(&self) -> fn(&compat::Shared<dyn AbstractPopupScreen>) {
        ValueBreakdownPopup::reposition_elements_dyn
    }

    /// java: @Override public void extractBackground(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a) (dispatch: the body for this class)
    fn extract_background(&self) -> fn(&compat::Shared<dyn AbstractPopupScreen>, &mut GuiGraphicsExtractor, i32, i32, f32) {
        ValueBreakdownPopup::extract_background_dyn
    }

}

impl ValueBreakdownPopup {
    /// java: @Override protected void init()
    pub fn init_impl(__this: &compat::Shared<Self>) {
        let mut layout: LinearLayout = LinearLayout::vertical();
        //   java: layout.addChild(createTextWidget(Component.translatable("skyblocker.valueBreakdownPopup.baseItemPrice", getCoinsText(networthResult.base(), networthResult.price())), font));
        {
            let (__base, __price) = compat::read(__this, |__t| (__t.networth_result.base(), __t.networth_result.price()));
            let __text = component::translatable_args("skyblocker.valueBreakdownPopup.baseItemPrice", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(ValueBreakdownPopup::get_coins_text_2(__base, __price))]);
            let __e = compat::write(__this, |__t| ValueBreakdownPopup::create_text_widget(&mut __t.base.base, &__text, &compat::font()));
            layout.add_child(__e);
        }
        let __entries: Vec<(crate::net::azureaaron::networth::calculation::Type, Vec<crate::net::azureaaron::networth::calculation::Calculation>)> = compat::read(__this, |__t| __t.map.iter().map(|(k, v)| (k.clone(), v.clone())).collect());
        let __networth_result = compat::read(__this, |__t| __t.networth_result.clone());
        for entry in __entries {
            let mut appender: Option<compat::Shared<dyn LayoutAppender>> = FORMATTERS.get_mut().get(&entry.0).cloned();
            // java: if (appender == EMPTY_APPENDER) continue; - a null appender is not the empty one
            if appender.as_ref().is_some_and(|__a| std::ptr::addr_eq(std::sync::Arc::as_ptr(__a), std::sync::Arc::as_ptr(EMPTY_APPENDER.get_mut()))) {
                continue;
            }
            layout.add_child(SpacerElement::height(5).as_layout_element());
            if let Some(appender) = &appender {
                ({ let __f = compat::dispatch(appender, |__t| __t.append_to()); compat::write(__this, |__t| __f(appender, &__networth_result, &entry.1, &mut layout, &mut __t.base.base)) });
                continue;
            }
            { let __e = compat::write(__this, |__t| ValueBreakdownPopup::create_text_widget(&mut __t.base.base, &component::literal(&compat::jstring(&entry.0)), &compat::font())); layout.add_child(__e); }
            for calculation in entry.1.iter() {
                //   java: layout.addChild(createTextWidget(Component.literal(calculation.id() + ": ").append(getCoinsText(calculation.price())), font), p -> p.paddingLeft(20));
                let __text = component::literal(&format!("{}: ", calculation.id())).append(ValueBreakdownPopup::get_coins_text(calculation.price()));
                let __e = compat::write(__this, |__t| ValueBreakdownPopup::create_text_widget(&mut __t.base.base, &__text, &compat::font()));
                layout.add_child_adjusted(__e, &mut |p| { *p = p.clone().padding_left(20); });
            }
        }
        layout.add_child(SpacerElement::height(10).as_layout_element());
        //   java: layout.addChild(createTextWidget(Component.translatable("skyblocker.valueBreakdownPopup.total", getCoinsText(networthResult.price())), font), LayoutSettings::alignHorizontallyRight);
        {
            let __text = component::translatable_args("skyblocker.valueBreakdownPopup.total", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(ValueBreakdownPopup::get_coins_text(__networth_result.price()))]);
            let __e = compat::write(__this, |__t| ValueBreakdownPopup::create_text_widget(&mut __t.base.base, &__text, &compat::font()));
            layout.add_child_adjusted(__e, &mut |p| { *p = p.clone().align_horizontally_right(); });
        }
        // java: scrollable = new ScrollableLayout(minecraft, layout, 200); - the three-argument constructor's strategy is BOTH
        { let __v = Some(ScrollableLayoutElement::new(LayoutElement::Grid(layout.wrapped.clone()), 200, ReserveStrategy::Both)); compat::write(__this, |__t| __t.scrollable = __v); }
        // java: layout.visitWidgets(this::addRenderableWidget); - the widgets joined this screen when the layout took them
        { let mut __vw = Vec::new(); LayoutElement::Grid(layout.wrapped.clone()).visit_widgets(&mut __vw); compat::write(__this, |__t| for (__i, _, _) in __vw { __t.base.base.mark_renderable(__i); }); }
        <dyn AbstractPopupScreen>::init_body(&compat::view(__this, |a| a as compat::Shared<dyn AbstractPopupScreen>));
    }

    /// ValueBreakdownPopup.init as the dispatch of AbstractPopupScreen.init
    pub fn init_dyn(__this: &compat::Shared<dyn AbstractPopupScreen>) {
        Self::init_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override protected void repositionElements()
    pub fn reposition_elements_impl(__this: &compat::Shared<Self>) {
        <dyn AbstractPopupScreen>::reposition_elements_body(&compat::view(__this, |a| a as compat::Shared<dyn AbstractPopupScreen>));
        // java: scrollable.arrangeElements();
        // java: scrollable.setPosition((width - scrollable.getWidth()) / 2, (height - scrollable.getHeight()) / 2);
        compat::write(__this, |__t| {
            let (__width, __height) = (__t.width, __t.height);
            let __s = &mut *__t;
            let __l = __s.scrollable.as_mut().unwrap();
            __l.arrange_elements();
            let (__x, __y) = ((__width.wrapping_sub(__l.get_width())).wrapping_div(2), (__height.wrapping_sub(__l.get_height())).wrapping_div(2));
            __l.set_x(__x);
            __l.set_y(__y);
            // the layout's widgets are this screen's: they take the places the layout gave them
            let mut __placed = Vec::new();
            __l.visit_widgets(&mut __placed);
            for (__i, __wx, __wy) in __placed {
                __s.base.base.widget_mut(__i).set_position(__wx, __wy);
            }
        });
    }

    /// ValueBreakdownPopup.repositionElements as the dispatch of AbstractPopupScreen.repositionElements
    pub fn reposition_elements_dyn(__this: &compat::Shared<dyn AbstractPopupScreen>) {
        Self::reposition_elements_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public void extractBackground(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_background_impl(__this: &compat::Shared<Self>, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        <dyn AbstractPopupScreen>::extract_background_body(&compat::view(__this, |a| a as compat::Shared<dyn AbstractPopupScreen>), graphics, mouse_x, mouse_y, a);
        let (x, y, width, height) = compat::read(__this, |__t| { let __l = __t.scrollable.as_ref().unwrap(); (__l.get_x(), __l.get_y(), __l.get_width(), __l.get_height()) });
        AbstractPopupScreenBase::extract_popup_background(graphics, x, y, width, height);
    }

    /// ValueBreakdownPopup.extractBackground as the dispatch of AbstractPopupScreen.extractBackground
    pub fn extract_background_dyn(__this: &compat::Shared<dyn AbstractPopupScreen>, __p0: &mut GuiGraphicsExtractor, __p1: i32, __p2: i32, __p3: f32) {
        Self::extract_background_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2, __p3)
    }

}

/// ValueBreakdownPopup's `Screen` half (`super`'s fields and widget tree), lent by the game for each call
impl compat::ScreenHalf for ValueBreakdownPopup {
    fn screen_half(&mut self) -> &mut mcrust::net::minecraft::client::gui::screens::screen::Screen {
        &mut self.base.base
    }
    fn screen_parts(&mut self) -> (&mut mcrust::net::minecraft::client::gui::screens::screen::Screen, Option<&mut compat::WidgetCache>) {
        let __s = &mut self.base;
        (&mut __s.base, Some(&mut __s.__widgets))
    }
}

/// ValueBreakdownPopup as the game's screen: `game::set_screen(Box::new(compat::ScreenAdapter::new(..)))`
impl ModScreen for compat::ScreenAdapter<ValueBreakdownPopup> {
    /// java: AbstractPopupScreen.extractBackground overrides `Screen.extractBackground` without `super`
    fn extract_background_calls_super(&self) -> bool {
        false
    }

    /// java: @Override public void extractBackground(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_background(&mut self, _screen: &mcrust::net::minecraft::client::gui::screens::screen::Screen, context: &mut mcrust::mcrust::modding::screens::ScreenRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        self.call(|__h| ({ let __f = compat::dispatch(__h, |__t| __t.extract_background()); __f(&compat::view(__h, |a| a as compat::Shared<dyn AbstractPopupScreen>), &mut context.graphics, mouse_x, mouse_y, a) }))
    }

    fn get_title(&self) -> String {
        self.title()
    }

    /// java: @Override protected void init()
    fn init(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        self.lend(screen, |__h| ({ let __f = compat::dispatch(__h, |__t| __t.init()); __f(&compat::view(__h, |a| a as compat::Shared<dyn AbstractPopupScreen>)) }))
    }

    /// java: @Override protected void repositionElements()
    fn reposition_elements(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        self.lend(screen, |__h| ({ let __f = compat::dispatch(__h, |__t| __t.reposition_elements()); __f(&compat::view(__h, |a| a as compat::Shared<dyn AbstractPopupScreen>)) }))
    }

    /// java: Screen.tick (the base body)
    fn tick(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        self.lend(screen, |_| ())
    }

    /// java: @Override public void added()
    fn added(&mut self) {
        self.call(|__h| ({ let __f = compat::dispatch(__h, |__t| __t.added()); __f(&compat::view(__h, |a| a as compat::Shared<dyn AbstractPopupScreen>)) }))
    }

    /// java: @Override public void extractRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_render_state(&mut self, screen: &mcrust::net::minecraft::client::gui::screens::screen::Screen, context: &mut ScreenRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        self.lend_ref(screen, |__h| ValueBreakdownPopup::extract_render_state(__h, &context.graphics, mouse_x, mouse_y, a))
    }

    /// java: Screen.keyPressed (the base body)
    fn key_pressed(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, event: &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        self.lend(screen, |_| ());
        if event.is_escape() && self.should_close_on_esc() {
            self.on_close();
            return true;
        }
        false
    }

    /// java: Screen.mouseClicked (the base body)
    fn mouse_clicked(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, x: f64, y: f64, button: i32) -> bool {
        self.lend(screen, |_| false)
    }

    /// java: @Override public void onClose()
    fn on_close(&mut self) {
        self.call(|__h| ({ let __f = compat::dispatch(__h, |__t| __t.on_close()); __f(&compat::view(__h, |a| a as compat::Shared<dyn AbstractPopupScreen>)) }))
    }

}

/// java: public interface LayoutAppender
pub trait LayoutAppender: compat::DynAny {
    fn as_any(&self) -> &dyn std::any::Any;
    /// java: void appendTo(NetworthResult networthResult, List<Calculation> calculations, LinearLayout layout); (dispatch: the body for this class)
    fn append_to(&self) -> fn(&compat::Shared<dyn LayoutAppender>, &crate::net::azureaaron::networth::networth_result::NetworthResult, &Vec<crate::net::azureaaron::networth::calculation::Calculation>, &mut LinearLayout, &mut mcrust::net::minecraft::client::gui::screens::screen::Screen);

    /// java: default void appendCountAndPrice(Calculation calc, double totalPrice, MutableComponent empty) (dispatch: the body for this class)
    fn append_count_and_price(&self) -> fn(&compat::Shared<dyn LayoutAppender>, &crate::net::azureaaron::networth::calculation::Calculation, f64, &mut MutableComponent) {
        <dyn LayoutAppender>::append_count_and_price_body
    }

}

impl dyn LayoutAppender {
    /// java: default void appendCountAndPrice(Calculation calc, double totalPrice, MutableComponent empty)
    pub fn append_count_and_price_body(__this: &compat::Shared<dyn LayoutAppender>, calc: &crate::net::azureaaron::networth::calculation::Calculation, mut total_price: f64, empty: &mut MutableComponent) {
        if calc.count() > 1 {
            //   java: empty.append(Component.literal(" x").withStyle(ChatFormatting.GRAY)).append(Component.literal(String.valueOf(calc.count())).withStyle(ChatFormatting.YELLOW));
            *empty = empty.clone().append(component::literal(" x").with_style_format(ChatFormatting::Gray)).append(component::literal(&calc.count().to_string()).with_style_format(ChatFormatting::Yellow));
        }
        *empty = empty.clone().append(component::literal(" (").with_style_format(ChatFormatting::Gray));
        //   java: empty.append(getCoinsText(calc.price(), totalPrice));
        *empty = empty.clone().append(ValueBreakdownPopup::get_coins_text_2(calc.price(), total_price));
        *empty = empty.clone().append(component::literal(")").with_style_format(ChatFormatting::Gray));
    }

}

/// java: private record BasicSingleAppender(Component displayName, Function<String, Component> idFormatter, boolean hideIfWorthNothing) implements LayoutAppender
#[derive(Clone)]
pub struct BasicSingleAppender {
    /// java: Component displayName
    pub display_name: MutableComponent,
    /// java: Function<String, Component> idFormatter
    pub id_formatter: Arc<dyn Fn(String) -> MutableComponent>,
    /// java: boolean hideIfWorthNothing
    pub hide_if_worth_nothing: bool,
}

impl BasicSingleAppender {
    /// java: private BasicSingleAppender(Component displayName, Function<String, Component> idFormatter)
    pub fn new(mut display_name: MutableComponent, id_formatter: Arc<dyn Fn(String) -> MutableComponent>) -> Self {
        let mut this = Self::new_3(display_name.clone(), id_formatter.clone(), true);
        this
    }

    /// java: the canonical record constructor
    /// (the canonical record constructor is `new_N` here: Rust has no overloading and `new` is the explicit one)
    pub fn new_3(mut display_name: MutableComponent, mut id_formatter: Arc<dyn Fn(String) -> MutableComponent>, mut hide_if_worth_nothing: bool) -> Self {
        Self { display_name, id_formatter, hide_if_worth_nothing }
    }

    /// java: the record accessor `displayName()`
    pub fn display_name(&self) -> MutableComponent {
        self.display_name.clone()
    }

    /// java: the record accessor `idFormatter()`
    pub fn id_formatter(&self) -> Arc<dyn Fn(String) -> MutableComponent> {
        self.id_formatter.clone()
    }

    /// java: the record accessor `hideIfWorthNothing()`
    pub fn hide_if_worth_nothing(&self) -> bool {
        self.hide_if_worth_nothing
    }

}

impl LayoutAppender for BasicSingleAppender {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void appendTo(NetworthResult networthResult, List<Calculation> calculations, LinearLayout layout) (dispatch: the body for this class)
    fn append_to(&self) -> fn(&compat::Shared<dyn LayoutAppender>, &crate::net::azureaaron::networth::networth_result::NetworthResult, &Vec<crate::net::azureaaron::networth::calculation::Calculation>, &mut LinearLayout, &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        BasicSingleAppender::append_to_dyn
    }

}

impl BasicSingleAppender {
    /// java: @Override public void appendTo(NetworthResult networthResult, List<Calculation> calculations, LinearLayout layout)
    pub fn append_to_impl(&self, networth_result: &crate::net::azureaaron::networth::networth_result::NetworthResult, calculations: &Vec<crate::net::azureaaron::networth::calculation::Calculation>, layout: &mut LinearLayout, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        if (calculations.len() as i32) > 1 {
            //   java: LOGGER.warn("More than one calculation was found for type {}", calculations.getFirst().type());
            logger().warn(&format!("More than one calculation was found for type {}", calculations[0].r#type()));
        }
        let calc = &calculations[0];
        if calc.price() <= 0.0 && self.hide_if_worth_nothing {
            return;
        }
        let mut empty: MutableComponent = component::empty();
        empty = empty.append(self.display_name());
        let mut apply: MutableComponent = (self.id_formatter)(calc.id());
        if !(compat::is_blank(&apply.get_string())) {
            empty = empty.append_str(": ").append(apply.clone());
        }
        //   java: appendCountAndPrice(calc, networthResult.price(), empty);
        { let __s: compat::Shared<dyn LayoutAppender> = compat::shared(self.clone()); let __f = compat::dispatch(&__s, |__t| __t.append_count_and_price()); __f(&__s, calc, networth_result.price(), &mut empty); }
        layout.add_child(ValueBreakdownPopup::create_text_widget(screen, &empty, &compat::font()));
    }

    /// BasicSingleAppender.appendTo as the dispatch of LayoutAppender.appendTo
    pub fn append_to_dyn(__this: &compat::Shared<dyn LayoutAppender>, __p0: &crate::net::azureaaron::networth::networth_result::NetworthResult, __p1: &Vec<crate::net::azureaaron::networth::calculation::Calculation>, __p2: &mut LinearLayout, __p3: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        compat::read_as::<Self, _, _>(__this, |__s| __s.append_to_impl(__p0, __p1, __p2, __p3))
    }

}

/// java: private record BasicListAppender(Component displayName, Function<String, Component> idFormatter) implements LayoutAppender
#[derive(Clone)]
pub struct BasicListAppender {
    /// java: Component displayName
    pub display_name: MutableComponent,
    /// java: Function<String, Component> idFormatter
    pub id_formatter: Arc<dyn Fn(String) -> MutableComponent>,
}

impl BasicListAppender {
    /// java: the canonical record constructor
    pub fn new(mut display_name: MutableComponent, mut id_formatter: Arc<dyn Fn(String) -> MutableComponent>) -> Self {
        Self { display_name, id_formatter }
    }

    /// java: the record accessor `displayName()`
    pub fn display_name(&self) -> MutableComponent {
        self.display_name.clone()
    }

    /// java: the record accessor `idFormatter()`
    pub fn id_formatter(&self) -> Arc<dyn Fn(String) -> MutableComponent> {
        self.id_formatter.clone()
    }

}

impl LayoutAppender for BasicListAppender {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void appendTo(NetworthResult networthResult, List<Calculation> calculations, LinearLayout layout) (dispatch: the body for this class)
    fn append_to(&self) -> fn(&compat::Shared<dyn LayoutAppender>, &crate::net::azureaaron::networth::networth_result::NetworthResult, &Vec<crate::net::azureaaron::networth::calculation::Calculation>, &mut LinearLayout, &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        BasicListAppender::append_to_dyn
    }

}

impl BasicListAppender {
    /// java: @Override public void appendTo(NetworthResult networthResult, List<Calculation> calculations, LinearLayout layout)
    pub fn append_to_impl(&self, networth_result: &crate::net::azureaaron::networth::networth_result::NetworthResult, calculations: &Vec<crate::net::azureaaron::networth::calculation::Calculation>, layout: &mut LinearLayout, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        let mut text_renderer: Font = compat::font();
        layout.add_child(ValueBreakdownPopup::create_text_widget(screen, &self.display_name, &text_renderer));
        let mut total: f64 = ((0) as f64) as f64;
        for calc in calculations.iter() {
            let mut empty: MutableComponent = component::empty();
            let mut apply: MutableComponent = (self.id_formatter)(calc.id());
            empty = empty.append(apply.clone());
            //   java: appendCountAndPrice(calc, networthResult.price(), empty);
            { let __s: compat::Shared<dyn LayoutAppender> = compat::shared(self.clone()); let __f = compat::dispatch(&__s, |__t| __t.append_count_and_price()); __f(&__s, calc, networth_result.price(), &mut empty); }
            total += calc.price();
            layout.add_child_adjusted(ValueBreakdownPopup::create_text_widget(screen, &empty, &text_renderer), &mut |p| { *p = p.clone().padding_left(15); });
        }
        //   java: layout.addChild(createTextWidget(Component.translatable("skyblocker.valueBreakdownPopup.total", getCoinsText(total, networthResult.price())), textRenderer), p -> p.paddingLeft(10));
        let __text = component::translatable_args("skyblocker.valueBreakdownPopup.total", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(ValueBreakdownPopup::get_coins_text_2(total, networth_result.price()))]);
        layout.add_child_adjusted(ValueBreakdownPopup::create_text_widget(screen, &__text, &text_renderer), &mut |p| { *p = p.clone().padding_left(10); });
    }

    /// BasicListAppender.appendTo as the dispatch of LayoutAppender.appendTo
    pub fn append_to_dyn(__this: &compat::Shared<dyn LayoutAppender>, __p0: &crate::net::azureaaron::networth::networth_result::NetworthResult, __p1: &Vec<crate::net::azureaaron::networth::calculation::Calculation>, __p2: &mut LinearLayout, __p3: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        compat::read_as::<Self, _, _>(__this, |__s| __s.append_to_impl(__p0, __p1, __p2, __p3))
    }

}

/// A lambda as a `LayoutAppender` (the interface's one abstract method, `appendTo`): `EMPTY_APPENDER`.
pub struct LayoutAppenderLambda(pub Arc<dyn Fn(&crate::net::azureaaron::networth::networth_result::NetworthResult, &Vec<crate::net::azureaaron::networth::calculation::Calculation>, &mut LinearLayout, &mut mcrust::net::minecraft::client::gui::screens::screen::Screen)>);

impl LayoutAppender for LayoutAppenderLambda {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: void appendTo(NetworthResult networthResult, List<Calculation> calculations, LinearLayout layout); (the lambda's body)
    fn append_to(&self) -> fn(&compat::Shared<dyn LayoutAppender>, &crate::net::azureaaron::networth::networth_result::NetworthResult, &Vec<crate::net::azureaaron::networth::calculation::Calculation>, &mut LinearLayout, &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        LayoutAppenderLambda::append_to_dyn
    }
}

impl LayoutAppenderLambda {
    pub fn append_to_dyn(__this: &compat::Shared<dyn LayoutAppender>, __p0: &crate::net::azureaaron::networth::networth_result::NetworthResult, __p1: &Vec<crate::net::azureaaron::networth::calculation::Calculation>, __p2: &mut LinearLayout, __p3: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        let __f = compat::read_as::<Self, _, _>(__this, |__s| __s.0.clone());
        (__f)(__p0, __p1, __p2, __p3)
    }
}
