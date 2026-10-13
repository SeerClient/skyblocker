//! Ported from de.hysky.skyblocker.skyblock.radialMenu.RadialMenuScreen by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::radial_menu::radial_button::RadialButton;
use crate::de::hysky::skyblocker::skyblock::radial_menu::radial_menu::RadialMenu;
use crate::de::hysky::skyblocker::utils::container_utils::ContainerUtils;
use mcrust::mcrust::modding::screens::ModScreen;
use mcrust::mcrust::modding::screens::ScreenRenderContext;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::abstract_widget::PressLambda;
use mcrust::net::minecraft::client::gui::components::button;
use mcrust::net::minecraft::client::gui::components::tooltip::Tooltip;
use mcrust::net::minecraft::client::gui::gui;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::navigation::screen_direction::ScreenDirection;
use mcrust::net::minecraft::client::gui::screens::inventory::container_screen::ContainerScreen;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::client::input::mouse_button_info::MouseButtonInfo;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::world::inventory::abstract_container_menu::AbstractContainerMenu;
use mcrust::net::minecraft::world::inventory::container_input::ContainerInput;
use mcrust::net::minecraft::world::inventory::chest_menu::ChestMenu;
use mcrust::net::minecraft::world::inventory::container_listener::ContainerListener;
use mcrust::net::minecraft::world::item::item_stack::ItemStack;
use mcrust::net::minecraft::client::gui::components::events::gui_event_listener::GuiEventListener as _;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final int INTERNAL_RADIUS = 55;
pub const INTERNAL_RADIUS: i32 = 55;
/// java: private static final int EXTERNAL_RADIUS = 110;
pub const EXTERNAL_RADIUS: i32 = 110;
/// java: private static final long NAVIGATION_DIRECTION_COOLDOWN_DELAY = 500;
pub const NAVIGATION_DIRECTION_COOLDOWN_DELAY: i64 = ((500) as i64) as i64;

/// java: public class RadialMenuScreen extends Screen implements ContainerListener
pub struct RadialMenuScreen {
    /// java: extends net.minecraft.client.gui.screens.Screen
    pub base: mcrust::net::minecraft::client::gui::screens::screen::Screen,
    /// java: private final ChestMenu handler;
    pub handler: Option<compat::LiveChestMenu>,
    /// java: private final RadialMenu menuType;
    pub menu_type: compat::Shared<dyn RadialMenu>,
    /// java: private final Int2ObjectOpenHashMap<ItemStack> options = new Int2ObjectOpenHashMap<>();
    pub options: std::collections::HashMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack>,
    /// java: private final List<RadialButton> buttons = new ArrayList<>();
    pub buttons: Vec<RadialButton>,
    /// java: private final Component parentName;
    pub parent_name: MutableComponent,
    /// java: private float buttonArcSize;
    pub button_arc_size: f32,
    /// java: private int buttonsHoveredIndex = -1;
    pub buttons_hovered_index: i32,
    /// java: private ScreenDirection lastNavigationDirectionInput = null;
    pub last_navigation_direction_input: Option<ScreenDirection>,
    /// java: private long navigationDirectionLastTime;
    pub navigation_direction_last_time: i64,
    /// java: private int navigationDirection = 1;
    pub navigation_direction: i32,
    /// `handler.getRowCount()`: the count of the menu this screen was built over (the menu itself is reached by
    /// its id, `handler`)
    pub row_count: i32,
    /// java: `implements ContainerListener` - the listener the menu holds is `SlotListener`, which records what
    /// `slotChanged` is handed (it runs inside the packet handler's hold of the player, where this screen cannot be
    /// reached); `drain_slot_changes` runs `slotChanged` with it in this screen's next own call
    pub pending_slot_changes: Arc<Mutex<Vec<(i32, ItemStack)>>>,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
    /// the screen's widgets as this object last saw them (see `compat::WidgetCache`)
    pub __widgets: compat::WidgetCache,
}

