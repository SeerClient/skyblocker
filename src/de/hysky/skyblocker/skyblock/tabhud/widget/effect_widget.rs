//! Ported from de.hysky.skyblocker.skyblock.tabhud.widget.EffectWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::itemlist::item_repository::ItemRepository;
use crate::de::hysky::skyblocker::skyblock::tabhud::config::option_widget_collector::OptionWidgetCollector;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::widget_manager::WidgetManager;
use crate::de::hysky::skyblocker::skyblock::tabhud::util::ico;
use crate::de::hysky::skyblocker::skyblock::tabhud::util::player_list_manager::PlayerListManager;
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
use crate::de::hysky::skyblocker::utils::json_value_input::JsonValueInput;
use crate::de::hysky::skyblocker::utils::json_value_output::JsonValueOutput;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final MutableComponent TITLE = Component.literal("Active Effects").withStyle(ChatFormatting.DARK_PURPLE, ChatFormatting.BOLD);
pub static TITLE: compat::MainThread<MutableComponent> = compat::MainThread::new(|| component::literal("Active Effects").with_style_formats(&[ChatFormatting::DarkPurple, ChatFormatting::Bold]));
/// java: private static final Pattern COOKIE_PATTERN = Pattern.compile(".*\\nCookie Buff\\n(?<buff>.*)\\n");
pub static COOKIE_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile(".*\\nCookie Buff\\n(?<buff>.*)\\n"));

/// java: @RegisterWidget public class EffectWidget extends TabHudWidget
#[derive(Clone)]
pub struct EffectWidget {
    /// java: extends TabHudWidget
    pub base: TabHudWidgetBase,
    /// java: private boolean effectsFromFooter = true;
    pub effects_from_footer: bool,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl std::ops::Deref for EffectWidget {
    type Target = TabHudWidgetBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for EffectWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl EffectWidget {
    /// java: public EffectWidget()
    pub fn new() -> compat::Shared<Self> {
        // NOTE(fabric2seer): `effectsFromFooter` is read by `updateContentMissing`, which TabHudWidget's constructor calls before this constructor assigns it: Java sees its default there (0 / false / null), and so does this port
        let (_, __this) = TabHudWidgetBase::new_3_in("Active Effects".to_string(), TITLE.get_mut().clone(), mcrust::net::minecraft::network::chat::text_color::TextColor::DARK_PURPLE.get_value(), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                self_ref: std::sync::Weak::new(),
                effects_from_footer: true,
            });
            compat::write(&__h, |__t| __t.self_ref = std::sync::Arc::downgrade(&__h));
            ((__h.clone() as compat::Shared<dyn TabHudWidget>), __h)
        });
        let __this_1 = compat::RenderThread::new(__this.clone());
        PlayerListManager::register_footer_listener(Arc::new(move || {
            if compat::read(&(*__this_1.get()), |__t| __t.effects_from_footer) && WidgetManager::is_widget_in_current_screen(&((*__this_1.get()).clone() as compat::Shared<dyn HudWidget>)) {
                ({ let __f = compat::dispatch(&(*__this_1.get()), |__t| __t.update()); __f(&compat::view(&(*__this_1.get()), |a| a as compat::Shared<dyn ElementBasedWidget>)) });
            }
        }));
        __this
    }

    /// java: private void fetchFromFooter()
    pub fn fetch_from_footer(__this: &compat::Shared<Self>) {
        let mut footertext: Option<String> = PlayerListManager::get_footer();
        if footertext.is_none() || !(footertext.as_ref().unwrap().contains("Active Effects")) {
            { let (__a0,) = (Elements::icon_text_component(),); __this.lock().borrow_mut().add_element(__a0) };
            return;
        }
        let mut m: compat::regex::Matcher = COOKIE_PATTERN.matcher(footertext.as_deref().unwrap());
        if m.find() && m.group_named("buff").is_some() {
            let mut buff: String = m.group_named("buff").unwrap();
            if buff.starts_with("Not") {
                { let (__a0,) = (Elements::icon_text_component_2(Some(ItemRepository::get_item_stack_2("BOOSTER_COOKIE", ico::COOKIE.get_mut())), Some(component::null_to_empty(Some("Cookie: not active")))),); __this.lock().borrow_mut().add_element(__a0) };
            } else {
                let mut cookie: MutableComponent = component::literal("Cookie: ").append_str(&buff);
                { let (__a0,) = (Elements::icon_text_component_2(Some(ItemRepository::get_item_stack_2("BOOSTER_COOKIE", ico::COOKIE.get_mut())), Some(cookie.clone())),); __this.lock().borrow_mut().add_element(__a0) };
            }
        }
        let mut lines: Vec<String> = compat::regex::split(&((compat::regex::split(&footertext.as_ref().unwrap(), "Active Effects"))[(1) as usize]), "\n");
        if (lines.len() as i32) < 2 {
            { let (__a0,) = (Elements::icon_text_component(),); __this.lock().borrow_mut().add_element(__a0) };
            return;
        }
        if (lines[(1) as usize]).starts_with("No") {
            let mut txt: MutableComponent = component::literal("No effects active").with_style_format(ChatFormatting::Gray);
            { let (__a0,) = (Elements::icon_text_component_2(Some(ico::POTION.get_mut().clone()), Some(txt.clone())),); __this.lock().borrow_mut().add_element(__a0) };
        } else if (lines[(1) as usize]).contains("God") {
            let mut timeleft: String = (compat::regex::split(&(lines[(1) as usize]), "! "))[(1) as usize].clone();
            let mut godpot: MutableComponent = (component::literal("God potion!")).with_style_format(ChatFormatting::Red);
            let mut txttleft: MutableComponent = component::literal(&timeleft).with_style_format(ChatFormatting::LightPurple);
            { let (__a0,) = (Elements::icon_fat_text_component_3(Some(ItemRepository::get_item_stack_2("GOD_POTION_2", ico::GOD_POTION.get_mut())), Some(godpot.clone()), Some(txttleft.clone())),); __this.lock().borrow_mut().add_element(__a0) };
        } else {
            let mut number: String = compat::substring(&(lines[(1) as usize]), compat::str_len("You have "));
            let mut idx: i32 = compat::index_of(&number, ' ');
            if idx == -1 || (lines.len() as i32) < 4 {
                { let (__a0,) = (Elements::icon_fat_text_component(),); __this.lock().borrow_mut().add_element(__a0) };
                return;
            }
            number = compat::substring2(&number, 0, idx);
            let mut active: MutableComponent = component::literal("Active Effects: ").append(component::literal(&number).with_style_format(ChatFormatting::Yellow));
            { let (__a0,) = (Elements::icon_fat_text_component_3(Some(ico::POTION.get_mut().clone()), Some(active.clone()), Some((component::literal(&lines[(2) as usize])).with_style_format(ChatFormatting::Aqua))),); __this.lock().borrow_mut().add_element(__a0) };
        }
    }

}

