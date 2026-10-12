//! Ported from de.hysky.skyblocker.skyblock.waypoint.WaypointsListWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::waypoint::waypoints::{named_waypoint_equals, waypoint_group_equals, LocationMultimap};
use crate::de::hysky::skyblocker::utils::waypoint::waypoint_group;
use mcrust::net::minecraft::client::gui::components::abstract_widget::WidgetKind;
use mcrust::net::minecraft::client::gui::layouts::layout_settings::LayoutSettings;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::waypoint::abstract_waypoints_screen::AbstractWaypointsScreen;
use crate::de::hysky::skyblocker::utils::location::Location;
use crate::de::hysky::skyblocker::utils::render::gui::argb_text_input::ARGBTextInput;
use crate::de::hysky::skyblocker::utils::render::gui::color_picker_widget::ColorPickerWidget;
use crate::de::hysky::skyblocker::utils::render::gui::cycling_icon_button_widget::CyclingIconButtonWidget;
use crate::de::hysky::skyblocker::utils::render::gui::cycling_icon_button_widget::Icon;
use crate::de::hysky::skyblocker::utils::render::gui::filtered_edit_box::FilteredEditBox;
use crate::de::hysky::skyblocker::utils::render::gui::filtered_edit_box::FilteredEditBoxBase;
use crate::de::hysky::skyblocker::utils::render::gui::noop_input;
use crate::de::hysky::skyblocker::utils::waypoint::named_waypoint::NamedWaypoint;
use crate::de::hysky::skyblocker::utils::waypoint::waypoint::Type;
use crate::de::hysky::skyblocker::utils::waypoint::waypoint::Waypoint;
use crate::de::hysky::skyblocker::utils::waypoint::waypoint_group::WaypointGroup;
use mcrust::com::mojang::blaze3d::platform::cursor::cursor_types::CursorTypes;
use mcrust::mcrust::modding::widgets::ModList;
use mcrust::mcrust::modding::widgets::ModListEntry;
use mcrust::mcrust::modding::widgets::ModSelectionList;
use mcrust::mcrust::modding::widgets::ModWidget;
use mcrust::mcrust::modding::widgets::WidgetRenderContext;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::components::abstract_selection_list::AbstractSelectionList;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::abstract_widget::PressLambda;
use mcrust::net::minecraft::client::gui::components::button;
use mcrust::net::minecraft::client::gui::components::checkbox::Checkbox;
use mcrust::net::minecraft::client::gui::components::edit_box::EditBox;
use mcrust::net::minecraft::client::gui::components::edit_box::Responder;
use mcrust::net::minecraft::client::gui::components::edit_box::ResponderLambda;
use mcrust::net::minecraft::client::gui::components::events::gui_event_listener::GuiEventListener;
use mcrust::net::minecraft::client::gui::components::sprite_icon_button;
use mcrust::net::minecraft::client::gui::components::tooltip::Tooltip;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::layouts::frame_layout::FrameLayout;
use mcrust::net::minecraft::client::gui::layouts::layout_element::LayoutElement;
use mcrust::net::minecraft::client::gui::layouts::linear_layout::LinearLayout;
use mcrust::net::minecraft::client::gui::layouts::spacer_element::SpacerElement;
use mcrust::net::minecraft::client::gui::narration::narratable_entry::NarratableEntry;
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use mcrust::net::minecraft::client::gui::navigation::screen_direction::ScreenDirection;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::input_with_modifiers::InputWithModifiers;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::client::input::mouse_button_info::MouseButtonInfo;
use mcrust::net::minecraft::core::block_pos::BlockPos;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::util::argb;
use mcrust::net::minecraft::world::phys::block_hit_result::BlockHitResult;
use mcrust::net::minecraft::client::gui::components::renderable::Renderable as _;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final Identifier DELETE_ICON = SkyblockerMod.id("trash_can");
pub static DELETE_ICON: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("trash_can"));
/// java: private static final int ICON_WIDTH = 12, ICON_HEIGHT = 15;
pub const ICON_WIDTH: i32 = 12;
/// java: private static final int ICON_WIDTH = 12, ICON_HEIGHT = 15;
pub const ICON_HEIGHT: i32 = 15;
/// java: private static final Identifier INSERT_TEXTURE = SkyblockerMod.id("insert_button");
pub static INSERT_TEXTURE: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("insert_button"));
/// java: private static final Identifier INSERT_HIGHLIGHTED_TEXTURE = SkyblockerMod.id("insert_button_highlighted");
pub static INSERT_HIGHLIGHTED_TEXTURE: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("insert_button_highlighted"));

/// java: public class WaypointsListWidget extends ContainerObjectSelectionList<WaypointsListWidget.AbstractWaypointEntry>
pub struct WaypointsListWidget {
    /// java: extends net.minecraft.client.gui.components.ContainerObjectSelectionList
    pub base: ModSelectionList,
    /// java: private final AbstractWaypointsScreen<?> screen;
    pub screen: compat::Shared<dyn AbstractWaypointsScreen<compat::Object>>,
    /// java: private Location island;
    pub island: Option<Location>,
    /// java: private List<WaypointGroup> waypoints;
    /// Java's list is `screen.waypoints.get(island)`, the LIVE value collection of the screen's map (adding a group here
    /// adds it to the map): the map is held and the island's list reached through it (`waypoint_groups` /
    /// `with_waypoint_groups`).
    pub waypoints: Option<compat::Shared<LocationMultimap>>,
    /// java: private @Nullable InsertPosition insertPosition = null;
    pub insert_position: Option<InsertPosition>,
    /// java: private final Set<WaypointGroup> collapsedGroups = new ReferenceOpenHashSet<>();
    /// A `ReferenceOpenHashSet` (identity): the members, asked with `contains_group` (`Arc::ptr_eq`).
    pub collapsed_groups: Vec<compat::Shared<WaypointGroup>>,
}

impl std::ops::Deref for WaypointsListWidget {
    type Target = AbstractSelectionList;
    fn deref(&self) -> &Self::Target {
        &self.base.list
    }
}

impl std::ops::DerefMut for WaypointsListWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base.list
    }
}

/// `collapsedGroups.contains(group)`: identity (`ReferenceOpenHashSet`).
fn contains_group(set: &[compat::Shared<WaypointGroup>], group: &compat::Shared<WaypointGroup>) -> bool {
    set.iter().any(|g| Arc::ptr_eq(g, group))
}

/// `children().indexOf(entry)`: rows are compared by identity (`Entry` does not override `equals`); -1 when absent.
fn index_of_entry(children: &[compat::Shared<dyn AbstractWaypointEntry>], entry: &compat::Shared<dyn AbstractWaypointEntry>) -> i32 {
    children.iter().position(|e| std::ptr::addr_eq(Arc::as_ptr(e), Arc::as_ptr(entry))).map_or(-1, |i| i as i32)
}

/// `group.waypoints().indexOf(waypoint)`: `List.indexOf` asks `NamedWaypoint.equals`; -1 when absent.
fn index_of_waypoint(group: &compat::Shared<WaypointGroup>, waypoint: &compat::Shared<dyn NamedWaypoint>) -> i32 {
    compat::read(group, |g| g.waypoints.iter().position(|w| named_waypoint_equals(w, waypoint)).map_or(-1, |i| i as i32))
}

impl WaypointsListWidget {
    /// java: public WaypointsListWidget(Minecraft client, AbstractWaypointsScreen<?> screen, int width, int height, int y, int itemHeight)
    pub fn new(client: &mut mcrust::net::minecraft::client::minecraft::Minecraft, mut screen: compat::Shared<dyn AbstractWaypointsScreen<compat::Object>>, mut width: i32, mut height: i32, mut y: i32, mut item_height: i32) -> compat::Shared<Self> {
        let __init_screen = screen.clone();
        let __this = compat::shared(Self {
            base: mcrust::mcrust::modding::widgets::ModSelectionList::new(mcrust::mcrust::modding::widgets::ListKind::Container, width, height, y, item_height),
            screen: __init_screen,
            island: None,
            waypoints: None,
            insert_position: None,
            collapsed_groups: Vec::new(),
        });
        WaypointsListWidget::set_island(&__this, compat::read(&screen, |__t| __t.abstract_waypoints_screen_base().island.clone()).unwrap());
        __this
    }

    /// java: @Override public int getRowWidth()
    pub fn get_row_width(__this: &compat::Shared<Self>) -> i32 {
        return 340;
    }

    /// java: @Override protected int scrollBarX()
    pub fn scroll_bar_x(__this: &compat::Shared<Self>) -> i32 {
        return compat::super_answer::<i32>();
    }

    /// java: Optional<WaypointGroupEntry> getGroup()
    pub fn get_group(__this: &compat::Shared<Self>) -> Option<compat::Shared<WaypointGroupEntry>> {
        let selected = compat::read(__this, |__t| { let __l = &__t.base; __l.get_selected().and_then(|__i| compat::entry_at::<dyn AbstractWaypointEntry>(__l, __i)) });
        let Some(selected) = selected else { return None };
        if let Some(group_entry) = compat::try_view::<WaypointGroupEntry, _>(&selected) {
            return Some((&*group_entry).clone());
        } else if let Some(waypoint_entry) = compat::try_view::<WaypointEntry, _>(&selected) {
            return compat::read(&*waypoint_entry, |__t| __t.group_entry.clone());
        }
        return None;
    }

    /// java: void setIsland(Location island)
    pub fn set_island(__this: &compat::Shared<Self>, mut island: Location) {
        { let __v = Some(island); compat::write(__this, |__t| __t.island = __v); }
        // `waypoints = (List<WaypointGroup>) screen.waypoints.get(island)`: the screen's map, the island's list read through it.
        let __screen = compat::read(__this, |__t| __t.screen.clone());
        let __map = compat::read(&__screen, |__t| __t.abstract_waypoints_screen_base().waypoints.clone());
        compat::write(__this, |__t| __t.waypoints = __map);
        let __groups = WaypointsListWidget::waypoint_groups(__this);
        compat::write(__this, |__t| {
            __t.collapsed_groups.clear();
            __t.collapsed_groups.extend(__groups);
        });
        WaypointsListWidget::update_entries(__this);
    }

    /// `waypoints` read: the island's groups in the screen's map.
    pub fn waypoint_groups(__this: &compat::Shared<Self>) -> Vec<compat::Shared<WaypointGroup>> {
        let (__map, __island) = compat::read(__this, |__t| (__t.waypoints.clone(), __t.island));
        compat::read(&__map.expect("NullPointerException: waypoints"), |__m| __m.get(__island.unwrap()))
    }

    /// `waypoints` changed: the island's live list in the screen's map.
    pub fn with_waypoint_groups<R>(__this: &compat::Shared<Self>, f: impl FnOnce(&mut Vec<compat::Shared<WaypointGroup>>) -> R) -> R {
        let (__map, __island) = compat::read(__this, |__t| (__t.waypoints.clone(), __t.island));
        compat::write(&__map.expect("NullPointerException: waypoints"), |__m| f(__m.get_mut(__island.unwrap())))
    }

    /// `children()`: the rows' entry objects, in order.
    fn children(__this: &compat::Shared<Self>) -> Vec<compat::Shared<dyn AbstractWaypointEntry>> {
        compat::read(__this, |__t| compat::list_entries::<dyn AbstractWaypointEntry>(&__t.base))
    }

    /// java: void addWaypointGroupAfterSelected()
    pub fn add_waypoint_group_after_selected(__this: &compat::Shared<Self>) {
        let mut group_entry: compat::Shared<WaypointGroupEntry> = WaypointGroupEntry::new();
        let mut selected_group_entry_optional: Option<compat::Shared<WaypointGroupEntry>> = WaypointsListWidget::get_group(__this);
        let mut index: i32 = (WaypointsListWidget::waypoint_groups(__this).len() as i32);
        if selected_group_entry_optional.is_some() {
            let mut selected_group_entry: compat::Shared<WaypointGroupEntry> = selected_group_entry_optional.clone().unwrap();
            // `waypoints.indexOf(selectedGroupEntry.group)`: `List.indexOf` asks `WaypointGroup.equals`.
            let selected_group = compat::read(&selected_group_entry, |__t| __t.group.clone()).unwrap();
            index = (WaypointsListWidget::waypoint_groups(__this).iter().position(|e| waypoint_group_equals(e, &selected_group)).map_or(-1, |i| i as i32)).wrapping_add(1);
        }
        let __h0 = (index) as usize;
        let __h1 = compat::read(&group_entry, |__t| __t.group.clone()).unwrap();
        WaypointsListWidget::with_waypoint_groups(__this, |__w| __w.insert(__h0, __h1));
        WaypointsListWidget::update_entries(__this);
    }

