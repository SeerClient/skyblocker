//! Ported from de.hysky.skyblocker.skyblock.garden.FarmingHudWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::configs::farming_config;
use crate::de::hysky::skyblocker::config::configs::farming_config::Type;
use crate::de::hysky::skyblocker::skyblock::garden::farming_hud;
use crate::de::hysky::skyblocker::skyblock::garden::farming_hud::CounterType;
use crate::de::hysky::skyblocker::skyblock::garden::farming_hud::FarmingHud;
use crate::de::hysky::skyblocker::skyblock::garden::lower_sensitivity::LowerSensitivity;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::info::tooltip_info_type;
use crate::de::hysky::skyblocker::skyblock::itemlist::item_repository::ItemRepository;
use crate::de::hysky::skyblocker::skyblock::tabhud::config::option_widget_collector::OptionWidgetCollector;
use crate::de::hysky::skyblocker::skyblock::tabhud::util::ico;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::element::Element;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::element_collector::ElementCollector;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::elements::Elements;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::plain_text_element::PlainTextElement;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element_based_widget::ElementBasedWidget;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element_based_widget::ElementBasedWidgetBase;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::hud_widget::HudWidget;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::hud_widget::HudWidgetBase;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::hud_widget::Information;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
use crate::de::hysky::skyblocker::utils::item_utils::ItemUtils;
use crate::de::hysky::skyblocker::utils::json_value_input::JsonValueInput;
use crate::de::hysky::skyblocker::utils::json_value_output::JsonValueOutput;
use crate::de::hysky::skyblocker::utils::location::Location;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::components::cycle_button;
use mcrust::net::minecraft::client::gui::components::tooltip::Tooltip;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::util::mth;
use mcrust::net::minecraft::world::entity::entity::Entity;
use crate::de::hysky::skyblocker::injected::skyblocker_stack::SkyblockerStack as _;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::info::data_tooltip_info_type::DataTooltipInfoType as _;
use mcrust::net::minecraft::network::chat::component::Component as _;
use mcrust::net::minecraft::world::entity::living_entity::LivingEntity as _;

