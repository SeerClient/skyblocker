//! Ported from de.hysky.skyblocker.skyblock.tabhud.config.SidePanelWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::tabhud::config::copy_to_popup::CopyToPopup;
use crate::de::hysky::skyblocker::skyblock::tabhud::config::option_widget_collector::OptionWidgetCollector;
use crate::de::hysky::skyblocker::skyblock::tabhud::config::position_rule_widget::PositionRuleWidget;
use crate::de::hysky::skyblocker::skyblock::tabhud::config::widgets_configuration_screen::WidgetsConfigurationScreen;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::positioned_widget::PositionedWidget;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::widget_manager;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::hud_widget::HudWidget;
use mcrust::mcrust::modding::widgets::WidgetRenderContext;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::components::abstract_container_widget::AbstractContainerWidget;
use mcrust::net::minecraft::client::gui::components::abstract_scroll_area::ScrollArea;
use mcrust::net::minecraft::client::gui::components::abstract_scroll_area::ScrollbarSettings;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::abstract_widget::PressLambda;
use mcrust::net::minecraft::client::gui::components::abstract_widget::WidgetKind;
use mcrust::net::minecraft::client::gui::components::button;
use mcrust::net::minecraft::client::gui::components::string_widget::TextOverflow;
use mcrust::net::minecraft::client::gui::components::tooltip::Tooltip;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::layouts::linear_layout::LinearLayout;
use mcrust::net::minecraft::client::gui::layouts::spacer_element::SpacerElement;
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use mcrust::net::minecraft::client::gui::screens::screen::Screen;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final Identifier TEXTURE = SkyblockerMod.id("menu_outer_space");
pub static TEXTURE: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("menu_outer_space"));
/// java: private static final int TOP_MARGIN = 16;
pub const TOP_MARGIN: i32 = 16;
/// java: private static final int SCROLLBAR_AREA = SCROLLBAR_WIDTH + 1;
pub static SCROLLBAR_AREA: LazyLock<i32> = LazyLock::new(|| mcrust::net::minecraft::client::gui::components::abstract_scroll_area::SCROLLBAR_WIDTH.wrapping_add(1));

/// java: class SidePanelWidget extends AbstractContainerWidget
pub struct SidePanelWidget {
    /// java: extends net.minecraft.client.gui.components.AbstractContainerWidget
    pub base: AbstractContainerWidget,
    /// java: private final List<AbstractWidget> optionWidgets = new ArrayList<>();
    /// (the widget objects, held in the panel's own `Screen` list - the list this port's layouts index, and the one
    /// its widget pass draws and asks)
    pub option_widgets: Screen,
    /// java: private LinearLayout layout = LinearLayout.vertical();
    pub layout: LinearLayout,
    /// java: boolean rightSide = false;
    pub right_side: bool,
    /// java: private int targetX = 0;
    pub target_x: i32,
    /// java: private float animation = 0.0f;
    pub animation: f32,
    /// java: private int animationStart = 0;
    pub animation_start: i32,
    /// java: private int animationEnd = 0;
    pub animation_end: i32,
    /// java: private boolean isOpen = false;
    pub is_open: bool,
    /// java: private final WidgetsConfigurationScreen configScreen;
    pub config_screen: compat::WeakShared<WidgetsConfigurationScreen>,
    /// java: private @Nullable PositionedWidget positionedWidget;
    pub positioned_widget: Option<compat::Shared<PositionedWidget>>,
    /// `AbstractScrollArea`'s state (`AbstractContainerWidget extends AbstractScrollArea`): `scrollAmount`, the settings
    /// it was built with (`defaultSettings(5)`), and the box it scrolls (kept equal to the widget's by `scroll_area_mut`)
    pub scroll_area: ScrollArea,
    /// The anonymous `StringWidget` subclass of `open(..)` (`setWidth(w)` is `setMaxWidth(w, SCROLLING)`): its index
    pub title_index: Option<usize>,
    /// The `PositionRuleWidget` among the option widgets (its `setWidth` / `setX` / `setY` are its own overrides)
    pub position_rule: Option<(usize, compat::Shared<PositionRuleWidget>)>,
    /// Bumped by `open(..)`, which replaces `option_widgets`: a click that ran it does not put the old list back.
    pub generation: u64,
}

