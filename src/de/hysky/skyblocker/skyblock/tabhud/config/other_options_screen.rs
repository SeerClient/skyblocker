//! Ported from de.hysky.skyblocker.skyblock.tabhud.config.OtherOptionsScreen by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::configs::ui_and_visuals_config::TabHudConf;
use crate::de::hysky::skyblocker::config::configs::ui_and_visuals_config::TabHudStyle;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::tabhud::config::widgets_configuration_screen::WidgetsConfigurationScreen;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::screen_builder::ScreenBuilder;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::screen_config::ScreenConfig;
use crate::de::hysky::skyblocker::skyblock::tabhud::util::player_list_manager::PlayerListManager;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::hud_widget::HudWidget;
use crate::de::hysky::skyblocker::utils::formatters;
use crate::de::hysky::skyblocker::utils::render::gui::abstract_popup_screen::AbstractPopupScreen;
use crate::de::hysky::skyblocker::utils::render::gui::abstract_popup_screen::AbstractPopupScreenBase;
use crate::de::hysky::skyblocker::utils::render::gui::ranged_slider_widget::RangedSliderWidget;
use mcrust::mcrust::modding::screens::ModScreen;
use mcrust::mcrust::modding::screens::ScreenRenderContext;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::abstract_widget::PressLambda;
use mcrust::net::minecraft::client::gui::components::button;
use mcrust::net::minecraft::client::gui::components::checkbox::Checkbox;
use mcrust::net::minecraft::client::gui::components::cycle_button;
use mcrust::net::minecraft::client::gui::components::scrollable_layout::ScrollableLayout;
use mcrust::net::minecraft::client::gui::components::tooltip::Tooltip;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::layouts::grid_layout::GridLayout;
use mcrust::net::minecraft::client::gui::layouts::grid_layout::RowHelper;
use mcrust::net::minecraft::client::gui::layouts::header_and_footer_layout::HeaderAndFooterLayout;
use mcrust::net::minecraft::client::gui::layouts::layout_element::LayoutElement;
use mcrust::net::minecraft::client::gui::layouts::linear_layout::LinearLayout;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::client::input::mouse_button_info::MouseButtonInfo;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: public class OtherOptionsScreen extends Screen
pub struct OtherOptionsScreen {
    /// java: extends net.minecraft.client.gui.screens.Screen
    pub base: mcrust::net::minecraft::client::gui::screens::screen::Screen,
    /// java: private final Tooltip STYLE_TOOLTIP = Tooltip.create(Component.translatable("skyblocker.config.uiAndVisuals.tabHud.style.@Tooltip[0]").append("\n") .append(Comp
    pub style_tooltip: Tooltip,
    /// java: private final Component YES = CommonComponents.GUI_YES.copy().withStyle(ChatFormatting.GREEN);
    pub yes: MutableComponent,
    /// java: private final Component NO = CommonComponents.GUI_NO.copy().withStyle(ChatFormatting.RED);
    pub no: MutableComponent,
    /// java: private final HeaderAndFooterLayout layout = new HeaderAndFooterLayout(this);
    pub layout: Option<HeaderAndFooterLayout>,
    /// java: private final WidgetsConfigurationScreen parent;
    pub parent: compat::Shared<WidgetsConfigurationScreen>,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
    /// the screen's widgets as this object last saw them (see `compat::WidgetCache`)
    pub __widgets: compat::WidgetCache,
}