/// java: private static final MutableComponent TITLE = Component.literal("Farming").withStyle(ChatFormatting.YELLOW, ChatFormatting.BOLD);
pub static TITLE: compat::MainThread<MutableComponent> = compat::MainThread::new(|| component::literal("Farming").with_style_formats(&[ChatFormatting::Yellow, ChatFormatting::Bold]));
/// java: public static final Map<String, String> FARMING_TOOLS = Map.ofEntries( Map.entry("THEORETICAL_HOE_WHEAT_1", "WHEAT"), Map.entry("THEORETICAL_HOE_WHEAT_2", "WHEAT"), Map.entry("THEORETICAL_HOE_WHEAT_3"
pub static FARMING_TOOLS: LazyLock<std::collections::HashMap<String, String>> = LazyLock::new(|| std::collections::HashMap::from([("THEORETICAL_HOE_WHEAT_1".to_string(), "WHEAT".to_string()), ("THEORETICAL_HOE_WHEAT_2".to_string(), "WHEAT".to_string()), ("THEORETICAL_HOE_WHEAT_3".to_string(), "WHEAT".to_string()), ("THEORETICAL_HOE_CARROT_1".to_string(), "CARROT_ITEM".to_string()), ("THEORETICAL_HOE_CARROT_2".to_string(), "CARROT_ITEM".to_string()), ("THEORETICAL_HOE_CARROT_3".to_string(), "CARROT_ITEM".to_string()), ("THEORETICAL_HOE_POTATO_1".to_string(), "POTATO_ITEM".to_string()), ("THEORETICAL_HOE_POTATO_2".to_string(), "POTATO_ITEM".to_string()), ("THEORETICAL_HOE_POTATO_3".to_string(), "POTATO_ITEM".to_string()), ("THEORETICAL_HOE_CANE_1".to_string(), "SUGAR_CANE".to_string()), ("THEORETICAL_HOE_CANE_2".to_string(), "SUGAR_CANE".to_string()), ("THEORETICAL_HOE_CANE_3".to_string(), "SUGAR_CANE".to_string()), ("THEORETICAL_HOE_SUNFLOWER_1".to_string(), "DOUBLE_PLANT".to_string()), ("THEORETICAL_HOE_SUNFLOWER_2".to_string(), "DOUBLE_PLANT".to_string()), ("THEORETICAL_HOE_SUNFLOWER_3".to_string(), "DOUBLE_PLANT".to_string()), ("THEORETICAL_HOE_WILD_ROSE_1".to_string(), "WILD_ROSE".to_string()), ("THEORETICAL_HOE_WILD_ROSE_2".to_string(), "WILD_ROSE".to_string()), ("THEORETICAL_HOE_WILD_ROSE_3".to_string(), "WILD_ROSE".to_string()), ("THEORETICAL_HOE_WARTS_1".to_string(), "NETHER_STALK".to_string()), ("THEORETICAL_HOE_WARTS_2".to_string(), "NETHER_STALK".to_string()), ("THEORETICAL_HOE_WARTS_3".to_string(), "NETHER_STALK".to_string()), ("FUNGI_CUTTER".to_string(), "RED_MUSHROOM".to_string()), ("FUNGI_CUTTER_2".to_string(), "RED_MUSHROOM".to_string()), ("FUNGI_CUTTER_3".to_string(), "RED_MUSHROOM".to_string()), ("CACTUS_KNIFE".to_string(), "CACTUS".to_string()), ("CACTUS_KNIFE_2".to_string(), "CACTUS".to_string()), ("CACTUS_KNIFE_3".to_string(), "CACTUS".to_string()), ("MELON_DICER".to_string(), "MELON".to_string()), ("MELON_DICER_2".to_string(), "MELON".to_string()), ("MELON_DICER_3".to_string(), "MELON".to_string()), ("PUMPKIN_DICER".to_string(), "PUMPKIN".to_string()), ("PUMPKIN_DICER_2".to_string(), "PUMPKIN".to_string()), ("PUMPKIN_DICER_3".to_string(), "PUMPKIN".to_string()), ("COCO_CHOPPER".to_string(), "INK_SACK:3".to_string()), ("COCO_CHOPPER_2".to_string(), "INK_SACK:3".to_string()), ("COCO_CHOPPER_3".to_string(), "INK_SACK:3".to_string()), ("BASIC_GARDENING_HOE".to_string(), "".to_string()), ("ADVANCED_GARDENING_HOE".to_string(), "".to_string()), ("BASIC_GARDENING_AXE".to_string(), "".to_string()), ("ADVANCED_GARDENING_AXE".to_string(), "".to_string()), ("BINGHOE".to_string(), "".to_string())]));
/// java: private static @Nullable FarmingHudWidget instance = null;
pub static INSTANCE: compat::MainThread<Option<compat::Shared<FarmingHudWidget>>> = compat::MainThread::new(|| None);