impl std::ops::Deref for SidePanelWidget {
    type Target = AbstractContainerWidget;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for SidePanelWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl SidePanelWidget {
    /// java: SidePanelWidget(int width, int height, WidgetsConfigurationScreen configScreen)
    pub fn new(mut width: i32, mut height: i32, mut config_screen: &compat::Shared<WidgetsConfigurationScreen>) -> compat::Shared<Self> {
        // java: super(0, TOP_MARGIN, width, height - TOP_MARGIN, Component.literal("Side Panel"), defaultSettings(5));
        let mut this: Self = Self {
            base: AbstractContainerWidget::new_with_scrollbar_settings(0, TOP_MARGIN, width, height.wrapping_sub(TOP_MARGIN), mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::literal("Side Panel")), ScrollbarSettings::default_settings(5.0)),
            option_widgets: Screen::new(String::new()),
            layout: LinearLayout::vertical(),
            right_side: false,
            target_x: 0,
            animation: 0.0f32,
            animation_start: 0,
            animation_end: 0,
            is_open: false,
            config_screen: std::sync::Arc::downgrade(config_screen),
            positioned_widget: None,
            scroll_area: ScrollArea::new(TOP_MARGIN, height.wrapping_sub(TOP_MARGIN), ScrollbarSettings::default_settings(5.0)),
            title_index: None,
            position_rule: None,
            generation: 0,
        };
        // java: visible = false;
        this.visible = false;
        compat::shared(this)
    }

    fn config_screen(&self) -> compat::Shared<WidgetsConfigurationScreen> {
        self.config_screen.upgrade().expect("the side panel outlived its screen")
    }

    /// The `AbstractScrollArea` half with the widget's current box (`getY()`, `getHeight()`).
    pub fn scroll_area_mut(&mut self) -> &mut ScrollArea {
        self.scroll_area.y = self.base.get_y();
        self.scroll_area.height = self.base.get_height();
        &mut self.scroll_area
    }

    /// `AbstractScrollArea.scrollAmount()`.
    pub fn scroll_amount(&self) -> f64 {
        self.scroll_area.scroll_amount()
    }

    /// java: @Override protected int contentHeight()
    pub fn content_height(&self) -> i32 {
        return self.layout.get_height();
    }

    /// java: private boolean isNotVisible(int top, int bottom)
    pub fn is_not_visible(&self, mut top: i32, mut bottom: i32) -> bool {
        return !(bottom >= self.get_y() && top <= self.get_bottom());
    }

    /// `AbstractWidget.isMouseOver` - `isActive() && areCoordinatesInRectangle(x, y)`
    pub fn is_mouse_over(&self, mouse_x: f64, mouse_y: f64) -> bool {
        self.base.is_mouse_over(mouse_x, mouse_y)
    }

    /// The layout's positions written into the widget list (Java's layout moves the widget objects), and the
    /// `PositionRuleWidget` moved by its own `setX` / `setY` (which move its layout).
    fn place(&mut self) {
        self.layout.place_widgets(&mut self.option_widgets);
        if let Some((index, rule)) = self.position_rule.clone() {
            if index < self.option_widgets.children().len() {
                let (x, y) = { let w = self.option_widgets.widget(index); (w.get_x(), w.get_y()) };
                compat::write(&rule, |__t| {
                    if __t.get_x() != x {
                        __t.set_x(x);
                    }
                    if __t.get_y() != y {
                        __t.set_y(y);
                    }
                });
            }
        }
    }

    /// `layout.arrangeElements()` over the widgets' live sizes, then `place`.
    fn arrange(&mut self) {
        self.layout.sync_widget_sizes(&self.option_widgets);
        self.layout.arrange_elements();
        self.place();
    }

