//! Ported from de.hysky.skyblocker.skyblock.tabhud.widget.PowderWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::itemlist::item_repository::ItemRepository;
use crate::de::hysky::skyblocker::skyblock::tabhud::config::option_widget_collector::OptionWidgetCollector;
use crate::de::hysky::skyblocker::skyblock::tabhud::util::ico;
use crate::de::hysky::skyblocker::skyblock::tabhud::util::player_list_manager::Widget;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::element::Element;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::element_collector::ElementCollector;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::elements::Elements;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element_based_widget::ElementBasedWidget;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element_based_widget::ElementBasedWidgetBase;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::hud_widget::HudWidget;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::hud_widget::HudWidgetBase;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::tab_hud_widget::TabHudWidget;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::tab_hud_widget::TabHudWidgetBase;
use crate::de::hysky::skyblocker::utils::formatters;
use crate::de::hysky::skyblocker::utils::json_value_input::JsonValueInput;
use crate::de::hysky::skyblocker::utils::json_value_output::JsonValueOutput;
use crate::de::hysky::skyblocker::utils::location::Location;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::util::util;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final MutableComponent TITLE = Component.literal("Powders").withStyle(ChatFormatting.DARK_AQUA, ChatFormatting.BOLD);
pub static TITLE: compat::MainThread<MutableComponent> = compat::MainThread::new(|| component::literal("Powders").with_style_formats(&[ChatFormatting::DarkAqua, ChatFormatting::Bold]));
/// java: private static final short UPDATE_INTERVAL = 2000;
pub const UPDATE_INTERVAL: i16 = (2000) as i16;
/// java: private static final Pattern MITHRIL_PATTERN = Pattern.compile("Mithril: ([\\d,]+)");
pub static MITHRIL_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("Mithril: ([\\d,]+)"));
/// java: private static final Pattern GEMSTONE_PATTERN = Pattern.compile("Gemstone: ([\\d,]+)");
pub static GEMSTONE_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("Gemstone: ([\\d,]+)"));
/// java: private static final Pattern GLACITE_PATTERN = Pattern.compile("Glacite: ([\\d,]+)");
pub static GLACITE_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("Glacite: ([\\d,]+)"));

/// java: @RegisterWidget public class PowderWidget extends TabHudWidget
#[derive(Clone)]
pub struct PowderWidget {
    /// java: extends TabHudWidget
    pub base: TabHudWidgetBase,
    /// java: private int lastMithril = 0;
    pub last_mithril: i32,
    /// java: private int lastGemstone = 0;
    pub last_gemstone: i32,
    /// java: private int lastGlacite = 0;
    pub last_glacite: i32,
    /// java: private int lastMithrilDiff = 0;
    pub last_mithril_diff: i32,
    /// java: private int lastGemstoneDiff = 0;
    pub last_gemstone_diff: i32,
    /// java: private int lastGlaciteDiff = 0;
    pub last_glacite_diff: i32,
    /// java: private byte updated = 0b0000;
    pub updated: i8,
    /// java: private long lastUpdate = 0;
    pub last_update: i64,
}