/// java: @RegisterWidget public class FarmingHudWidget extends ElementBasedWidget
#[derive(Clone)]
pub struct FarmingHudWidget {
    /// java: extends ElementBasedWidget
    pub base: ElementBasedWidgetBase,
    /// java: private boolean showCounter = true;
    pub show_counter: bool,
    /// java: private boolean showCoins = true;
    pub show_coins: bool,
    /// java: private boolean showBlocks = true;
    pub show_blocks: bool,
    /// java: private boolean showExperience = true;
    pub show_experience: bool,
    /// java: private boolean showAngle = true;
    pub show_angle: bool,
    /// java: private boolean includeSeedsPrice = true;
    pub include_seeds_price: bool,
    /// java: private FarmingConfig.Type priceType = FarmingConfig.Type.BOTH;
    pub price_type: Type,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl std::ops::Deref for FarmingHudWidget {
    type Target = ElementBasedWidgetBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for FarmingHudWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl FarmingHudWidget {
    /// java: public FarmingHudWidget()
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = ElementBasedWidgetBase::new_in(TITLE.get_mut().clone(), mcrust::net::minecraft::network::chat::text_color::TextColor::YELLOW.get_value(), Information::new3("hud_farming".to_string(), component::literal("Farming HUD"), Location::Garden), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                self_ref: std::sync::Weak::new(),
                show_counter: true,
                show_coins: true,
                show_blocks: true,
                show_experience: true,
                show_angle: true,
                include_seeds_price: true,
                price_type: Type::Both,
            });
            compat::write(&__h, |__t| __t.self_ref = std::sync::Arc::downgrade(&__h));
            ((__h.clone() as compat::Shared<dyn ElementBasedWidget>), __h)
        });
        *INSTANCE.get_mut() = Some(__this.clone());
        ({ let __f = compat::dispatch(&__this, |__t| __t.update()); __f(&compat::view(&__this, |a| a as compat::Shared<dyn ElementBasedWidget>)) });
        __this
    }

    /// java: public static FarmingHudWidget getInstance()
    pub fn get_instance() -> compat::Shared<FarmingHudWidget> {
        if INSTANCE.get_mut().is_none() {
            *INSTANCE.get_mut() = Some(FarmingHudWidget::new());
        }
        return INSTANCE.get_mut().clone().unwrap();
    }

    /// java: private Component getPriceText(String cropItemId, float cropsPerMinute, boolean hasReplenish, double blockBreaks)
    pub fn get_price_text(__this: &compat::Shared<Self>, crop_item_id: &str, mut crops_per_minute: f32, mut has_replenish: bool, mut block_breaks: f64) -> MutableComponent {
        let mut bazaar: Option<f64> = ItemUtils::get_item_price_string(Some(crop_item_id.to_string()));
        // TODO(fabric2seer): `.getDouble(..)` on a T
        //   java: OptionalDouble npc = TooltipInfoType.NPC.hasOrNullWarning(cropItemId) ? OptionalDouble.of(TooltipInfoType.NPC.getData().getDouble(cropItemId)) : OptionalDouble.empty();
        let mut npc: Option<f64> = todo!("fabric2seer: the initialiser of `npc` (TODO above)");
        let mut used_by_replenish: f64 = ((60 as f64) * block_breaks);
        if crop_item_id == "WHEAT" {
            let mut seeds_ratio: f64 = (if has_replenish { ((((crops_per_minute as f64) * 0.6) - used_by_replenish).max((0 as f64)) / (crops_per_minute as f64)) } else { 0.6 });
            let mut wheat_ratio: f64 = ((1 as f64) - seeds_ratio);
            let mut seeds_bazaar_price: Option<f64>;
            let mut seeds_npc_price: Option<f64>;
            if compat::read(__this, |__t| __t.include_seeds_price) {
                seeds_bazaar_price = ItemUtils::get_item_price_string(Some("SEEDS".to_string()));
                // TODO(fabric2seer): `.getDouble(..)` on a T
                //   java: seedsNpcPrice = TooltipInfoType.NPC.hasOrNullWarning("SEEDS") ?
                //   java: 						OptionalDouble.of(TooltipInfoType.NPC.getData().getDouble("SEEDS")) :
                //   java: 						OptionalDouble.empty();
            } else {
                used_by_replenish = ((0) as f64) as f64;
                seeds_bazaar_price = Some(0);
                seeds_npc_price = Some(0);
            }
            let mut wheat_price: Option<f64> = ItemUtils::get_item_price_string(Some("WHEAT".to_string()));
            if seeds_bazaar_price.is_some() && wheat_price.is_some() {
                bazaar = Some(((wheat_price.clone().unwrap() * wheat_ratio) + (seeds_bazaar_price.clone().unwrap() * seeds_ratio)));
            } else {
                bazaar = None;
            }
            // TODO(fabric2seer): `.getDouble(..)` on a T
            //   java: npc = TooltipInfoType.NPC.hasOrNullWarning("WHEAT") && seedsNpcPrice.isPresent() ?
            //   java: 					OptionalDouble.of(seedsNpcPrice.getAsDouble() * seedsRatio + TooltipInfoType.NPC.getData().getDouble("WHEAT") * wheatRatio) :
            //   java: 					OptionalDouble.empty();
        }
        let mut price_to_use: f64 = ((0) as f64) as f64;
        let mut source_label: Option<MutableComponent> = None;
        let mut has_valid_price: bool = false;
        match compat::read(__this, |__t| __t.price_type) {
            Type::Npc => {
                if npc.is_some() {
                    price_to_use = npc.clone().unwrap();
                    source_label = Some(component::literal(" (").append(component::translatable("skyblocker.config.farming.farmingHud.type.NPC")).append_str(")"));
                    has_valid_price = true;
                }
            }
            Type::Bazaar => {
                if bazaar.is_some() {
                    price_to_use = bazaar.clone().unwrap();
                    source_label = Some(component::literal(" (").append(component::translatable("skyblocker.config.farming.farmingHud.type.BAZAAR")).append_str(")"));
                    has_valid_price = true;
                }
            }
            Type::Both => {
                if npc.is_some() && npc.clone().unwrap() > bazaar.clone().unwrap_or(((0) as f64) as f64) {
                    price_to_use = npc.clone().unwrap();
                    source_label = Some(component::literal(" (").append(component::translatable("skyblocker.config.farming.farmingHud.type.NPC")).append_str(")"));
                    has_valid_price = true;
                } else if bazaar.is_some() {
                    price_to_use = bazaar.clone().unwrap();
                    source_label = Some(component::literal(" (").append(component::translatable("skyblocker.config.farming.farmingHud.type.BAZAAR")).append_str(")"));
                    has_valid_price = true;
                }
            }
            _ => {}
        }
        if has_replenish {
            crops_per_minute = (crops_per_minute - ((used_by_replenish) as f32));
        }
        return (if has_valid_price { (component::literal(&farming_hud::NUMBER_FORMAT.get_mut().format((((((price_to_use * (crops_per_minute as f64)) * 0.6)) as i32).wrapping_mul(100)) as f64))).append(source_label.clone().unwrap()) } else { component::translatable("skyblocker.farming.farmingHud.noData") });
    }

}