    /// java: @Override protected void extractListItems(GuiGraphicsExtractor context, int mouseX, int mouseY, float deltaTicks)
    pub fn extract_list_items(__this: &compat::Shared<Self>, context: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut delta_ticks: f32) {
        compat::call_super();
        { let __v = None; compat::write(__this, |__t| __t.insert_position = __v); }
        let mut insert_button_y: i32;
        let mut position: i32;
        let mut group_entry: compat::Shared<WaypointGroupEntry>;
        let children = WaypointsListWidget::children(__this);
        // `getEntryAtPosition(getRowLeft() + getRowWidth() / 2d, mouseY)`
        let hovered_index: Option<usize> = { let (__q0, __q1,) = (((compat::read(__this, |__t| __t.get_row_left()) as f64) + ((WaypointsListWidget::get_row_width(__this) as f64) / 2.0)), (mouse_y) as f64,); compat::read(__this, |__t| __t.get_entry_at_position(__q0, __q1)) };
        let mut hovered_entry: Option<compat::Shared<dyn AbstractWaypointEntry>> = hovered_index.and_then(|__i| children.get(__i).cloned());
        let collapsed_groups = compat::read(__this, |__t| __t.collapsed_groups.clone());
        if hovered_entry.is_none() {
            if children.is_empty() {
                return;
            }
            let mut row_bottom: i32 = { let (__q0,) = (((children.len() as i32).wrapping_sub(1)) as usize,); compat::read(__this, |__t| __t.get_row_bottom(__q0)) };
            if mouse_y >= compat::read(__this, |__t| __t.get_bottom()) || mouse_y < row_bottom {
                return;
            }
            insert_button_y = row_bottom;
            // `switch (children().getLast())`
            let last = children[children.len() - 1].clone();
            if let Some(__view1) = compat::try_view::<WaypointEntry, _>(&last) {
                group_entry = compat::read((&*__view1), |__t| __t.group_entry.clone()).unwrap();
                position = ((children.len() as i32).wrapping_sub(index_of_entry(&children, &(group_entry.clone() as compat::Shared<dyn AbstractWaypointEntry>)))).wrapping_sub(1);
            } else if let Some(__view2) = compat::try_view::<WaypointGroupEntry, _>(&last) && !(contains_group(&collapsed_groups, compat::read((&*__view2), |__t| __t.group.clone()).as_ref().unwrap())) {
                group_entry = (&*__view2).clone();
                position = 0;
            } else {
                return;
            }
        } else if let Some(__sel3) = hovered_entry.as_ref() && let Some(__view4) = compat::try_view::<WaypointEntry, _>(__sel3) {
            let mut i: i32 = index_of_entry(&children, __sel3);
            let mut row_top: i32 = { let (__q0,) = ((i) as usize,); compat::read(__this, |__t| __t.get_row_top(__q0)) };
            let mut row_bottom: i32 = { let (__q0,) = ((i) as usize,); compat::read(__this, |__t| __t.get_row_bottom(__q0)) };
            let mut top: bool = ((mouse_y.wrapping_sub(row_top)).wrapping_abs() < (mouse_y.wrapping_sub(row_bottom)).wrapping_abs());
            group_entry = compat::read((&*__view4), |__t| __t.group_entry.clone()).unwrap();
            let group_index = index_of_entry(&children, &(group_entry.clone() as compat::Shared<dyn AbstractWaypointEntry>));
            if top {
                position = (i.wrapping_sub(group_index)).wrapping_sub(1);
                insert_button_y = row_top;
            } else {
                position = i.wrapping_sub(group_index);
                insert_button_y = row_bottom;
            }
        } else if let Some(__sel5) = hovered_entry.as_ref() && let Some(__view6) = compat::try_view::<WaypointGroupEntry, _>(__sel5) {
            let mut i: i32 = index_of_entry(&children, __sel5);
            let mut row_top: i32 = { let (__q0,) = ((i) as usize,); compat::read(__this, |__t| __t.get_row_top(__q0)) };
            let mut row_bottom: i32 = { let (__q0,) = ((i) as usize,); compat::read(__this, |__t| __t.get_row_bottom(__q0)) };
            let mut top: bool = ((mouse_y.wrapping_sub(row_top)).wrapping_abs() < (mouse_y.wrapping_sub(row_bottom)).wrapping_abs());
            if top {
                insert_button_y = row_top;
                // `nextEntry(ScreenDirection.UP, _ -> true, hoveredEntry)`: the row above (null at the top).
                let above_index = compat::read(__this, |__t| __t.next_entry(mcrust::net::minecraft::client::gui::navigation::screen_direction::ScreenDirection::Up, hovered_index));
                let Some(above) = above_index.and_then(|__i| children.get(__i).cloned()) else {
                    return;
                };
                if let Some(__view7) = compat::try_view::<WaypointEntry, _>(&above) {
                    group_entry = compat::read((&*__view7), |__t| __t.group_entry.clone()).unwrap();
                    position = index_of_entry(&children, &above).wrapping_sub(index_of_entry(&children, &(group_entry.clone() as compat::Shared<dyn AbstractWaypointEntry>)));
                } else if let Some(__view8) = compat::try_view::<WaypointGroupEntry, _>(&above) && !(contains_group(&collapsed_groups, compat::read((&*__view8), |__t| __t.group.clone()).as_ref().unwrap())) {
                    group_entry = (&*__view8).clone();
                    position = 0;
                } else {
                    return;
                }
            } else if !(contains_group(&collapsed_groups, compat::read((&*__view6), |__t| __t.group.clone()).as_ref().unwrap())) {
                insert_button_y = row_bottom;
                position = 0;
                group_entry = (&*__view6).clone();
            } else {
                return;
            }
        } else {
            return;
        }
        let mut m_x: i32 = mouse_x.wrapping_sub(__this.lock().borrow().get_row_left());
        if insert_button_y <= __this.lock().borrow().get_y() || insert_button_y >= __this.lock().borrow().get_bottom() || m_x > 32 {
            return;
        }
        let mut hovering: bool = ({ let (__q0, __q1,) = ((mouse_x) as f64, (mouse_y) as f64,); __this.lock().borrow().is_mouse_over(__q0, __q1) } && (mouse_y.wrapping_sub(insert_button_y)).wrapping_abs() <= 6 && m_x < 16 && m_x >= -8);
        gui_graphics_extractor::blit_sprite_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), &(if hovering { INSERT_HIGHLIGHTED_TEXTURE.get_mut().clone() } else { INSERT_TEXTURE.get_mut().clone() }), __this.lock().borrow().get_row_left(), insert_button_y.wrapping_sub(5), 48, 11);
        if crate::de::hysky::skyblocker::debug::debug::Debug::debug_enabled() {
            gui_graphics_extractor::text_string_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), Some(&position.to_string()), __this.lock().borrow().get_x(), __this.lock().borrow().get_y(), -1, true);
        }
        if hovering {
            { let __v = Some(InsertPosition::new(group_entry.clone(), position)); compat::write(__this, |__t| __t.insert_position = __v); }
            // TODO(fabric2seer): static `CursorTypes.POINTING_HAND` has no Rust counterpart in the port's CursorTypes file
            // TODO(fabric2seer): `GuiGraphicsExtractor.requestCursor` has no Rust method in the port (looked for `request_cursor` in GuiGraphicsExtractor and its supertypes)
            // rust: context.request_cursor(&CursorTypes::POINTING_HAND);
        }
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    pub fn mouse_clicked(__this: &compat::Shared<Self>, click: &MouseButtonEvent, mut doubled: bool) -> bool {
        if compat::read(__this, |__t| __t.insert_position.clone()).is_some() {
            let insert_position = compat::read(__this, |__t| __t.insert_position.clone()).unwrap();
            let mut entry: compat::Shared<WaypointEntry> = WaypointEntry::new(insert_position.group_entry.clone());
            // `insertPosition.groupEntry.group.waypoints().add(position, entry.waypoint)`: the group's own list.
            let group = compat::read(&insert_position.group_entry, |__t| __t.group.clone()).unwrap();
            let waypoint = compat::read(&entry, |__t| __t.waypoint.clone()).unwrap();
            compat::write(&group, |__g| __g.waypoints.insert((insert_position.position) as usize, waypoint));
            WaypointsListWidget::update_entries(__this);
        }
        // TODO(fabric2seer): `AbstractContainerWidget::mouse_clicked` ports `mouseClicked`, but the port's `AbstractSelectionList` holds no `AbstractContainerWidget` (no Deref, no `base` field) to call it on
        // rust: return { let (__q0, __q1,) = (click.clone(), doubled,); __this.lock().borrow_mut().base.list.mouse_clicked(__q0, __q1) };
        return todo!("fabric2seer: see the TODO above");
    }

    /// java: void updateEntries()
    pub fn update_entries(__this: &compat::Shared<Self>) {
        compat::write(__this, |__t| { let __l = &mut __t.base; __l.clear_entries() });
        WaypointsListWidget::check_and_add_warning(__this);
        for group in WaypointsListWidget::waypoint_groups(__this) {
            let mut collapsed: bool = compat::read(__this, |__t| contains_group(&__t.collapsed_groups, &group));
            let mut group_entry: compat::Shared<WaypointGroupEntry> = WaypointGroupEntry::new_2(group.clone(), collapsed);
            let __entry = (group_entry.clone() as compat::Shared<dyn AbstractWaypointEntry>);
            compat::write(__this, |__t| { let __l = &mut __t.base; (__l.add_entry(Box::new(compat::EntryAdapter::new(__entry))) as i32) });
            if collapsed {
                continue;
            }
            for waypoint in WaypointGroup::waypoints(&group) {
                let __entry = (WaypointEntry::new_2(group_entry.clone(), waypoint.clone()) as compat::Shared<dyn AbstractWaypointEntry>);
                compat::write(__this, |__t| { let __l = &mut __t.base; (__l.add_entry(Box::new(compat::EntryAdapter::new(__entry))) as i32) });
            }
        }
    }

    /// java: void updateButtons()
    pub fn update_buttons(__this: &compat::Shared<Self>) {
        for entry in WaypointsListWidget::children(__this) {
            if let Some(group_entry) = compat::try_view::<WaypointGroupEntry, _>(&entry) && WaypointGroupEntry::enabled_selected(&group_entry) != WaypointGroupEntry::should_be_checked(&group_entry) {
                // `((CheckboxAccessor) groupEntry.enabled).setSelected(!groupEntry.enabled.selected())`
                let selected = WaypointGroupEntry::enabled_selected(&group_entry);
                compat::write(&*group_entry, |__t| { let __s = &mut *__t; let __r = __s.enabled.as_mut().unwrap(); __s.base.__widgets.with(&mut __s.base.__row, __r, |__w| if let WidgetKind::Checkbox(__c) = &mut __w.kind { __c.selected = !selected; }) });
            } else if let Some(waypoint_entry) = compat::try_view::<WaypointEntry, _>(&entry) {
                let waypoint = compat::read(&*waypoint_entry, |__t| __t.waypoint.clone()).unwrap();
                let screen = compat::read(__this, |__t| __t.screen.clone());
                if WaypointEntry::enabled_selected(&waypoint_entry) != compat::read(&screen, |__s| __s.is_enabled(&waypoint)) {
                    // `waypointEntry.enabled.onPress(NoopInput.INSTANCE)`
                    compat::write(&*waypoint_entry, |__t| { let __s = &mut *__t; let __r = __s.enabled.as_mut().unwrap(); __s.base.__widgets.with(&mut __s.base.__row, __r, |__w| if let WidgetKind::Checkbox(__c) = &mut __w.kind { __c.on_press(); }) });
                }
            }
        }
    }

    /// java: void checkAndAddWarning()
    pub fn check_and_add_warning(__this: &compat::Shared<Self>) {
        if !(SkyblockerConfigManager::get().ui_and_visuals.waypoints.enable_waypoints) {
            let __entry = WaypointsDisabledWarningEntry::new() as compat::Shared<dyn AbstractWaypointEntry>;
            compat::write(__this, |__t| __t.base.add_entry_to_top(Box::new(compat::EntryAdapter::new(__entry))));
        }
    }

    /// java: private BlockPos getDefaultPos()
    pub fn get_default_pos(__this: &compat::Shared<Self>) -> BlockPos {
        // `minecraft.hitResult instanceof BlockHitResult b && hitResult.getType() == BLOCK ? b.getBlockPos()
        // : minecraft.player != null ? minecraft.player.blockPosition() : BlockPos.ZERO`
        if let Some(mcrust::net::minecraft::world::phys::hit_result::AnyHitResult::Block(block_hit_result)) = compat::minecraft().hit_result.as_ref() && block_hit_result.get_type() == mcrust::net::minecraft::world::phys::hit_result::Type::Block {
            return block_hit_result.get_block_pos();
        }
        return mcrust::mcrust::modding::game::with_player(|player| player.base.base.base.base.block_position()).unwrap_or(mcrust::net::minecraft::core::block_pos::ZERO);
    }

    /// java: @Override public boolean mouseScrolled(double mouseX, double mouseY, double horizontalAmount, double verticalAmount)
    pub fn mouse_scrolled(__this: &compat::Shared<Self>, mut mouse_x: f64, mut mouse_y: f64, mut horizontal_amount: f64, mut vertical_amount: f64) -> bool {
        // TODO(fabric2seer): Optional.mouseScrolled
        //   java: return this.getChildAt(mouseX, mouseY).filter(element -> element.mouseScrolled(mouseX, mouseY, horizontalAmount, verticalAmount)).isPresent() || super.mouseScrolled(mouseX, mouseY, horizontalAmount, verticalAmount);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}

/// WaypointsListWidget's list (the port's `ModSelectionList`) until the game takes it (`compat::attach_list`)
impl compat::ListHalf for WaypointsListWidget {
    fn list_half(&mut self) -> &mut ModSelectionList {
        &mut self.base
    }
}

/// WaypointsListWidget's overrides of the list's own methods, asked by the port's list bodies
impl ModList for compat::ListClass<WaypointsListWidget> {
    /// java: @Override public int getRowWidth()
    fn get_row_width(&self, list: &AbstractSelectionList, super_answer: i32) -> i32 {
        self.ask(list, super_answer, |__h| WaypointsListWidget::get_row_width(__h))
    }

    /// java: @Override protected int scrollBarX()
    fn scroll_bar_x(&self, list: &AbstractSelectionList, super_answer: i32) -> i32 {
        self.ask(list, super_answer, |__h| WaypointsListWidget::scroll_bar_x(__h))
    }

    /// java: @Override protected void extractListItems(GuiGraphicsExtractor context, int mouseX, int mouseY, float deltaTicks)
    fn extract_list_items(&self, list: &mut ModSelectionList, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32, super_extract_list_items: &mut dyn FnMut(&mut ModSelectionList, &mut WidgetRenderContext<'_>)) {
        super_extract_list_items(list, context);
        compat::with_super(&mut || {}, || self.ask(&list.list, (), |__h| WaypointsListWidget::extract_list_items(__h, &context.graphics, mouse_x, mouse_y, a)))
    }

}

// NOTE(fabric2seer): extractListItems, getRowWidth, mouseClicked, mouseScrolled, scrollBarX override(s) of ContainerObjectSelectionList - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them

/// java: protected abstract static class AbstractWaypointEntry extends ContainerObjectSelectionList.Entry<AbstractWaypointEntry>
pub struct AbstractWaypointEntryBase {
    /// java: extends net.minecraft.client.gui.components.ContainerObjectSelectionList$Entry
    pub base: mcrust::net::minecraft::client::gui::components::abstract_selection_list::Entry,
    /// the row's widgets (`children()`): lent to the object by its `compat::EntryAdapter`
    pub __row: mcrust::net::minecraft::client::gui::screens::screen::Screen,
    /// the screen's widgets as this object last saw them (see `compat::WidgetCache`)
    pub __widgets: compat::WidgetCache,
}

impl std::ops::Deref for AbstractWaypointEntryBase {
    type Target = mcrust::net::minecraft::client::gui::components::abstract_selection_list::Entry;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for AbstractWaypointEntryBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl AbstractWaypointEntryBase {
    /// java: the implicit no-argument constructor (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_in<__R>(__wrap: impl FnOnce(Self) -> (compat::Shared<dyn AbstractWaypointEntry>, __R)) -> (compat::Shared<dyn AbstractWaypointEntry>, __R) {
        let this = Self {
            base: mcrust::net::minecraft::client::gui::components::abstract_selection_list::Entry::new(0),
            __widgets: Default::default(),
            __row: mcrust::net::minecraft::client::gui::screens::screen::Screen::new(String::new()),
        };
        let (__h, __r) = __wrap(this);
        let __this = __h;
        (__this, __r)
    }

}

/// java: the virtual methods of AbstractWaypointEntry (implemented by AbstractWaypointEntryBase and every subclass)
pub trait AbstractWaypointEntry: compat::DynAny {
    fn abstract_waypoint_entry_base(&self) -> &AbstractWaypointEntryBase;
    fn abstract_waypoint_entry_base_mut(&mut self) -> &mut AbstractWaypointEntryBase;
    fn as_any(&self) -> &dyn std::any::Any;

}

/// AbstractWaypointEntry's row box and its widgets, lent by its `compat::EntryAdapter` for each call
impl compat::RowHalf for dyn AbstractWaypointEntry {
    fn entry_half(&mut self) -> &mut mcrust::net::minecraft::client::gui::components::abstract_selection_list::Entry {
        &mut self.abstract_waypoint_entry_base_mut().base
    }
    fn row_parts(&mut self) -> (&mut mcrust::net::minecraft::client::gui::screens::screen::Screen, &mut compat::WidgetCache) {
        let __s = self.abstract_waypoint_entry_base_mut();
        (&mut __s.__row, &mut __s.__widgets)
    }
}

/// AbstractWaypointEntry as a row of the game's list (`addEntry` attaches it)
impl ModListEntry for compat::EntryAdapter<dyn AbstractWaypointEntry> {
    /// java: @Override public void extractContent(GuiGraphicsExtractor context, int mouseX, int mouseY, boolean hovered, float deltaTicks)
    fn extract_content(&mut self, list: &mut ModSelectionList, index: usize, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, hovered: bool, a: f32) {
        self.lend(Some(list.list.entry(index)), |__h| { if let Some(__c) = compat::try_view::<WaypointEntry, _>(__h) { return WaypointEntry::extract_content(&*__c, &context.graphics, mouse_x, mouse_y, hovered, a); } if let Some(__c) = compat::try_view::<WaypointGroupEntry, _>(__h) { return WaypointGroupEntry::extract_content(&*__c, &context.graphics, mouse_x, mouse_y, hovered, a); } if let Some(__c) = compat::try_view::<WaypointsDisabledWarningEntry, _>(__h) { return WaypointsDisabledWarningEntry::extract_content(&*__c, &context.graphics, mouse_x, mouse_y, hovered, a); }  });
        // the row's widgets (Java's `child.extractRenderState(..)` over `children()`)
        context.extract_widgets(&mut self.row, mouse_x, mouse_y);
    }

    fn children(&mut self) -> Option<&mut mcrust::net::minecraft::client::gui::screens::screen::Screen> {
        Some(&mut self.row)
    }

}

/// java: protected class WaypointsDisabledWarningEntry extends AbstractWaypointEntry
pub struct WaypointsDisabledWarningEntry {
    /// java: extends AbstractWaypointEntry
    pub base: AbstractWaypointEntryBase,
    /// java: Component text = Component.translatable("skyblocker.waypoints.disabledWarning").withStyle(ChatFormatting.RED);
    pub text: MutableComponent,
    /// java: StringWidget textWidget = new StringWidget(text, minecraft.font);
    pub text_widget: Option<compat::WidgetRef>,
    /// java: int textSize = minecraft.font.width(text);
    pub text_size: i32,
    /// java: List<AbstractWidget> children = List.of(textWidget);
    /// The row's widgets (`__row`), by reference.
    pub children: Vec<compat::WidgetRef>,
}

impl std::ops::Deref for WaypointsDisabledWarningEntry {
    type Target = AbstractWaypointEntryBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for WaypointsDisabledWarningEntry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl WaypointsDisabledWarningEntry {
    /// java: the implicit no-argument constructor
    pub fn new() -> compat::Shared<Self> {
        // TODO(fabric2seer): the initialiser of `textSize`: unknown name `minecraft`
        //   java: int textSize = minecraft.font.width(text);
        // TODO(fabric2seer): the initialiser of `children`: instance field `textWidget` read in a static context
        //   java: List<AbstractWidget> children = List.of(textWidget);
        let (_, __this) = AbstractWaypointEntryBase::new_in(move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                text: component::translatable("skyblocker.waypoints.disabledWarning").with_style_format(ChatFormatting::Red),
                text_widget: None,
                text_size: Default::default(),
                children: Default::default(),
            });
            ((__h.clone() as compat::Shared<dyn AbstractWaypointEntry>), __h)
        });
        // TODO(fabric2seer): unknown name `minecraft`
        //   java: StringWidget textWidget = new StringWidget(text, minecraft.font);
        __this
    }

    /// java: @Override public void extractContent(GuiGraphicsExtractor graphics, int mouseX, int mouseY, boolean hovered, float a)
    pub fn extract_content(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut hovered: bool, mut a: f32) {
        // TODO(fabric2seer): `getRowWidth(..)` is a method of the enclosing instance (WaypointsListWidget.this) - an inner class has no outer reference in Rust
        //   java: textWidget.setPosition(getX() + (getRowWidth() - textSize) / 2, getY() + (getHeight() - minecraft.font.lineHeight) / 2);
        // TODO(fabric2seer): `extract_render_state` is a method of the port's trait `Renderable`, which the port's `AbstractWidget` does not implement
        // rust: let __wa1 = graphics.clone();
        // rust: let __wa2 = mouse_x;
        // rust: let __wa3 = mouse_y;
        // rust: let __wa4 = a;
        // rust: compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.text_widget.as_mut().unwrap(); __s.base.__widgets.with(&mut __s.base.__row, __r, |__w| __w.extract_render_state(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), __wa2, __wa3, __wa4)) });
    }

    /// java: @Override public List<? extends GuiEventListener> children()
    pub fn children(__this: &compat::Shared<Self>) -> Vec<compat::WidgetRef> {
        return compat::read(__this, |__t| __t.children.clone());
    }

    /// java: @Override public List<? extends NarratableEntry> narratables()
    pub fn narratables(__this: &compat::Shared<Self>) -> Vec<compat::WidgetRef> {
        return compat::read(__this, |__t| __t.children.clone());
    }

}

