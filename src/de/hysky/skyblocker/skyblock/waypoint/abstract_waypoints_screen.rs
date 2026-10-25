//! Ported from de.hysky.skyblocker.skyblock.waypoint.AbstractWaypointsScreen by fabric2seer.
//!
//! The screen object is reached by its handle (`compat::Shared<dyn AbstractWaypointsScreen>`) from every method: the
//! list it creates, the list's rows and the island dropdown all call back into it (`isEnabled`, `enabledChanged`,
//! `setPopup`, `islandChanged`) while the screen's own code is running, which a `&mut self` method would hold borrowed.
//! Java's protected methods are therefore the associated functions of `dyn AbstractWaypointsScreen` below, each taking
//! the handle and borrowing the object one statement at a time; the two a subclass overrides (`isEnabled`,
//! `enabledChanged`) are dispatch slots of the trait.
//!
//! `T parent` is not kept by type: the game's screen stack holds the screen this one was opened over
//! (`set_screen_keeping_parent`) and hands it back where Java's `setScreen(parent)` runs.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::waypoint::waypoints_list_widget::WaypointsListWidget;
use crate::de::hysky::skyblocker::skyblock::waypoint::waypoints::LocationMultimap;
use crate::de::hysky::skyblocker::utils::location::Location;
use crate::de::hysky::skyblocker::utils::render::gui::dropdown_widget::DropdownWidget;
use crate::de::hysky::skyblocker::utils::render::gui::dropdown_widget::DropdownWidgetBase;
use crate::de::hysky::skyblocker::utils::render::gui_helper::GuiHelper;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use crate::de::hysky::skyblocker::utils::waypoint::named_waypoint::NamedWaypoint;
use crate::de::hysky::skyblocker::utils::waypoint::waypoint::Waypoint;
use mcrust::mcrust::modding::screens::ScreenRenderContext;
use mcrust::mcrust::modding::widgets::ModSelectionList;
use mcrust::mcrust::modding::widgets::WidgetRenderContext;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::abstract_widget::PressLambda;
use mcrust::net::minecraft::client::gui::components::abstract_widget::WidgetKind;
use mcrust::net::minecraft::client::gui::components::widget_sprites::WidgetSprites;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::layouts::frame_layout;
use mcrust::net::minecraft::client::gui::layouts::header_and_footer_layout::HeaderAndFooterLayout;
use mcrust::net::minecraft::client::gui::layouts::layout_element::LayoutElement;
use mcrust::net::minecraft::client::gui::screens::screen::Screen;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::client::input::mouse_button_info::MouseButtonInfo;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::util::argb;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final int BORDER_SIZE = 4; (PopupContainer)
pub const POPUP_CONTAINER_BORDER_SIZE: i32 = 4;

/// java: public abstract class AbstractWaypointsScreen<T extends Screen> extends Screen
pub struct AbstractWaypointsScreenBase {
    /// java: extends net.minecraft.client.gui.screens.Screen
    pub base: Screen,
    /// java: protected final T parent; - the screen this one was opened over. The game's screen stack holds it
    /// (`set_screen_keeping_parent`); the object is kept here as Java keeps it, for what reads it by type.
    pub parent: Option<std::rc::Rc<dyn std::any::Any>>,
    /// java: protected final Multimap<Location, WaypointGroup> waypoints;
    pub waypoints: Option<compat::Shared<LocationMultimap>>,
    /// java: protected Location island;
    pub island: Option<Location>,
    /// java: protected WaypointsListWidget waypointsListWidget;
    pub waypoints_list_widget: Option<compat::WidgetRef>,
    /// java: protected DropdownWidget<Location> islandWidget;
    /// (`addRenderableWidget`: the dropdown is a mod container widget the game's widget list cannot hold; the screen
    /// draws it after its other widgets and hands it its clicks and scrolls first, as Java's overrides do)
    pub island_widget: Option<compat::Shared<dyn DropdownWidget<Location>>>,
    /// java: protected final HeaderAndFooterLayout layout = new HeaderAndFooterLayout(this);
    pub layout: Option<HeaderAndFooterLayout>,
    /// java: private final PopupContainer popupContainer = new PopupContainer();
    pub popup_container: PopupContainer,
    /// the screen's widgets as this object last saw them (see `compat::WidgetCache`)
    pub __widgets: compat::WidgetCache,
    /// `updateButtons()` after `super.mouseClicked(..)`: the game's widget sweep runs after this screen's
    /// `mouseClicked` answers, so the call is made at the screen's next tick (see `mouse_clicked`)
    pub update_buttons_requested: bool,
}