impl std::ops::Deref for PowderWidget {
    type Target = TabHudWidgetBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for PowderWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl PowderWidget {
    /// java: public PowderWidget()
    pub fn new() -> compat::Shared<Self> {
        // NOTE(fabric2seer): `lastGemstone` is read by `updateContent`, which TabHudWidget's constructor calls before this constructor assigns it: Java sees its default there (0 / false / null), and so does this port
        // NOTE(fabric2seer): `lastGemstoneDiff` is read by `updateContent`, which TabHudWidget's constructor calls before this constructor assigns it: Java sees its default there (0 / false / null), and so does this port
        // NOTE(fabric2seer): `lastGlacite` is read by `updateContent`, which TabHudWidget's constructor calls before this constructor assigns it: Java sees its default there (0 / false / null), and so does this port
        // NOTE(fabric2seer): `lastGlaciteDiff` is read by `updateContent`, which TabHudWidget's constructor calls before this constructor assigns it: Java sees its default there (0 / false / null), and so does this port
        // NOTE(fabric2seer): `lastMithril` is read by `updateContent`, which TabHudWidget's constructor calls before this constructor assigns it: Java sees its default there (0 / false / null), and so does this port
        // NOTE(fabric2seer): `lastMithrilDiff` is read by `updateContent`, which TabHudWidget's constructor calls before this constructor assigns it: Java sees its default there (0 / false / null), and so does this port
        // NOTE(fabric2seer): `lastUpdate` is read by `updateContent`, which TabHudWidget's constructor calls before this constructor assigns it: Java sees its default there (0 / false / null), and so does this port
        // NOTE(fabric2seer): `updated` is read by `updateContent`, which TabHudWidget's constructor calls before this constructor assigns it: Java sees its default there (0 / false / null), and so does this port
        let (_, __this) = TabHudWidgetBase::new_5_in("Powders".to_string(), TITLE.get_mut().clone(), mcrust::net::minecraft::network::chat::text_color::TextColor::DARK_AQUA.get_value(), Location::DwarvenMines, vec![Location::CrystalHollows, Location::GlaciteMineshafts], move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                last_mithril: 0,
                last_gemstone: 0,
                last_glacite: 0,
                last_mithril_diff: 0,
                last_gemstone_diff: 0,
                last_glacite_diff: 0,
                updated: (0) as i8,
                last_update: ((0) as i64) as i64,
            });
            ((__h.clone() as compat::Shared<dyn TabHudWidget>), __h)
        });
        __this
    }

    /// java: private int parseAmount(Matcher matcher)
    pub fn parse_amount(__this: &compat::Shared<Self>, matcher: &compat::regex::Matcher) -> i32 {
        // TODO(fabric2seer): static call NumberUtils.toInt(..)
        //   java: return NumberUtils.toInt(matcher.group(1).replace(",", ""));
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: private MutableComponent getAppendix(int diff, ChatFormatting formatting)
    pub fn get_appendix(__this: &compat::Shared<Self>, mut diff: i32, formatting: &ChatFormatting) -> MutableComponent {
        return (component::literal(&format!(" ({})", formatters::DIFF_NUMBERS.get_mut().format((diff) as f64)))).with_style_format(*formatting);
    }

    /// java: private Component getTextToDisplay(int diff, Component line, ChatFormatting formatting)
    pub fn get_text_to_display(__this: &compat::Shared<Self>, mut diff: i32, line: &MutableComponent, formatting: &ChatFormatting) -> MutableComponent {
        return (if diff != 0 { line.copy().append(PowderWidget::get_appendix(__this, diff, formatting)) } else { line.clone() });
    }

}

impl TabHudWidget for PowderWidget {
    fn tab_hud_widget_base(&self) -> &TabHudWidgetBase {
        &self.base
    }
    fn tab_hud_widget_base_mut(&mut self) -> &mut TabHudWidgetBase {
        &mut self.base
    }

    /// java: @Override public void updateContent(PlayerListManager.Widget widget) (dispatch: the body for this class)
    fn update_content_1(&self) -> fn(&compat::Shared<dyn TabHudWidget>, &Widget) {
        PowderWidget::update_content_1_dyn
    }

    /// java: @Override protected void updateConfigContentTab(ElementCollector collector) (dispatch: the body for this class)
    fn update_config_content_tab(&self) -> fn(&compat::Shared<dyn TabHudWidget>, &compat::Shared<dyn ElementCollector>) {
        PowderWidget::update_config_content_tab_dyn
    }

}

impl ElementBasedWidget for PowderWidget {
    fn element_based_widget_base(&self) -> &ElementBasedWidgetBase {
        &self.base.base
    }
    fn element_based_widget_base_mut(&mut self) -> &mut ElementBasedWidgetBase {
        &mut self.base.base
    }

    /// java: @Override public void updateContent() (dispatch: the body for this class)
    fn update_content(&self) -> fn(&compat::Shared<dyn ElementBasedWidget>) {
        <dyn TabHudWidget>::update_content_from::<Self>
    }

    /// java: @Override protected final void updateConfigContent(ElementCollector collector) (dispatch: the body for this class)
    fn update_config_content(&self) -> fn(&compat::Shared<dyn ElementBasedWidget>, &compat::Shared<dyn ElementCollector>) {
        <dyn TabHudWidget>::update_config_content_from::<Self>
    }

}