impl ElementBasedWidget for FarmingHudWidget {
    fn element_based_widget_base(&self) -> &ElementBasedWidgetBase {
        &self.base
    }
    fn element_based_widget_base_mut(&mut self) -> &mut ElementBasedWidgetBase {
        &mut self.base
    }

    /// java: @Override public boolean shouldUpdateBeforeRendering() (dispatch: the body for this class)
    fn should_update_before_rendering(&self) -> fn(&compat::Shared<dyn ElementBasedWidget>) -> bool {
        FarmingHudWidget::should_update_before_rendering_dyn
    }

    /// java: @Override public void updateContent() (dispatch: the body for this class)
    fn update_content(&self) -> fn(&compat::Shared<dyn ElementBasedWidget>) {
        FarmingHudWidget::update_content_dyn
    }

}

impl HudWidget for FarmingHudWidget {
    fn hud_widget_base(&self) -> &HudWidgetBase {
        &self.base.base
    }
    fn hud_widget_base_mut(&mut self) -> &mut HudWidgetBase {
        &mut self.base.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void getOptionWidgets(OptionWidgetCollector collector) (dispatch: the body for this class)
    fn get_option_widgets(&self) -> fn(&compat::Shared<dyn HudWidget>, &mut OptionWidgetCollector) {
        FarmingHudWidget::get_option_widgets_dyn
    }

    /// java: @Override public void load(JsonValueInput input) (dispatch: the body for this class)
    fn load(&self) -> fn(&compat::Shared<dyn HudWidget>, &JsonValueInput) {
        FarmingHudWidget::load_dyn
    }

    /// java: @Override public void save(JsonValueOutput output) (dispatch: the body for this class)
    fn save(&self) -> fn(&compat::Shared<dyn HudWidget>, &mut JsonValueOutput) {
        FarmingHudWidget::save_dyn
    }

    /// java: @Override protected final void extractWidgetRenderState(GuiGraphicsExtractor context, float delta) (dispatch: the body for this class)
    fn extract_widget_render_state(&self) -> fn(&compat::Shared<dyn HudWidget>, &mut GuiGraphicsExtractor, f32) {
        <dyn ElementBasedWidget>::extract_widget_render_state_from::<Self>
    }

    /// java: @Override protected void extractWidgetRenderStateForConfig(GuiGraphicsExtractor graphics, float delta) (dispatch: the body for this class)
    fn extract_widget_render_state_for_config(&self) -> fn(&compat::Shared<dyn HudWidget>, &mut GuiGraphicsExtractor, f32) {
        <dyn ElementBasedWidget>::extract_widget_render_state_for_config_from::<Self>
    }

    /// java: @Override public void updateConfigPreview() (dispatch: the body for this class)
    fn update_config_preview(&self) -> fn(&compat::Shared<dyn HudWidget>) {
        <dyn ElementBasedWidget>::update_config_preview_from::<Self>
    }

}

impl ElementCollector for FarmingHudWidget {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: public <T extends Element> T addElement(T c)
    fn add_element(&mut self, mut c: compat::Shared<dyn Element>) -> compat::Shared<dyn Element> {
        ({ let __f = compat::dispatch(&c, |__t| __t.set_parent()); __f(&c, self.clone()) });
        self.elements.push(c.clone());
        return c;
    }

}

impl FarmingHudWidget {
    /// java: @Override public boolean shouldUpdateBeforeRendering()
    pub fn should_update_before_rendering_impl(__this: &compat::Shared<Self>) -> bool {
        return true;
    }

