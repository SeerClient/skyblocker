//! Ported from de.hysky.skyblocker.skyblock.storageoverlay.StorageOverlayScreen by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::dungeon::partyfinder::finder_settings_container::ContainerWidgetAdapter;
use crate::de::hysky::skyblocker::skyblock::dungeon::partyfinder::finder_settings_container::ContainerWidgetHalf;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::item::background::item_background_manager::ItemBackgroundManager;
use crate::de::hysky::skyblocker::skyblock::item::item_protection::ItemProtection;
use crate::de::hysky::skyblocker::skyblock::item::slottext::slot_text_manager::SlotTextManager;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::backpack_preview::BackpackPreview;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::backpack_preview::Storage;
use crate::de::hysky::skyblocker::skyblock::storageoverlay::storage_overlay_screen_handler::StorageOverlayScreenHandler;
use crate::de::hysky::skyblocker::utils::item_utils::ItemUtils;
use crate::de::hysky::skyblocker::utils::render::gui::searchable_grid_widget::SearchableGridWidget;
use crate::de::hysky::skyblocker::utils::render::gui::searchable_grid_widget::SearchableGridWidgetBase;
use crate::de::hysky::skyblocker::utils::render::texture::fallbacked_texture;
use crate::de::hysky::skyblocker::utils::render::texture::fallbacked_texture::FallbackedTexture;
use crate::de::hysky::skyblocker::utils::scheduler::message_scheduler;
use crate::de::hysky::skyblocker::utils::scheduler::message_scheduler::MessageScheduler;
use mcrust::java::text::decimal_format_symbols::Locale;
use mcrust::java::util::bit_set::BitSet;
use mcrust::mcrust::modding::container_screens::ModContainerScreen;
use mcrust::mcrust::modding::widgets::ModWidget;
use mcrust::mcrust::modding::widgets::WidgetRenderContext;
use mcrust::net::minecraft::client::gui::component_path::ComponentPath;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::abstract_widget::PressLambda;
use mcrust::net::minecraft::client::gui::components::button;
use mcrust::net::minecraft::client::gui::components::edit_box::EditBox;
use mcrust::net::minecraft::client::gui::font::Font;
use mcrust::net::minecraft::client::gui::gui;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::layouts::layout;
use mcrust::net::minecraft::client::gui::layouts::layout_element::LayoutElement;
use mcrust::net::minecraft::client::gui::layouts::linear_layout::LinearLayout;
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use mcrust::net::minecraft::client::gui::navigation::screen_rectangle::ScreenRectangle;
use mcrust::net::minecraft::client::gui::screens::inventory::abstract_container_screen::AbstractContainerScreen;
use mcrust::net::minecraft::client::gui::screens::inventory::container_screen::ContainerScreen;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::client::input::mouse_button_info::MouseButtonInfo;
use mcrust::net::minecraft::client::renderer::rect2i::Rect2i;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::util::argb;
use mcrust::net::minecraft::world::entity::player::inventory::Inventory;
use mcrust::net::minecraft::world::inventory::container_input::ContainerInput;
use mcrust::net::minecraft::world::inventory::slot::Slot;
use mcrust::net::minecraft::client::gui::components::events::gui_event_listener::GuiEventListener as _;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final Identifier TEXTURE = Identifier.withDefaultNamespace("textures/gui/container/generic_54.png");
pub static TEXTURE: compat::MainThread<Identifier> = compat::MainThread::new(|| Identifier::with_default_namespace("textures/gui/container/generic_54.png"));
/// java: private static final FallbackedTexture<Identifier> BACKGROUND = FallbackedTexture.ofGuiSprite( SkyblockerMod.id("storage_overlay/background"), Identifier.withDefaultNamespace("social_interactions/back
pub static BACKGROUND: compat::MainThread<compat::Shared<dyn FallbackedTexture<Identifier>>> = compat::MainThread::new(|| fallbacked_texture::of_gui_sprite(&crate::entry::skyblocker_mod::id("storage_overlay/background"), &Identifier::with_default_namespace("social_interactions/background")));
/// java: private static final Pattern CHANGING_BACKPACK_PATTERN = Pattern.compile("(Placing backpack)|(Removed backpack)");
pub static CHANGING_BACKPACK_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("(Placing backpack)|(Removed backpack)"));
/// java: private static final int HEADER_H = 17;
pub const HEADER_H: i32 = 17;
/// java: private static final int SLOT_SIZE = 18;
pub const SLOT_SIZE: i32 = 18;
/// java: private static final int MAX_ROW_BATCH = 6;
pub const MAX_ROW_BATCH: i32 = 6;
/// java: private static final int MAX_COL_BATCH = 9;
pub const MAX_COL_BATCH: i32 = 9;
/// java: private static final int BOTTOM_V = 215;
pub const BOTTOM_V: i32 = 215;
/// java: private static final int EDGE_PADDING = 7;
pub const EDGE_PADDING: i32 = 7;
/// java: private static final int NOT_MATCHED_COLOR = ARGB.black(0.6f);
pub static NOT_MATCHED_COLOR: LazyLock<i32> = LazyLock::new(|| argb::black_f32(0.6f32));
/// java: protected static int openStorage;
pub static OPEN_STORAGE: LazyLock<Mutex<i32>> = LazyLock::new(|| Mutex::new(0));
/// java: private static double savedScroll = 0;
pub static SAVED_SCROLL: LazyLock<Mutex<f64>> = LazyLock::new(|| Mutex::new(((0) as f64) as f64));
/// java: private static String savedSearch = "";
pub static SAVED_SEARCH: LazyLock<Mutex<String>> = LazyLock::new(|| Mutex::new("".to_string()));
/// java: private static int savedIndex = -1;
pub static SAVED_INDEX: LazyLock<Mutex<i32>> = LazyLock::new(|| Mutex::new(-1));
/// java: private static boolean disableOnNextLoad = false;
pub static DISABLE_ON_NEXT_LOAD: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));
/// java: private static @Nullable Vector2dc previousMousePosition;
pub static PREVIOUS_MOUSE_POSITION: compat::MainThread<Option<crate::unresolved::Vector2dc>> = compat::MainThread::new(|| None);

/// java: public class StorageOverlayScreen extends AbstractContainerScreen<StorageOverlayScreenHandler>
pub struct StorageOverlayScreen {
    /// java: extends net.minecraft.client.gui.screens.inventory.AbstractContainerScreen
    pub base: AbstractContainerScreen,
    /// java: @Nullable private BackpackGridWidget grid;
    pub grid: Option<compat::Shared<BackpackGridWidget>>,
    /// java: private int oldHeight;
    pub old_height: i32,
    /// java: private final StorageOverlayScreenHandler handler;
    pub handler: Option<compat::Shared<StorageOverlayScreenHandler>>,
    /// java: private final Component name;
    pub name: Option<MutableComponent>,
    /// java: private final ChestMenu defaultHandler;
    pub default_handler: Option<compat::LiveChestMenu>,
    /// java: private boolean saveMousePosition;
    pub save_mouse_position: bool,
}

