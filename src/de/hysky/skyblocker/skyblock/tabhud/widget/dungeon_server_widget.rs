//! Ported from de.hysky.skyblocker.skyblock.tabhud.widget.DungeonServerWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::tabhud::config::option_widget_collector::OptionWidgetCollector;
use crate::de::hysky::skyblocker::skyblock::tabhud::util::ico;
use crate::de::hysky::skyblocker::skyblock::tabhud::util::player_list_manager::PlayerListManager;
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
use crate::de::hysky::skyblocker::utils::json_value_input::JsonValueInput;
use crate::de::hysky::skyblocker::utils::json_value_output::JsonValueOutput;
use crate::de::hysky::skyblocker::utils::location::Location;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: private static final MutableComponent TITLE = Component.literal("Dungeon Info").withStyle(ChatFormatting.GOLD, ChatFormatting.BOLD);
pub static TITLE: compat::MainThread<MutableComponent> = compat::MainThread::new(|| component::literal("Dungeon Info").with_style_formats(&[ChatFormatting::Gold, ChatFormatting::Bold]));
/// java: private static final Pattern SECRET_PATTERN = Pattern.compile("Secrets Found: (?<secnum>.*)%");
pub static SECRET_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("Secrets Found: (?<secnum>.*)%"));

/// java: @RegisterWidget public class DungeonServerWidget extends TabHudWidget
#[derive(Clone)]
pub struct DungeonServerWidget {
    /// java: extends TabHudWidget
    pub base: TabHudWidgetBase,
}

impl std::ops::Deref for DungeonServerWidget {
    type Target = TabHudWidgetBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for DungeonServerWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl DungeonServerWidget {
    /// java: public DungeonServerWidget()
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = TabHudWidgetBase::new3_in("Dungeon Info".to_string(), TITLE.get_mut().clone(), mcrust::net::minecraft::network::chat::text_color::TextColor::GOLD.get_value(), Location::Dungeon, move |__base| {
            let __h = compat::shared(Self {
                base: __base,
            });
            ((__h.clone() as compat::Shared<dyn TabHudWidget>), __h)
        });
        __this
    }

}

impl TabHudWidget for DungeonServerWidget {
    fn tab_hud_widget_base(&self) -> &TabHudWidgetBase {
        &self.base
    }
    fn tab_hud_widget_base_mut(&mut self) -> &mut TabHudWidgetBase {
        &mut self.base
    }

    /// java: @Override public void updateContent(PlayerListManager.Widget ignored) (dispatch: the body for this class)
    fn update_content_1(&self) -> fn(&compat::Shared<dyn TabHudWidget>, &Widget) {
        DungeonServerWidget::update_content_1_dyn
    }

}

impl ElementBasedWidget for DungeonServerWidget {
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

impl HudWidget for DungeonServerWidget {
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

impl ElementCollector for DungeonServerWidget {
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

impl DungeonServerWidget {
    /// java: @Override public void updateContent(PlayerListManager.Widget ignored)
    pub fn update_content_1_impl(__this: &compat::Shared<Self>, ignored: &Widget) {
        ({ let __f = compat::dispatch(__this, |__t| __t.add_simple_ico_text()); __f(&compat::view(__this, |a| a as compat::Shared<dyn ElementCollector>), Some(ico::NTAG.get_mut().clone()), "Name:", &ChatFormatting::Aqua, 41) });
        ({ let __f = compat::dispatch(__this, |__t| __t.add_simple_ico_text()); __f(&compat::view(__this, |a| a as compat::Shared<dyn ElementCollector>), Some(ico::SIGN.get_mut().clone()), "Rooms Visited:", &ChatFormatting::DarkPurple, 42) });
        ({ let __f = compat::dispatch(__this, |__t| __t.add_simple_ico_text()); __f(&compat::view(__this, |a| a as compat::Shared<dyn ElementCollector>), Some(ico::SIGN.get_mut().clone()), "Rooms Completed:", &ChatFormatting::LightPurple, 43) });
        let mut m: Option<compat::regex::Matcher> = PlayerListManager::regex_at(44, &SECRET_PATTERN);
        if let Some(m) = &m {
            { let (__a0,) = (Elements::progress_component_flexible_item_stack_component_float_int(Some(ico::CHEST.get_mut().clone()), &component::null_to_empty(Some("Secrets found:")), m.group_named("secnum").unwrap().trim().parse::<f32>().expect("NumberFormatException"), mcrust::net::minecraft::network::chat::text_color::TextColor::GOLD.get_value()),); __this.lock().borrow_mut().add_element(__a0) };
        } else {
            { let (__a0,) = (Elements::progress_component(),); __this.lock().borrow_mut().add_element(__a0) };
        }
        ({ let __f = compat::dispatch(__this, |__t| __t.add_simple_ico_text()); __f(&compat::view(__this, |a| a as compat::Shared<dyn ElementCollector>), Some(ico::CLOCK.get_mut().clone()), "Time:", &ChatFormatting::Gold, 45) });
    }

    /// DungeonServerWidget.updateContent as the dispatch of TabHudWidget.updateContent
    pub fn update_content_1_dyn(__this: &compat::Shared<dyn TabHudWidget>, __p0: &Widget) {
        Self::update_content_1_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

}
