//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.widgets.LevelBarWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::item::skyblock_item_rarity::SkyblockItemRarity;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::utils::elite_leaderboards;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::utils::level_info::LevelInfo;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::utils::level_info::Progress;
use crate::de::hysky::skyblocker::skyblock::tabhud::util::ico;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
use crate::de::hysky::skyblocker::utils::formatters;
use crate::de::hysky::skyblocker::utils::render::gui_helper::GuiHelper;
use mcrust::java::awt::color::Color;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::abstract_widget::WidgetKind;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final Identifier BACKGROUND = SkyblockerMod.id("profile_viewer2/basic_background");
pub static BACKGROUND: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("profile_viewer2/basic_background"));
/// java: private static final Identifier BAR_BACKGROUND = SkyblockerMod.id("bars/bar_back");
pub static BAR_BACKGROUND: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("bars/bar_back"));
/// java: private static final Identifier BAR_FILL = SkyblockerMod.id("bars/bar_fill");
pub static BAR_FILL: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("bars/bar_fill"));
/// java: private static final int HEIGHT = 22;
pub const HEIGHT: i32 = 22;
/// java: private static final int ICON_AREA_SIZE = 22;
pub const ICON_AREA_SIZE: i32 = 22;
/// java: private static final int TEXT_Y_OFFSET = 3;
pub const TEXT_Y_OFFSET: i32 = 3;
/// java: private static final int BAR_OFFSET = TEXT_Y_OFFSET + Minecraft.getInstance().font.lineHeight + 1;
pub static BAR_OFFSET: LazyLock<i32> = LazyLock::new(|| (TEXT_Y_OFFSET.wrapping_add(mcrust::net::minecraft::client::gui::font::LINE_HEIGHT)).wrapping_add(1));
/// java: private static final int BAR_WIDTH = 75;
pub const BAR_WIDTH: i32 = 75;
/// java: private static final int BAR_HEIGHT = 6;
pub const BAR_HEIGHT: i32 = 6;

/// java: public sealed class LevelBarWidget extends AbstractWidget permits DungeonLevelBarWidget, SkillLevelBarWidget, SkyBlockLevelBarWidget
#[derive(Clone)]
pub struct LevelBarWidgetBase {
    /// java: extends net.minecraft.client.gui.components.AbstractWidget
    pub base: AbstractWidget,
    /// java: private final ItemStack icon;
    pub icon: mcrust::net::minecraft::world::item::item_stack::ItemStack,
    /// java: private final double barFillPercentage;
    pub bar_fill_percentage: f64,
    /// java: private final int barFillColour;
    pub bar_fill_colour: i32,
    /// java: private final List<Component> tooltip;
    pub tooltip: Vec<MutableComponent>,
    /// java: private final @Nullable Identifier tooltipStyle;
    pub tooltip_style: Option<Identifier>,
}

impl std::ops::Deref for LevelBarWidgetBase {
    type Target = AbstractWidget;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for LevelBarWidgetBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl LevelBarWidgetBase {
    /// java: protected LevelBarWidget(int width, ItemStack icon, Component label, double barFillPercentage, int barFillColour, List<Component> tooltip, @Nullable Identifier tooltipStyle) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_in<__R>(mut width: i32, mut icon: mcrust::net::minecraft::world::item::item_stack::ItemStack, mut label: MutableComponent, mut bar_fill_percentage: f64, mut bar_fill_colour: i32, mut tooltip: Vec<MutableComponent>, mut tooltip_style: Option<Identifier>, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn LevelBarWidget>, __R)) -> (compat::Shared<dyn LevelBarWidget>, __R) {
        let __init_icon = icon.clone();
        let __init_bar_fill_percentage = bar_fill_percentage;
        let __init_bar_fill_colour = bar_fill_colour;
        let __init_tooltip = tooltip.clone();
        let __init_tooltip_style = tooltip_style.clone();
        let this = Self {
            base: AbstractWidget::new(0, 0, width, HEIGHT, (&label.get_string()).to_string(), WidgetKind::PlainText, OnPress::Inert),
            icon: __init_icon,
            bar_fill_percentage: __init_bar_fill_percentage,
            bar_fill_colour: __init_bar_fill_colour,
            tooltip: __init_tooltip,
            tooltip_style: __init_tooltip_style,
        };
        let (__h, __r) = __wrap(this);
        let __this = __h;
        { let __v = false; compat::write(&__this, |__t| __t.level_bar_widget_base_mut().active = __v); }
        (__this, __r)
    }

