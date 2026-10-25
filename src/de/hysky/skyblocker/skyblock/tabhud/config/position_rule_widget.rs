//! Ported from de.hysky.skyblocker.skyblock.tabhud.config.PositionRuleWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::tabhud::config::widgets_configuration_screen::WidgetsConfigurationScreen;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::pipeline::position_rule;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::pipeline::position_rule::HorizontalPoint;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::pipeline::position_rule::Point;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::pipeline::position_rule::PositionRule;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::pipeline::position_rule::VerticalPoint;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::positioned_widget::PositionedWidget;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::widget_manager::WidgetManager;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::hud_widget::HudWidget;
use crate::de::hysky::skyblocker::utils::render::gui_helper::GuiHelper;
use mcrust::mcrust::modding::widgets::ModWidget;
use mcrust::mcrust::modding::widgets::WidgetRenderContext;
use mcrust::net::minecraft::client::gui::components::abstract_container_widget::AbstractContainerWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::abstract_widget::PressLambda;
use mcrust::net::minecraft::client::gui::components::abstract_widget::WidgetKind;
use mcrust::net::minecraft::client::gui::components::button;
use mcrust::net::minecraft::client::gui::components::string_widget::TextOverflow;
use mcrust::net::minecraft::client::gui::components::tooltip::Tooltip;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::layouts::linear_layout::LinearLayout;
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use mcrust::net::minecraft::client::gui::navigation::screen_position::ScreenPosition;
use mcrust::net::minecraft::client::gui::screens::screen::Screen;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::formatted_text::FormattedText;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: class PositionRuleWidget extends AbstractContainerWidget
pub struct PositionRuleWidget {
    /// java: extends net.minecraft.client.gui.components.AbstractContainerWidget
    pub base: AbstractContainerWidget,
    /// java: LinearLayout layout = LinearLayout.vertical();
    pub layout: LinearLayout,
    /// java: List<AbstractWidget> widgets = new ArrayList<>();
    /// (the widget objects, held in this widget's own `Screen` list - the list this port's layouts index, and the one
    /// its widget pass draws and asks)
    pub widgets: Screen,
    /// java: private final StringWidget coordsDisplay; - its place in `widgets`
    pub coords_display: usize,
    /// java: private final Button parentButton; - its place in `widgets`
    pub parent_button: usize,
    /// java: private final PositionedWidget modifiedWidget;
    pub modified_widget: compat::Shared<PositionedWidget>,
    /// java: private final WidgetsConfigurationScreen widgetConfig;
    pub widget_config: compat::Shared<WidgetsConfigurationScreen>,
}

