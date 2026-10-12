//! Ported from de.hysky.skyblocker.skyblock.tabhud.config.TopBarWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::tabhud::config::custom_dropdown_widget::CustomDropdownWidget;
use crate::de::hysky::skyblocker::skyblock::tabhud::config::other_options_screen::OtherOptionsScreen;
use crate::de::hysky::skyblocker::skyblock::tabhud::config::styled_button_widget::StyledButtonWidget;
use crate::de::hysky::skyblocker::skyblock::tabhud::config::toggle_button_widget::ToggleButtonWidget;
use crate::de::hysky::skyblocker::skyblock::tabhud::config::widgets_configuration_screen::WidgetsConfigurationScreen;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::widget_manager;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::widget_manager::ScreenLayer;
use crate::de::hysky::skyblocker::utils::location::Location;
use mcrust::net::minecraft::client::gui::components::abstract_container_widget::AbstractContainerWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::abstract_widget::PressLambda;
use mcrust::net::minecraft::client::gui::components::events::container_event_handler;
use mcrust::net::minecraft::client::gui::components::events::gui_event_listener::GuiEventListener;
use mcrust::net::minecraft::client::gui::components::tooltip::Tooltip;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::layouts::abstract_layout::AbstractLayout;
use mcrust::net::minecraft::client::gui::layouts::frame_layout;
use mcrust::net::minecraft::client::gui::layouts::layout;
use mcrust::net::minecraft::client::gui::layouts::layout_element::LayoutElement;
use mcrust::net::minecraft::client::gui::layouts::linear_layout::LinearLayout;
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::util::argb;
use crate::de::hysky::skyblocker::utils::render::gui::dropdown_widget::DropdownWidget as _;
use mcrust::net::minecraft::client::gui::components::renderable::Renderable as _;
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
    pub location_dropdown: Option<compat::Shared<CustomDropdownWidget<Location>>>,
    /// java: private final CustomDropdownWidget<WidgetManager.ScreenLayer> screenLayerDropdown;
    pub screen_layer_dropdown: Option<compat::Shared<CustomDropdownWidget<ScreenLayer>>>,
    /// java: private final Layout layout;
    pub layout: Layout,
    /// java: private final List<AbstractWidget> widgets;
    pub widgets: Vec<AbstractWidget>,
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
    pub fn new(mut width: i32, mut parent: compat::Shared<WidgetsConfigurationScreen>) -> Self {
        let __init_layout = Layout::new();
        let mut left_buttons: LinearLayout = LinearLayout::horizontal();
        let mut options_button: compat::Shared<StyledButtonWidget> = StyledButtonWidget::new(60, HEIGHT, component::translatable("skyblocker.config.hud.topBar.options"), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { WidgetsConfigurationScreen::open_popup(&parent, Arc::new(move |__a0| OtherOptionsScreen::new(__a0.clone()))); })));
        // TODO(fabric2seer): call to `defaultSettings(..)` (not a method of this class)
        //   java: super(0, 0, width, HEIGHT, Component.literal("hi"), defaultSettings(0));
        let mut this: Self = todo!("constructor");
        let __wa1 = Tooltip::create((component::translatable("skyblocker.config.hud.topBar.options.@Tooltip")).get_string());
        let __o1 = options_button.clone();
        compat::write(&__o1, |__t| __t.set_tooltip(__wa1.clone()));
        // TODO(fabric2seer): `addButton` on a void
        //   java: StyledButtonWidget helpButton = new StyledButtonWidget(60, HEIGHT, Component.literal("(?) ").append(Component.translatable("skyblocker.config.hud.topBar.help")),
        //   java: 				_ -> parent.openPopup(screen -> new PopupScreen.Builder(screen, Component.translatable("skyblocker.config.hud.help"))
        //   java: 						.addMessage(Component.translatable("skyblocker.config.hud.helpText"))
        //   java: 						.addButton(CommonComponents.GUI_OK, PopupScreen::onClose)
        //   java: 						.setWidth(300)
        //   java: 						.build()));
        let mut help_button: compat::Shared<StyledButtonWidget> = todo!("fabric2seer: the initialiser of `helpButton` (TODO above)");
        left_buttons.add_child(options_button.clone());
        left_buttons.add_child(help_button.clone());
        this.layout.add(LayoutElement::Grid(left_buttons.clone().wrapped));
        this.location_dropdown = Some(CustomDropdownWidget::new(((width.wrapping_div(2)).wrapping_sub(100)).wrapping_sub(5), 0, 100, 200, widget_manager::ALLOWED_LOCATIONS.iter().cloned().collect::<Vec<_>>(), Arc::new(move |__a0| { WidgetsConfigurationScreen::set_current_location(&parent, __a0.clone()); }), WidgetsConfigurationScreen::get_current_location(&parent)));
        { let (__a0,) = (Arc::new(move |l| component::literal(&compat::jstring(&l))),); this.location_dropdown.as_mut().unwrap().lock().borrow_mut().set_formatter(__a0) };
        this.screen_layer_dropdown = Some(CustomDropdownWidget::new((width.wrapping_div(2)).wrapping_add(5), 0, 100, 200, ScreenLayer::values().to_vec(), Arc::new(move |__a0| { WidgetsConfigurationScreen::set_current_screen_layer(&parent, __a0.clone()); }), ScreenLayer::Hud));
        let mut dropdowns_layout: LinearLayout = LinearLayout::horizontal().spacing(2);
        dropdowns_layout.add_child(this.location_dropdown.clone().unwrap());
        dropdowns_layout.add_child(this.screen_layer_dropdown.clone().unwrap());
        this.layout.add(LayoutElement::Grid(dropdowns_layout.clone().wrapped));
        let mut auto_anchor_toggle: compat::Shared<ToggleButtonWidget> = ToggleButtonWidget::new(100, HEIGHT, component::translatable("skyblocker.config.hud.topBar.autoAnchor"), Arc::new(move |b| { { let __v = b; compat::write(&parent, |__t| __t.auto_anchor = __v); } }));
        ToggleButtonWidget::set_state(&auto_anchor_toggle, true);
        let __wa2 = Tooltip::create((component::translatable("skyblocker.config.hud.topBar.autoAnchor.@Tooltip")).get_string());
        let __o2 = auto_anchor_toggle.clone();
        compat::write(&__o2, |__t| __t.set_tooltip(__wa2.clone()));
        let mut right_buttons: LinearLayout = LinearLayout::horizontal();
        // TODO(fabric2seer): a widget put in a layout outside a mod screen's own methods: the port's layouts hold a widget by its index in the screen's widget list
        // rust: right_buttons.add_child(auto_anchor_toggle.clone());
        this.layout.add(LayoutElement::Grid(right_buttons.clone().wrapped));
        this.widgets = vec![options_button.clone(), help_button.clone(), this.location_dropdown.clone().unwrap(), this.screen_layer_dropdown.clone().unwrap(), auto_anchor_toggle.clone()];
        this.layout.arrange_elements();
        this
    }

    /// java: static void drawButtonBorder(GuiGraphicsExtractor context, int x, int y, int y2)
    pub fn draw_button_border(context: &GuiGraphicsExtractor, mut x: i32, mut y: i32, mut y2: i32) {
        gui_graphics_extractor::vertical_line(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), x.wrapping_sub(1), y, y2, argb::color_alpha_rgb(15, -1));
        gui_graphics_extractor::vertical_line(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), x, y, y2, argb::color_alpha_rgb(100, 0));
        gui_graphics_extractor::vertical_line(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), x.wrapping_add(1), y, y2, argb::color_alpha_rgb(15, -1));
    }

    /// java: @Override public int getHeight()
    pub fn get_height(&self) -> i32 {
        let __o1 = self.location_dropdown.clone().unwrap();
        let __o2 = self.screen_layer_dropdown.clone().unwrap();
        return self.base.get_height().max((compat::write(&__o1, |__t| __t.get_height())).max(compat::write(&__o2, |__t| __t.get_height())));
    }

    /// java: @Override public List<? extends GuiEventListener> children()
    pub fn children(&self) -> Vec<Box<dyn GuiEventListener>> {
        return self.widgets.clone();
    }

    /// java: @Override public void setWidth(int width)
    pub fn set_width(&self, mut width: i32) {
        self.base.set_width(width);
        self.layout.arrange_elements();
    }

    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor context, int mouseX, int mouseY, float deltaTicks)
    pub fn extract_widget_render_state(&self, context: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut delta_ticks: f32) {
        gui_graphics_extractor::blit_sprite_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), TEXTURE.get_mut(), self.get_x().wrapping_sub(2), self.get_y().wrapping_sub(2), self.get_width().wrapping_add(4), HEIGHT.wrapping_add(2));
        for widget in self.widgets.iter() {
            // TODO(fabric2seer): `extract_render_state` is a method of the port's trait `Renderable`, which the port's `AbstractWidget` does not implement
            // rust: widget.extract_render_state(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mouse_x, mouse_y, delta_ticks);
        }
    }

    /// java: @Override public boolean mouseScrolled(double mouseX, double mouseY, double horizontalAmount, double verticalAmount)
    pub fn mouse_scrolled(&self, mut mouse_x: f64, mut mouse_y: f64, mut horizontal_amount: f64, mut vertical_amount: f64) -> bool {
        if !self.visible {
            return false;
        }
        // TODO(fabric2seer): `get_child_at` takes 3 argument(s) in the port (`children: &[(usize, ScreenRectangle)], x: f64, y: f64`), Java passes 2
        // TODO(fabric2seer): the condition: Optional.mouseScrolled
        //   java: this.getChildAt(..).filter(..).isPresent(..)
        if todo!("fabric2seer: the condition (TODO above)") {
            return true;
        }
        // TODO(fabric2seer): `mouse_scrolled` takes 2 argument(s) in the port (`content_height: i32, scroll_y: f64`), Java passes 4
        // TODO(fabric2seer): `ScrollArea::mouse_scrolled` ports `mouseScrolled`, but the port's `AbstractContainerWidget` holds no `ScrollArea` (no Deref, no `base` field) to call it on
        // rust: return self.base.mouse_scrolled(mouse_x, mouse_y, horizontal_amount, vertical_amount);
        return todo!("fabric2seer: see the TODO above");
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    pub fn update_widget_narration(&self, builder: &dyn NarrationElementOutput) {
    }

    /// java: @Override protected int contentHeight()
    pub fn content_height(&self) -> i32 {
        return 0;
    }

}