    /// FarmingHudWidget.shouldUpdateBeforeRendering as the dispatch of ElementBasedWidget.shouldUpdateBeforeRendering
    pub fn should_update_before_rendering_dyn(__this: &compat::Shared<dyn ElementBasedWidget>) -> bool {
        Self::should_update_before_rendering_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public void updateContent()
    pub fn update_content_impl(__this: &compat::Shared<Self>) {
        if mcrust::mcrust::modding::game::with_player(|_| ()).is_none() || mcrust::mcrust::modding::game::level().is_none() {
            { let (__a0,) = ((PlainTextElement::new(Some(component::literal("Nothing to show :p"))) as compat::Shared<dyn Element>),); __this.lock().borrow_mut().add_element(__a0) };
            return;
        }
        let mut farming_tool_stack: mcrust::net::minecraft::world::item::item_stack::ItemStack = mcrust::mcrust::modding::game::with_player(|player| player.get_main_hand_item().clone()).unwrap();
        let mut item_id: String = farming_tool_stack.get_skyblock_id();
        let mut crop_item_id: String = FARMING_TOOLS.get(&item_id).cloned().unwrap_or_else(|| "".to_string());
        if crop_item_id == "DOUBLE_PLANT" && mcrust::mcrust::modding::game::level().unwrap().get_default_clock_time() >= (12000 as i64) {
            crop_item_id = "MOONFLOWER".to_string();
        }
        let mut crop_stack: compat::Shared<FlexibleItemStack> = ItemRepository::get_item_stack(&crop_item_id.replace(":", "-")).unwrap();
        let mut crops_per_minute: f32 = FarmingHud::crops_per_minute();
        if compat::read(__this, |__t| __t.show_counter) {
            let mut counter_type: CounterType = FarmingHud::counter_type();
            let mut counter_number: String = farming_hud::NUMBER_FORMAT.get_mut().format((FarmingHud::counter()) as f64);
            if counter_type == CounterType::None {
                counter_number = "".to_string();
            }
            ({ let __f = compat::dispatch(__this, |__t| __t.add_simple_ico_text_flexible_item_stack_string_chat_formatting_string()); __f(&compat::view(__this, |a| a as compat::Shared<dyn ElementCollector>), Some(crop_stack.clone()), &counter_type.fields().text, &ChatFormatting::Yellow, &counter_number) });
            ({ let __f = compat::dispatch(__this, |__t| __t.add_simple_icon_translatable_text()); __f(&compat::view(__this, |a| a as compat::Shared<dyn ElementCollector>), Some(crop_stack.clone()), "skyblocker.farming.farmingHud.cropsPerMin", &ChatFormatting::Yellow, &farming_hud::NUMBER_FORMAT.get_mut().format(((((crops_per_minute) as i32).wrapping_div(10)).wrapping_mul(10)) as f64)) });
        }
        let mut block_breaks: f64 = FarmingHud::block_breaks();
        if compat::read(__this, |__t| __t.show_coins) {
            let mut has_counter: bool = (FarmingHud::counter_type() != CounterType::None);
            let mut has_replenish: bool = (has_counter && ItemUtils::get_custom_data(&farming_tool_stack).get_compound_or_empty("enchantments").contains("replenish"));
            ({ let __f = compat::dispatch(__this, |__t| __t.add_simple_icon_translatable_text_flexible_item_stack_string_chat_formatting_component()); __f(&compat::view(__this, |a| a as compat::Shared<dyn ElementCollector>), ico::GOLD.get_mut(), "skyblocker.farming.farmingHud.coinsPerHour", &ChatFormatting::Gold, &FarmingHudWidget::get_price_text(__this, &crop_item_id, crops_per_minute, has_replenish, block_breaks)) });
        }
        if compat::read(__this, |__t| __t.show_blocks) {
            ({ let __f = compat::dispatch(__this, |__t| __t.add_simple_icon_translatable_text()); __f(&compat::view(__this, |a| a as compat::Shared<dyn ElementCollector>), Some(crop_stack.clone()), "skyblocker.farming.farmingHud.blocksPerSec", &ChatFormatting::Yellow, &compat::fstr(block_breaks as f64)) });
        }
        if compat::read(__this, |__t| __t.show_experience) {
            { let (__a0,) = (Elements::progress_component_flexible_item_stack_component_float_int(Some(ico::LANTERN.get_mut().clone()), &component::translatable("skyblocker.farming.farmingHud.farmingLevel"), FarmingHud::farming_xp_percent_progress(), mcrust::net::minecraft::network::chat::text_color::TextColor::GOLD.get_value()),); __this.lock().borrow_mut().add_element(__a0) };
            ({ let __f = compat::dispatch(__this, |__t| __t.add_simple_icon_translatable_text()); __f(&compat::view(__this, |a| a as compat::Shared<dyn ElementCollector>), Some(ico::LIME_DYE.get_mut().clone()), "skyblocker.farming.farmingHud.farmingXPPerHour", &ChatFormatting::Yellow, &farming_hud::NUMBER_FORMAT.get_mut().format(FarmingHud::farming_xp_per_hour())) });
        }
        if compat::read(__this, |__t| __t.show_angle) {
            let mut camera_entity: Option<Box<dyn Entity>> = compat::minecraft().get_camera_entity();
            let mut yaw: MutableComponent = (if camera_entity.is_none() { component::translatable("skyblocker.farming.farmingHud.noCameraEntity") } else { component::literal(&format!("{:.2}", mth::wrap_degrees_f32(camera_entity.as_ref().unwrap().get_y_rot()))) });
            let mut pitch: MutableComponent = (if camera_entity.is_none() { component::translatable("skyblocker.farming.farmingHud.noCameraEntity") } else { component::literal(&format!("{:.2}", mth::wrap_degrees_f32(camera_entity.as_ref().unwrap().get_x_rot()))) });
            { let (__a0,) = ((PlainTextElement::new(Some((component::translatable_args("skyblocker.farming.farmingHud.yaw", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(yaw.clone())])).with_style_format(ChatFormatting::Gold))) as compat::Shared<dyn Element>),); __this.lock().borrow_mut().add_element(__a0) };
            { let (__a0,) = ((PlainTextElement::new(Some((component::translatable_args("skyblocker.farming.farmingHud.pitch", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(pitch.clone())])).with_style_format(ChatFormatting::Gold))) as compat::Shared<dyn Element>),); __this.lock().borrow_mut().add_element(__a0) };
        }
        if LowerSensitivity::is_sensitivity_lowered() {
            { let (__a0,) = ((PlainTextElement::new(Some(component::translatable("skyblocker.garden.hud.mouseLocked").with_style_format(ChatFormatting::Italic))) as compat::Shared<dyn Element>),); __this.lock().borrow_mut().add_element(__a0) };
        }
    }