impl std::ops::Deref for RadialMenuScreen {
    type Target = mcrust::net::minecraft::client::gui::screens::screen::Screen;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for RadialMenuScreen {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl RadialMenuScreen {
    /// java: public RadialMenuScreen(ChestMenu handler, RadialMenu type, Component title)
    pub fn new(mut handler: compat::LiveChestMenu, mut r#type: compat::Shared<dyn RadialMenu>, mut title: MutableComponent) -> compat::Shared<Self> {
        let __init_menu_type = r#type.clone();
        let __init_parent_name = title.clone();
        let __this = compat::shared(Self {
            base: mcrust::net::minecraft::client::gui::screens::screen::Screen::new(({ let __r = &r#type; let __f = compat::dispatch(__r, |__t| __t.get_title()); __f(__r, &title) }).get_string()),
            self_ref: std::sync::Weak::new(),
            __widgets: Default::default(),
            handler: None,
            menu_type: __init_menu_type,
            options: std::collections::HashMap::new(),
            buttons: Vec::new(),
            parent_name: __init_parent_name,
            button_arc_size: 0.0,
            buttons_hovered_index: -1,
            last_navigation_direction_input: None,
            navigation_direction_last_time: 0,
            navigation_direction: 1,
            row_count: handler.with(|__chest| __chest.get_row_count()).unwrap_or(0),
            pending_slot_changes: Arc::new(Mutex::new(Vec::new())),
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        __this.lock().borrow_mut().options.clear();
        { let __v = Some(handler); compat::write(&__this, |__t| __t.handler = __v); }
        //   java: handler.addSlotListener(this);
        // The menu `fromPacket` builds is not the player's yet when the screen is made, so it is reachable only as the
        // factory hands it: `new_listening` is this constructor over that menu (it adds the listener).
        __this
    }

    /// java: `new RadialMenuScreen(handler, type, title)` over the `ChestMenu` the screen factory
    /// (`MenuScreens.fromPacket`) is handed: the constructor, then `handler.addSlotListener(this)` on that menu.
    pub fn new_listening(handler: &mut ChestMenu, r#type: compat::Shared<dyn RadialMenu>, title: MutableComponent) -> compat::Shared<Self> {
        let __this = RadialMenuScreen::new(compat::LiveChestMenu::of(handler), r#type, title);
        let row_count = handler.get_row_count();
        compat::write(&__this, |__t| __t.row_count = row_count);
        //   java: handler.addSlotListener(this);
        let pending = compat::read(&__this, |__t| __t.pending_slot_changes.clone());
        compat::add_slot_listener(handler, Box::new(compat::QueueSlotListener { pending, name: "RadialMenuScreen" }));
        __this
    }

    /// The `ContainerListener` calls the menu made since this screen's last own call (see `pending_slot_changes`):
    /// `slotChanged(handler, slotId, stack)` for each, in order.
    pub fn drain_slot_changes(__this: &compat::Shared<Self>) {
        let pending: Vec<(i32, ItemStack)> = {
            let queue = compat::read(__this, |__t| __t.pending_slot_changes.clone());
            let mut queue = queue.lock().unwrap_or_else(|p| p.into_inner());
            std::mem::take(&mut *queue)
        };
        for (slot_id, stack) in pending {
            RadialMenuScreen::slot_changed(__this, slot_id, stack);
        }
    }

    /// java: private static boolean validName(ItemStack stack, String validName)
    pub fn valid_name(stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, valid_name: &str) -> bool {
        let Some(mut custom_name) = stack.get_custom_name() else {
            return false;
        };
        return (custom_name.get_string() == valid_name);
    }

    /// java: @Override protected void init()
    pub fn init(__this: &compat::Shared<Self>) {
        ();
        __this.lock().borrow_mut().buttons.clear();
        __this.lock().borrow_mut().clear_widgets();
        let mut angle: f32 = ((0) as f32) as f32;
        let mut index: i32 = 0;
        { let __v = (((((2 as f64) * std::f64::consts::PI) / ((__this.lock().borrow().options.len() as i32) as f64))) as f32); compat::write(__this, |__t| __t.button_arc_size = __v); }
        let mut option_ordered: Vec<(i32, mcrust::net::minecraft::world::item::item_stack::ItemStack)> = __this.lock().borrow().options.iter().map(|(k, v)| (k.clone(), v.clone())).collect::<Vec<_>>().into_iter().collect::<Vec<_>>();
        let mut navigation_names: Vec<String> = ({ let __r = &compat::read(__this, |__t| __t.menu_type.clone()); let __f = compat::dispatch(__r, |__t| __t.get_navigation_item_names()); __f(__r) });
        let mut navigation_entries: Vec<(i32, mcrust::net::minecraft::world::item::item_stack::ItemStack)> = Vec::new();
        for navigation_name in navigation_names.iter() {
            if let Some(__a0) = option_ordered.iter().cloned().filter(|option| RadialMenuScreen::valid_name(&option.1, navigation_name)).next() {
                navigation_entries.push(__a0.clone());
            }
        }
        //   java: int bottom = Math.ceilDiv((optionOrdered.size() - navigationEntries.size()), 2);
        let mut bottom: i32 = -compat::floor_div_i32(-((option_ordered.len() as i32).wrapping_sub(navigation_entries.len() as i32)), 2);
        for entry in navigation_entries.iter() {
            compat::remove_item(&mut option_ordered, &entry.clone());
        }
        for entry in navigation_entries.iter() {
            option_ordered.insert((bottom) as usize, entry.clone());
        }
        for stack in option_ordered.iter() {
            let mut new_button: RadialButton = RadialButton::new(angle, compat::read(__this, |__t| __t.button_arc_size), ((INTERNAL_RADIUS) as f32) as f32, ((EXTERNAL_RADIUS) as f32) as f32, stack.1.clone(), RadialMenuScreen::get_button_hovered(__this, index), stack.0);
            let __h0 = new_button.clone();
            __this.lock().borrow_mut().buttons.push(__h0);
            { let (__q0,) = (RadialButton::widget(new_button.clone()),); __this.lock().borrow_mut().add_renderable_widget(__q0) };
            angle = (angle + compat::read(__this, |__t| __t.button_arc_size));
            index += 1;
        }
        let __this_w1 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        { let (__q0,) = ((button::builder(component::translatable("skyblocker.config.uiAndVisuals.radialMenu.hideButton").get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { let Some(__this_1) = __this_w1.get().upgrade() else { return Default::default(); }; compat::lend_screen(&__this_1, __screen, || { let mut __pidx = compat::WidgetRef::Screen(__index); { RadialMenuScreen::hide((&__this_1), &compat::write((&__this_1), |__t| { let __s = &mut *__t; let __r = &mut __pidx; __s.__widgets.with(&mut __s.base, __r, |__w| __w.clone()) })); } }) })))).tooltip_text(Some((Tooltip::create((component::translatable("skyblocker.config.uiAndVisuals.radialMenu.hideButton.@Tooltip")).get_string())).message.clone())).pos((compat::read(__this, |__t| __t.width)).wrapping_sub(50), (compat::read(__this, |__t| __t.height)).wrapping_sub(25)).size(40, 15).build(),); __this.lock().borrow_mut().add_renderable_widget(__q0) };
    }

    /// java: private BooleanSupplier getButtonHovered(int index)
    pub fn get_button_hovered(__this: &compat::Shared<Self>, mut index: i32) -> Arc<dyn Fn() -> bool> {
        let __this_1 = compat::RenderThread::new(__this.clone());
        return Arc::new(move || (compat::read(__this_1.get(), |__t| __t.buttons_hovered_index) == index));
    }

    /// java: @Override public void mouseMoved(double mouseX, double mouseY)
    pub fn mouse_moved(__this: &compat::Shared<Self>, mut mouse_x: f64, mut mouse_y: f64) {
        //   java: super.mouseMoved(mouseX, mouseY); - `GuiEventListener.mouseMoved`'s empty default (neither `Screen` nor
        //   `AbstractContainerEventHandler` overrides it)
        // rust: port gap - `ModScreen` has no `mouseMoved` slot, so the game never calls this method (GAPS_K)
        if compat::minecraft().gui.screen().is_none() {
            return;
        }
        let mut actual_x: f32 = ((((mouse_x * (2 as f64))) as f32) - (compat::minecraft().gui.screen().unwrap().width as f32));
        let mut actual_y: f32 = ((((mouse_y * (2 as f64))) as f32) - (compat::minecraft().gui.screen().unwrap().height as f32));
        if actual_x > ((compat::minecraft().gui.screen().unwrap().width).wrapping_sub(100) as f32) && actual_y > ((compat::minecraft().gui.screen().unwrap().height).wrapping_sub(50) as f32) {
            { let __v = -1; compat::write(__this, |__t| __t.buttons_hovered_index = __v); }
            return;
        }
        let mut angle: f64 = ((-(actual_x as f64).atan2((actual_y as f64))) + std::f64::consts::PI);
        { let __v = (((angle / (compat::read(__this, |__t| __t.button_arc_size) as f64))) as i32); compat::write(__this, |__t| __t.buttons_hovered_index = __v); }
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    pub fn mouse_clicked(__this: &compat::Shared<Self>, click: &MouseButtonEvent, mut doubled: bool) -> bool {
        if compat::read(__this, |__t| __t.buttons_hovered_index) != -1 && compat::read(__this, |__t| __t.buttons_hovered_index) < (__this.lock().borrow().buttons.len() as i32) {
            RadialMenuScreen::click_slot_2(__this, (__this.lock().borrow().buttons[(compat::read(__this, |__t| __t.buttons_hovered_index)) as usize]).get_linked_slot(), click.button());
        }
        return false;
    }

    /// java: @Override public boolean keyPressed(KeyEvent keyEvent)
    pub fn key_pressed(__this: &compat::Shared<Self>, key_event: &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        match key_event.key() {
            262 => {
                RadialMenuScreen::navigate_direction(__this, ScreenDirection::Right);
            }
            263 => {
                RadialMenuScreen::navigate_direction(__this, ScreenDirection::Left);
            }
            264 => {
                RadialMenuScreen::navigate_direction(__this, ScreenDirection::Down);
            }
            265 => {
                RadialMenuScreen::navigate_direction(__this, ScreenDirection::Up);
            }
            mcrust::com::mojang::blaze3d::platform::input_constants::KEY_RETURN | 32 => {
                RadialMenuScreen::click_slot(__this);
            }
            _ => {
                // TODO(fabric2seer): `Options.keyUp` has no Rust field in the port (searched net.minecraft.client.Options and its supertypes)
                // TODO(fabric2seer): `Options.keyDown` has no Rust field in the port (searched net.minecraft.client.Options and its supertypes)
                // TODO(fabric2seer): `Options.keyLeft` has no Rust field in the port (searched net.minecraft.client.Options and its supertypes)
                // TODO(fabric2seer): `Options.keyRight` has no Rust field in the port (searched net.minecraft.client.Options and its supertypes)
                if mcrust::net::minecraft::client::key_mapping::matches("key.forward", key_event) {
                    RadialMenuScreen::navigate_direction(__this, ScreenDirection::Up);
                } else if mcrust::net::minecraft::client::key_mapping::matches("key.back", key_event) {
                    RadialMenuScreen::navigate_direction(__this, ScreenDirection::Down);
                } else if mcrust::net::minecraft::client::key_mapping::matches("key.left", key_event) {
                    RadialMenuScreen::navigate_direction(__this, ScreenDirection::Left);
                } else if mcrust::net::minecraft::client::key_mapping::matches("key.right", key_event) {
                    RadialMenuScreen::navigate_direction(__this, ScreenDirection::Right);
                } else {
                    return (if key_event.is_escape() && true { RadialMenuScreen::on_close(__this); true } else { false });
                }
            }
        }
        return false;
    }

    /// java: private void navigateDirection(ScreenDirection direction)
    pub fn navigate_direction(__this: &compat::Shared<Self>, mut direction: ScreenDirection) {
        let mut skip_update: bool = (compat::read(__this, |__t| __t.buttons_hovered_index) == -1);
        if compat::read(__this, |__t| __t.buttons_hovered_index) == -1 {
            { let __v = match direction {
                ScreenDirection::Up => 0,
                ScreenDirection::Down => (__this.lock().borrow().buttons.len() as i32).wrapping_div(2),
                ScreenDirection::Left => (((((__this.lock().borrow().buttons.len() as i32) as f64) * 0.75)) as i32),
                ScreenDirection::Right => (((((__this.lock().borrow().buttons.len() as i32) as f64) * 0.25)) as i32),
                _ => unreachable!("switch expression: no case matched"),
            }; compat::write(__this, |__t| __t.buttons_hovered_index = __v); }
        }
        if compat::read(__this, |__t| __t.last_navigation_direction_input.clone()).unwrap() != direction || compat::millis() > (compat::read(__this, |__t| __t.navigation_direction_last_time)).wrapping_add(NAVIGATION_DIRECTION_COOLDOWN_DELAY) {
            { let __v = Some(direction); compat::write(__this, |__t| __t.last_navigation_direction_input = __v); }
            let mut angle: f32 = ((compat::read(__this, |__t| __t.buttons_hovered_index) as f32) * compat::read(__this, |__t| __t.button_arc_size));
            { let __v = match direction {
                ScreenDirection::Up => (if (angle as f64) > std::f64::consts::PI { 1 } else { -1 }),
                ScreenDirection::Down => (if (angle as f64) > std::f64::consts::PI { -1 } else { 1 }),
                ScreenDirection::Left => (if (angle as f64) > (std::f64::consts::PI / (2 as f64)) && (angle as f64) < (std::f64::consts::PI * 1.5) { 1 } else { -1 }),
                ScreenDirection::Right => (if (angle as f64) > (std::f64::consts::PI / (2 as f64)) && (angle as f64) < (std::f64::consts::PI * 1.5) { -1 } else { 1 }),
                _ => unreachable!("switch expression: no case matched"),
            }; compat::write(__this, |__t| __t.navigation_direction = __v); }
        }
        if skip_update {
            return;
        }
        { let __v = (compat::read(__this, |__t| __t.buttons_hovered_index)).wrapping_add(compat::read(__this, |__t| __t.navigation_direction)); compat::write(__this, |__t| __t.buttons_hovered_index = __v); }
        if compat::read(__this, |__t| __t.buttons_hovered_index) < 0 {
            { let __v = (__this.lock().borrow().buttons.len() as i32).wrapping_sub(1); compat::write(__this, |__t| __t.buttons_hovered_index = __v); }
        }
        if compat::read(__this, |__t| __t.buttons_hovered_index) >= (__this.lock().borrow().buttons.len() as i32) {
            { let __v = 0; compat::write(__this, |__t| __t.buttons_hovered_index = __v); }
        }
        { let __v = compat::millis(); compat::write(__this, |__t| __t.navigation_direction_last_time = __v); }
    }

    /// java: private void hide(Button button)
    pub fn hide(__this: &compat::Shared<Self>, button: &AbstractWidget) {
        if mcrust::mcrust::modding::game::with_player(|_| ()).is_none() {
            return;
        }
        //   java: CLIENT.gui.setScreen(new ContainerScreen(handler, CLIENT.player.getInventory(), parentName));
        // rust: port gap - a mod cannot open the game's own `ContainerScreen` over the open menu (`game::set_screen` takes
        // a `ModScreen`; `gui::set_screen` is the client loop's) (GAPS_K)
        // rust: gui::set_screen(&mut ContainerScreen::new(compat::read(__this, |__t| __t.handler.clone()).unwrap(), mcrust::mcrust::modding::game::with_player(|player| player.base.base.get_inventory().clone()).unwrap(), __this.lock().borrow().parent_name.get_string()));
    }

    /// java: private void clickSlot()
    pub fn click_slot(__this: &compat::Shared<Self>) {
        if compat::read(__this, |__t| __t.buttons_hovered_index) != -1 && compat::read(__this, |__t| __t.buttons_hovered_index) < (__this.lock().borrow().buttons.len() as i32) {
            RadialMenuScreen::click_slot_2(__this, (__this.lock().borrow().buttons[(compat::read(__this, |__t| __t.buttons_hovered_index)) as usize]).get_linked_slot(), 0);
        }
    }

    /// java: private void clickSlot(int slotId, int button)
    pub fn click_slot_2(__this: &compat::Shared<Self>, mut slot_id: i32, mut button: i32) {
        if false /* java: compat::minecraft().game_mode == null: never null here */ {
            return;
        }
        //   java: CLIENT.gameMode.handleContainerInput(handler.containerId, slotId + menuType.clickSlotOffset(slotId), ContainerUtils.getContainerClickButton(menuType.remapClickSlotButton(button, slotId + menuType.clickSlotOffset(slotId))), ContainerInput.PICKUP, CLIENT.player);
        let menu_type = compat::read(__this, |__t| __t.menu_type.clone());
        let offset_slot: i32 = slot_id.wrapping_add({ let __f = compat::dispatch(&menu_type, |__t| __t.click_slot_offset()); __f(&menu_type, slot_id) });
        let remapped_button: i32 = { let __f = compat::dispatch(&menu_type, |__t| __t.remap_click_slot_button()); __f(&menu_type, button, offset_slot) };
        let container_id = compat::read(__this, |__t| __t.handler.as_ref().unwrap().container_id);
        compat::handle_container_input(container_id, offset_slot, ContainerUtils::get_container_click_button(remapped_button), ContainerInput::Pickup);
    }

    /// java: @Override public void slotChanged(AbstractContainerMenu handler, int slotId, ItemStack stack)
    ///
    /// (the port's `ContainerListener` is not handed the menu: it is `this.handler`, the one this screen was built over)
    pub fn slot_changed(__this: &compat::Shared<Self>, mut slot_id: i32, mut stack: mcrust::net::minecraft::world::item::item_stack::ItemStack) {
        if compat::read(__this, |__t| __t.handler.clone()).is_none() || slot_id >= compat::read(__this, |__t| __t.row_count).wrapping_mul(9) {
            return;
        }
        if { let __r = &compat::read(__this, |__t| __t.menu_type.clone()); let __f = compat::dispatch(__r, |__t| __t.item_matches()); __f(__r, slot_id, &stack) } {
            let __h0 = stack.clone();
            __this.lock().borrow_mut().options.insert(slot_id, __h0);
            RadialMenuScreen::init(__this);
        }
    }

    /// java: @Override public void extractRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_render_state(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        ();
        gui_graphics_extractor::centered_text_string(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), __this.lock().borrow().get_title(), (compat::read(__this, |__t| __t.width)).wrapping_div(2), ((compat::read(__this, |__t| __t.height)).wrapping_div(2)).wrapping_sub(mcrust::net::minecraft::client::gui::font::LINE_HEIGHT), 0xFFFFFFFFu32 as i32);
        let mut text_width: i32 = compat::font().width(__this.lock().borrow().get_title());
        gui_graphics_extractor::horizontal_line(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), ((compat::read(__this, |__t| __t.width)).wrapping_div(2)).wrapping_sub(text_width.wrapping_div(2)), ((compat::read(__this, |__t| __t.width)).wrapping_div(2)).wrapping_add(text_width.wrapping_div(2)), (compat::read(__this, |__t| __t.height)).wrapping_div(2), 0xFFFFFFFFu32 as i32);
        if compat::read(__this, |__t| __t.buttons_hovered_index) != -1 && compat::read(__this, |__t| __t.buttons_hovered_index) < (__this.lock().borrow().buttons.len() as i32) {
            gui_graphics_extractor::centered_text_string(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &(__this.lock().borrow().buttons[(compat::read(__this, |__t| __t.buttons_hovered_index)) as usize]).get_name(), (compat::read(__this, |__t| __t.width)).wrapping_div(2), ((compat::read(__this, |__t| __t.height)).wrapping_div(2)).wrapping_add(2), 0xFFFFFFFFu32 as i32);
        }
    }

    /// java: @Override public void dataChanged(AbstractContainerMenu handler, int property, int value)
    pub fn data_changed(__this: &compat::Shared<Self>, handler: &AbstractContainerMenu, mut property: i32, mut value: i32) {
    }

    /// java: @Override public void onClose()
    pub fn on_close(__this: &compat::Shared<Self>) {
        if mcrust::mcrust::modding::game::with_player(|_| ()).is_some() {
            mcrust::mcrust::modding::game::with_player(|player| { player.close_container(); });
        }
        mcrust::mcrust::modding::game::set_screen_null();
    }

}

/// RadialMenuScreen's `Screen` half (`super`'s fields and widget tree), lent by the game for each call
impl compat::ScreenHalf for RadialMenuScreen {
    fn screen_half(&mut self) -> &mut mcrust::net::minecraft::client::gui::screens::screen::Screen {
        &mut self.base
    }
    fn screen_parts(&mut self) -> (&mut mcrust::net::minecraft::client::gui::screens::screen::Screen, Option<&mut compat::WidgetCache>) {
        let __s = self;
        (&mut __s.base, Some(&mut __s.__widgets))
    }
}

/// RadialMenuScreen as the game's screen: `game::set_screen(Box::new(compat::ScreenAdapter::new(..)))`
impl ModScreen for compat::ScreenAdapter<RadialMenuScreen> {
    fn get_title(&self) -> String {
        self.title()
    }

    /// java: @Override protected void init()
    fn init(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        self.lend(screen, |__h| RadialMenuScreen::init(__h))
    }

    /// java: Screen.tick (the base body)
    fn tick(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        self.lend(screen, |__h| RadialMenuScreen::drain_slot_changes(__h))
    }

    /// java: @Override public void extractRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_render_state(&mut self, screen: &mcrust::net::minecraft::client::gui::screens::screen::Screen, context: &mut ScreenRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        self.lend_ref(screen, |__h| RadialMenuScreen::extract_render_state(__h, &context.graphics, mouse_x, mouse_y, a))
    }

    /// java: @Override public boolean keyPressed(KeyEvent keyEvent)
    fn key_pressed(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, event: &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        self.lend(screen, |__h| {
            RadialMenuScreen::drain_slot_changes(__h);
            RadialMenuScreen::key_pressed(__h, event)
        })
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    fn mouse_clicked(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, x: f64, y: f64, button: i32) -> bool {
        self.lend(screen, |__h| {
            RadialMenuScreen::drain_slot_changes(__h);
            RadialMenuScreen::mouse_clicked(__h, &MouseButtonEvent::new(x, y, MouseButtonInfo::new(button, 0)), false)
        })
    }

    /// java: @Override public void onClose()
    fn on_close(&mut self) {
        self.call(|__h| RadialMenuScreen::on_close(__h))
    }

}

// NOTE(fabric2seer): dataChanged, extractRenderState, init, keyPressed, mouseClicked, mouseMoved, onClose, slotChanged override(s) of Screen - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them

