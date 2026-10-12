//! Ported from de.hysky.skyblocker.skyblock.tabhud.config.CopyToPopup by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::layer_config::LayerConfig;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::positioned_widget::PositionedWidget;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::widget_config::WidgetConfig;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::widget_manager;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::widget_manager::ScreenLayer;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::widget_manager::WidgetManager;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::hud_widget::HudWidget;
use crate::de::hysky::skyblocker::utils::exclusive_grouping_set::ExclusiveGroupingSet;
use crate::de::hysky::skyblocker::utils::json_value_output::JsonValueOutput;
use crate::de::hysky::skyblocker::utils::location::Location;
use crate::de::hysky::skyblocker::utils::render::gui::abstract_popup_screen::AbstractPopupScreen;
use crate::de::hysky::skyblocker::utils::render::gui::abstract_popup_screen::AbstractPopupScreenBase;
use mcrust::mcrust::modding::screens::ModScreen;
use mcrust::mcrust::modding::screens::ScreenRenderContext;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::abstract_widget::PressLambda;
use mcrust::net::minecraft::client::gui::components::button;
use mcrust::net::minecraft::client::gui::components::checkbox::Checkbox;
use mcrust::net::minecraft::client::gui::components::scrollable_layout::ScrollableLayout;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::layouts::layout_element::LayoutElement;
use mcrust::net::minecraft::client::gui::layouts::linear_layout::LinearLayout;
use mcrust::net::minecraft::client::gui::layouts::spacer_element::SpacerElement;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::client::input::mouse_button_info::MouseButtonInfo;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::util::argb;
use mcrust::net::minecraft::util::problem_reporter::ScopedCollector;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: class CopyToPopup extends AbstractPopupScreen
pub struct CopyToPopup {
    /// java: extends AbstractPopupScreen
    pub base: AbstractPopupScreenBase,
    /// java: private final LinearLayout layout = LinearLayout.vertical().spacing(2);
    pub layout: LinearLayout,
    /// java: private final Set<Location> selectedLocations;
    pub selected_locations: std::collections::HashSet<Location>,
    /// java: private final Set<Location> availableLocations;
    pub available_locations: std::collections::HashSet<Location>,
    /// java: private final PositionedWidget copiedWidget;
    pub copied_widget: compat::Shared<PositionedWidget>,
    /// java: private final Location location;
    pub location: Location,
    /// java: private final WidgetManager.ScreenLayer layer;
    pub layer: ScreenLayer,
    /// java: private boolean copyPosition = true;
    pub copy_position: bool,
    /// java: private ScrollableLayout scrollable;
    pub scrollable: Option<ScrollableLayout>,
}