    /// FarmingHudWidget.updateContent as the dispatch of ElementBasedWidget.updateContent
    pub fn update_content_dyn(__this: &compat::Shared<dyn ElementBasedWidget>) {
        Self::update_content_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public void getOptionWidgets(OptionWidgetCollector collector)
    pub fn get_option_widgets_impl(__this: &compat::Shared<Self>, collector: &mut OptionWidgetCollector) {
        <dyn ElementBasedWidget>::get_option_widgets_body(&compat::view(__this, |a| a as compat::Shared<dyn ElementBasedWidget>), collector);
        let __this_w1 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        collector.yes_no_button_3(&component::translatable("skyblocker.config.farming.farmingHud.counter"), Arc::new(move |b| { let Some(__this_1) = __this_w1.get().upgrade() else { return Default::default(); }; { { let __v = b; compat::write((&__this_1), |__t| __t.show_counter = __v); } } }), compat::read(__this, |__t| __t.show_counter));
        let __this_w2 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        collector.yes_no_button_3(&component::translatable("skyblocker.config.farming.farmingHud.coins"), Arc::new(move |b| { let Some(__this_2) = __this_w2.get().upgrade() else { return Default::default(); }; { { let __v = b; compat::write((&__this_2), |__t| __t.show_coins = __v); } } }), compat::read(__this, |__t| __t.show_coins));
        let __this_w3 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        collector.yes_no_button_3(&component::translatable("skyblocker.config.farming.farmingHud.blocks"), Arc::new(move |b| { let Some(__this_3) = __this_w3.get().upgrade() else { return Default::default(); }; { { let __v = b; compat::write((&__this_3), |__t| __t.show_blocks = __v); } } }), compat::read(__this, |__t| __t.show_blocks));
        let __this_w4 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        collector.yes_no_button_3(&component::translatable("skyblocker.config.farming.farmingHud.experience"), Arc::new(move |b| { let Some(__this_4) = __this_w4.get().upgrade() else { return Default::default(); }; { { let __v = b; compat::write((&__this_4), |__t| __t.show_experience = __v); } } }), compat::read(__this, |__t| __t.show_experience));
        let __this_w5 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        collector.yes_no_button_3(&component::translatable("skyblocker.config.farming.farmingHud.angle"), Arc::new(move |b| { let Some(__this_5) = __this_w5.get().upgrade() else { return Default::default(); }; { { let __v = b; compat::write((&__this_5), |__t| __t.show_angle = __v); } } }), compat::read(__this, |__t| __t.show_angle));
        let __this_w6 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        collector.yes_no_button_3(&component::translatable("skyblocker.config.farming.farmingHud.includeSeedsPrice"), Arc::new(move |b| { let Some(__this_6) = __this_w6.get().upgrade() else { return Default::default(); }; { { let __v = b; compat::write((&__this_6), |__t| __t.include_seeds_price = __v); } } }), compat::read(__this, |__t| __t.include_seeds_price));
        // TODO(fabric2seer): the port's `builder` ports this Java method with other parameters (default_value for Java's value_stringifier, default_value)
        // TODO(fabric2seer): `builder` takes 1 argument(s) in the port (`default_value: String`), Java passes 2
        // TODO(fabric2seer): a lambda for a parameter the port does not type as a closure (`Function<T, net.minecraft.network.chat.Component>`)
        // TODO(fabric2seer): a lambda for a parameter the port does not type as a closure (`net.minecraft.client.OptionInstance.TooltipSupplier<T>`)
        // TODO(fabric2seer): the port's `create` ports this Java method with other parameters (name, generic_value_format, value_change_listener for Java's x, y, width, height, name, value_change_listener)
        // TODO(fabric2seer): `create` takes 3 argument(s) in the port (`name: String, generic_value_format: String, value_change_listener: OnValueChange`), Java passes 6
        // TODO(fabric2seer): a lambda for a parameter the port does not type as a closure (`net.minecraft.client.gui.components.CycleButton.OnValueChange<T>`)
        // rust: let __this_w7 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        // rust: collector.add_widget((cycle_button::builder(|d| component::literal(&compat::jstring(&d)), compat::read(__this, |__t| __t.price_type))).with_tooltip(|_| Tooltip::create((component::translatable("skyblocker.config.farming.farmingHud.type.@Tooltip")).get_string())).with_values(Type::values()).create(0, 0, 150, 20, component::translatable("skyblocker.config.farming.farmingHud.type"), |_, value| { let Some(__this_7) = __this_w7.get().upgrade() else { return Default::default(); }; { { let __v = value.clone(); compat::write((&__this_7), |__t| __t.price_type = __v); } } }).unwrap());
    }

    /// FarmingHudWidget.getOptionWidgets as the dispatch of HudWidget.getOptionWidgets
    pub fn get_option_widgets_dyn(__this: &compat::Shared<dyn HudWidget>, __p0: &mut OptionWidgetCollector) {
        Self::get_option_widgets_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public void load(JsonValueInput input)
    pub fn load_impl(__this: &compat::Shared<Self>, input: &JsonValueInput) {
        <dyn ElementBasedWidget>::load_body(&compat::view(__this, |a| a as compat::Shared<dyn ElementBasedWidget>), input);
        { let __v = input.read_boolean_or("counter", true); compat::write(__this, |__t| __t.show_counter = __v); }
        { let __v = input.read_boolean_or("coins", true); compat::write(__this, |__t| __t.show_coins = __v); }
        { let __v = input.read_boolean_or("blocks", true); compat::write(__this, |__t| __t.show_blocks = __v); }
        { let __v = input.read_boolean_or("experience", true); compat::write(__this, |__t| __t.show_experience = __v); }
        { let __v = input.read_boolean_or("angle", true); compat::write(__this, |__t| __t.show_angle = __v); }
        { let __v = input.read_boolean_or("include_seeds_price", true); compat::write(__this, |__t| __t.include_seeds_price = __v); }
        { let __v = input.read("price_type", farming_config::TYPE_CODEC.get_mut()).unwrap_or(Type::Both); compat::write(__this, |__t| __t.price_type = __v); }
    }

    /// FarmingHudWidget.load as the dispatch of HudWidget.load
    pub fn load_dyn(__this: &compat::Shared<dyn HudWidget>, __p0: &JsonValueInput) {
        Self::load_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public void save(JsonValueOutput output)
    pub fn save_impl(__this: &compat::Shared<Self>, output: &mut JsonValueOutput) {
        <dyn ElementBasedWidget>::save_body(&compat::view(__this, |a| a as compat::Shared<dyn ElementBasedWidget>), output);
        output.write_bool("counter", compat::read(__this, |__t| __t.show_counter));
        output.write_bool("coins", compat::read(__this, |__t| __t.show_coins));
        output.write_bool("blocks", compat::read(__this, |__t| __t.show_blocks));
        output.write_bool("experience", compat::read(__this, |__t| __t.show_experience));
        output.write_bool("angle", compat::read(__this, |__t| __t.show_angle));
        output.write_bool("include_seeds_price", compat::read(__this, |__t| __t.include_seeds_price));
        output.write("price_type".to_string(), farming_config::TYPE_CODEC.get_mut(), &compat::read(__this, |__t| __t.price_type));
    }

    /// FarmingHudWidget.save as the dispatch of HudWidget.save
    pub fn save_dyn(__this: &compat::Shared<dyn HudWidget>, __p0: &mut JsonValueOutput) {
        Self::save_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

}
