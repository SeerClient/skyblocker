//! Ported from de.hysky.skyblocker.skyblock.dungeon.LeapOverlay by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::configs::dungeons_config::SpiritLeapOverlay;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::dungeon::dungeon_class::DungeonClass;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::dungeon_manager::DungeonManager;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::dungeon_player_manager::DungeonPlayerManager;
use crate::de::hysky::skyblocker::utils::constants;
use crate::de::hysky::skyblocker::utils::container_utils::ContainerUtils;
use crate::de::hysky::skyblocker::utils::item_utils::ItemUtils;
use crate::de::hysky::skyblocker::utils::render::gui_helper::GuiHelper;
use crate::de::hysky::skyblocker::utils::scheduler::message_scheduler;
use crate::de::hysky::skyblocker::utils::scheduler::message_scheduler::MessageScheduler;
use mcrust::java::text::decimal_format_symbols::Locale;
use mcrust::java::util::uuid::Uuid;
use mcrust::mcrust::modding::screens::ModScreen;
use mcrust::mcrust::modding::screens::ScreenRenderContext;
use mcrust::mcrust::modding::widgets::ModWidget;
use mcrust::mcrust::modding::widgets::WidgetRenderContext;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::abstract_widget::WidgetKind;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::layouts::frame_layout;
use mcrust::net::minecraft::client::gui::layouts::grid_layout::GridLayout;
use mcrust::net::minecraft::client::gui::layouts::grid_layout::RowHelper;
use mcrust::net::minecraft::client::gui::layouts::layout_element::LayoutElement;
use mcrust::net::minecraft::client::gui::layouts::linear_layout::LinearLayout;
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::client::input::mouse_button_info::MouseButtonInfo;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::util::argb;
use mcrust::net::minecraft::world::inventory::abstract_container_menu::AbstractContainerMenu;
use mcrust::net::minecraft::world::inventory::container_input::ContainerInput;
use mcrust::net::minecraft::world::item::component::resolvable_profile::ResolvableProfile;
use mcrust::net::minecraft::world::item::items;
use mcrust::org::joml::matrix3x2f_stack::Matrix3x2fStack;
use mcrust::com::mojang::blaze3d::platform::cursor::cursor_type::CursorType;
use mcrust::com::mojang::blaze3d::platform::cursor::cursor_types::CursorTypes;
use mcrust::net::minecraft::world::inventory::chest_menu::ChestMenu;
use mcrust::net::minecraft::world::inventory::container_listener::ContainerListener;
use mcrust::net::minecraft::world::item::item_stack::ItemStack;
use crate::de::hysky::skyblocker::skyblock::dungeon::dungeon_map::DungeonMap;
use mcrust::net::minecraft::core::component::data_component_map::DataComponentGetter as _;
use mcrust::net::minecraft::network::chat::component::Component as _;
use mcrust::net::minecraft::world::level::entity::uniquely_identifyable::UniquelyIdentifyable as _;

/// java: public static final String TITLE = "Spirit Leap";
pub const TITLE: &str = "Spirit Leap";
/// java: private static final Identifier BUTTON = SkyblockerMod.id("button/button");
pub static BUTTON: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("button/button"));
/// java: private static final Identifier BUTTON_HIGHLIGHTED = SkyblockerMod.id("button/button_highlighted");
pub static BUTTON_HIGHLIGHTED: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("button/button_highlighted"));
/// java: private static final Supplier<DungeonsConfig.SpiritLeapOverlay> CONFIG = () -> SkyblockerConfigManager.get().dungeons.leapOverlay;
pub static CONFIG: compat::MainThread<Arc<dyn Fn() -> compat::Shared<SpiritLeapOverlay>>> = compat::MainThread::new(|| Arc::new(move || SkyblockerConfigManager::get().dungeons.leap_overlay.clone()));
/// java: private static final int BUTTON_SPACING = 8;
pub const BUTTON_SPACING: i32 = 8;
/// java: private static final int BUTTON_WIDTH = 130;
pub const BUTTON_WIDTH: i32 = 130;
/// java: private static final int BUTTON_HEIGHT = 50;
pub const BUTTON_HEIGHT: i32 = 50;
/// java: private static final int BORDER_THICKNESS = 2;
pub const PLAYER_BUTTON_BORDER_THICKNESS: i32 = 2;
/// java: private static final int HEAD_SIZE = 24;
pub const PLAYER_BUTTON_HEAD_SIZE: i32 = 24;
/// java: private static final Comparator<PlayerReference> COMPARATOR = Comparator.<PlayerReference, String>comparing(ref -> ref.dungeonClass().displayName()).thenComparing(PlayerReference::name);
/// Compares first by class name then by player name (`String.compareTo`).
pub static PLAYER_REFERENCE_COMPARATOR: compat::MainThread<Arc<dyn Fn(&PlayerReference, &PlayerReference) -> std::cmp::Ordering>> = compat::MainThread::new(|| {
    Arc::new(|a: &PlayerReference, b: &PlayerReference| {
        compat::compare_to(&a.dungeon_class().display_name(), &b.dungeon_class().display_name())
            .cmp(&0)
            .then_with(|| compat::compare_to(&a.name(), &b.name()).cmp(&0))
    })
});