impl std::ops::Deref for OtherOptionsScreen {
    type Target = mcrust::net::minecraft::client::gui::screens::screen::Screen;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for OtherOptionsScreen {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl OtherOptionsScreen {
    /// java: OtherOptionsScreen(WidgetsConfigurationScreen parent)
    pub fn new(mut parent: compat::Shared<WidgetsConfigurationScreen>) -> compat::Shared<Self> {
        let __init_parent = parent.clone();
        let __this = compat::shared(Self {
            base: mcrust::net::minecraft::client::gui::screens::screen::Screen::new(component::translatable("skyblocker.config.hud.otherOptions.title").get_string()),
            self_ref: std::sync::Weak::new(),
            __widgets: Default::default(),
            style_tooltip: Tooltip::create((component::translatable("skyblocker.config.uiAndVisuals.tabHud.style.@Tooltip[0]")).append_str("\n").append(component::translatable("skyblocker.config.uiAndVisuals.tabHud.style.@Tooltip[1]")).append_str("\n").append(component::translatable("skyblocker.config.uiAndVisuals.tabHud.style.@Tooltip[2]")).append_str("\n").append(component::translatable("skyblocker.config.uiAndVisuals.tabHud.style.@Tooltip[3]")).get_string()),
            yes: mcrust::net::minecraft::network::chat::common_components::gui_yes().copy().with_style_format(ChatFormatting::Green),
            no: mcrust::net::minecraft::network::chat::common_components::gui_no().copy().with_style_format(ChatFormatting::Red),
            layout: None,
            parent: __init_parent,
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        { let __v = Some(HeaderAndFooterLayout::new(0, 0)); compat::write(&__this, |__t| __t.layout = __v); }
        __this
    }

    /// java: @Override protected void init()
    pub fn init(__this: &compat::Shared<Self>) {
        let __r0 = compat::read(__this, |__t| __t.title.clone());
        { let (__q0,) = ({ let __w = compat::string_widget(0, 0, -1, 9, __r0); let (__ww, __wh) = (__w.get_width(), __w.get_height()); LayoutElement::widget(compat::write(__this, |__t| __t.add_widget(__w)), __ww, __wh) },); __this.lock().borrow_mut().layout.as_mut().unwrap().add_to_header(__q0) };
        let mut body: LinearLayout = { let (__q0,) = (LayoutElement::Grid(LinearLayout::vertical().spacing(5).wrapped),); __this.lock().borrow_mut().layout.as_mut().unwrap().add_to_contents(__q0) };
        body.default_cell_setting().align_horizontally_center();
        body.add_child({ let __w = compat::string_widget(0, 0, -1, 9, component::translatable("skyblocker.config.hud.otherOptions.globalOptions")); let (__ww, __wh) = (__w.get_width(), __w.get_height()); LayoutElement::widget(compat::write(__this, |__t| __t.add_widget(__w)), __ww, __wh) });
        // TODO(fabric2seer): `.createRowHelper(..)` on a value of unknown type
        //   java: GridLayout.RowHelper globalOptions = body.addChild(new GridLayout().spacing(2)).createRowHelper(2);
        let mut global_options: RowHelper = todo!("fabric2seer: the initialiser of `globalOptions` (TODO above)");
        let __this_w1 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        { let (__q0,) = ({ let __w = (button::builder(mcrust::net::minecraft::network::chat::common_components::gui_done().get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { let Some(__this_1) = __this_w1.get().upgrade() else { return Default::default(); }; compat::lend_screen(&__this_1, __screen, || { let mut __pidx = compat::WidgetRef::Screen(__index); { OtherOptionsScreen::on_close((&__this_1)); } }) })))).build(); let (__ww, __wh) = (__w.get_width(), __w.get_height()); LayoutElement::widget(compat::write(__this, |__t| __t.add_widget(__w)), __ww, __wh) },); __this.lock().borrow_mut().layout.as_mut().unwrap().add_to_footer(__q0) };
        let conf = &SkyblockerConfigManager::get().ui_and_visuals.tab_hud;
        let mut large_width: i32 = ((mcrust::net::minecraft::client::gui::components::button::DEFAULT_WIDTH).wrapping_mul(2)).wrapping_add(2);
        // TODO(fabric2seer): the port's `builder` ports this Java method with other parameters (default_value for Java's value_stringifier, default_value)
        // TODO(fabric2seer): `builder` takes 1 argument(s) in the port (`default_value: String`), Java passes 2
        // TODO(fabric2seer): a lambda for a parameter the port does not type as a closure (`Function<T, net.minecraft.network.chat.Component>`)
        // TODO(fabric2seer): a lambda for a parameter the port does not type as a closure (`net.minecraft.client.OptionInstance.TooltipSupplier<T>`)
        // TODO(fabric2seer): the port's `create` ports this Java method with other parameters (name, generic_value_format, value_change_listener for Java's x, y, width, height, name, value_change_listener)
        // TODO(fabric2seer): `create` takes 3 argument(s) in the port (`name: String, generic_value_format: String, value_change_listener: OnValueChange`), Java passes 6
        // TODO(fabric2seer): a lambda for a parameter the port does not type as a closure (`net.minecraft.client.gui.components.CycleButton.OnValueChange<T>`)
        // TODO(fabric2seer): the port's `add_child_width` ports this Java method with other parameters (grid, child, column_width for Java's widget, column_width)
        // TODO(fabric2seer): `add_child_width` takes 3 argument(s) in the port (`grid: &mut GridLayout, child: LayoutElement, column_width: i32`), Java passes 2
        // rust: let __this_2 = compat::RenderThread::new(__this.clone());
        // rust: let __this_3 = compat::RenderThread::new(__this.clone());
        // rust: global_options.add_child_width(&mut (cycle_button::builder(|style| component::translatable(&compat::jstring(&style)), conf.style)).with_values(TabHudStyle::values()).with_tooltip(|_| compat::read(__this_2.get(), |__t| __t.style_tooltip.clone())).create(0, 0, large_width, mcrust::net::minecraft::client::gui::components::button::DEFAULT_HEIGHT, component::translatable("skyblocker.config.uiAndVisuals.tabHud.style"), |_, value| { OtherOptionsScreen::update_config(__this_3.get(), Arc::new(move |config| { config.style = value.clone(); })); }).unwrap(), 2).unwrap();
        // TODO(fabric2seer): the port's `add_child_width` ports this Java method with other parameters (grid, child, column_width for Java's widget, column_width)
        // TODO(fabric2seer): `add_child_width` takes 3 argument(s) in the port (`grid: &mut GridLayout, child: LayoutElement, column_width: i32`), Java passes 2
        // rust: let __this_4 = compat::RenderThread::new(__this.clone());
        // rust: global_options.add_child_width(&mut ((((((RangedSliderWidget::builder().option_formatter(&component::translatable("skyblocker.config.hud.globalScale"), &Arc::new(move |d| component::literal(&format!("{}%", formatters::INTEGER_NUMBERS.get_mut().format(d.clone())))))).default_value((conf.tab_hud_scale) as f64)).step(((1) as f64) as f64)).min_max(((10) as f64) as f64, ((200) as f64) as f64)).width(large_width)).callback(Arc::new(move |d| OtherOptionsScreen::update_config(__this_4.get(), Arc::new(move |config| { config.tab_hud_scale = ((compat::java_round_f64(d)) as i32); }))))).build(), 2).unwrap();
        // TODO(fabric2seer): the port's `create` ports this Java method with other parameters (name, generic_value_format, value_change_listener for Java's x, y, width, height, name, value_change_listener)
        // TODO(fabric2seer): `create` takes 3 argument(s) in the port (`name: String, generic_value_format: String, value_change_listener: OnValueChange`), Java passes 6
        // TODO(fabric2seer): a lambda for a parameter the port does not type as a closure (`net.minecraft.client.gui.components.CycleButton.OnValueChange<T>`)
        // TODO(fabric2seer): the port's `add_child` ports this Java method with other parameters (grid, child for Java's widget)
        // TODO(fabric2seer): `add_child` takes 2 argument(s) in the port (`grid: &mut GridLayout, child: LayoutElement`), Java passes 1
        // rust: let __this_5 = compat::RenderThread::new(__this.clone());
        // rust: global_options.add_child(&mut { let (__q0, __q1, __q2, __q3, __q4, __q5,) = (0, 0, 150, 20, component::translatable("skyblocker.config.uiAndVisuals.tabHud.displayIcons"), |_, value| { OtherOptionsScreen::update_config(__this_5.get(), Arc::new(move |config| { config.display_icons = value.clone(); })); },); (cycle_button::boolean_builder(__this.lock().borrow().yes.get_string(), __this.lock().borrow().no.get_string(), conf.display_icons)).create(__q0, __q1, __q2, __q3, __q4, __q5) }.unwrap()).unwrap();
        // TODO(fabric2seer): a lambda for a parameter the port does not type as a closure (`net.minecraft.client.OptionInstance.TooltipSupplier<T>`)
        // TODO(fabric2seer): the port's `create` ports this Java method with other parameters (name, generic_value_format, value_change_listener for Java's x, y, width, height, name, value_change_listener)
        // TODO(fabric2seer): `create` takes 3 argument(s) in the port (`name: String, generic_value_format: String, value_change_listener: OnValueChange`), Java passes 6
        // TODO(fabric2seer): a lambda for a parameter the port does not type as a closure (`net.minecraft.client.gui.components.CycleButton.OnValueChange<T>`)
        // TODO(fabric2seer): the port's `add_child` ports this Java method with other parameters (grid, child for Java's widget)
        // TODO(fabric2seer): `add_child` takes 2 argument(s) in the port (`grid: &mut GridLayout, child: LayoutElement`), Java passes 1
        // rust: let __this_6 = compat::RenderThread::new(__this.clone());
        // rust: global_options.add_child(&mut { let (__q0, __q1, __q2, __q3, __q4, __q5,) = (0, 0, 150, 20, component::translatable("skyblocker.config.uiAndVisuals.tabHud.compactWidgets"), |_, value| { OtherOptionsScreen::update_config(__this_6.get(), Arc::new(move |config| { config.compact_widgets = value.clone(); })); },); ({ let (__q0,) = (|_| Tooltip::create((component::translatable("skyblocker.config.uiAndVisuals.tabHud.compactWidgets.@Tooltip")).get_string()),); (cycle_button::boolean_builder(__this.lock().borrow().yes.get_string(), __this.lock().borrow().no.get_string(), conf.compact_widgets)).with_tooltip(__q0) }).create(__q0, __q1, __q2, __q3, __q4, __q5) }.unwrap()).unwrap();
        body.add_child({ let __w = compat::string_widget(0, 0, -1, 9, component::translatable_args("skyblocker.config.hud.otherOptions.locationOptions", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of(compat::jstring(&WidgetsConfigurationScreen::get_current_location(&compat::read(__this, |__t| __t.parent.clone()))))])); let (__ww, __wh) = (__w.get_width(), __w.get_height()); LayoutElement::widget(compat::write(__this, |__t| __t.add_widget(__w)), __ww, __wh) });
        // TODO(fabric2seer): `.createRowHelper(..)` on a value of unknown type
        //   java: GridLayout.RowHelper screenOptions = body.addChild(new GridLayout().spacing(2)).createRowHelper(2);
        let mut screen_options: RowHelper = todo!("fabric2seer: the initialiser of `screenOptions` (TODO above)");
        // TODO(fabric2seer): field `active` of a value of unknown type
        //   java: screenOptions.addChild(Button.builder(Component.translatable("skyblocker.config.hud.otherOptions.hiddenTabWidgets"), _ -> minecraft.gui.setScreen(new HiddenWidgetsPopup(this, parent.getScreenConfig()))).width(largeWidth).build(), 2).active = SkyblockerConfigManager.get().uiAndVisuals.tabHud.tabHudEnabled;
        { let __vw = __this.lock().borrow().layout.as_ref().unwrap().visit_widgets(); for (__i, _, _) in __vw { compat::write(__this, |__t| __t.mark_renderable(__i)); } };
        OtherOptionsScreen::reposition_elements(__this);
    }

    /// java: private void updateConfig(Consumer<UIAndVisualsConfig.TabHudConf> consumer)
    pub fn update_config(__this: &compat::Shared<Self>, consumer: Arc<dyn Fn(TabHudConf)>) {
        SkyblockerConfigManager::update_only(Arc::new(move |config| { (consumer)(config.ui_and_visuals.tab_hud.clone()); }));
    }

    /// java: @Override protected void repositionElements()
    pub fn reposition_elements(__this: &compat::Shared<Self>) {
        let __r0 = compat::read(__this, |__t| (__t.width, __t.height));
        { let (__sw, __sh) = __r0; __this.lock().borrow_mut().layout.as_mut().unwrap().screen_width = __sw; __this.lock().borrow_mut().layout.as_mut().unwrap().screen_height = __sh; __this.lock().borrow_mut().layout.as_mut().unwrap().arrange_elements(); let __vw = __this.lock().borrow().layout.as_ref().unwrap().visit_widgets(); for (__i, __x, __y) in __vw { compat::write(__this, |__t| __t.widget_mut(__i).set_position(__x, __y)); } };
    }

    /// java: @Override public void onClose()
    pub fn on_close(__this: &compat::Shared<Self>) {
        SkyblockerConfigManager::update(Arc::new(move |_| {

        }));
        mcrust::mcrust::modding::game::set_screen(Box::new(compat::ScreenAdapter::new(compat::read(__this, |__t| __t.parent.clone()))));
    }

}

/// OtherOptionsScreen's `Screen` half (`super`'s fields and widget tree), lent by the game for each call
impl compat::ScreenHalf for OtherOptionsScreen {
    fn screen_half(&mut self) -> &mut mcrust::net::minecraft::client::gui::screens::screen::Screen {
        &mut self.base
    }
    fn screen_parts(&mut self) -> (&mut mcrust::net::minecraft::client::gui::screens::screen::Screen, Option<&mut compat::WidgetCache>) {
        let __s = self;
        (&mut __s.base, Some(&mut __s.__widgets))
    }
}

/// OtherOptionsScreen as the game's screen: `game::set_screen(Box::new(compat::ScreenAdapter::new(..)))`
impl ModScreen for compat::ScreenAdapter<OtherOptionsScreen> {
    fn get_title(&self) -> String {
        self.title()
    }

