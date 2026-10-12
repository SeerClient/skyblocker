//! Ported from de.hysky.skyblocker.skyblock.tabhud.widget.PetWidget by fabric2seer.

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
use mcrust::com::google::common::cache::loading_cache::LoadingCache;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final MutableComponent TITLE = Component.literal("Pet").withStyle(ChatFormatting.YELLOW, ChatFormatting.BOLD);
pub static TITLE: compat::MainThread<MutableComponent> = compat::MainThread::new(|| component::literal("Pet").with_style_formats(&[ChatFormatting::Yellow, ChatFormatting::Bold]));
// TODO(fabric2seer): static call CacheBuilder.newBuilder(..)
//   java: private static final LoadingCache<String, FlexibleItemStack> PET_ICON_CACHE = CacheBuilder.newBuilder()
//   java: 			.expireAfterWrite(Duration.ofMinutes(5L))
//   java: 			.build(new CacheLoader<>() {
//   java: 				@Override
//   java: 				public FlexibleItemStack load(String key) throws Exception {
//   java: 					return getIcon(key);
//   java: 				}
//   java: 			});
/// java: private static final LoadingCache<String, FlexibleItemStack> PET_ICON_CACHE = CacheBuilder.newBuilder() .expireAfterWrite(Duration.ofMinutes(5L)) .build(new CacheLoader<>() { @Override public Flexible
pub static PET_ICON_CACHE: compat::MainThread<LoadingCache<String, compat::Shared<FlexibleItemStack>, compat::Object>> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `PET_ICON_CACHE` - see the TODO above"));

/// java: @RegisterWidget public class PetWidget extends TabHudWidget
#[derive(Clone)]
pub struct PetWidget {
    /// java: extends TabHudWidget
    pub base: TabHudWidgetBase,
    /// java: private String prevString = "";
    pub prev_string: String,
    /// java: private FlexibleItemStack icon = Ico.BONE;
    pub icon: compat::Shared<FlexibleItemStack>,
}

impl std::ops::Deref for PetWidget {
    type Target = TabHudWidgetBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for PetWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl PetWidget {
    /// java: public PetWidget()
    pub fn new() -> compat::Shared<Self> {
        // NOTE(fabric2seer): `icon` is read by `updateContent`, which TabHudWidget's constructor calls before this constructor assigns it: Java sees its default there (0 / false / null), and so does this port
        // NOTE(fabric2seer): `prevString` is read by `updateContent`, which TabHudWidget's constructor calls before this constructor assigns it: Java sees its default there (0 / false / null), and so does this port
        let (_, __this) = TabHudWidgetBase::new_3_in("Pet".to_string(), TITLE.get_mut().clone(), mcrust::net::minecraft::network::chat::text_color::TextColor::YELLOW.get_value(), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                prev_string: "".to_string(),
                icon: ico::BONE.get_mut().clone(),
            });
            ((__h.clone() as compat::Shared<dyn TabHudWidget>), __h)
        });
        __this
    }

    /// java: private static FlexibleItemStack getIcon(String petName)
    pub fn get_icon(pet_name: &str) -> compat::Shared<FlexibleItemStack> {
        return ItemRepository::get_items_stream().filter(|stack| {
            // TODO(fabric2seer): `.getString(..)` on a T
            //   java: String string1 = stack.get(DataComponents.CUSTOM_NAME).getString();
            let mut string1: String = todo!("fabric2seer: the initialiser of `string1` (TODO above)");
            if !(string1.contains("]")) {
                return false;
            }
            let mut trim: String = compat::trim(&((compat::regex::split(&string1, "]"))[(1) as usize]));
            return (trim == pet_name);
        }).next().unwrap_or(ico::BONE.get_mut().clone());
    }

}

impl TabHudWidget for PetWidget {
    fn tab_hud_widget_base(&self) -> &TabHudWidgetBase {
        &self.base
    }
    fn tab_hud_widget_base_mut(&mut self) -> &mut TabHudWidgetBase {
        &mut self.base
    }

    /// java: @Override protected void updateContent(PlayerListManager.Widget widget) (dispatch: the body for this class)
    fn update_content_1(&self) -> fn(&compat::Shared<dyn TabHudWidget>, &Widget) {
        PetWidget::update_content_1_dyn
    }

    /// java: @Override protected void updateConfigContentTab(ElementCollector collector) (dispatch: the body for this class)
    fn update_config_content_tab(&self) -> fn(&compat::Shared<dyn TabHudWidget>, &compat::Shared<dyn ElementCollector>) {
        PetWidget::update_config_content_tab_dyn
    }

}

impl ElementBasedWidget for PetWidget {
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

impl HudWidget for PetWidget {
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

impl ElementCollector for PetWidget {
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

impl PetWidget {
    /// java: @Override protected void updateContent(PlayerListManager.Widget widget)
    pub fn update_content_1_impl(__this: &compat::Shared<Self>, widget: &Widget) {
        for line in widget.lines() {
            let mut string: String = line.get_string();
            if string.contains("[") && string.contains("]") {
                let mut split: Vec<String> = compat::regex::split_limit(&string, "]", 2);
                if (split.len() as i32) < 2 {
                    { let (__a0,) = ((PlainTextElement::new(Some(line.clone())) as compat::Shared<dyn Element>),); __this.lock().borrow_mut().add_element(__a0) };
                    continue;
                }
                let mut pet_name: String = compat::trim(&((split[(1) as usize]).replace("✦", "")));
                if !(pet_name == compat::read(__this, |__t| __t.prev_string.clone())) {
                    // TODO(fabric2seer): `LoadingCache.getUnchecked` has no Rust method in the port (looked for `get_unchecked` in LoadingCache and its supertypes)
                    // rust: { let __v = PET_ICON_CACHE.get_mut().get_unchecked(&pet_name); compat::write(__this, |__t| __t.icon = __v); }
                    { let __v = pet_name.clone(); compat::write(__this, |__t| __t.prev_string = __v); }
                }
                let __r0 = compat::read(__this, |__t| __t.icon.clone());
                { let (__a0,) = (Elements::icon_text_component_2(Some(__r0), Some(line.clone())),); __this.lock().borrow_mut().add_element(__a0) };
            } else {
                { let (__a0,) = ((PlainTextElement::new(Some(line.clone())) as compat::Shared<dyn Element>),); __this.lock().borrow_mut().add_element(__a0) };
            }
        }
    }

    /// PetWidget.updateContent as the dispatch of TabHudWidget.updateContent
    pub fn update_content_1_dyn(__this: &compat::Shared<dyn TabHudWidget>, __p0: &Widget) {
        Self::update_content_1_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override protected void updateConfigContentTab(ElementCollector collector)
    pub fn update_config_content_tab_impl(__this: &compat::Shared<Self>, collector: &compat::Shared<dyn ElementCollector>) {
        { let (__a0,) = (Elements::icon_text_component_2(Some(ico::BONE.get_mut().clone()), Some((component::literal("[Lvl ???] ")).with_style_format(ChatFormatting::Gray).append(component::literal("Pet").with_style_format(ChatFormatting::Green)))),); collector.lock().borrow_mut().add_element(__a0) };
    }

    /// PetWidget.updateConfigContentTab as the dispatch of TabHudWidget.updateConfigContentTab
    pub fn update_config_content_tab_dyn(__this: &compat::Shared<dyn TabHudWidget>, __p0: &compat::Shared<dyn ElementCollector>) {
        Self::update_config_content_tab_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

}