/// java: public class LeapOverlay extends Screen implements ContainerListener
pub struct LeapOverlay {
    /// java: extends net.minecraft.client.gui.screens.Screen
    pub base: mcrust::net::minecraft::client::gui::screens::screen::Screen,
    /// java: private final ChestMenu handler;
    pub handler: compat::LiveChestMenu,
    /// java: private final SortedSet<PlayerReference> references = new TreeSet<>();
    pub references: std::collections::BTreeSet<PlayerReference>,
    /// java: private @Nullable UUID hovered;
    pub hovered: Option<Uuid>,
    /// `handler.getRowCount()`: the count of the menu this screen was built over (the menu itself is reached by
    /// its id, `handler`)
    pub row_count: i32,
    /// java: `implements ContainerListener` - the listener the menu holds is `SlotListener`, which records what
    /// `slotChanged` is handed (it runs inside the packet handler's hold of the player, where this screen cannot be
    /// reached); `drain_slot_changes` runs `slotChanged` with it in this screen's next own call
    pub pending_slot_changes: Arc<Mutex<Vec<(i32, ItemStack)>>>,
    /// the `PlayerButton`s `init` added, in the order of the screen's children (`getChildAt`)
    pub player_buttons: Vec<compat::Shared<PlayerButton>>,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
    /// the screen's widgets as this object last saw them (see `compat::WidgetCache`)
    pub __widgets: compat::WidgetCache,
}