impl HudWidget for PowderWidget {
    fn hud_widget_base(&self) -> &HudWidgetBase {
        &self.base.base.base
    }
    fn hud_widget_base_mut(&mut self) -> &mut HudWidgetBase {
        &mut self.base.base.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override protected final void extractWidgetRenderState(GuiGraphicsExtractor context, float delta) (dispatch: the body for this class)
    fn extract_widget_render_state(&self) -> fn(&compat::Shared<dyn HudWidget>, &mut GuiGraphicsExtractor, f32) {
        <dyn ElementBasedWidget>::extract_widget_render_state_from::<Self>
    }

    /// java: @Override protected void extractWidgetRenderStateForConfig(GuiGraphicsExtractor graphics, float delta) (dispatch: the body for this class)
    fn extract_widget_render_state_for_config(&self) -> fn(&compat::Shared<dyn HudWidget>, &mut GuiGraphicsExtractor, f32) {
        <dyn ElementBasedWidget>::extract_widget_render_state_for_config_from::<Self>
    }

    /// java: @Override public boolean shouldRender() (dispatch: the body for this class)
    fn should_render(&self) -> fn(&compat::Shared<dyn HudWidget>) -> bool {
        <dyn TabHudWidget>::should_render_from::<Self>
    }

    /// java: @Override public void load(JsonValueInput input) (dispatch: the body for this class)
    fn load(&self) -> fn(&compat::Shared<dyn HudWidget>, &JsonValueInput) {
        <dyn TabHudWidget>::load_from::<Self>
    }

    /// java: @Override public void save(JsonValueOutput output) (dispatch: the body for this class)
    fn save(&self) -> fn(&compat::Shared<dyn HudWidget>, &mut JsonValueOutput) {
        <dyn TabHudWidget>::save_from::<Self>
    }

    /// java: @Override public void getOptionWidgets(OptionWidgetCollector collector) (dispatch: the body for this class)
    fn get_option_widgets(&self) -> fn(&compat::Shared<dyn HudWidget>, &mut OptionWidgetCollector) {
        <dyn TabHudWidget>::get_option_widgets_from::<Self>
    }

    /// java: @Override public void updateConfigPreview() (dispatch: the body for this class)
    fn update_config_preview(&self) -> fn(&compat::Shared<dyn HudWidget>) {
        <dyn TabHudWidget>::update_config_preview_from::<Self>
    }

}

impl ElementCollector for PowderWidget {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: public <T extends Element> T addElement(T c)
    fn add_element(&mut self, mut c: compat::Shared<dyn Element>) -> compat::Shared<dyn Element> {
        ({ let __f = compat::dispatch(&c, |__t| __t.set_parent()); __f(&c, self.hud_widget_base().as_parent()) });
        self.elements.push(c.clone());
        return c;
    }

}

impl PowderWidget {
    /// java: @Override public void updateContent(PlayerListManager.Widget widget)
    pub fn update_content_1_impl(__this: &compat::Shared<Self>, widget: &Widget) {
        let mut matcher: compat::regex::Matcher = compat::regex::Pattern::compile("").matcher("");
        let mut ms_after_last_update: i64 = (util::get_millis()).wrapping_sub(compat::read(__this, |__t| __t.last_update));
        for line in widget.lines() {
            if true && matcher.reset_with(&line.get_string()).use_pattern(&MITHRIL_PATTERN).matches() {
                let mut mithril: i32 = PowderWidget::parse_amount(__this, &matcher.reset_with(&line.get_string()));
                if mithril != compat::read(__this, |__t| __t.last_mithril) || ms_after_last_update > (UPDATE_INTERVAL as i64) {
                    { let __v = mithril.wrapping_sub(compat::read(__this, |__t| __t.last_mithril)); compat::write(__this, |__t| __t.last_mithril_diff = __v); }
                    { let __v = (((compat::read(__this, |__t| __t.updated) as i32) | 8)) as i8; compat::write(__this, |__t| __t.updated = __v); }
                    let __r0 = compat::read(__this, |__t| __t.last_mithril_diff);
                    { let (__a0,) = (Elements::icon_text_component_2(Some(ItemRepository::get_item_stack_2("MITHRIL_ORE", ico::MITHRIL.get_mut())), Some(PowderWidget::get_text_to_display(__this, __r0, &line, &ChatFormatting::DarkGreen))),); __this.lock().borrow_mut().add_element(__a0) };
                    { let __v = mithril; compat::write(__this, |__t| __t.last_mithril = __v); }
                } else {
                    let __r0 = compat::read(__this, |__t| __t.last_mithril_diff);
                    { let (__a0,) = (Elements::icon_text_component_2(Some(ItemRepository::get_item_stack_2("MITHRIL_ORE", ico::MITHRIL.get_mut())), Some(PowderWidget::get_text_to_display(__this, __r0, &line, &ChatFormatting::DarkGreen))),); __this.lock().borrow_mut().add_element(__a0) };
                }
                { let __v = (((compat::read(__this, |__t| __t.updated) as i32) | 1)) as i8; compat::write(__this, |__t| __t.updated = __v); }
            } else if true && matcher.reset_with(&line.get_string()).use_pattern(&GEMSTONE_PATTERN).matches() {
                let mut gemstone: i32 = PowderWidget::parse_amount(__this, &matcher.reset_with(&line.get_string()));
                if gemstone != compat::read(__this, |__t| __t.last_gemstone) || ms_after_last_update > (UPDATE_INTERVAL as i64) {
                    { let __v = gemstone.wrapping_sub(compat::read(__this, |__t| __t.last_gemstone)); compat::write(__this, |__t| __t.last_gemstone_diff = __v); }
                    { let __v = (((compat::read(__this, |__t| __t.updated) as i32) | 8)) as i8; compat::write(__this, |__t| __t.updated = __v); }
                    let __r0 = compat::read(__this, |__t| __t.last_gemstone_diff);
                    { let (__a0,) = (Elements::icon_text_component_2(Some(ItemRepository::get_item_stack_2("GEMSTONE_COLLECTION", ico::GEMSTONE.get_mut())), Some(PowderWidget::get_text_to_display(__this, __r0, &line, &ChatFormatting::LightPurple))),); __this.lock().borrow_mut().add_element(__a0) };
                    { let __v = gemstone; compat::write(__this, |__t| __t.last_gemstone = __v); }
                } else {
                    let __r0 = compat::read(__this, |__t| __t.last_gemstone_diff);
                    { let (__a0,) = (Elements::icon_text_component_2(Some(ItemRepository::get_item_stack_2("GEMSTONE_COLLECTION", ico::GEMSTONE.get_mut())), Some(PowderWidget::get_text_to_display(__this, __r0, &line, &ChatFormatting::LightPurple))),); __this.lock().borrow_mut().add_element(__a0) };
                }
                { let __v = (((compat::read(__this, |__t| __t.updated) as i32) | 2)) as i8; compat::write(__this, |__t| __t.updated = __v); }
            } else if true && matcher.reset_with(&line.get_string()).use_pattern(&GLACITE_PATTERN).matches() {
                let mut glacite: i32 = PowderWidget::parse_amount(__this, &matcher.reset_with(&line.get_string()));
                if glacite != compat::read(__this, |__t| __t.last_glacite) || ms_after_last_update > (UPDATE_INTERVAL as i64) {
                    { let __v = glacite.wrapping_sub(compat::read(__this, |__t| __t.last_glacite)); compat::write(__this, |__t| __t.last_glacite_diff = __v); }
                    { let __v = (((compat::read(__this, |__t| __t.updated) as i32) | 8)) as i8; compat::write(__this, |__t| __t.updated = __v); }
                    let __r0 = compat::read(__this, |__t| __t.last_glacite_diff);
                    { let (__a0,) = (Elements::icon_text_component_2(Some(ItemRepository::get_item_stack_2("GLACITE", ico::PACKED_ICE.get_mut())), Some(PowderWidget::get_text_to_display(__this, __r0, &line, &ChatFormatting::Aqua))),); __this.lock().borrow_mut().add_element(__a0) };
                    { let __v = glacite; compat::write(__this, |__t| __t.last_glacite = __v); }
                } else {
                    let __r0 = compat::read(__this, |__t| __t.last_glacite_diff);
                    { let (__a0,) = (Elements::icon_text_component_2(Some(ItemRepository::get_item_stack_2("GLACITE", ico::PACKED_ICE.get_mut())), Some(PowderWidget::get_text_to_display(__this, __r0, &line, &ChatFormatting::Aqua))),); __this.lock().borrow_mut().add_element(__a0) };
                }
                { let __v = (((compat::read(__this, |__t| __t.updated) as i32) | 4)) as i8; compat::write(__this, |__t| __t.updated = __v); }
            } else {
            }
            if ((compat::read(__this, |__t| __t.updated) as i32) & 7) == 7 {
                break;
            }
        }
        if ((compat::read(__this, |__t| __t.updated) as i32) & 8) == 8 {
            { let __v = util::get_millis(); compat::write(__this, |__t| __t.last_update = __v); }
        }
        { let __v = (0) as i8; compat::write(__this, |__t| __t.updated = __v); }
    }