impl std::ops::Deref for PositionRuleWidget {
    type Target = AbstractContainerWidget;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for PositionRuleWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl PositionRuleWidget {
    /// java: PositionRuleWidget(WidgetsConfigurationScreen config, PositionedWidget modifiedWidget)
    /// (a shared handle: the parent button's action calls back into this object, `this::onWidgetSelected`)
    pub fn new(mut config: compat::Shared<WidgetsConfigurationScreen>, mut modified_widget: compat::Shared<PositionedWidget>) -> compat::Shared<Self> {
        // java: super(0, 0, 0, 0, Component.literal("hi"), defaultSettings(0));
        let __this: compat::Shared<Self> = compat::shared(Self {
            base: AbstractContainerWidget::new_with_scrollbar_settings(0, 0, 0, 0, FormattedText::from(component::literal("hi")), mcrust::net::minecraft::client::gui::components::abstract_scroll_area::ScrollbarSettings::default_settings(0.0)),
            layout: LinearLayout::vertical(),
            widgets: Screen::new(String::new()),
            coords_display: 0,
            parent_button: 0,
            modified_widget: modified_widget.clone(),
            widget_config: config.clone(),
        });
        // java: layout.defaultCellSetting().alignHorizontallyCenter();
        compat::write(&__this, |__t| { let __s = __t.layout.default_cell_setting(); *__s = (*__s).align_horizontally_center(); });
        // java: parentButton = Button.builder(Component.translatable("skyblocker.config.hud.position.parent", getParentName()),
        //           _ -> config.promptSelectWidget(this::onWidgetSelected, false, Component.translatable("skyblocker.config.hud.position.parent.@Tooltip")))
        //       .tooltip(Tooltip.create(Component.translatable("skyblocker.config.hud.position.parent.@Tooltip"))).build();
        // (this widget belongs to the screen: the action holds the screen and this object weakly)
        let __parent_name = compat::read(&__this, |__t| __t.get_parent_name());
        let __config_w = compat::RenderThread::new(std::sync::Arc::downgrade(&config));
        let __this_w = compat::RenderThread::new(std::sync::Arc::downgrade(&__this));
        let mut parent_button = (button::builder(String::new(), OnPress::Lambda(PressLambda::new(move |__screen: &mut Screen, __index: usize| {
            let Some(__config) = __config_w.get().upgrade() else { return; };
            let __self_w = __this_w.get().clone();
            WidgetsConfigurationScreen::prompt_select_widget(&__config, Arc::new(move |__a0| { if let Some(__self) = __self_w.upgrade() { PositionRuleWidget::on_widget_selected(&__self, __a0); } }), false, &component::translatable("skyblocker.config.hud.position.parent.@Tooltip"));
        })))).tooltip_text(Some((Tooltip::create((component::translatable("skyblocker.config.hud.position.parent.@Tooltip")).get_string())).message.clone())).build();
        parent_button.set_message_text(FormattedText::from(component::translatable_args("skyblocker.config.hud.position.parent", vec![FormattedText::from(__parent_name)])));
        // java: coordsDisplay = new StringWidget(Component.literal("hi"), Minecraft.getInstance().font);
        //       coordsDisplay.setHeight(coordsDisplay.getHeight() + 6);
        let mut coords_display = compat::string_widget(0, 0, -1, 9, component::literal("hi"));
        let __coords_height = coords_display.get_height().wrapping_add(6);
        coords_display.set_height(__coords_height);
        // java: AnchorSelectionWidget parentPoint = new AnchorSelectionWidget(Component.translatable("skyblocker.config.hud.position.pointParent"), true);
        //       AnchorSelectionWidget thisPoint = new AnchorSelectionWidget(Component.translatable("skyblocker.config.hud.position.pointThis"), false);
        let parent_point: compat::Shared<AnchorSelectionWidget> = AnchorSelectionWidget::new(component::translatable("skyblocker.config.hud.position.pointParent"), true, modified_widget.clone(), config.clone());
        let this_point: compat::Shared<AnchorSelectionWidget> = AnchorSelectionWidget::new(component::translatable("skyblocker.config.hud.position.pointThis"), false, modified_widget.clone(), config.clone());
        compat::write(&__this, |__t| {
            // java: add(new StringWidget(Component.literal("Positioning"), Minecraft.getInstance().font));
            __t.add(compat::string_widget(0, 0, -1, 9, component::literal("Positioning")));
            // java: add(parentButton); add(coordsDisplay); add(parentPoint); add(thisPoint);
            __t.parent_button = __t.add(parent_button);
            __t.coords_display = __t.add(coords_display);
            __t.add(compat::WidgetAdapter::widget(parent_point.clone()));
            __t.add(compat::WidgetAdapter::widget(this_point.clone()));
            // java: layout.arrangeElements(); setHeight(layout.getHeight());
            __t.arrange();
            let __h = __t.layout.get_height();
            __t.set_height(__h);
        });
        __this
    }

    /// java: private void add(AbstractWidget widget) - `widgets.add(widget); layout.addChild(widget);` (answers the
    /// widget's place in `widgets`)
    pub fn add(&mut self, mut widget: AbstractWidget) -> usize {
        let index = self.widgets.add_renderable_widget(widget);
        self.layout.add_widget_child(&self.widgets, index)
    }

    /// `layout.arrangeElements()` over the widgets' live sizes, its positions written into the widget list (Java's
    /// layout moves the widget objects).
    fn arrange(&mut self) {
        self.layout.sync_widget_sizes(&self.widgets);
        self.layout.arrange_elements();
        self.layout.place_widgets(&mut self.widgets);
    }

    /// java: private void onWidgetSelected(@Nullable HudWidget selectedWidget)
    pub fn on_widget_selected(__this: &compat::Shared<Self>, mut selected_widget: Option<compat::Shared<dyn HudWidget>>) {
        let (modified_widget, widget_config) = compat::read(__this, |__t| (__t.modified_widget.clone(), __t.widget_config.clone()));
        let mut old_rule: PositionRule = compat::read(&modified_widget, |__t| __t.rule.clone());
        let mut edited_widget: compat::Shared<dyn HudWidget> = WidgetsConfigurationScreen::get_edited_widget(&widget_config);
        let mut this_anchor_x: i32 = ((((({ let __f = compat::dispatch(&edited_widget, |__t| __t.get_x()); __f(&edited_widget) }) as f32) + (old_rule.this_point().horizontal_point().get_percentage() * (({ let __f = compat::dispatch(&edited_widget, |__t| __t.get_width()); __f(&edited_widget) }) as f32)))) as i32);
        let mut this_anchor_y: i32 = ((((({ let __f = compat::dispatch(&edited_widget, |__t| __t.get_y()); __f(&edited_widget) }) as f32) + (old_rule.this_point().vertical_point().get_percentage() * (({ let __f = compat::dispatch(&edited_widget, |__t| __t.get_height()); __f(&edited_widget) }) as f32)))) as i32);
        let mut other_anchor_x: i32 = match &selected_widget { None => 0, Some(__r) => { let __f = compat::dispatch(__r, |__t| __t.get_x()); __f(__r) } };
        let mut other_anchor_y: i32 = match &selected_widget { None => 0, Some(__r) => { let __f = compat::dispatch(__r, |__t| __t.get_y()); __f(__r) } };
        let mut new_rule: PositionRule = PositionRule::new(selected_widget.clone().map(|__a0| ({ let __f = compat::dispatch(&__a0, |__t| __t.get_internal_id()); __f(&__a0) })), position_rule::POINT_DEFAULT.clone(), old_rule.this_point(), this_anchor_x.wrapping_sub(other_anchor_x), this_anchor_y.wrapping_sub(other_anchor_y));
        let __message = if let Some(selected_widget) = &selected_widget {
            FormattedText::from(component::translatable_args("skyblocker.config.hud.position.parent", vec![FormattedText::from(({ let __f = compat::dispatch(selected_widget, |__t| __t.get_information()); __f(selected_widget) }).display_name().unwrap())]))
        } else {
            FormattedText::from(component::translatable_args("skyblocker.config.hud.position.parent", vec![FormattedText::from(component::translatable("skyblocker.config.hud.position.parent.screen"))]))
        };
        compat::write(__this, |__t| { let __i = __t.parent_button; __t.widgets.widget_mut(__i).set_message_text(__message); });
        { let __v = new_rule.clone(); compat::write(&modified_widget, |__t| __t.rule = __v); }
    }

    /// java: @Override public void setWidth(int width)
    pub fn set_width(&mut self, mut width: i32) {
        self.base.set_width(width);
        // java: for (AbstractWidget widget : widgets) widget.setWidth(width);
        for i in 0..self.widgets.children().len() {
            self.widgets.widget_mut(i).set_width(width);
        }
        // java: coordsDisplay.setMaxWidth(width, StringWidget.TextOverflow.SCROLLING);
        let __i = self.coords_display;
        self.widgets.widget_mut(__i).set_max_width(width, TextOverflow::Scrolling);
        self.arrange();
        { let __s0 = self.layout.get_height(); self.set_height(__s0) };
    }

    /// java: @Override public void setX(int x)
    pub fn set_x(&mut self, mut x: i32) {
        self.base.set_x(x);
        self.layout.set_x(x);
        self.layout.place_widgets(&mut self.widgets);
    }

    /// java: @Override public void setY(int y)
    pub fn set_y(&mut self, mut y: i32) {
        self.base.set_y(y);
        self.layout.set_y(y);
        self.layout.place_widgets(&mut self.widgets);
    }

    /// java: private Component getParentName()
    pub fn get_parent_name(&self) -> MutableComponent {
        let mut rule: PositionRule = compat::read(&self.modified_widget, |__t| __t.rule.clone());
        if rule.parent().is_none() {
            return component::translatable("skyblocker.config.hud.position.parent.screen");
        } else {
            let mut widget: compat::Shared<dyn HudWidget> = WidgetManager::get_widget_or_placeholder(rule.parent().unwrap());
            return ({ let __f = compat::dispatch(&widget, |__t| __t.get_information()); __f(&widget) }).display_name().unwrap();
        }
    }

    /// java: @Override protected int contentHeight()
    pub fn content_height(&self) -> i32 {
        return self.get_height();
    }

    /// java: @Override protected double scrollRate()
    pub fn scroll_rate(&self) -> f64 {
        return 0.0;
    }

    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor context, int mouseX, int mouseY, float deltaTicks)
    /// (`clip` is the box this widget is drawn clipped to, which its children are drawn clipped to as well)
    pub fn extract_widget_render_state(__this: &compat::Shared<Self>, context: &mut WidgetRenderContext<'_>, mut mouse_x: i32, mut mouse_y: i32, mut delta_ticks: f32, clip: Option<(i32, i32, i32, i32)>) {
        // java: coordsDisplay.setMessage(Component.literal("x: " + modifiedWidget.rule.relativeX() + ", y: " + modifiedWidget.rule.relativeY()));
        let __rule = compat::read(&compat::read(__this, |__t| __t.modified_widget.clone()), |__t| __t.rule.clone());
        let __coords = FormattedText::from(component::literal(&format!("x: {}, y: {}", __rule.relative_x(), __rule.relative_y())));
        compat::write(__this, |__t| {
            let __i = __t.coords_display;
            __t.widgets.widget_mut(__i).set_message_text(__coords);
            // java: layout.arrangeElements();
            __t.arrange();
            for i in 0..__t.widgets.children().len() {
                __t.widgets.widget_mut(i).clip = clip;
            }
        });
        // java: for (AbstractWidget widget : widgets) widget.extractRenderState(context, mouseX, mouseY, deltaTicks);
        let mut __widgets = compat::write(__this, |__t| std::mem::replace(&mut __t.widgets, Screen::new(String::new())));
        context.extract_widgets(&mut __widgets, mouse_x, mouse_y);
        compat::write(__this, |__t| __t.widgets = __widgets);
        let _ = delta_ticks;
    }

