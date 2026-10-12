//! Ported from de.hysky.skyblocker.skyblock.storageoverlay.StorageOverlayScreen by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
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
        let mut screen_position: crate::unresolved::Vector2dc = PREVIOUS_MOUSE_POSITION.get_mut().clone().unwrap();
        *PREVIOUS_MOUSE_POSITION.get_mut() = None;
        return Some(screen_position);
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
        let mut enabled: bool = (SkyblockerConfigManager::get().ui_and_visuals.storage_overlay.enabled && title == "storage" || compat::get_state(&OPEN_STORAGE) != -1 && !(compat::get_state(&DISABLE_ON_NEXT_LOAD)));
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
        // TODO(fabric2seer): `Minecraft.player` held as a value: the port lends the player only inside `game::with_player(|player| ..)` (a mod must never lock it)
        //   java: CLIENT.player.containerMenu = defaultHandler;
        // TODO(fabric2seer): the mod keeps the game's `Inventory` (Java holds a reference to the live object); the port's `Inventory` is not a value to copy and the port lends the mod no handle to it
        // TODO(fabric2seer): `set_screen` takes 14 argument(s) in the port (`gfx: &mut Minecraft, screen: &mut GuiScreen, screen_extra: &mut ScreenExtra, screen_stack: &mut Vec<HeldScreen>, screen_kind_before_pause: &mut Option<ScreenKind>, screen_is_open: &mut bool, level_screen_closed: &mut bool, frames_on_this_sc ..`), Java passes 1
        // rust: gui::set_screen(&mut ContainerScreen::new(compat::read(__this, |__t| __t.default_handler.clone()).unwrap(), mcrust::mcrust::modding::game::with_player(|player| player.base.base.get_inventory().clone()).unwrap(), __this.lock().borrow().name.as_ref().unwrap().get_string()));
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
        let __o1 = compat::read(__this, |__t| __t.grid.clone()).unwrap();
        let __o2 = compat::read(__this, |__t| __t.grid.clone()).unwrap();
        return (if compat::read(__this, |__t| __t.grid.clone()).is_some() { (compat::write(&__o2, |__t| __t.get_width())).wrapping_add(16) } else { StorageOverlayScreen::get_max_width(__this) });
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
        // TODO(fabric2seer): `reposition_elements` takes 1 argument(s) in the port (`build: impl FnOnce(&mut Screen)`), Java passes 0
        // rust: __this.lock().borrow_mut().base.screen.reposition_elements();
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
        return (__this.lock().borrow_mut().base.is_hovering_slot(slot, xm, ym) && compat::read(__this, |__t| __t.grid.clone()).is_none() || slot.container.as_any().is::<Inventory>() || (({ let __r = &compat::read(__this, |__t| __t.grid.clone()).unwrap(); let __f = compat::dispatch(__r, |__t| __t.get_grid_rectangle()); __f(&compat::view(__r, |a| a as compat::Shared<dyn SearchableGridWidget>)) })).contains_point(((xm) as i32), ((ym) as i32)));
    }

    /// java: @Override protected void init()
    pub fn init(__this: &compat::Shared<Self>) {
        ();
        let mut storages_per_row: i32 = SkyblockerConfigManager::get().ui_and_visuals.storage_overlay.storages_per_row;
        storages_per_row = (if storages_per_row > 0 { storages_per_row } else { i32::MAX });
        let mut internal_cols: i32 = SkyblockerConfigManager::get().ui_and_visuals.storage_overlay.backpack_width;
        { let __v = Some(BackpackGridWidget::new((StorageOverlayScreen::get_min_left_pos(__this)).wrapping_add(8), (compat::read(__this, |__t| __t.top_pos)).wrapping_add(8), (StorageOverlayScreen::get_max_width(__this)).wrapping_sub(16), (StorageOverlayScreen::get_height(__this)).wrapping_sub(16), storages_per_row, internal_cols, true)); compat::write(__this, |__t| __t.grid = __v); }
        ({ let __r = &compat::read(__this, |__t| __t.grid.clone()).unwrap(); let __f = compat::dispatch(__r, |__t| __t.set_search()); __f(&compat::view(__r, |a| a as compat::Shared<dyn SearchableGridWidget>), &compat::get_state(&SAVED_SEARCH)) });
        ({ let __r = &compat::read(__this, |__t| __t.grid.clone()).unwrap(); let __f = compat::dispatch(__r, |__t| __t.set_scroll_amount()); __f(&compat::view(__r, |a| a as compat::Shared<dyn SearchableGridWidget>), compat::get_state(&SAVED_SCROLL)) });
        let __r0 = compat::read(__this, |__t| __t.grid.clone());
        { let (__q0,) = (__r0.unwrap(),); __this.lock().borrow_mut().screen.add_renderable_widget(__q0) };
        // TODO(fabric2seer): field `VERTICAL` of a value of unknown type
        //   java: LinearLayout extraButtons = new LinearLayout(width - 90, height - 84, LinearLayout.Orientation.VERTICAL);
        let mut extra_buttons: LinearLayout = todo!("fabric2seer: the initialiser of `extraButtons` (TODO above)");
        extra_buttons.spacing(5);
        // TODO(fabric2seer): a widget put in a layout outside a mod screen's own methods: the port's layouts hold a widget by its index in the screen's widget list
        // rust: let __this_w1 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        // rust: extra_buttons.add_child((button::builder(component::translatable("skyblocker.config.uiAndVisuals.storageOverlay.farmingToolkitButton").get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { let Some(__this_1) = __this_w1.get().upgrade() else { return Default::default(); }; { StorageOverlayScreen::toolkit((&__this_1), (__screen.widget_mut(__index))); } })))).size(80, 16).build());
        // TODO(fabric2seer): a widget put in a layout outside a mod screen's own methods: the port's layouts hold a widget by its index in the screen's widget list
        // rust: let __this_w2 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        // rust: extra_buttons.add_child((button::builder(component::translatable("skyblocker.config.uiAndVisuals.storageOverlay.huntingToolkitButton").get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { let Some(__this_2) = __this_w2.get().upgrade() else { return Default::default(); }; { StorageOverlayScreen::hunting_toolkit((&__this_2), (__screen.widget_mut(__index))); } })))).size(80, 16).build());
        // TODO(fabric2seer): a widget put in a layout outside a mod screen's own methods: the port's layouts hold a widget by its index in the screen's widget list
        // rust: let __this_w3 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        // rust: extra_buttons.add_child((button::builder(component::translatable("skyblocker.config.uiAndVisuals.storageOverlay.homeButton").get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { let Some(__this_3) = __this_w3.get().upgrade() else { return Default::default(); }; { StorageOverlayScreen::home((&__this_3), (__screen.widget_mut(__index))); } })))).size(80, 16).build());
        // TODO(fabric2seer): a widget put in a layout outside a mod screen's own methods: the port's layouts hold a widget by its index in the screen's widget list
        // rust: let __this_w4 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        // rust: extra_buttons.add_child((button::builder(component::translatable("skyblocker.config.uiAndVisuals.storageOverlay.hideButton").get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { let Some(__this_4) = __this_w4.get().upgrade() else { return Default::default(); }; { StorageOverlayScreen::hide((&__this_4), (__screen.widget_mut(__index))); } })))).size(80, 16).build());
        extra_buttons.arrange_elements();
        // TODO(fabric2seer): a lambda for a parameter the port does not type as a closure (`Consumer<net.minecraft.client.gui.components.AbstractWidget>`)
        // rust: let __this_w5 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        // rust: layout::visit_widgets(|__a0| { let Some(__this_5) = __this_w5.get().upgrade() else { return Default::default(); }; { { let (__q0,) = (__a0.clone(),); (&__this_5).lock().borrow_mut().screen.add_renderable_widget(__q0) }; } });
    }

    /// java: @Override public void extractRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_render_state(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        ();
        // TODO(fabric2seer): `AbstractContainerScreen.extractTooltip` has no Rust method in the port (looked for `extract_tooltip` in AbstractContainerScreen and its supertypes)
        // rust: __this.lock().borrow().extract_tooltip(graphics, mouse_x, mouse_y);
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
        // TODO(fabric2seer): `on_close` takes 2 argument(s) in the port (`menu: &AbstractContainerMenu, containers: &mut Containers<'_>`), Java passes 0
        // rust: __this.lock().borrow_mut().base.on_close();
    }

    /// java: @Override protected void extractSlot(GuiGraphicsExtractor graphics, Slot slot, int mouseX, int mouseY)
    pub fn extract_slot(__this: &compat::Shared<Self>, graphics: &mut GuiGraphicsExtractor, slot: &Slot, mut mouse_x: i32, mut mouse_y: i32) {
        if slot.container.as_any().is::<Inventory>() || compat::read(__this, |__t| __t.grid.clone()).is_none() {
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
        // TODO(fabric2seer): `has_clicked_outside` takes 2 argument(s) in the port (`mx: f64, my: f64`), Java passes 4
        // rust: let __r0 = compat::read(__this, |__t| __t.screen.width);
        // rust: let __r1 = compat::read(__this, |__t| __t.top_pos);
        // rust: let __r2 = compat::read(__this, |__t| __t.top_pos);
        // rust: return (__this.lock().borrow_mut().base.has_clicked_outside(mouse_x, mouse_y, xo, yo) && mouse_x < (StorageOverlayScreen::get_left_pos(__this) as f64) || mouse_x > ((StorageOverlayScreen::get_left_pos(__this)).wrapping_add(__r0) as f64) || mouse_y < (__r1 as f64) || mouse_y > ((__r2).wrapping_add(StorageOverlayScreen::get_height(__this)) as f64));
        return todo!("fabric2seer: see the TODO above");
    }

    /// java: @Override public boolean mouseScrolled(double x, double y, double scrollX, double scrollY)
    pub fn mouse_scrolled(__this: &compat::Shared<Self>, mut x: f64, mut y: f64, mut scroll_x: f64, mut scroll_y: f64) -> bool {
        // TODO(fabric2seer): `has_item` takes 1 argument(s) in the port (`containers: &Containers<'_>`), Java passes 0
        if compat::read(__this, |__t| __t.hovered_slot.clone()).is_some() && __this.lock().borrow().hovered_slot.as_ref().unwrap().has_item() {
            return false;
        }
        // TODO(fabric2seer): Optional.mouseScrolled
        //   java: return this.getChildAt(x, y).filter(child -> child.mouseScrolled(x, y, scrollX, scrollY)).isPresent();
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public boolean keyPressed(KeyEvent event)
    pub fn key_pressed(__this: &compat::Shared<Self>, event: &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        // TODO(fabric2seer): `get_current_focus_path` takes 1 argument(s) in the port (`own_index: usize`), Java passes 0
        // TODO(fabric2seer): `get_current_focus_path` is a method of the port's trait `GuiEventListener`, which the port's `AbstractContainerScreen` does not implement
        // rust: let mut focus_path: Option<ComponentPath> = __this.lock().borrow().get_current_focus_path();
        let mut focus_path: ComponentPath = todo!("fabric2seer: the initialiser of `focusPath` (TODO above)");
        // TODO(fabric2seer): `Options.keyInventory` has no Rust field in the port (searched net.minecraft.client.Options and its supertypes)
        if compat::minecraft().options.borrow().key_inventory.matches(event) && true /* java: focus_path != null: never null here */ && focus_path.leaf_component().as_any().is::<EditBox>() {
            return true;
        }
        return false;
    }

    /// java: @Override public void removed()
    pub fn removed(__this: &compat::Shared<Self>) {
        ();
        if compat::read(__this, |__t| __t.save_mouse_position) {
            // TODO(fabric2seer): `new Vector2d(..)`
            //   java: previousMousePosition = new Vector2d(minecraft.mouseHandler.xpos(), minecraft.mouseHandler.ypos());
        }
    }

    /// java: @Override public void extractBackground(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_background(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        compat::call_super();
        // TODO(fabric2seer): `get` is not a method of FallbackedTexture
        //   java: graphics.blitSprite(RenderPipelines.GUI_TEXTURED, BACKGROUND.get(), getLeftPos(), this.topPos, getWidth(), getHeight());
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
    pub fn new(mut x: i32, mut y: i32, mut width: i32, mut height: i32, mut max_storages_per_row: i32, mut internal_cols: i32, mut packed: bool) -> compat::Shared<Self> {
        let mut expected_width: i32 = (internal_cols.wrapping_mul(SLOT_SIZE)).wrapping_add(EDGE_PADDING.wrapping_mul(2));
        // TODO(fabric2seer): super(..) with 0 argument(s)
        //   java:
        let __this: compat::Shared<Self> = todo!("constructor");
        while expected_width > width.wrapping_sub(mcrust::net::minecraft::client::gui::components::abstract_scroll_area::SCROLLBAR_WIDTH) {
            expected_width = (({ internal_cols -= 1; internal_cols }).wrapping_mul(SLOT_SIZE)).wrapping_add(EDGE_PADDING.wrapping_mul(2));
        }
        // TODO(fabric2seer): super statement
        //   java: super(x, y, width, height, Component.literal("BackPack grid"), expectedWidth, maxStoragesPerRow, packed, true);
        let mut storage_loaded: bool = false;
        let mut storages: Vec<compat::Shared<Storage>> = BackpackPreview::get_storages().unwrap();
        for i in 0..(storages.len() as i32) {
            let mut storage: Option<compat::Shared<Storage>> = Some(storages[(i) as usize].clone());
            let mut open: bool = (compat::get_state(&OPEN_STORAGE) == i);
            if let Some(storage) = &storage {
                storage_loaded = true;
                let mut widget: compat::Shared<BackpackWidget> = BackpackWidget::new(internal_cols, i, storage.clone(), open);
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
        // TODO(fabric2seer): the condition: unknown name `hoveredSlot`
        //   java: hoveredSlot != null && getGridRectangle().containsPoint((int) mouseX, (int) mouseY)
        if todo!("fabric2seer: the condition (TODO above)") {
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
        return compat::read(__this, |__t| __t.backpack_widgets.clone()).into_iter().filter(|backpack| BackpackWidget::matches(backpack, &input)).collect::<Vec<_>>();
    }

    /// BackpackGridWidget.filterWidgets as the dispatch of SearchableGridWidget.filterWidgets
    pub fn filter_widgets_dyn(__this: &compat::Shared<dyn SearchableGridWidget>, __p0: String) -> Vec<AbstractWidget> {
        Self::filter_widgets_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor context, int mouseX, int mouseY, float deltaTicks)
    pub fn extract_widget_render_state_impl(__this: &compat::Shared<Self>, context: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut delta_ticks: f32) {
        <dyn SearchableGridWidget>::extract_widget_render_state_body(&compat::view(__this, |a| a as compat::Shared<dyn SearchableGridWidget>), context, mouse_x, mouse_y, delta_ticks);
        let __o1 = compat::read(__this, |__t| __t.open_backpack.clone()).unwrap();
        let __o2 = compat::read(__this, |__t| __t.open_backpack.clone()).unwrap();
        let __o3 = compat::read(__this, |__t| __t.open_backpack.clone()).unwrap();
        let __o4 = compat::read(__this, |__t| __t.open_backpack.clone()).unwrap();
        if compat::read(__this, |__t| __t.open_backpack.clone()).is_some() && compat::write(&__o3, |__t| __t.get_bottom()) < (__this.lock().borrow().get_y()).wrapping_add(HEADER_H) || compat::write(&__o4, |__t| __t.get_y()) >= __this.lock().borrow().get_bottom() {
            // TODO(fabric2seer): `handler` is a field of the enclosing instance (StorageOverlayScreen.this) - an inner class has no outer reference in Rust
            //   java: handler.moveBackpackSlots(0, -1000, 9);
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
    pub fn new(mut columns: i32, mut index: i32, mut storage: compat::Shared<Storage>, mut open: bool) -> compat::Shared<Self> {
        // TODO(fabric2seer): Math.ceilDiv
        //   java: int rows = Math.ceilDiv(storage.size() - 9, columns);
        let mut rows: i32 = todo!("fabric2seer: the initialiser of `rows` (TODO above)");
        // TODO(fabric2seer): `new` takes 7 argument(s) in the port (`x: i32, y: i32, width: i32, height: i32, message: String, kind: WidgetKind, on_press: OnPress`), Java passes 0
        //   java:
        let __this: compat::Shared<Self> = todo!("constructor");
        if open {
            // TODO(fabric2seer): `handler` is a field of the enclosing instance (StorageOverlayScreen.this) - an inner class has no outer reference in Rust
            //   java: rows = Math.ceilDiv(handler.getContainer().getContainerSize() - 9, columns);
        }
        // TODO(fabric2seer): super statement
        //   java: super(0, 0, columns * SLOT_SIZE + EDGE_PADDING * 2, rows * SLOT_SIZE + HEADER_H + EDGE_PADDING, Component.literal("Backpack preview"));
        { let __v = rows; compat::write(&__this, |__t| __t.rows = __v); }
        { let __v = columns; compat::write(&__this, |__t| __t.columns = __v); }
        { let __v = StorageOverlayScreen::get_storage_name(index); compat::write(&__this, |__t| __t.label = __v); }
        { let __v = index; compat::write(&__this, |__t| __t.index = __v); }
        { let __v = Some(storage.clone()); compat::write(&__this, |__t| __t.storage = __v); }
        { let __v = open; compat::write(&__this, |__t| __t.open = __v); }
        __this
    }

    /// java: private int size()
    pub fn size(__this: &compat::Shared<Self>) -> i32 {
        if compat::read(__this, |__t| __t.open) {
            // TODO(fabric2seer): `handler` is a field of the enclosing instance (StorageOverlayScreen.this) - an inner class has no outer reference in Rust
            //   java: return handler.getContainer().getContainerSize();
        } else {
            return Storage::size(&compat::read(__this, |__t| __t.storage.clone()).unwrap());
        }
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: public boolean matches(String filter)
    pub fn matches(__this: &compat::Shared<Self>, filter: &str) -> bool {
        __this.lock().borrow_mut().matched_slots.clear();
        let mut filter_lower_case: String = filter.to_lowercase();
        for i in 9..BackpackWidget::size(__this) {
            // TODO(fabric2seer): `handler` is a field of the enclosing instance (StorageOverlayScreen.this) - an inner class has no outer reference in Rust
            //   java: ItemStack currentStack = open ? handler.getContainer().getItem(i) : storage.getStack(i);
            let mut current_stack: mcrust::net::minecraft::world::item::item_stack::ItemStack = todo!("fabric2seer: the initialiser of `currentStack` (TODO above)");
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
                    // TODO(fabric2seer): the condition: unknown name `topPos`
                    //   java: graphics.containsPointInScissor(mouseX, mouseY) && mouseX > itemX && mouseX <= itemX + SLOT_SIZE && mouseY > itemY && mouseY <= itemY + SLOT_SIZE && mouseY > topPos && mouseY < topPos + StorageOverlayScreen.this.getHeight()
                    if todo!("fabric2seer: the condition (TODO above)") {
                        // TODO(fabric2seer): `GuiGraphicsExtractor.setTooltipForNextFrame` has no Rust method in the port (looked for `set_tooltip_for_next_frame` in GuiGraphicsExtractor and its supertypes)
                        // rust: graphics.set_tooltip_for_next_frame(&compat::font(), &current_stack, mouse_x, mouse_y);
                    }
                }
                if !({ let (__q0,) = ((i) as usize,); __this.lock().borrow().matched_slots.get(__q0) }) {
                    graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), item_x, item_y, item_x.wrapping_add(16), item_y.wrapping_add(16), *NOT_MATCHED_COLOR);
                }
            }
        } else {
            // TODO(fabric2seer): `handler` is a field of the enclosing instance (StorageOverlayScreen.this) - an inner class has no outer reference in Rust
            //   java: handler.moveBackpackSlots(getX() - leftPos + 8, getY() - topPos + SLOT_SIZE, columns);
        }
    }

    /// java: @Override public void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_widget_render_state(__this: &compat::Shared<Self>, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        BackpackWidget::render_custom_backpack(__this, graphics, mouse_x, mouse_y, __this.lock().borrow().get_x(), __this.lock().borrow().get_y(), compat::read(__this, |__t| __t.rows), compat::read(__this, |__t| __t.columns), &compat::read(__this, |__t| __t.label.clone()), &compat::read(__this, |__t| __t.storage.clone()).unwrap());
        if compat::read(__this, |__t| __t.open) {
            // TODO(fabric2seer): `getWidth(..)` is a method of the enclosing instance (StorageOverlayScreen.this) - an inner class has no outer reference in Rust
            //   java: graphics.outline(getX(), getY(), getWidth(), getHeight(), Color.yellow.getRGB());
        }
        if __this.lock().borrow().is_hovered() && !(compat::read(__this, |__t| __t.open)) {
            graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), __this.lock().borrow().get_x(), __this.lock().borrow().get_y(), __this.lock().borrow().get_right(), __this.lock().borrow().get_bottom(), argb::white_f32(0.1f32));
        }
    }

    /// java: @Override public void onClick(MouseButtonEvent event, boolean doubleClick)
    pub fn on_click(__this: &compat::Shared<Self>, event: &MouseButtonEvent, mut double_click: bool) {
        // TODO(fabric2seer): `AbstractWidget.onClick` has no Rust method in the port (looked for `on_click` in AbstractWidget and its supertypes)
        // rust: __this.lock().borrow_mut().base.on_click(event, double_click);
        if !(compat::read(__this, |__t| __t.open)) {
            // TODO(fabric2seer): `switchOpenStorage(..)` is a method of the enclosing instance (StorageOverlayScreen.this) - an inner class has no outer reference in Rust
            //   java: switchOpenStorage(index);
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