impl std::ops::Deref for LeapOverlay {
    type Target = mcrust::net::minecraft::client::gui::screens::screen::Screen;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for LeapOverlay {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl LeapOverlay {
    /// java: public LeapOverlay(ChestMenu handler)
    pub fn new(mut handler: compat::LiveChestMenu) -> compat::Shared<Self> {
        let __init_handler = handler;
        let __this = compat::shared(Self {
            base: mcrust::net::minecraft::client::gui::screens::screen::Screen::new(component::literal("Skyblocker Leap Overlay").get_string()),
            self_ref: std::sync::Weak::new(),
            __widgets: Default::default(),
            handler: __init_handler,
            references: std::collections::BTreeSet::new(),
            hovered: None,
            row_count: handler.with(|__chest| __chest.get_row_count()).unwrap_or(0),
            pending_slot_changes: Arc::new(Mutex::new(Vec::new())),
            player_buttons: Vec::new(),
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        //   java: handler.addSlotListener(this);
        // The menu `fromPacket` builds is not the player's yet when the screen is made, so it is reachable only as the
        // factory hands it: `new_listening` is this constructor over that menu (it adds the listener).
        __this
    }

    /// java: `new LeapOverlay(handler)` over the `ChestMenu` the screen factory (`MenuScreens.fromPacket`) is handed:
    /// the constructor, then `handler.addSlotListener(this)` on that menu.
    pub fn new_listening(handler: &mut ChestMenu) -> compat::Shared<Self> {
        let __this = LeapOverlay::new(compat::LiveChestMenu::of(handler));
        let row_count = handler.get_row_count();
        compat::write(&__this, |__t| __t.row_count = row_count);
        //   java: handler.addSlotListener(this);
        let pending = compat::read(&__this, |__t| __t.pending_slot_changes.clone());
        add_slot_listener(handler, Box::new(SlotListener { pending }));
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
        let container_id = compat::read(__this, |__t| __t.handler.container_id);
        for (slot_id, stack) in pending.iter() {
            LeapOverlay::slot_changed(__this, container_id, *slot_id, stack);
        }
    }

    /// java: public static boolean shouldShowMap()
    pub fn should_show_map() -> bool {
        return (DungeonManager::is_clearing_dungeon() && compat::read(&(CONFIG.get_mut())(), |__t| __t.show_map));
    }

    /// java: @Override protected void init()
    pub fn init(__this: &compat::Shared<Self>) {
        let mut layout: LinearLayout = LinearLayout::vertical();
        // java: layout.spacing(32).defaultCellSetting().alignHorizontallyCenter(); - `spacing` answers `this`
        layout = layout.spacing(32);
        layout.default_cell_setting().align_horizontally_center();
        if LeapOverlay::should_show_map() {
            layout.add_child({ let __w = compat::WidgetAdapter::widget(MapWidget::new(std::sync::Arc::downgrade(__this), 0, 0)); let (__ww, __wh) = (__w.get_width(), __w.get_height()); LayoutElement::widget(compat::write(__this, |__t| __t.add_widget(__w)), __ww, __wh) });
        }
        let mut grid_widget: GridLayout = GridLayout::new_default();
        grid_widget.spacing(BUTTON_SPACING);
        let mut adder: RowHelper = grid_widget.create_row_helper(2);
        compat::write(__this, |__t| __t.player_buttons.clear());
        for reference in compat::read(__this, |__t| __t.references.clone()) {
            //   java: adder.addChild(new PlayerButton(0, 0, (int) (BUTTON_WIDTH * CONFIG.get().scale), (int) (BUTTON_HEIGHT * CONFIG.get().scale), reference));
            let button = PlayerButton::new(std::sync::Arc::downgrade(__this), 0, 0, ((((BUTTON_WIDTH as f32) * compat::read(&(CONFIG.get_mut())(), |__t| __t.scale))) as i32), ((((BUTTON_HEIGHT as f32) * compat::read(&(CONFIG.get_mut())(), |__t| __t.scale))) as i32), reference.clone());
            compat::write(__this, |__t| __t.player_buttons.push(button.clone()));
            let __w = compat::WidgetAdapter::widget(button);
            let (__ww, __wh) = (__w.get_width(), __w.get_height());
            let __i = compat::write(__this, |__t| __t.add_widget(__w));
            adder.add_child(&mut grid_widget, LayoutElement::widget(__i, __ww, __wh)).unwrap();
        }
        layout.add_child(LayoutElement::Grid(grid_widget.clone()));
        { layout.arrange_elements(); let mut __vw = Vec::new(); LayoutElement::Grid(layout.wrapped.clone()).visit_widgets(&mut __vw); for (__i, __x, __y) in __vw { compat::write(__this, |__t| __t.widget_mut(__i).set_position(__x, __y)); } };
        frame_layout::center_in_rectangle(&mut *compat::LayoutBack::grid(&mut layout.wrapped), 0, 0, compat::read(__this, |__t| __t.width), compat::read(__this, |__t| __t.height));
        { let mut __vw = Vec::new(); LayoutElement::Grid(layout.wrapped.clone()).visit_widgets(&mut __vw); for (__i, _, _) in __vw { compat::write(__this, |__t| __t.mark_renderable(__i)); } };
    }

    /// java: @Override public void slotChanged(AbstractContainerMenu handler, int slotId, ItemStack stack)
    ///
    /// `handler` is the menu's `containerId` (the port's `ContainerListener` is not handed the menu; it is the one
    /// this screen was built over).
    pub fn slot_changed(__this: &compat::Shared<Self>, handler_container_id: i32, mut slot_id: i32, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack) {
        let mut container_slots: i32 = compat::read(__this, |__t| __t.row_count).wrapping_mul(9);
        if slot_id < container_slots && stack.is_item(&items::items().get("player_head")) && stack.has(mcrust::net::minecraft::core::component::data_components::profile()) {
            let mut profile: ResolvableProfile = mcrust::com::mojang::serialization::cast::<ResolvableProfile>(&stack.get(mcrust::net::minecraft::core::component::data_components::profile()).unwrap()).cloned().unwrap();
            let mut uuid: Uuid = profile.partial_profile().id();
            let mut name: String = compat::component_of(stack.get_hover_name()).get_string();
            let mut dungeon_class: DungeonClass = DungeonPlayerManager::get_class_from_player_string(&name);
            // java: PlayerStatus status = switch (ItemUtils.getConcatenatedLore(stack).toLowerCase(Locale.ENGLISH)) {
            //   case String s when s.contains("dead") -> PlayerStatus.DEAD;
            //   case String s when s.contains("offline") -> PlayerStatus.OFFLINE;
            //   default -> null; };
            let lore: String = ItemUtils::get_concatenated_lore(stack).to_lowercase();
            let mut status: Option<PlayerStatus> = if lore.contains("dead") {
                Some(PlayerStatus::Dead)
            } else if lore.contains("offline") {
                Some(PlayerStatus::Offline)
            } else {
                None
            };
            LeapOverlay::update_reference(__this, PlayerReference::new(uuid.clone(), name.clone(), dungeon_class, status, handler_container_id, slot_id));
        }
    }

    /// java: @Override public void dataChanged(AbstractContainerMenu handler, int property, int value)
    pub fn data_changed(__this: &compat::Shared<Self>, handler: &AbstractContainerMenu, mut property: i32, mut value: i32) {
    }

    /// java: private void updateReference(PlayerReference reference)
    pub fn update_reference(__this: &compat::Shared<Self>, mut reference: PlayerReference) {
        __this.lock().borrow_mut().references.remove(&reference);
        let __h0 = reference.clone();
        __this.lock().borrow_mut().references.insert(__h0);
        //   java: rebuildWidgets();
        // `Screen.rebuildWidgets()` with this screen's `init` as its build step: the live screen (lent to this call) is
        // taken out of the object for the port's method, and handed back to the object for `init`.
        let mut __screen = compat::write(__this, |__t| std::mem::replace(&mut __t.base, mcrust::net::minecraft::client::gui::screens::screen::Screen::new(String::new())));
        __screen.rebuild_widgets(&mut |__s: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen| {
            compat::write(__this, |__t| std::mem::swap(&mut __t.base, __s));
            LeapOverlay::init(__this);
            compat::write(__this, |__t| std::mem::swap(&mut __t.base, __s));
        });
        compat::write(__this, |__t| __t.base = __screen);
    }

    /// java: @Override public boolean keyPressed(KeyEvent input)
    pub fn key_pressed(__this: &compat::Shared<Self>, input: &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        if if input.is_escape() && true { LeapOverlay::on_close(__this); true } else { false } {
            return true;
        } else if mcrust::net::minecraft::client::key_mapping::matches("key.inventory", input) {
            LeapOverlay::on_close(__this);
            return true;
        } else if compat::read(&(CONFIG.get_mut())(), |__t| __t.leap_keybinds) {
            return match input.key() {
                49 => LeapOverlay::leap_to_player(__this, 0),
                50 => LeapOverlay::leap_to_player(__this, 1),
                51 => LeapOverlay::leap_to_player(__this, 2),
                52 => LeapOverlay::leap_to_player(__this, 3),
                _ => false,
            };
        }
        return false;
    }

    /// java: private boolean leapToPlayer(int index)
    pub fn leap_to_player(__this: &compat::Shared<Self>, mut index: i32) -> bool {
        let mut players: Vec<PlayerReference> = __this.lock().borrow().references.iter().cloned().collect::<Vec<_>>();
        if (players.len() as i32) > 0 && index < (players.len() as i32) {
            (players[(index) as usize]).click_slot();
            return true;
        }
        return false;
    }

    /// java: @Override public void tick()
    pub fn tick(__this: &compat::Shared<Self>) {
        ();
        use mcrust::net::minecraft::world::entity::entity::Entity as _;
        if !(mcrust::mcrust::modding::game::with_player(|player| mcrust::net::minecraft::world::entity::entity_selector::is_alive(&*player)).unwrap_or(false)) || mcrust::mcrust::modding::game::with_player(|player| player.base().is_removed()).unwrap_or(false) {
            mcrust::mcrust::modding::game::with_player(|player| { player.close_container(); });
        }
    }

    /// java: @Override public void onClose()
    pub fn on_close(__this: &compat::Shared<Self>) {
        mcrust::mcrust::modding::game::with_player(|player| { player.close_container(); });
        mcrust::mcrust::modding::game::set_screen_null();
    }

    /// java: @Override public void removed()
    pub fn removed(__this: &compat::Shared<Self>) {
        if mcrust::mcrust::modding::game::with_player(|_| ()).is_some() {
            //   java: this.handler.removed(this.minecraft.player);
            //   java: this.handler.removeSlotListener(this);
            // (the port's `removed` takes no player - its client body reads none - and `removeSlotListener` removes the
            // client's one listener)
            let handler = compat::read(__this, |__t| __t.handler);
            let _ = handler.with(|__chest| {
                __chest.removed();
                __chest.menu.remove_slot_listener();
            });
        }
    }

}

/// LeapOverlay's `Screen` half (`super`'s fields and widget tree), lent by the game for each call
impl compat::ScreenHalf for LeapOverlay {
    fn screen_half(&mut self) -> &mut mcrust::net::minecraft::client::gui::screens::screen::Screen {
        &mut self.base
    }
    fn screen_parts(&mut self) -> (&mut mcrust::net::minecraft::client::gui::screens::screen::Screen, Option<&mut compat::WidgetCache>) {
        let __s = self;
        (&mut __s.base, Some(&mut __s.__widgets))
    }
}

/// LeapOverlay as the game's screen: `game::set_screen(Box::new(compat::ScreenAdapter::new(..)))`
impl ModScreen for compat::ScreenAdapter<LeapOverlay> {
    fn get_title(&self) -> String {
        self.title()
    }

    /// java: @Override protected void init()
    fn init(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        self.lend(screen, |__h| LeapOverlay::init(__h))
    }

    /// java: @Override public void tick()
    fn tick(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        self.lend(screen, |__h| {
            LeapOverlay::drain_slot_changes(__h);
            LeapOverlay::tick(__h)
        })
    }

    /// java: @Override public void removed()
    fn removed(&mut self) {
        self.call(|__h| LeapOverlay::removed(__h))
    }

    /// java: @Override public boolean keyPressed(KeyEvent input)
    fn key_pressed(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, event: &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        self.lend(screen, |__h| {
            LeapOverlay::drain_slot_changes(__h);
            LeapOverlay::key_pressed(__h, event)
        })
    }

    /// java: Screen.mouseClicked (the base body)
    fn mouse_clicked(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, x: f64, y: f64, button: i32) -> bool {
        self.lend(screen, |__h| {
            LeapOverlay::drain_slot_changes(__h);
            false
        })
    }

    /// java: @Override public void onClose()
    fn on_close(&mut self) {
        self.call(|__h| LeapOverlay::on_close(__h))
    }

}

// NOTE(fabric2seer): dataChanged, init, keyPressed, onClose, removed, slotChanged, tick override(s) of Screen - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them

/// java: public class MapWidget extends AbstractWidget
#[derive(Clone)]
pub struct MapWidget {
    /// java: extends net.minecraft.client.gui.components.AbstractWidget
    pub base: AbstractWidget,
    /// `LeapOverlay.this` (an inner class)
    pub outer: compat::WeakShared<LeapOverlay>,
}

impl std::ops::Deref for MapWidget {
    type Target = AbstractWidget;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for MapWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl MapWidget {
    /// java: public MapWidget(int x, int y)
    pub fn new(outer: compat::WeakShared<LeapOverlay>, mut x: i32, mut y: i32) -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            outer,
            base: AbstractWidget::new(x, y, ((((128 as f32) * compat::read(&(CONFIG.get_mut())(), |__t| __t.scale))) as i32), ((((128 as f32) * compat::read(&(CONFIG.get_mut())(), |__t| __t.scale))) as i32), (&component::translatable("skyblocker.config.dungeons.map.fancyMap").get_string()).to_string(), WidgetKind::PlainText, OnPress::Inert),
        });
        __this
    }

    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_widget_render_state(__this: &compat::Shared<Self>, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        //   java: LeapOverlay.this.hovered = DungeonMap.extractRenderState(graphics, getX(), getY(), CONFIG.get().scale, true, mouseX - getX(), mouseY - getY(), getChildAt(mouseX, mouseY).filter(PlayerButton.class::isInstance).map(PlayerButton.class::cast).map(p -> p.reference.uuid()).orElse(null));
        let (x, y, outer) = compat::read(__this, |__t| (__t.get_x(), __t.get_y(), __t.outer.clone()));
        // `getChildAt(mouseX, mouseY)` - the screen's first child the mouse is over: this widget (added first), else
        // the player buttons in the order `init` added them (their boxes as the game last lent them)
        let over_this = compat::read(__this, |__t| __t.base.is_mouse_over(mouse_x as f64, mouse_y as f64));
        let child_uuid: Option<Uuid> = if over_this {
            None
        } else {
            outer.upgrade().and_then(|outer| {
                let buttons = compat::read(&outer, |__t| __t.player_buttons.clone());
                buttons.iter().find(|button| compat::read(button, |__b| __b.base.is_mouse_over(mouse_x as f64, mouse_y as f64))).map(|button| compat::read(button, |__b| __b.reference.uuid()))
            })
        };
        let hovered = DungeonMap::extract_render_state(graphics, x, y, compat::read(&(CONFIG.get_mut())(), |__t| __t.scale), true, mouse_x.wrapping_sub(x), mouse_y.wrapping_sub(y), child_uuid);
        if let Some(outer) = outer.upgrade() {
            compat::write(&outer, |__t| __t.hovered = hovered);
        }
        GuiHelper::border(graphics, __this.lock().borrow().get_x(), __this.lock().borrow().get_y(), ((((128 as f32) * compat::read(&(CONFIG.get_mut())(), |__t| __t.scale))) as i32), ((((128 as f32) * compat::read(&(CONFIG.get_mut())(), |__t| __t.scale))) as i32), mcrust::net::minecraft::util::common_colors::WHITE);
    }

    /// java: @Override public void onClick(MouseButtonEvent click, boolean doubled)
    pub fn on_click(__this: &compat::Shared<Self>, click: &MouseButtonEvent, mut doubled: bool) {
        //   java: if (LeapOverlay.this.hovered == null) return;
        let Some(outer) = compat::read(__this, |__t| __t.outer.upgrade()) else {
            return;
        };
        let Some(hovered) = compat::read(&outer, |__t| __t.hovered.clone()) else {
            return;
        };
        //   java: assert minecraft.player != null && minecraft.gameMode != null;
        //   java: references.stream().filter(ref -> ref.uuid().equals(LeapOverlay.this.hovered)).findAny().ifPresent(PlayerReference::clickSlot);
        let reference = compat::read(&outer, |__t| __t.references.iter().find(|r| r.uuid() == hovered).cloned());
        if let Some(reference) = reference {
            reference.click_slot();
        }
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    pub fn update_widget_narration(__this: &compat::Shared<Self>, builder: &dyn NarrationElementOutput) {
    }

}

/// MapWidget's widget half (`super`'s fields: x, y, size, message, ...), lent by the game for each call
impl compat::WidgetHalf for MapWidget {
    fn widget_half(&mut self) -> &mut AbstractWidget {
        &mut self.base
    }
}

/// MapWidget as the game's widget: `compat::WidgetAdapter::widget(handle)` where Java hands it on
impl ModWidget for compat::WidgetAdapter<MapWidget> {
    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_widget_render_state(&mut self, widget: &AbstractWidget, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        self.lend_ref(widget, |__h| MapWidget::extract_widget_render_state(__h, &mut context.graphics, mouse_x, mouse_y, a))
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    fn update_widget_narration(&mut self, widget: &AbstractWidget, output: &mut dyn NarrationElementOutput) {
        self.lend_ref(widget, |__h| MapWidget::update_widget_narration(__h, output))
    }

    /// java: @Override public void onClick(MouseButtonEvent click, boolean doubled)
    fn on_click(&mut self, widget: &mut AbstractWidget, event: &MouseButtonEvent, double_click: bool) {
        self.lend(widget, |__h| MapWidget::on_click(__h, event, double_click))
    }

}

// NOTE(fabric2seer): extractWidgetRenderState, onClick, updateWidgetNarration override(s) of AbstractWidget - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them

/// java: private class PlayerButton extends Button
pub struct PlayerButton {
    /// java: extends net.minecraft.client.gui.components.Button
    pub base: AbstractWidget,
    /// java: private final PlayerReference reference;
    pub reference: PlayerReference,
    /// `LeapOverlay.this` (an inner class)
    pub outer: compat::WeakShared<LeapOverlay>,
}

impl std::ops::Deref for PlayerButton {
    type Target = AbstractWidget;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for PlayerButton {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl PlayerButton {
    /// java: private PlayerButton(int x, int y, int width, int height, PlayerReference reference)
    pub fn new(outer: compat::WeakShared<LeapOverlay>, mut x: i32, mut y: i32, mut width: i32, mut height: i32, mut reference: PlayerReference) -> compat::Shared<Self> {
        //   java: super(x, y, width, height, Component.empty(), _ -> {}, _ -> Component.empty());
        // (the press is `onClick`'s override and the narration message is empty; the class is the game's widget through
        // `compat::WidgetAdapter`)
        compat::shared(Self {
            base: AbstractWidget::new(x, y, width, height, String::new(), WidgetKind::PlainText, OnPress::Inert),
            reference,
            outer,
        })
    }

    /// java: @Override protected void extractContents(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_contents(&self, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        //   java: Identifier texture = this.isHoveredOrFocused() || reference.uuid().equals(LeapOverlay.this.hovered) ? BUTTON_HIGHLIGHTED : BUTTON;
        let outer_hovered: Option<Uuid> = self.outer.upgrade().and_then(|outer| compat::read(&outer, |__t| __t.hovered.clone()));
        let mut texture: Identifier = if self.is_hovered_or_focused() || outer_hovered.as_ref() == Some(&self.reference.uuid()) { BUTTON_HIGHLIGHTED.get().clone() } else { BUTTON.get().clone() };
        gui_graphics_extractor::blit_sprite_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), &texture, self.get_x(), self.get_y(), self.get_width(), self.get_height());
        let mut scale: f32 = compat::read(&(CONFIG.get_mut())(), |__t| __t.scale);
        let mut base_x: i32 = self.get_x().wrapping_add(PLAYER_BUTTON_BORDER_THICKNESS);
        let mut centre_x: i32 = self.get_x().wrapping_add(self.get_width().wrapping_shr((1) as u32));
        let mut centre_y: i32 = self.get_y().wrapping_add(self.get_height().wrapping_shr((1) as u32));
        let mut half_font_height: i32 = ((((mcrust::net::minecraft::client::gui::font::LINE_HEIGHT as f32) * scale)) as i32).wrapping_shr((1) as u32);
        GuiHelper::player_head(graphics, base_x.wrapping_add(4), centre_y.wrapping_sub(((((PLAYER_BUTTON_HEAD_SIZE as f32) * scale)) as i32).wrapping_shr((1) as u32)), ((((PLAYER_BUTTON_HEAD_SIZE as f32) * scale)) as i32), &self.reference.uuid());
        let __pose_1 = gui_graphics_extractor::push_pose();
        gui_graphics_extractor::pose_translate((centre_x) as f32, (self.get_y().wrapping_add(half_font_height)) as f32);
        gui_graphics_extractor::pose_scale((scale) as f32, (scale) as f32);
        gui_graphics_extractor::centered_text_string(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &self.reference.dungeon_class().display_name(), 0, 0, self.reference.dungeon_class().color());
        gui_graphics_extractor::pop_pose(__pose_1);
        let __pose_2 = gui_graphics_extractor::push_pose();
        gui_graphics_extractor::pose_translate(((((base_x as f32) + ((PLAYER_BUTTON_HEAD_SIZE as f32) * scale)) + (8 as f32))) as f32, (centre_y.wrapping_sub(half_font_height)) as f32);
        gui_graphics_extractor::pose_scale((scale) as f32, (scale) as f32);
        gui_graphics_extractor::draw_formatted_char_sequence(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::literal(&self.reference.name())), 0, 0, mcrust::net::minecraft::util::common_colors::WHITE);
        gui_graphics_extractor::pop_pose(__pose_2);
        if self.reference.status().is_some() {
            let __pose_3 = gui_graphics_extractor::push_pose();
            gui_graphics_extractor::pose_translate((centre_x) as f32, ((self.get_y().wrapping_add(self.get_height())).wrapping_sub(half_font_height.wrapping_mul(3))) as f32);
            gui_graphics_extractor::pose_scale((scale) as f32, (scale) as f32);
            gui_graphics_extractor::centered_text(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(self.reference.status().unwrap().fields().text.clone()), 0, 0, mcrust::net::minecraft::util::common_colors::WHITE);
            gui_graphics_extractor::pop_pose(__pose_3);
            graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), self.get_x(), self.get_y(), self.get_x().wrapping_add(self.get_width()), self.get_y().wrapping_add(self.get_height()), self.reference.status().unwrap().fields().overlay_color);
        }
    }

