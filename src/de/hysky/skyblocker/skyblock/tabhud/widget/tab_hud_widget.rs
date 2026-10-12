//! Ported from de.hysky.skyblocker.skyblock.tabhud.widget.TabHudWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::tabhud::config::option_widget_collector::OptionWidgetCollector;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::widget_manager::WidgetManager;
use crate::de::hysky::skyblocker::skyblock::tabhud::tab_hud;
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
use crate::de::hysky::skyblocker::utils::json_value_input::JsonValueInput;
use crate::de::hysky::skyblocker::utils::json_value_output::JsonValueOutput;
use crate::de::hysky::skyblocker::utils::location::Location;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::key_mapping;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: public abstract class TabHudWidget extends ElementBasedWidget
#[derive(Clone)]
pub struct TabHudWidgetBase {
    /// java: extends ElementBasedWidget
    pub base: ElementBasedWidgetBase,
    /// java: private final String hypixelWidgetName;
    pub hypixel_widget_name: String,
    /// java: protected boolean hideIfHypixelWidgetMissing;
    pub hide_if_hypixel_widget_missing: bool,
    /// java: protected boolean hideIfMissingByDefault = false;
    pub hide_if_missing_by_default: bool,
    /// java: protected boolean hide;
    pub hide: bool,
}

impl std::ops::Deref for TabHudWidgetBase {
    type Target = ElementBasedWidgetBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for TabHudWidgetBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl TabHudWidgetBase {
    /// java: public TabHudWidget(String hypixelWidgetName, MutableComponent title, @Nullable Integer colorValue, Information information) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_in<__R>(mut hypixel_widget_name: String, mut title: MutableComponent, mut color_value: i32, mut information: Information, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn TabHudWidget>, __R)) -> (compat::Shared<dyn TabHudWidget>, __R) {
        let __init_hypixel_widget_name = hypixel_widget_name.clone();
        let (_, (__this, __r)) = ElementBasedWidgetBase::new_in(title.clone(), color_value, information.clone(), move |__base| {
            let this = Self {
                base: __base,
                hypixel_widget_name: __init_hypixel_widget_name,
                hide_if_hypixel_widget_missing: false,
                hide_if_missing_by_default: false,
                hide: false,
            };
            let (__h, __r) = __wrap(this);
            ((__h.clone() as compat::Shared<dyn ElementBasedWidget>), (__h, __r))
        });
        ({ let __f = compat::dispatch(&__this, |__t| __t.register_auto_update()); __f(&__this) });
        PlayerListManager::add_handled_tab_widget(hypixel_widget_name.clone(), (__this.clone() as compat::Shared<dyn HudWidget>));
        (__this, __r)
    }

    /// java: public TabHudWidget(String hypixelWidgetName, MutableComponent title, @Nullable Integer colorValue) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_3_in<__R>(mut hypixel_widget_name: String, mut title: MutableComponent, mut color_value: i32, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn TabHudWidget>, __R)) -> (compat::Shared<dyn TabHudWidget>, __R) {
        let (__this, __r) = Self::new_in(hypixel_widget_name.clone(), title.clone(), color_value, Information::new(Some(HudWidgetBase::name_to_id(&hypixel_widget_name)), Some(title.plain_copy())), __wrap);
        (__this, __r)
    }

    /// java: public TabHudWidget(String hypixelWidgetName, MutableComponent title, @Nullable Integer colorValue, Location location) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new3_in<__R>(mut hypixel_widget_name: String, mut title: MutableComponent, mut color_value: i32, mut location: Location, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn TabHudWidget>, __R)) -> (compat::Shared<dyn TabHudWidget>, __R) {
        let (__this, __r) = Self::new_in(hypixel_widget_name.clone(), title.clone(), color_value, Information::new3(HudWidgetBase::name_to_id(&hypixel_widget_name), title.plain_copy(), location), __wrap);
        (__this, __r)
    }

    /// java: public TabHudWidget(String hypixelWidgetName, MutableComponent title, @Nullable Integer colorValue, Location location, Location... otherLocations) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_5_in<__R>(mut hypixel_widget_name: String, mut title: MutableComponent, mut color_value: i32, mut location: Location, mut other_locations: Vec<Location>, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn TabHudWidget>, __R)) -> (compat::Shared<dyn TabHudWidget>, __R) {
        let (__this, __r) = Self::new_in(hypixel_widget_name.clone(), title.clone(), color_value, Information::new_4(HudWidgetBase::name_to_id(&hypixel_widget_name), title.plain_copy(), location, other_locations.clone()), __wrap);
        (__this, __r)
    }

}

/// java: the virtual methods of TabHudWidget (implemented by TabHudWidgetBase and every subclass)
pub trait TabHudWidget: ElementBasedWidget {
    fn tab_hud_widget_base(&self) -> &TabHudWidgetBase;
    fn tab_hud_widget_base_mut(&mut self) -> &mut TabHudWidgetBase;