impl std::ops::Deref for AbstractWaypointsScreenBase {
    type Target = Screen;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for AbstractWaypointsScreenBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl AbstractWaypointsScreenBase {
    /// java: public AbstractWaypointsScreen(Component title, T parent)
    pub fn new(title: MutableComponent, parent: Option<std::rc::Rc<dyn std::any::Any>>) -> Self {
        // java: this(title, parent, MultimapBuilder.enumKeys(Location.class).arrayListValues().build());
        Self::new_3(title, parent, compat::shared(LocationMultimap::new()))
    }

    /// java: public AbstractWaypointsScreen(Component title, T parent, Multimap<Location, WaypointGroup> waypoints)
    pub fn new_3(title: MutableComponent, parent: Option<std::rc::Rc<dyn std::any::Any>>, waypoints: compat::Shared<LocationMultimap>) -> Self {
        // java: this(title, parent, waypoints, Utils.getLocation());
        Self::new_4(title, parent, waypoints, Utils::get_location())
    }

    /// java: public AbstractWaypointsScreen(Component title, T parent, Multimap<Location, WaypointGroup> waypoints, Location island)
    pub fn new_4(title: MutableComponent, parent: Option<std::rc::Rc<dyn std::any::Any>>, waypoints: compat::Shared<LocationMultimap>, island: Location) -> Self {
        // java: super(title);
        let mut this = Self {
            base: Screen::new(title.get_string()),
            parent,
            waypoints: Some(waypoints),
            island: Some(island),
            waypoints_list_widget: None,
            island_widget: None,
            // java: protected final HeaderAndFooterLayout layout = new HeaderAndFooterLayout(this); - the screen's size is
            // the layout's at each `arrangeElements` (`repositionElements` hands it in)
            layout: Some(HeaderAndFooterLayout::new(0, 0)),
            popup_container: PopupContainer::new(),
            __widgets: Default::default(),
            update_buttons_requested: false,
        };
        // java: this.layout.setHeaderHeight(32);
        this.layout.as_mut().unwrap().set_header_height(32);
        this
    }
}

/// java: the virtual methods of AbstractWaypointsScreen (implemented by every subclass)
pub trait AbstractWaypointsScreen: compat::DynAny {
    fn abstract_waypoints_screen_base(&self) -> &AbstractWaypointsScreenBase;
    fn abstract_waypoints_screen_base_mut(&mut self) -> &mut AbstractWaypointsScreenBase;
    fn as_any(&self) -> &dyn std::any::Any;

    /// java: protected boolean isEnabled(NamedWaypoint waypoint) (dispatch: the body for this class)
    fn is_enabled(&self) -> fn(&compat::Shared<dyn AbstractWaypointsScreen>, &compat::Shared<dyn NamedWaypoint>) -> bool {
        <dyn AbstractWaypointsScreen>::is_enabled_body
    }

    /// java: protected void enabledChanged(NamedWaypoint waypoint, boolean enabled) (dispatch: the body for this class)
    fn enabled_changed(&self) -> fn(&compat::Shared<dyn AbstractWaypointsScreen>, &compat::Shared<dyn NamedWaypoint>, bool) {
        <dyn AbstractWaypointsScreen>::enabled_changed_body
    }
}

impl dyn AbstractWaypointsScreen {
    /// `screen.isEnabled(waypoint)` - the object's own class's body.
    pub fn is_enabled_of(__this: &compat::Shared<dyn AbstractWaypointsScreen>, waypoint: &compat::Shared<dyn NamedWaypoint>) -> bool {
        let __f = compat::dispatch(__this, |__t| __t.is_enabled());
        __f(__this, waypoint)
    }

    /// `screen.enabledChanged(waypoint, enabled)` - the object's own class's body.
    pub fn enabled_changed_of(__this: &compat::Shared<dyn AbstractWaypointsScreen>, waypoint: &compat::Shared<dyn NamedWaypoint>, enabled: bool) {
        let __f = compat::dispatch(__this, |__t| __t.enabled_changed());
        __f(__this, waypoint, enabled)
    }

    /// java: protected boolean isEnabled(NamedWaypoint waypoint) - `return waypoint.isEnabled();`
    pub fn is_enabled_body(__this: &compat::Shared<dyn AbstractWaypointsScreen>, waypoint: &compat::Shared<dyn NamedWaypoint>) -> bool {
        let _ = __this;
        return ({ let __f = compat::dispatch(waypoint, |__t| __t.is_enabled()); __f(&compat::view(waypoint, |a| a as compat::Shared<dyn Waypoint>)) });
    }

