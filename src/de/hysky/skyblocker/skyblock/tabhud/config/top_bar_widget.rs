//! Ported from de.hysky.skyblocker.skyblock.tabhud.config.TopBarWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::tabhud::config::custom_dropdown_widget::CustomDropdownAdapter;
use crate::de::hysky::skyblocker::skyblock::tabhud::config::custom_dropdown_widget::CustomDropdownWidget;
use crate::de::hysky::skyblocker::skyblock::tabhud::config::other_options_screen::OtherOptionsScreen;
use crate::de::hysky::skyblocker::skyblock::tabhud::config::styled_button_widget::StyledButtonWidget;
use crate::de::hysky::skyblocker::skyblock::tabhud::config::toggle_button_widget::ToggleButtonWidget;
use crate::de::hysky::skyblocker::skyblock::tabhud::config::widgets_configuration_screen::WidgetsConfigurationScreen;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::widget_manager;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::widget_manager::ScreenLayer;
use crate::de::hysky::skyblocker::utils::location::Location;
use mcrust::mcrust::modding::widgets::WidgetRenderContext;
use mcrust::net::minecraft::client::gui::components::abstract_container_widget::AbstractContainerWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::abstract_widget::PressLambda;
use mcrust::net::minecraft::client::gui::components::tooltip::Tooltip;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::layouts::abstract_layout::AbstractLayout;
use mcrust::net::minecraft::client::gui::layouts::frame_layout;
use mcrust::net::minecraft::client::gui::layouts::layout;
use mcrust::net::minecraft::client::gui::layouts::layout_element::LayoutElement;
use mcrust::net::minecraft::client::gui::layouts::linear_layout::LinearLayout;
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use mcrust::net::minecraft::client::gui::screens::screen::Screen;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::network::chat::component;
use mcrust::mcrust::modding::screens::ModScreen;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::util::argb;
use crate::de::hysky::skyblocker::utils::render::gui::dropdown_widget::DropdownWidget as _;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final Identifier TEXTURE = SkyblockerMod.id("menu_outer_space");
pub static TEXTURE: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("menu_outer_space"));
/// java: private static final int HEIGHT = 15;
pub const HEIGHT: i32 = 15;

/// java: class TopBarWidget extends AbstractContainerWidget
pub struct TopBarWidget {
    /// java: extends net.minecraft.client.gui.components.AbstractContainerWidget
    pub base: AbstractContainerWidget,
    /// java: private final CustomDropdownWidget<Location> locationDropdown;
    pub location_dropdown: compat::Shared<CustomDropdownWidget<Location>>,
    /// java: private final CustomDropdownWidget<WidgetManager.ScreenLayer> screenLayerDropdown;
    pub screen_layer_dropdown: compat::Shared<CustomDropdownWidget<ScreenLayer>>,
    /// java: private final Layout layout;
    pub layout: Layout,
    /// java: private final List<AbstractWidget> widgets; - `List.of(optionsButton, helpButton, locationDropdown,
    /// screenLayerDropdown, autoAnchorToggle)`, held in this widget's own `Screen` list (the list this port's layouts
    /// index, and the one its widget pass draws and asks)
    pub widgets: Screen,
}

