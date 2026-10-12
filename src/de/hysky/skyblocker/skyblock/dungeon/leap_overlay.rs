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
// TODO(fabric2seer): expression not parsed
//   java:
/// java: 
pub static PLAYER_REFERENCE_COMPARATOR: compat::MainThread<Arc<dyn Fn(&PlayerReference, &PlayerReference) -> std::cmp::Ordering>> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `COMPARATOR` - see the TODO above"));
/// java: 
pub static PLAYER_REFERENCE_STRING: compat::MainThread<Arc<dyn Fn(&PlayerReference, &PlayerReference) -> std::cmp::Ordering>> = compat::MainThread::new(|| Default::default());

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
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        // TODO(fabric2seer): `add_slot_listener` takes 2 argument(s) in the port (`listener: Box<dyn ContainerListener>, containers: &mut Containers<'_>`), Java passes 1
        // rust: { let _ = handler.with(|__chest| __chest.menu.add_slot_listener(__this)); };
        __this
    }

    /// java: public static boolean shouldShowMap()
    pub fn should_show_map() -> bool {
        return (DungeonManager::is_clearing_dungeon() && compat::read(&(CONFIG.get_mut())(), |__t| __t.show_map));
    }

    /// java: @Override protected void init()
    pub fn init(__this: &compat::Shared<Self>) {
        let mut layout: LinearLayout = LinearLayout::vertical();
        layout.spacing(32).default_cell_setting().align_horizontally_center();
        if LeapOverlay::should_show_map() {
            layout.add_child({ let __w = compat::WidgetAdapter::widget(MapWidget::new(0, 0)); let (__ww, __wh) = (__w.get_width(), __w.get_height()); LayoutElement::widget(compat::write(__this, |__t| __t.add_widget(__w)), __ww, __wh) });
        }
        let mut grid_widget: GridLayout = GridLayout::new_default().spacing(BUTTON_SPACING);
        let mut adder: RowHelper = grid_widget.create_row_helper(2);
        for reference in compat::read(__this, |__t| __t.references.clone()) {
            // TODO(fabric2seer): the port's `add_child` ports this Java method with other parameters (grid, child for Java's widget)
            // TODO(fabric2seer): `add_child` takes 2 argument(s) in the port (`grid: &mut GridLayout, child: LayoutElement`), Java passes 1
            // rust: adder.add_child(&mut PlayerButton::new(0, 0, ((((BUTTON_WIDTH as f32) * compat::read(&(CONFIG.get_mut())(), |__t| __t.scale))) as i32), ((((BUTTON_HEIGHT as f32) * compat::read(&(CONFIG.get_mut())(), |__t| __t.scale))) as i32), reference.clone())).unwrap();
        }
        layout.add_child(LayoutElement::Grid(grid_widget.clone()));
        { layout.arrange_elements(); let mut __vw = Vec::new(); LayoutElement::Grid(layout.wrapped.clone()).visit_widgets(&mut __vw); for (__i, __x, __y) in __vw { compat::write(__this, |__t| __t.widget_mut(__i).set_position(__x, __y)); } };
        frame_layout::center_in_rectangle(&mut *compat::LayoutBack::grid(&mut layout.wrapped), 0, 0, compat::read(__this, |__t| __t.width), compat::read(__this, |__t| __t.height));
        { let mut __vw = Vec::new(); LayoutElement::Grid(layout.wrapped.clone()).visit_widgets(&mut __vw); for (__i, _, _) in __vw { compat::write(__this, |__t| __t.mark_renderable(__i)); } };
    }

    /// java: @Override public void slotChanged(AbstractContainerMenu handler, int slotId, ItemStack stack)
    pub fn slot_changed(__this: &compat::Shared<Self>, handler: &AbstractContainerMenu, mut slot_id: i32, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack) {
        let mut container_slots: i32 = (__this.lock().borrow().handler.with(|__chest| __chest.get_row_count()).unwrap()).wrapping_mul(9);
        if slot_id < container_slots && stack.is_item(&items::items().get("player_head")) && stack.has(mcrust::net::minecraft::core::component::data_components::profile()) {
            let mut profile: ResolvableProfile = stack.get(mcrust::net::minecraft::core::component::data_components::profile()).unwrap();
            let mut uuid: Uuid = profile.partial_profile().id();
            let mut name: String = compat::component_of(stack.get_hover_name()).get_string();
            let mut dungeon_class: DungeonClass = DungeonPlayerManager::get_class_from_player_string(&name);
            // TODO(fabric2seer): a pattern switch expression
            //   java: PlayerStatus status = switch (ItemUtils.getConcatenatedLore(stack).toLowerCase(Locale.ENGLISH)) {
            //   java: 				case String s when s.contains("dead") -> PlayerStatus.DEAD;
            //   java: 				case String s when s.contains("offline") -> PlayerStatus.OFFLINE;
            //   java: 				default -> null;
            //   java: 			};
            let mut status: PlayerStatus = todo!("fabric2seer: the initialiser of `status` (TODO above)");
            LeapOverlay::update_reference(__this, PlayerReference::new(uuid.clone(), name.clone(), dungeon_class, Some(status), handler.container_id, slot_id));
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
        // TODO(fabric2seer): `rebuild_widgets` takes 1 argument(s) in the port (`build: &mut dyn FnMut(&mut Screen)`), Java passes 0
        // rust: __this.lock().borrow_mut().rebuild_widgets();
    }

    /// java: @Override public boolean keyPressed(KeyEvent input)
    pub fn key_pressed(__this: &compat::Shared<Self>, input: &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        // TODO(fabric2seer): `Options.keyInventory` has no Rust field in the port (searched net.minecraft.client.Options and its supertypes)
        if if input.is_escape() && true { LeapOverlay::on_close(__this); true } else { false } {
            return true;
        } else if compat::minecraft().options.borrow().key_inventory.matches(input) {
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
        if !(mcrust::mcrust::modding::game::with_player(|player| player.base.base.base.is_alive()).unwrap_or(false)) || mcrust::mcrust::modding::game::with_player(|player| player.is_removed()).unwrap_or(false) {
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
            // TODO(fabric2seer): `removed` takes 0 argument(s) in the port (``), Java passes 1
            // TODO(fabric2seer): the mod keeps the game's `LocalPlayer` (Java holds a reference to the live object); the port's `LocalPlayer` is not a value to copy and the port lends the mod no handle to it
            // rust: mcrust::mcrust::modding::game::with_player(|player| { { let _ = __this.lock().borrow().handler.with(|__chest| __chest.removed(player.clone())); }; });
            // TODO(fabric2seer): `remove_slot_listener` takes 0 argument(s) in the port (``), Java passes 1
            // rust: { let _ = __this.lock().borrow().handler.with(|__chest| __chest.menu.remove_slot_listener(__this.clone())); };
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
        self.lend(screen, |__h| LeapOverlay::tick(__h))
    }

    /// java: @Override public void removed()
    fn removed(&mut self) {
        self.call(|__h| LeapOverlay::removed(__h))
    }

    /// java: @Override public boolean keyPressed(KeyEvent input)
    fn key_pressed(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, event: &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        self.lend(screen, |__h| LeapOverlay::key_pressed(__h, event))
    }

    /// java: Screen.mouseClicked (the base body)
    fn mouse_clicked(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, x: f64, y: f64, button: i32) -> bool {
        self.lend(screen, |_| false)
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
    pub fn new(mut x: i32, mut y: i32) -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            base: AbstractWidget::new(x, y, ((((128 as f32) * compat::read(&(CONFIG.get_mut())(), |__t| __t.scale))) as i32), ((((128 as f32) * compat::read(&(CONFIG.get_mut())(), |__t| __t.scale))) as i32), (&component::translatable("skyblocker.config.dungeons.map.fancyMap").get_string()).to_string(), WidgetKind::PlainText, OnPress::Inert),
        });
        __this
    }

    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_widget_render_state(__this: &compat::Shared<Self>, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        // TODO(fabric2seer): `LeapOverlay.this` (the enclosing instance of an inner class)
        //   java: LeapOverlay.this.hovered = DungeonMap.extractRenderState(graphics, getX(), getY(), CONFIG.get().scale, true, mouseX - getX(), mouseY - getY(), getChildAt(mouseX, mouseY).filter(PlayerButton.class::isInstance).map(PlayerButton.class::cast).map(p -> p.reference.uuid()).orElse(null));
        GuiHelper::border(graphics, __this.lock().borrow().get_x(), __this.lock().borrow().get_y(), ((((128 as f32) * compat::read(&(CONFIG.get_mut())(), |__t| __t.scale))) as i32), ((((128 as f32) * compat::read(&(CONFIG.get_mut())(), |__t| __t.scale))) as i32), mcrust::net::minecraft::util::common_colors::WHITE);
    }

    /// java: @Override public void onClick(MouseButtonEvent click, boolean doubled)
    pub fn on_click(__this: &compat::Shared<Self>, click: &MouseButtonEvent, mut doubled: bool) {
        // TODO(fabric2seer): the condition: `LeapOverlay.this` (the enclosing instance of an inner class)
        //   java: LeapOverlay.this.hovered == null
        if todo!("fabric2seer: the condition (TODO above)") {
            return;
        }
        // TODO(fabric2seer): unknown name `minecraft`
        //   java: assert minecraft.player != null && minecraft.gameMode != null;
        // TODO(fabric2seer): `references` is a field of the enclosing instance (LeapOverlay.this) - an inner class has no outer reference in Rust
        //   java: references.stream()
        //   java: 					.filter(ref -> ref.uuid().equals(LeapOverlay.this.hovered))
        //   java: 					.findAny()
        //   java: 					.ifPresent(PlayerReference::clickSlot);
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
    pub fn new(mut x: i32, mut y: i32, mut width: i32, mut height: i32, mut reference: PlayerReference) -> Self {
        let __init_reference = reference.clone();
        // TODO(fabric2seer): `new Button(..)`: the port has no `Button::new`
        //   java: super(x, y, width, height, Component.empty(), _ -> {}, _ -> Component.empty());
        let mut this: Self = todo!("constructor");
        this
    }

    /// java: @Override protected void extractContents(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_contents(&self, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        // TODO(fabric2seer): `LeapOverlay.this` (the enclosing instance of an inner class)
        //   java: Identifier texture = this.isHoveredOrFocused() || reference.uuid().equals(LeapOverlay.this.hovered) ? BUTTON_HIGHLIGHTED : BUTTON;
        let mut texture: Identifier = todo!("fabric2seer: the initialiser of `texture` (TODO above)");
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

// NOTE(fabric2seer): extractContents, onClick override(s) of Button - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them

/// java: private record PlayerReference(UUID uuid, String name, DungeonClass dungeonClass, @Nullable PlayerStatus status, int syncId, int slotId) implements Comparable<PlayerReference>
#[derive(Clone, Debug, PartialEq)]
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
        // TODO(fabric2seer): instanceof with a pattern binding outside an `if` condition
        //   java: return obj instanceof PlayerReference playerRef && uuid.equals(playerRef.uuid);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public int hashCode()
    pub fn hash_code(&self) -> i32 {
        return self.uuid.java_hash_code();
    }

    /// java: @Override public int compareTo(LeapOverlay.PlayerReference o)
    pub fn compare_to(&self, o: &PlayerReference) -> i32 {
        return (PLAYER_REFERENCE_COMPARATOR.get_mut())(self.clone(), o.clone());
    }

    /// java: private void clickSlot()
    pub fn click_slot(&self) {
        // TODO(fabric2seer): `MultiPlayerGameMode.handleContainerInput` has no Rust method in the port (looked for `handle_container_input` in MultiPlayerGameMode and its supertypes)
        // rust: mcrust::mcrust::modding::game::with_player(|player| { compat::minecraft().game_mode.handle_container_input(self.sync_id(), self.slot_id(), ContainerUtils::get_container_click_button(0), ContainerInput::Pickup, player); });
        if compat::read(&(CONFIG.get_mut())(), |__t| __t.enable_leap_message) {
            MessageScheduler::send_message_after_cooldown(message_scheduler::INSTANCE.get_mut(), &format!("/pc {}{}", (constants::PREFIX.get_mut())().get_string(), compat::regex::replace_all(&(CONFIG.get_mut())().lock().borrow().leap_message, "\\[name]", &self.name)), true);
        }
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
            text: Default::default(),
            overlay_color: 0,
        };
        this.text = text.clone();
        this.overlay_color = overlay_color;
        this
    }

}
