//! Ported from de.hysky.skyblocker.skyblock.tabhud.widget.JacobsContestWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::itemlist::item_repository::ItemRepository;
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
use crate::de::hysky::skyblocker::utils::location::Location;
use crate::de::hysky::skyblocker::utils::sky_block_icons;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final MutableComponent TITLE = Component.literal("Jacob's Contest").withStyle(ChatFormatting.YELLOW, ChatFormatting.BOLD);
pub static TITLE: compat::MainThread<MutableComponent> = compat::MainThread::new(|| component::literal("Jacob's Contest").with_style_formats(&[ChatFormatting::Yellow, ChatFormatting::Bold]));
/// java: private static final Pattern CROP_PATTERN = Pattern.compile(String.format("(?<fortune>[%s○]) (?<crop>.+?)(?: ◆ )?(?<percentage>Top [\\d.]+%%)?", SkyBlockIcons.FARMING_FORTUNE));
pub static CROP_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile(&format!("(?<fortune>[{}○]) (?<crop>.+?)(?: ◆ )?(?<percentage>Top [\\d.]+%)?", sky_block_icons::FARMING_FORTUNE)));
/// java: public static final Map<String, FlexibleItemStack> FARM_DATA = Map.ofEntries( entry("Wheat", Ico.WHEAT), entry("Carrot", Ico.CARROT), entry("Potato", Ico.POTATO), entry("Pumpkin", Ico.PUMPKIN), entry(
pub static FARM_DATA: compat::MainThread<std::collections::HashMap<String, compat::Shared<FlexibleItemStack>>> = compat::MainThread::new(|| std::collections::HashMap::from([("Wheat".to_string(), ico::WHEAT.get_mut().clone()), ("Carrot".to_string(), ico::CARROT.get_mut().clone()), ("Potato".to_string(), ico::POTATO.get_mut().clone()), ("Pumpkin".to_string(), ico::PUMPKIN.get_mut().clone()), ("Melon Slice".to_string(), ico::MELON_SLICE.get_mut().clone()), ("Mushroom".to_string(), ico::RED_MUSHROOM.get_mut().clone()), ("Cactus".to_string(), ico::CACTUS.get_mut().clone()), ("Sugar Cane".to_string(), ico::SUGAR_CANE.get_mut().clone()), ("Nether Wart".to_string(), ico::NETHER_WART.get_mut().clone()), ("Cocoa Beans".to_string(), ico::COCOA_BEANS.get_mut().clone()), ("Sunflower".to_string(), ItemRepository::get_item_stack_2("DOUBLE_PLANT", ico::SUNFLOWER.get_mut())), ("Moonflower".to_string(), ItemRepository::get_item_stack_2("MOONFLOWER", ico::BLUE_ORCHID.get_mut())), ("Wild Rose".to_string(), ItemRepository::get_item_stack_2("WILD_ROSE", ico::ROSE_BUSH.get_mut()))]));

/// java: @RegisterWidget public class JacobsContestWidget extends TabHudWidget
#[derive(Clone)]
pub struct JacobsContestWidget {
    /// java: extends TabHudWidget
    pub base: TabHudWidgetBase,
}

impl std::ops::Deref for JacobsContestWidget {
    type Target = TabHudWidgetBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for JacobsContestWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl JacobsContestWidget {
    /// java: public JacobsContestWidget()
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = TabHudWidgetBase::new_5_in("Jacob's Contest".to_string(), TITLE.get_mut().clone(), mcrust::net::minecraft::network::chat::text_color::TextColor::YELLOW.get_value(), Location::Hub, vec![Location::TheFarmingIsland, Location::Garden], move |__base| {
            let __h = compat::shared(Self {
                base: __base,
            });
            ((__h.clone() as compat::Shared<dyn TabHudWidget>), __h)
        });
        __this
    }

}

impl TabHudWidget for JacobsContestWidget {
    fn tab_hud_widget_base(&self) -> &TabHudWidgetBase {
        &self.base
    }
    fn tab_hud_widget_base_mut(&mut self) -> &mut TabHudWidgetBase {
        &mut self.base
    }

    /// java: @Override public void updateContent(PlayerListManager.Widget widget) (dispatch: the body for this class)
    fn update_content_1(&self) -> fn(&compat::Shared<dyn TabHudWidget>, &Widget) {
        JacobsContestWidget::update_content_1_dyn
    }

