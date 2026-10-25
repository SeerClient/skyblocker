//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.widgets.CollectionItemWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::itemlist::item_repository::ItemRepository;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::loading_information::LoadingInformation;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::model::api_profile::ApiProfile;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::utils::collection_tiers::CollectionTiers;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::utils::collection_tiers::Report;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::utils::elite_leaderboards;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::utils::elite_leaderboards::EliteLeaderboards;
use crate::de::hysky::skyblocker::skyblock::tabhud::util::ico;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
use crate::de::hysky::skyblocker::utils::formatters;
use crate::de::hysky::skyblocker::utils::roman_numerals::RomanNumerals;
use crate::de::hysky::skyblocker::utils::sky_block_colors;
use crate::de::hysky::skyblocker::utils::sky_block_tooltip_styles;
use mcrust::java::text::decimal_format_symbols::Locale;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::abstract_widget::WidgetKind;
use mcrust::net::minecraft::client::gui::font::Font;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final Identifier BACKGROUND = SkyblockerMod.id("profile_viewer2/basic_background");
pub static BACKGROUND: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("profile_viewer2/basic_background"));
/// java: private static final int WIDTH = 20;
pub const WIDTH: i32 = 20;
/// java: private static final int TEXT_Y_OFFSET = 2;
pub const TEXT_Y_OFFSET: i32 = 2;

/// java: public sealed class CollectionItemWidget extends AbstractWidget permits MinionWidget
#[derive(Clone)]
pub struct CollectionItemWidgetBase {
    /// java: extends net.minecraft.client.gui.components.AbstractWidget
    pub base: AbstractWidget,
    /// java: private final ItemStack icon;
    pub icon: mcrust::net::minecraft::world::item::item_stack::ItemStack,
    /// java: private final Component tierText;
    pub tier_text: MutableComponent,
    /// java: private final List<Component> tooltip;
    pub tooltip: Vec<MutableComponent>,
    /// java: private final @Nullable Identifier tooltipStyle;
    pub tooltip_style: Option<Identifier>,
}

impl std::ops::Deref for CollectionItemWidgetBase {
    type Target = AbstractWidget;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for CollectionItemWidgetBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl CollectionItemWidgetBase {
    /// java: public CollectionItemWidget(ItemStack icon, Component tierText, List<Component> tooltip, @Nullable Identifier tooltipStyle) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_in<__R>(mut icon: mcrust::net::minecraft::world::item::item_stack::ItemStack, mut tier_text: MutableComponent, mut tooltip: Vec<MutableComponent>, mut tooltip_style: Option<Identifier>, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn CollectionItemWidget>, __R)) -> (compat::Shared<dyn CollectionItemWidget>, __R) {
        let __init_icon = icon.clone();
        let __init_tier_text = tier_text.clone();
        let __init_tooltip = tooltip.clone();
        let __init_tooltip_style = tooltip_style.clone();
        let this = Self {
            base: AbstractWidget::new(0, 0, WIDTH, (WIDTH.wrapping_add(TEXT_Y_OFFSET)).wrapping_add(mcrust::net::minecraft::client::gui::font::LINE_HEIGHT), (&component::empty().get_string()).to_string(), WidgetKind::PlainText, OnPress::Inert),
            icon: __init_icon,
            tier_text: __init_tier_text,
            tooltip: __init_tooltip,
            tooltip_style: __init_tooltip_style,
        };
        let (__h, __r) = __wrap(this);
        let __this = __h;
        { let __v = false; compat::write(&__this, |__t| __t.collection_item_widget_base_mut().active = __v); }
        (__this, __r)
    }

    /// java: public CollectionItemWidget(ItemStack icon, Component tierText, List<Component> tooltip, @Nullable Identifier tooltipStyle)
    pub fn new(mut icon: mcrust::net::minecraft::world::item::item_stack::ItemStack, mut tier_text: MutableComponent, mut tooltip: Vec<MutableComponent>, mut tooltip_style: Option<Identifier>) -> compat::Shared<Self> {
        Self::new_in(icon, tier_text, tooltip, tooltip_style, |this| {
            let __h = compat::shared(this);
            ((__h.clone() as compat::Shared<dyn CollectionItemWidget>), __h)
        }).1
    }

    /// java: public static CollectionItemWidget create(String id, LoadingInformation info, CollectionTiers.Report report)
    pub fn create(id: &str, info: &LoadingInformation, report: &Report) -> compat::Shared<dyn CollectionItemWidget> {
        let mut tier: i32 = report.tier();
        let mut is_max_tier: bool = (tier == CollectionTiers::get_max_tier(id));
        let mut neu_id: String = id.replace(':', &'-'.to_string());
        let mut stack: mcrust::net::minecraft::world::item::item_stack::ItemStack = (FlexibleItemStack::get_stack_or_throw(&ItemRepository::get_item_stack_2(&neu_id, ico::BARRIER.get_mut()))).copy();
        if is_max_tier {
            stack.set(mcrust::net::minecraft::core::component::data_components::enchantment_glint_override().clone(), Some(mcrust::com::mojang::serialization::obj(true)));
        }
        let mut tier_text: MutableComponent = CollectionItemWidgetBase::build_tier_text(tier, is_max_tier);
        let mut tooltip: Vec<MutableComponent> = CollectionItemWidgetBase::build_tooltip(id, &compat::component_of(stack.get_hover_name()).get_string(), info, report);
        // java: @Nullable Identifier tooltipStyle = getTooltipType(isMaxTier); - null below the max tier
        let tooltip_style: Option<Identifier> = CollectionItemWidgetBase::get_tooltip_type(is_max_tier);
        return (CollectionItemWidgetBase::new(stack, tier_text, tooltip, tooltip_style) as compat::Shared<dyn CollectionItemWidget>);
    }