impl std::ops::Deref for TopBarWidget {
    type Target = AbstractContainerWidget;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for TopBarWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl TopBarWidget {
    /// java: TopBarWidget(int width, WidgetsConfigurationScreen parent)
    /// (the screen holds this widget: the actions hold the screen weakly)
    pub fn new(width: i32, parent: &compat::Shared<WidgetsConfigurationScreen>) -> compat::Shared<Self> {
        // java: layout = new Layout();
        let mut layout = Layout::new();
        let mut widgets = Screen::new(String::new());
        // java: LinearLayout leftButtons = LinearLayout.horizontal();
        let mut left_buttons: LinearLayout = LinearLayout::horizontal();
        // java: StyledButtonWidget optionsButton = new StyledButtonWidget(60, HEIGHT, Component.translatable("skyblocker.config.hud.topBar.options"), _ -> parent.openPopup(OtherOptionsScreen::new));
        let __parent_w1 = compat::RenderThread::new(std::sync::Arc::downgrade(parent));
        let options_button: compat::Shared<StyledButtonWidget> = StyledButtonWidget::new(60, HEIGHT, component::translatable("skyblocker.config.hud.topBar.options"), OnPress::Lambda(PressLambda::new(move |__screen: &mut Screen, __index: usize| {
            let Some(parent) = __parent_w1.get().upgrade() else { return; };
            WidgetsConfigurationScreen::open_popup(&parent, Arc::new(move |__a0| Box::new(compat::ScreenAdapter::new(OtherOptionsScreen::new(__a0))) as Box<dyn ModScreen>));
        })));
        // java: optionsButton.setTooltip(Tooltip.create(Component.translatable("skyblocker.config.hud.topBar.options.@Tooltip")));
        let __tooltip = Tooltip::create((component::translatable("skyblocker.config.hud.topBar.options.@Tooltip")).get_string());
        compat::write(&options_button, |__t| __t.set_tooltip(__tooltip));
        // java: StyledButtonWidget helpButton = new StyledButtonWidget(60, HEIGHT, Component.literal("(?) ").append(Component.translatable("skyblocker.config.hud.topBar.help")),
        //           _ -> parent.openPopup(screen -> new PopupScreen.Builder(screen, Component.translatable("skyblocker.config.hud.help"))
        //               .addMessage(Component.translatable("skyblocker.config.hud.helpText")).addButton(CommonComponents.GUI_OK, PopupScreen::onClose).setWidth(300).build()));
        // (the builder's `screen` is the popup's background: the screen it is opened over, held by `set_screen_keeping_parent`)
        let __parent_w5 = compat::RenderThread::new(std::sync::Arc::downgrade(parent));
        let help_button: compat::Shared<StyledButtonWidget> = StyledButtonWidget::new(60, HEIGHT, component::literal("(?) ").append(component::translatable("skyblocker.config.hud.topBar.help")), OnPress::Lambda(PressLambda::new(move |__screen: &mut Screen, __index: usize| {
            let Some(parent) = __parent_w5.get().upgrade() else { return; };
            WidgetsConfigurationScreen::open_popup(&parent, Arc::new(move |_screen| {
                Box::new(compat::BatchUPopupScreenBuilder::new(component::translatable("skyblocker.config.hud.help"))
                    .add_message(component::translatable("skyblocker.config.hud.helpText"))
                    .add_button(component::translatable("gui.ok"), |popup_screen| popup_screen.on_close())
                    .set_width(300)
                    .build()) as Box<dyn ModScreen>
            }));
        })));
        // java: leftButtons.addChild(optionsButton); leftButtons.addChild(helpButton); layout.add(leftButtons);
        let __i = widgets.add_renderable_widget(compat::WidgetAdapter::widget(options_button.clone()));
        left_buttons.add_widget_child(&widgets, __i);
        let __i = widgets.add_renderable_widget(compat::WidgetAdapter::widget(help_button.clone()));
        left_buttons.add_widget_child(&widgets, __i);
        layout.add(LayoutElement::Grid(left_buttons.wrapped.clone()));
        // java: locationDropdown = new CustomDropdownWidget<>(width / 2 - 100 - 5, 0, 100, 200, List.copyOf(WidgetManager.ALLOWED_LOCATIONS), parent::setCurrentLocation, parent.getCurrentLocation());
        let __parent_w2 = compat::RenderThread::new(std::sync::Arc::downgrade(parent));
        let location_dropdown = CustomDropdownWidget::new(((width.wrapping_div(2)).wrapping_sub(100)).wrapping_sub(5), 0, 100, 200, widget_manager::ALLOWED_LOCATIONS.iter().cloned().collect::<Vec<_>>(), Arc::new(move |__a0: Location| { if let Some(parent) = __parent_w2.get().upgrade() { WidgetsConfigurationScreen::set_current_location(&parent, __a0); } }), WidgetsConfigurationScreen::get_current_location(parent));
        // java: locationDropdown.setFormatter(l -> Component.literal(l.toString()));
        compat::write(&location_dropdown, |__t| __t.set_formatter(Arc::new(move |l: Location| component::literal(&compat::jstring(&l)))));
        // java: screenLayerDropdown = new CustomDropdownWidget<>(width / 2 + 5, 0, 100, 200, List.of(WidgetManager.ScreenLayer.values()), parent::setCurrentScreenLayer, WidgetManager.ScreenLayer.HUD);
        let __parent_w3 = compat::RenderThread::new(std::sync::Arc::downgrade(parent));
        let screen_layer_dropdown = CustomDropdownWidget::new((width.wrapping_div(2)).wrapping_add(5), 0, 100, 200, ScreenLayer::values().to_vec(), Arc::new(move |__a0: ScreenLayer| { if let Some(parent) = __parent_w3.get().upgrade() { WidgetsConfigurationScreen::set_current_screen_layer(&parent, __a0); } }), ScreenLayer::Hud);
        // java: LinearLayout dropdownsLayout = LinearLayout.horizontal().spacing(2);
        //       dropdownsLayout.addChild(locationDropdown); dropdownsLayout.addChild(screenLayerDropdown); layout.add(dropdownsLayout);
        let mut dropdowns_layout: LinearLayout = LinearLayout::horizontal().spacing(2);
        let __i = widgets.add_renderable_widget(CustomDropdownAdapter::widget(location_dropdown.clone()));
        dropdowns_layout.add_widget_child(&widgets, __i);
        let __i = widgets.add_renderable_widget(CustomDropdownAdapter::widget(screen_layer_dropdown.clone()));
        dropdowns_layout.add_widget_child(&widgets, __i);
        layout.add(LayoutElement::Grid(dropdowns_layout.wrapped.clone()));
        // java: ToggleButtonWidget autoAnchorToggle = new ToggleButtonWidget(100, HEIGHT, Component.translatable("skyblocker.config.hud.topBar.autoAnchor"), b -> parent.autoAnchor = b);
        let __parent_w4 = compat::RenderThread::new(std::sync::Arc::downgrade(parent));
        let auto_anchor_toggle: compat::Shared<ToggleButtonWidget> = ToggleButtonWidget::new(100, HEIGHT, component::translatable("skyblocker.config.hud.topBar.autoAnchor"), Arc::new(move |b| { if let Some(parent) = __parent_w4.get().upgrade() { compat::write(&parent, |__t| __t.auto_anchor = b); } }));
        // java: autoAnchorToggle.setState(true);
        ToggleButtonWidget::set_state(&auto_anchor_toggle, true);
        // java: autoAnchorToggle.setTooltip(Tooltip.create(Component.translatable("skyblocker.config.hud.topBar.autoAnchor.@Tooltip")));
        let __tooltip = Tooltip::create((component::translatable("skyblocker.config.hud.topBar.autoAnchor.@Tooltip")).get_string());
        compat::write(&auto_anchor_toggle, |__t| __t.set_tooltip(__tooltip));
        // java: LinearLayout rightButtons = LinearLayout.horizontal(); rightButtons.addChild(autoAnchorToggle); layout.add(rightButtons);
        let mut right_buttons: LinearLayout = LinearLayout::horizontal();
        let __i = widgets.add_renderable_widget(compat::WidgetAdapter::widget(auto_anchor_toggle.clone()));
        right_buttons.add_widget_child(&widgets, __i);
        layout.add(LayoutElement::Grid(right_buttons.wrapped.clone()));
        // java: super(0, 0, width, HEIGHT, Component.literal("hi"), defaultSettings(0));
        let mut this: Self = Self {
            base: AbstractContainerWidget::new_with_scrollbar_settings(0, 0, width, HEIGHT, mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::literal("hi")), mcrust::net::minecraft::client::gui::components::abstract_scroll_area::ScrollbarSettings::default_settings(0.0)),
            location_dropdown,
            screen_layer_dropdown,
            layout,
            widgets,
        };
        // java: layout.arrangeElements();
        this.arrange_layout();
        compat::shared(this)
    }