impl TabHudWidget for EffectWidget {
    fn tab_hud_widget_base(&self) -> &TabHudWidgetBase {
        &self.base
    }
    fn tab_hud_widget_base_mut(&mut self) -> &mut TabHudWidgetBase {
        &mut self.base
    }

    /// java: @Override public void updateContent(PlayerListManager.Widget widget) (dispatch: the body for this class)
    fn update_content_1(&self) -> fn(&compat::Shared<dyn TabHudWidget>, &Widget) {
        EffectWidget::update_content_1_dyn
    }

    /// java: @Override protected void updateContentMissing() (dispatch: the body for this class)
    fn update_content_missing(&self) -> fn(&compat::Shared<dyn TabHudWidget>) {
        EffectWidget::update_content_missing_dyn
    }

}

impl ElementBasedWidget for EffectWidget {
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

impl HudWidget for EffectWidget {
    fn hud_widget_base(&self) -> &HudWidgetBase {
        &self.base.base.base
    }
    fn hud_widget_base_mut(&mut self) -> &mut HudWidgetBase {
        &mut self.base.base.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void getOptionWidgets(OptionWidgetCollector collector) (dispatch: the body for this class)
    fn get_option_widgets(&self) -> fn(&compat::Shared<dyn HudWidget>, &mut OptionWidgetCollector) {
        EffectWidget::get_option_widgets_dyn
    }

    /// java: @Override public void load(JsonValueInput input) (dispatch: the body for this class)
    fn load(&self) -> fn(&compat::Shared<dyn HudWidget>, &JsonValueInput) {
        EffectWidget::load_dyn
    }

    /// java: @Override public void save(JsonValueOutput output) (dispatch: the body for this class)
    fn save(&self) -> fn(&compat::Shared<dyn HudWidget>, &mut JsonValueOutput) {
        EffectWidget::save_dyn
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

    /// java: @Override public void updateConfigPreview() (dispatch: the body for this class)
    fn update_config_preview(&self) -> fn(&compat::Shared<dyn HudWidget>) {
        <dyn TabHudWidget>::update_config_preview_from::<Self>
    }

}

impl ElementCollector for EffectWidget {
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

impl EffectWidget {
    /// java: @Override public void updateContent(PlayerListManager.Widget widget)
    pub fn update_content_1_impl(__this: &compat::Shared<Self>, widget: &Widget) {
        let mut string: String = compat::regex::replace_all(&widget.detail().get_string(), "[()]", "");
        { let (__a0,) = ((PlainTextElement::new(Some((component::literal(&string).with_style_formats(&[ChatFormatting::Yellow, ChatFormatting::Bold])).append(component::literal(" effect(s) active").with_style_format(ChatFormatting::White)))) as compat::Shared<dyn Element>),); __this.lock().borrow_mut().add_element(__a0) };
        for line in widget.lines() {
            { let (__a0,) = ((PlainTextElement::new(Some(line.clone())) as compat::Shared<dyn Element>),); __this.lock().borrow_mut().add_element(__a0) };
        }
    }

    /// EffectWidget.updateContent as the dispatch of TabHudWidget.updateContent
    pub fn update_content_1_dyn(__this: &compat::Shared<dyn TabHudWidget>, __p0: &Widget) {
        Self::update_content_1_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override protected void updateContentMissing()
    pub fn update_content_missing_impl(__this: &compat::Shared<Self>) {
        if compat::read(__this, |__t| __t.effects_from_footer) {
            EffectWidget::fetch_from_footer(__this);
        }
    }

    /// EffectWidget.updateContentMissing as the dispatch of TabHudWidget.updateContentMissing
    pub fn update_content_missing_dyn(__this: &compat::Shared<dyn TabHudWidget>) {
        Self::update_content_missing_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public void getOptionWidgets(OptionWidgetCollector collector)
    pub fn get_option_widgets_impl(__this: &compat::Shared<Self>, collector: &mut OptionWidgetCollector) {
        <dyn TabHudWidget>::get_option_widgets_body(&compat::view(__this, |a| a as compat::Shared<dyn TabHudWidget>), collector);
        let __this_w1 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        collector.yes_no_button(&component::translatable("skyblocker.config.uiAndVisuals.tabHud.effectsFooter"), Arc::new(move |b| { let Some(__this_1) = __this_w1.get().upgrade() else { return Default::default(); }; { { let __v = b; compat::write((&__this_1), |__t| __t.effects_from_footer = __v); } } }), compat::read(__this, |__t| __t.effects_from_footer), Some(component::translatable("skyblocker.config.uiAndVisuals.tabHud.effectsFooter.@Tooltip")));
    }

    /// EffectWidget.getOptionWidgets as the dispatch of HudWidget.getOptionWidgets
    pub fn get_option_widgets_dyn(__this: &compat::Shared<dyn HudWidget>, __p0: &mut OptionWidgetCollector) {
        Self::get_option_widgets_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public void load(JsonValueInput input)
    pub fn load_impl(__this: &compat::Shared<Self>, input: &JsonValueInput) {
        <dyn TabHudWidget>::load_body(&compat::view(__this, |a| a as compat::Shared<dyn TabHudWidget>), input);
        { let __v = input.read_boolean_or("effects_from_footer", false); compat::write(__this, |__t| __t.effects_from_footer = __v); }
    }

    /// EffectWidget.load as the dispatch of HudWidget.load
    pub fn load_dyn(__this: &compat::Shared<dyn HudWidget>, __p0: &JsonValueInput) {
        Self::load_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public void save(JsonValueOutput output)
    pub fn save_impl(__this: &compat::Shared<Self>, output: &mut JsonValueOutput) {
        <dyn TabHudWidget>::save_body(&compat::view(__this, |a| a as compat::Shared<dyn TabHudWidget>), output);
        output.write_bool("effects_from_footer", compat::read(__this, |__t| __t.effects_from_footer));
    }

    /// EffectWidget.save as the dispatch of HudWidget.save
    pub fn save_dyn(__this: &compat::Shared<dyn HudWidget>, __p0: &mut JsonValueOutput) {
        Self::save_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

}