    /// java: protected static Component buildTierText(int tier, boolean isMaxTier)
    pub fn build_tier_text(mut tier: i32, mut is_max_tier: bool) -> MutableComponent {
        let mut tier_text: String = (if tier == 0 { "0".to_string() } else { RomanNumerals::decimal_to_roman(tier) });
        return component::literal(&tier_text).with_color((if is_max_tier { sky_block_colors::GOLD.get_mut().get_value() } else { mcrust::net::minecraft::util::common_colors::DARK_GRAY }));
    }

    /// java: private static List<Component> buildTooltip(String id, String name, LoadingInformation info, CollectionTiers.Report report)
    pub fn build_tooltip(id: &str, name: &str, info: &LoadingInformation, report: &Report) -> Vec<MutableComponent> {
        let mut tooltip: Vec<MutableComponent> = Vec::new();
        tooltip.push(component::literal(name));
        tooltip.push(component::literal("Collection Item").with_style_format(ChatFormatting::DarkGray));
        tooltip.push(component::empty());
        if ApiProfile::has_been_coop(&info.profile()) {
            tooltip.push((component::literal(&format!("Personal: {}", formatters::INTEGER_NUMBERS.get_mut().format((report.personal()) as f64)))).with_style_format(ChatFormatting::Gold));
            tooltip.push((component::literal(&format!("Co-op: {}", formatters::INTEGER_NUMBERS.get_mut().format((report.coop()) as f64)))).with_style_format(ChatFormatting::Aqua));
        }
        tooltip.push((component::literal(&format!("Collection: {}", formatters::INTEGER_NUMBERS.get_mut().format((report.total()) as f64)))).with_style_format(ChatFormatting::Yellow));
        tooltip.push(component::empty());
        tooltip.push((component::literal(&format!("Collection Tier: {}/{}", report.tier(), CollectionTiers::get_max_tier(id)))).with_style_format(ChatFormatting::LightPurple));
        let mut leaderboard_id: String = EliteLeaderboards::get_collection_leaderboard_mappings().get(id).cloned().unwrap_or_else(|| "".to_string());
        let mut leaderboard_position: i32 = info.get_leaderboard_position(&leaderboard_id);
        if leaderboard_position != elite_leaderboards::NO_POSITION {
            tooltip.push((component::literal(&format!("Leaderboard: #{}", formatters::INTEGER_NUMBERS.get_mut().format((leaderboard_position) as f64)))).with_style_format(ChatFormatting::LightPurple));
        }
        return tooltip.iter().cloned().collect::<Vec<_>>();
    }

    /// java: protected static @Nullable Identifier getTooltipType(boolean isMaxTier)
    pub fn get_tooltip_type(mut is_max_tier: bool) -> Option<Identifier> {
        return (if is_max_tier { Some(sky_block_tooltip_styles::LEGENDARY.get_mut().clone()) } else { None });
    }

}

/// java: the virtual methods of CollectionItemWidget (implemented by CollectionItemWidgetBase and every subclass)
pub trait CollectionItemWidget: compat::DynAny {
    fn collection_item_widget_base(&self) -> &CollectionItemWidgetBase;
    fn collection_item_widget_base_mut(&mut self) -> &mut CollectionItemWidgetBase;
    fn as_any(&self) -> &dyn std::any::Any;

    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a) (dispatch: the body for this class)
    fn extract_widget_render_state(&self) -> fn(&compat::Shared<dyn CollectionItemWidget>, &mut GuiGraphicsExtractor, i32, i32, f32) {
        <dyn CollectionItemWidget>::extract_widget_render_state_body
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput output) (dispatch: the body for this class)
    fn update_widget_narration(&self) -> fn(&compat::Shared<dyn CollectionItemWidget>, &dyn NarrationElementOutput) {
        <dyn CollectionItemWidget>::update_widget_narration_body
    }

    /// java: @Override public boolean shouldTakeFocusAfterInteraction() (dispatch: the body for this class)
    fn should_take_focus_after_interaction(&self) -> fn(&compat::Shared<dyn CollectionItemWidget>) -> bool {
        <dyn CollectionItemWidget>::should_take_focus_after_interaction_body
    }

}