    /// java: `AbstractContainerWidget.mouseClicked` - `updateScrolling` (nothing to scroll: `contentHeight()` is the
    /// height) and `ContainerEventHandler.mouseClicked` over `children()`
    pub fn mouse_clicked(__this: &compat::Shared<Self>, click: &MouseButtonEvent, doubled: bool) -> bool {
        let _ = doubled;
        PositionRuleWidget::with_children(__this, |widgets| {
            if widgets.get_child_at(click.x(), click.y()).is_none() {
                return false;
            }
            let _ = widgets.mouse_clicked(click.button(), click.x(), click.y());
            true
        })
    }

    /// java: `ContainerEventHandler.mouseReleased` - the focused, dragging child's
    pub fn mouse_released(__this: &compat::Shared<Self>, click: &MouseButtonEvent) -> bool {
        PositionRuleWidget::with_children(__this, |widgets| {
            widgets.mouse_released(click.button(), click.x(), click.y());
            false
        })
    }

    /// java: `ContainerEventHandler.mouseDragged` - the focused, dragging child's
    pub fn mouse_dragged(__this: &compat::Shared<Self>, click: &MouseButtonEvent, dx: f64, dy: f64) -> bool {
        PositionRuleWidget::with_children(__this, |widgets| {
            widgets.mouse_dragged(click.x(), click.y(), dx, dy);
            false
        })
    }