impl AbstractWaypointEntry for WaypointsDisabledWarningEntry {
    fn abstract_waypoint_entry_base(&self) -> &AbstractWaypointEntryBase {
        &self.base
    }
    fn abstract_waypoint_entry_base_mut(&mut self) -> &mut AbstractWaypointEntryBase {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

}

/// WaypointsDisabledWarningEntry's row box and its widgets, lent by its `compat::EntryAdapter` for each call
impl compat::RowHalf for WaypointsDisabledWarningEntry {
    fn entry_half(&mut self) -> &mut mcrust::net::minecraft::client::gui::components::abstract_selection_list::Entry {
        &mut self.base.base
    }
    fn row_parts(&mut self) -> (&mut mcrust::net::minecraft::client::gui::screens::screen::Screen, &mut compat::WidgetCache) {
        let __s = &mut self.base;
        (&mut __s.__row, &mut __s.__widgets)
    }
}

/// WaypointsDisabledWarningEntry as a row of the game's list (`addEntry` attaches it)
impl ModListEntry for compat::EntryAdapter<WaypointsDisabledWarningEntry> {
    /// java: @Override public void extractContent(GuiGraphicsExtractor graphics, int mouseX, int mouseY, boolean hovered, float a)
    fn extract_content(&mut self, list: &mut ModSelectionList, index: usize, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, hovered: bool, a: f32) {
        self.lend(Some(list.list.entry(index)), |__h| WaypointsDisabledWarningEntry::extract_content(__h, &context.graphics, mouse_x, mouse_y, hovered, a));
        // the row's widgets (Java's `child.extractRenderState(..)` over `children()`)
        context.extract_widgets(&mut self.row, mouse_x, mouse_y);
    }

