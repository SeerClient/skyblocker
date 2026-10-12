//! Ported from de.hysky.skyblocker.skyblock.dungeon.partyfinder.PartyFinderScreen by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::dungeon::partyfinder::finder_settings_container::FinderSettingsContainer;
use crate::de::hysky::skyblocker::skyblock::dungeon::partyfinder::finder_settings_container::ContainerWidgetAdapter;
use crate::de::hysky::skyblocker::skyblock::dungeon::partyfinder::party_entry::NoParties;
use crate::de::hysky::skyblocker::skyblock::dungeon::partyfinder::party_entry::PartyEntry;
use crate::de::hysky::skyblocker::skyblock::dungeon::partyfinder::party_entry::PartyEntryBase;
use crate::de::hysky::skyblocker::skyblock::dungeon::partyfinder::party_entry::YourParty;
use crate::de::hysky::skyblocker::skyblock::dungeon::partyfinder::party_entry_list_widget::PartyEntryListWidget;
use crate::de::hysky::skyblocker::utils::container_utils::ContainerUtils;
use crate::de::hysky::skyblocker::utils::item_utils::ItemUtils;
use crate::de::hysky::skyblocker::utils::render::texture::fallbacked_texture;
use crate::de::hysky::skyblocker::utils::render::texture::fallbacked_texture::FallbackedTexture;
use mcrust::com::mojang::authlib::properties::property_map::PropertyMap;
use mcrust::java::text::decimal_format_symbols::Locale;
use mcrust::mcrust::modding::screens::ModScreen;
use mcrust::mcrust::modding::screens::ScreenRenderContext;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::abstract_widget::PressLambda;
use mcrust::net::minecraft::client::gui::components::button;
use mcrust::net::minecraft::client::gui::components::edit_box::EditBox;
use mcrust::net::minecraft::client::gui::components::edit_box::Responder;
use mcrust::net::minecraft::client::gui::components::edit_box::ResponderLambda;
use mcrust::net::minecraft::client::gui::components::toasts::system_toast::SystemToast;
use mcrust::net::minecraft::client::gui::gui;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::screens::inventory::container_screen::ContainerScreen;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::client::input::mouse_button_info::MouseButtonInfo;
use mcrust::net::minecraft::client::minecraft;
use mcrust::net::minecraft::client::player::local_player;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::network::protocol::game::serverbound_sign_update_packet::ServerboundSignUpdatePacket;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::world::entity::player::inventory::Inventory;
use mcrust::net::minecraft::world::inventory::container_input::ContainerInput;
use mcrust::net::minecraft::world::inventory::slot::Slot;
use mcrust::net::minecraft::world::item::items;
use mcrust::net::minecraft::world::level::block::entity::sign_block_entity::SignBlockEntity;
use crate::de::hysky::skyblocker::injected::skyblocker_stack::SkyblockerStack as _;
use mcrust::net::minecraft::network::chat::component::Component as _;
use mcrust::net::minecraft::world::level::entity::uniquely_identifyable::UniquelyIdentifyable as _;

/// java: protected static final FallbackedTexture<Identifier> BACKGROUND_TEXTURE = FallbackedTexture.ofGuiSprite( SkyblockerMod.id("party_finder/background"), Identifier.withDefaultNamespace("social_interactio
pub static BACKGROUND_TEXTURE: compat::MainThread<compat::Shared<dyn FallbackedTexture<Identifier>>> = compat::MainThread::new(|| fallbacked_texture::of_gui_sprite(&crate::entry::skyblocker_mod::id("party_finder/background"), &Identifier::with_default_namespace("social_interactions/background")));
/// java: protected static final Identifier SEARCH_ICON_TEXTURE = Identifier.withDefaultNamespace("icon/search");
pub static SEARCH_ICON_TEXTURE: compat::MainThread<Identifier> = compat::MainThread::new(|| Identifier::with_default_namespace("icon/search"));
/// java: protected static final Component SEARCH_TEXT = Component.translatable("gui.socialInteractions.search_hint").withStyle(ChatFormatting.ITALIC).withStyle(ChatFormatting.GRAY);
pub static SEARCH_TEXT: compat::MainThread<MutableComponent> = compat::MainThread::new(|| component::translatable("gui.socialInteractions.search_hint").with_style_format(ChatFormatting::Italic).with_style_format(ChatFormatting::Gray));
/// java: public static boolean isInKuudraPartyFinder = false;
pub static IS_IN_KUUDRA_PARTY_FINDER: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));
/// java: public static boolean DEBUG = false;
pub static DEBUG: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));
/// java: public static final List<String> possibleInventoryNames = List.of( "party finder", "search settings", "select floor", "select type", "class level range", "dungeon level range", "sort" );
pub static POSSIBLE_INVENTORY_NAMES: LazyLock<Vec<String>> = LazyLock::new(|| vec!["party finder".to_string(), "search settings".to_string(), "select floor".to_string(), "select type".to_string(), "class level range".to_string(), "dungeon level range".to_string(), "sort".to_string()]);
/// java: public static Map<String, PropertyMap> floorIconsNormal = null;
pub static FLOOR_ICONS_NORMAL: compat::MainThread<Option<std::collections::HashMap<String, PropertyMap>>> = compat::MainThread::new(|| None);
/// java: public static Map<String, PropertyMap> floorIconsMaster = null;
pub static FLOOR_ICONS_MASTER: compat::MainThread<Option<std::collections::HashMap<String, PropertyMap>>> = compat::MainThread::new(|| None);

/// java: public class PartyFinderScreen extends Screen
pub struct PartyFinderScreen {
    /// java: extends net.minecraft.client.gui.screens.Screen
    pub base: mcrust::net::minecraft::client::gui::screens::screen::Screen,
    /// java: private Component name;
    pub name: MutableComponent,
    /// java: private ChestMenu handler;
    pub handler: compat::LiveChestMenu,
    /// java: private final Inventory inventory;
    pub inventory: Inventory,
    /// java: private Page currentPage;
    pub current_page: Option<Page>,
    /// java: public PartyEntryListWidget partyEntryListWidget;
    pub party_entry_list_widget: Option<compat::WidgetRef>,
    /// java: private FinderSettingsContainer settingsContainer;
    pub settings_container: Option<compat::Shared<FinderSettingsContainer>>,
    /// java: private int refreshSlotId = -1;
    pub refresh_slot_id: i32,
    /// java: private EditBox searchField;
    /// (a widget of the screen, as the buttons are: the edit box is its `WidgetKind::EditBox`)
    pub search_field: Option<compat::WidgetRef>,
    /// java: private Button refreshButton;
    pub refresh_button: Option<compat::WidgetRef>,
    /// java: private Button previousPageButton;
    pub previous_page_button: Option<compat::WidgetRef>,
    /// java: private int prevPageSlotId = -1;
    pub prev_page_slot_id: i32,
    /// java: private Button nextPageButton;
    pub next_page_button: Option<compat::WidgetRef>,
    /// java: private int nextPageSlotId = -1;
    pub next_page_slot_id: i32,
    /// java: protected Button partyFinderButton;
    pub party_finder_button: Option<compat::WidgetRef>,
    /// java: protected int partyButtonSlotId = -1;
    pub party_button_slot_id: i32,
    /// java: private Button settingsButton;
    pub settings_button: Option<compat::WidgetRef>,
    /// java: private int settingsButtonSlotId = -1;
    pub settings_button_slot_id: i32,
    /// java: private Button createPartyButton;
    pub create_party_button: Option<compat::WidgetRef>,
    /// java: private int createPartyButtonSlotId = -1;
    pub create_party_button_slot_id: i32,
    /// java: private boolean dirty = false;
    pub dirty: bool,
    /// java: private boolean resetScroll = false;
    pub reset_scroll: bool,
    /// java: private long dirtiedTime;
    pub dirtied_time: i64,
    /// java: private boolean waitingForServer = false;
    pub waiting_for_server: bool,
    /// java: private boolean signFront = true;
    pub sign_front: bool,
    /// java: private @Nullable SignBlockEntity sign = null;
    /// (the level's own block entity object, as `ClientLevel.getBlockEntity` shares it - a `SignBlockEntity`)
    pub sign: Option<std::sync::Arc<dyn mcrust::net::minecraft::world::level::block::entity::block_entity::BlockEntityObj>>,
    /// java: private boolean aborted = false;
    pub aborted: bool,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
    /// the screen's widgets as this object last saw them (see `compat::WidgetCache`)
    pub __widgets: compat::WidgetCache,
}

