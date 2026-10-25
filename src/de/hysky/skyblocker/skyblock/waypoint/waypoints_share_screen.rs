//! Ported from de.hysky.skyblocker.skyblock.waypoint.WaypointsShareScreen by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::waypoint::abstract_waypoints_screen;
use crate::de::hysky::skyblocker::skyblock::waypoint::abstract_waypoints_screen::AbstractWaypointsScreen;
use crate::de::hysky::skyblocker::skyblock::waypoint::abstract_waypoints_screen::AbstractWaypointsScreenBase;
use crate::de::hysky::skyblocker::skyblock::waypoint::waypoints;
use crate::de::hysky::skyblocker::skyblock::waypoint::waypoints::Waypoints;
use crate::de::hysky::skyblocker::skyblock::waypoint::waypoints_list_widget::WaypointsListWidget;
use crate::de::hysky::skyblocker::skyblock::waypoint::waypoints_screen::WaypointsScreen;
use crate::de::hysky::skyblocker::utils::location::Location;
use crate::de::hysky::skyblocker::utils::waypoint::named_waypoint;
use crate::de::hysky::skyblocker::utils::waypoint::named_waypoint::NamedWaypoint;
use crate::de::hysky::skyblocker::utils::waypoint::waypoint_group::WaypointGroup;
use mcrust::mcrust::modding::screens::ModScreen;
use mcrust::mcrust::modding::screens::ScreenRenderContext;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::abstract_widget::PressLambda;
use mcrust::net::minecraft::client::gui::components::button;
use mcrust::net::minecraft::client::gui::components::toasts::system_toast;
use mcrust::net::minecraft::client::gui::components::tooltip::Tooltip;
use mcrust::net::minecraft::client::gui::layouts::grid_layout::GridLayout;
use mcrust::net::minecraft::client::gui::layouts::grid_layout::RowHelper;
use mcrust::net::minecraft::client::gui::layouts::layout_element::LayoutElement;
use mcrust::net::minecraft::client::gui::screens::screen::Screen;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::formatted_text::FormattedText;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: public class WaypointsShareScreen extends AbstractWaypointsScreen<WaypointsScreen>
pub struct WaypointsShareScreen {
    /// java: extends AbstractWaypointsScreen
    pub base: AbstractWaypointsScreenBase,
    /// java: private final Set<NamedWaypoint> selectedWaypoints = new HashSet<>();
    /// A `HashSet` keyed by `NamedWaypoint.equals` / `hashCode` (structural, not identity): the members are kept in a
    /// list and asked with `NamedWaypoint.equals` (`<dyn NamedWaypoint>::equals_shared`).
    pub selected_waypoints: Vec<compat::Shared<dyn NamedWaypoint>>,
    /// java: private boolean overrideLocation = false;
    pub override_location: bool,
    /// java: private boolean sortWaypoints = false;
    pub sort_waypoints: bool,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl std::ops::Deref for WaypointsShareScreen {
    type Target = AbstractWaypointsScreenBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for WaypointsShareScreen {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

/// `Set.add(waypoint)` over the structural set: only when no equal member is there.
fn set_add(set: &mut Vec<compat::Shared<dyn NamedWaypoint>>, waypoint: &compat::Shared<dyn NamedWaypoint>) {
    if !set.iter().any(|w| <dyn NamedWaypoint>::equals_shared(w, waypoint)) {
        set.push(waypoint.clone());
    }
}

/// `minecraft.keyboardHandler.getClipboard()`
fn get_clipboard() -> String {
    let mut __memory = mcrust::net::minecraft::client::gui::components::edit_box::MemoryClipboard::default();
    match mcrust::net::minecraft::client::minecraft::SdlClipboard::published() {
        Some(mut __glfw) => compat::minecraft().keyboard_handler.get_clipboard(&mut __glfw),
        None => compat::minecraft().keyboard_handler.get_clipboard(&mut __memory),
    }
}

/// `minecraft.keyboardHandler.setClipboard(value)`
fn set_clipboard(value: &str) {
    let mut __memory = mcrust::net::minecraft::client::gui::components::edit_box::MemoryClipboard::default();
    match mcrust::net::minecraft::client::minecraft::SdlClipboard::published() {
        Some(mut __glfw) => compat::minecraft().keyboard_handler.set_clipboard(&mut __glfw, value),
        None => compat::minecraft().keyboard_handler.set_clipboard(&mut __memory, value),
    }
}

/// `SystemToast.addOrUpdate(minecraft.gui.toastManager(), Waypoints.WAYPOINTS_TOAST_TYPE, title, message)`
fn waypoints_toast(title: MutableComponent, message: MutableComponent) {
    let minecraft = compat::minecraft();
    system_toast::add_or_update(&mut minecraft.gui.toast_manager().borrow_mut(), &minecraft.font, *waypoints::WAYPOINTS_TOAST_TYPE.get(), &FormattedText::from(title), Some(&FormattedText::from(message)));
}

/// `Component.translatable(key, waypointCount, groupCount)` - the two `Integer` arguments
fn count_text(key: &str, waypoint_count: i32, group_count: i32) -> MutableComponent {
    component::translatable_args(key, vec![FormattedText::of(waypoint_count.to_string()), FormattedText::of(group_count.to_string())])
}

impl WaypointsShareScreen {
    /// java: protected WaypointsShareScreen(WaypointsScreen parent, Multimap<Location, WaypointGroup> waypoints)
    pub fn new(parent: compat::Shared<WaypointsScreen>, waypoints: compat::Shared<waypoints::LocationMultimap>) -> compat::Shared<Self> {
        // java: super(Component.translatable("skyblocker.waypoints.shareWaypoints"), parent, waypoints, parent.island);
        let __island = compat::read(&parent, |__t| __t.island).unwrap();
        let __this = compat::shared(Self {
            base: AbstractWaypointsScreenBase::new_4(component::translatable("skyblocker.waypoints.shareWaypoints"), Some(std::rc::Rc::new(parent) as std::rc::Rc<dyn std::any::Any>), waypoints, __island),
            self_ref: std::sync::Weak::new(),
            selected_waypoints: Vec::new(),
            override_location: false,
            sort_waypoints: false,
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        __this
    }

    /// The object as the class it extends (`AbstractWaypointsScreen<?>`), for the bodies the base class declares.
    fn as_abstract(__this: &compat::Shared<Self>) -> compat::Shared<dyn AbstractWaypointsScreen> {
        __this.clone()
    }

    /// `adder.addChild(widget[, occupiedColumns])`: the widget joins the screen's widget list, the grid holds its index.
    fn add_to_grid(__this: &compat::Shared<Self>, adder: &mut RowHelper, grid: &mut GridLayout, widget: AbstractWidget, columns: i32) {
        let (__ww, __wh) = (widget.get_width(), widget.get_height());
        let __i = compat::write(__this, |__t| __t.base.base.add_widget(widget));
        adder.add_child_width(grid, LayoutElement::widget(__i, __ww, __wh), columns).unwrap();
    }

    /// A button whose press runs `action` on this screen (the live screen lent to it for the call).
    fn button(__this: &compat::Shared<Self>, message: MutableComponent, action: fn(&compat::Shared<Self>)) -> mcrust::net::minecraft::client::gui::components::button::Builder {
        let __this_w = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        button::builder(message.get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut Screen, __index: usize| {
            let Some(__this) = __this_w.get().upgrade() else { return };
            compat::lend_screen(&__this, __screen, || action(&__this))
        })))
    }

    /// `Checkbox.builder(message, font).maxWidth(Button.DEFAULT_WIDTH).onValueChange((_, checked) -> field = checked)
    /// .tooltip(Tooltip.create(tooltip)).build()`
    fn option_checkbox(__this: &compat::Shared<Self>, message: &str, tooltip: &str, set: fn(&mut Self, bool)) -> AbstractWidget {
        let __this_w = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        let mut checkbox = compat::checkbox(FormattedText::from(component::translatable(message)), 0, 0, Some(button::DEFAULT_WIDTH), false, move |checked: bool| {
            if let Some(__this) = __this_w.get().upgrade() {
                compat::write(&__this, |__t| set(__t, checked));
            }
        });
        checkbox.set_tooltip(Tooltip::create(component::translatable(tooltip).get_string()));
        checkbox
    }

    /// java: @Override protected void init()
    pub fn init(__this: &compat::Shared<Self>) {
        let __abstract = WaypointsShareScreen::as_abstract(__this);
        // java: super.init();
        <dyn AbstractWaypointsScreen>::init_super(&__abstract);
        let mut row_spacing: i32 = 2;
        let mut column_spacing: i32 = 5;
        // java: GridLayout gridWidget = new GridLayout().columnSpacing(columnSpacing).rowSpacing(rowSpacing);
        let mut grid_widget: GridLayout = GridLayout::new_default();
        grid_widget.column_spacing(column_spacing);
        grid_widget.row_spacing(row_spacing);
        let mut adder: RowHelper = grid_widget.create_row_helper(2);
        // First row
        let __override = WaypointsShareScreen::option_checkbox(__this, "skyblocker.waypoints.importOptions.overrideLocation", "skyblocker.waypoints.importOptions.overrideLocation.tooltip", |__t, checked| __t.override_location = checked);
        WaypointsShareScreen::add_to_grid(__this, &mut adder, &mut grid_widget, __override, 1);
        let __sort = WaypointsShareScreen::option_checkbox(__this, "skyblocker.waypoints.importOptions.sortWaypoints", "skyblocker.waypoints.importOptions.sortWaypoints.tooltip", |__t, checked| __t.sort_waypoints = checked);
        WaypointsShareScreen::add_to_grid(__this, &mut adder, &mut grid_widget, __sort, 1);
        // Second Row
        let __import_skyblocker = WaypointsShareScreen::button(__this, component::translatable("skyblocker.waypoints.importWaypointsSkyblocker"), WaypointsShareScreen::import_waypoints_skyblocker).tooltip(Some(component::translatable("skyblocker.waypoints.importWaypointsSkyblocker.tooltip").get_string())).build();
        WaypointsShareScreen::add_to_grid(__this, &mut adder, &mut grid_widget, __import_skyblocker, 1);
        let __export_skyblocker = WaypointsShareScreen::button(__this, component::translatable("skyblocker.waypoints.exportWaypointsSkyblocker"), WaypointsShareScreen::export_waypoints_skyblocker).tooltip(Some(component::translatable("skyblocker.waypoints.exportWaypointsSkyblocker.tooltip").get_string())).build();
        WaypointsShareScreen::add_to_grid(__this, &mut adder, &mut grid_widget, __export_skyblocker, 1);
        // Third row
        let __import_skytils = WaypointsShareScreen::button(__this, component::translatable("skyblocker.waypoints.importWaypointsSkytils"), WaypointsShareScreen::import_waypoints_skytils).tooltip(Some(component::translatable("skyblocker.waypoints.importWaypointsSkytils.tooltip").get_string())).build();
        WaypointsShareScreen::add_to_grid(__this, &mut adder, &mut grid_widget, __import_skytils, 1);
        // Fourth row
        let __import_snoopy = WaypointsShareScreen::button(__this, component::translatable("skyblocker.waypoints.importWaypointsSnoopy"), WaypointsShareScreen::import_waypoints_snoopy).tooltip(Some(component::translatable("skyblocker.waypoints.importWaypointsSnoopy.tooltip").get_string())).build();
        WaypointsShareScreen::add_to_grid(__this, &mut adder, &mut grid_widget, __import_snoopy, 1);
        // java: adder.addChild(Button.builder(CommonComponents.GUI_DONE, _ -> onClose()).width((Button.DEFAULT_WIDTH * 2) + columnSpacing).build(), 2);
        let __done = WaypointsShareScreen::button(__this, compat::component_of(mcrust::net::minecraft::network::chat::common_components::gui_done()), WaypointsShareScreen::on_close).width((button::DEFAULT_WIDTH.wrapping_mul(2)).wrapping_add(column_spacing)).build();
        WaypointsShareScreen::add_to_grid(__this, &mut adder, &mut grid_widget, __done, 2);
        // java: layout.addToFooter(gridWidget); int rows = 4; layout.setFooterHeight(20 * rows + rowSpacing * (rows - 1) + 8);
        let mut rows: i32 = 4;
        compat::write(__this, |__t| {
            let __layout = __t.base.layout.as_mut().unwrap();
            __layout.add_to_footer(LayoutElement::Grid(grid_widget));
            __layout.set_footer_height(((20i32.wrapping_mul(rows)).wrapping_add(row_spacing.wrapping_mul(rows.wrapping_sub(1)))).wrapping_add(8));
        });
        // java: super.lateInit();
        <dyn AbstractWaypointsScreen>::late_init(&__abstract);
    }

    /// The imported groups into the screen's map: `withIsland` / `sortWaypoints` by the import options, the waypoints
    /// selected, each group put under its island (the loop the three importers share)
    fn put_imported(__this: &compat::Shared<Self>, waypoint_groups: &[compat::Shared<WaypointGroup>]) {
        let (island, override_location, sort_waypoints, map) = compat::read(__this, |__t| (__t.base.island.unwrap(), __t.override_location, __t.sort_waypoints, __t.base.waypoints.clone().unwrap()));
        for waypoint_group in waypoint_groups.iter() {
            let mut waypoint_group: compat::Shared<WaypointGroup> = waypoint_group.clone();
            // java: if (overrideLocation) waypointGroup = waypointGroup.withIsland(island);
            if override_location {
                waypoint_group = WaypointGroup::with_island(&waypoint_group, island);
            }
            // java: if (sortWaypoints) waypointGroup = waypointGroup.sortWaypoints(NamedWaypoint.NAME_COMPARATOR);
            if sort_waypoints {
                waypoint_group = WaypointGroup::sort_waypoints(&waypoint_group, named_waypoint::NAME_COMPARATOR.get().clone());
            }
            // java: selectedWaypoints.addAll(waypointGroup.waypoints());
            let __waypoints = WaypointGroup::waypoints(&waypoint_group);
            compat::write(__this, |__t| {
                for __w in __waypoints.iter() {
                    set_add(&mut __t.selected_waypoints, __w);
                }
            });
            // java: waypoints.put(waypointGroup.island(), waypointGroup);
            let __island = WaypointGroup::island(&waypoint_group);
            compat::write(&map, |__m| __m.put(__island, waypoint_group.clone()));
        }
    }

    /// java: the "importWaypointsSkyblocker" button's action
    fn import_waypoints_skyblocker(__this: &compat::Shared<Self>) {
        WaypointsShareScreen::import_waypoints(__this, Waypoints::from_skyblocker, "Skyblocker");
    }

    /// java: the "importWaypointsSkytils" button's action
    fn import_waypoints_skytils(__this: &compat::Shared<Self>) {
        WaypointsShareScreen::import_waypoints(__this, Waypoints::from_skytils, "Skytils");
    }

    /// java: `try { List<WaypointGroup> waypointGroups = Waypoints.fromX(minecraft.keyboardHandler.getClipboard(), island);
    ///   if (waypointGroups == null) { showErrorToast(); return; } for (..) {..} waypointsListWidget.updateEntries();
    ///   SystemToast.addOrUpdate(.., "skyblocker.waypoints.importSuccess", "skyblocker.waypoints.importSuccessText", ..); }
    ///   catch (Exception e) { LOGGER.error("[Skyblocker Waypoints] Encountered exception while parsing X waypoint data", e); showErrorToast(); }`
    fn import_waypoints(__this: &compat::Shared<Self>, parse: fn(String, Location) -> Option<Vec<compat::Shared<WaypointGroup>>>, source: &str) {
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let island = compat::read(__this, |__t| __t.base.island.unwrap());
            let Some(waypoint_groups) = parse(get_clipboard(), island) else {
                WaypointsShareScreen::show_error_toast(__this);
                return;
            };
            WaypointsShareScreen::put_imported(__this, &waypoint_groups);
            // java: waypointsListWidget.updateEntries();
            <dyn AbstractWaypointsScreen>::with_list(&WaypointsShareScreen::as_abstract(__this), |__h| WaypointsListWidget::update_entries(__h));
            // java: waypointGroups.stream().map(WaypointGroup::waypoints).mapToInt(List::size).sum(), waypointGroups.size()
            let waypoint_count: i32 = waypoint_groups.iter().map(|__g| WaypointGroup::waypoints(__g).len() as i32).sum();
            waypoints_toast(component::translatable("skyblocker.waypoints.importSuccess"), count_text("skyblocker.waypoints.importSuccessText", waypoint_count, waypoint_groups.len() as i32));
        }));
        if let Err(e) = __result {
            logger().error(&format!("[Skyblocker Waypoints] Encountered exception while parsing {} waypoint data {}", source, compat::panic_message(&e)));
            WaypointsShareScreen::show_error_toast(__this);
        }
    }

