//! Ported from de.hysky.skyblocker.skyblock.fancybars.StatusBarsConfigScreen by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::fancybars::bar_positioner;
use crate::de::hysky::skyblocker::skyblock::fancybars::bar_positioner::BarAnchor;
use crate::de::hysky::skyblocker::skyblock::fancybars::bar_positioner::BarLocation;
use crate::de::hysky::skyblocker::skyblock::fancybars::bar_positioner::SizeRule;
use crate::de::hysky::skyblocker::skyblock::fancybars::edit_bar_widget::EditBarWidget;
use crate::de::hysky::skyblocker::skyblock::fancybars::fancy_status_bars::FancyStatusBars;
use crate::de::hysky::skyblocker::skyblock::fancybars::status_bar::StatusBar;
use mcrust::com::mojang::blaze3d::platform::cursor::cursor_types::CursorTypes;
use mcrust::mcrust::modding::screens::ModScreen;
use mcrust::mcrust::modding::screens::ScreenRenderContext;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::navigation::screen_axis::ScreenAxis;
use mcrust::net::minecraft::client::gui::navigation::screen_direction::ScreenDirection;
use mcrust::net::minecraft::client::gui::navigation::screen_position::ScreenPosition;
use mcrust::net::minecraft::client::gui::navigation::screen_rectangle::ScreenRectangle;
use mcrust::net::minecraft::client::gui::screens::screen;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::client::input::mouse_button_info::MouseButtonInfo;
use mcrust::net::minecraft::client::minecraft;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::client::gui::components::renderable::Renderable as _;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final Identifier HOTBAR_TEXTURE = Identifier.withDefaultNamespace("hud/hotbar");
pub static HOTBAR_TEXTURE: compat::MainThread<Identifier> = compat::MainThread::new(|| Identifier::with_default_namespace("hud/hotbar"));
/// java: private static final int HOTBAR_WIDTH = 182;
pub const HOTBAR_WIDTH: i32 = 182;
/// java: private static final float RESIZE_THRESHOLD = 0.75f;
pub const RESIZE_THRESHOLD: f32 = 0.75f32;
/// java: private static final int BAR_MINIMUM_WIDTH = 30;
pub const BAR_MINIMUM_WIDTH: i32 = 30;
/// java: private static final ScreenDirection[] DIRECTION_CHECK_ORDER = new ScreenDirection[]{ScreenDirection.LEFT, ScreenDirection.RIGHT, ScreenDirection.UP, ScreenDirection.DOWN};
pub static DIRECTION_CHECK_ORDER: compat::MainThread<Vec<ScreenDirection>> = compat::MainThread::new(|| vec![ScreenDirection::Left, ScreenDirection::Right, ScreenDirection::Up, ScreenDirection::Down]);

/// java: public class StatusBarsConfigScreen extends Screen
pub struct StatusBarsConfigScreen {
    /// java: extends net.minecraft.client.gui.screens.Screen
    pub base: mcrust::net::minecraft::client::gui::screens::screen::Screen,
    /// java: protected final FancyStatusBars bars;
    pub bars: compat::Shared<FancyStatusBars>,
    /// java: private final Map<ScreenRectangle, Pair<StatusBar, BarLocation>> rectToBar = new HashMap<>();
    pub rect_to_bar: std::collections::HashMap<ScreenRectangle, (compat::Shared<dyn StatusBar>, BarLocation)>,
    /// java: private final ObjectBooleanPair<@Nullable StatusBar> resizeHover = new ObjectBooleanMutablePair<>(null, false);
    pub resize_hover: (Option<compat::Shared<dyn StatusBar>>, bool),
    /// java: private final Pair<@Nullable StatusBar, @Nullable StatusBar> resizedBars = ObjectObjectMutablePair.of(null, null);
    pub resized_bars: (Option<compat::Shared<dyn StatusBar>>, Option<compat::Shared<dyn StatusBar>>),
    /// java: private @Nullable StatusBar cursorBar = null;
    pub cursor_bar: Option<compat::Shared<dyn StatusBar>>,
    /// java: private ScreenPosition cursorOffset = new ScreenPosition(0, 0);
    pub cursor_offset: ScreenPosition,
    /// java: private BarLocation currentInsertLocation = new BarLocation(null, 0, 0);
    pub current_insert_location: BarLocation,
    /// java: private boolean resizing = false;
    pub resizing: bool,
    /// java: private EditBarWidget editBarWidget;
    pub edit_bar_widget: Option<EditBarWidget>,
    /// `addRenderableWidget(statusBar)` (`setup`): the bars the screen draws and hands its clicks, in order
    pub bar_children: Vec<compat::Shared<dyn StatusBar>>,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
    /// the screen's widgets as this object last saw them (see `compat::WidgetCache`)
    pub __widgets: compat::WidgetCache,
}

