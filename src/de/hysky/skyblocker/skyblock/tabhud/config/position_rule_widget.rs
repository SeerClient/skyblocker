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
use mcrust::net::minecraft::client::gui::components::events::gui_event_listener::GuiEventListener;
use mcrust::net::minecraft::client::gui::components::tooltip::Tooltip;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::layouts::layout_element::LayoutElement;
use mcrust::net::minecraft::client::gui::layouts::linear_layout::LinearLayout;
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use mcrust::net::minecraft::client::gui::navigation::screen_position::ScreenPosition;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::client::input::mouse_button_info::MouseButtonInfo;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::client::gui::components::renderable::Renderable as _;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: class PositionRuleWidget extends AbstractContainerWidget
pub struct PositionRuleWidget {
    /// java: extends net.minecraft.client.gui.components.AbstractContainerWidget
    pub base: AbstractContainerWidget,
    /// java: LinearLayout layout = LinearLayout.vertical();
    pub layout: LinearLayout,
    /// java: List<AbstractWidget> widgets = new ArrayList<>();
    pub widgets: Vec<AbstractWidget>,
    /// java: private final StringWidget coordsDisplay;
    pub coords_display: Option<AbstractWidget>,
    /// java: private final Button parentButton;
    pub parent_button: Option<AbstractWidget>,
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
    pub fn new(mut config: compat::Shared<WidgetsConfigurationScreen>, mut modified_widget: compat::Shared<PositionedWidget>) -> Self {
        let __init_widget_config = config.clone();
        let __init_modified_widget = modified_widget.clone();
        // TODO(fabric2seer): call to `defaultSettings(..)` (not a method of this class)
        //   java: super(0, 0, 0, 0, Component.literal("hi"), defaultSettings(0));
        let mut this: Self = todo!("constructor");
        this.layout.default_cell_setting().align_horizontally_center();
        this.parent_button = Some((button::builder((component::translatable_args("skyblocker.config.hud.position.parent", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(this.get_parent_name())])).get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { WidgetsConfigurationScreen::prompt_select_widget(&config, Arc::new(move |__a0| { this.on_widget_selected(__a0.clone()); }), false, &component::translatable("skyblocker.config.hud.position.parent.@Tooltip")); })))).tooltip_text(Some((Tooltip::create((component::translatable("skyblocker.config.hud.position.parent.@Tooltip")).get_string())).message.clone())).build());
        this.coords_display = Some(compat::string_widget(0, 0, -1, 9, component::literal("hi")));
        this.coords_display.as_mut().unwrap().set_height((this.coords_display.as_ref().unwrap().get_height()).wrapping_add(6));
        let mut parent_point: compat::Shared<AnchorSelectionWidget> = AnchorSelectionWidget::new(component::translatable("skyblocker.config.hud.position.pointParent"), true);
        let mut this_point: compat::Shared<AnchorSelectionWidget> = AnchorSelectionWidget::new(component::translatable("skyblocker.config.hud.position.pointThis"), false);
        this.add(compat::string_widget(0, 0, -1, 9, component::literal("Positioning")));
        this.add(this.parent_button.clone().unwrap());
        this.add(this.coords_display.clone().unwrap());
        this.add(parent_point.clone());
        this.add(this_point.clone());
        this.layout.arrange_elements();
        this.set_height(this.layout.get_height());
        this
    }

    /// java: private void add(AbstractWidget widget)
    pub fn add(&mut self, mut widget: AbstractWidget) {
        self.widgets.push(widget.clone());
        // TODO(fabric2seer): a widget put in a layout outside a mod screen's own methods: the port's layouts hold a widget by its index in the screen's widget list
        // rust: self.layout.add_child(widget.clone());
    }

    /// java: private void onWidgetSelected(@Nullable HudWidget selectedWidget)
    pub fn on_widget_selected(&mut self, mut selected_widget: Option<compat::Shared<dyn HudWidget>>) {
        let mut old_rule: PositionRule = compat::read(&self.modified_widget, |__t| __t.rule.clone());
        let mut edited_widget: compat::Shared<dyn HudWidget> = WidgetsConfigurationScreen::get_edited_widget(&self.widget_config);
        let mut this_anchor_x: i32 = ((((({ let __f = compat::dispatch(&edited_widget, |__t| __t.get_x()); __f(&edited_widget) }) as f32) + (old_rule.this_point().horizontal_point().get_percentage() * (({ let __f = compat::dispatch(&edited_widget, |__t| __t.get_width()); __f(&edited_widget) }) as f32)))) as i32);
        let mut this_anchor_y: i32 = ((((({ let __f = compat::dispatch(&edited_widget, |__t| __t.get_y()); __f(&edited_widget) }) as f32) + (old_rule.this_point().vertical_point().get_percentage() * (({ let __f = compat::dispatch(&edited_widget, |__t| __t.get_height()); __f(&edited_widget) }) as f32)))) as i32);
        let mut other_anchor_x: i32 = (if selected_widget.is_none() { 0 } else { ({ let __r = selected_widget.as_ref().unwrap(); let __f = compat::dispatch(__r, |__t| __t.get_x()); __f(__r) }) });
        let mut other_anchor_y: i32 = (if selected_widget.is_none() { 0 } else { ({ let __r = selected_widget.as_ref().unwrap(); let __f = compat::dispatch(__r, |__t| __t.get_y()); __f(__r) }) });
        let mut new_rule: PositionRule = PositionRule::new(selected_widget.clone().map(|__a0| ({ let __f = compat::dispatch(&__a0, |__t| __t.get_internal_id()); __f(&__a0) })), position_rule::POINT_DEFAULT.clone(), old_rule.this_point(), this_anchor_x.wrapping_sub(other_anchor_x), this_anchor_y.wrapping_sub(other_anchor_y));
        if let Some(selected_widget) = &selected_widget {
            self.parent_button.as_mut().unwrap().set_message_text(mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::translatable_args("skyblocker.config.hud.position.parent", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(({ let __f = compat::dispatch(selected_widget, |__t| __t.get_information()); __f(selected_widget) }).display_name().unwrap())])));
        } else {
            self.parent_button.as_mut().unwrap().set_message_text(mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::translatable_args("skyblocker.config.hud.position.parent", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::translatable("skyblocker.config.hud.position.parent.screen"))])));
        }
        { let __v = new_rule.clone(); compat::write(&self.modified_widget, |__t| __t.rule = __v); }
    }

    /// java: @Override public void setWidth(int width)
    pub fn set_width(&mut self, mut width: i32) {
        self.base.set_width(width);
        for widget in self.widgets.iter_mut() {
            widget.set_width(width);
        }
        // TODO(fabric2seer): field `SCROLLING` of a value of unknown type
        //   java: coordsDisplay.setMaxWidth(width, StringWidget.TextOverflow.SCROLLING);
        self.layout.arrange_elements();
        { let __s0 = self.layout.get_height(); self.set_height(__s0) };
    }

    /// java: @Override public void setX(int x)
    pub fn set_x(&mut self, mut x: i32) {
        self.base.set_x(x);
        self.layout.set_x(x);
    }

    /// java: @Override public void setY(int y)
    pub fn set_y(&mut self, mut y: i32) {
        self.base.set_y(y);
        self.layout.set_y(y);
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

    /// java: @Override public List<? extends GuiEventListener> children()
    pub fn children(&self) -> Vec<Box<dyn GuiEventListener>> {
        return self.widgets.clone();
    }

    /// java: @Override protected int contentHeight()
    pub fn content_height(&self) -> i32 {
        return self.get_height();
    }

    /// java: @Override protected double scrollRate()
    pub fn scroll_rate(&self) -> f64 {
        return ((0) as f64) as f64;
    }

    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor context, int mouseX, int mouseY, float deltaTicks)
    pub fn extract_widget_render_state(&mut self, context: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut delta_ticks: f32) {
        { let __s0 = mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::literal(&format!("x: {}, y: {}", self.modified_widget.lock().borrow().rule.relative_x(), self.modified_widget.lock().borrow().rule.relative_y()))); self.coords_display.as_mut().unwrap().set_message_text(__s0) };
        self.layout.arrange_elements();
        for widget in self.widgets.iter() {
            // TODO(fabric2seer): `extract_render_state` is a method of the port's trait `Renderable`, which the port's `AbstractWidget` does not implement
            // rust: widget.extract_render_state(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mouse_x, mouse_y, delta_ticks);
        }
    }

    /// java: @Override public boolean mouseScrolled(double mouseX, double mouseY, double horizontalAmount, double verticalAmount)
    pub fn mouse_scrolled(&self, mut mouse_x: f64, mut mouse_y: f64, mut horizontal_amount: f64, mut vertical_amount: f64) -> bool {
        return false;
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    pub fn update_widget_narration(&self, builder: &dyn NarrationElementOutput) {
    }

}