impl dyn CollectionItemWidget {
    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_widget_render_state_body(__this: &compat::Shared<dyn CollectionItemWidget>, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        let mut font: Font = compat::font();
        let mut item_box_height: i32 = ((__this.lock().borrow().collection_item_widget_base().get_height()).wrapping_sub(TEXT_Y_OFFSET)).wrapping_sub(mcrust::net::minecraft::client::gui::font::LINE_HEIGHT);
        gui_graphics_extractor::blit_sprite_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), BACKGROUND.get_mut(), __this.lock().borrow().collection_item_widget_base().get_x(), __this.lock().borrow().collection_item_widget_base().get_y(), __this.lock().borrow().collection_item_widget_base().get_width(), item_box_height);
        graphics.item(&compat::read(__this, |__t| __t.collection_item_widget_base().icon.clone()), (__this.lock().borrow().collection_item_widget_base().get_x()).wrapping_add(((__this.lock().borrow().collection_item_widget_base().get_width()).wrapping_sub(mcrust::net::minecraft::client::gui::render::gui_renderer::DEFAULT_ITEM_SIZE)).wrapping_div(2)), (__this.lock().borrow().collection_item_widget_base().get_y()).wrapping_add((item_box_height.wrapping_sub(mcrust::net::minecraft::client::gui::render::gui_renderer::DEFAULT_ITEM_SIZE)).wrapping_div(2)));
        gui_graphics_extractor::draw_formatted_text_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(compat::read(__this, |__t| __t.collection_item_widget_base().tier_text.clone())), (__this.lock().borrow().collection_item_widget_base().get_x()).wrapping_add(((__this.lock().borrow().collection_item_widget_base().get_width()).wrapping_sub(font.width_of_cached(&mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(compat::read(__this, |__t| __t.collection_item_widget_base().tier_text.clone()))))).wrapping_div(2)), (__this.lock().borrow().collection_item_widget_base().get_y()).wrapping_add((__this.lock().borrow().collection_item_widget_base().get_height()).wrapping_sub(mcrust::net::minecraft::client::gui::font::LINE_HEIGHT)), mcrust::net::minecraft::util::common_colors::WHITE, false);
        if __this.lock().borrow().collection_item_widget_base().is_hovered() {
            // java: graphics.setComponentTooltipForNextFrame(font, this.tooltip, mouseX, mouseY, this.tooltipStyle); (the game's
            // deferred tooltip draws every tooltip in the default style: its `_styled` overload takes no style)
            let __lines: Vec<mcrust::net::minecraft::network::chat::formatted_text::FormattedText> = compat::read(__this, |__t| __t.collection_item_widget_base().tooltip.iter().cloned().map(mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from).collect());
            let _ = (&font, compat::read(__this, |__t| __t.collection_item_widget_base().tooltip_style.clone()));
            compat::batch_s_set_tooltip_for_next_frame(__lines);
        }
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput output)
    pub fn update_widget_narration_body(__this: &compat::Shared<dyn CollectionItemWidget>, output: &dyn NarrationElementOutput) {
    }

    /// java: @Override public boolean shouldTakeFocusAfterInteraction()
    pub fn should_take_focus_after_interaction_body(__this: &compat::Shared<dyn CollectionItemWidget>) -> bool {
        return false;
    }

}

impl CollectionItemWidget for CollectionItemWidgetBase {
    fn collection_item_widget_base(&self) -> &CollectionItemWidgetBase {
        self
    }
    fn collection_item_widget_base_mut(&mut self) -> &mut CollectionItemWidgetBase {
        self
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

}

/// CollectionItemWidget's widget half (`super`'s fields: x, y, size, message, ...), lent by the game for each call
impl compat::WidgetHalf for CollectionItemWidgetBase {
    fn widget_half(&mut self) -> &mut AbstractWidget {
        &mut self.base
    }
}

/// CollectionItemWidget as the game's widget: `compat::WidgetAdapter::widget(handle)` where Java hands it on
impl mcrust::mcrust::modding::widgets::ModWidget for compat::WidgetAdapter<CollectionItemWidgetBase> {
    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_widget_render_state(&mut self, widget: &AbstractWidget, context: &mut mcrust::mcrust::modding::widgets::WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        self.lend_ref(widget, |__h| ({ let __f = compat::dispatch(__h, |__t| __t.extract_widget_render_state()); __f(&compat::view(__h, |a| a as compat::Shared<dyn CollectionItemWidget>), &mut context.graphics, mouse_x, mouse_y, a) }))
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput output)
    fn update_widget_narration(&mut self, widget: &AbstractWidget, output: &mut dyn NarrationElementOutput) {
        self.lend_ref(widget, |__h| ({ let __f = compat::dispatch(__h, |__t| __t.update_widget_narration()); __f(&compat::view(__h, |a| a as compat::Shared<dyn CollectionItemWidget>), output) }))
    }

    /// java: @Override public boolean shouldTakeFocusAfterInteraction()
    fn should_take_focus_after_interaction(&self) -> bool {
        self.call(|__h| ({ let __f = compat::dispatch(__h, |__t| __t.should_take_focus_after_interaction()); __f(&compat::view(__h, |a| a as compat::Shared<dyn CollectionItemWidget>)) }))
    }
}
