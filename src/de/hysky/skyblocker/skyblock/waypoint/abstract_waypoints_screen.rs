//! Ported from de.hysky.skyblocker.skyblock.waypoint.AbstractWaypointsScreen by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::waypoint::waypoints_list_widget::WaypointsListWidget;
use crate::de::hysky::skyblocker::utils::location::Location;
use crate::de::hysky::skyblocker::utils::render::gui::dropdown_widget::DropdownWidget;
use crate::de::hysky::skyblocker::utils::render::gui::dropdown_widget::DropdownWidgetBase;
use crate::de::hysky::skyblocker::utils::render::gui_helper::GuiHelper;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use crate::de::hysky::skyblocker::utils::waypoint::named_waypoint::NamedWaypoint;
use crate::de::hysky::skyblocker::utils::waypoint::waypoint::Waypoint;
use crate::de::hysky::skyblocker::utils::waypoint::waypoint_group::WaypointGroup;
use mcrust::net::minecraft::client::gui::components::abstract_container_widget::AbstractContainerWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::events::gui_event_listener::GuiEventListener;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::layouts::frame_layout;
use mcrust::net::minecraft::client::gui::layouts::header_and_footer_layout::HeaderAndFooterLayout;
use mcrust::net::minecraft::client::gui::layouts::layout_element::LayoutElement;
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::util::argb;
use mcrust::net::minecraft::client::gui::components::renderable::Renderable as _;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final int BORDER_SIZE = 4;
pub const POPUP_CONTAINER_BORDER_SIZE: i32 = 4;

/// java: public abstract class AbstractWaypointsScreen<T extends Screen> extends Screen
pub struct AbstractWaypointsScreenBase<T> {
    /// java: extends net.minecraft.client.gui.screens.Screen
    pub base: mcrust::net::minecraft::client::gui::screens::screen::Screen,
    /// java: protected final T parent;
    pub parent: Option<T>,
    /// java: protected final Multimap<Location, WaypointGroup> waypoints;
    pub waypoints: Option<crate::unresolved::Multimap>,
    /// java: protected Location island;
    pub island: Option<Location>,
    /// java: protected WaypointsListWidget waypointsListWidget;
    pub waypoints_list_widget: Option<compat::WidgetRef>,
    /// java: protected DropdownWidget<Location> islandWidget;
    pub island_widget: Option<compat::Shared<dyn DropdownWidget<Location>>>,
    /// java: protected final HeaderAndFooterLayout layout = new HeaderAndFooterLayout(this);
    pub layout: Option<HeaderAndFooterLayout>,
    /// java: private final PopupContainer popupContainer = new PopupContainer();
    pub popup_container: PopupContainer,
    /// the screen's widgets as this object last saw them (see `compat::WidgetCache`)
    pub __widgets: compat::WidgetCache,
}

impl<T: 'static> std::ops::Deref for AbstractWaypointsScreenBase<T> {
    type Target = mcrust::net::minecraft::client::gui::screens::screen::Screen;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<T: 'static> std::ops::DerefMut for AbstractWaypointsScreenBase<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl<T: Clone + 'static> AbstractWaypointsScreenBase<T> {
    /// java: public AbstractWaypointsScreen(Component title, T parent)
    pub fn new(mut title: MutableComponent, mut parent: T) -> Self {
        // TODO(fabric2seer): static call MultimapBuilder.enumKeys(..)
        //   java: this(title, parent, MultimapBuilder.enumKeys(Location.class).arrayListValues().build());
        let mut this: Self = todo!("constructor");
        this
    }

    /// java: public AbstractWaypointsScreen(Component title, T parent, Multimap<Location, WaypointGroup> waypoints)
    pub fn new_3(mut title: MutableComponent, mut parent: T, mut waypoints: crate::unresolved::Multimap) -> Self {
        let mut this = Self::new_4(title.clone(), parent.clone(), waypoints.clone(), Utils::get_location());
        this
    }

    /// java: public AbstractWaypointsScreen(Component title, T parent, Multimap<Location, WaypointGroup> waypoints, Location island)
    pub fn new_4(mut title: MutableComponent, mut parent: T, mut waypoints: crate::unresolved::Multimap, mut island: Location) -> Self {
        let __init_parent = Some(parent.clone());
        let __init_waypoints = Some(waypoints.clone());
        let __init_island = Some(island);
        let mut this = Self {
            base: mcrust::net::minecraft::client::gui::screens::screen::Screen::new(title.get_string()),
            __widgets: Default::default(),
            parent: __init_parent,
            waypoints: __init_waypoints,
            island: __init_island,
            waypoints_list_widget: None,
            island_widget: None,
            layout: None,
            popup_container: PopupContainer::new(),
        };
        this.layout = Some(HeaderAndFooterLayout::new(0, 0));
        this.layout.as_mut().unwrap().set_header_height(32);
        this
    }

}