    /// java: protected void enabledChanged(NamedWaypoint waypoint, boolean enabled) - `waypoint.setEnabled(enabled);`
    pub fn enabled_changed_body(__this: &compat::Shared<dyn AbstractWaypointsScreen>, waypoint: &compat::Shared<dyn NamedWaypoint>, enabled: bool) {
        let _ = __this;
        ({ let __f = compat::dispatch(waypoint, |__t| __t.set_enabled()); __f(&compat::view(waypoint, |a| a as compat::Shared<dyn Waypoint>), enabled) });
    }

    /// `waypointsListWidget.x(..)`: the list object with the live list (the game widget's) lent to it for `f`. The
    /// live list is taken out of its widget for the call (the screen object is not held borrowed meanwhile: the list's
    /// rows call back into this screen) and put back after.
    pub fn with_list<R>(__this: &compat::Shared<dyn AbstractWaypointsScreen>, f: impl FnOnce(&compat::Shared<WaypointsListWidget>) -> R) -> R {
        let (handle, mut live) = compat::write(__this, |__t| {
            let __b = __t.abstract_waypoints_screen_base_mut();
            let __r = __b.waypoints_list_widget.as_mut().expect("NullPointerException: waypointsListWidget");
            __b.__widgets.with(&mut __b.base, __r, |__w| {
                __w.mod_widget()
                    .and_then(|__ms| {
                        __ms.with::<ModSelectionList, _>(|__l| {
                            let __h = __l.class::<compat::ListClass<WaypointsListWidget>>().map(|__c| __c.object().clone()).expect("a mod list without its class");
                            let __kind = __l.kind;
                            (__h, std::mem::replace(__l, ModSelectionList::new(__kind, 0, 0, 0, 0)))
                        })
                    })
                    .expect("a mod list reached from inside its own method")
            })
        });
        let result = compat::lend_list(&handle, &mut live, || f(&handle));
        compat::write(__this, |__t| {
            let __b = __t.abstract_waypoints_screen_base_mut();
            let __r = __b.waypoints_list_widget.as_mut().expect("NullPointerException: waypointsListWidget");
            __b.__widgets.with(&mut __b.base, __r, |__w| {
                if let Some(__ms) = __w.mod_widget() {
                    __ms.with::<ModSelectionList, _>(|__l| *__l = live);
                }
            })
        });
        result
    }

    /// java: @Override protected void init() - `AbstractWaypointsScreen.init()`'s body, which a subclass's `init` runs as
    /// `super.init()`
    pub fn init_super(__this: &compat::Shared<dyn AbstractWaypointsScreen>) {
        // java: super.init(); - `Screen.init()` is empty
        // java: layout.addToHeader(new StringWidget(title, font)); - the widget joins the screen's list, the layout holds
        // its index
        compat::write(__this, |__t| {
            let __base = __t.abstract_waypoints_screen_base_mut();
            let __w = compat::string_widget(0, 0, -1, 9, __base.base.title.clone());
            let (__ww, __wh) = (__w.get_width(), __w.get_height());
            let __i = __base.base.add_widget(__w);
            __base.layout.as_mut().unwrap().add_to_header(LayoutElement::widget(__i, __ww, __wh));
        });
        // java: addWidget(popupContainer); - the container takes the clicks first while it is shown (`mouse_clicked`)
        // java: waypointsListWidget = addRenderableWidget(new WaypointsListWidget(minecraft, this, width, height - 120, 32, 24));
        let (width, height, island, waypoints) = compat::read(__this, |__t| {
            let __b = __t.abstract_waypoints_screen_base();
            (__b.base.width, __b.base.height, __b.island.unwrap(), __b.waypoints.clone().unwrap())
        });
        let list = WaypointsListWidget::new(compat::minecraft(), __this.clone(), island, waypoints, width, height.wrapping_sub(120), 32, 24);
        let widget = compat::attach_list(list);
        compat::write(__this, |__t| {
            let __b = __t.abstract_waypoints_screen_base_mut();
            let mut __r = compat::WidgetRef::value(widget);
            __b.__widgets.add(&mut __b.base, &mut __r, true);
            __b.waypoints_list_widget = Some(__r);
        });
    }