    /// The children, taken out of the object for a call that reaches back into it (the parent button's prompt, an
    /// anchor's rule write) and put back after it.
    fn with_children<R>(__this: &compat::Shared<Self>, f: impl FnOnce(&mut Screen) -> R) -> R {
        let mut __widgets = compat::write(__this, |__t| std::mem::replace(&mut __t.widgets, Screen::new(String::new())));
        let out = f(&mut __widgets);
        compat::write(__this, |__t| __t.widgets = __widgets);
        out
    }

    /// java: @Override public boolean mouseScrolled(double mouseX, double mouseY, double horizontalAmount, double verticalAmount)
    pub fn mouse_scrolled(&self, mut mouse_x: f64, mut mouse_y: f64, mut horizontal_amount: f64, mut vertical_amount: f64) -> bool {
        return false;
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    pub fn update_widget_narration(&self, builder: &dyn NarrationElementOutput) {
    }

}

/// `PositionRuleWidget` as a child widget OBJECT of a container (`SidePanelWidget.optionWidgets`, its `LinearLayout`):
/// the game's widget in the panel's child `Screen`, attached to the object. Every call first moves the object to the
/// game widget's position through its own `setX` / `setY` (which move its layout and children).
pub struct PositionRuleAdapter {
    object: compat::RenderThread<compat::Shared<PositionRuleWidget>>,
}

impl PositionRuleAdapter {
    /// `add(new PositionRuleWidget(..))` - the game's widget, from the object's box, with the object attached.
    pub fn widget(object: compat::Shared<PositionRuleWidget>) -> AbstractWidget {
        let (x, y, w, h) = compat::read(&object, |__t| (__t.get_x(), __t.get_y(), __t.get_width(), __t.get_height()));
        mcrust::mcrust::modding::widgets::ModWidgetSlot::widget(x, y, w, h, String::from("hi"), Box::new(PositionRuleAdapter { object: compat::RenderThread::new(object) }))
    }