impl std::ops::Deref for StorageOverlayScreen {
    type Target = AbstractContainerScreen;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for StorageOverlayScreen {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl StorageOverlayScreen {
    /// java: public StorageOverlayScreen(StorageOverlayScreenHandler handler, ChestMenu defaultHandler, Component name, Inventory inventory, int height)
    pub fn new(mut handler: compat::Shared<StorageOverlayScreenHandler>, mut default_handler: compat::LiveChestMenu, mut name: MutableComponent, inventory: compat::PlayerInventory, mut height: i32) -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            base: { let mut __s = AbstractContainerScreen::new((&name.get_string()).to_string(), String::new()); __s.image_width = 176; __s.image_height = height; __s },
            grid: None,
            old_height: 0,
            handler: None,
            name: None,
            default_handler: None,
            save_mouse_position: false,
        });
        { let __v = -1000; compat::write(&__this, |__t| __t.title_label_y = __v); }
        { let __v = (compat::read(&__this, |__t| __t.inventory_label_y)).wrapping_add(2); compat::write(&__this, |__t| __t.inventory_label_y = __v); }
        { let __v = Some(handler.clone()); compat::write(&__this, |__t| __t.handler = __v); }
        { let __v = Some(default_handler); compat::write(&__this, |__t| __t.default_handler = __v); }
        { let __v = Some(name.clone()); compat::write(&__this, |__t| __t.name = __v); }
        { let __v = height.wrapping_sub(87); compat::write(&__this, |__t| __t.old_height = __v); }
        __this
    }

    /// java: public static @Nullable Vector2dc getPreviousMousePosition()
    pub fn get_previous_mouse_position() -> Option<crate::unresolved::Vector2dc> {
        let screen_position: Option<crate::unresolved::Vector2dc> = *PREVIOUS_MOUSE_POSITION.get();
        *PREVIOUS_MOUSE_POSITION.get_mut() = None;
        return screen_position;
    }

    /// java: @Init public static void setup()
    pub fn setup() {
        // TODO(fabric2seer): ClientReceiveMessageEvents.ALLOW_GAME.register: no SeerClient equivalent yet
        //   java: ClientReceiveMessageEvents.ALLOW_GAME.register((message, overlay) -> {
        //   java: 			if (Utils.isOnSkyblock() && !overlay && CHANGING_BACKPACK_PATTERN.matcher(message.getString()).find()) {
        //   java: 				disableOnNextLoad = true;
        //   java: 			}
        //   java: 			return true;
        //   java: 		});
        // TODO(fabric2seer): ClientSendMessageEvents.MODIFY_COMMAND.register: no SeerClient equivalent yet
        //   java: ClientSendMessageEvents.MODIFY_COMMAND.register(command -> {
        //   java: 			if (!SkyblockerConfigManager.get().uiAndVisuals.storageOverlay.enabled || savedIndex < 0 || Minecraft.getInstance().gui.screen() instanceof StorageOverlayScreen || !command.equals("storage")) {
        //   java: 				return command;
        //   java: 			} else {
        //   java: 				String c = getCommandForIndex(savedIndex);
        //   java: 				savedIndex = -1;
        //   java: 				return c.substring(1);
        //   java: 			}
        //   java: 		});
    }

    /// java: public static boolean enabled(String title)
    pub fn enabled(title: &str) -> bool {
        compat::set_state(&OPEN_STORAGE, BackpackPreview::get_storage_index_from_title(title));
        let mut enabled: bool = (SkyblockerConfigManager::get().ui_and_visuals.storage_overlay.enabled && (title == "storage" || compat::get_state(&OPEN_STORAGE) != -1) && !(compat::get_state(&DISABLE_ON_NEXT_LOAD)));
        compat::set_state(&DISABLE_ON_NEXT_LOAD, false);
        return enabled;
    }

    /// java: private static String getCommandForIndex(int index)
    pub fn get_command_for_index(mut index: i32) -> String {
        if index <= 8 {
            return format!("/echest {}", index.wrapping_add(1));
        } else {
            return format!("/backpack {}", index.wrapping_sub(8));
        }
    }

    /// java: protected static String getStorageName(int index)
    pub fn get_storage_name(mut index: i32) -> String {
        if index <= 8 {
            return format!("Ender Chest {}", index.wrapping_add(1));
        } else {
            return format!("Backpack {}", index.wrapping_sub(8));
        }
    }

    /// java: protected void switchOpenStorage(int index)
    pub fn switch_open_storage(__this: &compat::Shared<Self>, mut index: i32) {
        if compat::read(__this, |__t| __t.grid.clone()).is_none() {
            return;
        }
        compat::set_state(&SAVED_SCROLL, ({ let __r = &compat::read(__this, |__t| __t.grid.clone()).unwrap(); let __f = compat::dispatch(__r, |__t| __t.get_scroll_amount()); __f(&compat::view(__r, |a| a as compat::Shared<dyn SearchableGridWidget>)) }));
        MessageScheduler::send_message_after_cooldown(message_scheduler::INSTANCE.get_mut(), &StorageOverlayScreen::get_command_for_index(index), true);
        { let __v = SkyblockerConfigManager::get().ui_and_visuals.storage_overlay.do_not_reset_cursor; compat::write(__this, |__t| __t.save_mouse_position = __v); }
    }

    /// java: public void refreshSearch()
    pub fn refresh_search(__this: &compat::Shared<Self>) {
        if compat::read(__this, |__t| __t.grid.clone()).is_none() {
            return;
        }
        ({ let __r = &compat::read(__this, |__t| __t.grid.clone()).unwrap(); let __f = compat::dispatch(__r, |__t| __t.refresh_search()); __f(&compat::view(__r, |a| a as compat::Shared<dyn SearchableGridWidget>)) });
    }

    /// java: private void hide(Button button)
    pub fn hide(__this: &compat::Shared<Self>, button: &AbstractWidget) {
        if mcrust::mcrust::modding::game::with_player(|_| ()).is_none() {
            return;
        }
        //   java: CLIENT.player.containerMenu = defaultHandler;
        //   java: CLIENT.gui.setScreen(new ContainerScreen(defaultHandler, CLIENT.player.getInventory(), name));
        // rust: port gap - a mod cannot detach its `ModMenu` from the open menu (`containerMenu = defaultHandler`) nor
        // open the vanilla `ContainerScreen` over that menu in place of its own container screen (`container_screens`
        // offers the swap only from `fromPacket`) (GAPS_M #13). Old form: gui::set_screen(&mut ContainerScreen::new(compat::read(__this, |__t| __t.default_handler.clone()).unwrap(), mcrust::mcrust::modding::game::with_player(|player| player.base.base.get_inventory().clone()).unwrap(), __this.lock().borrow().name.as_ref().unwrap().get_string()));
    }

    /// java: private void home(Button button)
    pub fn home(__this: &compat::Shared<Self>, button: &AbstractWidget) {
        MessageScheduler::send_message_after_cooldown(message_scheduler::INSTANCE.get_mut(), "/storage", true);
        compat::set_state(&DISABLE_ON_NEXT_LOAD, true);
        compat::set_state(&SAVED_INDEX, -1);
    }

    /// java: private void toolkit(Button button)
    pub fn toolkit(__this: &compat::Shared<Self>, button: &AbstractWidget) {
        MessageScheduler::send_message_after_cooldown(message_scheduler::INSTANCE.get_mut(), "/farmingtoolkit", true);
    }

    /// java: private void huntingToolkit(Button button)
    pub fn hunting_toolkit(__this: &compat::Shared<Self>, button: &AbstractWidget) {
        MessageScheduler::send_message_after_cooldown(message_scheduler::INSTANCE.get_mut(), "/huntingtoolkit", true);
    }

    /// java: private int getMinLeftPos()
    pub fn get_min_left_pos(__this: &compat::Shared<Self>) -> i32 {
        return ((compat::read(__this, |__t| __t.screen.width)).wrapping_sub(StorageOverlayScreen::get_max_width(__this))).wrapping_div(2);
    }

    /// java: private int getLeftPos()
    pub fn get_left_pos(__this: &compat::Shared<Self>) -> i32 {
        return ((compat::read(__this, |__t| __t.screen.width)).wrapping_sub(StorageOverlayScreen::get_width(__this))).wrapping_div(2);
    }

    /// java: private int getMaxWidth()
    pub fn get_max_width(__this: &compat::Shared<Self>) -> i32 {
        return ((compat::read(__this, |__t| __t.screen.width)).wrapping_div(8)).wrapping_mul(7);
    }

    /// java: private int getWidth()
    pub fn get_width(__this: &compat::Shared<Self>) -> i32 {
        // (the grid is read only when it is there - Java's `grid != null ?`)
        return match compat::read(__this, |__t| __t.grid.clone()) {
            Some(__grid) => (compat::write(&__grid, |__t| __t.get_width())).wrapping_add(16),
            None => StorageOverlayScreen::get_max_width(__this),
        };
    }

    /// java: private int getHeight()
    pub fn get_height(__this: &compat::Shared<Self>) -> i32 {
        return ((compat::read(__this, |__t| __t.screen.height)).wrapping_sub((compat::read(__this, |__t| __t.screen.height)).wrapping_div(5))).wrapping_sub(87);
    }

    /// java: public Rect2i getMainExclusionZone()
    pub fn get_main_exclusion_zone(__this: &compat::Shared<Self>) -> Rect2i {
        return Rect2i::new(StorageOverlayScreen::get_left_pos(__this), 0, StorageOverlayScreen::get_width(__this), compat::read(__this, |__t| __t.screen.height));
    }

    /// java: public Rect2i getButtonsExclusionZone()
    pub fn get_buttons_exclusion_zone(__this: &compat::Shared<Self>) -> Rect2i {
        return Rect2i::new((compat::read(__this, |__t| __t.screen.width)).wrapping_sub(90), (compat::read(__this, |__t| __t.screen.height)).wrapping_sub(80), compat::read(__this, |__t| __t.screen.width), compat::read(__this, |__t| __t.screen.height));
    }

    /// java: @Override protected void repositionElements()
    pub fn reposition_elements(__this: &compat::Shared<Self>) {
        //   java: super.repositionElements(); - `Screen.repositionElements` is `rebuildWidgets()`: the widgets cleared, then
        //   the virtual `init()` (this class's, whose `super.init()` is `AbstractContainerScreen.init:80-81`'s centring)
        compat::write(__this, |__t| {
            __t.base.screen.reposition_elements(|_| {});
            __t.base.left_pos = mcrust::net::minecraft::client::gui::screens::inventory::abstract_container_screen::left_pos(__t.base.screen.width, __t.base.image_width);
            __t.base.top_pos = mcrust::net::minecraft::client::gui::screens::inventory::abstract_container_screen::top_pos(__t.base.screen.height, __t.base.image_height);
        });
        StorageOverlayScreen::init(__this);
        { let __v = (compat::read(__this, |__t| __t.screen.height)).wrapping_div(10); compat::write(__this, |__t| __t.top_pos = __v); }
        if compat::read(__this, |__t| __t.grid.clone()).is_some() {
            ({ let __r = &compat::read(__this, |__t| __t.grid.clone()).unwrap(); let __f = compat::dispatch(__r, |__t| __t.set_y()); __f(&compat::view(__r, |a| a as compat::Shared<dyn SearchableGridWidget>), (compat::read(__this, |__t| __t.top_pos)).wrapping_add(8)) });
        }
        StorageOverlayScreenHandler::update_backpack_slots(&compat::read(__this, |__t| __t.handler.clone()).unwrap(), compat::read(__this, |__t| __t.screen.height));
        let mut change: i32 = (StorageOverlayScreen::get_height(__this)).wrapping_sub(compat::read(__this, |__t| __t.old_height));
        { let __v = StorageOverlayScreen::get_height(__this); compat::write(__this, |__t| __t.old_height = __v); }
        StorageOverlayScreenHandler::shift_inventory(&compat::read(__this, |__t| __t.handler.clone()).unwrap(), change);
        { let __v = (compat::read(__this, |__t| __t.inventory_label_y)).wrapping_add(change); compat::write(__this, |__t| __t.inventory_label_y = __v); }
    }

    /// java: @Override protected boolean isHovering(Slot slot, double xm, double ym)
    pub fn is_hovering(__this: &compat::Shared<Self>, slot: &Slot, mut xm: f64, mut ym: f64) -> bool {
        // java: super.isHovering(slot, xm, ym) && (grid == null || slot.container instanceof Inventory || grid.getGridRectangle().containsPoint(..))
        return (__this.lock().borrow_mut().base.is_hovering_slot(slot, xm, ym) && (compat::read(__this, |__t| __t.grid.clone()).is_none() || slot.container == mcrust::net::minecraft::world::inventory::slot::SlotContainer::Inventory || (({ let __r = &compat::read(__this, |__t| __t.grid.clone()).unwrap(); let __f = compat::dispatch(__r, |__t| __t.get_grid_rectangle()); __f(&compat::view(__r, |a| a as compat::Shared<dyn SearchableGridWidget>)) })).contains_point(((xm) as i32), ((ym) as i32))));
    }

    /// java: @Override protected void init()
    pub fn init(__this: &compat::Shared<Self>) {
        ();
        let mut storages_per_row: i32 = SkyblockerConfigManager::get().ui_and_visuals.storage_overlay.storages_per_row;
        storages_per_row = (if storages_per_row > 0 { storages_per_row } else { i32::MAX });
        let mut internal_cols: i32 = SkyblockerConfigManager::get().ui_and_visuals.storage_overlay.backpack_width;
        { let __v = Some(BackpackGridWidget::new(std::sync::Arc::downgrade(__this), (StorageOverlayScreen::get_min_left_pos(__this)).wrapping_add(8), (compat::read(__this, |__t| __t.top_pos)).wrapping_add(8), (StorageOverlayScreen::get_max_width(__this)).wrapping_sub(16), (StorageOverlayScreen::get_height(__this)).wrapping_sub(16), storages_per_row, internal_cols, true)); compat::write(__this, |__t| __t.grid = __v); }
        ({ let __r = &compat::read(__this, |__t| __t.grid.clone()).unwrap(); let __f = compat::dispatch(__r, |__t| __t.set_search()); __f(&compat::view(__r, |a| a as compat::Shared<dyn SearchableGridWidget>), &compat::get_state(&SAVED_SEARCH)) });
        ({ let __r = &compat::read(__this, |__t| __t.grid.clone()).unwrap(); let __f = compat::dispatch(__r, |__t| __t.set_scroll_amount()); __f(&compat::view(__r, |a| a as compat::Shared<dyn SearchableGridWidget>), compat::get_state(&SAVED_SCROLL)) });
        let __r0 = compat::read(__this, |__t| __t.grid.clone());
        { let (__q0,) = (ContainerWidgetAdapter::widget(__r0.unwrap()),); __this.lock().borrow_mut().screen.add_renderable_widget(__q0) };
        //extra control buttons out the way
        //   java: LinearLayout extraButtons = new LinearLayout(width - 90, height - 84, LinearLayout.Orientation.VERTICAL);
        let (__width, __height) = compat::read(__this, |__t| (__t.screen.width, __t.screen.height));
        let mut extra_buttons: LinearLayout = LinearLayout::new(__width.wrapping_sub(90), __height.wrapping_sub(84), mcrust::net::minecraft::client::gui::layouts::linear_layout::Orientation::Vertical);
        extra_buttons = extra_buttons.spacing(5);
        // The port's layouts hold a widget by its index in the screen's widget list, so each button is added to the
        // screen as it is built (Java's closing `extraButtons.visitWidgets(this::addRenderableWidget)`) and laid out by
        // that index; the arranged positions are written back to the widgets after `arrangeElements()`.
        let __buttons: [(&str, fn(&compat::Shared<StorageOverlayScreen>, &AbstractWidget)); 4] = [
            //add toolkit button
            ("skyblocker.config.uiAndVisuals.storageOverlay.farmingToolkitButton", StorageOverlayScreen::toolkit),
            //add hunting toolkit button
            ("skyblocker.config.uiAndVisuals.storageOverlay.huntingToolkitButton", StorageOverlayScreen::hunting_toolkit),
            //add button to go home
            ("skyblocker.config.uiAndVisuals.storageOverlay.homeButton", StorageOverlayScreen::home),
            //add button to temperately disable menu
            ("skyblocker.config.uiAndVisuals.storageOverlay.hideButton", StorageOverlayScreen::hide),
        ];
        for (__key, __action) in __buttons {
            let __this_w = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
            let __button: AbstractWidget = button::builder(component::translatable(__key).get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { let Some(__this_1) = __this_w.get().upgrade() else { return; }; __action(&__this_1, __screen.widget_mut(__index)); }))).size(80, 16).build();
            let __index: usize = compat::write(__this, |__t| __t.screen.add_renderable_widget(__button));
            extra_buttons.add_child(LayoutElement::widget(__index, 80, 16));
        }
        extra_buttons.arrange_elements();
        let mut __vw: Vec<(usize, i32, i32)> = Vec::new();
        LayoutElement::Grid(extra_buttons.wrapped).visit_widgets(&mut __vw);
        for (__i, __x, __y) in __vw {
            compat::write(__this, |__t| __t.screen.widget_mut(__i).set_position(__x, __y));
        }
    }

    /// java: @Override public void extractRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_render_state(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        ();
        //   java: this.extractTooltip(graphics, mouseX, mouseY);
        // `super.extractRenderState` (`AbstractContainerScreen.extractRenderState:94`) already ran `extractTooltip` for this
        // frame - the base pass runs it before this override - and a second call sets the same next-frame tooltip again.
        let _ = (graphics, mouse_x, mouse_y, a);
    }

    /// java: @Override public void onClose()
    pub fn on_close(__this: &compat::Shared<Self>) {
        compat::set_state(&SAVED_SCROLL, ((0) as f64) as f64);
        if SkyblockerConfigManager::get().ui_and_visuals.storage_overlay.remember_search && compat::read(__this, |__t| __t.grid.clone()).is_some() {
            compat::set_state(&SAVED_SCROLL, ({ let __r = &compat::read(__this, |__t| __t.grid.clone()).unwrap(); let __f = compat::dispatch(__r, |__t| __t.get_scroll_amount()); __f(&compat::view(__r, |a| a as compat::Shared<dyn SearchableGridWidget>)) }));
        } else {
            compat::set_state(&SAVED_SEARCH, "".to_string());
        }
        if SkyblockerConfigManager::get().ui_and_visuals.storage_overlay.remember_opened && compat::read(__this, |__t| __t.grid.clone()).is_some() && compat::read(&compat::read(__this, |__t| __t.grid.clone()).unwrap(), |__t| __t.open_backpack.clone()).is_some() {
            compat::set_state(&SAVED_INDEX, compat::read(&compat::read(&compat::read(__this, |__t| __t.grid.clone()).unwrap(), |__t| __t.open_backpack.clone()).unwrap(), |__t| __t.index));
            compat::set_state(&SAVED_SCROLL, ({ let __r = &compat::read(__this, |__t| __t.grid.clone()).unwrap(); let __f = compat::dispatch(__r, |__t| __t.get_scroll_amount()); __f(&compat::view(__r, |a| a as compat::Shared<dyn SearchableGridWidget>)) }));
        } else {
            compat::set_state(&SAVED_INDEX, -1);
        }
        //   java: super.onClose();
        // rust: port gap - `AbstractContainerScreen.onClose` (`:582-586`, the bundle-hover reset then the container close) is
        // the game's own close of the screen, which a `ModContainerScreen` has no override of (`on_close` is not one of
        // its methods): the game runs it when it closes the screen, and this override is not reached (GAPS_M #14).
    }

    /// java: @Override protected void extractSlot(GuiGraphicsExtractor graphics, Slot slot, int mouseX, int mouseY)
    pub fn extract_slot(__this: &compat::Shared<Self>, graphics: &mut GuiGraphicsExtractor, slot: &Slot, mut mouse_x: i32, mut mouse_y: i32) {
        if slot.container == mcrust::net::minecraft::world::inventory::slot::SlotContainer::Inventory || compat::read(__this, |__t| __t.grid.clone()).is_none() {
            compat::call_super();
        } else {
            let mut rectangle: ScreenRectangle = ({ let __r = &compat::read(__this, |__t| __t.grid.clone()).unwrap(); let __f = compat::dispatch(__r, |__t| __t.get_grid_rectangle()); __f(&compat::view(__r, |a| a as compat::Shared<dyn SearchableGridWidget>)) });
            rectangle = ScreenRectangle::new_at((rectangle.position().x()).wrapping_sub(compat::read(__this, |__t| __t.left_pos)), (rectangle.position().y()).wrapping_sub(compat::read(__this, |__t| __t.top_pos)), rectangle.width(), rectangle.height());
            graphics.enable_scissor(rectangle.left(), rectangle.top(), rectangle.right(), rectangle.bottom());
            compat::call_super();
            graphics.disable_scissor();
            if compat::read(&compat::read(__this, |__t| __t.grid.clone()).unwrap(), |__t| __t.open_backpack.clone()).is_some() && !({ let (__q0,) = ((slot.get_container_slot()) as usize,); compat::read(&compat::read(__this, |__t| __t.grid.clone()).unwrap(), |__t| __t.open_backpack.clone()).unwrap().lock().borrow().matched_slots.get(__q0) }) {
                graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), slot.x, slot.y, slot.x.wrapping_add(16), slot.y.wrapping_add(16), *NOT_MATCHED_COLOR);
            }
        }
    }

    /// java: @Override protected boolean hasClickedOutside(double mouseX, double mouseY, int xo, int yo)
    pub fn has_clicked_outside(__this: &compat::Shared<Self>, mut mouse_x: f64, mut mouse_y: f64, mut xo: i32, mut yo: i32) -> bool {
        //   java: return super.hasClickedOutside(mouseX, mouseY, xo, yo) && (mouseX < getLeftPos() || mouseX > getLeftPos() + width || mouseY < this.topPos || mouseY > this.topPos + getHeight());
        let (__image_width, __image_height, __width, __top_pos) = compat::read(__this, |__t| (__t.image_width, __t.image_height, __t.screen.width, __t.top_pos));
        let __left_pos: i32 = StorageOverlayScreen::get_left_pos(__this);
        return mcrust::net::minecraft::client::gui::screens::inventory::abstract_container_screen::has_clicked_outside(mouse_x, mouse_y, xo, yo, __image_width, __image_height) && (mouse_x < (__left_pos as f64) || mouse_x > (__left_pos.wrapping_add(__width) as f64) || mouse_y < (__top_pos as f64) || mouse_y > (__top_pos.wrapping_add(StorageOverlayScreen::get_height(__this)) as f64));
    }

    /// java: @Override public boolean mouseScrolled(double x, double y, double scrollX, double scrollY)
    pub fn mouse_scrolled(__this: &compat::Shared<Self>, mut x: f64, mut y: f64, mut scroll_x: f64, mut scroll_y: f64) -> bool {
        // java: if (this.hoveredSlot != null && this.hoveredSlot.hasItem())
        if compat::read(__this, |__t| __t.hovered_slot.clone()).is_some() && compat::with_open_menu(|__menu, __containers| __menu.slots.get(compat::read(__this, |__t| __t.hovered_slot).unwrap() as usize).is_some_and(|__slot| __slot.has_item(__containers))).unwrap_or(false) {
            return false;
        }
        //   java: return this.getChildAt(x, y).filter(child -> child.mouseScrolled(x, y, scrollX, scrollY)).isPresent();
        // (`Screen::mouse_scrolled_mod_widget` is that statement for the screen's mod widgets - the grid; its buttons do
        // not scroll)
        return compat::write(__this, |__t| __t.base.screen.mouse_scrolled_mod_widget(x, y, scroll_x, scroll_y));
    }

    /// java: @Override public boolean keyPressed(KeyEvent event)
    pub fn key_pressed(__this: &compat::Shared<Self>, event: &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        // java: ComponentPath focusPath = this.getCurrentFocusPath();
        //   if (this.minecraft.options.keyInventory.matches(event) && focusPath != null && focusPath.leafComponent() instanceof EditBox)
        // (the focus path's leaf among the screen's own widgets: the focused child, when it is an edit box; an edit box
        // inside the backpack grid is in the grid object's own widgets, which the screen's focus does not reach)
        let __focused_edit_box: bool = compat::read(__this, |__t| __t.base.screen.get_focused().is_some_and(|__i| matches!(__t.base.screen.widget(__i).kind, mcrust::net::minecraft::client::gui::components::abstract_widget::WidgetKind::EditBox(_))));
        if key_mapping::matches("key.inventory", event) && __focused_edit_box {
            return true;
        }
        return false;
    }

    /// java: @Override public void removed()
    pub fn removed(__this: &compat::Shared<Self>) {
        ();
        if compat::read(__this, |__t| __t.save_mouse_position) {
            //   java: previousMousePosition = new Vector2d(minecraft.mouseHandler.xpos(), minecraft.mouseHandler.ypos());
            let __mouse_handler = &compat::minecraft().mouse_handler;
            *PREVIOUS_MOUSE_POSITION.get_mut() = Some(crate::unresolved::Vector2d::new(__mouse_handler.xpos(), __mouse_handler.ypos()));
        }
    }

    /// java: @Override public void extractBackground(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_background(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        compat::call_super();
        //render background
        //   java: graphics.blitSprite(RenderPipelines.GUI_TEXTURED, BACKGROUND.get(), getLeftPos(), this.topPos, getWidth(), getHeight());
        let __texture: Identifier = compat::read(BACKGROUND.get(), |__t| __t.get());
        gui_graphics_extractor::blit_sprite_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), &__texture, StorageOverlayScreen::get_left_pos(__this), compat::read(__this, |__t| __t.top_pos), StorageOverlayScreen::get_width(__this), StorageOverlayScreen::get_height(__this));
        //render inventory
        gui_graphics_extractor::blit_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), TEXTURE.get_mut(), compat::read(__this, |__t| __t.left_pos), ((compat::read(__this, |__t| __t.top_pos)).wrapping_add(StorageOverlayScreen::get_height(__this))).wrapping_sub(3), ((0) as f32) as f32, ((132) as f32) as f32, 175, 90, 256, 256);
    }

}