    /// java: protected final void lateInit() - this should be called at the end of the implementation's init to
    /// ensure that these elements render last.
    pub fn late_init(__this: &compat::Shared<dyn AbstractWaypointsScreen>) {
        // java: layout.visitWidgets(this::addRenderableWidget); - the layout's widgets are in the screen's list already;
        // each is made renderable
        compat::write(__this, |__t| {
            let __base = __t.abstract_waypoints_screen_base_mut();
            let __vw = __base.layout.as_ref().unwrap().visit_widgets();
            for (__i, _, _) in __vw {
                __base.base.mark_renderable(__i);
            }
        });
        // Not using layout due to dynamic height.
        // java: islandWidget = addRenderableWidget(new DropdownWidget<>(minecraft, width - 160, 8, 150, height - 8,
        //   Arrays.asList(Location.values()), this::islandChanged, island, _ -> {}));
        let (__width, __height, __island) = compat::read(__this, |__t| {
            let __b = __t.abstract_waypoints_screen_base();
            (__b.base.width, __b.base.height, __b.island.unwrap())
        });
        let __this_w = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        let __island_changed: Arc<dyn Fn(Location)> = Arc::new(move |location: Location| {
            if let Some(__this) = __this_w.get().upgrade() {
                <dyn AbstractWaypointsScreen>::island_changed(&__this, location);
            }
        });
        let __island_widget = compat::shared(DropdownWidgetBase::new_9(compat::minecraft(), __width.wrapping_sub(160), 8, 150, __height.wrapping_sub(8), Location::values().to_vec(), __island_changed, __island, Arc::new(move |_: bool| {}))) as compat::Shared<dyn DropdownWidget<Location>>;
        compat::write(__this, |__t| __t.abstract_waypoints_screen_base_mut().island_widget = Some(__island_widget));
        // java: addRenderableOnly(popupContainer); - drawn last (`extract_render_state`)
        <dyn AbstractWaypointsScreen>::reposition_elements(__this);
    }

    /// java: protected void setPopup(LayoutElement w, int x, int y) - the popup's content (Java passes a widget; `None` is
    /// Java's `null`)
    pub fn set_popup(__this: &compat::Shared<dyn AbstractWaypointsScreen>, w: Option<AbstractWidget>, x: i32, y: i32) {
        compat::write(__this, |__t| {
            let __b = __t.abstract_waypoints_screen_base_mut();
            let (width, height) = (__b.base.width, __b.base.height);
            match w {
                // java: if (w == null) popupContainer.visible = false;
                None => __b.popup_container.base.visible = false,
                Some(w) => {
                    // java: popupContainer.visible = true; popupContainer.setWidget(w);
                    __b.popup_container.base.visible = true;
                    __b.popup_container.set_widget(w);
                    // java: popupContainer.setPosition(Math.clamp(x, 0, width - popupContainer.getWidth()),
                    //   Math.clamp(y, 0, height - popupContainer.getHeight()));
                    // (`Math.clamp` throws when the bounds cross, as `i32::clamp` panics)
                    let px = x.clamp(0, width.wrapping_sub(__b.popup_container.get_width()));
                    let py = y.clamp(0, height.wrapping_sub(__b.popup_container.get_height()));
                    __b.popup_container.set_position(px, py);
                }
            }
        });
    }