    /// PowderWidget.updateContent as the dispatch of TabHudWidget.updateContent
    pub fn update_content_1_dyn(__this: &compat::Shared<dyn TabHudWidget>, __p0: &Widget) {
        Self::update_content_1_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override protected void updateConfigContentTab(ElementCollector collector)
    pub fn update_config_content_tab_impl(__this: &compat::Shared<Self>, collector: &compat::Shared<dyn ElementCollector>) {
        let mut literal: MutableComponent = component::literal("???");
        { let (__a0,) = (Elements::icon_text_component_2(Some(ItemRepository::get_item_stack_2("MITHRIL_ORE", ico::MITHRIL.get_mut())), Some(literal.clone())),); collector.lock().borrow_mut().add_element(__a0) };
        { let (__a0,) = (Elements::icon_text_component_2(Some(ItemRepository::get_item_stack_2("GEMSTONE_COLLECTION", ico::GEMSTONE.get_mut())), Some(literal.clone())),); collector.lock().borrow_mut().add_element(__a0) };
        { let (__a0,) = (Elements::icon_text_component_2(Some(ItemRepository::get_item_stack_2("GLACITE", ico::PACKED_ICE.get_mut())), Some(literal.clone())),); collector.lock().borrow_mut().add_element(__a0) };
    }

    /// PowderWidget.updateConfigContentTab as the dispatch of TabHudWidget.updateConfigContentTab
    pub fn update_config_content_tab_dyn(__this: &compat::Shared<dyn TabHudWidget>, __p0: &compat::Shared<dyn ElementCollector>) {
        Self::update_config_content_tab_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

}