    /// `layout.arrangeElements()` with the inner class's `TopBarWidget.this` box brought up to date first, over the
    /// widgets' live sizes, and the arranged positions written into the widget list.
    pub fn arrange_layout(&mut self) {
        self.layout.outer_box = (self.base.get_x(), self.base.get_y(), self.base.get_width(), self.get_height());
        self.layout.sync_widget_sizes(&self.widgets);
        self.layout.arrange_elements();
        self.layout.place_widgets(&mut self.widgets);
    }

    /// java: static void drawButtonBorder(GuiGraphicsExtractor context, int x, int y, int y2)
    pub fn draw_button_border(context: &GuiGraphicsExtractor, mut x: i32, mut y: i32, mut y2: i32) {
        gui_graphics_extractor::vertical_line(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), x.wrapping_sub(1), y, y2, argb::color_alpha_rgb(15, -1));
        gui_graphics_extractor::vertical_line(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), x, y, y2, argb::color_alpha_rgb(100, 0));
        gui_graphics_extractor::vertical_line(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), x.wrapping_add(1), y, y2, argb::color_alpha_rgb(15, -1));
    }

    /// java: @Override public int getHeight()
    pub fn get_height(&self) -> i32 {
        let __location = compat::read(&self.location_dropdown, |__t| __t.get_height());
        let __layer = compat::read(&self.screen_layer_dropdown, |__t| __t.get_height());
        return self.base.get_height().max(__location.max(__layer));
    }

    /// `AbstractWidget.isMouseOver` - `isActive() && areCoordinatesInRectangle(x, y)`, over this class's `getHeight()`
    pub fn is_mouse_over(&self, mouse_x: f64, mouse_y: f64) -> bool {
        self.base.active && self.base.visible
            && mouse_x >= self.get_x() as f64 && mouse_y >= self.get_y() as f64
            && mouse_x < (self.get_x() + self.get_width()) as f64 && mouse_y < (self.get_y() + self.get_height()) as f64
    }

    /// java: @Override public void setWidth(int width)
    pub fn set_width(&mut self, mut width: i32) {
        self.base.set_width(width);
        self.arrange_layout();
    }

    /// `AbstractWidget.extractRenderState` - `if (visible) extractWidgetRenderState(..)` (hover is the widget pass's)
    pub fn extract_render_state(__this: &compat::Shared<Self>, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, delta_ticks: f32) {
        if !compat::read(__this, |__t| __t.visible) {
            return;
        }
        TopBarWidget::extract_widget_render_state(__this, context, mouse_x, mouse_y, delta_ticks);
    }

    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor context, int mouseX, int mouseY, float deltaTicks)
    pub fn extract_widget_render_state(__this: &compat::Shared<Self>, context: &mut WidgetRenderContext<'_>, mut mouse_x: i32, mut mouse_y: i32, mut delta_ticks: f32) {
        let (x, y, w) = compat::read(__this, |__t| (__t.get_x(), __t.get_y(), __t.get_width()));
        gui_graphics_extractor::blit_sprite_identifier(context.gl, context.minecraft, mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), TEXTURE.get(), x.wrapping_sub(2), y.wrapping_sub(2), w.wrapping_add(4), HEIGHT.wrapping_add(2));
        // java: for (AbstractWidget widget : widgets) widget.extractRenderState(context, mouseX, mouseY, deltaTicks);
        let mut __widgets = compat::write(__this, |__t| std::mem::replace(&mut __t.widgets, Screen::new(String::new())));
        context.extract_widgets(&mut __widgets, mouse_x, mouse_y);
        compat::write(__this, |__t| __t.widgets = __widgets);
        let _ = delta_ticks;
    }

    /// The children, taken out of the object for a call that reaches back into it (a dropdown's select callback
    /// rebuilds the screen's layer, a button opens a popup) and put back after it. A child's size may have changed (a
    /// dropdown opened), so the layout is arranged again.
    fn with_children<R>(__this: &compat::Shared<Self>, f: impl FnOnce(&mut Screen) -> R) -> R {
        let mut __widgets = compat::write(__this, |__t| std::mem::replace(&mut __t.widgets, Screen::new(String::new())));
        let out = f(&mut __widgets);
        compat::write(__this, |__t| __t.widgets = __widgets);
        out
    }

    /// java: `AbstractContainerWidget.mouseClicked` - `updateScrolling` (`NO_SCROLL`: never) and
    /// `ContainerEventHandler.mouseClicked` over `children()`
    pub fn mouse_clicked(__this: &compat::Shared<Self>, click: &MouseButtonEvent, doubled: bool) -> bool {
        let _ = doubled;
        TopBarWidget::with_children(__this, |widgets| {
            if widgets.get_child_at(click.x(), click.y()).is_none() {
                return false;
            }
            let _ = widgets.mouse_clicked(click.button(), click.x(), click.y());
            true
        })
    }

    /// java: `ContainerEventHandler.mouseReleased` - the focused, dragging child's
    pub fn mouse_released(__this: &compat::Shared<Self>, click: &MouseButtonEvent) -> bool {
        TopBarWidget::with_children(__this, |widgets| {
            widgets.mouse_released(click.button(), click.x(), click.y());
            false
        })
    }

    /// java: `ContainerEventHandler.mouseDragged` - the focused, dragging child's (a dropdown list's scrollbar)
    pub fn mouse_dragged(__this: &compat::Shared<Self>, click: &MouseButtonEvent, dx: f64, dy: f64) -> bool {
        TopBarWidget::with_children(__this, |widgets| {
            widgets.mouse_dragged(click.x(), click.y(), dx, dy);
            false
        })
    }

    /// java: @Override public boolean mouseScrolled(double mouseX, double mouseY, double horizontalAmount, double verticalAmount)
    pub fn mouse_scrolled(__this: &compat::Shared<Self>, mut mouse_x: f64, mut mouse_y: f64, mut horizontal_amount: f64, mut vertical_amount: f64) -> bool {
        if !compat::read(__this, |__t| __t.visible) {
            return false;
        }
        // java: if (this.getChildAt(mouseX, mouseY).filter(element -> element.mouseScrolled(mouseX, mouseY, horizontalAmount, verticalAmount)).isPresent()) return true;
        if TopBarWidget::with_children(__this, |widgets| widgets.mouse_scrolled_mod_widget(mouse_x, mouse_y, horizontal_amount, vertical_amount)) {
            return true;
        }
        // java: return super.mouseScrolled(mouseX, mouseY, horizontalAmount, verticalAmount);
        // (`AbstractScrollArea.mouseScrolled`: `setScrollAmount(scrollAmount() - verticalAmount * scrollRate())` - this
        // widget is `NO_SCROLL` with a `contentHeight()` of 0, so the amount stays 0 - then `return true`)
        return true;
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    pub fn update_widget_narration(&self, builder: &dyn NarrationElementOutput) {
    }

    /// java: @Override protected int contentHeight()
    pub fn content_height(&self) -> i32 {
        return 0;
    }

}