/// java: the virtual methods of AbstractWaypointsScreen (implemented by AbstractWaypointsScreenBase and every subclass)
pub trait AbstractWaypointsScreen<T> {
    fn abstract_waypoints_screen_base(&self) -> &AbstractWaypointsScreenBase<T>;
    fn abstract_waypoints_screen_base_mut(&mut self) -> &mut AbstractWaypointsScreenBase<T>;
    fn as_any(&self) -> &dyn std::any::Any;

    /// java: @Override protected void init()
    fn init(&mut self) {
        ();
        // TODO(fabric2seer): a widget put in a layout outside a mod screen's own methods: the port's layouts hold a widget by its index in the screen's widget list
        // rust: { let __s0 = compat::string_widget(0, 0, -1, 9, self.abstract_waypoints_screen_base().title.clone()); self.abstract_waypoints_screen_base_mut().layout.as_mut().unwrap().add_to_header(__s0) };
        { let __s0 = self.abstract_waypoints_screen_base().popup_container.clone(); self.abstract_waypoints_screen_base().add_widget(__s0) };
        // TODO(fabric2seer): `this` handed on as a reference other objects keep: the object has no shared handle to itself (port by hand)
        //   java: waypointsListWidget = addRenderableWidget(new WaypointsListWidget(minecraft, this, width, height - 120, 32, 24));
    }

    /// java: protected final void lateInit()
    fn late_init(&mut self) {
        // TODO(fabric2seer): `visit_widgets` takes 0 argument(s) in the port (``), Java passes 1
        // TODO(fabric2seer): a lambda for a parameter the port does not type as a closure (`Consumer<net.minecraft.client.gui.components.AbstractWidget>`)
        // rust: self.abstract_waypoints_screen_base().layout.as_ref().unwrap().visit_widgets(|__a0| { self.abstract_waypoints_screen_base().add_renderable_widget(__a0.clone()); });
        self.abstract_waypoints_screen_base_mut().island_widget = Some({ let __s0 = (compat::shared(DropdownWidgetBase::new_9(compat::minecraft(), (self.abstract_waypoints_screen_base().width).wrapping_sub(160), 8, 150, (self.abstract_waypoints_screen_base().height).wrapping_sub(8), Location::values().to_vec(), Arc::new(move |__a0| { self.island_changed(__a0.clone()); }), self.abstract_waypoints_screen_base().island.clone().unwrap(), Arc::new(move |_| {

        }))) as compat::Shared<dyn DropdownWidget<compat::Object>>); self.abstract_waypoints_screen_base().add_renderable_widget(__s0) });
        // TODO(fabric2seer): `Screen.addRenderableOnly` has no Rust method in the port (looked for `add_renderable_only` in Screen and its supertypes)
        // rust: self.abstract_waypoints_screen_base().add_renderable_only(&self.abstract_waypoints_screen_base().popup_container);
        self.reposition_elements();
    }