    /// java: the "exportWaypointsSkyblocker" button's action
    fn export_waypoints_skyblocker(__this: &compat::Shared<Self>) {
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            // java: List<WaypointGroup> waypointGroups = waypoints.values().stream().filter(waypointGroup -> waypointGroup.island().equals(island))
            //   .map(waypointGroup -> waypointGroup.filterWaypoints(selectedWaypoints::contains)).filter(waypointGroup -> !waypointGroup.waypoints().isEmpty()).toList();
            let (island, map, selected) = compat::read(__this, |__t| (__t.base.island.unwrap(), __t.base.waypoints.clone().unwrap(), __t.selected_waypoints.clone()));
            let selected = compat::RenderThread::new(selected);
            let contains: Arc<dyn Fn(compat::Shared<dyn NamedWaypoint>) -> bool> = Arc::new(move |__w: compat::Shared<dyn NamedWaypoint>| selected.get().iter().any(|__s| <dyn NamedWaypoint>::equals_shared(__s, &__w)));
            let waypoint_groups: Vec<compat::Shared<WaypointGroup>> = compat::read(&map, |__m| __m.values())
                .into_iter()
                .filter(|__g| WaypointGroup::island(__g) == island)
                .map(|__g| WaypointGroup::filter_waypoints(&__g, contains.clone()))
                .filter(|__g| !WaypointGroup::waypoints(__g).is_empty())
                .collect();
            // java: minecraft.keyboardHandler.setClipboard(Waypoints.toSkyblocker(waypointGroups));
            set_clipboard(&Waypoints::to_skyblocker(&waypoint_groups));
            let waypoint_count: i32 = waypoint_groups.iter().map(|__g| WaypointGroup::waypoints(__g).len() as i32).sum();
            waypoints_toast(component::translatable("skyblocker.waypoints.exportSuccess"), count_text("skyblocker.waypoints.exportSuccessText", waypoint_count, waypoint_groups.len() as i32));
        }));
        if let Err(e) = __result {
            logger().error(&format!("[Skyblocker Waypoints] Encountered exception while serializing Skyblocker waypoint data {}", compat::panic_message(&e)));
            waypoints_toast(component::translatable("skyblocker.waypoints.exportError"), component::translatable("skyblocker.waypoints.exportErrorText"));
        }
    }

    /// java: the "importWaypointsSnoopy" button's action
    fn import_waypoints_snoopy(__this: &compat::Shared<Self>) {
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            // java: WaypointGroup waypointGroup = Waypoints.fromColeweightJson(minecraft.keyboardHandler.getClipboard(), island);
            let island = compat::read(__this, |__t| __t.base.island.unwrap());
            let waypoint_group: compat::Shared<WaypointGroup> = Waypoints::from_coleweight_json(&get_clipboard(), island);
            WaypointsShareScreen::put_imported(__this, std::slice::from_ref(&waypoint_group));
            // java: waypointsListWidget.updateEntries();
            <dyn AbstractWaypointsScreen>::with_list(&WaypointsShareScreen::as_abstract(__this), |__h| WaypointsListWidget::update_entries(__h));
            // java: Component.translatable("skyblocker.waypoints.importSuccessText", waypointGroup.waypoints().size(), 1) - the
            // group as imported (before `withIsland` / `sortWaypoints`, which keep its waypoint count)
            waypoints_toast(component::translatable("skyblocker.waypoints.importSuccess"), count_text("skyblocker.waypoints.importSuccessText", WaypointGroup::waypoints(&waypoint_group).len() as i32, 1));
        }));
        if let Err(e) = __result {
            logger().error(&format!("[Skyblocker Waypoints] Encountered exception while parsing Snoopy waypoint data {}", compat::panic_message(&e)));
            WaypointsShareScreen::show_error_toast(__this);
        }
    }

    /// java: private void showErrorToast()
    pub fn show_error_toast(__this: &compat::Shared<Self>) {
        let _ = __this;
        // java: SystemToast.addOrUpdate(client.getToastManager(), Waypoints.WAYPOINTS_TOAST_TYPE, Component.translatable("skyblocker.waypoints.importError"), Component.translatable("skyblocker.waypoints.importErrorText"));
        waypoints_toast(component::translatable("skyblocker.waypoints.importError"), component::translatable("skyblocker.waypoints.importErrorText"));
    }

    /// java: @Override protected boolean isEnabled(NamedWaypoint waypoint) - `selectedWaypoints.contains(waypoint)`
    pub fn is_enabled_dyn(__this: &compat::Shared<dyn AbstractWaypointsScreen>, waypoint: &compat::Shared<dyn NamedWaypoint>) -> bool {
        let __this = compat::view_as::<Self, _, Self>(__this, |a| a);
        return compat::read(&*__this, |__t| __t.selected_waypoints.iter().any(|w| <dyn NamedWaypoint>::equals_shared(w, waypoint)));
    }

    /// java: @Override protected void enabledChanged(NamedWaypoint waypoint, boolean enabled)
    pub fn enabled_changed_dyn(__this: &compat::Shared<dyn AbstractWaypointsScreen>, waypoint: &compat::Shared<dyn NamedWaypoint>, enabled: bool) {
        let __this = compat::view_as::<Self, _, Self>(__this, |a| a);
        compat::write(&*__this, |__t| {
            if enabled {
                // java: selectedWaypoints.add(waypoint);
                set_add(&mut __t.selected_waypoints, waypoint);
            } else if let Some(index) = __t.selected_waypoints.iter().position(|w| <dyn NamedWaypoint>::equals_shared(w, waypoint)) {
                // java: selectedWaypoints.remove(waypoint);
                __t.selected_waypoints.remove(index);
            }
        });
    }

    /// java: @SuppressWarnings("DataFlowIssue") @Override public void onClose()
    pub fn on_close(__this: &compat::Shared<Self>) {
        let _ = __this;
        // java: minecraft.gui.setScreen(parent); - the waypoints screen opened this one keeping itself as the parent
        // (`set_screen_keeping_parent`), and the game's stack hands that same object back.
        mcrust::mcrust::modding::game::set_screen_to_parent();
    }
}