    /// java: @Override protected void repositionElements()
    pub fn reposition_elements(__this: &compat::Shared<dyn AbstractWaypointsScreen>) {
        // java: layout.arrangeElements(); - over the screen's size; the widgets take the places it gives them
        let (__width, __layout) = compat::write(__this, |__t| {
            let __b = __t.abstract_waypoints_screen_base_mut();
            let (width, height) = (__b.base.width, __b.base.height);
            let layout = __b.layout.as_mut().unwrap();
            layout.screen_width = width;
            layout.screen_height = height;
            layout.arrange_elements();
            for (__i, __x, __y) in layout.visit_widgets() {
                if __i < __b.base.children().len() {
                    __b.base.widget_mut(__i).set_position(__x, __y);
                }
            }
            (width, __b.layout.clone().unwrap())
        });
        // java: waypointsListWidget.updateSize(width, layout); waypointsListWidget.updateEntries();
        <dyn AbstractWaypointsScreen>::with_list(__this, |__h| compat::write(__h, |__t| __t.base.list.update_size(__width, &__layout)));
        <dyn AbstractWaypointsScreen>::with_list(__this, |__h| WaypointsListWidget::update_entries(__h));
        let (__island_widget, __header_height, __height) = compat::read(__this, |__t| {
            let __b = __t.abstract_waypoints_screen_base();
            (__b.island_widget.clone().unwrap(), __b.layout.as_ref().unwrap().get_header_height(), __b.base.height)
        });
        // java: islandWidget.setX(width - islandWidget.getWidth() - 10);
        compat::write(&__island_widget, |__t| {
            let __x = (__width.wrapping_sub(__t.dropdown_widget_base().get_width())).wrapping_sub(10);
            __t.set_x(__x);
        });
        // java: FrameLayout.alignInDimension(0, layout.getHeaderHeight(), islandWidget.getHeaderHeight(), islandWidget::setY, 0.5f);
        compat::write(&__island_widget, |__t| {
            let __y = frame_layout::align_in_dimension(0, __header_height, __t.get_header_height(), 0.5f32);
            __t.set_y(__y);
        });
        // java: islandWidget.setMaxHeight(Math.max(height - islandWidget.getY() - 8, 20));
        compat::write(&__island_widget, |__t| {
            let __max = ((__height.wrapping_sub(__t.dropdown_widget_base().get_y())).wrapping_sub(8)).max(20);
            __t.set_max_height(__max);
        });
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled) - the part before the game's
    /// widget sweep (`super.mouseClicked`); `true` ends the click there.
    pub fn mouse_clicked(__this: &compat::Shared<dyn AbstractWaypointsScreen>, click: &MouseButtonEvent, doubled: bool) -> bool {
        // java: if (islandWidget.mouseClicked(click, doubled)) return true;
        let __island_widget = compat::read(__this, |__t| __t.abstract_waypoints_screen_base().island_widget.clone());
        if let Some(__island_widget) = __island_widget
            && compat::write(&__island_widget, |__t| __t.mouse_clicked(click, doubled))
        {
            return true;
        }
        // java: boolean mouseClicked = super.mouseClicked(click, doubled); - `ContainerEventHandler.mouseClicked` over
        // `children()`, whose first is the popup container: a shown container under the cursor takes the click (its
        // own children: the close button and the popup's widget), and is focused
        let __popup_hit = compat::write(__this, |__t| {
            let __popup = &mut __t.abstract_waypoints_screen_base_mut().popup_container;
            if __popup.base.visible && __popup.is_mouse_over(click.x(), click.y()) {
                __popup.base.set_focused(true, mcrust::net::minecraft::client::input_type::InputType::Mouse);
                __popup.dragging = click.button() == 1;
                Some(compat::children_mouse_clicked(&mut __popup.children, click))
            } else {
                __popup.base.set_focused(false, mcrust::net::minecraft::client::input_type::InputType::Mouse);
                None
            }
        });
        // the close button's press (`_ -> visible = false`) ran inside the container's children
        compat::write(__this, |__t| __t.abstract_waypoints_screen_base_mut().popup_container.apply_close_request());
        // java: updateButtons(); return mouseClicked;
        if __popup_hit.is_some() {
            <dyn AbstractWaypointsScreen>::update_buttons(__this);
            return true;
        }
        // the rest of `super.mouseClicked` is the game's widget sweep, which runs after this answers `false`: the
        // `updateButtons()` that follows it in Java is made at the screen's next tick
        compat::write(__this, |__t| __t.abstract_waypoints_screen_base_mut().update_buttons_requested = true);
        false
    }

    /// java: `ContainerEventHandler.mouseReleased` - the popup container while it is the dragged child
    pub fn mouse_released(__this: &compat::Shared<dyn AbstractWaypointsScreen>, click: &MouseButtonEvent) -> bool {
        compat::write(__this, |__t| {
            let __popup = &mut __t.abstract_waypoints_screen_base_mut().popup_container;
            if __popup.dragging {
                __popup.dragging = false;
                compat::children_mouse_released(&mut __popup.children, click);
                return true;
            }
            false
        })
    }

    /// java: `ContainerEventHandler.mouseDragged` - the popup container while it is the dragged child
    pub fn mouse_dragged(__this: &compat::Shared<dyn AbstractWaypointsScreen>, click: &MouseButtonEvent, dx: f64, dy: f64) -> bool {
        compat::write(__this, |__t| {
            let __popup = &mut __t.abstract_waypoints_screen_base_mut().popup_container;
            if __popup.dragging && __popup.base.visible {
                compat::children_mouse_dragged(&mut __popup.children, click, dx, dy);
                return true;
            }
            false
        })
    }