// NOTE(fabric2seer): children, contentHeight, extractWidgetRenderState, mouseScrolled, scrollRate, setWidth, setX, setY, updateWidgetNarration override(s) of AbstractContainerWidget - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them

/// java: private class AnchorSelectionWidget extends AbstractWidget
#[derive(Clone)]
pub struct AnchorSelectionWidget {
    /// java: extends net.minecraft.client.gui.components.AbstractWidget
    pub base: AbstractWidget,
    /// java: private final boolean parent;
    pub parent: bool,
    /// java: private PositionRule.@Nullable Point hoveredPoint = null;
    pub hovered_point: Option<Point>,
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
    pub fn new(mut text: MutableComponent, mut parent: bool) -> compat::Shared<Self> {
        let __init_parent = parent;
        let __this = compat::shared(Self {
            base: AbstractWidget::new(0, 0, 20, 40, (&text.get_string()).to_string(), WidgetKind::PlainText, OnPress::Inert),
            parent: __init_parent,
            hovered_point: None,
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
        // TODO(fabric2seer): `modifiedWidget` is a field of the enclosing instance (PositionRuleWidget.this) - an inner class has no outer reference in Rust
        //   java: PositionRule rule = modifiedWidget.rule;
        let mut rule: PositionRule = todo!("fabric2seer: the initialiser of `rule` (TODO above)");
        for i in 0..3 {
            for j in 0..3 {
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
        // TODO(fabric2seer): `widgetConfig` is a field of the enclosing instance (PositionRuleWidget.this) - an inner class has no outer reference in Rust
        //   java: HudWidget editedWidget = widgetConfig.getEditedWidget();
        let mut edited_widget: compat::Shared<dyn HudWidget> = todo!("fabric2seer: the initialiser of `editedWidget` (TODO above)");
        if compat::read(__this, |__t| __t.hovered_point.clone()).is_some() {
            // TODO(fabric2seer): `modifiedWidget` is a field of the enclosing instance (PositionRuleWidget.this) - an inner class has no outer reference in Rust
            //   java: PositionRule oldRule = modifiedWidget.rule;
            let mut old_rule: PositionRule = todo!("fabric2seer: the initialiser of `oldRule` (TODO above)");
            // TODO(fabric2seer): `widgetConfig` is a field of the enclosing instance (PositionRuleWidget.this) - an inner class has no outer reference in Rust
            //   java: ScreenPosition startPos = WidgetPositioner.getStartPosition(oldRule.parent().orElse(null), widgetConfig.getScreenWidth(), widgetConfig.getScreenHeight(), parent ? hoveredPoint : oldRule.parentPoint());
            let mut start_pos: ScreenPosition = todo!("fabric2seer: the initialiser of `startPos` (TODO above)");
            let mut this_point: Point = (if compat::read(__this, |__t| __t.parent) { Some(old_rule.this_point()) } else { compat::read(__this, |__t| __t.hovered_point.clone()) }).unwrap();
            let mut end_pos: ScreenPosition = ScreenPosition::new(((((({ let __f = compat::dispatch(&edited_widget, |__t| __t.get_x()); __f(&edited_widget) }) as f32) + (this_point.horizontal_point().get_percentage() * (({ let __f = compat::dispatch(&edited_widget, |__t| __t.get_width()); __f(&edited_widget) }) as f32)))) as i32), ((((({ let __f = compat::dispatch(&edited_widget, |__t| __t.get_y()); __f(&edited_widget) }) as f32) + (this_point.vertical_point().get_percentage() * (({ let __f = compat::dispatch(&edited_widget, |__t| __t.get_height()); __f(&edited_widget) }) as f32)))) as i32));
            if compat::read(__this, |__t| __t.parent) {
                // TODO(fabric2seer): `modifiedWidget` is a field of the enclosing instance (PositionRuleWidget.this) - an inner class has no outer reference in Rust
                //   java: modifiedWidget.rule = new PositionRule(
                //   java: 							oldRule.parent(),
                //   java: 							hoveredPoint,
                //   java: 							oldRule.thisPoint(),
                //   java: 							endPos.x() - startPos.x(),
                //   java: 							endPos.y() - startPos.y());
            } else {
                // TODO(fabric2seer): `modifiedWidget` is a field of the enclosing instance (PositionRuleWidget.this) - an inner class has no outer reference in Rust
                //   java: modifiedWidget.rule = new PositionRule(
                //   java: 							oldRule.parent(),
                //   java: 							oldRule.parentPoint(),
                //   java: 							hoveredPoint,
                //   java: 							endPos.x() - startPos.x(),
                //   java: 							endPos.y() - startPos.y());
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

// NOTE(fabric2seer): extractWidgetRenderState, onClick, updateWidgetNarration override(s) of AbstractWidget - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them