impl std::ops::Deref for StatusBarsConfigScreen {
    type Target = mcrust::net::minecraft::client::gui::screens::screen::Screen;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for StatusBarsConfigScreen {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl StatusBarsConfigScreen {
    /// java: public StatusBarsConfigScreen(FancyStatusBars bars)
    pub fn new(mut bars: compat::Shared<FancyStatusBars>) -> compat::Shared<Self> {
        let __init_bars = bars.clone();
        let __this = compat::shared(Self {
            base: mcrust::net::minecraft::client::gui::screens::screen::Screen::new(component::null_to_empty(Some("Status Bars Config")).get_string()),
            self_ref: std::sync::Weak::new(),
            __widgets: Default::default(),
            bars: __init_bars,
            rect_to_bar: std::collections::HashMap::new(),
            resize_hover: (None, false),
            resized_bars: (None, None),
            cursor_bar: None,
            cursor_offset: ScreenPosition::new(0, 0),
            current_insert_location: BarLocation::new(None, 0, 0),
            resizing: false,
            edit_bar_widget: None,
            bar_children: Vec::new(),
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        __this
    }

    /// java: private static int getNeighborInsertX(BarLocation barLocation, boolean right)
    pub fn get_neighbor_insert_x(bar_location: &BarLocation, mut right: bool) -> i32 {
        let mut bar_anchor: Option<BarAnchor> = bar_location.bar_anchor();
        let mut grid_x: i32 = bar_location.x();
        let Some(mut bar_anchor) = bar_anchor.clone() else {
            return 0;
        };
        if right {
            return (if bar_anchor.is_right() { grid_x.wrapping_add(1) } else { grid_x });
        } else {
            return (if bar_anchor.is_right() { grid_x } else { grid_x.wrapping_add(1) });
        }
    }

    /// java: private static int getNeighborInsertY(BarLocation barLocation, boolean up)
    pub fn get_neighbor_insert_y(bar_location: &BarLocation, mut up: bool) -> i32 {
        let mut bar_anchor: Option<BarAnchor> = bar_location.bar_anchor();
        let mut grid_y: i32 = bar_location.y();
        let Some(mut bar_anchor) = bar_anchor.clone() else {
            return 0;
        };
        if up {
            return (if bar_anchor.is_up() { grid_y.wrapping_add(1) } else { grid_y });
        } else {
            return (if bar_anchor.is_up() { grid_y } else { grid_y.wrapping_add(1) });
        }
    }

    /// java: @Override public void extractRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_render_state(__this: &compat::Shared<Self>, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        // java: super.extractRenderState(graphics, mouseX, mouseY, a); - the screen's renderables: the bars `setup` added
        for __bar in compat::read(__this, |__t| __t.bar_children.clone()) {
            let __f = compat::dispatch(&__bar, |__t| __t.extract_render_state());
            __f(&__bar, graphics, mouse_x, mouse_y, a);
        }
        gui_graphics_extractor::blit_sprite_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), HOTBAR_TEXTURE.get_mut(), ((compat::read(__this, |__t| __t.width)).wrapping_div(2)).wrapping_sub(HOTBAR_WIDTH.wrapping_div(2)), (compat::read(__this, |__t| __t.height)).wrapping_sub(22), HOTBAR_WIDTH, 22);
        // java: editBarWidget.extractRenderState(graphics, mouseX, mouseY, a); - the widget is taken out of the screen for
        // the call (its options reach the screen)
        let mut __edit = compat::write(__this, |__t| __t.edit_bar_widget.take());
        if let Some(__w) = __edit.as_mut() {
            __w.extract_render_state(graphics, mouse_x, mouse_y, a);
        }
        compat::write(__this, |__t| __t.edit_bar_widget = __edit);
        let window = compat::minecraft().get_window();
        let mut scale_factor: i32 = ((window.calculate_scale(0, minecraft::is_enforce_unicode(compat::minecraft()))).wrapping_sub(window.get_gui_scale())).wrapping_add(3);
        if (scale_factor & 2) == 0 {
            scale_factor += 1;
        }
        let mut mouse_rect: ScreenRectangle = ScreenRectangle::new(ScreenPosition::new(mouse_x.wrapping_sub(scale_factor.wrapping_div(2)), mouse_y.wrapping_sub(scale_factor.wrapping_div(2))), scale_factor, scale_factor);
        if compat::read(__this, |__t| __t.cursor_bar.clone()).is_some() {
            ({ let __r = &compat::read(__this, |__t| __t.cursor_bar.clone()).unwrap(); let __f = compat::dispatch(__r, |__t| __t.extract_cursor()); __f(__r, graphics, mouse_x.wrapping_add(__this.lock().borrow().cursor_offset.x()), mouse_y.wrapping_add(__this.lock().borrow().cursor_offset.y()), a) });
            let mut inserted: bool = false;
            let mut update_positions: bool = false;
            'rect_loop: for screen_rect in { __this.lock().borrow().rect_to_bar.keys().cloned().collect::<Vec<_>>() } {
                for direction in DIRECTION_CHECK_ORDER.get_mut().iter().copied() {
                    let mut overlaps: bool = screen_rect.get_border(direction).step(direction).overlaps(mouse_rect.clone());
                    if overlaps {
                        let mut bar_pair: (compat::Shared<dyn StatusBar>, BarLocation) = __this.lock().borrow().rect_to_bar.get(&screen_rect).cloned().unwrap();
                        let bar_snap = &bar_pair.1;
                        if bar_snap.bar_anchor().is_none() {
                            break;
                        }
                        if direction.get_axis() == ScreenAxis::Vertical {
                            let mut neighbor_insert_y: i32 = StatusBarsConfigScreen::get_neighbor_insert_y(bar_snap, !direction.is_positive());
                            inserted = true;
                            if !(__this.lock().borrow().current_insert_location.equals(bar_snap.bar_anchor().unwrap(), bar_snap.x(), neighbor_insert_y)) {
                                if compat::read(&compat::read(__this, |__t| __t.cursor_bar.clone()).unwrap(), |__t| __t.status_bar_base().anchor.clone()).is_some() {
                                    { let __bars = compat::read(__this, |__t| __t.bars.clone()); compat::write(&__bars, |__b| __b.bar_positioner.remove_bar_bar_anchor_int_status_bar(compat::read(&compat::read(__this, |__t| __t.cursor_bar.clone()).unwrap(), |__t| __t.status_bar_base().anchor.clone()).unwrap(), compat::read(&compat::read(__this, |__t| __t.cursor_bar.clone()).unwrap(), |__t| __t.status_bar_base().grid_y), &compat::read(__this, |__t| __t.cursor_bar.clone()).unwrap())); }
                                }
                                { let __bars = compat::read(__this, |__t| __t.bars.clone()); compat::write(&__bars, |__b| __b.bar_positioner.add_row_2(bar_snap.bar_anchor().unwrap(), neighbor_insert_y)); }
                                { let __bars = compat::read(__this, |__t| __t.bars.clone()); compat::write(&__bars, |__b| __b.bar_positioner.add_bar(bar_snap.bar_anchor().unwrap(), neighbor_insert_y, compat::read(__this, |__t| __t.cursor_bar.clone()).unwrap())); }
                                { let __v = BarLocation::of(&compat::read(__this, |__t| __t.cursor_bar.clone()).unwrap()); compat::write(__this, |__t| __t.current_insert_location = __v); }
                                update_positions = true;
                            }
                        } else {
                            let mut neighbor_insert_x: i32 = StatusBarsConfigScreen::get_neighbor_insert_x(bar_snap, direction.is_positive());
                            inserted = true;
                            if !(__this.lock().borrow().current_insert_location.equals(bar_snap.bar_anchor().unwrap(), neighbor_insert_x, bar_snap.y())) {
                                if compat::read(&compat::read(__this, |__t| __t.cursor_bar.clone()).unwrap(), |__t| __t.status_bar_base().anchor.clone()).is_some() {
                                    { let __bars = compat::read(__this, |__t| __t.bars.clone()); compat::write(&__bars, |__b| __b.bar_positioner.remove_bar_bar_anchor_int_status_bar(compat::read(&compat::read(__this, |__t| __t.cursor_bar.clone()).unwrap(), |__t| __t.status_bar_base().anchor.clone()).unwrap(), compat::read(&compat::read(__this, |__t| __t.cursor_bar.clone()).unwrap(), |__t| __t.status_bar_base().grid_y), &compat::read(__this, |__t| __t.cursor_bar.clone()).unwrap())); }
                                }
                                { let __bars = compat::read(__this, |__t| __t.bars.clone()); compat::write(&__bars, |__b| __b.bar_positioner.add_bar_4(bar_snap.bar_anchor().unwrap(), bar_snap.y(), neighbor_insert_x, compat::read(__this, |__t| __t.cursor_bar.clone()).unwrap())); }
                                { let __v = BarLocation::of(&compat::read(__this, |__t| __t.cursor_bar.clone()).unwrap()); compat::write(__this, |__t| __t.current_insert_location = __v); }
                                update_positions = true;
                            }
                        }
                        break 'rect_loop;
                    }
                }
            }
            if update_positions {
                FancyStatusBars::update_positions(&compat::read(__this, |__t| __t.bars.clone()), true);
                return;
            }
            for bar_anchor in BarAnchor::all_anchors() {
                let mut anchor_hitbox: ScreenRectangle = bar_anchor.get_anchor_hitbox(&bar_anchor.get_anchor_position(compat::read(__this, |__t| __t.width), compat::read(__this, |__t| __t.height)));
                if compat::read(__this, |__t| __t.bars.clone()).lock().borrow().bar_positioner.get_row_count(bar_anchor) != 0 {
                    if compat::read(__this, |__t| __t.bars.clone()).lock().borrow().bar_positioner.get_row_count(bar_anchor) == 1 {
                        let mut row: Vec<compat::Shared<dyn StatusBar>> = compat::read(__this, |__t| __t.bars.clone()).lock().borrow().bar_positioner.get_row(bar_anchor, 0);
                        if (row.len() as i32) == 1 && std::ptr::addr_eq(std::sync::Arc::as_ptr(&row[0]), std::sync::Arc::as_ptr(&compat::read(__this, |__t| __t.cursor_bar.clone()).unwrap())) && anchor_hitbox.overlaps(mouse_rect.clone()) {
                            inserted = true;
                        }
                    }
                    continue;
                }
                graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), anchor_hitbox.left(), anchor_hitbox.top(), anchor_hitbox.right(), anchor_hitbox.bottom(), 0x99FFFFFFu32 as i32);
                if anchor_hitbox.overlaps(mouse_rect.clone()) {
                    inserted = true;
                    if __this.lock().borrow().current_insert_location.bar_anchor().unwrap() == bar_anchor {
                        continue;
                    }
                    if compat::read(&compat::read(__this, |__t| __t.cursor_bar.clone()).unwrap(), |__t| __t.status_bar_base().anchor.clone()).is_some() {
                        { let __bars = compat::read(__this, |__t| __t.bars.clone()); compat::write(&__bars, |__b| __b.bar_positioner.remove_bar_bar_anchor_int_status_bar(compat::read(&compat::read(__this, |__t| __t.cursor_bar.clone()).unwrap(), |__t| __t.status_bar_base().anchor.clone()).unwrap(), compat::read(&compat::read(__this, |__t| __t.cursor_bar.clone()).unwrap(), |__t| __t.status_bar_base().grid_y), &compat::read(__this, |__t| __t.cursor_bar.clone()).unwrap())); }
                    }
                    { let __bars = compat::read(__this, |__t| __t.bars.clone()); compat::write(&__bars, |__b| __b.bar_positioner.add_row(bar_anchor)); }
                    { let __bars = compat::read(__this, |__t| __t.bars.clone()); compat::write(&__bars, |__b| __b.bar_positioner.add_bar(bar_anchor, 0, compat::read(__this, |__t| __t.cursor_bar.clone()).unwrap())); }
                    { let __v = BarLocation::of(&compat::read(__this, |__t| __t.cursor_bar.clone()).unwrap()); compat::write(__this, |__t| __t.current_insert_location = __v); }
                    FancyStatusBars::update_positions(&compat::read(__this, |__t| __t.bars.clone()), true);
                }
            }
            if !inserted {
                if compat::read(&compat::read(__this, |__t| __t.cursor_bar.clone()).unwrap(), |__t| __t.status_bar_base().anchor.clone()).is_some() {
                    { let __bars = compat::read(__this, |__t| __t.bars.clone()); compat::write(&__bars, |__b| __b.bar_positioner.remove_bar_bar_anchor_int_status_bar(compat::read(&compat::read(__this, |__t| __t.cursor_bar.clone()).unwrap(), |__t| __t.status_bar_base().anchor.clone()).unwrap(), compat::read(&compat::read(__this, |__t| __t.cursor_bar.clone()).unwrap(), |__t| __t.status_bar_base().grid_y), &compat::read(__this, |__t| __t.cursor_bar.clone()).unwrap())); }
                }
                { let __v = bar_positioner::BAR_LOCATION_NULL.clone(); compat::write(__this, |__t| __t.current_insert_location = __v); }
                FancyStatusBars::update_positions(&compat::read(__this, |__t| __t.bars.clone()), true);
                ({ let __r = &compat::read(__this, |__t| __t.cursor_bar.clone()).unwrap(); let __f = compat::dispatch(__r, |__t| __t.set_x()); __f(__r, (compat::read(__this, |__t| __t.width)).wrapping_add(5)) });
            }
        } else {
            if compat::read(__this, |__t| __t.resizing) {
                let mut middle_x: i32;
                let mut right_bar: Option<compat::Shared<dyn StatusBar>> = compat::read(__this, |__t| __t.resized_bars.clone()).1.clone();
                let mut left_bar: Option<compat::Shared<dyn StatusBar>> = compat::read(__this, |__t| __t.resized_bars.clone()).0.clone();
                let mut has_right: bool = right_bar.is_some();
                let mut has_left: bool = left_bar.is_some();
                let mut bar_anchor: Option<BarAnchor> = None;
                if !has_right {
                    bar_anchor = compat::read(left_bar.as_ref().unwrap(), |__t| __t.status_bar_base().anchor.clone());
                    middle_x = ({ let __r = left_bar.as_ref().unwrap(); let __f = compat::dispatch(__r, |__t| __t.get_x()); __f(__r) }).wrapping_add({ let __r = left_bar.as_ref().unwrap(); let __f = compat::dispatch(__r, |__t| __t.get_width()); __f(__r) });
                } else {
                    bar_anchor = compat::read(right_bar.as_ref().unwrap(), |__t| __t.status_bar_base().anchor.clone());
                    middle_x = ({ let __r = right_bar.as_ref().unwrap(); let __f = compat::dispatch(__r, |__t| __t.get_x()); __f(__r) });
                }
                if let Some(bar_anchor) = &bar_anchor {
                    let mut size_rule: SizeRule = bar_anchor.get_size_rule();
                    let mut do_resize: bool = true;
                    let mut width_per_size: f32;
                    if size_rule.is_target_size() {
                        width_per_size = (((size_rule.total_width()) as f32) / (size_rule.target_size() as f32));
                    } else {
                        width_per_size = ((size_rule.width_per_size()) as f32) as f32;
                    }
                    if mouse_x < middle_x {
                        if (middle_x.wrapping_sub(mouse_x) as f32) > (width_per_size / RESIZE_THRESHOLD) {
                            if has_right {
                                if (compat::read(right_bar.as_ref().unwrap(), |__t| __t.status_bar_base().size)).wrapping_add(1) > size_rule.max_size() {
                                    do_resize = false;
                                }
                            }
                            if has_left {
                                if (compat::read(left_bar.as_ref().unwrap(), |__t| __t.status_bar_base().size)).wrapping_sub(1) < size_rule.min_size() {
                                    do_resize = false;
                                }
                            }
                            if do_resize {
                                if has_right {
                                    { let __v = compat::read(right_bar.as_ref().unwrap(), |__t| __t.status_bar_base().size) + 1; compat::write(right_bar.as_ref().unwrap(), |__t| __t.status_bar_base_mut().size = __v); }
                                }
                                if has_left {
                                    { let __v = compat::read(left_bar.as_ref().unwrap(), |__t| __t.status_bar_base().size) - 1; compat::write(left_bar.as_ref().unwrap(), |__t| __t.status_bar_base_mut().size = __v); }
                                }
                                FancyStatusBars::update_positions(&compat::read(__this, |__t| __t.bars.clone()), true);
                            }
                        }
                    } else {
                        if (mouse_x.wrapping_sub(middle_x) as f32) > (width_per_size / RESIZE_THRESHOLD) {
                            if has_right {
                                if (compat::read(right_bar.as_ref().unwrap(), |__t| __t.status_bar_base().size)).wrapping_sub(1) < size_rule.min_size() {
                                    do_resize = false;
                                }
                            }
                            if has_left {
                                if (compat::read(left_bar.as_ref().unwrap(), |__t| __t.status_bar_base().size)).wrapping_add(1) > size_rule.max_size() {
                                    do_resize = false;
                                }
                            }
                            if do_resize {
                                if has_right {
                                    { let __v = compat::read(right_bar.as_ref().unwrap(), |__t| __t.status_bar_base().size) - 1; compat::write(right_bar.as_ref().unwrap(), |__t| __t.status_bar_base_mut().size = __v); }
                                }
                                if has_left {
                                    { let __v = compat::read(left_bar.as_ref().unwrap(), |__t| __t.status_bar_base().size) + 1; compat::write(left_bar.as_ref().unwrap(), |__t| __t.status_bar_base_mut().size = __v); }
                                }
                                FancyStatusBars::update_positions(&compat::read(__this, |__t| __t.bars.clone()), true);
                            }
                        }
                    }
                } else {
                    if has_left {
                        ({ let __r = left_bar.as_ref().unwrap(); let __f = compat::dispatch(__r, |__t| __t.set_width()); __f(__r, BAR_MINIMUM_WIDTH.max(mouse_x.wrapping_sub({ let __r = left_bar.as_ref().unwrap(); let __f = compat::dispatch(__r, |__t| __t.get_x()); __f(__r) }))) });
                    } else if has_right {
                        let mut end_x: i32 = ({ let __r = right_bar.as_ref().unwrap(); let __f = compat::dispatch(__r, |__t| __t.get_x()); __f(__r) }).wrapping_add({ let __r = right_bar.as_ref().unwrap(); let __f = compat::dispatch(__r, |__t| __t.get_width()); __f(__r) });
                        ({ let __r = right_bar.as_ref().unwrap(); let __f = compat::dispatch(__r, |__t| __t.set_x()); __f(__r, (end_x.wrapping_sub(BAR_MINIMUM_WIDTH)).min(mouse_x)) });
                        ({ let __r = right_bar.as_ref().unwrap(); let __f = compat::dispatch(__r, |__t| __t.set_width()); __f(__r, end_x.wrapping_sub({ let __r = right_bar.as_ref().unwrap(); let __f = compat::dispatch(__r, |__t| __t.get_x()); __f(__r) })) });
                    }
                }
            } else {
                'rect_loop: for screen_rect in { __this.lock().borrow().rect_to_bar.keys().cloned().collect::<Vec<_>>() } {
                    for direction in vec![ScreenDirection::Left, ScreenDirection::Right] {
                        let mut overlaps: bool = screen_rect.get_border(direction).step(direction).overlaps(mouse_rect.clone());
                        if overlaps && !({ let (__q0, __q1,) = ((mouse_x) as f64, (mouse_y) as f64,); __this.lock().borrow().edit_bar_widget.as_ref().unwrap().is_mouse_over(__q0, __q1) }) {
                            let mut bar_pair: (compat::Shared<dyn StatusBar>, BarLocation) = __this.lock().borrow().rect_to_bar.get(&screen_rect).cloned().unwrap();
                            let bar_location = &bar_pair.1;
                            let mut bar: compat::Shared<dyn StatusBar> = bar_pair.0.clone();
                            if !(compat::read(&bar, |__t| __t.status_bar_base().enabled)) {
                                break;
                            }
                            let mut right: bool = (direction == ScreenDirection::Right);
                            if bar_location.bar_anchor().is_some() {
                                if bar_location.bar_anchor().unwrap().get_size_rule().is_target_size() && !(compat::read(__this, |__t| __t.bars.clone()).lock().borrow().bar_positioner.has_neighbor(bar_location.bar_anchor().unwrap(), bar_location.y(), bar_location.x(), right)) {
                                    break;
                                }
                                if !(bar_location.bar_anchor().unwrap().get_size_rule().is_target_size()) && bar_location.x() == 0 && bar_location.bar_anchor().unwrap().is_right() != right {
                                    break;
                                }
                            }
                            // java: resizeHover.first(bar); resizeHover.right(right);
                            compat::write(__this, |__t| __t.resize_hover.0 = Some(bar.clone()));
                            compat::write(__this, |__t| __t.resize_hover.1 = right);
                            // TODO(fabric2seer): static `CursorTypes.RESIZE_EW` has no Rust counterpart in the port's CursorTypes file
                            // TODO(fabric2seer): `GuiGraphicsExtractor.requestCursor` has no Rust method in the port (looked for `request_cursor` in GuiGraphicsExtractor and its supertypes)
                            // rust: graphics.request_cursor(&CursorTypes::RESIZE_EW);
                            break 'rect_loop;
                        } else {
                            // java: resizeHover.first(null);
                            compat::write(__this, |__t| __t.resize_hover.0 = None);
                        }
                    }
                }
            }
        }
    }

    /// java: @Override protected void init()
    pub fn init(__this: &compat::Shared<Self>) {
        ();
        FancyStatusBars::update_positions(&compat::read(__this, |__t| __t.bars.clone()), true);
        compat::write(__this, |__t| __t.bar_children.clear());
        { let __v = Some(EditBarWidget::new(0, 0, __this.clone())); compat::write(__this, |__t| __t.edit_bar_widget = __v); }
        __this.lock().borrow_mut().edit_bar_widget.as_mut().unwrap().visible = false;
        // java: addWidget(editBarWidget); - the screen holds the widget itself and hands it its clicks first (`mouseClicked`)
        let mut values: Vec<compat::Shared<dyn StatusBar>> = compat::read(__this, |__t| __t.bars.clone()).lock().borrow().status_bars.values().cloned().collect::<Vec<_>>();
        for __a0 in values.iter() {
            StatusBarsConfigScreen::setup(__this, __a0);
        }
        StatusBarsConfigScreen::update_screen_rects(__this);
        // java: this.addRenderableWidget(Button.builder(Component.literal("?"), _ -> minecraft.gui.setScreen(new PopupScreen.Builder(this,
        //   Component.translatable("skyblocker.bars.config.explanationTitle")).addButton(Component.translatable("gui.ok"), PopupScreen::onClose)
        //   .addMessage(Component.translatable("skyblocker.bars.config.explanation")).build())).bounds(width - 20, height / 2 - 17, 15, 15).build());
        // rust: port gap (GAPS_L G3) - a mod cannot build the game's `PopupScreen`: the game's confirm dialog stands in, both
        // of its buttons closing it (`PopupScreen::onClose`)
        let (__w, __h) = compat::read(__this, |__t| (__t.width, __t.height));
        let __explain = mcrust::net::minecraft::client::gui::components::button::builder(component::literal("?").get_string(), mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress::Lambda(mcrust::net::minecraft::client::gui::components::abstract_widget::PressLambda::new(|_, _| {
            let popup = compat::ConfirmScreen::new(|_: bool| mcrust::mcrust::modding::game::set_screen_to_parent(), component::translatable("skyblocker.bars.config.explanationTitle"), component::translatable("skyblocker.bars.config.explanation"), component::translatable("gui.ok"), component::translatable("gui.ok"));
            mcrust::mcrust::modding::game::set_screen_keeping_parent(Box::new(popup));
        }))).bounds(__w.wrapping_sub(20), (__h.wrapping_div(2)).wrapping_sub(17), 15, 15).build();
        compat::write(__this, |__t| __t.add_renderable_widget(__explain));
        // java: this.addRenderableWidget(Button.builder(Component.literal("⟲"), _ -> minecraft.gui.setScreen(new PopupScreen.Builder(this,
        //   Component.translatable("skyblocker.bars.config.resetTitle")).addButton(CommonComponents.GUI_NO, PopupScreen::onClose)
        //   .addButton(CommonComponents.GUI_YES, popup -> { bars.resetBarPositions(); popup.onClose(); })
        //   .addMessage(Component.translatable("skyblocker.bars.config.reset")).build())).bounds(width - 20, height / 2 + 2, 15, 15).build());
        // rust: port gap (GAPS_L G3) - the game's confirm dialog stands in for the `PopupScreen` (YES resets, both close it)
        let __bars = compat::RenderThread::new(compat::read(__this, |__t| __t.bars.clone()));
        let __reset = mcrust::net::minecraft::client::gui::components::button::builder(component::literal("⟲").get_string(), mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress::Lambda(mcrust::net::minecraft::client::gui::components::abstract_widget::PressLambda::new(move |_, _| {
            let __bars = compat::RenderThread::new(__bars.get().clone());
            let popup = compat::ConfirmScreen::new(move |yes: bool| {
                if yes {
                    FancyStatusBars::reset_bar_positions(__bars.get());
                }
                mcrust::mcrust::modding::game::set_screen_to_parent();
            }, component::translatable("skyblocker.bars.config.resetTitle"), component::translatable("skyblocker.bars.config.reset"), component::translatable(mcrust::net::minecraft::client::gui::screens::confirm_screen::GUI_YES_KEY), component::translatable(mcrust::net::minecraft::client::gui::screens::confirm_screen::GUI_NO_KEY));
            mcrust::mcrust::modding::game::set_screen_keeping_parent(Box::new(popup));
        }))).bounds(__w.wrapping_sub(20), (__h.wrapping_div(2)).wrapping_add(2), 15, 15).build();
        compat::write(__this, |__t| __t.add_renderable_widget(__reset));
    }

    /// java: private void setup(StatusBar statusBar)
    pub fn setup(__this: &compat::Shared<Self>, status_bar: &compat::Shared<dyn StatusBar>) {
        // java: addRenderableWidget(statusBar); - a StatusBar is a mod `Renderable + GuiEventListener + LayoutElement`,
        // not an `AbstractWidget`: the screen keeps it in its own list, draws it (`extractRenderState`) and hands it its
        // clicks (`mouseClicked`)
        compat::write(__this, |__t| __t.bar_children.push(status_bar.clone()));
        let __this_w1 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        ({ let __f = compat::dispatch(status_bar, |__t| __t.set_on_click()); __f(status_bar, Some(Arc::new(move |__a0: compat::Shared<dyn StatusBar>, __a1: MouseButtonEvent| { let Some(__this_1) = __this_w1.get().upgrade() else { return Default::default(); }; { StatusBarsConfigScreen::on_bar_click((&__this_1), __a0.clone(), &__a1); } }))) });
    }

    /// java: @Override public void removed()
    pub fn removed(__this: &compat::Shared<Self>) {
        ();
        for status_bar in compat::read(&compat::read(__this, |__t| __t.bars.clone()), |__b| __b.status_bars.values().cloned().collect::<Vec<_>>()) {
            ({ let __f = compat::dispatch(&status_bar, |__t| __t.set_on_click()); __f(&status_bar, None) });
        }
        if compat::read(__this, |__t| __t.cursor_bar.clone()).is_some() {
            { let __v = false; compat::write(&compat::read(__this, |__t| __t.cursor_bar.clone()).unwrap(), |__t| __t.status_bar_base_mut().in_mouse = __v); }
        }
        FancyStatusBars::update_positions(&compat::read(__this, |__t| __t.bars.clone()), false);
        FancyStatusBars::save_bar_config(&compat::read(__this, |__t| __t.bars.clone()));
    }

    /// java: @Override public boolean isPauseScreen()
    pub fn is_pause_screen(__this: &compat::Shared<Self>) -> bool {
        return false;
    }

    /// java: private void onBarClick(StatusBar statusBar, MouseButtonEvent click)
    pub fn on_bar_click(__this: &compat::Shared<Self>, mut status_bar: compat::Shared<dyn StatusBar>, click: &MouseButtonEvent) {
        if click.button() == 0 {
            { let __v = ScreenPosition::new(((((({ let __f = compat::dispatch(&status_bar, |__t| __t.get_x()); __f(&status_bar) }) as f64) - click.x())) as i32), ((((({ let __f = compat::dispatch(&status_bar, |__t| __t.get_y()); __f(&status_bar) }) as f64) - click.y())) as i32)); compat::write(__this, |__t| __t.cursor_offset = __v); }
            { let __v = Some(status_bar.clone()); compat::write(__this, |__t| __t.cursor_bar = __v); }
            { let __v = true; compat::write(&compat::read(__this, |__t| __t.cursor_bar.clone()).unwrap(), |__t| __t.status_bar_base_mut().in_mouse = __v); }
            { let __v = true; compat::write(&compat::read(__this, |__t| __t.cursor_bar.clone()).unwrap(), |__t| __t.status_bar_base_mut().enabled = __v); }
            { let __v = BarLocation::of(&compat::read(__this, |__t| __t.cursor_bar.clone()).unwrap()); compat::write(__this, |__t| __t.current_insert_location = __v); }
            if compat::read(&status_bar, |__t| __t.status_bar_base().anchor.clone()).is_some() {
                { let __bars = compat::read(__this, |__t| __t.bars.clone()); compat::write(&__bars, |__b| __b.bar_positioner.remove_bar_bar_anchor_int_status_bar(compat::read(&status_bar, |__t| __t.status_bar_base().anchor.clone()).unwrap(), compat::read(&status_bar, |__t| __t.status_bar_base().grid_y), &status_bar)); }
            }
            FancyStatusBars::update_positions(&compat::read(__this, |__t| __t.bars.clone()), true);
            ({ let __r = &compat::read(__this, |__t| __t.cursor_bar.clone()).unwrap(); let __f = compat::dispatch(__r, |__t| __t.set_x()); __f(__r, (compat::read(__this, |__t| __t.width)).wrapping_add(5)) });
            StatusBarsConfigScreen::update_screen_rects(__this);
        } else if click.button() == 1 {
            let mut x: i32 = (((click.x() - (1 as f64)).min(((compat::read(__this, |__t| __t.width)).wrapping_sub(__this.lock().borrow().edit_bar_widget.as_ref().unwrap().get_width()) as f64))) as i32);
            let mut y: i32 = (((click.y() - (1 as f64)).min(((compat::read(__this, |__t| __t.height)).wrapping_sub(__this.lock().borrow().edit_bar_widget.as_ref().unwrap().get_height()) as f64))) as i32);
            __this.lock().borrow_mut().edit_bar_widget.as_mut().unwrap().visible = true;
            __this.lock().borrow_mut().edit_bar_widget.as_mut().unwrap().set_status_bar(&status_bar);
            __this.lock().borrow_mut().edit_bar_widget.as_mut().unwrap().set_x(x);
            __this.lock().borrow_mut().edit_bar_widget.as_mut().unwrap().set_y(y);
        }
    }

    /// java: private void updateScreenRects()
    pub fn update_screen_rects(__this: &compat::Shared<Self>) {
        __this.lock().borrow_mut().rect_to_bar.clear();
        for status_bar1 in { compat::read(__this, |__t| __t.bars.clone()).lock().borrow().status_bars.values().cloned().collect::<Vec<_>>() } {
            if !(compat::read(&status_bar1, |__t| __t.status_bar_base().enabled)) {
                continue;
            }
            let __h0 = ScreenRectangle::new(ScreenPosition::new(({ let __f = compat::dispatch(&status_bar1, |__t| __t.get_x()); __f(&status_bar1) }), ({ let __f = compat::dispatch(&status_bar1, |__t| __t.get_y()); __f(&status_bar1) })), ({ let __f = compat::dispatch(&status_bar1, |__t| __t.get_width()); __f(&status_bar1) }), ({ let __f = compat::dispatch(&status_bar1, |__t| __t.get_height()); __f(&status_bar1) }));
            let __h1 = (status_bar1.clone(), BarLocation::of(&status_bar1));
            __this.lock().borrow_mut().rect_to_bar.insert(__h0, __h1);
        }
    }

    /// java: @Override public boolean mouseReleased(MouseButtonEvent click)
    pub fn mouse_released(__this: &compat::Shared<Self>, click: &MouseButtonEvent) -> bool {
        if compat::read(__this, |__t| __t.cursor_bar.clone()).is_some() {
            { let __v = false; compat::write(&compat::read(__this, |__t| __t.cursor_bar.clone()).unwrap(), |__t| __t.status_bar_base_mut().in_mouse = __v); }
            // java: currentInsertLocation == BarLocation.NULL - an identity test on the NULL constant; the port's record is a
            // value, so it is compared by value (BarLocation(null, -1, -1) is only ever NULL itself: a real location has an anchor)
            if compat::read(__this, |__t| __t.current_insert_location.clone()) == *bar_positioner::BAR_LOCATION_NULL {
                { let __v = ((((click.x() + (__this.lock().borrow().cursor_offset.x() as f64)) / (compat::read(__this, |__t| __t.width) as f64))) as f32); compat::write(&compat::read(__this, |__t| __t.cursor_bar.clone()).unwrap(), |__t| __t.status_bar_base_mut().x = __v); }
                { let __v = ((((click.y() + (__this.lock().borrow().cursor_offset.y() as f64)) / (compat::read(__this, |__t| __t.height) as f64))) as f32); compat::write(&compat::read(__this, |__t| __t.cursor_bar.clone()).unwrap(), |__t| __t.status_bar_base_mut().y = __v); }
                { let __v = (compat::read(&compat::read(__this, |__t| __t.cursor_bar.clone()).unwrap(), |__t| __t.status_bar_base().width)).clamp((((BAR_MINIMUM_WIDTH) as f32) / (compat::read(__this, |__t| __t.width) as f32)), (1 as f32)); compat::write(&compat::read(__this, |__t| __t.cursor_bar.clone()).unwrap(), |__t| __t.status_bar_base_mut().width = __v); }
            }
            { let __v = bar_positioner::BAR_LOCATION_NULL.clone(); compat::write(__this, |__t| __t.current_insert_location = __v); }
            { let __v = None; compat::write(__this, |__t| __t.cursor_bar = __v); }
            FancyStatusBars::update_positions(&compat::read(__this, |__t| __t.bars.clone()), true);
            StatusBarsConfigScreen::update_screen_rects(__this);
            return true;
        } else if compat::read(__this, |__t| __t.resizing) {
            { let __v = false; compat::write(__this, |__t| __t.resizing = __v); }
            let mut bar: Option<compat::Shared<dyn StatusBar>> = None;
            if compat::read(__this, |__t| __t.resized_bars.clone()).0.is_some() {
                bar = compat::read(__this, |__t| __t.resized_bars.clone()).0.clone();
            } else if compat::read(__this, |__t| __t.resized_bars.clone()).1.is_some() {
                bar = compat::read(__this, |__t| __t.resized_bars.clone()).1.clone();
            }
            if bar.is_some() && compat::read(bar.as_ref().unwrap(), |__t| __t.status_bar_base().anchor.clone()).is_none() {
                { let __v = (((({ let __r = bar.as_ref().unwrap(); let __f = compat::dispatch(__r, |__t| __t.get_x()); __f(__r) })) as f32) / (compat::read(__this, |__t| __t.width) as f32)); compat::write(bar.as_ref().unwrap(), |__t| __t.status_bar_base_mut().x = __v); }
                { let __v = (((({ let __r = bar.as_ref().unwrap(); let __f = compat::dispatch(__r, |__t| __t.get_width()); __f(__r) })) as f32) / (compat::read(__this, |__t| __t.width) as f32)); compat::write(bar.as_ref().unwrap(), |__t| __t.status_bar_base_mut().width = __v); }
            }
            // java: resizedBars.left(null); resizedBars.right(null);
            compat::write(__this, |__t| __t.resized_bars.0 = None);
            compat::write(__this, |__t| __t.resized_bars.1 = None);
            StatusBarsConfigScreen::update_screen_rects(__this);
            return true;
        }
        return false;
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    pub fn mouse_clicked(__this: &compat::Shared<Self>, click: &MouseButtonEvent, mut doubled: bool) -> bool {
        let mut first: Option<compat::Shared<dyn StatusBar>> = compat::read(__this, |__t| __t.resize_hover.clone()).0.clone();
        if !({ let (__q0, __q1,) = (click.x(), click.y(),); __this.lock().borrow().edit_bar_widget.as_ref().unwrap().is_mouse_over(__q0, __q1) }) && click.button() == 0 && first.is_some() {
            let first_bar: compat::Shared<dyn StatusBar> = first.clone().unwrap();
            let mut bar_anchor: Option<BarAnchor> = compat::read(&first_bar, |__t| __t.status_bar_base().anchor.clone());
            let right_boolean: bool = compat::read(__this, |__t| __t.resize_hover.1);
            let (grid_x, grid_y) = compat::read(&first_bar, |__t| (__t.status_bar_base().grid_x, __t.status_bar_base().grid_y));
            let bars = compat::read(__this, |__t| __t.bars.clone());
            if let Some(bar_anchor) = bar_anchor {
                if right_boolean {
                    // java: resizedBars.left(first);
                    compat::write(__this, |__t| __t.resized_bars.0 = Some(first_bar.clone()));
                    if compat::read(&bars, |__b| __b.bar_positioner.has_neighbor(bar_anchor, grid_y, grid_x, true)) {
                        let neighbor = compat::read(&bars, |__b| __b.bar_positioner.get_bar(bar_anchor, grid_y, grid_x + (if bar_anchor.is_right() { 1 } else { -1 })));
                        compat::write(__this, |__t| __t.resized_bars.1 = Some(neighbor));
                    } else {
                        compat::write(__this, |__t| __t.resized_bars.1 = None);
                    }
                } else {
                    // java: resizedBars.right(first);
                    compat::write(__this, |__t| __t.resized_bars.1 = Some(first_bar.clone()));
                    if compat::read(&bars, |__b| __b.bar_positioner.has_neighbor(bar_anchor, grid_y, grid_x, false)) {
                        let neighbor = compat::read(&bars, |__b| __b.bar_positioner.get_bar(bar_anchor, grid_y, grid_x + (if bar_anchor.is_right() { -1 } else { 1 })));
                        compat::write(__this, |__t| __t.resized_bars.0 = Some(neighbor));
                    } else {
                        compat::write(__this, |__t| __t.resized_bars.0 = None);
                    }
                }
            } else {
                // if they have no anchor no need to do any checking
                if right_boolean {
                    compat::write(__this, |__t| __t.resized_bars.0 = Some(first_bar.clone()));
                    compat::write(__this, |__t| __t.resized_bars.1 = None);
                } else {
                    compat::write(__this, |__t| __t.resized_bars.1 = Some(first_bar.clone()));
                    compat::write(__this, |__t| __t.resized_bars.0 = None);
                }
            }
            { let __v = true; compat::write(__this, |__t| __t.resizing = __v); }
            return true;
        }
        // java: return super.mouseClicked(click, doubled); - `ContainerEventHandler.mouseClicked`: the first child under
        // the click that takes it (`addWidget(editBarWidget)` first, then the bars in the order `setup` added them)
        let mut __edit = compat::write(__this, |__t| __t.edit_bar_widget.take());
        let __taken = __edit.as_mut().is_some_and(|__w| __w.mouse_clicked(click, doubled));
        compat::write(__this, |__t| __t.edit_bar_widget = __edit);
        if __taken {
            return true;
        }
        for __bar in compat::read(__this, |__t| __t.bar_children.clone()) {
            let __f = compat::dispatch(&__bar, |__t| __t.mouse_clicked());
            if __f(&__bar, click, doubled) {
                return true;
            }
        }
        return false;
    }

}