impl std::ops::Deref for CopyToPopup {
    type Target = AbstractPopupScreenBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for CopyToPopup {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl CopyToPopup {
    /// java: CopyToPopup(Screen backgroundScreen, PositionedWidget copiedWidget, Location location, WidgetManager.ScreenLayer layer)
    pub fn new(mut background_screen: mcrust::net::minecraft::client::gui::screens::screen::Screen, mut copied_widget: compat::Shared<PositionedWidget>, mut location: Location, mut layer: ScreenLayer) -> compat::Shared<Self> {
        let __init_copied_widget = copied_widget.clone();
        let __init_selected_locations = WidgetManager::get_copy_tracker().get(layer).get(&({ let __r = &compat::read(&copied_widget, |__t| __t.widget.clone()); let __f = compat::dispatch(__r, |__t| __t.get_internal_id()); __f(__r) })).and_then(|s| ExclusiveGroupingSet::get_group(&s, &location)).unwrap_or_else(|| std::collections::HashSet::<_>::new());
        let __init_available_locations = widget_manager::ALLOWED_LOCATIONS.iter().copied().filter(|l| (({ let __r = &compat::read(&copied_widget, |__t| __t.widget.clone()); let __f = compat::dispatch(__r, |__t| __t.get_information()); __f(__r) }).available())(*l)).collect::<std::collections::HashSet<Location>>();
        let __init_location = location;
        let __init_layer = layer;
        // TODO(fabric2seer): the mod keeps the game's `Screen` (Java holds a reference to the live object); the port's `Screen` is not a value to copy and the port lends the mod no handle to it
        //   java: super(Component.literal("Copy to other locations"), backgroundScreen);
        let __this: compat::Shared<Self> = todo!("constructor");
        __this.lock().borrow_mut().layout.default_cell_setting().align_horizontally_center();
        __this
    }

    /// java: private void apply()
    pub fn apply(__this: &compat::Shared<Self>) {
        let mut widget_config: serde_json::Map<String, serde_json::Value> = serde_json::Map::new();
        {
            // TODO(fabric2seer): `logger` (a logger) used as a value
            //   java: ProblemReporter.ScopedCollector reporter = new ProblemReporter.ScopedCollector(WidgetManager.LOGGER);
            let mut reporter: ScopedCollector = todo!("fabric2seer: the initialiser of `reporter` (TODO above)");
            // TODO(fabric2seer): the mod keeps the game's `ProblemReporter$ScopedCollector` (Java holds a reference to the live object); the port's `ScopedCollector` is not a value to copy and the port lends the mod no handle to it
            // rust: ({ let __r = &compat::read(&compat::read(__this, |__t| __t.copied_widget.clone()), |__t| __t.widget.clone()); let __f = compat::dispatch(__r, |__t| __t.save()); __f(__r, &mut JsonValueOutput::new(reporter.clone(), widget_config.clone())) });
        }
        { let __o = compat::read(__this, |__t| __t.available_locations.clone()); __this.lock().borrow_mut().selected_locations.retain(|e| __o.contains(e)); };
        for loc in compat::read(__this, |__t| __t.selected_locations.clone()) {
            let mut config: LayerConfig = WidgetManager::get_screen_config(loc).get(compat::read(__this, |__t| __t.layer));
            let __id: String = ({ let __r = &compat::read(&compat::read(__this, |__t| __t.copied_widget.clone()), |__t| __t.widget.clone()); let __f = compat::dispatch(__r, |__t| __t.get_internal_id()); __f(__r) });
            if compat::read(__this, |__t| __t.copy_position) {
                // java: config.widgets().put(copiedWidget.widget.getInternalID(), new WidgetConfig(widgetConfig, copiedWidget.rule)); - the config's own map
                let __v = WidgetConfig::new(widget_config.clone(), compat::read(&compat::read(__this, |__t| __t.copied_widget.clone()), |__t| __t.rule.clone()));
                compat::write(&config.widgets(), |__m| { __m.insert(__id, __v); });
            } else {
                // java: config.widgets().computeIfPresent(copiedWidget.widget.getInternalID(), (_, oldConfig) -> new WidgetConfig(Optional.of(widgetConfig), oldConfig.position()));
                compat::write(&config.widgets(), |__m| {
                    if let Some(old_config) = __m.get(&__id).cloned() {
                        __m.insert(__id, WidgetConfig::new_canonical(Some(widget_config.clone()), old_config.position()));
                    }
                });
            }
        }
        let __h0 = compat::read(__this, |__t| __t.location);
        __this.lock().borrow_mut().selected_locations.insert(__h0);
        ExclusiveGroupingSet::group(&(WidgetManager::get_copy_tracker().get(compat::read(__this, |__t| __t.layer))).get_or_create(({ let __r = &compat::read(&compat::read(__this, |__t| __t.copied_widget.clone()), |__t| __t.widget.clone()); let __f = compat::dispatch(__r, |__t| __t.get_internal_id()); __f(__r) })), &compat::read(__this, |__t| __t.selected_locations.clone()));
    }

}

impl AbstractPopupScreen for CopyToPopup {
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
        CopyToPopup::init_dyn
    }

    /// java: @Override protected void repositionElements() (dispatch: the body for this class)
    fn reposition_elements(&self) -> fn(&compat::Shared<dyn AbstractPopupScreen>) {
        CopyToPopup::reposition_elements_dyn
    }

    /// java: @Override public void extractBackground(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a) (dispatch: the body for this class)
    fn extract_background(&self) -> fn(&compat::Shared<dyn AbstractPopupScreen>, &mut GuiGraphicsExtractor, i32, i32, f32) {
        CopyToPopup::extract_background_dyn
    }

}

impl CopyToPopup {
    /// java: @Override protected void init()
    pub fn init_impl(__this: &compat::Shared<Self>) {
        // TODO(fabric2seer): an assignment used as a value
        //   java: layout.addChild(Checkbox.builder(Component.translatable("skyblocker.config.hud.copy.copyPosition"), font)
        //   java: 				.selected(selectedLocations.isEmpty()) // automatically select if it's empty
        //   java: 				.onValueChange((_, value) -> copyPosition = value).build()
        //   java: 		).setTooltip(Tooltip.create(Component.translatable("skyblocker.config.hud.copy.copyPosition.@Tooltip")));
        { let (__q0, __q1,) = ({ let __w = compat::string_widget(0, 0, -1, 9, component::translatable("skyblocker.config.hud.copy.targetLocations").with_style_format(ChatFormatting::Bold)); let (__ww, __wh) = (__w.get_width(), __w.get_height()); LayoutElement::widget(compat::write(__this, |__t| __t.add_widget(__w)), __ww, __wh) }, &mut |settings: &mut mcrust::net::minecraft::client::gui::layouts::layout_settings::LayoutSettings| { *settings = settings.clone().padding_vertical(4); },); __this.lock().borrow_mut().layout.add_child_adjusted(__q0, __q1) };
        let mut content: LinearLayout = LinearLayout::vertical().spacing(2);
        let mut checkboxes: Vec<Checkbox> = Vec::new();
        // TODO(fabric2seer): static `Checkbox.builder` has no Rust counterpart in the port
        // TODO(fabric2seer): `Builder.maxWidth` has no Rust method in the port (looked for `max_width` in Checkbox and its supertypes)
        // TODO(fabric2seer): `Builder.selected` has no Rust method in the port (looked for `selected` in Checkbox and its supertypes)
        // TODO(fabric2seer): `Builder.onValueChange` has no Rust method in the port (looked for `on_value_change` in Checkbox and its supertypes)
        // TODO(fabric2seer): `Builder.build` has no Rust method in the port (looked for `build` in Checkbox and its supertypes)
        // rust: let __this_1 = compat::RenderThread::new(__this.clone());
        // rust: let mut select_all: Checkbox = content.add_child((Checkbox::builder(component::translatable("skyblocker.config.hud.copy.selectAll").with_style_format(ChatFormatting::Bold), compat::font())).max_width(200).selected(compat::read(__this, |__t| __t.available_locations.clone()).into_iter().all(|x| __this.lock().borrow().selected_locations.contains(&x))).on_value_change(&Arc::new(move |_, value| {
        // rust: if value {
        // rust: __this_1.get().lock().borrow_mut().selected_locations.clear();
        // rust: let __h0 = compat::read(__this_1.get(), |__t| __t.available_locations.clone()).into_iter();
        // rust: __this_1.get().lock().borrow_mut().selected_locations.extend(__h0);
        // rust: } else {
        // rust: __this_1.get().lock().borrow_mut().selected_locations.clear();
        // rust: }
        // rust: for checkbox in checkboxes.iter() {
        // rust: checkbox.selected = value;
        // rust: }
        // rust: })).build());
        let mut select_all: Checkbox = todo!("fabric2seer: the initialiser of `selectAll` (TODO above)");
        for location in compat::read(__this, |__t| __t.available_locations.clone()) {
            if location == compat::read(__this, |__t| __t.location) {
                continue;
            }
            // TODO(fabric2seer): static `Checkbox.builder` has no Rust counterpart in the port
            // TODO(fabric2seer): `Builder.maxWidth` has no Rust method in the port (looked for `max_width` in Checkbox and its supertypes)
            // TODO(fabric2seer): `Builder.selected` has no Rust method in the port (looked for `selected` in Checkbox and its supertypes)
            // TODO(fabric2seer): `Builder.onValueChange` has no Rust method in the port (looked for `on_value_change` in Checkbox and its supertypes)
            // TODO(fabric2seer): `Builder.build` has no Rust method in the port (looked for `build` in Checkbox and its supertypes)
            // rust: let __this_2 = compat::RenderThread::new(__this.clone());
            // rust: checkboxes.push(content.add_child((Checkbox::builder(component::literal(&location.to_string()), compat::font())).max_width(200).selected(__this.lock().borrow().selected_locations.contains(&location)).on_value_change(&Arc::new(move |_, value| {
            // rust: if value {
            // rust: __this_2.get().lock().borrow_mut().selected_locations.insert(location);
            // rust: } else {
            // rust: __this_2.get().lock().borrow_mut().selected_locations.remove(&location);
            // rust: }
            // rust: select_all.selected = checkboxes.iter().cloned().all(|__a0| __a0.selected());
            // rust: })).build()));
        }
        { content.arrange_elements(); let mut __vw = Vec::new(); LayoutElement::Grid(content.wrapped.clone()).visit_widgets(&mut __vw); for (__i, __x, __y) in __vw { compat::write(__this, |__t| __t.widget_mut(__i).set_position(__x, __y)); } };
        let mut max_height: i32 = 200i32.min((compat::read(__this, |__t| __t.height)).wrapping_sub(150));
        // TODO(fabric2seer): `new` takes 2 argument(s) in the port (`height: i32, reserve_strategy: ReserveStrategy`), Java passes 3
        // rust: { let __v = Some(ScrollableLayout::new(compat::minecraft(), LayoutElement::Grid(content.clone().wrapped), max_height)); compat::write(__this, |__t| __t.scrollable = __v); }
        // TODO(fabric2seer): `ScrollableLayoutElement::set_max_height` ports `setMaxHeight`, but the port's `ScrollableLayout` holds no `ScrollableLayoutElement` (no Deref, no `base` field) to call it on
        // rust: __this.lock().borrow_mut().scrollable.as_mut().unwrap().set_max_height(max_height);
        // TODO(fabric2seer): `ScrollableLayoutElement::set_min_width` ports `setMinWidth`, but the port's `ScrollableLayout` holds no `ScrollableLayoutElement` (no Deref, no `base` field) to call it on
        // rust: __this.lock().borrow_mut().scrollable.as_mut().unwrap().set_min_width(150);
        // rust: port gap (GAPS_batchC #C6) - Java's `ScrollableLayout` is a layout element holding the content; the port's
        // `ScrollableLayout` is its scroll helper and the element is `ScrollableLayoutElement`, built by the screens
        // that own it (the construction above is the converter's TODO)
        // rust: let __r0 = compat::read(__this, |__t| __t.scrollable.clone());
        // rust: { let (__q0,) = (__r0.unwrap(),); __this.lock().borrow_mut().layout.add_child(__q0) };
        { let (__q0,) = (SpacerElement::height(10).as_layout_element(),); __this.lock().borrow_mut().layout.add_child(__q0) };
        let __this_w3 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        { let (__q0,) = ({ let __w = (button::builder(mcrust::net::minecraft::network::chat::common_components::gui_done().get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { let Some(__this_3) = __this_w3.get().upgrade() else { return Default::default(); }; compat::lend_screen(&__this_3, __screen, || { let mut __pidx = compat::WidgetRef::Screen(__index); {
            CopyToPopup::apply((&__this_3));
            ({ let __f = compat::dispatch((&__this_3), |__t| __t.on_close()); __f(&compat::view((&__this_3), |a| a as compat::Shared<dyn AbstractPopupScreen>)) });
        } }) })))).build(); let (__ww, __wh) = (__w.get_width(), __w.get_height()); LayoutElement::widget(compat::write(__this, |__t| __t.add_widget(__w)), __ww, __wh) },); __this.lock().borrow_mut().layout.add_child(__q0) };
        let __this_w4 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        { let (__q0,) = ({ let __w = (button::builder(mcrust::net::minecraft::network::chat::common_components::gui_cancel().get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { let Some(__this_4) = __this_w4.get().upgrade() else { return Default::default(); }; compat::lend_screen(&__this_4, __screen, || { let mut __pidx = compat::WidgetRef::Screen(__index); { ({ let __f = compat::dispatch((&__this_4), |__t| __t.on_close()); __f(&compat::view((&__this_4), |a| a as compat::Shared<dyn AbstractPopupScreen>)) }); } }) })))).build(); let (__ww, __wh) = (__w.get_width(), __w.get_height()); LayoutElement::widget(compat::write(__this, |__t| __t.add_widget(__w)), __ww, __wh) },); __this.lock().borrow_mut().layout.add_child(__q0) };
        { let mut __vw = Vec::new(); LayoutElement::Grid(__this.lock().borrow().layout.wrapped.clone()).visit_widgets(&mut __vw); for (__i, _, _) in __vw { compat::write(__this, |__t| __t.mark_renderable(__i)); } };
        <dyn AbstractPopupScreen>::init_body(&compat::view(__this, |a| a as compat::Shared<dyn AbstractPopupScreen>));
    }

    /// CopyToPopup.init as the dispatch of AbstractPopupScreen.init
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

    /// CopyToPopup.repositionElements as the dispatch of AbstractPopupScreen.repositionElements
    pub fn reposition_elements_dyn(__this: &compat::Shared<dyn AbstractPopupScreen>) {
        Self::reposition_elements_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public void extractBackground(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_background_impl(__this: &compat::Shared<Self>, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        <dyn AbstractPopupScreen>::extract_background_body(&compat::view(__this, |a| a as compat::Shared<dyn AbstractPopupScreen>), graphics, mouse_x, mouse_y, a);
        AbstractPopupScreenBase::extract_popup_background(graphics, __this.lock().borrow().layout.get_x(), __this.lock().borrow().layout.get_y(), __this.lock().borrow().layout.get_width(), __this.lock().borrow().layout.get_height());
        // TODO(fabric2seer): `ScrollableLayoutElement::get_x` ports `getX`, but the port's `ScrollableLayout` holds no `ScrollableLayoutElement` (no Deref, no `base` field) to call it on
        // TODO(fabric2seer): `ScrollableLayoutElement::get_y` ports `getY`, but the port's `ScrollableLayout` holds no `ScrollableLayoutElement` (no Deref, no `base` field) to call it on
        // TODO(fabric2seer): `ScrollableLayoutElement::get_width` ports `getWidth`, but the port's `ScrollableLayout` holds no `ScrollableLayoutElement` (no Deref, no `base` field) to call it on
        // TODO(fabric2seer): `ScrollableLayoutElement::get_height` ports `getHeight`, but the port's `ScrollableLayout` holds no `ScrollableLayoutElement` (no Deref, no `base` field) to call it on
        // rust: graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), __this.lock().borrow().scrollable.as_ref().unwrap().get_x(), __this.lock().borrow().scrollable.as_ref().unwrap().get_y(), (__this.lock().borrow().scrollable.as_ref().unwrap().get_x()).wrapping_add(__this.lock().borrow().scrollable.as_ref().unwrap().get_width()), (__this.lock().borrow().scrollable.as_ref().unwrap().get_y()).wrapping_add(__this.lock().borrow().scrollable.as_ref().unwrap().get_height()), argb::black_f32(0.1f32));
    }

    /// CopyToPopup.extractBackground as the dispatch of AbstractPopupScreen.extractBackground
    pub fn extract_background_dyn(__this: &compat::Shared<dyn AbstractPopupScreen>, __p0: &mut GuiGraphicsExtractor, __p1: i32, __p2: i32, __p3: f32) {
        Self::extract_background_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2, __p3)
    }

}

/// CopyToPopup's `Screen` half (`super`'s fields and widget tree), lent by the game for each call
impl compat::ScreenHalf for CopyToPopup {
    fn screen_half(&mut self) -> &mut mcrust::net::minecraft::client::gui::screens::screen::Screen {
        &mut self.base.base
    }
    fn screen_parts(&mut self) -> (&mut mcrust::net::minecraft::client::gui::screens::screen::Screen, Option<&mut compat::WidgetCache>) {
        let __s = &mut self.base;
        (&mut __s.base, Some(&mut __s.__widgets))
    }
}

/// CopyToPopup as the game's screen: `game::set_screen(Box::new(compat::ScreenAdapter::new(..)))`
impl ModScreen for compat::ScreenAdapter<CopyToPopup> {
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