    /// java: protected void registerAutoUpdate() (dispatch: the body for this class)
    fn register_auto_update(&self) -> fn(&compat::Shared<dyn TabHudWidget>) {
        <dyn TabHudWidget>::register_auto_update_body
    }

    /// java: public String getHypixelWidgetName() (dispatch: the body for this class)
    fn get_hypixel_widget_name(&self) -> fn(&compat::Shared<dyn TabHudWidget>) -> String {
        <dyn TabHudWidget>::get_hypixel_widget_name_body
    }

    /// java: protected abstract void updateContent(PlayerListManager.Widget widget); (dispatch: the body for this class)
    fn update_content_1(&self) -> fn(&compat::Shared<dyn TabHudWidget>, &Widget);

    /// java: protected void updateContentMissing() (dispatch: the body for this class)
    fn update_content_missing(&self) -> fn(&compat::Shared<dyn TabHudWidget>) {
        <dyn TabHudWidget>::update_content_missing_body
    }

    /// java: protected void updateConfigContentTab(ElementCollector collector) (dispatch: the body for this class)
    fn update_config_content_tab(&self) -> fn(&compat::Shared<dyn TabHudWidget>, &compat::Shared<dyn ElementCollector>) {
        <dyn TabHudWidget>::update_config_content_tab_body
    }

    /// java: public List<Component> createErrorMessage() (dispatch: the body for this class)
    fn create_error_message(&self) -> fn(&compat::Shared<dyn TabHudWidget>) -> Vec<MutableComponent> {
        <dyn TabHudWidget>::create_error_message_body
    }

}

impl dyn TabHudWidget {
    /// java: protected void registerAutoUpdate()
    pub fn register_auto_update_body(__this: &compat::Shared<dyn TabHudWidget>) {
        let __this_1 = compat::RenderThread::new(__this.clone());
        PlayerListManager::register_tab_listener(Arc::new(move || {
            if WidgetManager::is_widget_in_current_screen(&(__this_1.get().clone() as compat::Shared<dyn HudWidget>)) {
                ({ let __f = compat::dispatch(__this_1.get(), |__t| __t.update()); __f(&compat::view(__this_1.get(), |a| a as compat::Shared<dyn ElementBasedWidget>)) });
            }
        }));
    }

    /// java: @Override public boolean shouldRender()
    pub fn should_render_body(__this: &compat::Shared<dyn TabHudWidget>) -> bool {
        return (PlayerListManager::is_player_list_loaded() && !(compat::read(__this, |__t| __t.tab_hud_widget_base().hide)) && <dyn HudWidget>::should_render_body(&compat::view(__this, |a| a as compat::Shared<dyn HudWidget>)));
    }

