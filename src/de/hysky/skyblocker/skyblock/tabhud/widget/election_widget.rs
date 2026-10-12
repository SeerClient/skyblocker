//! Ported from de.hysky.skyblocker.skyblock.tabhud.widget.ElectionWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::tabhud::config::option_widget_collector::OptionWidgetCollector;
use crate::de::hysky::skyblocker::skyblock::tabhud::util::ico;
use crate::de::hysky::skyblocker::skyblock::tabhud::util::player_list_manager::Widget;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::element::Element;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::element_collector::ElementCollector;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::elements::Elements;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::plain_text_element::PlainTextElement;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element_based_widget::ElementBasedWidget;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element_based_widget::ElementBasedWidgetBase;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::hud_widget::HudWidget;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::hud_widget::HudWidgetBase;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::tab_hud_widget::TabHudWidget;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::tab_hud_widget::TabHudWidgetBase;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
use crate::de::hysky::skyblocker::utils::json_value_input::JsonValueInput;
use crate::de::hysky::skyblocker::utils::json_value_output::JsonValueOutput;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::network::chat::text_color::TextColor;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final MutableComponent TITLE = Component.literal("Election Info").withStyle(ChatFormatting.YELLOW, ChatFormatting.BOLD);
pub static TITLE: compat::MainThread<MutableComponent> = compat::MainThread::new(|| component::literal("Election Info").with_style_formats(&[ChatFormatting::Yellow, ChatFormatting::Bold]));
/// java: private static final Map<String, FlexibleItemStack> MAYOR_DATA = Map.ofEntries( Map.entry("Aatrox", Ico.DIA_SWORD), Map.entry("Cole", Ico.IRON_PICKAXE), Map.entry("Diana", Ico.BONE), Map.entry("Diaz",
pub static MAYOR_DATA: compat::MainThread<std::collections::HashMap<String, compat::Shared<FlexibleItemStack>>> = compat::MainThread::new(|| std::collections::HashMap::from([("Aatrox".to_string(), ico::DIA_SWORD.get_mut().clone()), ("Cole".to_string(), ico::IRON_PICKAXE.get_mut().clone()), ("Diana".to_string(), ico::BONE.get_mut().clone()), ("Diaz".to_string(), ico::GOLD.get_mut().clone()), ("Finnegan".to_string(), ico::IRON_HOE.get_mut().clone()), ("Foxy".to_string(), ico::SUGAR.get_mut().clone()), ("Paul".to_string(), ico::COMPASS.get_mut().clone()), ("Scorpius".to_string(), ico::GOLDEN_APPLE.get_mut().clone()), ("Jerry".to_string(), ico::VILLAGER.get_mut().clone()), ("Derpy".to_string(), ico::DBUSH.get_mut().clone()), ("Marina".to_string(), ico::FISH_ROD.get_mut().clone()), ("Aura".to_string(), ico::OMINOUS_BOTTLE.get_mut().clone())]));
/// java: private static final Component EL_OVER = Component.literal("Election: ") .append(Component.literal("Over!").withStyle(ChatFormatting.RED));
pub static EL_OVER: compat::MainThread<MutableComponent> = compat::MainThread::new(|| component::literal("Election: ").append((component::literal("Over!")).with_style_format(ChatFormatting::Red)));
/// java: private static final Pattern VOTE_PATTERN = Pattern.compile("(?<mayor>\\S*): \\|+ \\((?<pcnt>\\d*)%\\)");
pub static VOTE_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("(?<mayor>\\S*): \\|+ \\((?<pcnt>\\d*)%\\)"));
/// java: private static final ChatFormatting[] COLS = { ChatFormatting.RED, ChatFormatting.LIGHT_PURPLE, ChatFormatting.GREEN, ChatFormatting.AQUA, ChatFormatting.YELLOW };
pub static COLS: compat::MainThread<Vec<ChatFormatting>> = compat::MainThread::new(|| vec![ChatFormatting::Red, ChatFormatting::LightPurple, ChatFormatting::Green, ChatFormatting::Aqua, ChatFormatting::Yellow]);

/// java: @RegisterWidget public class ElectionWidget extends TabHudWidget
#[derive(Clone)]
pub struct ElectionWidget {
    /// java: extends TabHudWidget
    pub base: TabHudWidgetBase,
}

impl std::ops::Deref for ElectionWidget {
    type Target = TabHudWidgetBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for ElectionWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl ElectionWidget {
    /// java: public ElectionWidget()
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = TabHudWidgetBase::new_3_in("Election".to_string(), TITLE.get_mut().clone(), mcrust::net::minecraft::network::chat::text_color::TextColor::YELLOW.get_value(), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
            });
            ((__h.clone() as compat::Shared<dyn TabHudWidget>), __h)
        });
        __this
    }

}