    fn sync(&self, widget: &AbstractWidget) -> compat::Shared<PositionRuleWidget> {
        let object = self.object.get().clone();
        let (wx, wy) = (widget.get_x(), widget.get_y());
        compat::write(&object, |__t| {
            if __t.get_x() != wx {
                __t.set_x(wx);
            }
            if __t.get_y() != wy {
                __t.set_y(wy);
            }
        });
        object
    }
}

impl ModWidget for PositionRuleAdapter {
    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor context, int mouseX, int mouseY, float deltaTicks)
    fn extract_widget_render_state(&mut self, widget: &AbstractWidget, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        let object = self.sync(widget);
        PositionRuleWidget::extract_widget_render_state(&object, context, mouse_x, mouse_y, a, widget.clip);
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder) - empty
    fn update_widget_narration(&mut self, _widget: &AbstractWidget, _output: &mut dyn NarrationElementOutput) {}

    /// java: AbstractContainerWidget.mouseClicked
    fn mouse_clicked(&mut self, widget: &mut AbstractWidget, event: &MouseButtonEvent, double_click: bool) -> bool {
        let object = self.sync(widget);
        PositionRuleWidget::mouse_clicked(&object, event, double_click)
    }

    /// java: AbstractContainerWidget.mouseReleased
    fn mouse_released(&mut self, widget: &mut AbstractWidget, event: &MouseButtonEvent) -> bool {
        let object = self.sync(widget);
        PositionRuleWidget::mouse_released(&object, event)
    }

    /// java: AbstractContainerWidget.mouseDragged
    fn mouse_dragged(&mut self, widget: &mut AbstractWidget, event: &MouseButtonEvent, dx: f64, dy: f64) -> bool {
        let object = self.sync(widget);
        PositionRuleWidget::mouse_dragged(&object, event, dx, dy)
    }

    /// java: @Override public boolean mouseScrolled(..) - `false`
    fn mouse_scrolled(&mut self, _widget: &mut AbstractWidget, _x: f64, _y: f64, _scroll_x: f64, _scroll_y: f64) -> bool {
        false
    }
}

/// java: private class AnchorSelectionWidget extends AbstractWidget
#[derive(Clone)]
pub struct AnchorSelectionWidget {
    /// java: extends net.minecraft.client.gui.components.AbstractWidget
    pub base: AbstractWidget,
    /// java: private final boolean parent;
    pub parent: bool,
    /// java: private PositionRule.@Nullable Point hoveredPoint = null;
    pub hovered_point: Option<Point>,
    /// `PositionRuleWidget.this.modifiedWidget` - the enclosing instance's field this inner class reads
    pub outer_modified_widget: compat::Shared<PositionedWidget>,
    /// `PositionRuleWidget.this.widgetConfig` - the enclosing instance's field this inner class reads
    pub outer_widget_config: compat::Shared<WidgetsConfigurationScreen>,
}