impl AbstractWaypointsScreen for WaypointsShareScreen {
    fn abstract_waypoints_screen_base(&self) -> &AbstractWaypointsScreenBase {
        &self.base
    }
    fn abstract_waypoints_screen_base_mut(&mut self) -> &mut AbstractWaypointsScreenBase {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override protected boolean isEnabled(NamedWaypoint waypoint)
    fn is_enabled(&self) -> fn(&compat::Shared<dyn AbstractWaypointsScreen>, &compat::Shared<dyn NamedWaypoint>) -> bool {
        WaypointsShareScreen::is_enabled_dyn
    }

    /// java: @Override protected void enabledChanged(NamedWaypoint waypoint, boolean enabled)
    fn enabled_changed(&self) -> fn(&compat::Shared<dyn AbstractWaypointsScreen>, &compat::Shared<dyn NamedWaypoint>, bool) {
        WaypointsShareScreen::enabled_changed_dyn
    }
}

/// WaypointsShareScreen's `Screen` half (`super`'s fields and widget tree), lent by the game for each call
impl compat::ScreenHalf for WaypointsShareScreen {
    fn screen_half(&mut self) -> &mut Screen {
        &mut self.base.base
    }
    fn screen_parts(&mut self) -> (&mut Screen, Option<&mut compat::WidgetCache>) {
        let __s = &mut self.base;
        (&mut __s.base, Some(&mut __s.__widgets))
    }
}

/// WaypointsShareScreen as the game's screen: `game::set_screen(Box::new(compat::ScreenAdapter::new(..)))`
impl ModScreen for compat::ScreenAdapter<WaypointsShareScreen> {
    fn get_title(&self) -> String {
        self.title()
    }