impl std::ops::Deref for PartyFinderScreen {
    type Target = mcrust::net::minecraft::client::gui::screens::screen::Screen;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for PartyFinderScreen {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl PartyFinderScreen {
    /// java: public PartyFinderScreen(ChestMenu handler, Inventory inventory, Component title)
    pub fn new(mut handler: compat::LiveChestMenu, mut inventory: Inventory, mut title: MutableComponent) -> compat::Shared<Self> {
        let __init_handler = handler;
        // TODO(fabric2seer): the mod keeps the game's `Inventory` (Java holds a reference to the live object); the port's `Inventory` is not a value to copy and the port lends the mod no handle to it
        //   java: this.inventory = inventory;
        let __init_name = title.clone();
        let __this = compat::shared(Self {
            base: mcrust::net::minecraft::client::gui::screens::screen::Screen::new(title.get_string()),
            self_ref: std::sync::Weak::new(),
            __widgets: Default::default(),
            name: __init_name,
            handler: __init_handler,
            inventory: todo!("fabric2seer: the field value (TODO above)"),
            current_page: None,
            party_entry_list_widget: None,
            settings_container: None,
            refresh_slot_id: -1,
            search_field: None,
            refresh_button: None,
            previous_page_button: None,
            prev_page_slot_id: -1,
            next_page_button: None,
            next_page_slot_id: -1,
            party_finder_button: None,
            party_button_slot_id: -1,
            settings_button: None,
            settings_button_slot_id: -1,
            create_party_button: None,
            create_party_button_slot_id: -1,
            dirty: false,
            reset_scroll: false,
            dirtied_time: 0,
            waiting_for_server: false,
            sign_front: true,
            sign: None,
            aborted: false,
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        __this
    }

    /// java: @Init public static void initClass()
    pub fn init_class() {
        // TODO(fabric2seer): no SeerClient event yet: hook RETURN of Minecraft's constructor (`net.minecraft.client.Minecraft#<init>`: exists)
        //   java: ClientLifecycleEvents.CLIENT_STARTED.register(client -> {
        //   java: 			//Checking when this is loaded probably isn't necessary as the maps are always null checked
        //   java: 			CompletableFuture.runAsync(() -> {
        //   java: 				floorIconsNormal = new HashMap<>();
        //   java: 				floorIconsMaster = new HashMap<>();
        //   java: 				try (BufferedReader skullTextureReader = client.getResourceManager().openAsReader(SkyblockerMod.id("dungeons/catacombs/floorskulls.json"))) {
        //   java: 					JsonObject json = SkyblockerMod.GSON.fromJson(skullTextureReader, JsonObject.class);
        //   java: 					json.getAsJsonObject("normal").asMap().forEach((s, tex) -> floorIconsNormal.put(s, ItemUtils.propertyMapWithTexture(tex.getAsString())));
        //   java: 					json.getAsJsonObject("master").asMap().forEach((s, tex) -> floorIconsMaster.put(s, ItemUtils.propertyMapWithTexture(tex.getAsString())));
        //   java: 					LOGGER.debug("[Skyblocker] Dungeons floor skull textures json loaded");
        //   java: 				} catch (Exception e) {
        //   java: 					LOGGER.error("[Skyblocker] Failed to load dungeons floor skull textures json", e);
        //   java: 				}
        //   java: 			}, SkyblockerMod.VIRTUAL_THREAD_EXECUTOR);
        //   java: 		});
        // TODO(fabric2seer): ClientSendMessageEvents.COMMAND.register: no SeerClient equivalent yet
        //   java: ClientSendMessageEvents.COMMAND.register(command -> {
        //   java: 			if (!Utils.isOnSkyblock() || !SkyblockerConfigManager.get().dungeons.fancyPartyFinder) return;
        //   java: 			command = command.toLowerCase(Locale.ENGLISH);
        //   java: 			if (!command.startsWith("join")) return;
        //   java: 			isInKuudraPartyFinder = command.startsWith("joinkuudra") || command.startsWith("joininstance kuudra_");
        //   java: 		});
    }

    /// java: public ChestMenu getHandler()
    pub fn get_handler(__this: &compat::Shared<Self>) -> compat::LiveChestMenu {
        return compat::read(__this, |__t| __t.handler.clone());
    }

    /// java: public FinderSettingsContainer getSettingsContainer()
    pub fn get_settings_container(__this: &compat::Shared<Self>) -> compat::Shared<FinderSettingsContainer> {
        return compat::read(__this, |__t| __t.settings_container.clone()).unwrap();
    }

    /// java: public void markDirty()
    pub fn mark_dirty(__this: &compat::Shared<Self>) {
        if compat::read(__this, |__t| __t.sign.is_some()) {
            return;
        }
        { let __v = compat::millis(); compat::write(__this, |__t| __t.dirtied_time = __v); }
        { let __v = true; compat::write(__this, |__t| __t.dirty = __v); }
    }

    /// java: @Override protected void init()
    pub fn init(__this: &compat::Shared<Self>) {
        ();
        let mut top_row_buttons_height: i32 = 20;
        let mut widget_height: i32 = ((((compat::read(__this, |__t| __t.height) as f64) * 0.8)) as i32);
        let mut entry_list_top_y: i32 = 43i32.max(((((compat::read(__this, |__t| __t.height) as f64) * 0.1)) as i32));
        { let __v = (Some(PartyEntryListWidget::new(compat::minecraft(), compat::read(__this, |__t| __t.width), widget_height, entry_list_top_y, 68))).map(|__o| compat::WidgetRef::value(compat::attach_list(__o))); compat::write(__this, |__t| { let __s = &mut *__t; __s.party_entry_list_widget = __v; }); }
        // TODO(fabric2seer): `new` takes 5 argument(s) in the port (`splitter: &StringSplitter<W>, x: i32, y: i32, width: i32, height: i32`), Java passes 6
        // rust: { let __v = Some(EditBox::new(&compat::font(), component::literal("Search..."), (compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.party_entry_list_widget.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.mod_widget().and_then(|__ms| __ms.with::<mcrust::mcrust::modding::widgets::ModSelectionList, _>(|__l| __l.list.get_row_left())).expect("a mod list reached from inside its own method")) })).wrapping_add(12), entry_list_top_y.wrapping_sub(12), ((compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.party_entry_list_widget.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.mod_widget().and_then(|__ms| __ms.with::<mcrust::mcrust::modding::widgets::ModSelectionList, _>(|__l| { let __h = &__l.class::<compat::ListClass<PartyEntryListWidget>>().map(|__c| __c.object().clone()).expect("a mod list without its class"); compat::lend_list(__h, __l, || PartyEntryListWidget::get_row_width(__h)) })).expect("a mod list reached from inside its own method")) })).wrapping_sub(12i32.wrapping_mul(3))).wrapping_sub(6), 12)); compat::write(__this, |__t| __t.search_field = __v); }
        { let __q0 = SEARCH_TEXT.get().clone(); compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.search_field.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| if let mcrust::net::minecraft::client::gui::components::abstract_widget::WidgetKind::EditBox(__e) = &mut __w.kind { __e.set_hint_component(mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(__q0)); }) }); }
        let __this_w1 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        { let (__q0,) = (Responder::Lambda(ResponderLambda::new(move |s| { let Some(__this_1) = __this_w1.get().upgrade() else { return Default::default(); }; { let __wa1 = s.to_string(); compat::write((&__this_1), |__t| { let __s = &mut *__t; let __r = __s.party_entry_list_widget.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.mod_widget().and_then(|__ms| __ms.with::<mcrust::mcrust::modding::widgets::ModSelectionList, _>(|__l| { let __h = &__l.class::<compat::ListClass<PartyEntryListWidget>>().map(|__c| __c.object().clone()).expect("a mod list without its class"); compat::lend_list(__h, __l, || PartyEntryListWidget::set_search(__h, &__wa1)) })).expect("a mod list reached from inside its own method")) }); } })),); compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.search_field.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| if let mcrust::net::minecraft::client::gui::components::abstract_widget::WidgetKind::EditBox(__e) = &mut __w.kind { __e.set_responder(__q0); }) }) };
        let __this_w2 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        { let __v = (Some((button::builder((component::literal("⟳")).set_style(mcrust::net::minecraft::network::chat::style::Style::EMPTY.with_color_format(Some(ChatFormatting::Green))).get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { let Some(__this_2) = __this_w2.get().upgrade() else { return Default::default(); }; compat::lend_screen(&__this_2, __screen, || { let mut __pidx = compat::WidgetRef::Screen(__index); {
            if compat::read((&__this_2), |__t| __t.refresh_slot_id) != -1 {
                PartyFinderScreen::click_and_wait_for_server((&__this_2), compat::read((&__this_2), |__t| __t.refresh_slot_id));
                { let __v = true; compat::write((&__this_2), |__t| __t.reset_scroll = __v); }
            }
        } }) })))).pos(((compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.search_field.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.get_x()) })).wrapping_add(compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.search_field.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.get_width()) }))).wrapping_add(12i32.wrapping_mul(2)), compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.search_field.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.get_y()) })).size(12, 12).build())).map(compat::WidgetRef::value); compat::write(__this, |__t| { let __s = &mut *__t; __s.refresh_button = __v; }); }
        { let __v = false; compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.refresh_button.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.active = __v) }); }
        let __this_w3 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        { let __v = (Some((button::builder((component::literal("←")).get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { let Some(__this_3) = __this_w3.get().upgrade() else { return Default::default(); }; compat::lend_screen(&__this_3, __screen, || { let mut __pidx = compat::WidgetRef::Screen(__index); {
            if compat::read((&__this_3), |__t| __t.prev_page_slot_id) != -1 {
                PartyFinderScreen::click_and_wait_for_server((&__this_3), compat::read((&__this_3), |__t| __t.prev_page_slot_id));
                { let __v = true; compat::write((&__this_3), |__t| __t.reset_scroll = __v); }
            }
        } }) })))).pos((compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.search_field.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.get_x()) })).wrapping_add(compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.search_field.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.get_width()) })), compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.search_field.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.get_y()) })).size(12, 12).build())).map(compat::WidgetRef::value); compat::write(__this, |__t| { let __s = &mut *__t; __s.previous_page_button = __v; }); }
        { let __v = false; compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.previous_page_button.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.active = __v) }); }
        let __this_w4 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        { let __v = (Some((button::builder((component::literal("→")).get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { let Some(__this_4) = __this_w4.get().upgrade() else { return Default::default(); }; compat::lend_screen(&__this_4, __screen, || { let mut __pidx = compat::WidgetRef::Screen(__index); {
            if compat::read((&__this_4), |__t| __t.next_page_slot_id) != -1 {
                PartyFinderScreen::click_and_wait_for_server((&__this_4), compat::read((&__this_4), |__t| __t.next_page_slot_id));
                { let __v = true; compat::write((&__this_4), |__t| __t.reset_scroll = __v); }
            }
        } }) })))).pos(((compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.search_field.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.get_x()) })).wrapping_add(compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.search_field.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.get_width()) }))).wrapping_add(12), compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.search_field.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.get_y()) })).size(12, 12).build())).map(compat::WidgetRef::value); compat::write(__this, |__t| { let __s = &mut *__t; __s.next_page_button = __v; }); }
        { let __v = false; compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.next_page_button.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.active = __v) }); }
        if compat::read(__this, |__t| __t.settings_container.clone()).is_none() {
            { let __v = Some(FinderSettingsContainer::new(compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.party_entry_list_widget.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.mod_widget().and_then(|__ms| __ms.with::<mcrust::mcrust::modding::widgets::ModSelectionList, _>(|__l| __l.list.get_row_left())).expect("a mod list reached from inside its own method")) }), entry_list_top_y.wrapping_sub(12), widget_height.wrapping_add(12))); compat::write(__this, |__t| __t.settings_container = __v); }
        } else {
            FinderSettingsContainer::set_rectangle(&compat::read(__this, |__t| __t.settings_container.clone()).unwrap(), (compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.party_entry_list_widget.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.mod_widget().and_then(|__ms| __ms.with::<mcrust::mcrust::modding::widgets::ModSelectionList, _>(|__l| { let __h = &__l.class::<compat::ListClass<PartyEntryListWidget>>().map(|__c| __c.object().clone()).expect("a mod list without its class"); compat::lend_list(__h, __l, || PartyEntryListWidget::get_row_width(__h)) })).expect("a mod list reached from inside its own method")) })).wrapping_sub(2), widget_height.wrapping_add(12), compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.party_entry_list_widget.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.mod_widget().and_then(|__ms| __ms.with::<mcrust::mcrust::modding::widgets::ModSelectionList, _>(|__l| __l.list.get_row_left())).expect("a mod list reached from inside its own method")) }), entry_list_top_y.wrapping_sub(12));
        }
        let mut search_button_margin: i32 = 2;
        let mut search_button_width: i32 = (((compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.party_entry_list_widget.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.mod_widget().and_then(|__ms| __ms.with::<mcrust::mcrust::modding::widgets::ModSelectionList, _>(|__l| { let __h = &__l.class::<compat::ListClass<PartyEntryListWidget>>().map(|__c| __c.object().clone()).expect("a mod list without its class"); compat::lend_list(__h, __l, || PartyEntryListWidget::get_row_width(__h)) })).expect("a mod list reached from inside its own method")) })).wrapping_add(6)).wrapping_div(3)).wrapping_sub(2i32.wrapping_mul(search_button_margin));
        let __this_w5 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        { let __v = (Some((button::builder(component::translatable("skyblocker.partyFinder.tabs.partyFinder").get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { let Some(__this_5) = __this_w5.get().upgrade() else { return Default::default(); }; compat::lend_screen(&__this_5, __screen, || { let mut __pidx = compat::WidgetRef::Screen(__index); {
            if compat::read((&__this_5), |__t| __t.party_button_slot_id) != -1 {
                PartyFinderScreen::set_current_page((&__this_5), Page::Finder);
                PartyFinderScreen::click_and_wait_for_server((&__this_5), compat::read((&__this_5), |__t| __t.party_button_slot_id));
            }
        } }) })))).pos(compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.party_entry_list_widget.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.mod_widget().and_then(|__ms| __ms.with::<mcrust::mcrust::modding::widgets::ModSelectionList, _>(|__l| __l.list.get_row_left())).expect("a mod list reached from inside its own method")) }), entry_list_top_y.wrapping_sub(39)).size(search_button_width.wrapping_add(search_button_margin), top_row_buttons_height).build())).map(compat::WidgetRef::value); compat::write(__this, |__t| { let __s = &mut *__t; __s.party_finder_button = __v; }); }
        let __this_w6 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        { let __v = (Some((button::builder(component::translatable("skyblocker.partyFinder.tabs.searchSettings").get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { let Some(__this_6) = __this_w6.get().upgrade() else { return Default::default(); }; compat::lend_screen(&__this_6, __screen, || { let mut __pidx = compat::WidgetRef::Screen(__index); {
            if compat::read((&__this_6), |__t| __t.settings_button_slot_id) != -1 {
                PartyFinderScreen::set_current_page((&__this_6), Page::Settings);
                PartyFinderScreen::click_and_wait_for_server((&__this_6), compat::read((&__this_6), |__t| __t.settings_button_slot_id));
            }
        } }) })))).pos(((compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.party_entry_list_widget.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.mod_widget().and_then(|__ms| __ms.with::<mcrust::mcrust::modding::widgets::ModSelectionList, _>(|__l| __l.list.get_row_left())).expect("a mod list reached from inside its own method")) })).wrapping_add(search_button_width)).wrapping_add(3i32.wrapping_mul(search_button_margin)), entry_list_top_y.wrapping_sub(39)).size(search_button_width, top_row_buttons_height).build())).map(compat::WidgetRef::value); compat::write(__this, |__t| { let __s = &mut *__t; __s.settings_button = __v; }); }
        let __this_w7 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        { let __v = (Some((button::builder(component::translatable("skyblocker.partyFinder.tabs.createParty").get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { let Some(__this_7) = __this_w7.get().upgrade() else { return Default::default(); }; compat::lend_screen(&__this_7, __screen, || { let mut __pidx = compat::WidgetRef::Screen(__index); {
            if compat::read((&__this_7), |__t| __t.create_party_button_slot_id) != -1 {
                PartyFinderScreen::click_and_wait_for_server((&__this_7), compat::read((&__this_7), |__t| __t.create_party_button_slot_id));
            }
        } }) })))).pos(((compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.party_entry_list_widget.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.mod_widget().and_then(|__ms| __ms.with::<mcrust::mcrust::modding::widgets::ModSelectionList, _>(|__l| __l.list.get_row_left())).expect("a mod list reached from inside its own method")) })).wrapping_add(search_button_width.wrapping_mul(2))).wrapping_add(5i32.wrapping_mul(search_button_margin)), entry_list_top_y.wrapping_sub(39)).size(search_button_width, top_row_buttons_height).build())).map(compat::WidgetRef::value); compat::write(__this, |__t| { let __s = &mut *__t; __s.create_party_button = __v; }); }
        { let __v = false; compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.create_party_button.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.active = __v) }); }
        compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.party_entry_list_widget.as_mut().unwrap(); __s.__widgets.add(&mut __s.base, __r, true) });
        compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.search_field.as_mut().unwrap(); __s.__widgets.add(&mut __s.base, __r, true) });
        compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.refresh_button.as_mut().unwrap(); __s.__widgets.add(&mut __s.base, __r, true) });
        compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.previous_page_button.as_mut().unwrap(); __s.__widgets.add(&mut __s.base, __r, true) });
        compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.next_page_button.as_mut().unwrap(); __s.__widgets.add(&mut __s.base, __r, true) });
        compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.party_finder_button.as_mut().unwrap(); __s.__widgets.add(&mut __s.base, __r, true) });
        compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.settings_button.as_mut().unwrap(); __s.__widgets.add(&mut __s.base, __r, true) });
        compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.create_party_button.as_mut().unwrap(); __s.__widgets.add(&mut __s.base, __r, true) });
        let __r0 = compat::read(__this, |__t| __t.settings_container.clone());
        { let (__q0,) = (ContainerWidgetAdapter::widget(__r0.unwrap()),); __this.lock().borrow_mut().add_renderable_widget(__q0) };
        if crate::de::hysky::skyblocker::debug::debug::Debug::debug_enabled() {
            { let (__q0,) = ((button::builder(component::null_to_empty(Some("DEBUG")).get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { compat::set_state(&DEBUG, !(compat::get_state(&DEBUG))); })))).bounds((compat::read(__this, |__t| __t.width)).wrapping_sub(40), 0, 40, 20).build(),); __this.lock().borrow_mut().add_renderable_widget(__q0) };
        }
        { let __v = compat::millis(); compat::write(__this, |__t| __t.dirtied_time = __v); }
        PartyFinderScreen::set_current_page(__this, compat::read(__this, |__t| __t.current_page.clone()).unwrap());
        if compat::read(__this, |__t| __t.current_page.clone()).unwrap() != Page::Sign {
            PartyFinderScreen::update(__this);
        }
    }

    /// java: @Override public boolean isPauseScreen()
    pub fn is_pause_screen(__this: &compat::Shared<Self>) -> bool {
        return false;
    }

    /// java: @Override public void extractRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_render_state(__this: &compat::Shared<Self>, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        if !(FinderSettingsContainer::can_interact(&compat::read(__this, |__t| __t.settings_container.clone()).unwrap(), None /* java: null */)) {
            graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), 0, 0, compat::read(__this, |__t| __t.width), compat::read(__this, |__t| __t.height), 0x40000000);
        }
        ();
        if compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.search_field.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.visible) }) {
            gui_graphics_extractor::blit_sprite_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), SEARCH_ICON_TEXTURE.get_mut(), (compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.party_entry_list_widget.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.mod_widget().and_then(|__ms| __ms.with::<mcrust::mcrust::modding::widgets::ModSelectionList, _>(|__l| __l.list.get_row_left())).expect("a mod list reached from inside its own method")) })).wrapping_add(1), (compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.search_field.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.get_y()) })).wrapping_add(1), 10, 10);
        }
        if compat::get_state(&DEBUG) {
            gui_graphics_extractor::text_string_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), Some(&compat::opt_str(&compat::read(__this, |__t| __t.current_page.clone()))), 0, 0, mcrust::net::minecraft::util::common_colors::WHITE, true);
            gui_graphics_extractor::text_string_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), Some("Truly a party finder"), 20, 20, mcrust::net::minecraft::util::common_colors::WHITE, true);
            if compat::read(__this, |__t| __t.sign.is_some()) {
                gui_graphics_extractor::text_string_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), Some("You are in a sign btw"), 20, 30, mcrust::net::minecraft::util::common_colors::WHITE, true);
            } else {
                gui_graphics_extractor::text_string_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), Some(&compat::read(__this, |__t| __t.refresh_slot_id).to_string()), (compat::read(__this, |__t| __t.width)).wrapping_sub(25), 30, mcrust::net::minecraft::util::common_colors::WHITE, true);
                gui_graphics_extractor::text_string_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), Some(&compat::read(__this, |__t| __t.prev_page_slot_id).to_string()), (compat::read(__this, |__t| __t.width)).wrapping_sub(25), 40, mcrust::net::minecraft::util::common_colors::WHITE, true);
                gui_graphics_extractor::text_string_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), Some(&compat::read(__this, |__t| __t.next_page_slot_id).to_string()), (compat::read(__this, |__t| __t.width)).wrapping_sub(25), 50, mcrust::net::minecraft::util::common_colors::WHITE, true);
                for i in 0..((__this.lock().borrow().handler.with(|__chest| __chest.menu.slots.clone()).unwrap()).len() as i32) {
                    // TODO(fabric2seer): `get_item` takes 1 argument(s) in the port (`containers: &'a Containers<'_>`), Java passes 0
                    // rust: graphics.item(((__this.lock().borrow().handler.with(|__chest| __chest.menu.slots.clone()).unwrap())[(i) as usize]).get_item(), (i.wrapping_rem(9)).wrapping_mul(16), (i.wrapping_div(9)).wrapping_mul(16));
                }
                gui_graphics_extractor::text_string_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), Some(&compat::read(__this, |__t| __t.settings_button_slot_id).to_string()), (compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.settings_button.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.get_x()) })).wrapping_add((compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.settings_button.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.get_width()) })).wrapping_div(2)), 0i32.max((compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.settings_button.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.get_y()) })).wrapping_sub(8)), mcrust::net::minecraft::util::common_colors::WHITE, true);
            }
        }
        if PartyFinderScreen::is_waiting_for_server(__this) {
            let mut s: String = "Waiting for server...".to_string();
            gui_graphics_extractor::text_string_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), Some(&s), ((compat::read(__this, |__t| __t.width)).wrapping_sub(compat::font().width(&s))).wrapping_sub(5), ((compat::read(__this, |__t| __t.height)).wrapping_sub(mcrust::net::minecraft::client::gui::font::LINE_HEIGHT)).wrapping_sub(2), mcrust::net::minecraft::util::common_colors::WHITE, true);
        }
    }

    /// java: @Override public void extractBackground(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_background(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        // TODO(fabric2seer): `Screen.extractTransparentBackground` has no Rust method in the port (looked for `extract_transparent_background` in Screen and its supertypes)
        // rust: __this.lock().borrow().extract_transparent_background(graphics);
        let mut i: i32 = ((compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.party_entry_list_widget.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.mod_widget().and_then(|__ms| __ms.with::<mcrust::mcrust::modding::widgets::ModSelectionList, _>(|__l| { let __h = &__l.class::<compat::ListClass<PartyEntryListWidget>>().map(|__c| __c.object().clone()).expect("a mod list without its class"); compat::lend_list(__h, __l, || PartyEntryListWidget::get_row_width(__h)) })).expect("a mod list reached from inside its own method")) })).wrapping_add(16)).wrapping_add(6);
        // TODO(fabric2seer): `get` is not a method of FallbackedTexture
        //   java: graphics.blitSprite(RenderPipelines.GUI_TEXTURED, BACKGROUND_TEXTURE.get(), partyEntryListWidget.getRowLeft() - 8, partyEntryListWidget.getY() - 12 - 8, i, partyEntryListWidget.getBottom() - partyEntryListWidget.getY() + 16 + 12);
    }

    /// java: @Override public void onClose()
    pub fn on_close(__this: &compat::Shared<Self>) {
        debug_assert!(mcrust::mcrust::modding::game::with_player(|_| ()).is_some());
        if compat::read(__this, |__t| __t.current_page.clone()).unwrap() != Page::Sign {
            mcrust::mcrust::modding::game::with_player(|player| { player.close_container(); });
        } else {
            let mut network_handler: Option<mcrust::net::minecraft::client::multiplayer::client_packet_listener::PlayListenerSinks> = minecraft::get_connection(&*compat::minecraft());
            if network_handler.is_some() && compat::read(__this, |__t| __t.sign.is_some()) {
                let mut original_text: Vec<String> = ({ let (__q0,) = (compat::read(__this, |__t| __t.sign_front),); compat::read(__this, |__t| __t.sign.clone()).unwrap().as_sign().expect("ClassCastException: SignBlockEntity").get_text(__q0) }).get_messages(true).iter().cloned().map(|__a0| __a0.get_string()).collect::<Vec<_>>();
                // TODO(fabric2seer): `ClientPacketListener.send` has no Rust method in the port (looked for `send` in ClientPacketListener and its supertypes)
                // rust: network_handler.as_ref().unwrap().send(&ServerboundSignUpdatePacket::new(__this.lock().borrow().sign.as_ref().unwrap().get_block_pos(), compat::read(__this, |__t| __t.sign_front), original_text[0].clone(), original_text[(1) as usize].clone(), original_text[(2) as usize].clone(), original_text[(3) as usize].clone()));
            }
        }
        mcrust::mcrust::modding::game::set_screen_null();
    }

    /// java: public void setCurrentPage(Page page)
    pub fn set_current_page(__this: &compat::Shared<Self>, mut page: Page) {
        { let __v = Some(page); compat::write(__this, |__t| __t.current_page = __v); }
        if page == Page::Finder {
            // TODO(fabric2seer): `AbstractContainerWidget.visible` is a field of the port's `AbstractContainerWidget`, which the port's `AbstractSelectionList` does not hold (no Deref, no `base` field)
            // rust: { let __v = true; compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.party_entry_list_widget.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.visible = __v) }); }
            { let __v = false; compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.party_finder_button.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.active = __v) }); }
            let __wa1 = (compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.party_finder_button.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| compat::component_of(__w.get_message_component())) })).copy().set_style(mcrust::net::minecraft::network::chat::style::Style::EMPTY.with_underlined(Some(true)));
            compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.party_finder_button.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.set_message_text(mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(__wa1.clone()))) });
            { let __v = true; compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.settings_button.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.active = __v) }); }
            let __wa2 = (compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.settings_button.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| compat::component_of(__w.get_message_component())) })).copy().set_style(mcrust::net::minecraft::network::chat::style::Style::EMPTY.with_underlined(Some(false)));
            compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.settings_button.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.set_message_text(mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(__wa2.clone()))) });
            { let __v = true; compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.create_party_button.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.active = __v) }); }
            compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.search_field.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.active = true) });
            compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.search_field.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.visible = true) });
            FinderSettingsContainer::set_visible(&compat::read(__this, |__t| __t.settings_container.clone()).unwrap(), false);
            { let __v = true; compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.refresh_button.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.visible = __v) }); }
            { let __v = true; compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.previous_page_button.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.visible = __v) }); }
            { let __v = true; compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.next_page_button.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.visible = __v) }); }
        } else if page == Page::Settings || page == Page::Sign {
            // TODO(fabric2seer): `AbstractContainerWidget.visible` is a field of the port's `AbstractContainerWidget`, which the port's `AbstractSelectionList` does not hold (no Deref, no `base` field)
            // rust: { let __v = false; compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.party_entry_list_widget.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.visible = __v) }); }
            { let __v = (page != Page::Sign); compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.party_finder_button.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.active = __v) }); }
            let __wa3 = (compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.party_finder_button.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| compat::component_of(__w.get_message_component())) })).copy().set_style(mcrust::net::minecraft::network::chat::style::Style::EMPTY.with_underlined(Some(false)));
            compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.party_finder_button.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.set_message_text(mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(__wa3.clone()))) });
            { let __v = false; compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.settings_button.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.active = __v) }); }
            let __wa4 = (compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.settings_button.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| compat::component_of(__w.get_message_component())) })).copy().set_style(mcrust::net::minecraft::network::chat::style::Style::EMPTY.with_underlined(Some(true)));
            compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.settings_button.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.set_message_text(mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(__wa4.clone()))) });
            { let __v = false; compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.create_party_button.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.active = __v) }); }
            compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.search_field.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.active = false) });
            compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.search_field.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.visible = false) });
            FinderSettingsContainer::set_visible(&compat::read(__this, |__t| __t.settings_container.clone()).unwrap(), true);
            { let __v = false; compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.refresh_button.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.visible = __v) }); }
            { let __v = false; compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.previous_page_button.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.visible = __v) }); }
            { let __v = false; compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.next_page_button.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.visible = __v) }); }
        }
    }

    /// java: public void updateHandler(ChestMenu handler, Component name)
    pub fn update_handler(__this: &compat::Shared<Self>, mut handler: compat::LiveChestMenu, mut name: MutableComponent) {
        { let __v = handler; compat::write(__this, |__t| __t.handler = __v); }
        { let __v = name.clone(); compat::write(__this, |__t| __t.name = __v); }
        PartyFinderScreen::closed_sign(__this);
        PartyFinderScreen::mark_dirty(__this);
    }

    /// java: public boolean isSignFront()
    pub fn is_sign_front(__this: &compat::Shared<Self>) -> bool {
        return compat::read(__this, |__t| __t.sign_front);
    }

    /// java: public @Nullable SignBlockEntity getSign()
    pub fn get_sign(__this: &compat::Shared<Self>) -> Option<std::sync::Arc<dyn mcrust::net::minecraft::world::level::block::entity::block_entity::BlockEntityObj>> {
        return compat::read(__this, |__t| __t.sign.clone());
    }

    /// java: public void updateSign(SignBlockEntity sign, boolean front)
    /// `sign` is the level's block entity object (a `SignBlockEntity`; see the field).
    pub fn update_sign(__this: &compat::Shared<Self>, mut sign: std::sync::Arc<dyn mcrust::net::minecraft::world::level::block::entity::block_entity::BlockEntityObj>, mut front: bool) {
        PartyFinderScreen::set_current_page(__this, Page::Sign);
        { let __v = front; compat::write(__this, |__t| __t.sign_front = __v); }
        // this.sign = sign; - the same object
        { let __v = Some(sign.clone()); compat::write(__this, |__t| __t.sign = __v); }
        { let __v = false; compat::write(__this, |__t| __t.waiting_for_server = __v); }
        if !(FinderSettingsContainer::handle_sign(&compat::read(__this, |__t| __t.settings_container.clone()).unwrap(), sign.as_sign().expect("ClassCastException: SignBlockEntity"), front)) {
            PartyFinderScreen::abort(__this);
        }
    }

    /// java: public void closedSign()
    pub fn closed_sign(__this: &compat::Shared<Self>) {
        { let __v = None; compat::write(__this, |__t| __t.sign = __v); }
    }

    /// java: public void update()
    pub fn update(__this: &compat::Shared<Self>) {
        { let __v = false; compat::write(__this, |__t| __t.dirty = __v); }
        { let __v = false; compat::write(__this, |__t| __t.waiting_for_server = __v); }
        let mut title_text: String = __this.lock().borrow().name.get_string();
        if title_text.contains("Party Finder") {
            PartyFinderScreen::check_if_kuudra(__this);
            PartyFinderScreen::update_party_finder_page(__this);
        } else {
            if compat::read(__this, |__t| __t.current_page.clone()).unwrap() != Page::Settings {
                PartyFinderScreen::set_current_page(__this, Page::Settings);
            }
            if !(FinderSettingsContainer::handle(&compat::read(__this, |__t| __t.settings_container.clone()).unwrap(), __this, &title_text)) {
                PartyFinderScreen::abort(__this);
            }
        }
    }

    /// java: private void checkIfKuudra()
    pub fn check_if_kuudra(__this: &compat::Shared<Self>) {
        if ((__this.lock().borrow().handler.with(|__chest| __chest.menu.slots.clone()).unwrap()).len() as i32) < 54 {
            return;
        }
        // TODO(fabric2seer): `get_item` takes 1 argument(s) in the port (`containers: &'a Containers<'_>`), Java passes 0
        // rust: let stack = ((__this.lock().borrow().handler.with(|__chest| __chest.menu.slots.clone()).unwrap())[(45) as usize]).get_item();
        let mut stack: mcrust::net::minecraft::world::item::item_stack::ItemStack = todo!("fabric2seer: the initialiser of `stack` (TODO above)");
        if stack.is_empty() {
            return;
        }
        compat::set_state(&IS_IN_KUUDRA_PARTY_FINDER, stack.skyblocker_get_lore_strings().into_iter().any(|s| s.contains("Kuudra")));
        if compat::get_state(&IS_IN_KUUDRA_PARTY_FINDER) {
            PartyFinderScreen::abort(__this);
        }
    }

    /// java: @SuppressWarnings("deprecation") private void updatePartyFinderPage()
    pub fn update_party_finder_page(__this: &compat::Shared<Self>) {
        { let __v = false; compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.previous_page_button.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.active = __v) }); }
        { let __v = false; compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.next_page_button.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.active = __v) }); }
        let mut parties: Vec<compat::Shared<dyn PartyEntry>> = Vec::new();
        if compat::read(__this, |__t| __t.current_page.clone()).unwrap() != Page::Finder {
            PartyFinderScreen::set_current_page(__this, Page::Finder);
        }
        // handler.slots and what each slot's getItem() answers (the port's slots read their items through the open
        // menu's containers): (slot.index, slot.getItem())
        let __handler = compat::read(__this, |__t| __t.handler);
        let __slots: Vec<(i32, mcrust::net::minecraft::world::item::item_stack::ItemStack)> = __handler.with_slots(|menu, containers| menu.slots.iter().map(|slot| (slot.index, slot.get_item(containers).duplicate())).collect()).unwrap_or_default();
        let __row_count: i32 = __handler.with(|__chest| __chest.get_row_count()).unwrap_or(0);
        if __slots.iter().any(|(_, item)| (!item.is_empty() && item.is_item(&items::items().get("bedrock")))) {
            parties.push((NoParties::new() as compat::Shared<dyn PartyEntry>));
        } else {
            for (slot_index, slot_item) in __slots.iter() {
                if *slot_index > ((__row_count.wrapping_sub(1)).wrapping_mul(9)).wrapping_sub(1) || slot_item.is_empty() {
                    continue;
                }
                let stack = slot_item;
                if stack.is_item(&items::items().get("player_head")) {
                    parties.push((PartyEntryBase::new(compat::component_of(stack.get_hover_name()), ItemUtils::get_lore(stack), __this.clone(), *slot_index) as compat::Shared<dyn PartyEntry>));
                } else if stack.is_item(&items::items().get("arrow")) && compat::component_of(stack.get_hover_name()).get_string().to_lowercase().contains("previous") {
                    { let __v = *slot_index; compat::write(__this, |__t| __t.prev_page_slot_id = __v); }
                    { let __v = true; compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.previous_page_button.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.active = __v) }); }
                } else if stack.is_item(&items::items().get("arrow")) && compat::component_of(stack.get_hover_name()).get_string().to_lowercase().contains("next") {
                    { let __v = *slot_index; compat::write(__this, |__t| __t.next_page_slot_id = __v); }
                    { let __v = true; compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.next_page_button.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.active = __v) }); }
                }
            }
        }
        let mut your_party_stack: Option<mcrust::net::minecraft::world::item::item_stack::ItemStack> = None;
        let mut de_list_slot_id: i32 = -1;
        for i in (__row_count.wrapping_sub(1)).wrapping_mul(9)..__row_count.wrapping_mul(9) {
            let (slot_index, slot_item) = &__slots[(i) as usize];
            if slot_item.is_empty() {
                continue;
            }
            if slot_item.is_item(&items::items().get("emerald_block")) {
                { let __v = *slot_index; compat::write(__this, |__t| __t.refresh_slot_id = __v); }
                { let __v = true; compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.refresh_button.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.active = __v) }); }
            } else if slot_item.is_item(&items::items().get("redstone_block")) {
                { let __v = *slot_index; compat::write(__this, |__t| __t.create_party_button_slot_id = __v); }
                { let __v = true; compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.create_party_button.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.active = __v) }); }
            } else if slot_item.is_item(&items::items().get("nether_star")) {
                { let __v = *slot_index; compat::write(__this, |__t| __t.settings_button_slot_id = __v); }
            } else if slot_item.is_item(&items::items().get("bookshelf")) {
                de_list_slot_id = *slot_index;
            } else if slot_item.is_item(&items::items().get("player_head")) {
                your_party_stack = Some(slot_item.clone());
            }
        }
        let mut player_name: String = compat::minecraft().get_user().get_name().to_string();
        // It's possible for the party to show up in the search results before it does next to the delist button.
        if your_party_stack.is_none() && de_list_slot_id != -1 {
            // parties.stream().filter(party -> party.partyLeader != null && party.partyLeader.name.getString().equals(playerName))
            //     .findFirst().map(party -> party.slotID).map(slotId -> handler.slots.get(slotId)).map(Slot::getItem).orElse(null)
            your_party_stack = parties.iter().find(|party| (compat::read(*party, |__t| __t.party_entry_base().party_leader.clone()).is_some() && compat::read(&compat::read(*party, |__t| __t.party_entry_base().party_leader.clone()).unwrap(), |__l| __l.name.get_string()) == player_name))
                .map(|party| compat::read(party, |__t| __t.party_entry_base().slot_id))
                .map(|slot_id| __slots[(slot_id) as usize].1.clone());
        }
        if let Some(your_party_stack) = &your_party_stack {
            let mut title: MutableComponent = compat::component_of(your_party_stack.get_hover_name());
            if de_list_slot_id != -1 {
                title = component::literal(&format!("{}'s party", player_name));
            }
            parties.retain(|party| !((compat::read(party, |__t| __t.party_entry_base().party_leader.clone()).is_some() && compat::read(party, |__t| __t.party_entry_base().party_leader.clone()).unwrap().lock().borrow().name.get_string() == player_name)));
            parties.push((YourParty::new(title.clone(), ItemUtils::get_lore(your_party_stack), __this.clone(), de_list_slot_id) as compat::Shared<dyn PartyEntry>));
        }
        let __wa1 = parties.clone();
        compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.party_entry_list_widget.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.mod_widget().and_then(|__ms| __ms.with::<mcrust::mcrust::modding::widgets::ModSelectionList, _>(|__l| { let __h = &__l.class::<compat::ListClass<PartyEntryListWidget>>().map(|__c| __c.object().clone()).expect("a mod list without its class"); compat::lend_list(__h, __l, || PartyEntryListWidget::set_entries(__h, __wa1.clone())) })).expect("a mod list reached from inside its own method")) });
        if compat::read(__this, |__t| __t.reset_scroll) {
            { let __v = false; compat::write(__this, |__t| __t.reset_scroll = __v); }
            compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.party_entry_list_widget.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.mod_widget().and_then(|__ms| __ms.with::<mcrust::mcrust::modding::widgets::ModSelectionList, _>(|__l| __l.list.set_scroll_amount((0) as f64))).expect("a mod list reached from inside its own method")) });
        }
    }

    /// java: public boolean isAborted()
    pub fn is_aborted(__this: &compat::Shared<Self>) -> bool {
        return compat::read(__this, |__t| __t.aborted);
    }

    /// java: public void abort()
    pub fn abort(__this: &compat::Shared<Self>) {
        if compat::read(__this, |__t| __t.current_page.clone()).unwrap() == Page::Sign {
            debug_assert!(mcrust::mcrust::modding::game::with_player(|_| ()).is_some());
            // TODO(fabric2seer): `open_text_edit_arm` takes 14 argument(s) in the port (`gfx: &mut crate::net::minecraft::client::minecraft::Minecraft, screen: &mut crate::net::minecraft::client::gui::screens::screen::Screen, screen_extra: &mut crate::net::minecraft::client::gui::gui::ScreenExtra, screen_kind_before_pause: &mut ..`), Java passes 2
            // rust: mcrust::mcrust::modding::game::with_player(|player| { local_player::open_text_edit_arm(compat::read(__this, |__t| __t.sign.clone()).as_mut().unwrap(), &mut compat::read(__this, |__t| __t.sign_front)); });
        } else {
            // TODO(fabric2seer): `set_screen` takes 14 argument(s) in the port (`gfx: &mut Minecraft, screen: &mut GuiScreen, screen_extra: &mut ScreenExtra, screen_stack: &mut Vec<HeldScreen>, screen_kind_before_pause: &mut Option<ScreenKind>, screen_is_open: &mut bool, level_screen_closed: &mut bool, frames_on_this_sc ..`), Java passes 1
            // rust: gui::set_screen(&mut ContainerScreen::new(compat::read(__this, |__t| __t.handler.clone()), compat::read(__this, |__t| __t.inventory.clone()), __this.lock().borrow().title.get_string()));
        }
        // TODO(fabric2seer): field `PERIODIC_NOTIFICATION` of a value of unknown type
        //   java: this.minecraft.gui.toastManager().addToast(new SystemToast(SystemToast.SystemToastId.PERIODIC_NOTIFICATION, Component.translatable("skyblocker.partyFinder.error.name"), Component.translatable("skyblocker.partyFinder.error.message")));
        { let __v = true; compat::write(__this, |__t| __t.aborted = __v); }
    }

    /// java: @Override public void removed()
    pub fn removed(__this: &compat::Shared<Self>) {
        if mcrust::mcrust::modding::game::with_player(|_| ()).is_none() || compat::read(__this, |__t| __t.aborted) || compat::read(__this, |__t| __t.current_page.clone()).unwrap() == Page::Sign {
            return;
        }
        // TODO(fabric2seer): `removed` takes 0 argument(s) in the port (``), Java passes 1
        // TODO(fabric2seer): the mod keeps the game's `LocalPlayer` (Java holds a reference to the live object); the port's `LocalPlayer` is not a value to copy and the port lends the mod no handle to it
        // rust: mcrust::mcrust::modding::game::with_player(|player| { { let _ = __this.lock().borrow().handler.with(|__chest| __chest.removed(player.clone())); }; });
    }

    /// java: @Override public final void tick()
    pub fn tick(__this: &compat::Shared<Self>) {
        ();
        if compat::read(__this, |__t| __t.dirty) && (compat::millis()).wrapping_sub(compat::read(__this, |__t| __t.dirtied_time)) > (60 as i64) {
            PartyFinderScreen::update(__this);
        }
        debug_assert!(mcrust::mcrust::modding::game::with_player(|_| ()).is_some());
        if !(mcrust::mcrust::modding::game::with_player(|player| player.base.base.base.is_alive()).unwrap_or(false)) || mcrust::mcrust::modding::game::with_player(|player| player.base.base.base.base.is_removed()).unwrap_or(false) && compat::read(__this, |__t| __t.current_page.clone()).unwrap() != Page::Sign {
            mcrust::mcrust::modding::game::with_player(|player| { player.close_container(); });
        }
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    pub fn mouse_clicked(__this: &compat::Shared<Self>, click: &MouseButtonEvent, mut doubled: bool) -> bool {
        if compat::read(__this, |__t| __t.settings_container.clone()).is_some() && FinderSettingsContainer::has_open_option(&compat::read(__this, |__t| __t.settings_container.clone()).unwrap()) {
            return FinderSettingsContainer::mouse_clicked(&compat::read(__this, |__t| __t.settings_container.clone()).unwrap(), click, doubled);
        }
        return false;
    }

    /// java: public void clickAndWaitForServer(int slotID)
    pub fn click_and_wait_for_server(__this: &compat::Shared<Self>, mut slot_id: i32) {
        debug_assert!(true /* java: compat::minecraft().game_mode != null: never null here */);
        // TODO(fabric2seer): `MultiPlayerGameMode.handleContainerInput` has no Rust method in the port (looked for `handle_container_input` in MultiPlayerGameMode and its supertypes)
        // rust: mcrust::mcrust::modding::game::with_player(|player| { compat::minecraft().game_mode.handle_container_input(__this.lock().borrow().handler.container_id, slot_id, ContainerUtils::get_container_click_button(0), ContainerInput::Pickup, player); });
        { let __v = true; compat::write(__this, |__t| __t.waiting_for_server = __v); }
    }

    /// java: public boolean isWaitingForServer()
    pub fn is_waiting_for_server(__this: &compat::Shared<Self>) -> bool {
        return compat::read(__this, |__t| __t.waiting_for_server);
    }

    /// java: public Minecraft getClient()
    pub fn get_client(__this: &compat::Shared<Self>) -> &'static mut mcrust::net::minecraft::client::minecraft::Minecraft {
        return compat::minecraft();
    }

}

