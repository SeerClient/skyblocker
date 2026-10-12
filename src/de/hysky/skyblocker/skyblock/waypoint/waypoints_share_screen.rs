//! Ported from de.hysky.skyblocker.skyblock.waypoint.WaypointsShareScreen by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
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
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::abstract_widget::PressLambda;
use mcrust::net::minecraft::client::gui::components::button;
use mcrust::net::minecraft::client::gui::components::toasts::system_toast;
use mcrust::net::minecraft::client::gui::components::tooltip::Tooltip;
use mcrust::net::minecraft::client::gui::gui;
use mcrust::net::minecraft::client::gui::layouts::grid_layout::GridLayout;
use mcrust::net::minecraft::client::gui::layouts::grid_layout::RowHelper;
use mcrust::net::minecraft::client::gui::layouts::layout_element::LayoutElement;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::client::input::mouse_button_info::MouseButtonInfo;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::component::Component as _;
use crate::de::hysky::skyblocker::skyblock::waypoint::waypoints::named_waypoint_equals;

/// java: public class WaypointsShareScreen extends AbstractWaypointsScreen<WaypointsScreen>
pub struct WaypointsShareScreen {
    /// java: extends AbstractWaypointsScreen
    pub base: AbstractWaypointsScreenBase<compat::Shared<WaypointsScreen>>,
    /// java: private final Set<NamedWaypoint> selectedWaypoints = new HashSet<>();
    /// A `HashSet` keyed by `NamedWaypoint.equals` / `hashCode` (structural, not identity): the members are kept in a
    /// list and asked with `named_waypoint_equals`.
    pub selected_waypoints: Vec<compat::Shared<dyn NamedWaypoint>>,
    /// java: private boolean overrideLocation = false;
    pub override_location: bool,
    /// java: private boolean sortWaypoints = false;
    pub sort_waypoints: bool,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl std::ops::Deref for WaypointsShareScreen {
    type Target = AbstractWaypointsScreenBase<compat::Shared<WaypointsScreen>>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for WaypointsShareScreen {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl WaypointsShareScreen {
    /// java: protected WaypointsShareScreen(WaypointsScreen parent, Multimap<Location, WaypointGroup> waypoints)
    pub fn new(mut parent: compat::Shared<WaypointsScreen>, mut waypoints: compat::Shared<crate::de::hysky::skyblocker::skyblock::waypoint::waypoints::LocationMultimap>) -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            base: AbstractWaypointsScreenBase::<compat::Shared<WaypointsScreen>>::new_4(component::translatable("skyblocker.waypoints.shareWaypoints"), parent.clone(), waypoints.clone(), compat::read(&parent, |__t| __t.island.clone()).unwrap()),
            self_ref: std::sync::Weak::new(),
            selected_waypoints: Vec::new(),
            override_location: false,
            sort_waypoints: false,
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        __this
    }

    /// java: private void showErrorToast()
    pub fn show_error_toast(__this: &compat::Shared<Self>) {
        // TODO(fabric2seer): `add_or_update` takes 5 argument(s) in the port (`toast_manager: &mut ToastManager, font: &FontSet, id: SystemToastId, title: &FormattedText, message: Option<&FormattedText>`), Java passes 4
        // rust: system_toast::add_or_update(&mut compat::minecraft().gui.toast_manager(), waypoints::WAYPOINTS_TOAST_TYPE.get_mut(), component::translatable("skyblocker.waypoints.importError"), &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::translatable("skyblocker.waypoints.importErrorText")));
    }

    /// java: @SuppressWarnings("DataFlowIssue") @Override public void onClose()
    pub fn on_close(__this: &compat::Shared<Self>) {
        // `minecraft.gui.setScreen(parent)`: the waypoints screen opened this one keeping itself as the parent
        // (`set_screen_keeping_parent`), and the game's stack hands that same object back.
        mcrust::mcrust::modding::game::set_screen_to_parent();
    }

}

impl AbstractWaypointsScreen<compat::Shared<crate::de::hysky::skyblocker::skyblock::waypoint::waypoints_screen::WaypointsScreen>> for WaypointsShareScreen {
    fn abstract_waypoints_screen_base(&self) -> &AbstractWaypointsScreenBase<compat::Shared<crate::de::hysky::skyblocker::skyblock::waypoint::waypoints_screen::WaypointsScreen>> {
        &self.base
    }
    fn abstract_waypoints_screen_base_mut(&mut self) -> &mut AbstractWaypointsScreenBase<compat::Shared<crate::de::hysky::skyblocker::skyblock::waypoint::waypoints_screen::WaypointsScreen>> {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override protected void init()
    fn init(&mut self) {
        self.init_super();
        let mut row_spacing: i32 = 2;
        let mut column_spacing: i32 = 5;
        let mut grid_widget: GridLayout = GridLayout::new_default();
        grid_widget.column_spacing(column_spacing);
        grid_widget.row_spacing(row_spacing);
        let mut adder: RowHelper = grid_widget.create_row_helper(2);
        // TODO(fabric2seer): static `Checkbox.builder` has no Rust counterpart in the port
        // TODO(fabric2seer): `Builder.maxWidth` has no Rust method in the port (looked for `max_width` in Checkbox and its supertypes)
        // TODO(fabric2seer): `Builder.onValueChange` has no Rust method in the port (looked for `on_value_change` in Checkbox and its supertypes)
        // TODO(fabric2seer): `Builder.tooltip` has no Rust method in the port (looked for `tooltip` in Checkbox and its supertypes)
        // TODO(fabric2seer): `Builder.build` has no Rust method in the port (looked for `build` in Checkbox and its supertypes)
        // TODO(fabric2seer): the port's `add_child` ports this Java method with other parameters (grid, child for Java's widget)
        // TODO(fabric2seer): `add_child` takes 2 argument(s) in the port (`grid: &mut GridLayout, child: LayoutElement`), Java passes 1
        // rust: adder.add_child(&mut (Checkbox::builder(component::translatable("skyblocker.waypoints.importOptions.overrideLocation"), compat::font())).max_width(mcrust::net::minecraft::client::gui::components::button::DEFAULT_WIDTH).on_value_change(&Arc::new(move |_, checked| { self.override_location = checked.clone(); self.override_location })).tooltip(&Tooltip::create(component::translatable("skyblocker.waypoints.importOptions.overrideLocation.tooltip").get_string())).build()).unwrap();
        // TODO(fabric2seer): static `Checkbox.builder` has no Rust counterpart in the port
        // TODO(fabric2seer): `Builder.maxWidth` has no Rust method in the port (looked for `max_width` in Checkbox and its supertypes)
        // TODO(fabric2seer): `Builder.onValueChange` has no Rust method in the port (looked for `on_value_change` in Checkbox and its supertypes)
        // TODO(fabric2seer): `Builder.tooltip` has no Rust method in the port (looked for `tooltip` in Checkbox and its supertypes)
        // TODO(fabric2seer): `Builder.build` has no Rust method in the port (looked for `build` in Checkbox and its supertypes)
        // TODO(fabric2seer): the port's `add_child` ports this Java method with other parameters (grid, child for Java's widget)
        // TODO(fabric2seer): `add_child` takes 2 argument(s) in the port (`grid: &mut GridLayout, child: LayoutElement`), Java passes 1
        // rust: adder.add_child(&mut (Checkbox::builder(component::translatable("skyblocker.waypoints.importOptions.sortWaypoints"), compat::font())).max_width(mcrust::net::minecraft::client::gui::components::button::DEFAULT_WIDTH).on_value_change(&Arc::new(move |_, checked| { self.sort_waypoints = checked.clone(); self.sort_waypoints })).tooltip(&Tooltip::create(component::translatable("skyblocker.waypoints.importOptions.sortWaypoints.tooltip").get_string())).build()).unwrap();
        // TODO(fabric2seer): the port's `add_child` ports this Java method with other parameters (grid, child for Java's widget)
        // TODO(fabric2seer): `add_child` takes 2 argument(s) in the port (`grid: &mut GridLayout, child: LayoutElement`), Java passes 1
        // rust: adder.add_child(&mut ((button::builder(component::translatable("skyblocker.waypoints.importWaypointsSkyblocker").get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| {
        // rust: // TODO(fabric2seer): `get_clipboard` takes 1 argument(s) in the port (`clipboard: &mut dyn ClipboardAccess`), Java passes 0
        // rust: // java: try {..} catch (Exception e) - the body runs inline: a Rust body does not throw checked exceptions (a panic in it propagates)
        // rust: {
        // rust: let Some(mut waypoint_groups) = Waypoints::from_skyblocker(compat::minecraft().keyboard_handler.get_clipboard(), self.island.clone().unwrap()) else {
        // rust: WaypointsShareScreen::show_error_toast(&self.self_ref.upgrade().unwrap());
        // rust: return;
        // rust: };
        // rust: for waypoint_group in waypoint_groups.iter() {
        // rust: if self.override_location {
        // rust: *waypoint_group = WaypointGroup::with_island(waypoint_group, self.island.clone().unwrap());
        // rust: }
        // rust: if self.sort_waypoints {
        // rust: *waypoint_group = WaypointGroup::sort_waypoints(waypoint_group, named_waypoint::NAME_COMPARATOR.get_mut().clone());
        // rust: }
        // rust: self.selected_waypoints.extend(WaypointGroup::waypoints(waypoint_group).into_iter());
        // rust: // TODO(fabric2seer): `.put(..)` on a Multimap
        // rust: //   java: waypoints.put(waypointGroup.island(), waypointGroup);
        // rust: }
        // rust: WaypointsListWidget::update_entries(self.waypoints_list_widget.as_ref().unwrap());
        // rust: // TODO(fabric2seer): static `List.size` has no Rust counterpart in the port
        // rust: // TODO(fabric2seer): `add_or_update` takes 5 argument(s) in the port (`toast_manager: &mut ToastManager, font: &FontSet, id: SystemToastId, title: &FormattedText, message: Option<&FormattedText>`), Java passes 4
        // rust: // rust: system_toast::add_or_update(&mut compat::minecraft().gui.toast_manager(), waypoints::WAYPOINTS_TOAST_TYPE.get_mut(), component::translatable("skyblocker.waypoints.importSuccess"), &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::translatable_args("skyblocker.waypoints.importSuccessText", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of(waypoint_groups.iter().cloned().map(|__a0| WaypointGroup::waypoints(&__a0)).map(|__a0| List::size(__a0.clone())).sum::<i32>().to_string()), mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of((waypoint_groups.len() as i32).to_string())])));
        // rust: }
        // rust: // TODO(fabric2seer): the `catch (Exception e)` block is not reached in Rust - the try body returns, breaks or continues, so it cannot run inside `catch_unwind`
        // rust: //   java: {
        // rust: //   java: 				Waypoints.LOGGER.error("[Skyblocker Waypoints] Encountered exception while parsing Skyblocker waypoint data", e);
        // rust: //   java: 				showErrorToast();
        // rust: //   java: 			}
        // rust: })))).tooltip_text(Some(Tooltip::create(component::translatable("skyblocker.waypoints.importWaypointsSkyblocker.tooltip").get_string()).message.clone()))).build()).unwrap();
        // TODO(fabric2seer): the port's `add_child` ports this Java method with other parameters (grid, child for Java's widget)
        // TODO(fabric2seer): `add_child` takes 2 argument(s) in the port (`grid: &mut GridLayout, child: LayoutElement`), Java passes 1
        // rust: adder.add_child(&mut ((button::builder(component::translatable("skyblocker.waypoints.exportWaypointsSkyblocker").get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| {
        // rust: // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
        // rust: let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        // rust: // TODO(fabric2seer): `.values(..)` on a Multimap
        // rust: //   java: List<WaypointGroup> waypointGroups = waypoints.values().stream().filter(waypointGroup -> waypointGroup.island().equals(island)).map(waypointGroup -> waypointGroup.filterWaypoints(selectedWaypoints::contains)).filter(waypointGroup -> !waypointGroup.waypoints().isEmpty()).toList();
        // rust: let mut waypoint_groups: Vec<compat::Shared<WaypointGroup>> = todo!("fabric2seer: the initialiser of `waypointGroups` (TODO above)");
        // rust: // TODO(fabric2seer): `set_clipboard` takes 2 argument(s) in the port (`clipboard: &mut dyn ClipboardAccess, value: &str`), Java passes 1
        // rust: // rust: compat::minecraft().keyboard_handler.set_clipboard(&mut Waypoints::to_skyblocker(&waypoint_groups));
        // rust: // TODO(fabric2seer): static `List.size` has no Rust counterpart in the port
        // rust: // TODO(fabric2seer): `add_or_update` takes 5 argument(s) in the port (`toast_manager: &mut ToastManager, font: &FontSet, id: SystemToastId, title: &FormattedText, message: Option<&FormattedText>`), Java passes 4
        // rust: // rust: system_toast::add_or_update(&mut compat::minecraft().gui.toast_manager(), waypoints::WAYPOINTS_TOAST_TYPE.get_mut(), component::translatable("skyblocker.waypoints.exportSuccess"), &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::translatable_args("skyblocker.waypoints.exportSuccessText", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of(waypoint_groups.iter().cloned().map(|__a0| WaypointGroup::waypoints(&__a0)).map(|__a0| List::size(__a0.clone())).sum::<i32>().to_string()), mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of((waypoint_groups.len() as i32).to_string())])));
        // rust: }));
        // rust: if let Err(e) = __result {
        // rust: logger().error(&format!("[Skyblocker Waypoints] Encountered exception while serializing Skyblocker waypoint data"));
        // rust: // TODO(fabric2seer): `add_or_update` takes 5 argument(s) in the port (`toast_manager: &mut ToastManager, font: &FontSet, id: SystemToastId, title: &FormattedText, message: Option<&FormattedText>`), Java passes 4
        // rust: // rust: system_toast::add_or_update(&mut compat::minecraft().gui.toast_manager(), waypoints::WAYPOINTS_TOAST_TYPE.get_mut(), component::translatable("skyblocker.waypoints.exportError"), &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::translatable("skyblocker.waypoints.exportErrorText")));
        // rust: }
        // rust: })))).tooltip_text(Some(Tooltip::create(component::translatable("skyblocker.waypoints.exportWaypointsSkyblocker.tooltip").get_string()).message.clone()))).build()).unwrap();
        // TODO(fabric2seer): the port's `add_child` ports this Java method with other parameters (grid, child for Java's widget)
        // TODO(fabric2seer): `add_child` takes 2 argument(s) in the port (`grid: &mut GridLayout, child: LayoutElement`), Java passes 1
        // rust: adder.add_child(&mut ((button::builder(component::translatable("skyblocker.waypoints.importWaypointsSkytils").get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| {
        // rust: // TODO(fabric2seer): `get_clipboard` takes 1 argument(s) in the port (`clipboard: &mut dyn ClipboardAccess`), Java passes 0
        // rust: // java: try {..} catch (Exception e) - the body runs inline: a Rust body does not throw checked exceptions (a panic in it propagates)
        // rust: {
        // rust: let Some(mut waypoint_groups) = Waypoints::from_skytils(compat::minecraft().keyboard_handler.get_clipboard(), self.island.clone().unwrap()) else {
        // rust: WaypointsShareScreen::show_error_toast(&self.self_ref.upgrade().unwrap());
        // rust: return;
        // rust: };
        // rust: for waypoint_group in waypoint_groups.iter() {
        // rust: if self.override_location {
        // rust: *waypoint_group = WaypointGroup::with_island(waypoint_group, self.island.clone().unwrap());
        // rust: }
        // rust: if self.sort_waypoints {
        // rust: *waypoint_group = WaypointGroup::sort_waypoints(waypoint_group, named_waypoint::NAME_COMPARATOR.get_mut().clone());
        // rust: }
        // rust: self.selected_waypoints.extend(WaypointGroup::waypoints(waypoint_group).into_iter());
        // rust: // TODO(fabric2seer): `.put(..)` on a Multimap
        // rust: //   java: waypoints.put(waypointGroup.island(), waypointGroup);
        // rust: }
        // rust: WaypointsListWidget::update_entries(self.waypoints_list_widget.as_ref().unwrap());
        // rust: // TODO(fabric2seer): static `List.size` has no Rust counterpart in the port
        // rust: // TODO(fabric2seer): `add_or_update` takes 5 argument(s) in the port (`toast_manager: &mut ToastManager, font: &FontSet, id: SystemToastId, title: &FormattedText, message: Option<&FormattedText>`), Java passes 4
        // rust: // rust: system_toast::add_or_update(&mut compat::minecraft().gui.toast_manager(), waypoints::WAYPOINTS_TOAST_TYPE.get_mut(), component::translatable("skyblocker.waypoints.importSuccess"), &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::translatable_args("skyblocker.waypoints.importSuccessText", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of(waypoint_groups.iter().cloned().map(|__a0| WaypointGroup::waypoints(&__a0)).map(|__a0| List::size(__a0.clone())).sum::<i32>().to_string()), mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of((waypoint_groups.len() as i32).to_string())])));
        // rust: }
        // rust: // TODO(fabric2seer): the `catch (Exception e)` block is not reached in Rust - the try body returns, breaks or continues, so it cannot run inside `catch_unwind`
        // rust: //   java: {
        // rust: //   java: 				Waypoints.LOGGER.error("[Skyblocker Waypoints] Encountered exception while parsing Skytils waypoint data", e);
        // rust: //   java: 				showErrorToast();
        // rust: //   java: 			}
        // rust: })))).tooltip_text(Some(Tooltip::create(component::translatable("skyblocker.waypoints.importWaypointsSkytils.tooltip").get_string()).message.clone()))).build()).unwrap();
        // TODO(fabric2seer): the port's `add_child` ports this Java method with other parameters (grid, child for Java's widget)
        // TODO(fabric2seer): `add_child` takes 2 argument(s) in the port (`grid: &mut GridLayout, child: LayoutElement`), Java passes 1
        // rust: adder.add_child(&mut ((button::builder(component::translatable("skyblocker.waypoints.importWaypointsSnoopy").get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| {
        // rust: // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
        // rust: let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        // rust: // TODO(fabric2seer): `get_clipboard` takes 1 argument(s) in the port (`clipboard: &mut dyn ClipboardAccess`), Java passes 0
        // rust: // rust: let mut waypoint_group: compat::Shared<WaypointGroup> = Waypoints::from_coleweight_json(&compat::minecraft().keyboard_handler.get_clipboard(), self.island.clone().unwrap());
        // rust: let mut waypoint_group: compat::Shared<WaypointGroup> = todo!("fabric2seer: the initialiser of `waypointGroup` (TODO above)");
        // rust: if self.override_location {
        // rust: waypoint_group = WaypointGroup::with_island(&waypoint_group, self.island.clone().unwrap());
        // rust: }
        // rust: if self.sort_waypoints {
        // rust: waypoint_group = WaypointGroup::sort_waypoints(&waypoint_group, named_waypoint::NAME_COMPARATOR.get_mut().clone());
        // rust: }
        // rust: self.selected_waypoints.extend(WaypointGroup::waypoints(&waypoint_group).into_iter());
        // rust: // TODO(fabric2seer): `.put(..)` on a Multimap
        // rust: //   java: waypoints.put(waypointGroup.island(), waypointGroup);
        // rust: WaypointsListWidget::update_entries(self.waypoints_list_widget.as_ref().unwrap());
        // rust: // TODO(fabric2seer): `add_or_update` takes 5 argument(s) in the port (`toast_manager: &mut ToastManager, font: &FontSet, id: SystemToastId, title: &FormattedText, message: Option<&FormattedText>`), Java passes 4
        // rust: // rust: system_toast::add_or_update(&mut compat::minecraft().gui.toast_manager(), waypoints::WAYPOINTS_TOAST_TYPE.get_mut(), component::translatable("skyblocker.waypoints.importSuccess"), &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::translatable_args("skyblocker.waypoints.importSuccessText", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of((WaypointGroup::waypoints(&waypoint_group).len() as i32).to_string()), mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of(1.to_string())])));
        // rust: }));
        // rust: if let Err(e) = __result {
        // rust: logger().error(&format!("[Skyblocker Waypoints] Encountered exception while parsing Snoopy waypoint data"));
        // rust: WaypointsShareScreen::show_error_toast(&self.self_ref.upgrade().unwrap());
        // rust: }
        // rust: })))).tooltip_text(Some(Tooltip::create(component::translatable("skyblocker.waypoints.importWaypointsSnoopy.tooltip").get_string()).message.clone()))).build()).unwrap();
        // TODO(fabric2seer): the port's `add_child_width` ports this Java method with other parameters (grid, child, column_width for Java's widget, column_width)
        // TODO(fabric2seer): `add_child_width` takes 3 argument(s) in the port (`grid: &mut GridLayout, child: LayoutElement, column_width: i32`), Java passes 2
        // rust: adder.add_child_width(&mut (button::builder(mcrust::net::minecraft::network::chat::common_components::gui_done().get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { WaypointsShareScreen::on_close(&self.self_ref.upgrade().unwrap()); })))).width(((mcrust::net::minecraft::client::gui::components::button::DEFAULT_WIDTH).wrapping_mul(2)).wrapping_add(column_spacing)).build(), 2).unwrap();
        self.layout.as_mut().unwrap().add_to_footer(LayoutElement::Grid(grid_widget.clone()));
        let mut rows: i32 = 4;
        self.layout.as_mut().unwrap().set_footer_height(((20i32.wrapping_mul(rows)).wrapping_add(row_spacing.wrapping_mul(rows.wrapping_sub(1)))).wrapping_add(8));
        self.late_init();
    }

    /// java: @Override protected boolean isEnabled(NamedWaypoint waypoint)
    fn is_enabled(&self, waypoint: &compat::Shared<dyn NamedWaypoint>) -> bool {
        return self.selected_waypoints.iter().any(|w| named_waypoint_equals(w, waypoint));
    }

    /// java: @Override protected void enabledChanged(NamedWaypoint waypoint, boolean enabled)
    fn enabled_changed(&mut self, waypoint: &compat::Shared<dyn NamedWaypoint>, mut enabled: bool) {
        if enabled {
            // `Set.add`: only when no equal member is there.
            if !self.selected_waypoints.iter().any(|w| named_waypoint_equals(w, waypoint)) {
                self.selected_waypoints.push(waypoint.clone());
            }
        } else if let Some(index) = self.selected_waypoints.iter().position(|w| named_waypoint_equals(w, waypoint)) {
            self.selected_waypoints.remove(index);
        }
    }

    /// `this::islandChanged` for the island dropdown, through this screen's own handle.
    fn island_changed_callback(&self) -> Arc<dyn Fn(Location)> {
        let __this_w = compat::RenderThread::new(self.self_ref.clone());
        Arc::new(move |location| {
            if let Some(__this) = __this_w.get().upgrade() {
                compat::write(&__this, |__t| __t.island_changed(location));
            }
        })
    }

}

/// WaypointsShareScreen's `Screen` half (`super`'s fields and widget tree), lent by the game for each call
impl compat::ScreenHalf for WaypointsShareScreen {
    fn screen_half(&mut self) -> &mut mcrust::net::minecraft::client::gui::screens::screen::Screen {
        &mut self.base.base
    }
    fn screen_parts(&mut self) -> (&mut mcrust::net::minecraft::client::gui::screens::screen::Screen, Option<&mut compat::WidgetCache>) {
        let __s = &mut self.base;
        (&mut __s.base, Some(&mut __s.__widgets))
    }
}

/// WaypointsShareScreen as the game's screen: `game::set_screen(Box::new(compat::ScreenAdapter::new(..)))`
impl ModScreen for compat::ScreenAdapter<WaypointsShareScreen> {
    fn get_title(&self) -> String {
        self.title()
    }

    /// java: Screen.tick (the base body)
    fn tick(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        self.lend(screen, |_| ())
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

    /// java: @SuppressWarnings("DataFlowIssue") @Override public void onClose()
    fn on_close(&mut self) {
        self.call(|__h| WaypointsShareScreen::on_close(__h))
    }

}