    fn children(&mut self) -> Option<&mut mcrust::net::minecraft::client::gui::screens::screen::Screen> {
        Some(&mut self.row)
    }

}

/// java: protected class WaypointGroupEntry extends AbstractWaypointEntry
pub struct WaypointGroupEntry {
    /// java: extends AbstractWaypointEntry
    pub base: AbstractWaypointEntryBase,
    /// java: private WaypointGroup group;
    pub group: Option<compat::Shared<WaypointGroup>>,
    /// java: private final List<AbstractWidget> children;
    /// The row's widgets (`__row`), by reference.
    pub children: Vec<compat::WidgetRef>,
    /// java: private final Checkbox enabled;
    /// A row widget (`WidgetKind::Checkbox`), by reference.
    pub enabled: Option<compat::WidgetRef>,
    /// java: private final FrameLayout layout = new FrameLayout(getRowWidth(), defaultEntryHeight);
    pub layout: Option<FrameLayout>,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl std::ops::Deref for WaypointGroupEntry {
    type Target = AbstractWaypointEntryBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for WaypointGroupEntry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl WaypointGroupEntry {
    /// `enabled.selected()`: the state of the checkbox row widget.
    pub fn enabled_selected(__this: &compat::Shared<Self>) -> bool {
        compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.enabled.as_mut().expect("NullPointerException: enabled"); __s.base.__widgets.with(&mut __s.base.__row, __r, |__w| if let WidgetKind::Checkbox(__c) = &__w.kind { __c.selected() } else { panic!("ClassCastException: the enabled widget is not a Checkbox") }) })
    }

    /// java: public WaypointGroupEntry()
    pub fn new() -> compat::Shared<Self> {
        // TODO(fabric2seer): `island` is a field of the enclosing instance (WaypointsListWidget.this) - an inner class has no outer reference in Rust
        //   java: this(new WaypointGroup("New Group", island), false);
        let __this: compat::Shared<Self> = todo!("constructor");
        __this
    }

    /// java: public WaypointGroupEntry(WaypointGroup initialGroup, boolean collapsed)
    pub fn new_2(mut initial_group: compat::Shared<WaypointGroup>, mut collapsed: bool) -> compat::Shared<Self> {
        let __init_group = Some(initial_group.clone());
        let mut left_layout: LinearLayout = LinearLayout::horizontal().spacing(4);
        let (_, __this) = AbstractWaypointEntryBase::new_in(move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                self_ref: std::sync::Weak::new(),
                group: __init_group,
                children: Default::default(),
                enabled: None,
                layout: None,
            });
            compat::write(&__h, |__t| __t.self_ref = std::sync::Arc::downgrade(&__h));
            ((__h.clone() as compat::Shared<dyn AbstractWaypointEntry>), __h)
        });
        // TODO(fabric2seer): `getRowWidth(..)` is a method of the enclosing instance (WaypointsListWidget.this) - an inner class has no outer reference in Rust
        //   java: private final FrameLayout layout = new FrameLayout(getRowWidth(), defaultEntryHeight);
        left_layout.default_cell_setting().align_vertically_middle();
        { let (__q0, __q1,) = (LayoutElement::Grid(left_layout.clone().wrapped), &mut |__a0: &mut LayoutSettings| { *__a0 = __a0.align_horizontally_left(); },); __this.lock().borrow_mut().layout.as_mut().unwrap().add_child_adjusted(__q0, __q1) };
        let mut right_layout: LinearLayout = LinearLayout::horizontal().spacing(4);
        right_layout.default_cell_setting().align_vertically_middle();
        { let (__q0, __q1,) = (LayoutElement::Grid(right_layout.clone().wrapped), &mut |__a0: &mut LayoutSettings| { *__a0 = __a0.align_horizontally_right(); },); __this.lock().borrow_mut().layout.as_mut().unwrap().add_child_adjusted(__q0, __q1) };
        let mut arrow: MutableComponent = component::null_to_empty(Some((if collapsed { "▲" } else { "▼" })));
        let mut collapse_waypoint: AbstractWidget = (button::builder(arrow.get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| {
            if collapsed {
                // TODO(fabric2seer): `collapsedGroups` is a field of the enclosing instance (WaypointsListWidget.this) - an inner class has no outer reference in Rust
                //   java: collapsedGroups.remove(group);
            } else {
                // TODO(fabric2seer): `collapsedGroups` is a field of the enclosing instance (WaypointsListWidget.this) - an inner class has no outer reference in Rust
                //   java: collapsedGroups.add(group);
            }
            // TODO(fabric2seer): `updateEntries(..)` is a method of the enclosing instance (WaypointsListWidget.this) - an inner class has no outer reference in Rust
            //   java: updateEntries();
        })))).size(11, 11).build();
        left_layout.add_child({ let __w = collapse_waypoint.clone(); let (__ww, __wh) = (__w.get_width(), __w.get_height()); LayoutElement::widget(compat::write(&__this, |__t| __t.base.__row.add_widget(__w)), __ww, __wh) });
        // TODO(fabric2seer): unknown name `minecraft`
        //   java: enabled = Checkbox.builder(Component.empty(), minecraft.font).selected(shouldBeChecked()).onValueChange((_, checked) -> group.waypoints().forEach(waypoint -> screen.enabledChanged(waypoint, checked))).build();
        let mut __children: Vec<compat::WidgetRef> = Vec::new();
        // `leftLayout.addChild(enabled)`: the checkbox (a row widget) joins the row's list.
        left_layout.add_child({ let (__i, __ww, __wh) = compat::write(&__this, |__t| { let __s = &mut *__t; let __r = __s.enabled.as_mut().expect("NullPointerException: enabled"); __s.base.__widgets.add_to_layout(&mut __s.base.__row, __r) }); __children.push(compat::WidgetRef::Screen(__i)); LayoutElement::widget(__i, __ww, __wh) });
        // TODO(fabric2seer): unknown name `minecraft`
        //   java: EditBox nameField = new EditBox(minecraft.font, 70, 20, Component.literal("Name"));
        let mut name_field: EditBox = todo!("fabric2seer: the initialiser of `nameField` (TODO above)");
        // TODO(fabric2seer): `set_value` takes 2 argument(s) in the port (`splitter: &StringSplitter<W>, value: &str`), Java passes 1
        // rust: name_field.set_value(&WaypointGroup::name(&compat::read(&__this, |__t| __t.group.clone()).unwrap()));
        let __this_w1 = compat::RenderThread::new(std::sync::Arc::downgrade(&__this));
        name_field.set_responder(Responder::Lambda(ResponderLambda::new(move |__a0| { let Some(__this_1) = __this_w1.get().upgrade() else { return Default::default(); }; { WaypointGroupEntry::update_name(&__this_1, __a0); } })));
        // java: leftLayout.addChild(nameField);
        // TODO(batchD): structural - the name field is an `EditBox` VALUE (its constructor is unported above); a row
        // widget is an `AbstractWidget` of kind `WidgetKind::EditBox`, which the field has to become before the layout
        // can hold it by index.
        // rust: left_layout.add_child(name_field.clone());
        // TODO(fabric2seer): static `CommonComponents.optionNameValue` has no Rust counterpart in the port
        // rust: let __this_w2 = compat::RenderThread::new(std::sync::Arc::downgrade(&__this));
        // rust: let mut ordered_widget: compat::Shared<CyclingIconButtonWidget<bool>> = left_layout.add_child({ let __w = compat::WidgetAdapter::widget(CyclingIconButtonWidget::new_7(20, 20, WaypointGroup::ordered(&compat::read(&__this, |__t| __t.group.clone()).unwrap()), vec![false, true], Arc::new(move |b| Icon::new(crate::entry::skyblocker_mod::id(&format!("waypoints_screen/ordered_{}", (if b { "enabled" } else { "disabled" }))), 16, 16)), Arc::new(move |b| Tooltip::create((CommonComponents::option_name_value(component::translatable("skyblocker.waypoints.groupType"), (component::translatable((if b { "skyblocker.waypoints.groupType.ordered" } else { "skyblocker.waypoints.groupType.normal" }))).with_style_format(ChatFormatting::Yellow))).get_string())), Arc::new(move |__a0| { let Some(__this_2) = __this_w2.get().upgrade() else { return Default::default(); }; { WaypointGroupEntry::update_ordered(&__this_2, __a0.clone()); } }))); let (__ww, __wh) = (__w.get_width(), __w.get_height()); LayoutElement::widget(compat::write(&__this, |__t| __t.base.__row.add_widget(__w)), __ww, __wh) });
        let mut ordered_widget: compat::Shared<CyclingIconButtonWidget<bool>> = todo!("fabric2seer: the initialiser of `orderedWidget` (TODO above)");
        // TODO(fabric2seer): static `CommonComponents.optionNameValue` has no Rust counterpart in the port
        // rust: let __this_w3 = compat::RenderThread::new(std::sync::Arc::downgrade(&__this));
        // rust: let mut through_walls_widget: compat::Shared<CyclingIconButtonWidget<bool>> = left_layout.add_child({ let __w = compat::WidgetAdapter::widget(CyclingIconButtonWidget::new_7(20, 20, WaypointGroup::render_through_walls(&compat::read(&__this, |__t| __t.group.clone()).unwrap()), vec![false, true], Arc::new(move |b| Icon::new(crate::entry::skyblocker_mod::id(&format!("waypoints_screen/through_walls_{}", (if b { "enabled" } else { "disabled" }))), 15, 16)), Arc::new(move |b| Tooltip::create((CommonComponents::option_name_value(component::translatable("skyblocker.waypoints.throughWalls"), (if b { mcrust::net::minecraft::network::chat::common_components::gui_yes() } else { mcrust::net::minecraft::network::chat::common_components::gui_no() }).copy().with_style_format(ChatFormatting::Yellow))).get_string())), Arc::new(move |__a0| { let Some(__this_3) = __this_w3.get().upgrade() else { return Default::default(); }; { WaypointGroupEntry::update_render_through_walls(&__this_3, __a0.clone()); } }))); let (__ww, __wh) = (__w.get_width(), __w.get_height()); LayoutElement::widget(compat::write(&__this, |__t| __t.base.__row.add_widget(__w)), __ww, __wh) });
        let mut through_walls_widget: compat::Shared<CyclingIconButtonWidget<bool>> = todo!("fabric2seer: the initialiser of `throughWallsWidget` (TODO above)");
        // TODO(fabric2seer): `.getSerializedName(..)` on a T
        //   java: CyclingIconButtonWidget<Waypoint.Type> waypointTypeWidget = leftLayout.addChild(new CyclingIconButtonWidget<>(
        //   java: 					20,
        //   java: 					20,
        //   java: 					group.waypointType(),
        //   java: 					Waypoint.Type.values(),
        //   java: 					t -> new CyclingIconButtonWidget.Icon(SkyblockerMod.id("waypoints_screen/waypoint_type_" + t.getSerializedName()), 12, 15),
        //   java: 					t -> Tooltip.create(CommonComponents.optionNameValue(Component.translatable("skyblocker.waypoints.waypointType"), Component.literal(t.toString()).withStyle(ChatFormatting.YELLOW))),
        //   java: 					this::updateWaypointType
        //   java: 			));
        let mut waypoint_type_widget: compat::Shared<CyclingIconButtonWidget<Type>> = todo!("fabric2seer: the initialiser of `waypointTypeWidget` (TODO above)");
        let __this_w4 = compat::RenderThread::new(std::sync::Arc::downgrade(&__this));
        let mut button_new_waypoint: AbstractWidget = (button::builder(component::translatable("skyblocker.waypoints.new").get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { let Some(__this_4) = __this_w4.get().upgrade() else { return Default::default(); }; compat::lend_row(&__this_4, __screen, || { let mut __pidx = compat::WidgetRef::Screen(__index); {
            let mut waypoint_entry: compat::Shared<WaypointEntry> = WaypointEntry::new(__this_4.clone());
            // `group.waypoints().add(waypointEntry.waypoint)`: the group's own list.
            let __group = compat::read(&__this_4, |__t| __t.group.clone()).unwrap();
            let __waypoint = compat::read(&waypoint_entry, |__t| __t.waypoint.clone()).unwrap();
            compat::write(&__group, |__g| __g.waypoints.push(__waypoint));
            // TODO(fabric2seer): `WaypointsListWidget.this` (the enclosing instance of an inner class)
            //   java: WaypointsListWidget.this.updateEntries();
            if collapsed {
                // TODO(fabric2seer): `collapsedGroups` is a field of the enclosing instance (WaypointsListWidget.this) - an inner class has no outer reference in Rust
                //   java: collapsedGroups.remove(group);
                // TODO(fabric2seer): `updateEntries(..)` is a method of the enclosing instance (WaypointsListWidget.this) - an inner class has no outer reference in Rust
                //   java: updateEntries();
            }
        } }) })))).width(72).build();
        right_layout.add_child({ let __w = button_new_waypoint.clone(); let (__ww, __wh) = (__w.get_width(), __w.get_height()); LayoutElement::widget(compat::write(&__this, |__t| __t.base.__row.add_widget(__w)), __ww, __wh) });
        let mut delete_text: MutableComponent = component::translatable("selectServer.deleteButton");
        let mut button_delete: AbstractWidget = (sprite_icon_button::builder(delete_text.get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| {
            // TODO(fabric2seer): `waypoints` is a field of the enclosing instance (WaypointsListWidget.this) - an inner class has no outer reference in Rust
            //   java: waypoints.remove(group);
            // TODO(fabric2seer): `updateEntries(..)` is a method of the enclosing instance (WaypointsListWidget.this) - an inner class has no outer reference in Rust
            //   java: updateEntries();
        })), true)).size(20, 20).sprite(DELETE_ICON.get_mut().clone(), ICON_WIDTH, ICON_HEIGHT).build().unwrap();
        button_delete.set_tooltip(Tooltip::create(delete_text.get_string()));
        right_layout.add_child({ let __w = button_delete.clone(); let (__ww, __wh) = (__w.get_width(), __w.get_height()); LayoutElement::widget(compat::write(&__this, |__t| __t.base.__row.add_widget(__w)), __ww, __wh) });
        { __this.lock().borrow_mut().layout.as_mut().unwrap().arrange_elements(); let mut __vw = Vec::new(); __this.lock().borrow().layout.as_ref().unwrap().visit_widgets(&mut __vw); for (__i, __x, __y) in __vw { compat::write(&__this, |__t| __t.base.__row.widget_mut(__i).set_position(__x, __y)); } };
        // `children = List.of(enabled, nameField, orderedWidget, throughWallsWidget, waypointTypeWidget, buttonNewWaypoint,
        // buttonDelete, collapseWaypoint)`: the row widgets that joined the row's list above, by reference.
        // TODO(batchD): structural - the name field and the three cycling widgets (`CyclingIconButtonWidget`, utils/render/gui,
        // no widget adapter) are not row widgets, so they are not in the list.
        compat::write(&__this, |__t| __t.children = __children);
        __this
    }

    /// java: @Override public List<? extends NarratableEntry> narratables()
    pub fn narratables(__this: &compat::Shared<Self>) -> Vec<compat::WidgetRef> {
        return compat::read(__this, |__t| __t.children.clone());
    }

    /// java: @Override public List<? extends GuiEventListener> children()
    pub fn children(__this: &compat::Shared<Self>) -> Vec<compat::WidgetRef> {
        return compat::read(__this, |__t| __t.children.clone());
    }

    /// java: private boolean shouldBeChecked()
    pub fn should_be_checked(__this: &compat::Shared<Self>) -> bool {
        // TODO(fabric2seer): `screen` is a field of the enclosing instance (WaypointsListWidget.this) - an inner class has no outer reference in Rust
        //   java: return !group.waypoints().isEmpty() && group.waypoints().stream().allMatch(screen::isEnabled);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: private void updateName(String name)
    pub fn update_name(__this: &compat::Shared<Self>, name: &str) {
        // TODO(fabric2seer): `waypoints` is a field of the enclosing instance (WaypointsListWidget.this) - an inner class has no outer reference in Rust
        //   java: int index = waypoints.indexOf(group);
        let mut index: i32 = todo!("fabric2seer: the initialiser of `index` (TODO above)");
        { let __v = Some(WaypointGroup::with_name(&compat::read(__this, |__t| __t.group.clone()).unwrap(), name)); compat::write(__this, |__t| __t.group = __v); }
        if index >= 0 {
            // TODO(fabric2seer): `waypoints` is a field of the enclosing instance (WaypointsListWidget.this) - an inner class has no outer reference in Rust
            //   java: waypoints.set(index, group);
        }
    }

    /// java: private void updateOrdered(boolean ordered)
    pub fn update_ordered(__this: &compat::Shared<Self>, mut ordered: bool) {
        // TODO(fabric2seer): `waypoints` is a field of the enclosing instance (WaypointsListWidget.this) - an inner class has no outer reference in Rust
        //   java: int index = waypoints.indexOf(group);
        let mut index: i32 = todo!("fabric2seer: the initialiser of `index` (TODO above)");
        { let __v = Some(WaypointGroup::with_ordered(&compat::read(__this, |__t| __t.group.clone()).unwrap(), ordered)); compat::write(__this, |__t| __t.group = __v); }
        if index >= 0 {
            // TODO(fabric2seer): `waypoints` is a field of the enclosing instance (WaypointsListWidget.this) - an inner class has no outer reference in Rust
            //   java: waypoints.set(index, group);
        }
    }

    /// java: private void updateRenderThroughWalls(boolean renderThroughWalls)
    pub fn update_render_through_walls(__this: &compat::Shared<Self>, mut render_through_walls: bool) {
        // TODO(fabric2seer): `waypoints` is a field of the enclosing instance (WaypointsListWidget.this) - an inner class has no outer reference in Rust
        //   java: int index = waypoints.indexOf(group);
        let mut index: i32 = todo!("fabric2seer: the initialiser of `index` (TODO above)");
        { let __v = Some(WaypointGroup::with_render_through_walls(&compat::read(__this, |__t| __t.group.clone()).unwrap(), render_through_walls)); compat::write(__this, |__t| __t.group = __v); }
        if index >= 0 {
            // TODO(fabric2seer): `waypoints` is a field of the enclosing instance (WaypointsListWidget.this) - an inner class has no outer reference in Rust
            //   java: waypoints.set(index, group);
        }
    }

    /// java: private void updateWaypointType(Waypoint.Type waypointType)
    pub fn update_waypoint_type(__this: &compat::Shared<Self>, mut waypoint_type: Type) {
        // TODO(fabric2seer): `waypoints` is a field of the enclosing instance (WaypointsListWidget.this) - an inner class has no outer reference in Rust
        //   java: int index = waypoints.indexOf(group);
        let mut index: i32 = todo!("fabric2seer: the initialiser of `index` (TODO above)");
        { let __v = Some(WaypointGroup::with_waypoint_type(&compat::read(__this, |__t| __t.group.clone()).unwrap(), waypoint_type)); compat::write(__this, |__t| __t.group = __v); }
        if index >= 0 {
            // TODO(fabric2seer): `waypoints` is a field of the enclosing instance (WaypointsListWidget.this) - an inner class has no outer reference in Rust
            //   java: waypoints.set(index, group);
        }
    }

    /// java: @Override public void extractContent(GuiGraphicsExtractor context, int mouseX, int mouseY, boolean hovered, float deltaTicks)
    pub fn extract_content(__this: &compat::Shared<Self>, context: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut hovered: bool, mut delta_ticks: f32) {
        { let (__q0, __q1,) = (__this.lock().borrow().get_x(), __this.lock().borrow().get_y(),); __this.lock().borrow_mut().layout.as_mut().unwrap().set_position(__q0, __q1) };
        for child in compat::read(__this, |__t| __t.children.clone()) {
            // TODO(fabric2seer): `extract_render_state` is a method of the port's trait `Renderable`, which the port's `AbstractWidget` does not implement
            // rust: child.extract_render_state(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mouse_x, mouse_y, delta_ticks);
        }
    }

}