    /// java: @Override public boolean mouseScrolled(double mouseX, double mouseY, double horizontalAmount, double verticalAmount)
    pub fn mouse_scrolled(__this: &compat::Shared<dyn AbstractWaypointsScreen>, mouse_x: f64, mouse_y: f64, horizontal_amount: f64, vertical_amount: f64) -> bool {
        // java: if (islandWidget.isMouseOver(mouseX, mouseY) && islandWidget.mouseScrolled(..)) return true;
        let __island_widget = compat::read(__this, |__t| __t.abstract_waypoints_screen_base().island_widget.clone());
        if let Some(__island_widget) = __island_widget
            && compat::read(&__island_widget, |__t| __t.dropdown_widget_base().is_mouse_over(mouse_x, mouse_y))
            && compat::read(&__island_widget, |__t| __t.mouse_scrolled(mouse_x, mouse_y, horizontal_amount, vertical_amount))
        {
            return true;
        }
        // java: return super.mouseScrolled(..); - `getChildAt` finds the popup container first while it is shown and
        // under the cursor (it does not scroll, and nothing under it is asked)
        let __over_popup = compat::read(__this, |__t| {
            let __popup = &__t.abstract_waypoints_screen_base().popup_container;
            __popup.base.visible && __popup.is_mouse_over(mouse_x, mouse_y)
        });
        if __over_popup {
            return true;
        }
        false
    }

    /// java: protected void islandChanged(Location location)
    pub fn island_changed(__this: &compat::Shared<dyn AbstractWaypointsScreen>, location: Location) {
        // java: island = location;
        compat::write(__this, |__t| __t.abstract_waypoints_screen_base_mut().island = Some(location));
        // java: waypointsListWidget.setIsland(island);
        <dyn AbstractWaypointsScreen>::with_list(__this, |__h| WaypointsListWidget::set_island(__h, location));
    }

    /// java: protected void updateButtons()
    pub fn update_buttons(__this: &compat::Shared<dyn AbstractWaypointsScreen>) {
        <dyn AbstractWaypointsScreen>::with_list(__this, |__h| WaypointsListWidget::update_buttons(__h));
    }

    /// `Screen.tick` - the `updateButtons()` a click asked for (see `mouse_clicked`)
    pub fn tick(__this: &compat::Shared<dyn AbstractWaypointsScreen>) {
        if compat::write(__this, |__t| std::mem::take(&mut __t.abstract_waypoints_screen_base_mut().update_buttons_requested)) {
            <dyn AbstractWaypointsScreen>::update_buttons(__this);
        }
    }

    /// `Screen.extractRenderState` after the generic pass (the list and the layout's widgets): the island dropdown
    /// (`addRenderableWidget` in `lateInit`, after the layout's) and the popup container (`addRenderableOnly`, last)
    pub fn extract_render_state(__this: &compat::Shared<dyn AbstractWaypointsScreen>, context: &mut ScreenRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        // java: islandWidget.extractRenderState(graphics, mouseX, mouseY, a) - `if (visible) extractWidgetRenderState(..)`
        let __island_widget = compat::read(__this, |__t| __t.abstract_waypoints_screen_base().island_widget.clone());
        if let Some(__island_widget) = __island_widget {
            compat::write(&__island_widget, |__t| {
                if __t.dropdown_widget_base().visible {
                    __t.extract_widget_render_state(&mut context.graphics, mouse_x, mouse_y, a);
                }
            });
        }
        // java: popupContainer.extractRenderState(graphics, mouseX, mouseY, a)
        let __widget_context = WidgetRenderContext::new(context.gl, &*context.minecraft);
        compat::write(__this, |__t| {
            let __popup = &mut __t.abstract_waypoints_screen_base_mut().popup_container;
            if __popup.base.visible {
                __popup.base.set_hovered_from_extract(mouse_x, mouse_y);
                // (the hover test reads the container's own size, which `getWidth`/`getHeight` override)
                let __hovered = __popup.is_mouse_over(mouse_x as f64, mouse_y as f64);
                __popup.extract_widget_render_state(&mut context.graphics, &__widget_context, __hovered, mouse_x, mouse_y, a);
            }
        });
    }
}

/// java: private static class PopupContainer extends AbstractContainerWidget
pub struct PopupContainer {
    /// java: extends net.minecraft.client.gui.components.AbstractContainerWidget - the widget's own state (box,
    /// `visible`, focus, hover)
    pub base: AbstractWidget,
    /// java: private LayoutElement widget = SpacerElement.width(0); - the popup's content, a leaf over its index in
    /// `children` (`None` is the empty spacer: no width, no height)
    pub widget: Option<LayoutElement>,
    /// java: private final List<AbstractWidget> children = new ArrayList<>(4); - held in a `Screen` the container draws
    /// and clicks: the close button (index 0) and the content's widgets once `setWidget` has run, empty before
    pub children: Screen,
    /// java: private final AbstractButton closeButton; - the button itself; `children` holds it from `setWidget` on
    pub close_button: AbstractWidget,
    /// the close button's press (`_ -> visible = false`), raised inside the children's click and applied after it
    pub close_requested: Arc<std::sync::atomic::AtomicBool>,
    /// the container is the screen's dragged child (a press inside it, until the release)
    pub dragging: bool,
}

/// `closeButton`'s index in `children` (it is added first by `setWidget`)
const CLOSE_BUTTON: usize = 0;

impl PopupContainer {
    /// java: PopupContainer()
    pub fn new() -> Self {
        // java: super(0, 0, 0, 0, Component.empty(), AbstractScrollArea.defaultSettings(8));
        let base = AbstractWidget::new(0, 0, 0, 0, String::new(), WidgetKind::PlainText, OnPress::Inert);
        // java: this.closeButton = new ImageButton(14, 14, new WidgetSprites(Identifier.withDefaultNamespace("widget/cross_button"),
        //   Identifier.withDefaultNamespace("widget/cross_button_highlighted")), _ -> visible = false, Component.empty());
        let close_requested = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let __request = close_requested.clone();
        let close_button = AbstractWidget::new(
            0,
            0,
            14,
            14,
            String::new(),
            WidgetKind::ImageButton(WidgetSprites::with_focused(Identifier::with_default_namespace("widget/cross_button"), Identifier::with_default_namespace("widget/cross_button_highlighted"))),
            OnPress::Lambda(PressLambda::new(move |_: &mut Screen, _: usize| {
                __request.store(true, std::sync::atomic::Ordering::Relaxed);
            })),
        );
        Self { base, widget: None, children: Screen::new(String::new()), close_button, close_requested, dragging: false }
    }