    /// java: @Override protected void init()
    fn init(&mut self, screen: &mut Screen) {
        self.lend(screen, |__h| WaypointsShareScreen::init(__h))
    }

    /// java: AbstractWaypointsScreen.repositionElements (it does not rebuild the widgets)
    fn reposition_elements(&mut self, screen: &mut Screen) {
        self.lend(screen, |__h| <dyn AbstractWaypointsScreen>::reposition_elements(&WaypointsShareScreen::as_abstract(__h)))
    }

    /// java: Screen.tick (the base body), and the `updateButtons()` a click asked for
    fn tick(&mut self, screen: &mut Screen) {
        self.lend(screen, |__h| <dyn AbstractWaypointsScreen>::tick(&WaypointsShareScreen::as_abstract(__h)))
    }

    /// java: Screen.extractRenderState (the base body) - the island dropdown and the popup container last
    fn extract_render_state(&mut self, screen: &Screen, context: &mut ScreenRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        self.lend_ref(screen, |__h| <dyn AbstractWaypointsScreen>::extract_render_state(&WaypointsShareScreen::as_abstract(__h), context, mouse_x, mouse_y, a))
    }

    /// java: Screen.keyPressed (the base body)
    fn key_pressed(&mut self, screen: &mut Screen, event: &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        self.lend(screen, |_| ());
        if event.is_escape() && self.should_close_on_esc() {
            self.on_close();
            return true;
        }
        false
    }