    /// java: @Override public void onClick(MouseButtonEvent click, boolean doubled)
    pub fn on_click(&self, click: &MouseButtonEvent, mut doubled: bool) {
        self.reference.click_slot();
    }

}

/// PlayerButton's widget half (`super`'s fields), lent by the game for each call
impl compat::WidgetHalf for PlayerButton {
    fn widget_half(&mut self) -> &mut AbstractWidget {
        &mut self.base
    }
}

/// PlayerButton as the game's widget: `compat::WidgetAdapter::widget(handle)`
impl ModWidget for compat::WidgetAdapter<PlayerButton> {
    /// java: `AbstractButton.extractWidgetRenderState` - `extractContents(..)` (the override above), then
    /// `handleCursor(graphics)` (asked after the frame: `handle_cursor` below)
    fn extract_widget_render_state(&mut self, widget: &AbstractWidget, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        self.lend_ref(widget, |__h| compat::read(__h, |__t| __t.extract_contents(&mut context.graphics, mouse_x, mouse_y, a)))
    }

    /// java: `Button.updateWidgetNarration` - `defaultButtonNarrationText(output)`
    fn update_widget_narration(&mut self, widget: &AbstractWidget, output: &mut dyn NarrationElementOutput) {
        widget.default_button_narration_text(output);
    }