impl AbstractWaypointEntry for WaypointGroupEntry {
    fn abstract_waypoint_entry_base(&self) -> &AbstractWaypointEntryBase {
        &self.base
    }
    fn abstract_waypoint_entry_base_mut(&mut self) -> &mut AbstractWaypointEntryBase {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

}

/// WaypointGroupEntry's row box and its widgets, lent by its `compat::EntryAdapter` for each call
impl compat::RowHalf for WaypointGroupEntry {
    fn entry_half(&mut self) -> &mut mcrust::net::minecraft::client::gui::components::abstract_selection_list::Entry {
        &mut self.base.base
    }
    fn row_parts(&mut self) -> (&mut mcrust::net::minecraft::client::gui::screens::screen::Screen, &mut compat::WidgetCache) {
        let __s = &mut self.base;
        (&mut __s.__row, &mut __s.__widgets)
    }
}

/// WaypointGroupEntry as a row of the game's list (`addEntry` attaches it)
impl ModListEntry for compat::EntryAdapter<WaypointGroupEntry> {
    /// java: @Override public void extractContent(GuiGraphicsExtractor context, int mouseX, int mouseY, boolean hovered, float deltaTicks)
    fn extract_content(&mut self, list: &mut ModSelectionList, index: usize, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, hovered: bool, a: f32) {
        self.lend(Some(list.list.entry(index)), |__h| WaypointGroupEntry::extract_content(__h, &context.graphics, mouse_x, mouse_y, hovered, a));
        // the row's widgets (Java's `child.extractRenderState(..)` over `children()`)
        context.extract_widgets(&mut self.row, mouse_x, mouse_y);
    }

    fn children(&mut self) -> Option<&mut mcrust::net::minecraft::client::gui::screens::screen::Screen> {
        Some(&mut self.row)
    }

}

/// java: protected class WaypointEntry extends AbstractWaypointEntry
pub struct WaypointEntry {
    /// java: extends AbstractWaypointEntry
    pub base: AbstractWaypointEntryBase,
    /// java: private final WaypointGroupEntry groupEntry;
    pub group_entry: Option<compat::Shared<WaypointGroupEntry>>,
    /// java: private NamedWaypoint waypoint;
    pub waypoint: Option<compat::Shared<dyn NamedWaypoint>>,
    /// java: private final List<AbstractWidget> children;
    /// The row's widgets (`__row`), by reference.
    pub children: Vec<compat::WidgetRef>,
    /// java: private final Button buttonUp;
    pub button_up: Option<compat::WidgetRef>,
    /// java: private final Button buttonDown;
    pub button_down: Option<compat::WidgetRef>,
    /// java: private final Checkbox enabled;
    /// A row widget (`WidgetKind::Checkbox`), by reference.
    pub enabled: Option<compat::WidgetRef>,
    /// java: private final ColorPickerButton colorPickerButton;
    pub color_picker_button: Option<compat::WidgetRef>,
    /// java: private final FrameLayout layout = new FrameLayout(getRowWidth(), defaultEntryHeight);
    pub layout: Option<FrameLayout>,
}

impl std::ops::Deref for WaypointEntry {
    type Target = AbstractWaypointEntryBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for WaypointEntry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl WaypointEntry {
    /// `enabled.selected()`: the state of the checkbox row widget.
    pub fn enabled_selected(__this: &compat::Shared<Self>) -> bool {
        compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.enabled.as_mut().expect("NullPointerException: enabled"); __s.base.__widgets.with(&mut __s.base.__row, __r, |__w| if let WidgetKind::Checkbox(__c) = &__w.kind { __c.selected() } else { panic!("ClassCastException: the enabled widget is not a Checkbox") }) })
    }

    /// java: public WaypointEntry(WaypointGroupEntry groupEntry)
    pub fn new(mut group_entry: compat::Shared<WaypointGroupEntry>) -> compat::Shared<Self> {
        // TODO(fabric2seer): `getDefaultPos(..)` is a method of the enclosing instance (WaypointsListWidget.this) - an inner class has no outer reference in Rust
        //   java: this(groupEntry, groupEntry.group.createWaypoint(getDefaultPos()));
        let __this: compat::Shared<Self> = todo!("constructor");
        __this
    }