/// PartyFinderScreen's `Screen` half (`super`'s fields and widget tree), lent by the game for each call
impl compat::ScreenHalf for PartyFinderScreen {
    fn screen_half(&mut self) -> &mut mcrust::net::minecraft::client::gui::screens::screen::Screen {
        &mut self.base
    }
    fn screen_parts(&mut self) -> (&mut mcrust::net::minecraft::client::gui::screens::screen::Screen, Option<&mut compat::WidgetCache>) {
        let __s = self;
        (&mut __s.base, Some(&mut __s.__widgets))
    }
}

/// PartyFinderScreen as the game's screen: `game::set_screen(Box::new(compat::ScreenAdapter::new(..)))`
impl ModScreen for compat::ScreenAdapter<PartyFinderScreen> {
    fn get_title(&self) -> String {
        self.title()
    }

    /// java: @Override protected void init()
    fn init(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        self.lend(screen, |__h| PartyFinderScreen::init(__h))
    }

    /// java: @Override public final void tick()
    fn tick(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        self.lend(screen, |__h| PartyFinderScreen::tick(__h))
    }

    /// java: @Override public void removed()
    fn removed(&mut self) {
        self.call(|__h| PartyFinderScreen::removed(__h))
    }

    /// java: @Override public void extractRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_render_state(&mut self, screen: &mcrust::net::minecraft::client::gui::screens::screen::Screen, context: &mut ScreenRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        self.lend_ref(screen, |__h| PartyFinderScreen::extract_render_state(__h, &mut context.graphics, mouse_x, mouse_y, a))
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
        self.lend(screen, |__h| PartyFinderScreen::mouse_clicked(__h, &MouseButtonEvent::new(x, y, MouseButtonInfo::new(button, 0)), false))
    }

    /// java: @Override public void onClose()
    fn on_close(&mut self) {
        self.call(|__h| PartyFinderScreen::on_close(__h))
    }

    /// java: @Override public boolean isPauseScreen()
    fn is_pause_screen(&self) -> bool {
        self.call(|__h| PartyFinderScreen::is_pause_screen(__h))
    }

}

// NOTE(fabric2seer): extractBackground, extractRenderState, init, isPauseScreen, mouseClicked, onClose, removed, tick override(s) of Screen - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them

/// java: public enum Page
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Page {
    Finder,
    Settings,
    Sign,
}

impl Page {
    pub const VALUES: [Page; 3] = [Page::Finder, Page::Settings, Page::Sign];

    /// java: values()
    pub fn values() -> Vec<Page> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            Page::Finder => "FINDER",
            Page::Settings => "SETTINGS",
            Page::Sign => "SIGN",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> Page {
        match name {
            "FINDER" => Page::Finder,
            "SETTINGS" => Page::Settings,
            "SIGN" => Page::Sign,
            _ => panic!("IllegalArgumentException: No enum constant Page.{name}"),
        }
    }

}

impl std::fmt::Display for Page {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