    /// java: protected LevelBarWidget(int width, ItemStack icon, Component label, double barFillPercentage, int barFillColour, List<Component> tooltip, @Nullable Identifier tooltipStyle)
    pub fn new(mut width: i32, mut icon: mcrust::net::minecraft::world::item::item_stack::ItemStack, mut label: MutableComponent, mut bar_fill_percentage: f64, mut bar_fill_colour: i32, mut tooltip: Vec<MutableComponent>, mut tooltip_style: Option<Identifier>) -> compat::Shared<Self> {
        Self::new_in(width, icon, label, bar_fill_percentage, bar_fill_colour, tooltip, tooltip_style, |this| {
            let __h = compat::shared(this);
            ((__h.clone() as compat::Shared<dyn LevelBarWidget>), __h)
        }).1
    }

    /// java: public static LevelBarWidget placeholder(int width)
    pub fn placeholder(mut width: i32) -> compat::Shared<dyn LevelBarWidget> {
        // java: return new LevelBarWidget(width, Ico.BARRIER.getStackOrThrow(), Component.literal("Placeholder"), 0.75d, Color.CYAN.getRGB(), List.of(), null);
        return (LevelBarWidgetBase::new(width, FlexibleItemStack::get_stack_or_throw(crate::de::hysky::skyblocker::skyblock::tabhud::util::ico::BARRIER.get_mut()), component::literal("Placeholder"), 0.75f64, mcrust::java::awt::color::cyan().get_rgb(), vec![], None) as compat::Shared<dyn LevelBarWidget>);
    }

    /// java: protected static double getBarFillPercentage(LevelInfo levelInfo)
    pub fn get_bar_fill_percentage(level_info: &LevelInfo) -> f64 {
        return (if level_info.progress().is_some() { level_info.progress().unwrap().percentage_to_next_level() } else { (1.0f32) as f64 });
    }

    /// java: protected static int getBarFillColour(LevelInfo levelInfo)
    pub fn get_bar_fill_colour(level_info: &LevelInfo) -> i32 {
        let mut bar_fill_colour: Color = mcrust::java::awt::color::green();
        if level_info.is_level_capped() {
            bar_fill_colour = mcrust::java::awt::color::yellow();
        } else if level_info.is_level_absolutely_maxed() {
            bar_fill_colour = mcrust::java::awt::color::magenta();
        }
        // java: return barFillColour.getRGB();
        return bar_fill_colour.get_rgb();
    }

    /// java: protected static List<Component> buildTooltip(Component label, LevelInfo levelInfo, int leaderboardPosition)
    pub fn build_tooltip(label: &MutableComponent, level_info: &LevelInfo, mut leaderboard_position: i32) -> Vec<MutableComponent> {
        let mut tooltip: Vec<MutableComponent> = Vec::new();
        tooltip.push(label.copy().with_style_format(ChatFormatting::Green));
        tooltip.push((component::literal(&format!("XP: {}", formatters::INTEGER_NUMBERS.get_mut().format((level_info.xp()) as f64)))).with_style_format(ChatFormatting::Gold));
        if level_info.is_level_not_at_any_maximum() && level_info.progress().is_some() {
            let mut progress: Progress = level_info.progress().unwrap();
            tooltip.push((component::literal(&format!("Progress: {}/{}", formatters::INTEGER_NUMBERS.get_mut().format((progress.xp_progress()) as f64), formatters::INTEGER_NUMBERS.get_mut().format((progress.xp_needed()) as f64)))).with_style_format(ChatFormatting::Gold));
        }
        if leaderboard_position != elite_leaderboards::NO_POSITION {
            tooltip.push((component::literal(&format!("Leaderboard: #{}", formatters::INTEGER_NUMBERS.get_mut().format((leaderboard_position) as f64)))).with_style_format(ChatFormatting::Gold));
        }
        return tooltip.iter().cloned().collect::<Vec<_>>();
    }

    /// java: protected static @Nullable Identifier getTooltipStyle(LevelInfo levelInfo, int divisor)
    pub fn get_tooltip_style(level_info: &LevelInfo, mut divisor: i32) -> Option<Identifier> {
        let mut real_divisor: i32 = 1i32.max(divisor);
        let mut tooltip_rarity: SkyblockItemRarity = SkyblockItemRarity::values()[((level_info.level().wrapping_div(real_divisor)).min(6)) as usize];
        return tooltip_rarity.to_tooltip_style();
    }

}