/// StorageOverlayScreen's screen half (`super`'s fields), lent by the game for each call
impl compat::ContainerHalf for StorageOverlayScreen {
    fn container_half(&mut self) -> &mut AbstractContainerScreen {
        &mut self.base
    }
}

/// StorageOverlayScreen as the game's container screen (`container_screens::register`'s answer)
impl ModContainerScreen for compat::ContainerScreenAdapter<StorageOverlayScreen> {
    /// java: @Override protected void init()
    fn init(&mut self, screen: &mut AbstractContainerScreen) {
        self.lend(screen, |__h| StorageOverlayScreen::init(__h))
    }

    /// java: @Override public void extractBackground(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_background(&mut self, screen: &AbstractContainerScreen, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32, super_extract_background: &mut dyn FnMut()) {
        compat::with_super(super_extract_background, || self.lend_ref(screen, |__h| StorageOverlayScreen::extract_background(__h, &context.graphics, mouse_x, mouse_y, a)))
    }

    /// java: @Override public void extractRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_render_state(&mut self, screen: &AbstractContainerScreen, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        self.lend_ref(screen, |__h| StorageOverlayScreen::extract_render_state(__h, &context.graphics, mouse_x, mouse_y, a))
    }

    /// java: @Override public boolean mouseScrolled(double x, double y, double scrollX, double scrollY)
    fn mouse_scrolled(&mut self, screen: &mut AbstractContainerScreen, x: f64, y: f64, scroll_x: f64, scroll_y: f64) -> bool {
        self.lend(screen, |__h| StorageOverlayScreen::mouse_scrolled(__h, x, y, scroll_x, scroll_y))
    }

    /// java: @Override public boolean keyPressed(KeyEvent event)
    fn key_pressed(&mut self, screen: &mut AbstractContainerScreen, event: &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        self.lend(screen, |__h| StorageOverlayScreen::key_pressed(__h, event))
    }

    /// java: @Override public void removed()
    fn removed(&mut self, screen: &mut AbstractContainerScreen) {
        self.lend(screen, |__h| StorageOverlayScreen::removed(__h))
    }

    /// java: @Override protected void extractSlot(GuiGraphicsExtractor graphics, Slot slot, int mouseX, int mouseY)
    fn extract_slot(&mut self, screen: &mut AbstractContainerScreen, context: &mut WidgetRenderContext<'_>, slot: &Slot, mouse_x: i32, mouse_y: i32, super_extract_slot: &mut dyn FnMut(&mut AbstractContainerScreen)) {
        let __object = self.object().clone();
        let mut __super = || compat::write(&__object, |__t| super_extract_slot(compat::ContainerHalf::container_half(__t)));
        compat::with_super(&mut __super, || self.lend(screen, |__h| StorageOverlayScreen::extract_slot(__h, &mut context.graphics, slot, mouse_x, mouse_y)))
    }

}