    /// `visible = false` from the close button's press
    pub fn apply_close_request(&mut self) {
        if self.close_requested.swap(false, std::sync::atomic::Ordering::Relaxed) {
            self.base.visible = false;
        }
    }

    fn widget_width(&self) -> i32 {
        self.widget.as_ref().map_or(0, |w| w.get_width())
    }

    fn widget_height(&self) -> i32 {
        self.widget.as_ref().map_or(0, |w| w.get_height())
    }

    /// The close button where it is: in `children` once `setWidget` has run, the field's own widget before.
    fn close_button_mut(&mut self) -> &mut AbstractWidget {
        if self.children.children().len() > CLOSE_BUTTON {
            self.children.widget_mut(CLOSE_BUTTON)
        } else {
            &mut self.close_button
        }
    }

    fn close_button_ref(&self) -> &AbstractWidget {
        if self.children.children().len() > CLOSE_BUTTON {
            self.children.widget(CLOSE_BUTTON)
        } else {
            &self.close_button
        }
    }

    /// java: @Override public int getWidth()
    pub fn get_width(&self) -> i32 {
        return self.widget_width().wrapping_add(POPUP_CONTAINER_BORDER_SIZE.wrapping_mul(2));
    }

    /// java: @Override public int getHeight()
    pub fn get_height(&self) -> i32 {
        return ((self.widget_height().wrapping_add(POPUP_CONTAINER_BORDER_SIZE.wrapping_mul(2))).wrapping_add(self.close_button_ref().get_height())).wrapping_add(2);
    }

    /// `getRight()` / `getBottom()` over the overridden size
    fn get_right(&self) -> i32 {
        self.base.get_x().wrapping_add(self.get_width())
    }

    fn get_bottom(&self) -> i32 {
        self.base.get_y().wrapping_add(self.get_height())
    }

    /// `AbstractWidget.isMouseOver` - `isActive() && areCoordinatesInRectangle(..)` over the overridden size
    pub fn is_mouse_over(&self, mouse_x: f64, mouse_y: f64) -> bool {
        self.base.visible
            && self.base.active
            && mouse_x >= self.base.get_x() as f64
            && mouse_y >= self.base.get_y() as f64
            && mouse_x < self.get_right() as f64
            && mouse_y < self.get_bottom() as f64
    }

    /// `setPosition(x, y)` - `setX(x); setY(y);`
    pub fn set_position(&mut self, x: i32, y: i32) {
        self.set_x(x);
        self.set_y(y);
    }

    /// java: @Override public void setX(int x)
    pub fn set_x(&mut self, x: i32) {
        // java: super.setX(x);
        self.base.set_x(x);
        // java: closeButton.setX(getRight() - BORDER_SIZE - closeButton.getWidth());
        let __close_x = (self.get_right().wrapping_sub(POPUP_CONTAINER_BORDER_SIZE)).wrapping_sub(self.close_button_ref().get_width());
        self.close_button_mut().set_x(__close_x);
        // java: widget.setX(getX() + BORDER_SIZE);
        let __x = self.base.get_x().wrapping_add(POPUP_CONTAINER_BORDER_SIZE);
        self.move_widget(Some(__x), None);
    }