/// java: the virtual methods of LevelBarWidget (implemented by LevelBarWidgetBase and every subclass)
pub trait LevelBarWidget: compat::DynAny {
    fn level_bar_widget_base(&self) -> &LevelBarWidgetBase;
    fn level_bar_widget_base_mut(&mut self) -> &mut LevelBarWidgetBase;
    fn as_any(&self) -> &dyn std::any::Any;

    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a) (dispatch: the body for this class)
    fn extract_widget_render_state(&self) -> fn(&compat::Shared<dyn LevelBarWidget>, &mut GuiGraphicsExtractor, i32, i32, f32) {
        <dyn LevelBarWidget>::extract_widget_render_state_body
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput output) (dispatch: the body for this class)
    fn update_widget_narration(&self) -> fn(&compat::Shared<dyn LevelBarWidget>, &dyn NarrationElementOutput) {
        <dyn LevelBarWidget>::update_widget_narration_body
    }

    /// java: @Override public boolean shouldTakeFocusAfterInteraction() (dispatch: the body for this class)
    fn should_take_focus_after_interaction(&self) -> fn(&compat::Shared<dyn LevelBarWidget>) -> bool {
        <dyn LevelBarWidget>::should_take_focus_after_interaction_body
    }

}

impl dyn LevelBarWidget {
    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_widget_render_state_body(__this: &compat::Shared<dyn LevelBarWidget>, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        gui_graphics_extractor::blit_sprite_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), BACKGROUND.get_mut(), __this.lock().borrow().level_bar_widget_base().get_x(), __this.lock().borrow().level_bar_widget_base().get_y(), __this.lock().borrow().level_bar_widget_base().get_width(), __this.lock().borrow().level_bar_widget_base().get_height());
        graphics.fake_item(&compat::read(__this, |__t| __t.level_bar_widget_base().icon.clone()), (__this.lock().borrow().level_bar_widget_base().get_x()).wrapping_add((ICON_AREA_SIZE.wrapping_sub(mcrust::net::minecraft::client::gui::render::gui_renderer::DEFAULT_ITEM_SIZE)).wrapping_div(2)), (__this.lock().borrow().level_bar_widget_base().get_y()).wrapping_add((ICON_AREA_SIZE.wrapping_sub(mcrust::net::minecraft::client::gui::render::gui_renderer::DEFAULT_ITEM_SIZE)).wrapping_div(2)));
        gui_graphics_extractor::draw_formatted_text(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), (__this.lock().borrow().level_bar_widget_base().get_x()).wrapping_add(ICON_AREA_SIZE), (__this.lock().borrow().level_bar_widget_base().get_y()).wrapping_add(TEXT_Y_OFFSET), &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(compat::component_of(__this.lock().borrow().level_bar_widget_base().get_message_component())), mcrust::net::minecraft::util::common_colors::WHITE);
        let mut bar_x: i32 = (__this.lock().borrow().level_bar_widget_base().get_x()).wrapping_add(ICON_AREA_SIZE);
        let mut bar_y: i32 = (__this.lock().borrow().level_bar_widget_base().get_y()).wrapping_add(*BAR_OFFSET);
        let mut bar_fill_width: i32 = (((compat::read(__this, |__t| __t.level_bar_widget_base().bar_fill_percentage) * (BAR_WIDTH as f64))) as i32);
        gui_graphics_extractor::blit_sprite_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), BAR_BACKGROUND.get_mut(), bar_x, bar_y, BAR_WIDTH, BAR_HEIGHT);
        gui_graphics_extractor::blit_sprite_identifier_with_color(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), BAR_FILL.get_mut(), bar_x, bar_y, bar_fill_width, BAR_HEIGHT, compat::read(__this, |__t| __t.level_bar_widget_base().bar_fill_colour));
        if GuiHelper::point_is_in_area((mouse_x) as f64, (mouse_y) as f64, (bar_x) as f64, (bar_y) as f64, ((bar_x.wrapping_add(BAR_WIDTH)) as f64) as f64, ((bar_y.wrapping_add(BAR_HEIGHT)) as f64) as f64) {
            // java: graphics.setComponentTooltipForNextFrame(Minecraft.getInstance().font, this.tooltip, mouseX, mouseY, this.tooltipStyle);
            // (the game's deferred tooltip draws every tooltip in the default style: its `_styled` overload takes none)
            let __lines: Vec<mcrust::net::minecraft::network::chat::formatted_text::FormattedText> = compat::read(__this, |__t| __t.level_bar_widget_base().tooltip.iter().cloned().map(mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from).collect());
            compat::batch_s_set_tooltip_for_next_frame(__lines);
        }
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput output)
    pub fn update_widget_narration_body(__this: &compat::Shared<dyn LevelBarWidget>, output: &dyn NarrationElementOutput) {
    }

    /// java: @Override public boolean shouldTakeFocusAfterInteraction()
    pub fn should_take_focus_after_interaction_body(__this: &compat::Shared<dyn LevelBarWidget>) -> bool {
        return false;
    }

}

impl LevelBarWidget for LevelBarWidgetBase {
    fn level_bar_widget_base(&self) -> &LevelBarWidgetBase {
        self
    }
    fn level_bar_widget_base_mut(&mut self) -> &mut LevelBarWidgetBase {
        self
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

}