// NOTE(fabric2seer): extractBackground, extractRenderState, extractSlot, hasClickedOutside, init, isHovering, keyPressed, mouseScrolled, onClose, removed, repositionElements override(s) of AbstractContainerScreen - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them

/// java: private class BackpackGridWidget extends SearchableGridWidget
pub struct BackpackGridWidget {
    /// java: extends SearchableGridWidget
    pub base: SearchableGridWidgetBase,
    /// java: private final List<BackpackWidget> backpackWidgets = new ArrayList<>();
    pub backpack_widgets: Vec<compat::Shared<BackpackWidget>>,
    /// java: private StorageOverlayScreen.@Nullable BackpackWidget openBackpack = null;
    pub open_backpack: Option<compat::Shared<BackpackWidget>>,
    /// java: @Nullable private final Button reloadButton;
    pub reload_button: Option<AbstractWidget>,
    /// `StorageOverlayScreen.this` - the enclosing instance of this inner class
    pub outer: compat::WeakShared<StorageOverlayScreen>,
}

impl std::ops::Deref for BackpackGridWidget {
    type Target = SearchableGridWidgetBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for BackpackGridWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl BackpackGridWidget {
    /// java: private BackpackGridWidget(int x, int y, int width, int height, int maxStoragesPerRow, int internalCols, boolean packed)
    /// (`outer` is `StorageOverlayScreen.this`, the enclosing instance)
    pub fn new(outer: compat::WeakShared<StorageOverlayScreen>, mut x: i32, mut y: i32, mut width: i32, mut height: i32, mut max_storages_per_row: i32, mut internal_cols: i32, mut packed: bool) -> compat::Shared<Self> {
        // cut down number of columns if it will not fit on to the current gui size
        let mut expected_width: i32 = (internal_cols.wrapping_mul(SLOT_SIZE)).wrapping_add(EDGE_PADDING.wrapping_mul(2));
        while expected_width > width.wrapping_sub(mcrust::net::minecraft::client::gui::components::abstract_scroll_area::SCROLLBAR_WIDTH) {
            expected_width = (({ internal_cols -= 1; internal_cols }).wrapping_mul(SLOT_SIZE)).wrapping_add(EDGE_PADDING.wrapping_mul(2));
        }
        //   java: super(x, y, width, height, Component.literal("BackPack grid"), expectedWidth, maxStoragesPerRow, packed, true);
        let __outer = outer.clone();
        let (_, __this) = SearchableGridWidgetBase::new_in(x, y, width, height, component::literal("BackPack grid"), expected_width, max_storages_per_row, packed, true, move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                backpack_widgets: Vec::new(),
                open_backpack: None,
                reload_button: None,
                outer: __outer,
            });
            ((__h.clone() as compat::Shared<dyn SearchableGridWidget>), __h)
        });
        let mut storage_loaded: bool = false;
        // java: BackpackPreview.@Nullable Storage[] storages = BackpackPreview.getStorages(); - the ELEMENTS are nullable
        let mut storages: Vec<Option<compat::Shared<Storage>>> = BackpackPreview::get_storages();
        for i in 0..(storages.len() as i32) {
            let mut storage: Option<compat::Shared<Storage>> = storages[(i) as usize].clone();
            let mut open: bool = (compat::get_state(&OPEN_STORAGE) == i);
            if let Some(storage) = &storage {
                storage_loaded = true;
                let mut widget: compat::Shared<BackpackWidget> = BackpackWidget::new(outer.clone(), internal_cols, i, storage.clone(), open);
                if open {
                    { let __v = Some(widget.clone()); compat::write(&__this, |__t| __t.open_backpack = __v); }
                }
                let __h0 = widget.clone();
                __this.lock().borrow_mut().backpack_widgets.push(__h0);
            }
        }
        if !storage_loaded {
            let __this_w1 = compat::RenderThread::new(std::sync::Arc::downgrade(&__this));
            { let __v = Some((button::builder(component::translatable("skyblocker.config.uiAndVisuals.storageOverlay.reloadButton").get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { let Some(__this_1) = __this_w1.get().upgrade() else { return Default::default(); }; { BackpackGridWidget::reload(&__this_1, (__screen.widget_mut(__index))); } })))).size(300, 30).build()); compat::write(&__this, |__t| __t.reload_button = __v); }
        } else {
            { let __v = None; compat::write(&__this, |__t| __t.reload_button = __v); }
        }
        __this
    }

    /// java: private void reload(Button button)
    pub fn reload(__this: &compat::Shared<Self>, button: &AbstractWidget) {
        MessageScheduler::send_message_after_cooldown(message_scheduler::INSTANCE.get_mut(), "/storage", true);
    }

    /// java: @Override public boolean isMouseOver(double mouseX, double mouseY)
    pub fn is_mouse_over(__this: &compat::Shared<Self>, mut mouse_x: f64, mut mouse_y: f64) -> bool {
        //let the mouse go though gui if the user is interacting with open backpack
        //   java: hoveredSlot != null && getGridRectangle().containsPoint((int) mouseX, (int) mouseY)
        let __hovered_slot: bool = compat::read(__this, |__t| __t.outer.upgrade()).is_some_and(|__outer| compat::read(&__outer, |__o| __o.hovered_slot.is_some()));
        if __hovered_slot && ({ let __v = compat::view(__this, |a| a as compat::Shared<dyn SearchableGridWidget>); let __f = compat::dispatch(&__v, |__t| __t.get_grid_rectangle()); __f(&__v) }).contains_point(mouse_x as i32, mouse_y as i32) {
            return false;
        }
        return __this.lock().borrow_mut().base.is_mouse_over(mouse_x, mouse_y);
    }

    /// java: @Override protected double scrollRate()
    pub fn scroll_rate(__this: &compat::Shared<Self>) -> f64 {
        return ((15) as f64) as f64;
    }

}