    /// java: protected void setPopup(LayoutElement w, int x, int y)
    fn set_popup(&mut self, w: &LayoutElement, mut x: i32, mut y: i32) {
        if false /* java: w == null: never null here */ {
            self.abstract_waypoints_screen_base_mut().popup_container.visible = false;
        } else {
            self.abstract_waypoints_screen_base_mut().popup_container.visible = true;
            self.abstract_waypoints_screen_base_mut().popup_container.set_widget(*w);
            { let __s0 = x.clamp(0, (self.abstract_waypoints_screen_base().width).wrapping_sub(self.abstract_waypoints_screen_base().popup_container.get_width())); let __s1 = y.clamp(0, (self.abstract_waypoints_screen_base().height).wrapping_sub(self.abstract_waypoints_screen_base().popup_container.get_height())); self.abstract_waypoints_screen_base_mut().popup_container.set_position(__s0, __s1) };
        }
    }

    /// java: @Override protected void repositionElements()
    fn reposition_elements(&mut self) {
        self.abstract_waypoints_screen_base_mut().layout.as_mut().unwrap().arrange_elements();
        let __wa1 = self.abstract_waypoints_screen_base().width;
        let __wa2 = self.abstract_waypoints_screen_base().layout.clone();
        let __o1 = self.abstract_waypoints_screen_base().waypoints_list_widget.clone().unwrap();
        compat::write(&__o1, |__t| __t.update_size(__wa1, __wa2.as_ref().unwrap()));
        WaypointsListWidget::update_entries(self.abstract_waypoints_screen_base().waypoints_list_widget.as_ref().unwrap());
        let __o2 = self.abstract_waypoints_screen_base().island_widget.clone().unwrap();
        { let __s0 = ((self.abstract_waypoints_screen_base().width).wrapping_sub(compat::write(&__o2, |__t| __t.dropdown_widget_base_mut().get_width()))).wrapping_sub(10); self.abstract_waypoints_screen_base_mut().island_widget.as_mut().unwrap().lock().borrow_mut().set_x(__s0) };
        // TODO(fabric2seer): `align_in_dimension` takes 4 argument(s) in the port (`pos: i32, length: i32, widget_length: i32, align: f32`), Java passes 5
        // TODO(fabric2seer): a lambda for a parameter the port does not type as a closure (`Consumer<Integer>`)
        // rust: frame_layout::align_in_dimension(0, self.abstract_waypoints_screen_base().layout.as_ref().unwrap().get_header_height(), self.abstract_waypoints_screen_base().island_widget.as_ref().unwrap().lock().borrow().get_header_height(), |__a0| { self.abstract_waypoints_screen_base_mut().island_widget.as_mut().unwrap().lock().borrow_mut().set_y(__a0); }, 0.5f32);
        let __o3 = self.abstract_waypoints_screen_base().island_widget.clone().unwrap();
        { let __s0 = (((self.abstract_waypoints_screen_base().height).wrapping_sub(compat::write(&__o3, |__t| __t.dropdown_widget_base_mut().get_y()))).wrapping_sub(8)).max(20); self.abstract_waypoints_screen_base_mut().island_widget.as_mut().unwrap().lock().borrow_mut().set_max_height(__s0) };
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    fn mouse_clicked(&mut self, click: &MouseButtonEvent, mut doubled: bool) -> bool {
        if self.abstract_waypoints_screen_base_mut().island_widget.as_mut().unwrap().lock().borrow_mut().mouse_clicked(click, doubled) {
            return true;
        }
        let mut mouse_clicked: bool = false;
        self.update_buttons();
        return mouse_clicked;
    }

    /// java: @Override public boolean mouseScrolled(double mouseX, double mouseY, double horizontalAmount, double verticalAmount)
    fn mouse_scrolled(&self, mut mouse_x: f64, mut mouse_y: f64, mut horizontal_amount: f64, mut vertical_amount: f64) -> bool {
        let __wa1 = mouse_x;
        let __wa2 = mouse_y;
        let __o1 = self.abstract_waypoints_screen_base().island_widget.clone().unwrap();
        let __wa3 = mouse_x;
        let __wa4 = mouse_y;
        let __o2 = self.abstract_waypoints_screen_base().island_widget.clone().unwrap();
        if compat::write(&__o2, |__t| __t.dropdown_widget_base_mut().is_mouse_over(__wa3, __wa4)) && self.abstract_waypoints_screen_base().island_widget.as_ref().unwrap().lock().borrow().mouse_scrolled(mouse_x, mouse_y, horizontal_amount, vertical_amount) {
            return true;
        }
        return false;
    }

    /// java: protected void islandChanged(Location location)
    fn island_changed(&mut self, mut location: Location) {
        self.abstract_waypoints_screen_base_mut().island = Some(location);
        WaypointsListWidget::set_island(self.abstract_waypoints_screen_base().waypoints_list_widget.as_ref().unwrap(), self.abstract_waypoints_screen_base().island.clone().unwrap());
    }

    /// java: protected boolean isEnabled(NamedWaypoint waypoint)
    fn is_enabled(&self, waypoint: &compat::Shared<dyn NamedWaypoint>) -> bool {
        return ({ let __f = compat::dispatch(waypoint, |__t| __t.is_enabled()); __f(&compat::view(waypoint, |a| a as compat::Shared<dyn Waypoint>)) });
    }

    /// java: protected void enabledChanged(NamedWaypoint waypoint, boolean enabled)
    fn enabled_changed(&mut self, waypoint: &compat::Shared<dyn NamedWaypoint>, mut enabled: bool) {
        ({ let __f = compat::dispatch(waypoint, |__t| __t.set_enabled()); __f(&compat::view(waypoint, |a| a as compat::Shared<dyn Waypoint>), enabled) });
    }

    /// java: protected void updateButtons()
    fn update_buttons(&self) {
        WaypointsListWidget::update_buttons(self.abstract_waypoints_screen_base().waypoints_list_widget.as_ref().unwrap());
    }

}

/// java: private static class PopupContainer extends AbstractContainerWidget
pub struct PopupContainer {
    /// java: extends net.minecraft.client.gui.components.AbstractContainerWidget
    pub base: AbstractContainerWidget,
    /// java: private LayoutElement widget = SpacerElement.width(0);
    pub widget: LayoutElement,
    /// java: private final List<AbstractWidget> children = new ArrayList<>(4);
    pub children: Vec<AbstractWidget>,
    /// java: private final AbstractButton closeButton;
    pub close_button: AbstractWidget,
}

impl std::ops::Deref for PopupContainer {
    type Target = AbstractContainerWidget;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for PopupContainer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl PopupContainer {
    /// java: PopupContainer()
    pub fn new() -> Self {
        // TODO(fabric2seer): the port's `new` ports this Java method with other parameters (enabled, disabled, enabled_focused, disabled_focused for Java's sprite, focused)
        //   java: this.closeButton = new ImageButton(14, 14, new WidgetSprites(
        //   java: 					Identifier.withDefaultNamespace("widget/cross_button"), Identifier.withDefaultNamespace("widget/cross_button_highlighted")),
        //   java: 					_ -> visible = false,
        //   java: 					Component.empty()
        //   java: 					);
        // TODO(fabric2seer): static `AbstractScrollArea.defaultSettings` has no Rust counterpart in the port
        //   java: super(0, 0, 0, 0, Component.empty(), AbstractScrollArea.defaultSettings(8));
        let mut this: Self = todo!("constructor");
        this
    }

    /// java: @Override public int getWidth()
    pub fn get_width(&self) -> i32 {
        return self.widget.get_width().wrapping_add(POPUP_CONTAINER_BORDER_SIZE.wrapping_mul(2));
    }

    /// java: @Override public int getHeight()
    pub fn get_height(&self) -> i32 {
        return ((self.widget.get_height().wrapping_add(POPUP_CONTAINER_BORDER_SIZE.wrapping_mul(2))).wrapping_add(self.close_button.get_height())).wrapping_add(2);
    }

    /// java: @Override public void setX(int x)
    pub fn set_x(&mut self, mut x: i32) {
        self.base.set_x(x);
        { let __s0 = (self.get_right().wrapping_sub(POPUP_CONTAINER_BORDER_SIZE)).wrapping_sub(self.close_button.get_width()); self.close_button.set_x(__s0) };
        { let __s0 = self.get_x().wrapping_add(POPUP_CONTAINER_BORDER_SIZE); self.widget.set_x(__s0) };
    }

    /// java: @Override public void setY(int y)
    pub fn set_y(&mut self, mut y: i32) {
        self.base.set_y(y);
        { let __s0 = self.get_y().wrapping_add(POPUP_CONTAINER_BORDER_SIZE); self.close_button.set_y(__s0) };
        { let __s0 = self.close_button.get_bottom().wrapping_add(2); self.widget.set_y(__s0) };
    }

    /// java: @Override public List<? extends GuiEventListener> children()
    pub fn children(&self) -> Vec<Box<dyn GuiEventListener>> {
        return (if self.visible { self.children.clone() } else { vec![] });
    }

    /// java: public void setWidget(LayoutElement widget)
    pub fn set_widget(&mut self, mut widget: LayoutElement) {
        self.widget = widget;
        self.children.clear();
        self.children.push(self.close_button.clone());
        // TODO(fabric2seer): a lambda for a parameter the port does not type as a closure (`Consumer<net.minecraft.client.gui.components.AbstractWidget>`)
        // rust: widget.visit_widgets(|__a0| { self.children.push(__a0.clone()); });
        { let __s0 = (self.get_right().wrapping_sub(POPUP_CONTAINER_BORDER_SIZE)).wrapping_sub(self.close_button.get_width()); self.close_button.set_x(__s0) };
    }

    /// java: @Override protected int contentHeight()
    pub fn content_height(&self) -> i32 {
        return 0;
    }

    /// java: @Override protected double scrollRate()
    pub fn scroll_rate(&self) -> f64 {
        return ((0) as f64) as f64;
    }

    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor context, int mouseX, int mouseY, float deltaTicks)
    pub fn extract_widget_render_state(&self, context: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut delta_ticks: f32) {
        if self.is_hovered() {
            // TODO(fabric2seer): `GuiGraphicsExtractor.requestCursor` has no Rust method in the port (looked for `request_cursor` in GuiGraphicsExtractor and its supertypes)
            // rust: context.request_cursor(&mcrust::com::mojang::blaze3d::platform::cursor::cursor_type::CursorType::DEFAULT);
        }
        context.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), self.get_x(), self.get_y(), self.get_right(), self.get_bottom(), argb::color_alpha_f32_rgb(0.6f32, 0));
        GuiHelper::border(context, self.get_x(), self.get_y(), self.get_width(), self.get_height(), mcrust::net::minecraft::util::common_colors::WHITE);
        for child in self.children.iter() {
            // TODO(fabric2seer): `extract_render_state` is a method of the port's trait `Renderable`, which the port's `AbstractWidget` does not implement
            // rust: child.extract_render_state(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mouse_x, mouse_y, delta_ticks);
        }
        if !self.is_focused() && mouse_x <= self.get_x().wrapping_sub(50) || mouse_x >= self.get_right().wrapping_add(50) || mouse_y <= self.get_y().wrapping_sub(50) || mouse_y >= self.get_bottom().wrapping_add(50) {
            self.visible = false;
        }
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    pub fn update_widget_narration(&self, builder: &dyn NarrationElementOutput) {
    }

}

// NOTE(fabric2seer): children, contentHeight, extractWidgetRenderState, getHeight, getWidth, scrollRate, setX, setY, updateWidgetNarration override(s) of AbstractContainerWidget - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them
