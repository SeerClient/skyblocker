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
use mcrust::net::minecraft::client::gui::components::scrollable_layout::ScrollableLayoutElement;
use mcrust::net::minecraft::client::gui::components::scrollable_layout::ReserveStrategy;
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
            // java: CommonComponents.GUI_YES.copy().withStyle(ChatFormatting.GREEN)
            yes: compat::component_of(mcrust::net::minecraft::network::chat::common_components::gui_yes()).with_style_format(ChatFormatting::Green),
            no: compat::component_of(mcrust::net::minecraft::network::chat::common_components::gui_no()).with_style_format(ChatFormatting::Red),
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
        // java: LinearLayout body = layout.addToContents(LinearLayout.vertical().spacing(5)); - the port's layouts own their
        // children, so the body is filled first and joins the contents once complete (the same tree)
        let mut body: LinearLayout = LinearLayout::vertical().spacing(5);
        { let __s = body.default_cell_setting(); *__s = (*__s).align_horizontally_center(); }
        body.add_child({ let __w = compat::string_widget(0, 0, -1, 9, component::translatable("skyblocker.config.hud.otherOptions.globalOptions")); let (__ww, __wh) = (__w.get_width(), __w.get_height()); LayoutElement::widget(compat::write(__this, |__t| __t.add_widget(__w)), __ww, __wh) });
        //   java: GridLayout.RowHelper globalOptions = body.addChild(new GridLayout().spacing(2)).createRowHelper(2);
        // (the port's layouts own their children: the grid is filled first and joins the body once complete, in the
        // same place)
        let mut global_grid: GridLayout = GridLayout::new_default();
        global_grid.spacing(2);
        let mut global_options: RowHelper = global_grid.create_row_helper(2);
        let __this_w1 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        { let (__q0,) = ({ let __w = (button::builder(mcrust::net::minecraft::network::chat::common_components::gui_done().get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { let Some(__this_1) = __this_w1.get().upgrade() else { return Default::default(); }; compat::lend_screen(&__this_1, __screen, || { let mut __pidx = compat::WidgetRef::Screen(__index); { OtherOptionsScreen::on_close((&__this_1)); } }) })))).build(); let (__ww, __wh) = (__w.get_width(), __w.get_height()); LayoutElement::widget(compat::write(__this, |__t| __t.add_widget(__w)), __ww, __wh) },); __this.lock().borrow_mut().layout.as_mut().unwrap().add_to_footer(__q0) };
        let conf = &SkyblockerConfigManager::get().ui_and_visuals.tab_hud;
        let mut large_width: i32 = ((mcrust::net::minecraft::client::gui::components::button::DEFAULT_WIDTH).wrapping_mul(2)).wrapping_add(2);
        // TODO(fabric2seer): the port's `builder` ports this Java method with other parameters (default_value for Java's value_stringifier, default_value)
        // TODO(fabric2seer): `builder` takes 1 argument(s) in the port (`default_value: String`), Java passes 2
        // TODO(fabric2seer): a lambda for a parameter the port does not type as a closure (`Function<T, net.minecraft.network.chat.Component>`)
        // TODO(fabric2seer): a lambda for a parameter the port does not type as a closure (`net.minecraft.client.OptionInstance.TooltipSupplier<T>`)
        // java: globalOptions.addChild(CycleButton.builder(style -> Component.translatable(style.toString()), conf.style)
        //   .withValues(TabHudStyle.values()).withTooltip(_ -> STYLE_TOOLTIP)
        //   .create(0, 0, largeWidth, Button.DEFAULT_HEIGHT, Component.translatable("..tabHud.style"), (_, value) -> updateConfig(config -> config.style = value)), 2);
        // (the port's cycle button holds each value's LABEL and answers the chosen INDEX, which maps back to the value)
        {
            let __values: Vec<TabHudStyle> = TabHudStyle::values();
            let __label = |__style: &TabHudStyle| component::translatable(&__style.to_string()).get_string();
            let __builder = cycle_button::builder(__label(&conf.style))
                .with_values(__values.iter().map(__label).collect())
                .with_tooltip(cycle_button::TooltipSupplier::Constant(mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from((component::translatable("skyblocker.config.uiAndVisuals.tabHud.style.@Tooltip[0]")).append_str("
").append(component::translatable("skyblocker.config.uiAndVisuals.tabHud.style.@Tooltip[1]")).append_str("
").append(component::translatable("skyblocker.config.uiAndVisuals.tabHud.style.@Tooltip[2]")).append_str("
").append(component::translatable("skyblocker.config.uiAndVisuals.tabHud.style.@Tooltip[3]")))/* STYLE_TOOLTIP's text */));
            let __this_w3 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
            let __w = compat::cycle_button_widget(0, 0, large_width, mcrust::net::minecraft::client::gui::components::button::DEFAULT_HEIGHT, &component::translatable("skyblocker.config.uiAndVisuals.tabHud.style"), __builder, move |__index: usize| {
                let Some(__this_3) = __this_w3.get().upgrade() else { return; };
                let value = __values[__index];
                OtherOptionsScreen::update_config(&__this_3, Arc::new(move |config: &mut TabHudConf| { config.style = value; }));
            });
            OtherOptionsScreen::grid_add(__this, &mut global_options, &mut global_grid, __w, 2);
        }
        // java: globalOptions.addChild(RangedSliderWidget.builder()
        //   .optionFormatter(Component.translatable("skyblocker.config.hud.globalScale"), d -> Component.literal(Formatters.INTEGER_NUMBERS.format(d) + '%'))
        //   .defaultValue(conf.tabHudScale).step(1).minMax(10, 200).width(largeWidth)
        //   .callback(d -> updateConfig(config -> config.tabHudScale = (int) Math.round(d))).build(), 2);
        {
            let __this_w4 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
            let __slider: compat::Shared<RangedSliderWidget> = RangedSliderWidget::builder()
                .option_formatter_component_double2_object_function(&component::translatable("skyblocker.config.hud.globalScale"), &(Arc::new(move |d: f64| component::literal(&format!("{}%", formatters::INTEGER_NUMBERS.get_mut().format(d)))) as Arc<dyn Fn(f64) -> MutableComponent>))
                .default_value((conf.tab_hud_scale) as f64)
                .step(1.0)
                .min_max(10.0, 200.0)
                .width(large_width)
                .callback(Arc::new(move |d: f64| {
                    let Some(__this_4) = __this_w4.get().upgrade() else { return; };
                    OtherOptionsScreen::update_config(&__this_4, Arc::new(move |config: &mut TabHudConf| { config.tab_hud_scale = (compat::java_round_f64(d)) as i32; }));
                }))
                .build();
            OtherOptionsScreen::grid_add(__this, &mut global_options, &mut global_grid, RangedSliderWidget::widget(__slider), 2);
        }
        // java: globalOptions.addChild(CycleButton.booleanBuilder(YES, NO, conf.displayIcons)
        //   .create(Component.translatable("..tabHud.displayIcons"), (_, value) -> updateConfig(config -> config.displayIcons = value)));
        // (`create(name, ..)` is CycleButton's 150x20 at (0, 0); index 0 is `true`)
        {
            let (__yes, __no) = compat::read(__this, |__t| (__t.yes.get_string(), __t.no.get_string()));
            let __builder = cycle_button::boolean_builder(__yes, __no, conf.display_icons);
            let __this_w5 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
            let __w = compat::cycle_button_widget(0, 0, 150, 20, &component::translatable("skyblocker.config.uiAndVisuals.tabHud.displayIcons"), __builder, move |__index: usize| {
                let Some(__this_5) = __this_w5.get().upgrade() else { return; };
                let value = __index == 0;
                OtherOptionsScreen::update_config(&__this_5, Arc::new(move |config: &mut TabHudConf| { config.display_icons = value; }));
            });
            OtherOptionsScreen::grid_add(__this, &mut global_options, &mut global_grid, __w, 1);
        }
        // java: globalOptions.addChild(CycleButton.booleanBuilder(YES, NO, conf.compactWidgets)
        //   .withTooltip(_ -> Tooltip.create(Component.translatable("..tabHud.compactWidgets.@Tooltip")))
        //   .create(Component.translatable("..tabHud.compactWidgets"), (_, value) -> updateConfig(config -> config.compactWidgets = value)));
        {
            let (__yes, __no) = compat::read(__this, |__t| (__t.yes.get_string(), __t.no.get_string()));
            let __builder = cycle_button::boolean_builder(__yes, __no, conf.compact_widgets)
                .with_tooltip(cycle_button::TooltipSupplier::Constant(mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::translatable("skyblocker.config.uiAndVisuals.tabHud.compactWidgets.@Tooltip"))));
            let __this_w6 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
            let __w = compat::cycle_button_widget(0, 0, 150, 20, &component::translatable("skyblocker.config.uiAndVisuals.tabHud.compactWidgets"), __builder, move |__index: usize| {
                let Some(__this_6) = __this_w6.get().upgrade() else { return; };
                let value = __index == 0;
                OtherOptionsScreen::update_config(&__this_6, Arc::new(move |config: &mut TabHudConf| { config.compact_widgets = value; }));
            });
            OtherOptionsScreen::grid_add(__this, &mut global_options, &mut global_grid, __w, 1);
        }
        body.add_child(LayoutElement::Grid(global_grid));
        body.add_child({ let __w = compat::string_widget(0, 0, -1, 9, component::translatable_args("skyblocker.config.hud.otherOptions.locationOptions", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of(compat::jstring(&WidgetsConfigurationScreen::get_current_location(&compat::read(__this, |__t| __t.parent.clone()))))])); let (__ww, __wh) = (__w.get_width(), __w.get_height()); LayoutElement::widget(compat::write(__this, |__t| __t.add_widget(__w)), __ww, __wh) });
        //   java: GridLayout.RowHelper screenOptions = body.addChild(new GridLayout().spacing(2)).createRowHelper(2);
        let mut screen_grid: GridLayout = GridLayout::new_default();
        screen_grid.spacing(2);
        let mut screen_options: RowHelper = screen_grid.create_row_helper(2);
        //   java: screenOptions.addChild(Button.builder(Component.translatable("skyblocker.config.hud.otherOptions.hiddenTabWidgets"), _ -> minecraft.gui.setScreen(new HiddenWidgetsPopup(this, parent.getScreenConfig()))).width(largeWidth).build(), 2).active = SkyblockerConfigManager.get().uiAndVisuals.tabHud.tabHudEnabled;
        {
            let __this_w7 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
            let mut __w = button::builder(component::translatable("skyblocker.config.hud.otherOptions.hiddenTabWidgets").get_string(), OnPress::Lambda(PressLambda::new(move |_: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, _: usize| {
                let Some(__this_7) = __this_w7.get().upgrade() else { return; };
                let screen_config = WidgetsConfigurationScreen::get_screen_config(&compat::read(&__this_7, |__t| __t.parent.clone()));
                // (opened over this screen, which the game keeps as its parent; the popup's background screen is that)
                let popup = HiddenWidgetsPopup::new(mcrust::net::minecraft::client::gui::screens::screen::Screen::new(String::new()), screen_config);
                compat::write(&popup, |__t| __t.background_other_options = std::sync::Arc::downgrade(&__this_7));
                mcrust::mcrust::modding::game::set_screen_keeping_parent(Box::new(compat::ScreenAdapter::new(popup)));
            })))
            .width(large_width)
            .build();
            __w.active = SkyblockerConfigManager::get().ui_and_visuals.tab_hud.tab_hud_enabled;
            OtherOptionsScreen::grid_add(__this, &mut screen_options, &mut screen_grid, __w, 2);
        }
        body.add_child(LayoutElement::Grid(screen_grid));
        { let (__q0,) = (LayoutElement::Grid(body.wrapped),); __this.lock().borrow_mut().layout.as_mut().unwrap().add_to_contents(__q0) };
        { let __vw = __this.lock().borrow().layout.as_ref().unwrap().visit_widgets(); for (__i, _, _) in __vw { compat::write(__this, |__t| __t.mark_renderable(__i)); } };
        OtherOptionsScreen::reposition_elements(__this);
    }

    /// `rowHelper.addChild(widget, columnWidth)` on one of `init`'s grids: the widget joins this screen (the port's
    /// layouts hold a widget by its place in the screen)
    fn grid_add(__this: &compat::Shared<Self>, adder: &mut RowHelper, grid: &mut GridLayout, widget: AbstractWidget, column_width: i32) {
        let (w, h) = (widget.get_width(), widget.get_height());
        let index = compat::write(__this, |__t| __t.add_widget(widget));
        adder.add_child_width(grid, LayoutElement::widget(index, w, h), column_width).expect("GridLayout.addChild");
    }

    /// java: private void updateConfig(Consumer<UIAndVisualsConfig.TabHudConf> consumer)
    /// (the consumer writes the config's own object, as Java's does - not a copy of it)
    pub fn update_config(__this: &compat::Shared<Self>, consumer: Arc<dyn Fn(&mut TabHudConf)>) {
        SkyblockerConfigManager::update_only(Arc::new(move |config| { (consumer)(&mut config.ui_and_visuals.tab_hud); }));
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
        // java: minecraft.setScreen(parent); - this screen was opened over its parent (`openPopup`), which comes back as
        // the object it was
        let _ = compat::read(__this, |__t| __t.parent.clone());
        mcrust::mcrust::modding::game::set_screen_to_parent();
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
    /// java: `backgroundScreen` as `init`'s `backgroundScreen instanceof OtherOptionsScreen screen` reads it (the base
    /// keeps only a stand-in for the screen the popup is opened over; the opener records itself here)
    pub background_other_options: compat::WeakShared<OtherOptionsScreen>,
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
        //   java: super(Component.literal("Edit hidden Fancy TAB widgets"), backgroundScreen);
        //   java: this.screenConfig = screenConfig;
        let (_, __this) = AbstractPopupScreenBase::new_in(component::literal("Edit hidden Fancy TAB widgets"), background_screen, move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                screen_config: __init_screen_config,
                layout: LinearLayout::vertical().spacing(10),
                background_other_options: std::sync::Weak::new(),
            });
            ((__h.clone() as compat::Shared<dyn AbstractPopupScreen>), __h)
        });
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
            //   java: content.addChild(Checkbox.builder(widget.getInformation().displayName(), font).maxWidth(200)
            //   java:     .selected(screenConfig.hiddenTabWidgets().contains(widget.getInternalID())).onValueChange((_, value) -> {..}).build());
            let __id: String = ({ let __f = compat::dispatch(&widget, |__t| __t.get_internal_id()); __f(&widget) });
            let __name = ({ let __f = compat::dispatch(&widget, |__t| __t.get_information()); __f(&widget) }).display_name();
            let __hidden = compat::read(__this, |__t| __t.screen_config.hidden_tab_widgets());
            let __selected = compat::read(&__hidden, |__s| __s.contains(&__id));
            let __this_w1 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
            let __w = compat::checkbox(mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(__name.unwrap_or_else(component::empty)), 0, 0, Some(200), __selected, move |value: bool| {
                let Some(__this_1) = __this_w1.get().upgrade() else { return; };
                let __hidden = compat::read(&__this_1, |__t| __t.screen_config.hidden_tab_widgets());
                if value {
                    compat::write(&__hidden, |__s| { __s.insert(__id.clone()); });
                } else {
                    compat::write(&__hidden, |__s| { __s.remove(&__id); });
                }
                // Update the fancy tab after hidden widgets changes
                // java: if (backgroundScreen instanceof OtherOptionsScreen screen) screen.parent.screenBuilder.updateFancyTab();
                if let Some(screen) = compat::read(&__this_1, |__t| __t.background_other_options.upgrade()) {
                    let __r = &compat::read(&compat::read(&screen, |__t| __t.parent.clone()), |__t| __t.screen_builder.clone());
                    let __f = compat::dispatch(__r, |__t| __t.update_fancy_tab());
                    __f(&compat::view(__r, |a| a as compat::Shared<dyn ScreenBuilder>));
                }
            });
            let (__ww, __wh) = (__w.get_width(), __w.get_height());
            content.add_child(LayoutElement::widget(compat::write(__this, |__t| __t.add_widget(__w)), __ww, __wh));
        }
        { content.arrange_elements(); let mut __vw = Vec::new(); LayoutElement::Grid(content.wrapped.clone()).visit_widgets(&mut __vw); for (__i, __x, __y) in __vw { compat::write(__this, |__t| __t.widget_mut(__i).set_position(__x, __y)); } };
        let mut max_height: i32 = 200i32.min((compat::read(__this, |__t| __t.height)).wrapping_sub(150));
        // java: ScrollableLayout scrollable = new ScrollableLayout(minecraft, content, maxHeight); - the three-argument
        // constructor's strategy is BOTH. (Its scroll container is not a widget of a mod screen, so the content does
        // not scroll - GAPS_O)
        let mut scrollable = ScrollableLayoutElement::new(LayoutElement::Grid(content.wrapped.clone()), max_height, ReserveStrategy::Both);
        // java: scrollable.setMaxHeight(maxHeight);
        scrollable.set_max_height(max_height);
        // java: scrollable.setMinWidth(150);
        scrollable.set_min_width(150);
        // java: layout.addChild(scrollable);
        compat::write(__this, |__t| __t.layout.add_child(LayoutElement::Scrollable(Box::new(scrollable))));
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
        // java: layout.setPosition((width - layout.getWidth()) / 2, (height - layout.getHeight()) / 2); - and the widgets
        // follow the layout (the port's layouts hold a widget by its place in the screen)
        compat::write(__this, |__t| {
            let (__width, __height) = (__t.width, __t.height);
            let mut __element = LayoutElement::Grid(__t.layout.wrapped.clone());
            let (__x, __y) = ((__width.wrapping_sub(__element.get_width())).wrapping_div(2), (__height.wrapping_sub(__element.get_height())).wrapping_div(2));
            __element.set_position(__x, __y);
            let mut __vw = Vec::new();
            __element.visit_widgets(&mut __vw);
            if let LayoutElement::Grid(__grid) = __element { __t.layout.wrapped = __grid; }
            for (__i, __wx, __wy) in __vw { __t.widget_mut(__i).set_position(__wx, __wy); }
        });
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