    /// java: @Override public void onClick(MouseButtonEvent click, boolean doubled)
    fn on_click(&mut self, widget: &mut AbstractWidget, event: &MouseButtonEvent, double_click: bool) {
        self.lend(widget, |__h| compat::read(__h, |__t| __t.on_click(event, double_click)))
    }

    /// `this.handleCursor(graphics)` from `AbstractButton.extractWidgetRenderState`
    fn handle_cursor(&self, widget: &AbstractWidget, cursors: &CursorTypes) -> Option<CursorType> {
        widget.handle_cursor(cursors)
    }
}

/// java: private record PlayerReference(UUID uuid, String name, DungeonClass dungeonClass, @Nullable PlayerStatus status, int syncId, int slotId) implements Comparable<PlayerReference>
/// `equals` / `hashCode` are the record's overrides (the uuid alone); the `SortedSet` orders by `compareTo`
/// (`Ord`), as Java's `TreeSet` does - the two disagree exactly as they do in Java.
#[derive(Clone, Debug)]
pub struct PlayerReference {
    /// java: UUID uuid
    pub uuid: Uuid,
    /// java: String name
    pub name: String,
    /// java: DungeonClass dungeonClass
    pub dungeon_class: DungeonClass,
    /// java: PlayerStatus status
    pub status: Option<PlayerStatus>,
    /// java: int syncId
    pub sync_id: i32,
    /// java: int slotId
    pub slot_id: i32,
}