    /// java: public WaypointEntry(WaypointGroupEntry groupEntry, NamedWaypoint initialWaypoint)
    pub fn new_2(mut group_entry: compat::Shared<WaypointGroupEntry>, mut initial_waypoint: compat::Shared<dyn NamedWaypoint>) -> compat::Shared<Self> {
        let __init_group_entry = Some(group_entry.clone());
        let __init_waypoint = Some(initial_waypoint.clone());
        let mut left_layout: LinearLayout = LinearLayout::horizontal();
        let (_, __this) = AbstractWaypointEntryBase::new_in(move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                group_entry: __init_group_entry,
                waypoint: __init_waypoint,
                children: Default::default(),
                button_up: None,
                button_down: None,
                enabled: None,
                color_picker_button: None,
                layout: None,
            });
            ((__h.clone() as compat::Shared<dyn AbstractWaypointEntry>), __h)
        });
        // TODO(fabric2seer): `getRowWidth(..)` is a method of the enclosing instance (WaypointsListWidget.this) - an inner class has no outer reference in Rust
        //   java: private final FrameLayout layout = new FrameLayout(getRowWidth(), defaultEntryHeight);
        left_layout.default_cell_setting().align_vertically_middle();
        { let (__q0, __q1,) = (LayoutElement::Grid(left_layout.clone().wrapped), &mut |__a0: &mut LayoutSettings| { *__a0 = __a0.align_horizontally_left(); },); __this.lock().borrow_mut().layout.as_mut().unwrap().add_child_adjusted(__q0, __q1) };
        left_layout.add_child(SpacerElement::width(6).as_layout_element());
        let __this_w1 = compat::RenderThread::new(std::sync::Arc::downgrade(&__this));
        { let __v = (Some((button::builder((component::null_to_empty(Some("↑"))).get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { let Some(__this_1) = __this_w1.get().upgrade() else { return Default::default(); }; compat::lend_row(&__this_1, __screen, || { let mut __pidx = compat::WidgetRef::Screen(__index); { WaypointEntry::shift_waypoint_index(&__this_1, -1); } }) })))).size(11, 11).build())).map(compat::WidgetRef::value); compat::write(&__this, |__t| { let __s = &mut *__t; __s.button_up = __v; }); }
        left_layout.add_child({ let (__i, __ww, __wh) = compat::write(&__this, |__t| { let __s = &mut *__t; let __r = __s.button_up.as_mut().unwrap(); __s.base.__widgets.add_to_layout(&mut __s.base.__row, __r) }); LayoutElement::widget(__i, __ww, __wh) });
        let __this_w2 = compat::RenderThread::new(std::sync::Arc::downgrade(&__this));
        { let __v = (Some((button::builder((component::null_to_empty(Some("↓"))).get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { let Some(__this_2) = __this_w2.get().upgrade() else { return Default::default(); }; compat::lend_row(&__this_2, __screen, || { let mut __pidx = compat::WidgetRef::Screen(__index); { WaypointEntry::shift_waypoint_index(&__this_2, 1); } }) })))).size(11, 11).build())).map(compat::WidgetRef::value); compat::write(&__this, |__t| { let __s = &mut *__t; __s.button_down = __v; }); }
        left_layout.add_child({ let (__i, __ww, __wh) = compat::write(&__this, |__t| { let __s = &mut *__t; let __r = __s.button_down.as_mut().unwrap(); __s.base.__widgets.add_to_layout(&mut __s.base.__row, __r) }); LayoutElement::widget(__i, __ww, __wh) });
        // TODO(fabric2seer): unknown name `minecraft`
        //   java: enabled = Checkbox.builder(Component.literal(""), minecraft.font).selected(screen.isEnabled(waypoint)).onValueChange((_, checked) -> screen.enabledChanged(waypoint, checked)).build();
        left_layout.add_child_adjusted({ let (__i, __ww, __wh) = compat::write(&__this, |__t| { let __s = &mut *__t; let __r = __s.enabled.as_mut().expect("NullPointerException: enabled"); __s.base.__widgets.add_to_layout(&mut __s.base.__row, __r) }); LayoutElement::widget(__i, __ww, __wh) }, &mut |p: &mut LayoutSettings| { *p = p.padding_left(4); });
        // TODO(fabric2seer): unknown name `minecraft`
        //   java: EditBox nameField = new EditBox(minecraft.font, 65, 20, Component.literal("Name"));
        let mut name_field: EditBox = todo!("fabric2seer: the initialiser of `nameField` (TODO above)");
        // TODO(fabric2seer): `set_value` takes 2 argument(s) in the port (`splitter: &StringSplitter<W>, value: &str`), Java passes 1
        // rust: name_field.set_value(&({ let __r = &compat::read(&__this, |__t| __t.waypoint.clone()).unwrap(); let __f = compat::dispatch(__r, |__t| __t.get_name()); __f(__r) }).get_string());
        let __this_w3 = compat::RenderThread::new(std::sync::Arc::downgrade(&__this));
        name_field.set_responder(Responder::Lambda(ResponderLambda::new(move |__a0| { let Some(__this_3) = __this_w3.get().upgrade() else { return Default::default(); }; { WaypointEntry::update_name(&__this_3, __a0); } })));
        // java: leftLayout.addChild(nameField, p -> p.paddingLeft(2));
        // TODO(batchD): structural - see `WaypointGroupEntry::new_2`: the name field is an `EditBox` value, not a row widget.
        // rust: left_layout.add_child_adjusted(name_field.clone(), &mut |p| { p.padding_left(2); });
        // TODO(fabric2seer): unknown name `minecraft`
        //   java: leftLayout.addChild(new StringWidget(Component.literal("X:"), minecraft.font), p -> p.paddingLeft(2));
        // TODO(fabric2seer): unknown name `minecraft`
        //   java: FilteredEditBox xField = new FilteredEditBox(minecraft.font, 26, 20, Component.literal("X"));
        let mut x_field: compat::Shared<dyn FilteredEditBox> = todo!("fabric2seer: the initialiser of `xField` (TODO above)");
        ({ let __f = compat::dispatch(&x_field, |__t| __t.set_value()); __f(&x_field, &compat::read(&__this, |__t| __t.waypoint.clone()).unwrap().lock().borrow().named_waypoint_base().pos.as_ref().unwrap().get_x().to_string()) });
        let __this_w4 = compat::RenderThread::new(std::sync::Arc::downgrade(&__this));
        ({ let __f = compat::dispatch(&x_field, |__t| __t.set_filter()); __f(&x_field, Arc::new(move |__a0| { let Some(__this_4) = __this_w4.get().upgrade() else { return Default::default(); }; WaypointEntry::check_int(&__this_4, &__a0) })) });
        let __o5 = x_field.clone();
        let __this_w6 = compat::RenderThread::new(std::sync::Arc::downgrade(&__this));
        compat::write(&__o5, |__t| __t.filtered_edit_box_base_mut().set_responder(Responder::Lambda(ResponderLambda::new(move |__a0| { let Some(__this_6) = __this_w6.get().upgrade() else { return Default::default(); }; { WaypointEntry::update_x(&__this_6, __a0); } }))));
        // java: leftLayout.addChild(xField);
        // TODO(batchD): cross-package - `FilteredEditBox` (utils/render/gui) has no widget adapter (`WidgetHalf` /
        // `ModWidget`), so the screen's layout cannot hold it.
        // rust: left_layout.add_child(x_field.clone());
        // TODO(fabric2seer): unknown name `minecraft`
        //   java: leftLayout.addChild(new StringWidget(Component.literal("Y:"), minecraft.font), p -> p.paddingLeft(2));
        // TODO(fabric2seer): unknown name `minecraft`
        //   java: FilteredEditBox yField = new FilteredEditBox(minecraft.font, 26, 20, Component.literal("Y"));
        let mut y_field: compat::Shared<dyn FilteredEditBox> = todo!("fabric2seer: the initialiser of `yField` (TODO above)");
        ({ let __f = compat::dispatch(&y_field, |__t| __t.set_value()); __f(&y_field, &compat::read(&__this, |__t| __t.waypoint.clone()).unwrap().lock().borrow().named_waypoint_base().pos.as_ref().unwrap().get_y().to_string()) });
        let __this_w7 = compat::RenderThread::new(std::sync::Arc::downgrade(&__this));
        ({ let __f = compat::dispatch(&y_field, |__t| __t.set_filter()); __f(&y_field, Arc::new(move |__a0| { let Some(__this_7) = __this_w7.get().upgrade() else { return Default::default(); }; WaypointEntry::check_int(&__this_7, &__a0) })) });
        let __o8 = y_field.clone();
        let __this_w9 = compat::RenderThread::new(std::sync::Arc::downgrade(&__this));
        compat::write(&__o8, |__t| __t.filtered_edit_box_base_mut().set_responder(Responder::Lambda(ResponderLambda::new(move |__a0| { let Some(__this_9) = __this_w9.get().upgrade() else { return Default::default(); }; { WaypointEntry::update_y(&__this_9, __a0); } }))));
        // java: leftLayout.addChild(yField);
        // TODO(batchD): cross-package - `FilteredEditBox` (utils/render/gui) has no widget adapter (`WidgetHalf` /
        // `ModWidget`), so the screen's layout cannot hold it.
        // rust: left_layout.add_child(y_field.clone());
        // TODO(fabric2seer): unknown name `minecraft`
        //   java: leftLayout.addChild(new StringWidget(Component.literal("Z:"), minecraft.font), p -> p.paddingLeft(2));
        // TODO(fabric2seer): unknown name `minecraft`
        //   java: FilteredEditBox zField = new FilteredEditBox(minecraft.font, 26, 20, Component.literal("Z"));
        let mut z_field: compat::Shared<dyn FilteredEditBox> = todo!("fabric2seer: the initialiser of `zField` (TODO above)");
        ({ let __f = compat::dispatch(&z_field, |__t| __t.set_value()); __f(&z_field, &compat::read(&__this, |__t| __t.waypoint.clone()).unwrap().lock().borrow().named_waypoint_base().pos.as_ref().unwrap().get_z().to_string()) });
        let __this_w10 = compat::RenderThread::new(std::sync::Arc::downgrade(&__this));
        ({ let __f = compat::dispatch(&z_field, |__t| __t.set_filter()); __f(&z_field, Arc::new(move |__a0| { let Some(__this_10) = __this_w10.get().upgrade() else { return Default::default(); }; WaypointEntry::check_int(&__this_10, &__a0) })) });
        let __o11 = z_field.clone();
        let __this_w12 = compat::RenderThread::new(std::sync::Arc::downgrade(&__this));
        compat::write(&__o11, |__t| __t.filtered_edit_box_base_mut().set_responder(Responder::Lambda(ResponderLambda::new(move |__a0| { let Some(__this_12) = __this_w12.get().upgrade() else { return Default::default(); }; { WaypointEntry::update_z(&__this_12, __a0); } }))));
        // java: leftLayout.addChild(zField);
        // TODO(batchD): cross-package - `FilteredEditBox` (utils/render/gui) has no widget adapter (`WidgetHalf` /
        // `ModWidget`), so the screen's layout cannot hold it.
        // rust: left_layout.add_child(z_field.clone());
        // TODO(fabric2seer): unknown name `minecraft`
        //   java: ARGBTextInput colorField = new ARGBTextInput(0, 0, minecraft.font, true, true);
        let mut color_field: compat::Shared<ARGBTextInput> = todo!("fabric2seer: the initialiser of `colorField` (TODO above)");
        let __this_w13 = compat::RenderThread::new(std::sync::Arc::downgrade(&__this));
        { let __v = (Some(ColorPickerButton::new(color_field.clone(), Arc::new(move |__a0| { let Some(__this_13) = __this_w13.get().upgrade() else { return Default::default(); }; { WaypointEntry::update_color(&__this_13, __a0); } })))).map(|__o| compat::WidgetRef::value(compat::WidgetAdapter::widget(__o))); compat::write(&__this, |__t| { let __s = &mut *__t; __s.color_picker_button = __v; }); }
        left_layout.add_child_adjusted({ let (__i, __ww, __wh) = compat::write(&__this, |__t| { let __s = &mut *__t; let __r = __s.color_picker_button.as_mut().unwrap(); __s.base.__widgets.add_to_layout(&mut __s.base.__row, __r) }); LayoutElement::widget(__i, __ww, __wh) }, &mut |p: &mut LayoutSettings| { *p = p.padding_left(2); });
        let mut color: i32 = argb::color_from_float(compat::read(&compat::read(&__this, |__t| __t.waypoint.clone()).unwrap(), |__t| __t.named_waypoint_base().alpha), compat::read(&__this, |__t| __t.waypoint.clone()).unwrap().lock().borrow().named_waypoint_base().color_components[(0) as usize], compat::read(&__this, |__t| __t.waypoint.clone()).unwrap().lock().borrow().named_waypoint_base().color_components[(1) as usize], compat::read(&__this, |__t| __t.waypoint.clone()).unwrap().lock().borrow().named_waypoint_base().color_components[(2) as usize]);
        { let __v = color; compat::write(&compat::write(&__this, |__t| { let __s = &mut *__t; match __s.color_picker_button.as_mut() { Some(__r) => Some(__s.base.__widgets.with(&mut __s.base.__row, __r, |__w| __w.mod_widget().and_then(|__ms| __ms.with::<compat::WidgetAdapter<ColorPickerButton>, _>(|__a| __a.object().clone())).expect("a mod widget reached from inside its own method"))), None => None } }).unwrap(), |__t| __t.color = __v); }
        ARGBTextInput::set_argb_color(&color_field, color);
        let __o14 = color_field.clone();
        compat::write(&__o14, |__t| __t.set_height(20));
        let __this_w15 = compat::RenderThread::new(std::sync::Arc::downgrade(&__this));
        ARGBTextInput::set_on_change(&color_field, Arc::new(move |__a0| { let Some(__this_15) = __this_w15.get().upgrade() else { return Default::default(); }; { WaypointEntry::update_color(&__this_15, __a0); } }));
        left_layout.add_child({ let __w = compat::WidgetAdapter::widget(color_field.clone()); let (__ww, __wh) = (__w.get_width(), __w.get_height()); LayoutElement::widget(compat::write(&__this, |__t| __t.base.__row.add_widget(__w)), __ww, __wh) });
        let mut delete_text: MutableComponent = component::translatable("selectServer.deleteButton");
        let __this_w16 = compat::RenderThread::new(std::sync::Arc::downgrade(&__this));
        // The action holds the group entry in a render-thread cell (a button's action must be `Send + Sync`).
        let __group_entry_rt = compat::RenderThread::new(group_entry.clone());
        let mut button_delete: AbstractWidget = (sprite_icon_button::builder(delete_text.get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { let Some(__this_16) = __this_w16.get().upgrade() else { return Default::default(); }; compat::lend_row(&__this_16, __screen, || { let mut __pidx = compat::WidgetRef::Screen(__index); {
            // `groupEntry.group.waypoints().remove(waypoint)`: the group's own list, the first equal waypoint.
            let group = compat::read(__group_entry_rt.get(), |__t| __t.group.clone()).unwrap();
            let waypoint = compat::read(&__this_16, |__t| __t.waypoint.clone()).unwrap();
            compat::write(&group, |__g| { if let Some(__i) = __g.waypoints.iter().position(|w| named_waypoint_equals(w, &waypoint)) { __g.waypoints.remove(__i); } });
            // TODO(fabric2seer): `WaypointsListWidget.this` (the enclosing instance of an inner class)
            //   java: WaypointsListWidget.this.updateEntries();
        } }) })), true)).size(20, 20).sprite(DELETE_ICON.get_mut().clone(), ICON_WIDTH, ICON_HEIGHT).build().unwrap();
        button_delete.set_tooltip(Tooltip::create(delete_text.get_string()));
        { let (__q0, __q1,) = ({ let __w = button_delete.clone(); let (__ww, __wh) = (__w.get_width(), __w.get_height()); LayoutElement::widget(compat::write(&__this, |__t| __t.base.__row.add_widget(__w)), __ww, __wh) }, &mut |__a0: &mut LayoutSettings| { *__a0 = __a0.align_horizontally_right(); },); __this.lock().borrow_mut().layout.as_mut().unwrap().add_child_adjusted(__q0, __q1) };
        { __this.lock().borrow_mut().layout.as_mut().unwrap().arrange_elements(); let mut __vw = Vec::new(); __this.lock().borrow().layout.as_ref().unwrap().visit_widgets(&mut __vw); for (__i, __x, __y) in __vw { compat::write(&__this, |__t| __t.base.__row.widget_mut(__i).set_position(__x, __y)); } };
        // TODO(fabric2seer): static call ImmutableList.builder(..)
        //   java: ImmutableList.Builder<AbstractWidget> builder = ImmutableList.builder();
        let mut builder: crate::unresolved::Builder = todo!("fabric2seer: the initialiser of `builder` (TODO above)");
        { let mut __vw = Vec::new(); __this.lock().borrow().layout.as_ref().unwrap().visit_widgets(&mut __vw); for (__i, _, _) in __vw { compat::write(&__this, |__t| __t.base.__row.mark_renderable(__i)); } };
        // TODO(fabric2seer): `.build(..)` on a Builder
        //   java: children = builder.build();
        __this
    }

    /// java: @Override public List<? extends NarratableEntry> narratables()
    pub fn narratables(__this: &compat::Shared<Self>) -> Vec<compat::WidgetRef> {
        return compat::read(__this, |__t| __t.children.clone());
    }

    /// java: @Override public List<? extends GuiEventListener> children()
    pub fn children(__this: &compat::Shared<Self>) -> Vec<compat::WidgetRef> {
        return compat::read(__this, |__t| __t.children.clone());
    }

    /// java: private void shiftWaypointIndex(int shift)
    pub fn shift_waypoint_index(__this: &compat::Shared<Self>, mut shift: i32) {
        let group = compat::read(&compat::read(__this, |__t| __t.group_entry.clone()).unwrap(), |__t| __t.group.clone()).unwrap();
        let waypoint = compat::read(__this, |__t| __t.waypoint.clone()).unwrap();
        let mut current_index: i32 = index_of_waypoint(&group, &waypoint);
        let mut new_index: i32 = (current_index.wrapping_add(shift)).clamp(0, (compat::read(&group, |__g| __g.waypoints.len()) as i32).wrapping_sub(1));
        // `waypoints.remove(currentIndex)` / `.add(newIndex, waypoint)` on the group's own list
        compat::write(&group, |__g| {
            __g.waypoints.remove((current_index) as usize);
            __g.waypoints.insert((new_index) as usize, waypoint);
        });
        // TODO(fabric2seer): `WaypointsListWidget.this` (the enclosing instance of an inner class)
        //   java: WaypointsListWidget.this.updateEntries();
    }

    /// java: private void updateName(String name)
    pub fn update_name(__this: &compat::Shared<Self>, name: &str) {
        if compat::read(&compat::read(__this, |__t| __t.waypoint.clone()).unwrap(), |__t| __t.named_waypoint_base().name.as_ref().unwrap().get_string()) == name {
            return;
        }
        let group = compat::read(&compat::read(__this, |__t| __t.group_entry.clone()).unwrap(), |__t| __t.group.clone()).unwrap();
        let mut index: i32 = index_of_waypoint(&group, &compat::read(__this, |__t| __t.waypoint.clone()).unwrap());
        { let __v = Some(({ let __r = &compat::read(__this, |__t| __t.waypoint.clone()).unwrap(); let __f = compat::dispatch(__r, |__t| __t.with_name()); __f(__r, name) })); compat::write(__this, |__t| __t.waypoint = __v); }
        if index >= 0 {
            // `waypoints.set(index, waypoint)` on the group's own list
            let waypoint = compat::read(__this, |__t| __t.waypoint.clone()).unwrap();
            compat::write(&group, |__g| __g.waypoints[(index) as usize] = waypoint);
        }
    }

    /// java: private boolean checkInt(String string)
    pub fn check_int(__this: &compat::Shared<Self>, string: &str) -> bool {
        // java: try {..} catch (NumberFormatException _) - an exception here is a panic, caught by `catch_unwind`; the body's `return` is the closure's value
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            WaypointEntry::parse_emptiable_int(__this, string);
            return true;
        })) {
            Ok(__v) => return __v,
            Err(_) => {
                return false;
            }
        }
    }

    /// java: private void updateX(String xString)
    pub fn update_x(__this: &compat::Shared<Self>, x_string: &str) {
        let __this_w1 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        WaypointEntry::update_int(__this, x_string, compat::read(&compat::read(__this, |__t| __t.waypoint.clone()).unwrap(), |__t| __t.named_waypoint_base().pos.as_ref().unwrap().get_x()), Arc::new(move |__a0| { let __this_1 = __this_w1.get().upgrade().expect("the waypoint entry is alive while its own update runs"); ({ let __r = &compat::read((&__this_1), |__t| __t.waypoint.clone()).unwrap(); let __f = compat::dispatch(__r, |__t| __t.with_x()); waypoint_group::as_named_waypoint(&__f(&compat::view(__r, |a| a as compat::Shared<dyn Waypoint>), __a0)) }) }));
    }

    /// java: private void updateY(String yString)
    pub fn update_y(__this: &compat::Shared<Self>, y_string: &str) {
        let __this_w1 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        WaypointEntry::update_int(__this, y_string, compat::read(&compat::read(__this, |__t| __t.waypoint.clone()).unwrap(), |__t| __t.named_waypoint_base().pos.as_ref().unwrap().get_y()), Arc::new(move |__a0| { let __this_1 = __this_w1.get().upgrade().expect("the waypoint entry is alive while its own update runs"); ({ let __r = &compat::read((&__this_1), |__t| __t.waypoint.clone()).unwrap(); let __f = compat::dispatch(__r, |__t| __t.with_y()); waypoint_group::as_named_waypoint(&__f(&compat::view(__r, |a| a as compat::Shared<dyn Waypoint>), __a0)) }) }));
    }

    /// java: private void updateZ(String zString)
    pub fn update_z(__this: &compat::Shared<Self>, z_string: &str) {
        let __this_w1 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        WaypointEntry::update_int(__this, z_string, compat::read(&compat::read(__this, |__t| __t.waypoint.clone()).unwrap(), |__t| __t.named_waypoint_base().pos.as_ref().unwrap().get_z()), Arc::new(move |__a0| { let __this_1 = __this_w1.get().upgrade().expect("the waypoint entry is alive while its own update runs"); ({ let __r = &compat::read((&__this_1), |__t| __t.waypoint.clone()).unwrap(); let __f = compat::dispatch(__r, |__t| __t.with_z()); waypoint_group::as_named_waypoint(&__f(&compat::view(__r, |a| a as compat::Shared<dyn Waypoint>), __a0)) }) }));
    }

    /// java: private void updateInt(String newValueString, int currentValue, Int2ObjectFunction<NamedWaypoint> wither)
    pub fn update_int(__this: &compat::Shared<Self>, new_value_string: &str, mut current_value: i32, wither: Arc<dyn Fn(i32) -> compat::Shared<dyn NamedWaypoint>>) {
        // java: try {..} catch (NumberFormatException e) - the body runs inline: a Rust body does not throw checked exceptions (a panic in it propagates)
        {
            let group = compat::read(&compat::read(__this, |__t| __t.group_entry.clone()).unwrap(), |__t| __t.group.clone()).unwrap();
            let mut index: i32 = index_of_waypoint(&group, &compat::read(__this, |__t| __t.waypoint.clone()).unwrap());
            let mut new_value: i32 = WaypointEntry::parse_emptiable_int(__this, new_value_string);
            if new_value == current_value {
                return;
            }
            { let __v = Some((wither)(new_value)); compat::write(__this, |__t| __t.waypoint = __v); }
            if index >= 0 {
                let waypoint = compat::read(__this, |__t| __t.waypoint.clone()).unwrap();
                compat::write(&group, |__g| __g.waypoints[(index) as usize] = waypoint);
            }
        }
        // TODO(fabric2seer): the `catch (NumberFormatException e)` block is not reached in Rust - the try body returns, breaks or continues, so it cannot run inside `catch_unwind`
        //   java: {
        //   java: 				Waypoints.LOGGER.warn("[Skyblocker Waypoints] Failed to parse integer: {}", newValueString, e);
        //   java: 			}
    }

    /// java: private void updateColor(int colorInt)
    pub fn update_color(__this: &compat::Shared<Self>, mut color_int: i32) {
        let group = compat::read(&compat::read(__this, |__t| __t.group_entry.clone()).unwrap(), |__t| __t.group.clone()).unwrap();
        let mut index: i32 = index_of_waypoint(&group, &compat::read(__this, |__t| __t.waypoint.clone()).unwrap());
        let mut color_components: Vec<f32> = vec![(((color_int & 0x00FF0000).wrapping_shr((16) as u32) as f32) / 255.0f32), (((color_int & 0x0000FF00).wrapping_shr((8) as u32) as f32) / 255.0f32), (((color_int & 0x000000FF) as f32) / 255.0f32)];
        let mut alpha: f32 = ((((((color_int & 0xFF000000u32 as i32)) as u32) >> 24) as i32 as f32) / 255.0f32);
        if compat::read(&compat::read(__this, |__t| __t.waypoint.clone()).unwrap(), |__t| __t.named_waypoint_base().color_components.clone()) == color_components && compat::read(&compat::read(__this, |__t| __t.waypoint.clone()).unwrap(), |__t| __t.named_waypoint_base().alpha) == alpha {
            return;
        }
        { let __v = Some(({ let __r = &compat::read(__this, |__t| __t.waypoint.clone()).unwrap(); let __f = compat::dispatch(__r, |__t| __t.with_color()); waypoint_group::as_named_waypoint(&__f(&compat::view(__r, |a| a as compat::Shared<dyn Waypoint>), &color_components, alpha)) })); compat::write(__this, |__t| __t.waypoint = __v); }
        if index >= 0 {
            let waypoint = compat::read(__this, |__t| __t.waypoint.clone()).unwrap();
            compat::write(&group, |__g| __g.waypoints[(index) as usize] = waypoint);
        }
        { let __v = color_int; compat::write(&compat::write(__this, |__t| { let __s = &mut *__t; match __s.color_picker_button.as_mut() { Some(__r) => Some(__s.base.__widgets.with(&mut __s.base.__row, __r, |__w| __w.mod_widget().and_then(|__ms| __ms.with::<compat::WidgetAdapter<ColorPickerButton>, _>(|__a| __a.object().clone())).expect("a mod widget reached from inside its own method"))), None => None } }).unwrap(), |__t| __t.color = __v); }
    }

    /// java: private int parseEmptiableInt(String value) throws NumberFormatException
    pub fn parse_emptiable_int(__this: &compat::Shared<Self>, value: &str) -> i32 {
        // `Integer.parseInt(value)`: no trimming - a space is a `NumberFormatException`.
        return (if value.is_empty() || value == "-" { 0 } else { value.parse::<i32>().unwrap_or_else(|_| panic!("NumberFormatException: For input string: \"{}\"", value)) });
    }

    /// java: @Override public void extractContent(GuiGraphicsExtractor context, int mouseX, int mouseY, boolean hovered, float deltaTicks)
    pub fn extract_content(__this: &compat::Shared<Self>, context: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut hovered: bool, mut delta_ticks: f32) {
        { let (__q0, __q1,) = (__this.lock().borrow().get_x(), __this.lock().borrow().get_y(),); __this.lock().borrow_mut().layout.as_mut().unwrap().set_position(__q0, __q1) };
        let mut show_buttons: bool = (hovered && mouse_y >= (compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.button_up.as_mut().unwrap(); __s.base.__widgets.with(&mut __s.base.__row, __r, |__w| __w.get_y()) })).wrapping_sub(1) && mouse_y <= compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.button_up.as_mut().unwrap(); __s.base.__widgets.with(&mut __s.base.__row, __r, |__w| __w.get_bottom()) }));
        { let __v = show_buttons; compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.button_up.as_mut().unwrap(); __s.base.__widgets.with(&mut __s.base.__row, __r, |__w| __w.visible = __v) }); }
        { let __v = show_buttons; compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.button_down.as_mut().unwrap(); __s.base.__widgets.with(&mut __s.base.__row, __r, |__w| __w.visible = __v) }); }
        for child in compat::read(__this, |__t| __t.children.clone()) {
            // TODO(fabric2seer): `extract_render_state` is a method of the port's trait `Renderable`, which the port's `AbstractWidget` does not implement
            // rust: child.extract_render_state(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mouse_x, mouse_y, delta_ticks);
        }
    }

}