impl SearchableGridWidget for BackpackGridWidget {
    fn searchable_grid_widget_base(&self) -> &SearchableGridWidgetBase {
        &self.base
    }
    fn searchable_grid_widget_base_mut(&mut self) -> &mut SearchableGridWidgetBase {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override protected Collection<? extends AbstractWidget> filterWidgets(String input) (dispatch: the body for this class)
    fn filter_widgets(&self) -> fn(&compat::Shared<dyn SearchableGridWidget>, String) -> Vec<AbstractWidget> {
        BackpackGridWidget::filter_widgets_dyn
    }

    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor context, int mouseX, int mouseY, float deltaTicks) (dispatch: the body for this class)
    fn extract_widget_render_state(&self) -> fn(&compat::Shared<dyn SearchableGridWidget>, &GuiGraphicsExtractor, i32, i32, f32) {
        BackpackGridWidget::extract_widget_render_state_dyn
    }

}

impl BackpackGridWidget {
    /// java: @Override protected Collection<? extends AbstractWidget> filterWidgets(String input)
    pub fn filter_widgets_impl(__this: &compat::Shared<Self>, mut input: String) -> Vec<AbstractWidget> {
        compat::set_state(&SAVED_SEARCH, input.clone());
        if compat::read(__this, |__t| __t.reload_button.clone()).is_some() {
            return vec![compat::read(__this, |__t| __t.reload_button.clone()).unwrap()];
        }
        // java: return backpackWidgets.stream().filter(backpack -> backpack.matches(input)).toList(); - the widgets
        // themselves, each the game's widget over the same object
        return compat::read(__this, |__t| __t.backpack_widgets.clone()).into_iter().filter(|backpack| BackpackWidget::matches(backpack, &input)).map(compat::WidgetAdapter::widget).collect::<Vec<_>>();
    }