impl PlayerReference {
    /// java: the canonical record constructor
    pub fn new(mut uuid: Uuid, mut name: String, mut dungeon_class: DungeonClass, mut status: Option<PlayerStatus>, mut sync_id: i32, mut slot_id: i32) -> Self {
        Self { uuid, name, dungeon_class, status, sync_id, slot_id }
    }

    /// java: the record accessor `uuid()`
    pub fn uuid(&self) -> Uuid {
        self.uuid.clone()
    }

    /// java: the record accessor `name()`
    pub fn name(&self) -> String {
        self.name.clone()
    }

    /// java: the record accessor `dungeonClass()`
    pub fn dungeon_class(&self) -> DungeonClass {
        self.dungeon_class
    }

    /// java: the record accessor `status()`
    pub fn status(&self) -> Option<PlayerStatus> {
        self.status
    }

    /// java: the record accessor `syncId()`
    pub fn sync_id(&self) -> i32 {
        self.sync_id
    }

    /// java: the record accessor `slotId()`
    pub fn slot_id(&self) -> i32 {
        self.slot_id
    }

    /// java: @Override public boolean equals(Object obj)
    pub fn equals(&self, obj: &compat::Object) -> bool {
        // java: return obj instanceof PlayerReference playerRef && uuid.equals(playerRef.uuid);
        obj.downcast_ref::<PlayerReference>().is_some_and(|player_ref| self.uuid == player_ref.uuid)
    }