// NOTE(fabric2seer): children, contentHeight, extractWidgetRenderState, getHeight, mouseScrolled, setWidth, updateWidgetNarration override(s) of AbstractContainerWidget - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them

/// java: private class Layout extends AbstractLayout
pub struct Layout {
    // TODO(fabric2seer): extends AbstractLayout - a TRAIT in the port: implement it for Layout instead of holding a base
    /// java: private final List<LayoutElement> widgets = new ArrayList<>(4);
    pub widgets: Vec<LayoutElement>,
}

impl Layout {
    /// java: Layout()
    pub fn new() -> Self {
        let mut this = Self {
            widgets: Vec::with_capacity((4) as usize),
        };
        this
    }

    /// java: private void add(LayoutElement widget)
    pub fn add(&mut self, mut widget: LayoutElement) {
        self.widgets.push(widget);
    }

    /// java: @Override public void visitChildren(Consumer<LayoutElement> consumer)
    pub fn visit_children(&self, consumer: Arc<dyn Fn(LayoutElement)>) {
        // TODO(fabric2seer): List.forEach(..)
        //   java: widgets.forEach(consumer);
    }

    /// java: @Override public int getX()
    pub fn get_x(&self) -> i32 {
        // TODO(fabric2seer): `TopBarWidget.this` (the enclosing instance of an inner class)
        //   java: return TopBarWidget.this.getX();
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public int getY()
    pub fn get_y(&self) -> i32 {
        // TODO(fabric2seer): `TopBarWidget.this` (the enclosing instance of an inner class)
        //   java: return TopBarWidget.this.getY();
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public int getWidth()
    pub fn get_width(&self) -> i32 {
        // TODO(fabric2seer): `TopBarWidget.this` (the enclosing instance of an inner class)
        //   java: return TopBarWidget.this.getWidth();
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public int getHeight()
    pub fn get_height(&self) -> i32 {
        // TODO(fabric2seer): `TopBarWidget.this` (the enclosing instance of an inner class)
        //   java: return TopBarWidget.this.getHeight();
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public void arrangeElements()
    pub fn arrange_elements(&self) {
        // TODO(fabric2seer): `arrange_elements` takes 1 argument(s) in the port (`root: &mut LayoutElement`), Java passes 0
        // rust: layout::arrange_elements();
        let mut first: LayoutElement = self.widgets[0];
        first.set_position(self.get_x(), self.get_y());
        let mut low: i32 = first.get_x().wrapping_add(first.get_width());
        let mut last: LayoutElement = self.widgets[self.widgets.len() - 1];
        last.set_position((self.get_x().wrapping_add(self.get_width())).wrapping_sub(last.get_width()), self.get_y());
        let mut high: i32 = last.get_x();
        for i in 1..(self.widgets.len() as i32).wrapping_sub(1) {
            let mut widget: LayoutElement = self.widgets[(i) as usize];
            widget.set_y(self.get_y());
            // TODO(fabric2seer): `align_in_dimension` takes 4 argument(s) in the port (`pos: i32, length: i32, widget_length: i32, align: f32`), Java passes 5
            // TODO(fabric2seer): a lambda for a parameter the port does not type as a closure (`Consumer<Integer>`)
            // rust: frame_layout::align_in_dimension(0, self.get_width(), widget.get_width(), |__a0| { widget.set_x(__a0); }, 0.5f32);
            if widget.get_x().wrapping_add(widget.get_width()) > high {
                // TODO(fabric2seer): `align_in_dimension` takes 4 argument(s) in the port (`pos: i32, length: i32, widget_length: i32, align: f32`), Java passes 5
                // TODO(fabric2seer): a lambda for a parameter the port does not type as a closure (`Consumer<Integer>`)
                // rust: frame_layout::align_in_dimension(low, high.wrapping_sub(low), widget.get_width(), |__a0| { widget.set_x(__a0); }, 0.5f32);
            }
        }
    }

    /// java: @Override public void removeChildren()
    pub fn remove_children(&mut self) {
        self.widgets.clear();
    }

}

// NOTE(fabric2seer): arrangeElements, getHeight, getWidth, getX, getY, removeChildren, visitChildren override(s) of AbstractLayout - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them