impl AbstractWaypointEntry for WaypointEntry {
    fn abstract_waypoint_entry_base(&self) -> &AbstractWaypointEntryBase {
        &self.base
    }
    fn abstract_waypoint_entry_base_mut(&mut self) -> &mut AbstractWaypointEntryBase {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

}

/// WaypointEntry's row box and its widgets, lent by its `compat::EntryAdapter` for each call
impl compat::RowHalf for WaypointEntry {
    fn entry_half(&mut self) -> &mut mcrust::net::minecraft::client::gui::components::abstract_selection_list::Entry {
        &mut self.base.base
    }
    fn row_parts(&mut self) -> (&mut mcrust::net::minecraft::client::gui::screens::screen::Screen, &mut compat::WidgetCache) {
        let __s = &mut self.base;
        (&mut __s.__row, &mut __s.__widgets)
    }
}

/// WaypointEntry as a row of the game's list (`addEntry` attaches it)
impl ModListEntry for compat::EntryAdapter<WaypointEntry> {
    /// java: @Override public void extractContent(GuiGraphicsExtractor context, int mouseX, int mouseY, boolean hovered, float deltaTicks)
    fn extract_content(&mut self, list: &mut ModSelectionList, index: usize, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, hovered: bool, a: f32) {
        self.lend(Some(list.list.entry(index)), |__h| WaypointEntry::extract_content(__h, &context.graphics, mouse_x, mouse_y, hovered, a));
        // the row's widgets (Java's `child.extractRenderState(..)` over `children()`)
        context.extract_widgets(&mut self.row, mouse_x, mouse_y);
    }