    /// java: @Override public void setY(int y)
    pub fn set_y(&mut self, y: i32) {
        // java: super.setY(y);
        self.base.set_y(y);
        // java: closeButton.setY(getY() + BORDER_SIZE);
        let __close_y = self.base.get_y().wrapping_add(POPUP_CONTAINER_BORDER_SIZE);
        self.close_button_mut().set_y(__close_y);
        // java: widget.setY(closeButton.getBottom() + 2);
        let __y = self.close_button_ref().get_bottom().wrapping_add(2);
        self.move_widget(None, Some(__y));
    }

    /// `widget.setX` / `setY` on the content leaf: the content widget takes the position
    fn move_widget(&mut self, x: Option<i32>, y: Option<i32>) {
        if let Some(LayoutElement::Widget { index, .. }) = self.widget.as_ref() {
            let index = *index;
            if index < self.children.children().len() {
                let w = self.children.widget_mut(index);
                if let Some(x) = x {
                    w.set_x(x);
                }
                if let Some(y) = y {
                    w.set_y(y);
                }
            }
        }
    }

    /// java: @Override public List<? extends GuiEventListener> children() - `visible ? children : List.of()`
    pub fn children(&self) -> Vec<AbstractWidget> {
        return (if self.base.visible { self.children.children().to_vec() } else { vec![] });
    }

    /// java: public void setWidget(LayoutElement widget)
    pub fn set_widget(&mut self, widget: AbstractWidget) {
        // java: children.clear(); children.add(closeButton);
        let close_button = self.close_button_ref().clone();
        self.children = Screen::new(String::new());
        self.children.add_renderable_widget(close_button);
        // java: widget.visitWidgets(children::add);
        let (w, h) = (widget.get_width(), widget.get_height());
        let index = self.children.add_renderable_widget(widget);
        // java: this.widget = widget;
        self.widget = Some(LayoutElement::widget(index, w, h));
        // java: closeButton.setX(getRight() - BORDER_SIZE - closeButton.getWidth());
        let __close_x = (self.get_right().wrapping_sub(POPUP_CONTAINER_BORDER_SIZE)).wrapping_sub(self.close_button_ref().get_width());
        self.close_button_mut().set_x(__close_x);
    }

    /// java: @Override protected int contentHeight()
    pub fn content_height(&self) -> i32 {
        return 0;
    }

    /// java: @Override protected double scrollRate()
    pub fn scroll_rate(&self) -> f64 {
        return 0.0;
    }

    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor context, int mouseX, int mouseY, float deltaTicks)
    pub fn extract_widget_render_state(&mut self, context: &mut GuiGraphicsExtractor, widget_context: &WidgetRenderContext<'_>, hovered: bool, mouse_x: i32, mouse_y: i32, delta_ticks: f32) {
        let _ = delta_ticks;
        // Set the cursor to default to prevent widgets from below taking over the shape when they cannot be interacted with
        if hovered {
            // java: context.requestCursor(CursorType.DEFAULT);
            compat::batch_w_request_cursor(compat::BatchWCursor::Default);
        }
        let (x, y, right, bottom) = (self.base.get_x(), self.base.get_y(), self.get_right(), self.get_bottom());
        context.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), x, y, right, bottom, argb::color_alpha_f32_rgb(0.6f32, 0));
        GuiHelper::border(context, x, y, self.get_width(), self.get_height(), mcrust::net::minecraft::util::common_colors::WHITE);
        // java: for (AbstractWidget child : children) child.extractRenderState(context, mouseX, mouseY, deltaTicks);
        widget_context.extract_widgets(&mut self.children, mouse_x, mouse_y);
        // java: if (!isFocused() && (mouseX <= getX() - 50 || mouseX >= getRight() + 50 || mouseY <= getY() - 50 || mouseY >= getBottom() + 50)) visible = false;
        if !self.base.is_focused() && (mouse_x <= x.wrapping_sub(50) || mouse_x >= right.wrapping_add(50) || mouse_y <= y.wrapping_sub(50) || mouse_y >= bottom.wrapping_add(50)) {
            self.base.visible = false;
        }
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder) - empty
    pub fn update_widget_narration(&self) {}
}

/// `MouseButtonEvent` for a press the game hands the screen as `(x, y, button)`
pub fn click_event(x: f64, y: f64, button: i32) -> MouseButtonEvent {
    MouseButtonEvent::new(x, y, MouseButtonInfo::new(button, 0))
}