/// StatusBarsConfigScreen's `Screen` half (`super`'s fields and widget tree), lent by the game for each call
impl compat::ScreenHalf for StatusBarsConfigScreen {
    fn screen_half(&mut self) -> &mut mcrust::net::minecraft::client::gui::screens::screen::Screen {
        &mut self.base
    }
    fn screen_parts(&mut self) -> (&mut mcrust::net::minecraft::client::gui::screens::screen::Screen, Option<&mut compat::WidgetCache>) {
        let __s = self;
        (&mut __s.base, Some(&mut __s.__widgets))
    }
}

/// StatusBarsConfigScreen as the game's screen: `game::set_screen(Box::new(compat::ScreenAdapter::new(..)))`
impl ModScreen for compat::ScreenAdapter<StatusBarsConfigScreen> {
    fn get_title(&self) -> String {
        self.title()
    }

    /// java: @Override protected void init()
    fn init(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        self.lend(screen, |__h| StatusBarsConfigScreen::init(__h))
    }

    /// java: Screen.tick (the base body)
    fn tick(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        self.lend(screen, |_| ())
    }

    /// java: @Override public void removed()
    fn removed(&mut self) {
        self.call(|__h| StatusBarsConfigScreen::removed(__h))
    }

    /// java: @Override public void extractRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_render_state(&mut self, screen: &mcrust::net::minecraft::client::gui::screens::screen::Screen, context: &mut ScreenRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        self.lend_ref(screen, |__h| StatusBarsConfigScreen::extract_render_state(__h, &mut context.graphics, mouse_x, mouse_y, a))
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

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    fn mouse_clicked(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, x: f64, y: f64, button: i32) -> bool {
        self.lend(screen, |__h| StatusBarsConfigScreen::mouse_clicked(__h, &MouseButtonEvent::new(x, y, MouseButtonInfo::new(button, 0)), false))
    }

    /// java: @Override public boolean mouseReleased(MouseButtonEvent click)
    fn mouse_released(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, x: f64, y: f64, button: i32) -> bool {
        self.lend(screen, |__h| StatusBarsConfigScreen::mouse_released(__h, &MouseButtonEvent::new(x, y, MouseButtonInfo::new(button, 0))))
    }

    /// java: @Override public boolean isPauseScreen()
    fn is_pause_screen(&self) -> bool {
        self.call(|__h| StatusBarsConfigScreen::is_pause_screen(__h))
    }

}

// NOTE(fabric2seer): extractRenderState, init, isPauseScreen, mouseClicked, mouseReleased, removed override(s) of Screen - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them