impl std::ops::Deref for AnchorSelectionWidget {
    type Target = AbstractWidget;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for AnchorSelectionWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl AnchorSelectionWidget {
    /// java: private AnchorSelectionWidget(Component text, boolean parent)
    /// (`modified_widget` and `widget_config` are the enclosing `PositionRuleWidget.this`'s two fields)
    pub fn new(mut text: MutableComponent, mut parent: bool, modified_widget: compat::Shared<PositionedWidget>, widget_config: compat::Shared<WidgetsConfigurationScreen>) -> compat::Shared<Self> {
        let __init_parent = parent;
        let __this = compat::shared(Self {
            base: AbstractWidget::new(0, 0, 20, 40, (&text.get_string()).to_string(), WidgetKind::PlainText, OnPress::Inert),
            parent: __init_parent,
            hovered_point: None,
            outer_modified_widget: modified_widget,
            outer_widget_config: widget_config,
        });
        __this
    }

    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor context, int mouseX, int mouseY, float delta)
    pub fn extract_widget_render_state(__this: &compat::Shared<Self>, context: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut delta: f32) {
        { let __v = None; compat::write(__this, |__t| __t.hovered_point = __v); }
        gui_graphics_extractor::draw_formatted_text_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(compat::component_of(__this.lock().borrow().get_message_component())), __this.lock().borrow().get_x(), __this.lock().borrow().get_y(), mcrust::net::minecraft::util::common_colors::WHITE, true);
        let __pose_1 = gui_graphics_extractor::push_pose();
        gui_graphics_extractor::pose_translate((__this.lock().borrow().get_x()) as f32, ((__this.lock().borrow().get_y()).wrapping_add(10)) as f32);
        let mut x: i32 = (__this.lock().borrow().get_width()).wrapping_div(6);
        let mut w: i32 = ((((4i32.wrapping_mul(__this.lock().borrow().get_width()) as f32) / 6.0f32)) as i32);
        let mut y: i32 = 5;
        let mut h: i32 = 20;
        GuiHelper::border(context, x, y.wrapping_add(1), w, h, mcrust::net::minecraft::util::common_colors::WHITE);
        // java: PositionRule rule = modifiedWidget.rule;
        let mut rule: PositionRule = compat::read(&compat::read(__this, |__t| __t.outer_modified_widget.clone()), |__t| __t.rule.clone());
        for i in 0..3i32 {
            for j in 0..3i32 {
                let mut square_x: i32 = x.wrapping_add((i.wrapping_mul(__this.lock().borrow().get_width())).wrapping_div(3));
                let mut square_y: i32 = y.wrapping_add((j.wrapping_mul(30)).wrapping_div(3));
                let mut point: Point = (if compat::read(__this, |__t| __t.parent) { rule.parent_point() } else { rule.this_point() });
                let mut selected_anchor: bool = (point.horizontal_point().ordinal() == i && point.vertical_point().ordinal() == j);
                let mut hovered_anchor: bool = (mouse_x >= (__this.lock().borrow().get_x()).wrapping_add((i.wrapping_mul(__this.lock().borrow().get_width())).wrapping_div(3)) && mouse_x < (__this.lock().borrow().get_x()).wrapping_add(((i.wrapping_add(1)).wrapping_mul(__this.lock().borrow().get_width())).wrapping_div(3)) && mouse_y >= ((__this.lock().borrow().get_y()).wrapping_add(10)).wrapping_add(j.wrapping_mul(10)) && mouse_y < ((__this.lock().borrow().get_y()).wrapping_add(10)).wrapping_add((j.wrapping_add(1)).wrapping_mul(10)));
                if hovered_anchor {
                    let mut vertical_points: Vec<VerticalPoint> = VerticalPoint::values();
                    let mut horizontal_points: Vec<HorizontalPoint> = HorizontalPoint::values();
                    { let __v = Some(Point::new(vertical_points[(j) as usize], horizontal_points[(i) as usize])); compat::write(__this, |__t| __t.hovered_point = __v); }
                }
                context.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), square_x.wrapping_sub(1), square_y.wrapping_sub(1), square_x.wrapping_add(2), square_y.wrapping_add(2), (if hovered_anchor { mcrust::net::minecraft::util::common_colors::RED } else { (if selected_anchor { mcrust::net::minecraft::util::common_colors::YELLOW } else { mcrust::net::minecraft::util::common_colors::WHITE }) }));
            }
        }
        gui_graphics_extractor::pop_pose(__pose_1);
    }

    /// java: @Override public void onClick(MouseButtonEvent click, boolean doubled)
    pub fn on_click(__this: &compat::Shared<Self>, click: &MouseButtonEvent, mut doubled: bool) {
        // java: HudWidget editedWidget = widgetConfig.getEditedWidget();
        let __widget_config: compat::Shared<WidgetsConfigurationScreen> = compat::read(__this, |__t| __t.outer_widget_config.clone());
        let __modified_widget: compat::Shared<PositionedWidget> = compat::read(__this, |__t| __t.outer_modified_widget.clone());
        let mut edited_widget: compat::Shared<dyn HudWidget> = WidgetsConfigurationScreen::get_edited_widget(&__widget_config);
        if compat::read(__this, |__t| __t.hovered_point.clone()).is_some() {
            // java: PositionRule oldRule = modifiedWidget.rule;
            let mut old_rule: PositionRule = compat::read(&__modified_widget, |__t| __t.rule.clone());
            // java: ScreenPosition startPos = WidgetPositioner.getStartPosition(oldRule.parent().orElse(null), widgetConfig.getScreenWidth(), widgetConfig.getScreenHeight(), parent ? hoveredPoint : oldRule.parentPoint());
            let __start_point: Point = if compat::read(__this, |__t| __t.parent) { compat::read(__this, |__t| __t.hovered_point.clone()).unwrap() } else { old_rule.parent_point() };
            let mut start_pos: ScreenPosition = crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::widget_positioner::WidgetPositionerBase::get_start_position(old_rule.parent(), WidgetsConfigurationScreen::get_screen_width(&__widget_config), WidgetsConfigurationScreen::get_screen_height(&__widget_config), &__start_point);
            let mut this_point: Point = (if compat::read(__this, |__t| __t.parent) { Some(old_rule.this_point()) } else { compat::read(__this, |__t| __t.hovered_point.clone()) }).unwrap();
            let mut end_pos: ScreenPosition = ScreenPosition::new(((((({ let __f = compat::dispatch(&edited_widget, |__t| __t.get_x()); __f(&edited_widget) }) as f32) + (this_point.horizontal_point().get_percentage() * (({ let __f = compat::dispatch(&edited_widget, |__t| __t.get_width()); __f(&edited_widget) }) as f32)))) as i32), ((((({ let __f = compat::dispatch(&edited_widget, |__t| __t.get_y()); __f(&edited_widget) }) as f32) + (this_point.vertical_point().get_percentage() * (({ let __f = compat::dispatch(&edited_widget, |__t| __t.get_height()); __f(&edited_widget) }) as f32)))) as i32));
            if compat::read(__this, |__t| __t.parent) {
                // java: modifiedWidget.rule = new PositionRule(oldRule.parent(), hoveredPoint, oldRule.thisPoint(), endPos.x() - startPos.x(), endPos.y() - startPos.y());
                let __rule = PositionRule::new_canonical(old_rule.parent(), compat::read(__this, |__t| __t.hovered_point.clone()).unwrap(), old_rule.this_point(), end_pos.x().wrapping_sub(start_pos.x()), end_pos.y().wrapping_sub(start_pos.y()));
                compat::write(&__modified_widget, |__t| __t.rule = __rule);
            } else {
                // java: modifiedWidget.rule = new PositionRule(oldRule.parent(), oldRule.parentPoint(), hoveredPoint, endPos.x() - startPos.x(), endPos.y() - startPos.y());
                let __rule = PositionRule::new_canonical(old_rule.parent(), old_rule.parent_point(), compat::read(__this, |__t| __t.hovered_point.clone()).unwrap(), end_pos.x().wrapping_sub(start_pos.x()), end_pos.y().wrapping_sub(start_pos.y()));
                compat::write(&__modified_widget, |__t| __t.rule = __rule);
            }
        }
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    pub fn update_widget_narration(__this: &compat::Shared<Self>, builder: &dyn NarrationElementOutput) {
    }

}