    /// java: @Override public int hashCode()
    pub fn hash_code(&self) -> i32 {
        return self.uuid.java_hash_code();
    }

    /// java: @Override public int compareTo(LeapOverlay.PlayerReference o)
    pub fn compare_to(&self, o: &PlayerReference) -> i32 {
        return compat::ord_int((PLAYER_REFERENCE_COMPARATOR.get())(self, o));
    }

    /// java: private void clickSlot()
    pub fn click_slot(&self) {
        //   java: CLIENT.gameMode.handleContainerInput(this.syncId(), this.slotId(), ContainerUtils.getContainerClickButton(InputConstants.MOUSE_BUTTON_LEFT), ContainerInput.PICKUP, CLIENT.player);
        handle_container_input(self.sync_id(), self.slot_id(), ContainerUtils::get_container_click_button(0), ContainerInput::Pickup);
        if compat::read(&(CONFIG.get_mut())(), |__t| __t.enable_leap_message) {
            MessageScheduler::send_message_after_cooldown(message_scheduler::INSTANCE.get_mut(), &format!("/pc {}{}", (constants::PREFIX.get_mut())().get_string(), compat::regex::replace_all(&(CONFIG.get_mut())().lock().borrow().leap_message, "\\[name]", &self.name)), true);
        }
    }

}

impl PartialEq for PlayerReference {
    /// `equals(Object)`: the uuid.
    fn eq(&self, other: &PlayerReference) -> bool {
        self.uuid == other.uuid
    }
}

impl Eq for PlayerReference {}