    /// java: @Override protected void updateConfigContentTab(ElementCollector collector) (dispatch: the body for this class)
    fn update_config_content_tab(&self) -> fn(&compat::Shared<dyn TabHudWidget>, &compat::Shared<dyn ElementCollector>) {
        JacobsContestWidget::update_config_content_tab_dyn
    }

}

impl ElementBasedWidget for JacobsContestWidget {
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

impl HudWidget for JacobsContestWidget {
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

impl ElementCollector for JacobsContestWidget {
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

impl JacobsContestWidget {
    /// java: @Override public void updateContent(PlayerListManager.Widget widget)
    pub fn update_content_1_impl(__this: &compat::Shared<Self>, widget: &Widget) {
        if widget.lines().is_empty() {
            { let (__a0,) = ((PlainTextElement::new(Some(widget.detail())) as compat::Shared<dyn Element>),); __this.lock().borrow_mut().add_element(__a0) };
        }
        if widget.detail().get_string().contains("left") {
            { let (__a0,) = (Elements::icon_text_component_2(Some(ico::CLOCK.get_mut().clone()), Some(widget.detail())),); __this.lock().borrow_mut().add_element(__a0) };
        }
        for line in widget.lines() {
            let mut string: String = line.get_string();
            if string.contains("Starts") {
                { let (__a0,) = (Elements::icon_text_component_2(Some(ico::CLOCK.get_mut().clone()), Some(line.clone())),); __this.lock().borrow_mut().add_element(__a0) };
            } else {
                let mut matcher: compat::regex::Matcher = CROP_PATTERN.matcher(&string);
                if matcher.matches() {
                    let mut crop: String = matcher.group_named("crop").unwrap();
                    let mut percentage: Option<String> = matcher.group_named("percentage");
                    let mut crop_text: MutableComponent = component::empty().append_str(&crop);
                    if matcher.group_named("fortune").as_deref() == Some(&*(sky_block_icons::FARMING_FORTUNE.to_string())) {
                        crop_text.append((component::literal(&format!(" {}", sky_block_icons::FARMING_FORTUNE))).with_style_format(ChatFormatting::Gold));
                    }
                    { let (__a0,) = (Elements::icon_text_component_2(FARM_DATA.get_mut().get(&crop).cloned(), Some(crop_text.clone())),); __this.lock().borrow_mut().add_element(__a0) };
                    if let Some(percentage) = &percentage {
                        { let (__a0,) = ((PlainTextElement::new(Some(component::literal(percentage))) as compat::Shared<dyn Element>),); __this.lock().borrow_mut().add_element(__a0) };
                    }
                } else {
                    { let (__a0,) = ((PlainTextElement::new(Some(line.clone())) as compat::Shared<dyn Element>),); __this.lock().borrow_mut().add_element(__a0) };
                }
            }
        }
    }

    /// JacobsContestWidget.updateContent as the dispatch of TabHudWidget.updateContent
    pub fn update_content_1_dyn(__this: &compat::Shared<dyn TabHudWidget>, __p0: &Widget) {
        Self::update_content_1_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override protected void updateConfigContentTab(ElementCollector collector)
    pub fn update_config_content_tab_impl(__this: &compat::Shared<Self>, collector: &compat::Shared<dyn ElementCollector>) {
        { let (__a0,) = (Elements::icon_text_component_2(Some(ico::CLOCK.get_mut().clone()), Some(component::literal("Starts in: ").append((component::literal("???")).with_style_format(ChatFormatting::Yellow)))),); collector.lock().borrow_mut().add_element(__a0) };
        let mut strings: Vec<compat::Shared<FlexibleItemStack>> = FARM_DATA.get_mut().values().cloned().collect::<Vec<_>>();
        for i in 0..3 {
            { let (__a0,) = (Elements::icon_text_component_2(Some(strings[(i) as usize].clone()), Some(component::literal(&format!("Crop {}", i.wrapping_add(1))))),); __this.lock().borrow_mut().add_element(__a0) };
        }
    }

    /// JacobsContestWidget.updateConfigContentTab as the dispatch of TabHudWidget.updateConfigContentTab
    pub fn update_config_content_tab_dyn(__this: &compat::Shared<dyn TabHudWidget>, __p0: &compat::Shared<dyn ElementCollector>) {
        Self::update_config_content_tab_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

}