    /// TabHudWidget.shouldRender as the dispatch of HudWidget.shouldRender for the concrete class `__C`
    pub fn should_render_from<__C: TabHudWidget + 'static>(__this: &compat::Shared<dyn HudWidget>) -> bool {
        <dyn TabHudWidget>::should_render_body(&compat::view_as::<__C, _, _>(__this, |a| a as compat::Shared<dyn TabHudWidget>))
    }

    /// java: public String getHypixelWidgetName()
    pub fn get_hypixel_widget_name_body(__this: &compat::Shared<dyn TabHudWidget>) -> String {
        return compat::read(__this, |__t| __t.tab_hud_widget_base().hypixel_widget_name.clone());
    }

    /// java: @Override public void updateContent()
    pub fn update_content_body(__this: &compat::Shared<dyn TabHudWidget>) {
        let mut widget: Option<Widget> = PlayerListManager::get_list_widget(&compat::read(__this, |__t| __t.tab_hud_widget_base().hypixel_widget_name.clone()));
        if let Some(widget) = &widget {
            { let __v = false; compat::write(__this, |__t| __t.tab_hud_widget_base_mut().hide = __v); }
            ({ let __f = compat::dispatch(__this, |__t| __t.update_content_1()); __f(__this, widget) });
        } else {
            { let __v = compat::read(__this, |__t| __t.tab_hud_widget_base().hide_if_hypixel_widget_missing); compat::write(__this, |__t| __t.tab_hud_widget_base_mut().hide = __v); }
            if !(compat::read(__this, |__t| __t.tab_hud_widget_base().hide)) {
                ({ let __f = compat::dispatch(__this, |__t| __t.update_content_missing()); __f(__this) });
            }
        }
    }

    /// TabHudWidget.updateContent as the dispatch of ElementBasedWidget.updateContent for the concrete class `__C`
    pub fn update_content_from<__C: TabHudWidget + 'static>(__this: &compat::Shared<dyn ElementBasedWidget>) {
        <dyn TabHudWidget>::update_content_body(&compat::view_as::<__C, _, _>(__this, |a| a as compat::Shared<dyn TabHudWidget>))
    }

    /// java: protected void updateContentMissing()
    pub fn update_content_missing_body(__this: &compat::Shared<dyn TabHudWidget>) {
        for component in ({ let __f = compat::dispatch(__this, |__t| __t.create_error_message()); __f(__this) }) {
            { let (__a0,) = ((PlainTextElement::new(Some(component.clone())) as compat::Shared<dyn Element>),); __this.lock().borrow_mut().add_element(__a0) };
        }
    }

    /// java: @Override protected final void updateConfigContent(ElementCollector collector)
    pub fn update_config_content_body(__this: &compat::Shared<dyn TabHudWidget>, collector: &compat::Shared<dyn ElementCollector>) {
        let mut widget: Option<Widget> = PlayerListManager::get_list_widget(&compat::read(__this, |__t| __t.tab_hud_widget_base().hypixel_widget_name.clone()));
        if let Some(widget) = &widget {
            <dyn ElementBasedWidget>::update_config_content_body(&compat::view(__this, |a| a as compat::Shared<dyn ElementBasedWidget>), collector);
        } else {
            ({ let __f = compat::dispatch(__this, |__t| __t.update_config_content_tab()); __f(__this, collector) });
        }
    }

    /// TabHudWidget.updateConfigContent as the dispatch of ElementBasedWidget.updateConfigContent for the concrete class `__C`
    pub fn update_config_content_from<__C: TabHudWidget + 'static>(__this: &compat::Shared<dyn ElementBasedWidget>, __p0: &compat::Shared<dyn ElementCollector>) {
        <dyn TabHudWidget>::update_config_content_body(&compat::view_as::<__C, _, _>(__this, |a| a as compat::Shared<dyn TabHudWidget>), __p0)
    }

    /// java: protected void updateConfigContentTab(ElementCollector collector)
    pub fn update_config_content_tab_body(__this: &compat::Shared<dyn TabHudWidget>, collector: &compat::Shared<dyn ElementCollector>) {
        { let (__a0,) = ((PlainTextElement::new((({ let __f = compat::dispatch(__this, |__t| __t.get_information()); __f(&compat::view(__this, |a| a as compat::Shared<dyn HudWidget>)) })).display_name()) as compat::Shared<dyn Element>),); collector.lock().borrow_mut().add_element(__a0) };
    }

    /// java: @Override public void updateConfigPreview()
    pub fn update_config_preview_body(__this: &compat::Shared<dyn TabHudWidget>) {
        PlayerListManager::update_list_now();
        <dyn ElementBasedWidget>::update_config_preview_body(&compat::view(__this, |a| a as compat::Shared<dyn ElementBasedWidget>));
    }

    /// TabHudWidget.updateConfigPreview as the dispatch of HudWidget.updateConfigPreview for the concrete class `__C`
    pub fn update_config_preview_from<__C: TabHudWidget + 'static>(__this: &compat::Shared<dyn HudWidget>) {
        <dyn TabHudWidget>::update_config_preview_body(&compat::view_as::<__C, _, _>(__this, |a| a as compat::Shared<dyn TabHudWidget>))
    }

    /// java: @Override public void getOptionWidgets(OptionWidgetCollector collector)
    pub fn get_option_widgets_body(__this: &compat::Shared<dyn TabHudWidget>, collector: &mut OptionWidgetCollector) {
        <dyn ElementBasedWidget>::get_option_widgets_body(&compat::view(__this, |a| a as compat::Shared<dyn ElementBasedWidget>), collector);
        let __this_w1 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        collector.yes_no_button_3(&component::translatable("skyblocker.config.hud.widget.tab.hideIfMissing"), Arc::new(move |b| { let Some(__this_1) = __this_w1.get().upgrade() else { return Default::default(); }; { { let __v = b; compat::write((&__this_1), |__t| __t.tab_hud_widget_base_mut().hide_if_hypixel_widget_missing = __v); } } }), compat::read(__this, |__t| __t.tab_hud_widget_base().hide_if_hypixel_widget_missing));
    }

    /// TabHudWidget.getOptionWidgets as the dispatch of HudWidget.getOptionWidgets for the concrete class `__C`
    pub fn get_option_widgets_from<__C: TabHudWidget + 'static>(__this: &compat::Shared<dyn HudWidget>, __p0: &mut OptionWidgetCollector) {
        <dyn TabHudWidget>::get_option_widgets_body(&compat::view_as::<__C, _, _>(__this, |a| a as compat::Shared<dyn TabHudWidget>), __p0)
    }

    /// java: @Override public void load(JsonValueInput input)
    pub fn load_body(__this: &compat::Shared<dyn TabHudWidget>, input: &JsonValueInput) {
        <dyn ElementBasedWidget>::load_body(&compat::view(__this, |a| a as compat::Shared<dyn ElementBasedWidget>), input);
        { let __v = input.read_boolean_or("hide_if_missing", compat::read(__this, |__t| __t.tab_hud_widget_base().hide_if_missing_by_default)); compat::write(__this, |__t| __t.tab_hud_widget_base_mut().hide_if_hypixel_widget_missing = __v); }
    }

    /// TabHudWidget.load as the dispatch of HudWidget.load for the concrete class `__C`
    pub fn load_from<__C: TabHudWidget + 'static>(__this: &compat::Shared<dyn HudWidget>, __p0: &JsonValueInput) {
        <dyn TabHudWidget>::load_body(&compat::view_as::<__C, _, _>(__this, |a| a as compat::Shared<dyn TabHudWidget>), __p0)
    }

    /// java: @Override public void save(JsonValueOutput output)
    pub fn save_body(__this: &compat::Shared<dyn TabHudWidget>, output: &mut JsonValueOutput) {
        <dyn ElementBasedWidget>::save_body(&compat::view(__this, |a| a as compat::Shared<dyn ElementBasedWidget>), output);
        output.write_bool("hide_if_missing", compat::read(__this, |__t| __t.tab_hud_widget_base().hide_if_hypixel_widget_missing));
    }

    /// TabHudWidget.save as the dispatch of HudWidget.save for the concrete class `__C`
    pub fn save_from<__C: TabHudWidget + 'static>(__this: &compat::Shared<dyn HudWidget>, __p0: &mut JsonValueOutput) {
        <dyn TabHudWidget>::save_body(&compat::view_as::<__C, _, _>(__this, |a| a as compat::Shared<dyn TabHudWidget>), __p0)
    }

    /// java: public List<Component> createErrorMessage()
    pub fn create_error_message_body(__this: &compat::Shared<dyn TabHudWidget>) -> Vec<MutableComponent> {
        // TODO(fabric2seer): `Options.keyPlayerList` has no Rust field in the port (searched net.minecraft.client.Options and its supertypes)
        // TODO(fabric2seer): `get_translated_key_message` takes 2 argument(s) in the port (`name: &str, key_name: &dyn Fn(i32, i32) -> Option<String>`), Java passes 0
        // rust: let mut tab_key: MutableComponent = compat::component_of(key_mapping::get_translated_key_message()).copy().with_style_format(ChatFormatting::Yellow);
        let mut tab_key: MutableComponent = todo!("fabric2seer: the initialiser of `tabKey` (TODO above)");
        // TODO(fabric2seer): `get_translated_key_message` takes 2 argument(s) in the port (`name: &str, key_name: &dyn Fn(i32, i32) -> Option<String>`), Java passes 0
        // rust: return vec![component::translatable_args("skyblocker.hud.missingTabWidget[0]", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from((component::literal(&compat::read(__this, |__t| __t.tab_hud_widget_base().hypixel_widget_name.clone()))).with_style_format(ChatFormatting::Yellow))]), component::translatable("skyblocker.hud.missingTabWidget[1]"), component::translatable("skyblocker.hud.missingTabWidget[2]"), component::translatable("skyblocker.hud.missingTabWidget[3]"), component::translatable_args("skyblocker.hud.missingTabWidget[4]", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(tab_key.clone()), mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(tab_key.clone()), mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(compat::component_of(key_mapping::get_translated_key_message()).copy().with_style_format(ChatFormatting::Yellow))])];
        return todo!("fabric2seer: see the TODO above");
    }

}

// TabHudWidget.updateContent implements ElementBasedWidget.updateContent: it is emitted in each concrete subclass's `impl ElementBasedWidget` (Rust has no inherited impls)
// TabHudWidget.updateConfigContent implements ElementBasedWidget.updateConfigContent: it is emitted in each concrete subclass's `impl ElementBasedWidget` (Rust has no inherited impls)
// TabHudWidget.shouldRender implements HudWidget.shouldRender: it is emitted in each concrete subclass's `impl HudWidget` (Rust has no inherited impls)
// TabHudWidget.updateConfigPreview implements HudWidget.updateConfigPreview: it is emitted in each concrete subclass's `impl HudWidget` (Rust has no inherited impls)
// TabHudWidget.getOptionWidgets implements HudWidget.getOptionWidgets: it is emitted in each concrete subclass's `impl HudWidget` (Rust has no inherited impls)
// TabHudWidget.load implements HudWidget.load: it is emitted in each concrete subclass's `impl HudWidget` (Rust has no inherited impls)
// TabHudWidget.save implements HudWidget.save: it is emitted in each concrete subclass's `impl HudWidget` (Rust has no inherited impls)
