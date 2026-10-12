//! Ported from de.hysky.skyblocker.skyblock.tabhud.widget.DungeonBuffWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::tabhud::config::option_widget_collector::OptionWidgetCollector;
use crate::de::hysky::skyblocker::skyblock::tabhud::util::player_list_manager::PlayerListManager;
use crate::de::hysky::skyblocker::skyblock::tabhud::util::player_list_manager::Widget;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::element::Element;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::element_collector::ElementCollector;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::plain_text_element::PlainTextElement;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element_based_widget::ElementBasedWidget;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element_based_widget::ElementBasedWidgetBase;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::hud_widget::HudWidget;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::hud_widget::HudWidgetBase;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::hud_widget::Information;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::tab_hud_widget::TabHudWidget;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::tab_hud_widget::TabHudWidgetBase;
use crate::de::hysky::skyblocker::utils::json_value_input::JsonValueInput;
use crate::de::hysky::skyblocker::utils::json_value_output::JsonValueOutput;
use crate::de::hysky::skyblocker::utils::location::Location;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: private static final MutableComponent TITLE = Component.literal("Dungeon Buffs").withStyle(ChatFormatting.GREEN, ChatFormatting.BOLD);
pub static TITLE: compat::MainThread<MutableComponent> = compat::MainThread::new(|| component::literal("Dungeon Buffs").with_style_formats(&[ChatFormatting::Green, ChatFormatting::Bold]));

/// java: @RegisterWidget public class DungeonBuffWidget extends TabHudWidget
#[derive(Clone)]
pub struct DungeonBuffWidget {
    /// java: extends TabHudWidget
    pub base: TabHudWidgetBase,
}

impl std::ops::Deref for DungeonBuffWidget {
    type Target = TabHudWidgetBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for DungeonBuffWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl DungeonBuffWidget {
    /// java: public DungeonBuffWidget()
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = TabHudWidgetBase::new_in("Dungeon Buffs".to_string(), TITLE.get_mut().clone(), mcrust::net::minecraft::network::chat::text_color::TextColor::GREEN.get_value(), Information::new3("dungeon_buffs".to_string(), component::literal("Dungeon Buffs"), Location::Dungeon), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
            });
            ((__h.clone() as compat::Shared<dyn TabHudWidget>), __h)
        });
        __this
    }

    /// java: public int getBlessingColor(String blessing)
    pub fn get_blessing_color(__this: &compat::Shared<Self>, blessing: &str) -> i32 {
        if blessing.contains("Life") {
            return mcrust::net::minecraft::network::chat::text_color::TextColor::LIGHT_PURPLE.get_value();
        }
        if blessing.contains("Power") {
            return mcrust::net::minecraft::network::chat::text_color::TextColor::RED.get_value();
        }
        if blessing.contains("Stone") {
            return mcrust::net::minecraft::network::chat::text_color::TextColor::GREEN.get_value();
        }
        if blessing.contains("Time") {
            return 0xAFB8C1;
        }
        if blessing.contains("Wisdom") {
            return mcrust::net::minecraft::network::chat::text_color::TextColor::AQUA.get_value();
        }
        return 0xFFFFFF;
    }

}

impl TabHudWidget for DungeonBuffWidget {
    fn tab_hud_widget_base(&self) -> &TabHudWidgetBase {
        &self.base
    }
    fn tab_hud_widget_base_mut(&mut self) -> &mut TabHudWidgetBase {
        &mut self.base
    }

    /// java: @Override public void updateContent(PlayerListManager.Widget ignored) (dispatch: the body for this class)
    fn update_content_1(&self) -> fn(&compat::Shared<dyn TabHudWidget>, &Widget) {
        DungeonBuffWidget::update_content_1_dyn
    }

}

impl ElementBasedWidget for DungeonBuffWidget {
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

impl HudWidget for DungeonBuffWidget {
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

impl ElementCollector for DungeonBuffWidget {
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

impl DungeonBuffWidget {
    /// java: @Override public void updateContent(PlayerListManager.Widget ignored)
    pub fn update_content_1_impl(__this: &compat::Shared<Self>, ignored: &Widget) {
        let mut footer: Option<String> = PlayerListManager::get_footer();
        if footer.is_none() || !(footer.as_ref().unwrap().contains("Dungeon Buffs")) {
            { let (__a0,) = ((PlainTextElement::new(Some(component::literal("No data").with_style_format(ChatFormatting::Gray))) as compat::Shared<dyn Element>),); __this.lock().borrow_mut().add_element(__a0) };
            return;
        }
        let mut lines: Vec<String> = compat::regex::split(&((compat::regex::split(&footer.as_ref().unwrap(), "Dungeon Buffs"))[(1) as usize]), "\n");
        if !((lines[(1) as usize]).starts_with("Blessing")) {
            { let (__a0,) = ((PlainTextElement::new(Some((component::literal("No buffs found!")).with_style_format(ChatFormatting::Gray))) as compat::Shared<dyn Element>),); __this.lock().borrow_mut().add_element(__a0) };
            return;
        }
        lines = lines.iter().cloned().filter(|s| s.contains("Blessing")).collect::<Vec<_>>();
        lines.sort_by(|a, b| { let key = |__a0: &String| __a0.to_lowercase(); key(a).cmp(&key(b)) });
        for line in lines.iter() {
            if compat::str_len(&line) < 3 {
                break;
            }
            let mut color: i32 = DungeonBuffWidget::get_blessing_color(__this, line);
            { let (__a0,) = ((PlainTextElement::new(Some(component::literal(line).with_color(color))) as compat::Shared<dyn Element>),); __this.lock().borrow_mut().add_element(__a0) };
        }
    }

    /// DungeonBuffWidget.updateContent as the dispatch of TabHudWidget.updateContent
    pub fn update_content_1_dyn(__this: &compat::Shared<dyn TabHudWidget>, __p0: &Widget) {
        Self::update_content_1_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

}