/// java: private class Layout extends AbstractLayout
pub struct Layout {
    /// java: extends AbstractLayout - `super(0, 0, 0, 0)`'s four fields (`AbstractLayout` is a trait here, implemented
    /// below; this class's `getX()` .. `getHeight()` overrides answer `outer_box` instead)
    pub fields: (i32, i32, i32, i32),
    /// java: private final List<LayoutElement> widgets = new ArrayList<>(4);
    pub widgets: Vec<LayoutElement>,
    /// `TopBarWidget.this`'s box `(getX(), getY(), getWidth(), getHeight())`, which the inner class's four getters
    /// answer: the enclosing widget writes it before every `arrangeElements` (`TopBarWidget::arrange_layout`)
    pub outer_box: (i32, i32, i32, i32),
}

impl Layout {
    /// java: Layout()
    pub fn new() -> Self {
        let mut this = Self {
            fields: (0, 0, 0, 0),
            widgets: Vec::with_capacity((4) as usize),
            outer_box: (0, 0, 0, 0),
        };
        this
    }

    /// java: private void add(LayoutElement widget)
    pub fn add(&mut self, mut widget: LayoutElement) {
        self.widgets.push(widget);
    }

    /// java: @Override public void visitChildren(Consumer<LayoutElement> consumer)
    pub fn visit_children(&self, consumer: Arc<dyn Fn(LayoutElement)>) {
        // java: widgets.forEach(consumer);
        for __w in self.widgets.iter() {
            (consumer)(__w.clone());
        }
    }