    /// java: AbstractWaypointsScreen.mouseClicked
    fn mouse_clicked(&mut self, screen: &mut Screen, x: f64, y: f64, button: i32) -> bool {
        self.lend(screen, |__h| <dyn AbstractWaypointsScreen>::mouse_clicked(&WaypointsShareScreen::as_abstract(__h), &abstract_waypoints_screen::click_event(x, y, button), false))
    }

    /// java: Screen.mouseReleased - the popup container while it is dragged
    fn mouse_released(&mut self, screen: &mut Screen, x: f64, y: f64, button: i32) -> bool {
        self.lend(screen, |__h| <dyn AbstractWaypointsScreen>::mouse_released(&WaypointsShareScreen::as_abstract(__h), &abstract_waypoints_screen::click_event(x, y, button)))
    }

    /// java: Screen.mouseDragged - the popup container while it is dragged
    fn mouse_dragged(&mut self, screen: &mut Screen, x: f64, y: f64, button: i32, dx: f64, dy: f64) -> bool {
        self.lend(screen, |__h| <dyn AbstractWaypointsScreen>::mouse_dragged(&WaypointsShareScreen::as_abstract(__h), &abstract_waypoints_screen::click_event(x, y, button), dx, dy))
    }

    /// java: AbstractWaypointsScreen.mouseScrolled
    fn mouse_scrolled(&mut self, screen: &mut Screen, x: f64, y: f64, scroll_x: f64, scroll_y: f64) -> bool {
        self.lend(screen, |__h| <dyn AbstractWaypointsScreen>::mouse_scrolled(&WaypointsShareScreen::as_abstract(__h), x, y, scroll_x, scroll_y))
    }

    /// java: @SuppressWarnings("DataFlowIssue") @Override public void onClose()
    fn on_close(&mut self) {
        self.call(|__h| WaypointsShareScreen::on_close(__h))
    }
}