impl TabHudWidget for ElectionWidget {
    fn tab_hud_widget_base(&self) -> &TabHudWidgetBase {
        &self.base
    }
    fn tab_hud_widget_base_mut(&mut self) -> &mut TabHudWidgetBase {
        &mut self.base
    }

    /// java: @Override public void updateContent(PlayerListManager.Widget widget) (dispatch: the body for this class)
    fn update_content_1(&self) -> fn(&compat::Shared<dyn TabHudWidget>, &Widget) {
        ElectionWidget::update_content_1_dyn
    }

    /// java: @Override protected void updateConfigContentTab(ElementCollector collector) (dispatch: the body for this class)
    fn update_config_content_tab(&self) -> fn(&compat::Shared<dyn TabHudWidget>, &compat::Shared<dyn ElementCollector>) {
        ElectionWidget::update_config_content_tab_dyn
    }

}

impl ElementBasedWidget for ElectionWidget {
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

impl HudWidget for ElectionWidget {
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

impl ElementCollector for ElectionWidget {
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

impl ElectionWidget {
    /// java: @Override public void updateContent(PlayerListManager.Widget widget)
    pub fn update_content_1_impl(__this: &compat::Shared<Self>, widget: &Widget) {
        let mut lines: Vec<MutableComponent> = widget.lines();
        let mut status: String = widget.detail().get_string();
        if status.contains("Over!") {
            { let (__a0,) = (Elements::icon_text_component_2(Some(ico::BARRIER.get_mut().clone()), Some(EL_OVER.get_mut().clone())),); __this.lock().borrow_mut().add_element(__a0) };
            for line in lines.iter() {
                { let (__a0,) = ((PlainTextElement::new(Some(line.clone())) as compat::Shared<dyn Element>),); __this.lock().borrow_mut().add_element(__a0) };
            }
        } else {
            ({ let __f = compat::dispatch(__this, |__t| __t.add_simple_ico_text_flexible_item_stack_string_chat_formatting_string()); __f(&compat::view(__this, |a| a as compat::Shared<dyn ElementCollector>), Some(ico::CLOCK.get_mut().clone()), "Ends in: ", &ChatFormatting::Gold, &compat::trim(&status)) });
            for i in 0..(lines.len() as i32) {
                let mut string: String = (lines[(i) as usize]).get_string();
                let mut m: compat::regex::Matcher = VOTE_PATTERN.matcher(&string);
                if m.matches() {
                    let mut mayorname: String = m.group_named("mayor").unwrap();
                    let mut pcntstr: String = m.group_named("pcnt").unwrap();
                    let mut pcnt: f32 = pcntstr.trim().parse::<f32>().expect("NumberFormatException");
                    let mut candidate: MutableComponent = component::literal(&mayorname).with_style_format(COLS.get_mut()[(i) as usize]);
                    { let (__a0,) = (Elements::progress_component_flexible_item_stack_component_float_int(MAYOR_DATA.get_mut().get(&mayorname).cloned(), &candidate, pcnt, (TextColor::from_legacy_format(COLS.get_mut()[(i) as usize]).unwrap()).get_value()),); __this.lock().borrow_mut().add_element(__a0) };
                } else {
                    { let (__a0,) = ((PlainTextElement::new(Some(lines[(i) as usize].clone())) as compat::Shared<dyn Element>),); __this.lock().borrow_mut().add_element(__a0) };
                }
            }
        }
    }

    /// ElectionWidget.updateContent as the dispatch of TabHudWidget.updateContent
    pub fn update_content_1_dyn(__this: &compat::Shared<dyn TabHudWidget>, __p0: &Widget) {
        Self::update_content_1_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override protected void updateConfigContentTab(ElementCollector collector)
    pub fn update_config_content_tab_impl(__this: &compat::Shared<Self>, collector: &compat::Shared<dyn ElementCollector>) {
        ({ let __f = compat::dispatch(collector, |__t| __t.add_simple_ico_text_flexible_item_stack_string_chat_formatting_string()); __f(collector, Some(ico::CLOCK.get_mut().clone()), "Ends in: ", &ChatFormatting::Gold, "???") });
        let mut progresses: Vec<f32> = vec![((60) as f32) as f32, ((30) as f32) as f32, ((1) as f32) as f32];
        for i in 0..3 {
            { let (__a0,) = (Elements::progress_component_3(Some(ico::VILLAGER.get_mut().clone()), &component::literal(&format!("Mayor {}", i.wrapping_add(1))), progresses[(i) as usize]),); collector.lock().borrow_mut().add_element(__a0) };
        }
    }

    /// ElectionWidget.updateConfigContentTab as the dispatch of TabHudWidget.updateConfigContentTab
    pub fn update_config_content_tab_dyn(__this: &compat::Shared<dyn TabHudWidget>, __p0: &compat::Shared<dyn ElementCollector>) {
        Self::update_config_content_tab_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

}