    /// BackpackGridWidget.filterWidgets as the dispatch of SearchableGridWidget.filterWidgets
    pub fn filter_widgets_dyn(__this: &compat::Shared<dyn SearchableGridWidget>, __p0: String) -> Vec<AbstractWidget> {
        Self::filter_widgets_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor context, int mouseX, int mouseY, float deltaTicks)
    pub fn extract_widget_render_state_impl(__this: &compat::Shared<Self>, context: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut delta_ticks: f32) {
        <dyn SearchableGridWidget>::extract_widget_render_state_body(&compat::view(__this, |a| a as compat::Shared<dyn SearchableGridWidget>), context, mouse_x, mouse_y, delta_ticks);
        // java: if (openBackpack != null && (openBackpack.getBottom() < getY() + HEADER_H || openBackpack.getY() >= getBottom()))
        let __open = compat::read(__this, |__t| __t.open_backpack.clone());
        if let Some(__o) = &__open && (compat::write(__o, |__t| __t.get_bottom()) < (__this.lock().borrow().get_y()).wrapping_add(HEADER_H) || compat::write(__o, |__t| __t.get_y()) >= __this.lock().borrow().get_bottom()) {
            //   java: handler.moveBackpackSlots(0, -1000, 9);
            if let Some(__handler) = outer_handler(&compat::read(__this, |__t| __t.outer.clone())) {
                StorageOverlayScreenHandler::move_backpack_slots(&__handler, 0, -1000, 9);
            }
        }
    }

    /// BackpackGridWidget.extractWidgetRenderState as the dispatch of SearchableGridWidget.extractWidgetRenderState
    pub fn extract_widget_render_state_dyn(__this: &compat::Shared<dyn SearchableGridWidget>, __p0: &GuiGraphicsExtractor, __p1: i32, __p2: i32, __p3: f32) {
        Self::extract_widget_render_state_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2, __p3)
    }

}

/// java: private class BackpackWidget extends AbstractWidget
#[derive(Clone)]
pub struct BackpackWidget {
    /// java: extends net.minecraft.client.gui.components.AbstractWidget
    pub base: AbstractWidget,
    /// java: private final int rows;
    pub rows: i32,
    /// java: private final int columns;
    pub columns: i32,
    /// java: private final String label;
    pub label: String,
    /// java: private final int index;
    pub index: i32,
    /// java: private final BackpackPreview.Storage storage;
    pub storage: Option<compat::Shared<Storage>>,
    /// java: private final boolean open;
    pub open: bool,
    /// java: private final BitSet matchedSlots = new BitSet();
    pub matched_slots: BitSet,
    /// `StorageOverlayScreen.this` - the enclosing instance of this inner class
    pub outer: compat::WeakShared<StorageOverlayScreen>,
}