    /// java: @Override public int getX()
    pub fn get_x(&self) -> i32 {
        // java: return TopBarWidget.this.getX();
        return self.outer_box.0;
    }

    /// java: @Override public int getY()
    pub fn get_y(&self) -> i32 {
        // java: return TopBarWidget.this.getY();
        return self.outer_box.1;
    }

    /// java: @Override public int getWidth()
    pub fn get_width(&self) -> i32 {
        // java: return TopBarWidget.this.getWidth();
        return self.outer_box.2;
    }

    /// java: @Override public int getHeight()
    pub fn get_height(&self) -> i32 {
        // java: return TopBarWidget.this.getHeight();
        return self.outer_box.3;
    }

    /// java: @Override public void arrangeElements()
    pub fn arrange_elements(&mut self) {
        // java: super.arrangeElements(); - each child layout arranges its own elements
        for __child in self.widgets.iter_mut() {
            layout::arrange_elements(__child);
        }
        let (x, y, width) = (self.get_x(), self.get_y(), self.get_width());
        let __last = self.widgets.len() - 1;
        let first: &mut LayoutElement = &mut self.widgets[0];
        first.set_position(x, y);
        let mut low: i32 = first.get_x().wrapping_add(first.get_width());
        let last: &mut LayoutElement = &mut self.widgets[__last];
        let __last_width = last.get_width();
        last.set_position((x.wrapping_add(width)).wrapping_sub(__last_width), y);
        let mut high: i32 = last.get_x();
        for i in 1..(self.widgets.len() as i32).wrapping_sub(1) {
            let widget: &mut LayoutElement = &mut self.widgets[(i) as usize];
            widget.set_y(y);
            // java: FrameLayout.alignInDimension(0, getWidth(), widget.getWidth(), widget::setX, 0.5f);
            let __x = frame_layout::align_in_dimension(0, width, widget.get_width(), 0.5f32);
            widget.set_x(__x);
            if widget.get_x().wrapping_add(widget.get_width()) > high {
                // java: FrameLayout.alignInDimension(low, high - low, widget.getWidth(), widget::setX, 0.5f);
                let __x = frame_layout::align_in_dimension(low, high.wrapping_sub(low), widget.get_width(), 0.5f32);
                widget.set_x(__x);
            }
        }
    }