/// AnchorSelectionWidget's widget half (`super`'s fields: x, y, size, message, ...), lent by the game for each call
impl compat::WidgetHalf for AnchorSelectionWidget {
    fn widget_half(&mut self) -> &mut AbstractWidget {
        &mut self.base
    }
}

/// AnchorSelectionWidget as the game's widget: `compat::WidgetAdapter::widget(handle)` where Java hands it on
impl ModWidget for compat::WidgetAdapter<AnchorSelectionWidget> {
    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor context, int mouseX, int mouseY, float delta)
    fn extract_widget_render_state(&mut self, widget: &AbstractWidget, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        self.lend_ref(widget, |__h| AnchorSelectionWidget::extract_widget_render_state(__h, &mut context.graphics, mouse_x, mouse_y, a))
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    fn update_widget_narration(&mut self, widget: &AbstractWidget, output: &mut dyn NarrationElementOutput) {
        self.lend_ref(widget, |__h| AnchorSelectionWidget::update_widget_narration(__h, output))
    }

    /// java: @Override public void onClick(MouseButtonEvent click, boolean doubled)
    fn on_click(&mut self, widget: &mut AbstractWidget, event: &MouseButtonEvent, double_click: bool) {
        self.lend(widget, |__h| AnchorSelectionWidget::on_click(__h, event, double_click))
    }

}