impl std::ops::Deref for BackpackWidget {
    type Target = AbstractWidget;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for BackpackWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl BackpackWidget {
    /// java: private BackpackWidget(int columns, int index, BackpackPreview.Storage storage, Boolean open)
    /// (`outer` is `StorageOverlayScreen.this`, the enclosing instance)
    pub fn new(outer: compat::WeakShared<StorageOverlayScreen>, mut columns: i32, mut index: i32, mut storage: compat::Shared<Storage>, mut open: bool) -> compat::Shared<Self> {
        //   java: int rows = Math.ceilDiv(storage.size() - 9, columns);
        let mut rows: i32 = ceil_div(Storage::size(&storage).wrapping_sub(9), columns);
        // if the storage is open use the handler to work out size
        if open {
            //   java: rows = Math.ceilDiv(handler.getContainer().getContainerSize() - 9, columns);
            if let Some(__size) = outer_container_size(&outer) {
                rows = ceil_div(__size.wrapping_sub(9), columns);
            }
        }
        //   java: super(0, 0, columns * SLOT_SIZE + EDGE_PADDING * 2, rows * SLOT_SIZE + HEADER_H + EDGE_PADDING, Component.literal("Backpack preview"));
        let __this = compat::shared(Self {
            base: AbstractWidget::new(0, 0, (columns.wrapping_mul(SLOT_SIZE)).wrapping_add(EDGE_PADDING.wrapping_mul(2)), ((rows.wrapping_mul(SLOT_SIZE)).wrapping_add(HEADER_H)).wrapping_add(EDGE_PADDING), component::literal("Backpack preview").get_string(), mcrust::net::minecraft::client::gui::components::abstract_widget::WidgetKind::PlainText, OnPress::Inert),
            rows,
            columns,
            label: StorageOverlayScreen::get_storage_name(index),
            index,
            storage: Some(storage.clone()),
            open,
            matched_slots: BitSet::new(),
            outer,
        });
        __this
    }

    /// java: private int size()
    pub fn size(__this: &compat::Shared<Self>) -> i32 {
        if compat::read(__this, |__t| __t.open) {
            //   java: return handler.getContainer().getContainerSize();
            // (the open storage's menu, read while it is still the open one; `0` - no slots - once it is gone)
            return outer_container_size(&compat::read(__this, |__t| __t.outer.clone())).unwrap_or(0);
        } else {
            return Storage::size(&compat::read(__this, |__t| __t.storage.clone()).unwrap());
        }
    }

    /// java: public boolean matches(String filter)
    pub fn matches(__this: &compat::Shared<Self>, filter: &str) -> bool {
        __this.lock().borrow_mut().matched_slots.clear();
        let mut filter_lower_case: String = filter.to_lowercase();
        let __open: bool = compat::read(__this, |__t| __t.open);
        let __outer = compat::read(__this, |__t| __t.outer.clone());
        let __storage = compat::read(__this, |__t| __t.storage.clone()).unwrap();
        //matches if any item in backpack contains the filter word
        for i in 9..BackpackWidget::size(__this) {
            //   java: ItemStack currentStack = open ? handler.getContainer().getItem(i) : storage.getStack(i);
            let current_stack: mcrust::net::minecraft::world::item::item_stack::ItemStack = if __open { outer_container_item(&__outer, i) } else { Storage::get_stack(&__storage, i) };
            let filter_lower_case = filter_lower_case.clone();
            if compat::component_of(current_stack.get_display_name()).get_string().to_lowercase().contains(&filter_lower_case) || ItemUtils::get_lore_line_if(&current_stack, Arc::new(move |s| s.to_lowercase().contains(&filter_lower_case))).is_some() {
                { let (__q0,) = ((i) as usize,); __this.lock().borrow_mut().matched_slots.set(__q0) };
            }
        }
        return (__this.lock().borrow().matched_slots.cardinality() != 0 || compat::read(__this, |__t| __t.open));
    }

    /// java: private void customInventorySize(GuiGraphicsExtractor graphics, int x, int y, int rows, int columns)
    pub fn custom_inventory_size(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor, mut x: i32, mut y: i32, mut rows: i32, mut columns: i32) {
        BackpackWidget::extract_section(__this, graphics, x, y, columns, HEADER_H, 0);
        let mut rows_remaining: i32 = rows;
        while rows_remaining > 0 {
            let mut row_batch: i32 = rows_remaining.min(MAX_ROW_BATCH);
            let mut batch_h: i32 = row_batch.wrapping_mul(SLOT_SIZE);
            let mut row_y: i32 = (y.wrapping_add(HEADER_H)).wrapping_add((rows.wrapping_sub(rows_remaining)).wrapping_mul(SLOT_SIZE));
            BackpackWidget::extract_section(__this, graphics, x, row_y, columns, batch_h, HEADER_H);
            rows_remaining = rows_remaining.wrapping_sub(row_batch);
        }
        let mut bottom_y: i32 = (y.wrapping_add(HEADER_H)).wrapping_add(rows.wrapping_mul(SLOT_SIZE));
        BackpackWidget::extract_section(__this, graphics, x, bottom_y, columns, EDGE_PADDING, BOTTOM_V);
    }

    /// java: private void extractSection(GuiGraphicsExtractor graphics, int x, int y, int columns, int height, int textureYOffset)
    pub fn extract_section(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor, mut x: i32, mut y: i32, mut columns: i32, mut height: i32, mut texture_y_offset: i32) {
        let mut columns_remaining: i32 = columns;
        while columns_remaining > 0 {
            let mut batch: i32 = columns_remaining.min(MAX_COL_BATCH);
            let mut batch_x: i32 = (((columns.wrapping_sub(columns_remaining)).wrapping_mul(SLOT_SIZE)).wrapping_add(EDGE_PADDING)).wrapping_add(x);
            gui_graphics_extractor::blit_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), TEXTURE.get_mut(), batch_x, y, ((EDGE_PADDING) as f32) as f32, (texture_y_offset) as f32, batch.wrapping_mul(SLOT_SIZE), height, 256, 256);
            columns_remaining = columns_remaining.wrapping_sub(batch);
        }
        gui_graphics_extractor::blit_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), TEXTURE.get_mut(), x, y, ((0) as f32) as f32, (texture_y_offset) as f32, EDGE_PADDING, height, 256, 256);
        gui_graphics_extractor::blit_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), TEXTURE.get_mut(), ((columns.wrapping_mul(SLOT_SIZE)).wrapping_add(EDGE_PADDING)).wrapping_add(x), y, ((169) as f32) as f32, (texture_y_offset) as f32, EDGE_PADDING, height, 256, 256);
    }

    /// java: private void renderCustomBackpack(GuiGraphicsExtractor graphics, int mouseX, int mouseY, int x, int y, int rows, int columns, String label, BackpackPreview.Storage storage)
    pub fn render_custom_backpack(__this: &compat::Shared<Self>, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut x: i32, mut y: i32, mut rows: i32, mut columns: i32, label: &str, storage: &compat::Shared<Storage>) {
        BackpackWidget::custom_inventory_size(__this, graphics, x, y, rows, columns);
        let mut text_renderer: Font = compat::font();
        gui_graphics_extractor::text_string_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), Some(label), x.wrapping_add(8), y.wrapping_add(6), 0xFF404040u32 as i32, false);
        for i in (BackpackWidget::size(__this)).wrapping_sub(9)..rows.wrapping_mul(columns) {
            let mut item_x: i32 = (x.wrapping_add((i.wrapping_rem(columns)).wrapping_mul(SLOT_SIZE))).wrapping_add(8);
            let mut item_y: i32 = (y.wrapping_add((i.wrapping_div(columns)).wrapping_mul(SLOT_SIZE))).wrapping_add(SLOT_SIZE);
            graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), item_x, item_y, item_x.wrapping_add(16), item_y.wrapping_add(16), 0xB0000000u32 as i32);
        }
        if !(compat::read(__this, |__t| __t.open)) {
            for i in 9..BackpackWidget::size(__this) {
                let mut current_stack: mcrust::net::minecraft::world::item::item_stack::ItemStack = Storage::get_stack(storage, i);
                let mut item_x: i32 = (x.wrapping_add(((i.wrapping_sub(9)).wrapping_rem(columns)).wrapping_mul(SLOT_SIZE))).wrapping_add(8);
                let mut item_y: i32 = (y.wrapping_add(((i.wrapping_sub(9)).wrapping_div(columns)).wrapping_mul(SLOT_SIZE))).wrapping_add(SLOT_SIZE);
                if !current_stack.is_empty() {
                    ItemBackgroundManager::draw_backgrounds(&current_stack, graphics, item_x, item_y);
                    if ItemProtection::is_item_protected(Some(current_stack.clone())) {
                        ItemProtection::draw_slot_icon(graphics, item_x, item_y);
                    }
                    graphics.item(&current_stack, item_x, item_y);
                    graphics.item_decorations(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &current_stack, item_x, item_y);
                    SlotTextManager::extract_slot_text_7(graphics, &text_renderer, None, &current_stack, i, item_x, item_y);
                    //draw tooltip if hovered
                    //   java: graphics.containsPointInScissor(mouseX, mouseY) && mouseX > itemX && mouseX <= itemX + SLOT_SIZE && mouseY > itemY && mouseY <= itemY + SLOT_SIZE && mouseY > topPos && mouseY < topPos + StorageOverlayScreen.this.getHeight()
                    let __outer = compat::read(__this, |__t| __t.outer.upgrade());
                    let (__top_pos, __outer_height) = match &__outer {
                        Some(__o) => (compat::read(__o, |__t| __t.top_pos), StorageOverlayScreen::get_height(__o)),
                        None => (0, 0),
                    };
                    if __outer.is_some() && gui_graphics_extractor::contains_point_in_scissor(mouse_x, mouse_y) && mouse_x > item_x && mouse_x <= item_x.wrapping_add(SLOT_SIZE) && mouse_y > item_y && mouse_y <= item_y.wrapping_add(SLOT_SIZE) && mouse_y > __top_pos && mouse_y < __top_pos.wrapping_add(__outer_height) {
                        //   java: graphics.setTooltipForNextFrame(CLIENT.font, currentStack, mouseX, mouseY);
                        // rust: port gap - the frame's deferred-tooltip slot (`setTooltipForNextFrame(Font, ItemStack, x, y)`) is
                        // the game's (`Option<DeferredTooltip>` in the screen pass); a mod's draw has no way to set it (GAPS_M #15).
                    }
                }
                if !({ let (__q0,) = ((i) as usize,); __this.lock().borrow().matched_slots.get(__q0) }) {
                    graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), item_x, item_y, item_x.wrapping_add(16), item_y.wrapping_add(16), *NOT_MATCHED_COLOR);
                }
            }
        } else {
            //move slots to fit this open backpack
            //   java: handler.moveBackpackSlots(getX() - leftPos + 8, getY() - topPos + SLOT_SIZE, columns);
            if let Some(__outer) = compat::read(__this, |__t| __t.outer.upgrade()) {
                let (__left_pos, __top_pos, __handler) = compat::read(&__outer, |__t| (__t.left_pos, __t.top_pos, __t.handler.clone()));
                let (__x, __y) = compat::read(__this, |__t| (__t.get_x(), __t.get_y()));
                if let Some(__handler) = __handler {
                    StorageOverlayScreenHandler::move_backpack_slots(&__handler, (__x.wrapping_sub(__left_pos)).wrapping_add(8), (__y.wrapping_sub(__top_pos)).wrapping_add(SLOT_SIZE), columns);
                }
            }
        }
    }

    /// java: @Override public void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_widget_render_state(__this: &compat::Shared<Self>, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        BackpackWidget::render_custom_backpack(__this, graphics, mouse_x, mouse_y, __this.lock().borrow().get_x(), __this.lock().borrow().get_y(), compat::read(__this, |__t| __t.rows), compat::read(__this, |__t| __t.columns), &compat::read(__this, |__t| __t.label.clone()), &compat::read(__this, |__t| __t.storage.clone()).unwrap());
        if compat::read(__this, |__t| __t.open) {
            //outline open
            //   java: graphics.outline(getX(), getY(), getWidth(), getHeight(), Color.yellow.getRGB());
            // (`getWidth()` / `getHeight()` here are this widget's own - `AbstractWidget`'s, which the inner class inherits and
            // which shadow the enclosing screen's private ones; `Color.yellow` is 0xFFFFFF00)
            let (__x, __y, __w, __h) = compat::read(__this, |__t| (__t.get_x(), __t.get_y(), __t.get_width(), __t.get_height()));
            gui_graphics_extractor::outline(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), __x, __y, __w, __h, 0xFFFFFF00u32 as i32);
        }
        if __this.lock().borrow().is_hovered() && !(compat::read(__this, |__t| __t.open)) {
            graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), __this.lock().borrow().get_x(), __this.lock().borrow().get_y(), __this.lock().borrow().get_right(), __this.lock().borrow().get_bottom(), argb::white_f32(0.1f32));
        }
    }

    /// java: @Override public void onClick(MouseButtonEvent event, boolean doubleClick)
    pub fn on_click(__this: &compat::Shared<Self>, event: &MouseButtonEvent, mut double_click: bool) {
        //   java: super.onClick(event, doubleClick); - `AbstractWidget.onClick` is an empty body
        let _ = (event, double_click);
        if !(compat::read(__this, |__t| __t.open)) {
            //   java: switchOpenStorage(index);
            if let Some(__outer) = compat::read(__this, |__t| __t.outer.upgrade()) {
                StorageOverlayScreen::switch_open_storage(&__outer, compat::read(__this, |__t| __t.index));
            }
        }
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput output)
    pub fn update_widget_narration(__this: &compat::Shared<Self>, output: &dyn NarrationElementOutput) {
    }

}