    fn children(&mut self) -> Option<&mut mcrust::net::minecraft::client::gui::screens::screen::Screen> {
        Some(&mut self.row)
    }

}

/// java: private class ColorPickerButton extends AbstractButton
pub struct ColorPickerButton {
    /// java: extends net.minecraft.client.gui.components.AbstractButton
    pub base: AbstractWidget,
    /// java: private final ARGBTextInput textInput;
    pub text_input: compat::Shared<ARGBTextInput>,
    /// java: private final IntConsumer colorConsumer;
    pub color_consumer: Arc<dyn Fn(i32)>,
    /// java: private int color;
    pub color: i32,
}

impl std::ops::Deref for ColorPickerButton {
    type Target = AbstractWidget;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for ColorPickerButton {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl ColorPickerButton {
    /// java: ColorPickerButton(ARGBTextInput textInput, IntConsumer colorConsumer)
    pub fn new(mut text_input: compat::Shared<ARGBTextInput>, color_consumer: Arc<dyn Fn(i32)>) -> compat::Shared<Self> {
        let __init_text_input = text_input.clone();
        let __init_color_consumer = color_consumer.clone();
        let __this = compat::shared(Self {
            base: AbstractWidget::new(0, 0, 20, 20, (&component::empty().get_string()).to_string(), WidgetKind::PlainText, OnPress::Inert),
            text_input: __init_text_input,
            color_consumer: __init_color_consumer,
            color: 0,
        });
        __this
    }

    /// java: @Override public void onPress(InputWithModifiers input)
    pub fn on_press(__this: &compat::Shared<Self>, input: &dyn InputWithModifiers) {
        let mut widget: compat::Shared<ColorPickerWidget> = ColorPickerWidget::new_5(0, 0, 200, 110, true);
        let __this_1 = compat::RenderThread::new(__this.clone());
        ColorPickerWidget::set_on_color_change(&widget, Arc::new(move |color, mouse_release| {
            ARGBTextInput::set_argb_color(&compat::read(__this_1.get(), |__t| __t.text_input.clone()), color);
            { let __v = color; compat::write(__this_1.get(), |__t| __t.color = __v); }
            if mouse_release {
                ((compat::read(__this_1.get(), |__t| __t.color_consumer.clone())))(color);
            }
        }));
        ColorPickerWidget::set_argb_color(&widget, ARGBTextInput::get_argb_color(&compat::read(__this, |__t| __t.text_input.clone())));
        // TODO(fabric2seer): `screen` is a field of the enclosing instance (WaypointsListWidget.this) - an inner class has no outer reference in Rust
        //   java: screen.setPopup(widget, getX(), getBottom());
    }

    /// java: @Override protected void extractContents(GuiGraphicsExtractor context, int mouseX, int mouseY, float deltaTicks)
    pub fn extract_contents(__this: &compat::Shared<Self>, context: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut delta_ticks: f32) {
        let mut padding: i32 = 1;
        context.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), (__this.lock().borrow().get_x()).wrapping_add(padding), (__this.lock().borrow().get_y()).wrapping_add(padding), (__this.lock().borrow().get_right()).wrapping_sub(padding), (__this.lock().borrow().get_bottom()).wrapping_sub(padding), (if __this.lock().borrow().is_hovered() { mcrust::net::minecraft::util::common_colors::WHITE } else { mcrust::net::minecraft::util::common_colors::BLACK }));
        context.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), ((__this.lock().borrow().get_x()).wrapping_add(padding)).wrapping_add(1), ((__this.lock().borrow().get_y()).wrapping_add(padding)).wrapping_add(1), ((__this.lock().borrow().get_right()).wrapping_sub(padding)).wrapping_sub(1), ((__this.lock().borrow().get_bottom()).wrapping_sub(padding)).wrapping_sub(1), compat::read(__this, |__t| __t.color));
        if __this.lock().borrow().is_hovered() {
            // TODO(fabric2seer): static `CursorTypes.POINTING_HAND` has no Rust counterpart in the port's CursorTypes file
            // TODO(fabric2seer): `GuiGraphicsExtractor.requestCursor` has no Rust method in the port (looked for `request_cursor` in GuiGraphicsExtractor and its supertypes)
            // rust: context.request_cursor(&CursorTypes::POINTING_HAND);
        }
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    pub fn update_widget_narration(__this: &compat::Shared<Self>, builder: &dyn NarrationElementOutput) {
    }

}

/// ColorPickerButton's widget half (`super`'s fields: x, y, size, message, ...), lent by the game for each call
impl compat::WidgetHalf for ColorPickerButton {
    fn widget_half(&mut self) -> &mut AbstractWidget {
        &mut self.base
    }
}

/// ColorPickerButton as the game's widget: `compat::WidgetAdapter::widget(handle)` where Java hands it on
impl ModWidget for compat::WidgetAdapter<ColorPickerButton> {
    /// java: extractWidgetRenderState - not overridden here (the base class's body has no entry point in the port)
    fn extract_widget_render_state(&mut self, widget: &AbstractWidget, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        let _ = widget;
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    fn update_widget_narration(&mut self, widget: &AbstractWidget, output: &mut dyn NarrationElementOutput) {
        self.lend_ref(widget, |__h| ColorPickerButton::update_widget_narration(__h, output))
    }

    /// java: @Override public void onPress(InputWithModifiers input)
    fn on_click(&mut self, widget: &mut AbstractWidget, event: &MouseButtonEvent, double_click: bool) {
        self.lend(widget, |__h| ColorPickerButton::on_press(__h, event))
    }

}

// NOTE(fabric2seer): extractContents, onPress, updateWidgetNarration override(s) of AbstractButton - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them

/// java: private record InsertPosition(WaypointGroupEntry groupEntry, int position)
#[derive(Clone)]
pub struct InsertPosition {
    /// java: WaypointGroupEntry groupEntry
    pub group_entry: compat::Shared<WaypointGroupEntry>,
    /// java: int position
    pub position: i32,
}

impl InsertPosition {
    /// java: the canonical record constructor
    pub fn new(mut group_entry: compat::Shared<WaypointGroupEntry>, mut position: i32) -> Self {
        Self { group_entry, position }
    }

    /// java: the record accessor `groupEntry()`
    pub fn group_entry(&self) -> compat::Shared<WaypointGroupEntry> {
        self.group_entry.clone()
    }

    /// java: the record accessor `position()`
    pub fn position(&self) -> i32 {
        self.position
    }

}
