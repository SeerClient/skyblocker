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
use mcrust::net::minecraft::client::gui::components::scrollable_layout::ScrollableLayoutElement;
use mcrust::net::minecraft::client::gui::components::scrollable_layout::ReserveStrategy;
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
    /// (the port's `ScrollableLayout` layout is `ScrollableLayoutElement`, and `layout` holds it by value: this is the
    /// value it was built with, the live one is `layout`'s child - see `scrollable_bounds`. Its scroll container is not a
    /// widget of a mod screen, so the content does not scroll - GAPS_O)
    pub scrollable: Option<ScrollableLayoutElement>,
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
        //   java: super(Component.literal("Copy to other locations"), backgroundScreen);
        // (open it with `game::set_screen_keeping_parent`: the screen it is opened over is its background screen, which
        // the base keeps a stand-in for - see `AbstractPopupScreenBase::new_in`)
        let (_, __this) = AbstractPopupScreenBase::new_in(component::literal("Copy to other locations"), background_screen, move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                layout: LinearLayout::vertical().spacing(2),
                selected_locations: __init_selected_locations,
                available_locations: __init_available_locations,
                copied_widget: __init_copied_widget,
                location: __init_location,
                layer: __init_layer,
                copy_position: true,
                scrollable: None,
            });
            ((__h.clone() as compat::Shared<dyn AbstractPopupScreen>), __h)
        });
        compat::write(&__this, |__t| { let __s = __t.layout.default_cell_setting(); *__s = (*__s).align_horizontally_center(); });
        __this
    }

    /// `scrollable`'s x, y, width and height: the live element is the one `layout` holds (it was added by value)
    fn scrollable_bounds(__this: &compat::Shared<Self>) -> Option<(i32, i32, i32, i32)> {
        let mut bounds = None;
        compat::read(__this, |__t| __t.layout.wrapped.visit_children(&mut |__child| {
            if let LayoutElement::Scrollable(__s) = __child {
                bounds = Some((__s.get_x(), __s.get_y(), __s.get_width(), __s.get_height()));
            }
        }));
        bounds
    }

    /// `Checkbox.builder(message, font).maxWidth(maxWidth).selected(selected).onValueChange(onValueChange).build()` whose
    /// value-change listener is handed the screen the press runs on (`compat::checkbox`'s body, for a listener that
    /// writes the screen's other checkboxes)
    fn checkbox_on_screen(message: mcrust::net::minecraft::network::chat::formatted_text::FormattedText, max_width: Option<i32>, selected: bool, on_value_change: impl Fn(&mut mcrust::net::minecraft::client::gui::screens::screen::Screen, bool) + 'static) -> AbstractWidget {
        use mcrust::net::minecraft::client::gui::components::abstract_widget::WidgetKind;
        let font = compat::font();
        let name = message.get_string();
        let checkbox = Checkbox::build(&font.splitter(), 0, 0, max_width, message, selected, true);
        let (width, height) = (checkbox.width, checkbox.height);
        let on_value_change = compat::RenderThread::new(on_value_change);
        AbstractWidget::new(0, 0, width, height, name, WidgetKind::Checkbox(checkbox), OnPress::Lambda(PressLambda::new(move |screen, index| {
            let checked = match &mut screen.widget_mut(index).kind {
                WidgetKind::Checkbox(c) => c.on_press(),
                _ => return,
            };
            (on_value_change.get())(screen, checked);
        })))
    }

    /// java: private void apply()
    pub fn apply(__this: &compat::Shared<Self>) {
        let mut widget_config: serde_json::Map<String, serde_json::Value> = serde_json::Map::new();
        {
            //   java: try (ProblemReporter.ScopedCollector reporter = new ProblemReporter.ScopedCollector(WidgetManager.LOGGER)) {
            //   java:     copiedWidget.widget.save(new JsonValueOutput(reporter, widgetConfig)); }
            // (the port's JsonValueOutput owns the object it writes; `widgetConfig` is what it wrote)
            let reporter: ScopedCollector = ScopedCollector::new("de.hysky.skyblocker.skyblock.tabhud.screenbuilder.WidgetManager");
            let mut output = JsonValueOutput::new(reporter.reporter(), widget_config.clone());
            ({ let __r = &compat::read(&compat::read(__this, |__t| __t.copied_widget.clone()), |__t| __t.widget.clone()); let __f = compat::dispatch(__r, |__t| __t.save()); __f(__r, &mut output) });
            widget_config = output.output.unwrap_or_default();
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
        //   java: layout.addChild(Checkbox.builder(Component.translatable("skyblocker.config.hud.copy.copyPosition"), font)
        //   java: 				.selected(selectedLocations.isEmpty()) // automatically select if it's empty
        //   java: 				.onValueChange((_, value) -> copyPosition = value).build()
        //   java: 		).setTooltip(Tooltip.create(Component.translatable("skyblocker.config.hud.copy.copyPosition.@Tooltip")));
        {
            let __this_w0 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
            let mut __w = compat::checkbox(mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::translatable("skyblocker.config.hud.copy.copyPosition")), 0, 0, None, compat::read(__this, |__t| __t.selected_locations.is_empty()), move |value: bool| { if let Some(__this_0) = __this_w0.get().upgrade() { compat::write(&__this_0, |__t| __t.copy_position = value); } });
            __w.set_tooltip(mcrust::net::minecraft::client::gui::components::tooltip::Tooltip::create(component::translatable("skyblocker.config.hud.copy.copyPosition.@Tooltip").get_string()));
            let (__ww, __wh) = (__w.get_width(), __w.get_height());
            let __e = LayoutElement::widget(compat::write(__this, |__t| __t.add_widget(__w)), __ww, __wh);
            compat::write(__this, |__t| __t.layout.add_child(__e));
        }
        { let (__q0, __q1,) = ({ let __w = compat::string_widget(0, 0, -1, 9, component::translatable("skyblocker.config.hud.copy.targetLocations").with_style_format(ChatFormatting::Bold)); let (__ww, __wh) = (__w.get_width(), __w.get_height()); LayoutElement::widget(compat::write(__this, |__t| __t.add_widget(__w)), __ww, __wh) }, &mut |settings: &mut mcrust::net::minecraft::client::gui::layouts::layout_settings::LayoutSettings| { *settings = settings.clone().padding_vertical(4); },); __this.lock().borrow_mut().layout.add_child_adjusted(__q0, __q1) };
        let mut content: LinearLayout = LinearLayout::vertical().spacing(2);
        // java: List<Checkbox> checkboxes = new ArrayList<>(); - the checkboxes are this screen's widgets, by index; the
        // value-change lambdas reach them through the screen the press runs on
        let checkboxes: Arc<Mutex<Vec<usize>>> = Arc::new(Mutex::new(Vec::new()));
        let select_all_index: Arc<std::sync::atomic::AtomicUsize> = Arc::new(std::sync::atomic::AtomicUsize::new(usize::MAX));
        //   java: Checkbox selectAll = content.addChild(Checkbox.builder(Component.translatable("skyblocker.config.hud.copy.selectAll").withStyle(ChatFormatting.BOLD), font)
        //   java:     .maxWidth(200).selected(selectedLocations.containsAll(availableLocations)).onValueChange((_, value) -> {..}).build());
        {
            let __this_w1 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
            let __checkboxes = checkboxes.clone();
            let __selected = compat::read(__this, |__t| __t.available_locations.iter().all(|x| __t.selected_locations.contains(x)));
            let __w = CopyToPopup::checkbox_on_screen(mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::translatable("skyblocker.config.hud.copy.selectAll").with_style_format(ChatFormatting::Bold)), Some(200), __selected, move |__screen, value| {
                let Some(__this_1) = __this_w1.get().upgrade() else { return; };
                if value {
                    compat::write(&__this_1, |__t| { __t.selected_locations.clear(); let __all = __t.available_locations.clone(); __t.selected_locations.extend(__all); });
                } else {
                    compat::write(&__this_1, |__t| __t.selected_locations.clear());
                }
                // java: checkboxes.forEach(checkbox -> ((CheckboxAccessor) checkbox).setSelected(value));
                for __i in compat::lock(&__checkboxes).iter().copied() {
                    compat::set_checkbox_selected(__screen.widget_mut(__i), value);
                }
            });
            let (__ww, __wh) = (__w.get_width(), __w.get_height());
            let __i = compat::write(__this, |__t| __t.add_widget(__w));
            select_all_index.store(__i, std::sync::atomic::Ordering::Relaxed);
            content.add_child(LayoutElement::widget(__i, __ww, __wh));
        }
        for location in compat::read(__this, |__t| __t.available_locations.clone()) {
            if location == compat::read(__this, |__t| __t.location) {
                continue;
            }
            //   java: checkboxes.add(content.addChild(Checkbox.builder(Component.literal(location.toString()), font).maxWidth(200)
            //   java:     .selected(selectedLocations.contains(location)).onValueChange((_, value) -> {..}).build()));
            let __this_w2 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
            let __checkboxes = checkboxes.clone();
            let __select_all = select_all_index.clone();
            let __selected = compat::read(__this, |__t| __t.selected_locations.contains(&location));
            let __w = CopyToPopup::checkbox_on_screen(mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::literal(&location.to_string())), Some(200), __selected, move |__screen, value| {
                let Some(__this_2) = __this_w2.get().upgrade() else { return; };
                if value {
                    compat::write(&__this_2, |__t| { __t.selected_locations.insert(location); });
                } else {
                    compat::write(&__this_2, |__t| { __t.selected_locations.remove(&location); });
                }
                // java: ((CheckboxAccessor) selectAll).setSelected(checkboxes.stream().allMatch(Checkbox::selected));
                let __all = compat::lock(&__checkboxes).iter().all(|__i| compat::checkbox_selected(__screen.widget_mut(*__i)));
                compat::set_checkbox_selected(__screen.widget_mut(__select_all.load(std::sync::atomic::Ordering::Relaxed)), __all);
            });
            let (__ww, __wh) = (__w.get_width(), __w.get_height());
            let __i = compat::write(__this, |__t| __t.add_widget(__w));
            compat::lock(&checkboxes).push(__i);
            content.add_child(LayoutElement::widget(__i, __ww, __wh));
        }
        { content.arrange_elements(); let mut __vw = Vec::new(); LayoutElement::Grid(content.wrapped.clone()).visit_widgets(&mut __vw); for (__i, __x, __y) in __vw { compat::write(__this, |__t| __t.widget_mut(__i).set_position(__x, __y)); } };
        let mut max_height: i32 = 200i32.min((compat::read(__this, |__t| __t.height)).wrapping_sub(150));
        // java: this.scrollable = new ScrollableLayout(minecraft, content, maxHeight); - the three-argument constructor's strategy is BOTH
        let mut scrollable = ScrollableLayoutElement::new(LayoutElement::Grid(content.wrapped.clone()), max_height, ReserveStrategy::Both);
        // java: scrollable.setMaxHeight(maxHeight);
        scrollable.set_max_height(max_height);
        // java: scrollable.setMinWidth(150);
        scrollable.set_min_width(150);
        { let __v = Some(scrollable.clone()); compat::write(__this, |__t| __t.scrollable = __v); }
        // java: layout.addChild(scrollable);
        compat::write(__this, |__t| __t.layout.add_child(LayoutElement::Scrollable(Box::new(scrollable))));
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

    /// CopyToPopup.repositionElements as the dispatch of AbstractPopupScreen.repositionElements
    pub fn reposition_elements_dyn(__this: &compat::Shared<dyn AbstractPopupScreen>) {
        Self::reposition_elements_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public void extractBackground(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_background_impl(__this: &compat::Shared<Self>, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        <dyn AbstractPopupScreen>::extract_background_body(&compat::view(__this, |a| a as compat::Shared<dyn AbstractPopupScreen>), graphics, mouse_x, mouse_y, a);
        AbstractPopupScreenBase::extract_popup_background(graphics, __this.lock().borrow().layout.get_x(), __this.lock().borrow().layout.get_y(), __this.lock().borrow().layout.get_width(), __this.lock().borrow().layout.get_height());
        // java: graphics.fill(scrollable.getX(), scrollable.getY(), scrollable.getX() + scrollable.getWidth(), scrollable.getY() + scrollable.getHeight(), ARGB.black(0.1f));
        if let Some((x, y, width, height)) = CopyToPopup::scrollable_bounds(__this) {
            graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), x, y, x.wrapping_add(width), y.wrapping_add(height), argb::black_f32(0.1f32));
        }
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
    /// java: AbstractPopupScreen.extractBackground overrides `Screen.extractBackground` without `super`
    fn extract_background_calls_super(&self) -> bool {
        false
    }

    /// java: @Override public void extractBackground(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_background(&mut self, _screen: &mcrust::net::minecraft::client::gui::screens::screen::Screen, context: &mut mcrust::mcrust::modding::screens::ScreenRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        self.call(|__h| ({ let __f = compat::dispatch(__h, |__t| __t.extract_background()); __f(&compat::view(__h, |a| a as compat::Shared<dyn AbstractPopupScreen>), &mut context.graphics, mouse_x, mouse_y, a) }))
    }

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