    /// `AbstractWidget.extractRenderState` - `if (visible) extractWidgetRenderState(..)` (hover is the widget pass's)
    pub fn extract_render_state(__this: &compat::Shared<Self>, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, delta_ticks: f32) {
        if !compat::read(__this, |__t| __t.visible) {
            return;
        }
        SidePanelWidget::extract_widget_render_state(__this, context, mouse_x, mouse_y, delta_ticks);
    }

    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor context, int mouseX, int mouseY, float deltaTicks)
    pub fn extract_widget_render_state(__this: &compat::Shared<Self>, context: &mut WidgetRenderContext<'_>, mut mouse_x: i32, mut mouse_y: i32, mut delta_ticks: f32) {
        compat::write(__this, |__t| {
            if __t.animation >= 0.0f32 {
                if __t.animation < 1.0f32 {
                    let __s0 = __t.animation_start.wrapping_add((((__t.animation_end.wrapping_sub(__t.animation_start) as f32) * __t.animation)) as i32);
                    __t.set_x(__s0);
                    // java: animation += Minecraft.getInstance().getDeltaTracker().getRealtimeDeltaTicks() * 50 * 7.5f / 1000.f;
                    let __delta: f32 = compat::minecraft().delta_tracker.get_realtime_delta_ticks();
                    __t.animation = __t.animation + (((__delta * 50.0f32) * 7.5f32) / 1000.0f32);
                } else {
                    let __s0 = __t.animation_end;
                    __t.set_x(__s0);
                    __t.animation = -1.0f32;
                }
            }
        });
        let (x, y, w, h, right, bottom) = compat::read(__this, |__t| (__t.get_x(), __t.get_y(), __t.get_width(), __t.get_height(), __t.get_right(), __t.get_bottom()));
        gui_graphics_extractor::blit_sprite_identifier(context.gl, context.minecraft, mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), TEXTURE.get(), x.wrapping_sub(4), (y.wrapping_sub(4)).wrapping_sub(TOP_MARGIN), w.wrapping_add(8), (h.wrapping_add(8)).wrapping_add(TOP_MARGIN));
        context.graphics.enable_scissor(x, y, right, bottom);
        // java: for (AbstractWidget clickableWidget : this.optionWidgets) { if (isNotVisible(..)) continue; clickableWidget.extractRenderState(..); }
        // (each widget is clipped to the panel - the scissor above, which this port's widget pass reads off the widget -
        // and one wholly outside it is skipped by that pass, as `isNotVisible` skips it)
        let mut __widgets = compat::write(__this, |__t| std::mem::replace(&mut __t.option_widgets, Screen::new(String::new())));
        let __generation = compat::read(__this, |__t| __t.generation);
        let __not_visible: Vec<usize> = (0..__widgets.children().len()).filter(|&i| { let c = __widgets.widget(i); compat::read(__this, |__t| __t.is_not_visible(c.get_y(), c.get_bottom())) }).collect();
        let mut __hidden: Vec<usize> = Vec::new();
        for i in 0..__widgets.children().len() {
            let c = __widgets.widget_mut(i);
            c.clip = Some((x, y, right, bottom));
            if __not_visible.contains(&i) && c.visible {
                c.visible = false;
                __hidden.push(i);
            }
        }
        context.extract_widgets(&mut __widgets, mouse_x, mouse_y);
        for i in __hidden {
            __widgets.widget_mut(i).visible = true;
        }
        compat::write(__this, |__t| if __t.generation == __generation { __t.option_widgets = __widgets; });
        context.graphics.disable_scissor();
        // java: this.extractScrollbar(context, mouseX, mouseY);
        let (area, content_height, scroll_bar_x) = compat::write(__this, |__t| { let __ch = __t.content_height(); let __x = __t.scroll_bar_x(); (*__t.scroll_area_mut(), __ch, __x) });
        compat::extract_scrollbar(context, &area, content_height, scroll_bar_x);
    }

    /// java: public void open()
    pub fn open(&mut self) {
        if self.is_open() {
            return;
        }
        self.visible = true;
        self.animation = 0.0f32;
        self.animation_end = self.target_x;
        self.animation_start = self.target_x.wrapping_add(if self.right_side { self.get_width().wrapping_add(8) } else { (-self.get_width()).wrapping_sub(8) });
        self.is_open = true;
    }

    /// java: public void open(PositionedWidget positionedWidget, boolean rightSide)
    pub fn open_2(__this: &compat::Shared<Self>, mut positioned_widget: compat::Shared<PositionedWidget>, mut right_side: bool) {
        let config_screen = compat::read(__this, |__t| __t.config_screen());
        let mut hud_widget: compat::Shared<dyn HudWidget> = compat::read(&positioned_widget, |__t| __t.widget.clone());
        compat::write(__this, |__t| {
            __t.positioned_widget = Some(positioned_widget.clone());
            __t.layout = LinearLayout::vertical().spacing(5);
            let __s = __t.layout.default_cell_setting();
            *__s = (*__s).align_horizontally_center();
            // java: optionWidgets.clear();
            __t.option_widgets = Screen::new(String::new());
            __t.title_index = None;
            __t.position_rule = None;
            __t.generation = __t.generation.wrapping_add(1);
        });
        // java: add(new StringWidget(0, 15, hudWidget.getInformation().displayName().copy().withStyle(ChatFormatting.UNDERLINE), client.font) {
        //           @Override public void setWidth(int width) { setMaxWidth(width, TextOverflow.SCROLLING); } });
        let __title = ({ let __f = compat::dispatch(&hud_widget, |__t| __t.get_information()); __f(&hud_widget) }).display_name().unwrap().copy().with_style_format(ChatFormatting::Underline);
        let __i = compat::write(__this, |__t| __t.add(compat::string_widget(0, 0, 0, 15, __title)));
        compat::write(__this, |__t| __t.title_index = Some(__i));
        if !(compat::read(&positioned_widget, |__t| __t.from_tab)) {
            // java: add(Button.builder(Component.translatable("skyblocker.config.hud.widget.remove"), _ -> configScreen.removeWidget(positionedWidget)).build());
            let __cs = compat::RenderThread::new(std::sync::Arc::downgrade(&config_screen));
            let __pw = compat::RenderThread::new(positioned_widget.clone());
            let __s0 = (button::builder(component::translatable("skyblocker.config.hud.widget.remove").get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut Screen, __index: usize| {
                let Some(__config_screen) = __cs.get().upgrade() else { return; };
                WidgetsConfigurationScreen::remove_widget(&__config_screen, __pw.get());
            })))).build();
            compat::write(__this, |__t| __t.add(__s0));
        }
        // java: Button copyButton = add(Button.builder(.., _ -> configScreen.openPopup(screen -> new CopyToPopup(screen, positionedWidget, screen.getCurrentLocation(), screen.getCurrentScreenLayer()))).tooltip(..).build());
        let __cs2 = compat::RenderThread::new(std::sync::Arc::downgrade(&config_screen));
        let __pw2 = compat::RenderThread::new(positioned_widget.clone());
        let mut copy_button = (button::builder(component::translatable("skyblocker.config.hud.copy.copyTo").get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut Screen, __index: usize| {
            let Some(__config_screen) = __cs2.get().upgrade() else { return; };
            let __copied = __pw2.get().clone();
            // (the popup's background screen is the one the game holds under it - `set_screen_keeping_parent`; the base's
            // `Screen` field gets an empty stand-in)
            WidgetsConfigurationScreen::open_popup(&__config_screen, Arc::new(move |screen| {
                let __location = WidgetsConfigurationScreen::get_current_location(&screen);
                let __layer = WidgetsConfigurationScreen::get_current_screen_layer(&screen);
                Box::new(compat::ScreenAdapter::new(CopyToPopup::new(Screen::new(String::new()), __copied.clone(), __location, __layer))) as Box<dyn mcrust::mcrust::modding::screens::ModScreen>
            }));
        })))).tooltip_text(Some((Tooltip::create((component::translatable("skyblocker.config.hud.copy.copyTo.@Tooltip")).get_string())).message.clone())).build();
        // java: if (WidgetManager.ALLOWED_LOCATIONS.stream().noneMatch(l -> hudWidget.getInformation().available().test(l) && l != configScreen.getCurrentLocation())) {
        //           copyButton.active = false; copyButton.setTooltip(..); }
        let __available = ({ let __f = compat::dispatch(&hud_widget, |__t| __t.get_information()); __f(&hud_widget) }).available();
        let __current_location = WidgetsConfigurationScreen::get_current_location(&config_screen);
        if !widget_manager::ALLOWED_LOCATIONS.iter().copied().any(|l| (__available)(l) && l != __current_location) {
            copy_button.active = false;
            copy_button.set_tooltip(Tooltip::create((component::translatable("skyblocker.config.hud.copy.copyTo.disabled.@Tooltip")).get_string()));
        }
        compat::write(__this, |__t| __t.add(copy_button));
        compat::write(__this, |__t| __t.layout.add_child(SpacerElement::height(10).as_layout_element()));
        let mut available_width: i32 = compat::read(__this, |__t| __t.get_width()).wrapping_sub(*SCROLLBAR_AREA);
        if !(compat::read(&positioned_widget, |__t| __t.from_tab)) {
            // java: add(new PositionRuleWidget(configScreen, positionedWidget));
            let __rule = PositionRuleWidget::new(config_screen.clone(), positioned_widget.clone());
            let __w = crate::de::hysky::skyblocker::skyblock::tabhud::config::position_rule_widget::PositionRuleAdapter::widget(__rule.clone());
            let __i = compat::write(__this, |__t| __t.add(__w));
            compat::write(__this, |__t| {
                __t.position_rule = Some((__i, __rule));
                __t.layout.add_child(SpacerElement::height(10).as_layout_element());
            });
        }
        // java: List<AbstractWidget> collector = new ArrayList<>();
        //       OptionWidgetCollector widgetCollector = new OptionWidgetCollector(collector, hudWidget::updateConfigPreview, configScreen, configScreen.getCurrentLocation());
        //       hudWidget.getOptionWidgets(widgetCollector); widgetCollector.buildDeferred(); collector.forEach(this::add);
        let __preview_widget = hud_widget.clone();
        let mut widget_collector: OptionWidgetCollector = OptionWidgetCollector::new(Vec::new(), Arc::new(move || { ({ let __f = compat::dispatch(&__preview_widget, |__t| __t.update_config_preview()); __f(&__preview_widget) }); }), config_screen.clone(), __current_location);
        ({ let __f = compat::dispatch(&hud_widget, |__t| __t.get_option_widgets()); __f(&hud_widget, &mut widget_collector) });
        widget_collector.build_deferred();
        for __a0 in std::mem::take(&mut widget_collector.collector_list) {
            compat::write(__this, |__t| __t.add(__a0));
        }
        compat::write(__this, |__t| {
            __t.layout.add_child(SpacerElement::height(10).as_layout_element());
            // Position everything
            // java: for (AbstractWidget widget : optionWidgets) widget.setWidth(availableWidth);
            __t.set_children_width(available_width);
            // java: layout.setPosition(rightSide ? getX() : getX() + SCROLLBAR_AREA, getY() - (int) scrollAmount());
            let __layout_x: i32 = if right_side { __t.get_x() } else { __t.get_x().wrapping_add(*SCROLLBAR_AREA) };
            let __layout_y: i32 = __t.get_y().wrapping_sub((__t.scroll_amount()) as i32);
            __t.layout.set_x(__layout_x);
            __t.layout.set_y(__layout_y);
            // java: layout.arrangeElements();
            __t.arrange();
        });
        let __screen_width = compat::read(&config_screen, |__t| __t.width);
        compat::write(__this, |__t| {
            let mut open_x: i32 = if right_side { __screen_width.wrapping_sub(__t.get_width()) } else { 0 };
            if __t.is_open() && (open_x != __t.target_x || right_side != __t.right_side) {
                __t.is_open = false;
            }
            __t.right_side = right_side;
            __t.target_x = open_x;
            __t.open();
        });
    }

    /// `widget.setWidth(width)` on every option widget, through each one's own override (the title's
    /// `setMaxWidth(width, SCROLLING)`, `PositionRuleWidget.setWidth`).
    fn set_children_width(&mut self, width: i32) {
        for i in 0..self.option_widgets.children().len() {
            if Some(i) == self.title_index {
                // java: @Override public void setWidth(int width) { setMaxWidth(width, TextOverflow.SCROLLING); }
                self.option_widgets.widget_mut(i).set_max_width(width, TextOverflow::Scrolling);
                continue;
            }
            if let Some((index, rule)) = self.position_rule.clone() && index == i {
                // `PositionRuleWidget.setWidth` - its children follow and its height becomes its layout's
                let (w, h) = compat::write(&rule, |__t| { __t.set_width(width); (__t.get_width(), __t.get_height()) });
                let widget = self.option_widgets.widget_mut(i);
                widget.width = w;
                widget.height = h;
                continue;
            }
            self.option_widgets.widget_mut(i).set_width(width);
        }
    }

    /// java: public void close()
    pub fn close(&mut self) {
        if !self.is_open() {
            return;
        }
        self.animation_start = self.get_x();
        self.animation_end = self.get_x().wrapping_add(if self.right_side { self.get_width().wrapping_add(8) } else { (-self.get_width()).wrapping_sub(8) });
        self.animation = 0.0f32;
        self.is_open = false;
    }

    /// java: public boolean isOpen()
    pub fn is_open(&self) -> bool {
        return self.is_open;
    }

    /// java: @Override public void setScrollAmount(double scrollY)
    pub fn set_scroll_amount(&mut self, mut scroll_y: f64) {
        // java: super.setScrollAmount(scrollY); - clamped to [0, maxScrollAmount()] over `contentHeight()`
        let __content_height: i32 = self.content_height();
        self.scroll_area_mut().set_scroll_amount(__content_height, scroll_y);
        // java: layout.setY(getY() - (int) scrollAmount());
        let __s0 = self.get_y().wrapping_sub((self.scroll_amount()) as i32);
        self.layout.set_y(__s0);
        self.place();
    }

    /// java: @Override public void setX(int x)
    pub fn set_x(&mut self, mut x: i32) {
        self.base.set_x(x);
        if self.right_side {
            self.layout.set_x(x);
        } else {
            self.layout.set_x(x.wrapping_add(*SCROLLBAR_AREA));
        }
        self.place();
    }

    /// java: @Override public boolean mouseClicked(..) - `AbstractContainerWidget.mouseClicked`: `updateScrolling(event)`,
    /// then `ContainerEventHandler.mouseClicked` over `children()` (the child under the pointer takes it), `|| scrolling`
    pub fn mouse_clicked(__this: &compat::Shared<Self>, click: &MouseButtonEvent, doubled: bool) -> bool {
        let scrolling = compat::write(__this, |__t| {
            let (__ch, __x) = (__t.content_height(), __t.scroll_bar_x());
            __t.scroll_area_mut().update_scrolling(__ch, __x, click.button(), click.x(), click.y())
        });
        let consumed = SidePanelWidget::with_children(__this, |widgets| {
            if widgets.get_child_at(click.x(), click.y()).is_none() {
                return false;
            }
            let _ = widgets.mouse_clicked(click.button(), click.x(), click.y());
            true
        });
        let _ = doubled;
        consumed || scrolling
    }

    /// java: `AbstractContainerWidget.mouseReleased` - `super.mouseReleased` (the scrollbar drag ends), then the focused,
    /// dragging child's
    pub fn mouse_released(__this: &compat::Shared<Self>, click: &MouseButtonEvent) -> bool {
        compat::write(__this, |__t| __t.scroll_area.on_release());
        SidePanelWidget::with_children(__this, |widgets| {
            widgets.mouse_released(click.button(), click.x(), click.y());
            false
        })
    }

    /// java: `AbstractContainerWidget.mouseDragged` - `super.mouseDragged` (the scrollbar drag, `setScrollAmount` - this
    /// class's override), then the focused, dragging child's
    pub fn mouse_dragged(__this: &compat::Shared<Self>, click: &MouseButtonEvent, dx: f64, dy: f64) -> bool {
        compat::write(__this, |__t| {
            if __t.scroll_area.is_scrolling() {
                let __ch = __t.content_height();
                if __t.scroll_area_mut().mouse_dragged(__ch, click.y(), dy) {
                    let __amount = __t.scroll_amount();
                    __t.set_scroll_amount(__amount);
                }
            }
        });
        SidePanelWidget::with_children(__this, |widgets| {
            widgets.mouse_dragged(click.x(), click.y(), dx, dy);
            false
        })
    }

    /// java: `ContainerEventHandler.keyPressed` - the focused child's
    pub fn key_pressed(__this: &compat::Shared<Self>, event: &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        SidePanelWidget::with_children(__this, |widgets| {
            widgets.get_focused().is_some() && widgets.key_pressed(event.key(), event.has_shift_down()).is_some()
        })
    }

    /// The option widgets, taken out of the panel for a call that may reach back into it (a button's action closes
    /// the panel, a copy popup reads it) and put back after it - unless the call replaced them (`open(..)`).
    fn with_children<R>(__this: &compat::Shared<Self>, f: impl FnOnce(&mut Screen) -> R) -> R {
        let __generation = compat::read(__this, |__t| __t.generation);
        let mut __widgets = compat::write(__this, |__t| std::mem::replace(&mut __t.option_widgets, Screen::new(String::new())));
        let out = f(&mut __widgets);
        compat::write(__this, |__t| {
            if __t.generation == __generation {
                __t.option_widgets = __widgets;
                // a child's size may have changed (a PositionRuleWidget's height follows its layout)
                __t.place();
            }
        });
        out
    }

    /// java: @Override public boolean mouseScrolled(double mouseX, double mouseY, double horizontalAmount, double verticalAmount)
    pub fn mouse_scrolled(__this: &compat::Shared<Self>, mut mouse_x: f64, mut mouse_y: f64, mut horizontal_amount: f64, mut vertical_amount: f64) -> bool {
        // java: if (this.getChildAt(mouseX, mouseY).filter(child -> !(child instanceof CycleButton<?>)).filter(element -> element.mouseScrolled(..)).isPresent()) return true;
        let __taken = SidePanelWidget::with_children(__this, |widgets| {
            let Some(index) = widgets.get_child_at(mouse_x, mouse_y) else { return false; };
            if matches!(widgets.widget(index).kind, WidgetKind::CycleButton(_)) {
                return false;
            }
            widgets.mouse_scrolled_mod_widget(mouse_x, mouse_y, horizontal_amount, vertical_amount)
        });
        if __taken {
            return true;
        }
        // java: return super.mouseScrolled(mouseX, mouseY, horizontalAmount, verticalAmount);
        // (`AbstractScrollArea.mouseScrolled`: `if (!visible) return false; setScrollAmount(scrollAmount() - verticalAmount *
        // scrollRate()); return true;` - `setScrollAmount` is this class's override, which moves the layout)
        compat::write(__this, |__t| {
            if !__t.visible {
                return false;
            }
            let __target: f64 = __t.scroll_amount() - vertical_amount * __t.scroll_area.scroll_rate();
            __t.set_scroll_amount(__target);
            true
        })
    }

    /// java: @Override public void setWidth(int width)
    pub fn set_width(&mut self, mut width: i32) {
        self.base.set_width(width);
        // java: widget.setWidth(getWidth() - SCROLLBAR_AREA); // remove 6 for scrollbar and one for a liiiitle padding
        let __width = self.get_width().wrapping_sub(*SCROLLBAR_AREA);
        self.set_children_width(__width);
        self.arrange();
    }

    /// java: @Override public void setHeight(int height)
    pub fn set_height(&mut self, mut height: i32) {
        self.base.set_height(height.wrapping_sub(TOP_MARGIN));
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput output)
    pub fn update_widget_narration(&self, output: &dyn NarrationElementOutput) {
    }

    /// java: public @Nullable PositionedWidget getPositionedWidget()
    pub fn get_positioned_widget(&self) -> Option<compat::Shared<PositionedWidget>> {
        return self.positioned_widget.clone();
    }

    /// java: private <T extends AbstractWidget> T add(T widget) - `optionWidgets.add(widget); return layout.addChild(widget);`
    /// (the widget's place in `option_widgets` - what the layout holds)
    pub fn add(&mut self, widget: AbstractWidget) -> usize {
        let index = self.option_widgets.add_renderable_widget(widget);
        self.layout.add_widget_child(&self.option_widgets, index)
    }

    /// java: @Override protected int scrollBarX()
    pub fn scroll_bar_x(&self) -> i32 {
        // java: return rightSide ? super.scrollBarX() : getX(); - `AbstractScrollArea.scrollBarX()` is `getRight() - scrollbarWidth()`
        return (if self.right_side { self.get_right().wrapping_sub(self.scroll_area.scrollbar_width()) } else { self.get_x() });
    }

}