impl PartialOrd for PlayerReference {
    fn partial_cmp(&self, other: &PlayerReference) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PlayerReference {
    /// `compareTo` (`Comparable<PlayerReference>`), which the `TreeSet` orders and finds by.
    fn cmp(&self, other: &PlayerReference) -> std::cmp::Ordering {
        self.compare_to(other).cmp(&0)
    }
}

/// java: private enum PlayerStatus
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PlayerStatus {
    Dead,
    Offline,
}

impl PlayerStatus {
    pub const VALUES: [PlayerStatus; 2] = [PlayerStatus::Dead, PlayerStatus::Offline];

    /// java: values()
    pub fn values() -> Vec<PlayerStatus> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            PlayerStatus::Dead => "DEAD",
            PlayerStatus::Offline => "OFFLINE",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> PlayerStatus {
        match name {
            "DEAD" => PlayerStatus::Dead,
            "OFFLINE" => PlayerStatus::Offline,
            _ => panic!("IllegalArgumentException: No enum constant PlayerStatus.{name}"),
        }
    }

    /// the constant's constructor data (Java keeps it in the constant's fields)
    pub fn fields(self) -> &'static PlayerStatusFields {
        static FIELDS: LazyLock<Vec<PlayerStatusFields>> = LazyLock::new(|| vec![
            PlayerStatusFields::new(component::translatable("text.skyblocker.dead").with_color(mcrust::net::minecraft::util::common_colors::RED), argb::color_alpha_rgb(64, mcrust::net::minecraft::util::common_colors::SOFT_RED)),
            PlayerStatusFields::new(component::translatable("text.skyblocker.offline").with_color(mcrust::net::minecraft::util::common_colors::GRAY), argb::color_alpha_rgb(64, mcrust::net::minecraft::util::common_colors::LIGHT_GRAY)),
        ]);
        &FIELDS[self as usize]
    }

}

impl std::fmt::Display for PlayerStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

/// the constructor data of each PlayerStatus constant
pub struct PlayerStatusFields {
    pub text: MutableComponent,
    pub overlay_color: i32,
}

impl PlayerStatusFields {
    fn new(mut text: MutableComponent, mut overlay_color: i32) -> Self {
        let mut this = Self {
            text: text.clone(),
            overlay_color: overlay_color,
        };
        this
    }

}

/// java: `LeapOverlay implements ContainerListener` - the listener the menu holds. The menu calls it inside the packet
/// handler's hold of the player (`AbstractContainerMenu.broadcastChanges`), where the screen object cannot be reached,
/// so it records the call for `LeapOverlay::drain_slot_changes`.
struct SlotListener {
    pending: Arc<Mutex<Vec<(i32, ItemStack)>>>,
}

impl std::fmt::Debug for SlotListener {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("LeapOverlay")
    }
}

impl ContainerListener for SlotListener {
    /// java: @Override public void slotChanged(AbstractContainerMenu handler, int slotId, ItemStack stack)
    fn slot_changed(&mut self, slot_index: i32, item_stack: &ItemStack) {
        self.pending.lock().unwrap_or_else(|p| p.into_inner()).push((slot_index, item_stack.duplicate()));
    }

    /// java: @Override public void dataChanged(AbstractContainerMenu handler, int property, int value) {}
    fn data_changed(&mut self, id: i32, value: i32) {
        let _ = (id, value);
    }
}

/// `handler.addSlotListener(listener)` on the menu `fromPacket` built (not the player's yet): the port's takes the
/// `Containers` its `broadcastChanges` reads - the chest's own and the player's inventory. The menu half is moved out
/// of the `ChestMenu` for the call (the chest's containers borrow the rest of it) and put back.
fn add_slot_listener(handler: &mut ChestMenu, listener: Box<dyn ContainerListener>) {
    use mcrust::net::minecraft::world::entity::player::player::Player as _;
    let container_id = handler.menu.container_id;
    let mut menu = std::mem::replace(&mut handler.menu, AbstractContainerMenu::new(container_id));
    mcrust::mcrust::modding::game::with_player(|player| {
        let (_, _, inventory) = player.player_mut().get_container_menu_and_inventory();
        let mut containers = handler.containers_for(inventory);
        menu.add_slot_listener(listener, &mut containers);
    });
    handler.menu = menu;
}

/// `CLIENT.gameMode.handleContainerInput(containerId, slotId, buttonNum, containerInput, CLIENT.player)`: the port's
/// `handleContainerInput:443-481` over the player (its `:445-447` check of the container id is made here - the port's
/// body is handed the open menu's own id), then `:480`'s send.
fn handle_container_input(container_id: i32, slot_id: i32, button_num: i32, container_input: ContainerInput) {
    use mcrust::net::minecraft::client::multiplayer::multi_player_game_mode;
    use mcrust::net::minecraft::world::entity::player::player::Player as _;
    let packet = mcrust::mcrust::modding::game::with_player(|player| {
        let open_container_id = player.player().container_menu_container_id();
        if container_id != open_container_id {
            logger().warn(&format!("Ignoring click in mismatching container. Click in {}, player has {}.", container_id, open_container_id));
            return None;
        }
        multi_player_game_mode::handle_container_input_on_player(&mut compat::minecraft().game_mode, player, slot_id, button_num, container_input)
    })
    .flatten();
    multi_player_game_mode::send_container_click(packet, container_input, slot_id, button_num);
}