/// BackpackWidget's widget half (`super`'s fields: x, y, size, message, ...), lent by the game for each call
impl compat::WidgetHalf for BackpackWidget {
    fn widget_half(&mut self) -> &mut AbstractWidget {
        &mut self.base
    }
}

/// BackpackWidget as the game's widget: `compat::WidgetAdapter::widget(handle)` where Java hands it on
impl ModWidget for compat::WidgetAdapter<BackpackWidget> {
    /// java: @Override public void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_widget_render_state(&mut self, widget: &AbstractWidget, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        self.lend_ref(widget, |__h| BackpackWidget::extract_widget_render_state(__h, &mut context.graphics, mouse_x, mouse_y, a))
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput output)
    fn update_widget_narration(&mut self, widget: &AbstractWidget, output: &mut dyn NarrationElementOutput) {
        self.lend_ref(widget, |__h| BackpackWidget::update_widget_narration(__h, output))
    }

    /// java: @Override public void onClick(MouseButtonEvent event, boolean doubleClick)
    fn on_click(&mut self, widget: &mut AbstractWidget, event: &MouseButtonEvent, double_click: bool) {
        self.lend(widget, |__h| BackpackWidget::on_click(__h, event, double_click))
    }

}

// NOTE(fabric2seer): extractWidgetRenderState, onClick, updateWidgetNarration override(s) of AbstractWidget - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them

/// BackpackGridWidget's container half (`SearchableGridWidget extends AbstractContainerWidget`)
impl ContainerWidgetHalf for BackpackGridWidget {
    fn container_half(&mut self) -> &mut mcrust::net::minecraft::client::gui::components::abstract_container_widget::AbstractContainerWidget {
        &mut self.base.base
    }
}

/// BackpackGridWidget as the game's widget (`this.addRenderableWidget(grid)`): its methods dispatch through
/// `SearchableGridWidget` (the class's overrides, else the base bodies)
impl ModWidget for ContainerWidgetAdapter<BackpackGridWidget> {
    /// java: SearchableGridWidget.extractWidgetRenderState (BackpackGridWidget overrides it)
    fn extract_widget_render_state(&mut self, _widget: &AbstractWidget, context: &mut mcrust::mcrust::modding::widgets::WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        self.lend(|__h| { let __v = compat::view(__h, |a| a as compat::Shared<dyn SearchableGridWidget>); let __f = compat::dispatch(&__v, |__t| __t.extract_widget_render_state()); __f(&__v, &context.graphics, mouse_x, mouse_y, a) })
    }

    /// java: SearchableGridWidget.updateWidgetNarration
    fn update_widget_narration(&mut self, _widget: &AbstractWidget, output: &mut dyn mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput) {
        self.lend(|__h| { let __v = compat::view(__h, |a| a as compat::Shared<dyn SearchableGridWidget>); let __f = compat::dispatch(&__v, |__t| __t.update_widget_narration()); __f(&__v, &*output) })
    }

    /// java: SearchableGridWidget.mouseScrolled
    fn mouse_scrolled(&mut self, _widget: &mut AbstractWidget, x: f64, y: f64, scroll_x: f64, scroll_y: f64) -> bool {
        self.lend(|__h| { let __v = compat::view(__h, |a| a as compat::Shared<dyn SearchableGridWidget>); let __f = compat::dispatch(&__v, |__t| __t.mouse_scrolled()); __f(&__v, x, y, scroll_x, scroll_y) })
    }

    fn container_widget_mut(&mut self) -> Option<&mut mcrust::net::minecraft::client::gui::components::abstract_container_widget::AbstractContainerWidget> {
        Some(self.part_mut())
    }
}

/// `Math.ceilDiv(x, y)`: the quotient rounded towards positive infinity (`-floorDiv(-x, y)`).
fn ceil_div(x: i32, y: i32) -> i32 {
    compat::floor_div_i32(x.wrapping_neg(), y).wrapping_neg()
}

/// `StorageOverlayScreen.this.handler` of an inner object.
fn outer_handler(outer: &compat::WeakShared<StorageOverlayScreen>) -> Option<compat::Shared<StorageOverlayScreenHandler>> {
    compat::read(&outer.upgrade()?, |__t| __t.handler.clone())
}

/// `handler.getContainer()` is the `ChestMenu`'s container the handler was built over - the screen's `defaultHandler`
/// (the server's menu, read while it is still the open one).
fn outer_container_size(outer: &compat::WeakShared<StorageOverlayScreen>) -> Option<i32> {
    use mcrust::net::minecraft::world::container::Container as _;
    let menu: compat::LiveChestMenu = compat::read(&outer.upgrade()?, |__t| __t.default_handler)?;
    menu.with(|chest| chest.get_container().get_container_size())
}

/// `handler.getContainer().getItem(i)` (see `outer_container_size`); EMPTY once the menu is gone.
fn outer_container_item(outer: &compat::WeakShared<StorageOverlayScreen>, i: i32) -> mcrust::net::minecraft::world::item::item_stack::ItemStack {
    use mcrust::net::minecraft::world::container::Container as _;
    outer
        .upgrade()
        .and_then(|outer| compat::read(&outer, |__t| __t.default_handler))
        .and_then(|menu| menu.with(|chest| chest.get_container().get_item(i).duplicate()))
        .unwrap_or_else(mcrust::net::minecraft::world::item::item_stack::empty)
}