    /// java: @Override public void removeChildren()
    pub fn remove_children(&mut self) {
        self.widgets.clear();
    }

}


/// java: `extends AbstractLayout` - `setX` / `setY` move every child by the layout's delta (the trait's bodies), and the
/// four getters are this class's overrides (`TopBarWidget.this`'s box)
impl AbstractLayout for Layout {
    fn abstract_layout_fields(&mut self) -> (&mut i32, &mut i32, &mut i32, &mut i32) {
        let f = &mut self.fields;
        (&mut f.0, &mut f.1, &mut f.2, &mut f.3)
    }

    fn abstract_layout_values(&self) -> (i32, i32, i32, i32) {
        self.outer_box
    }

    fn visit_children_mut(&mut self, visitor: &mut dyn FnMut(&mut LayoutElement)) {
        for __w in self.widgets.iter_mut() {
            visitor(__w);
        }
    }
}

impl Layout {
    /// The positions this layout's children were arranged to, written into the top bar's widget list (Java's layout
    /// moves the widget objects themselves).
    pub fn place_widgets(&self, widgets: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        for __w in self.widgets.iter() {
            __w.place_widgets(widgets);
        }
    }

    /// The layout's copy of each child widget's size brought up to the widget list (Java reads it off the widget).
    pub fn sync_widget_sizes(&mut self, widgets: &mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        for __w in self.widgets.iter_mut() {
            __w.sync_widget_sizes(widgets);
        }
    }
}