    /// java: @Override protected void init()
    fn init(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        self.lend(screen, |__h| OtherOptionsScreen::init(__h))
    }

    /// java: @Override protected void repositionElements()
    fn reposition_elements(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        self.lend(screen, |__h| OtherOptionsScreen::reposition_elements(__h))
    }

    /// java: Screen.tick (the base body)
    fn tick(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        self.lend(screen, |_| ())
    }

    /// java: Screen.keyPressed (the base body)
    fn key_pressed(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, event: &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        self.lend(screen, |_| ());
        if event.is_escape() && self.should_close_on_esc() {
            self.on_close();
            return true;
        }
        false
    }

    /// java: Screen.mouseClicked (the base body)
    fn mouse_clicked(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, x: f64, y: f64, button: i32) -> bool {
        self.lend(screen, |_| false)
    }

    /// java: @Override public void onClose()
    fn on_close(&mut self) {
        self.call(|__h| OtherOptionsScreen::on_close(__h))
    }

}

// NOTE(fabric2seer): init, onClose, repositionElements override(s) of Screen - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them

/// java: private static class HiddenWidgetsPopup extends AbstractPopupScreen
pub struct HiddenWidgetsPopup {
    /// java: extends AbstractPopupScreen
    pub base: AbstractPopupScreenBase,
    /// java: private final ScreenConfig screenConfig;
    pub screen_config: ScreenConfig,
    /// java: private final LinearLayout layout = LinearLayout.vertical().spacing(10);
    pub layout: LinearLayout,
}

impl std::ops::Deref for HiddenWidgetsPopup {
    type Target = AbstractPopupScreenBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for HiddenWidgetsPopup {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl HiddenWidgetsPopup {
    /// java: private HiddenWidgetsPopup(Screen backgroundScreen, ScreenConfig screenConfig)
    pub fn new(mut background_screen: mcrust::net::minecraft::client::gui::screens::screen::Screen, mut screen_config: ScreenConfig) -> compat::Shared<Self> {
        let __init_screen_config = screen_config.clone();
        // TODO(fabric2seer): the mod keeps the game's `Screen` (Java holds a reference to the live object); the port's `Screen` is not a value to copy and the port lends the mod no handle to it
        //   java: super(Component.literal("Edit hidden Fancy TAB widgets"), backgroundScreen);
        let __this: compat::Shared<Self> = todo!("constructor");
        __this
    }

}

impl AbstractPopupScreen for HiddenWidgetsPopup {
    fn abstract_popup_screen_base(&self) -> &AbstractPopupScreenBase {
        &self.base
    }
    fn abstract_popup_screen_base_mut(&mut self) -> &mut AbstractPopupScreenBase {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override protected void init() (dispatch: the body for this class)
    fn init(&self) -> fn(&compat::Shared<dyn AbstractPopupScreen>) {
        HiddenWidgetsPopup::init_dyn
    }

    /// java: @Override protected void repositionElements() (dispatch: the body for this class)
    fn reposition_elements(&self) -> fn(&compat::Shared<dyn AbstractPopupScreen>) {
        HiddenWidgetsPopup::reposition_elements_dyn
    }

    /// java: @Override public void extractBackground(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a) (dispatch: the body for this class)
    fn extract_background(&self) -> fn(&compat::Shared<dyn AbstractPopupScreen>, &mut GuiGraphicsExtractor, i32, i32, f32) {
        HiddenWidgetsPopup::extract_background_dyn
    }

}

impl HiddenWidgetsPopup {
    /// java: @Override protected void init()
    pub fn init_impl(__this: &compat::Shared<Self>) {
        let mut content: LinearLayout = LinearLayout::vertical().spacing(2);
        for widget_id in PlayerListManager::get_current_widgets() {
            let Some(mut widget) = PlayerListManager::get_tab_widget(&widget_id) else {
                continue;
            };
            // TODO(fabric2seer): static `Checkbox.builder` has no Rust counterpart in the port
            // TODO(fabric2seer): `Builder.maxWidth` has no Rust method in the port (looked for `max_width` in Checkbox and its supertypes)
            // TODO(fabric2seer): `Builder.selected` has no Rust method in the port (looked for `selected` in Checkbox and its supertypes)
            // TODO(fabric2seer): `Builder.onValueChange` has no Rust method in the port (looked for `on_value_change` in Checkbox and its supertypes)
            // TODO(fabric2seer): `Builder.build` has no Rust method in the port (looked for `build` in Checkbox and its supertypes)
            // rust: let __this_1 = compat::RenderThread::new(__this.clone());
            // rust: content.add_child(((Checkbox::builder(({ let __f = compat::dispatch(&widget, |__t| __t.get_information()); __f(&widget) }).display_name(), compat::font())).max_width(200).selected(__this.lock().borrow().screen_config.hidden_tab_widgets().contains(&({ let __f = compat::dispatch(&widget, |__t| __t.get_internal_id()); __f(&widget) }))).on_value_change(&Arc::new(move |_, value| {
            // rust: if value {
            // rust: let __h0 = ({ let __f = compat::dispatch(&widget, |__t| __t.get_internal_id()); __f(&widget) });
            // rust: __this_1.get().lock().borrow().screen_config.hidden_tab_widgets().insert(__h0);
            // rust: } else {
            // rust: let __h0 = &({ let __f = compat::dispatch(&widget, |__t| __t.get_internal_id()); __f(&widget) });
            // rust: __this_1.get().lock().borrow().screen_config.hidden_tab_widgets().remove(__h0);
            // rust: }
            // rust: if let Some(screen) = mcrust::mcrust::modding::game::current_mod_screen::<compat::ScreenAdapter<OtherOptionsScreen>, _>(|__s| __s.object().clone()) {
            // rust: ({ let __r = &compat::read(&compat::read(&screen, |__t| __t.parent.clone()), |__t| __t.screen_builder.clone()); let __f = compat::dispatch(__r, |__t| __t.update_fancy_tab()); __f(&compat::view(__r, |a| a as compat::Shared<dyn ScreenBuilder>)) });
            // rust: }
            // rust: }))).build());
        }
        { content.arrange_elements(); let mut __vw = Vec::new(); LayoutElement::Grid(content.wrapped.clone()).visit_widgets(&mut __vw); for (__i, __x, __y) in __vw { compat::write(__this, |__t| __t.widget_mut(__i).set_position(__x, __y)); } };
        let mut max_height: i32 = 200i32.min((compat::read(__this, |__t| __t.height)).wrapping_sub(150));
        // TODO(fabric2seer): `new` takes 2 argument(s) in the port (`height: i32, reserve_strategy: ReserveStrategy`), Java passes 3
        // rust: let mut scrollable: ScrollableLayout = ScrollableLayout::new(compat::minecraft(), LayoutElement::Grid(content.clone().wrapped), max_height);
        let mut scrollable: ScrollableLayout = todo!("fabric2seer: the initialiser of `scrollable` (TODO above)");
        // TODO(fabric2seer): `ScrollableLayoutElement::set_max_height` ports `setMaxHeight`, but the port's `ScrollableLayout` holds no `ScrollableLayoutElement` (no Deref, no `base` field) to call it on
        // rust: scrollable.set_max_height(max_height);
        // TODO(fabric2seer): `ScrollableLayoutElement::set_min_width` ports `setMinWidth`, but the port's `ScrollableLayout` holds no `ScrollableLayoutElement` (no Deref, no `base` field) to call it on
        // rust: scrollable.set_min_width(150);
        { let (__q0,) = (scrollable.clone(),); __this.lock().borrow_mut().layout.add_child(__q0) };
        let __this_w2 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        { let (__q0,) = ({ let __w = (button::builder(mcrust::net::minecraft::network::chat::common_components::gui_done().get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { let Some(__this_2) = __this_w2.get().upgrade() else { return Default::default(); }; compat::lend_screen(&__this_2, __screen, || { let mut __pidx = compat::WidgetRef::Screen(__index); { ({ let __f = compat::dispatch((&__this_2), |__t| __t.on_close()); __f(&compat::view((&__this_2), |a| a as compat::Shared<dyn AbstractPopupScreen>)) }); } }) })))).build(); let (__ww, __wh) = (__w.get_width(), __w.get_height()); LayoutElement::widget(compat::write(__this, |__t| __t.add_widget(__w)), __ww, __wh) },); __this.lock().borrow_mut().layout.add_child(__q0) };
        { let mut __vw = Vec::new(); LayoutElement::Grid(__this.lock().borrow().layout.wrapped.clone()).visit_widgets(&mut __vw); for (__i, _, _) in __vw { compat::write(__this, |__t| __t.mark_renderable(__i)); } };
        <dyn AbstractPopupScreen>::init_body(&compat::view(__this, |a| a as compat::Shared<dyn AbstractPopupScreen>));
    }

    /// HiddenWidgetsPopup.init as the dispatch of AbstractPopupScreen.init
    pub fn init_dyn(__this: &compat::Shared<dyn AbstractPopupScreen>) {
        Self::init_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override protected void repositionElements()
    pub fn reposition_elements_impl(__this: &compat::Shared<Self>) {
        <dyn AbstractPopupScreen>::reposition_elements_body(&compat::view(__this, |a| a as compat::Shared<dyn AbstractPopupScreen>));
        { __this.lock().borrow_mut().layout.arrange_elements(); let mut __vw = Vec::new(); LayoutElement::Grid(__this.lock().borrow().layout.wrapped.clone()).visit_widgets(&mut __vw); for (__i, __x, __y) in __vw { compat::write(__this, |__t| __t.widget_mut(__i).set_position(__x, __y)); } };
        // TODO(fabric2seer): `LayoutElement::set_position` ports `setPosition`, but the port's `LinearLayout` holds no `LayoutElement` (no Deref, no `base` field) to call it on
        // rust: let __r0 = compat::read(__this, |__t| __t.width);
        // rust: let __r1 = compat::read(__this, |__t| __t.height);
        // rust: { let (__q0, __q1,) = (((__r0).wrapping_sub(__this.lock().borrow().layout.get_width())).wrapping_div(2), ((__r1).wrapping_sub(__this.lock().borrow().layout.get_height())).wrapping_div(2),); __this.lock().borrow_mut().layout.set_position(__q0, __q1) };
    }

    /// HiddenWidgetsPopup.repositionElements as the dispatch of AbstractPopupScreen.repositionElements
    pub fn reposition_elements_dyn(__this: &compat::Shared<dyn AbstractPopupScreen>) {
        Self::reposition_elements_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public void extractBackground(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_background_impl(__this: &compat::Shared<Self>, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        <dyn AbstractPopupScreen>::extract_background_body(&compat::view(__this, |a| a as compat::Shared<dyn AbstractPopupScreen>), graphics, mouse_x, mouse_y, a);
        AbstractPopupScreenBase::extract_popup_background(graphics, __this.lock().borrow().layout.get_x(), __this.lock().borrow().layout.get_y(), __this.lock().borrow().layout.get_width(), __this.lock().borrow().layout.get_height());
    }

    /// HiddenWidgetsPopup.extractBackground as the dispatch of AbstractPopupScreen.extractBackground
    pub fn extract_background_dyn(__this: &compat::Shared<dyn AbstractPopupScreen>, __p0: &mut GuiGraphicsExtractor, __p1: i32, __p2: i32, __p3: f32) {
        Self::extract_background_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2, __p3)
    }

}

/// HiddenWidgetsPopup's `Screen` half (`super`'s fields and widget tree), lent by the game for each call
impl compat::ScreenHalf for HiddenWidgetsPopup {
    fn screen_half(&mut self) -> &mut mcrust::net::minecraft::client::gui::screens::screen::Screen {
        &mut self.base.base
    }
    fn screen_parts(&mut self) -> (&mut mcrust::net::minecraft::client::gui::screens::screen::Screen, Option<&mut compat::WidgetCache>) {
        let __s = &mut self.base;
        (&mut __s.base, Some(&mut __s.__widgets))
    }
}

/// HiddenWidgetsPopup as the game's screen: `game::set_screen(Box::new(compat::ScreenAdapter::new(..)))`
impl ModScreen for compat::ScreenAdapter<HiddenWidgetsPopup> {
    fn get_title(&self) -> String {
        self.title()
    }

    /// java: @Override protected void init()
    fn init(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        self.lend(screen, |__h| ({ let __f = compat::dispatch(__h, |__t| __t.init()); __f(&compat::view(__h, |a| a as compat::Shared<dyn AbstractPopupScreen>)) }))
    }

    /// java: @Override protected void repositionElements()
    fn reposition_elements(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        self.lend(screen, |__h| ({ let __f = compat::dispatch(__h, |__t| __t.reposition_elements()); __f(&compat::view(__h, |a| a as compat::Shared<dyn AbstractPopupScreen>)) }))
    }

    /// java: Screen.tick (the base body)
    fn tick(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        self.lend(screen, |_| ())
    }

    /// java: @Override public void added()
    fn added(&mut self) {
        self.call(|__h| ({ let __f = compat::dispatch(__h, |__t| __t.added()); __f(&compat::view(__h, |a| a as compat::Shared<dyn AbstractPopupScreen>)) }))
    }

    /// java: Screen.keyPressed (the base body)
    fn key_pressed(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, event: &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        self.lend(screen, |_| ());
        if event.is_escape() && self.should_close_on_esc() {
            self.on_close();
            return true;
        }
        false
    }

    /// java: Screen.mouseClicked (the base body)
    fn mouse_clicked(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, x: f64, y: f64, button: i32) -> bool {
        self.lend(screen, |_| false)
    }

    /// java: @Override public void onClose()
    fn on_close(&mut self) {
        self.call(|__h| ({ let __f = compat::dispatch(__h, |__t| __t.on_close()); __f(&compat::view(__h, |a| a as compat::Shared<dyn AbstractPopupScreen>)) }))
    }

}
