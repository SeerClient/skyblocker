//! Ported from de.hysky.skyblocker.skyblock.dungeon.partyfinder.PartyEntry by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::dungeon::partyfinder::party_finder_screen;
use crate::de::hysky::skyblocker::skyblock::dungeon::partyfinder::party_finder_screen::PartyFinderScreen;
use mcrust::com::mojang::authlib::game_profile::GameProfile;
use mcrust::com::mojang::authlib::properties::property_map::PropertyMap;
use mcrust::java::text::decimal_format_symbols::Locale;
use mcrust::java::util::uuid::Uuid;
use mcrust::mcrust::modding::widgets::ModListEntry;
use mcrust::mcrust::modding::widgets::ModSelectionList;
use mcrust::mcrust::modding::widgets::WidgetRenderContext;
use mcrust::net::minecraft::chat_formatting;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::components::events::gui_event_listener::GuiEventListener;
use mcrust::net::minecraft::client::gui::components::player_face_extractor;
use mcrust::net::minecraft::client::gui::font::Font;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::narration::narratable_entry::NarratableEntry;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::client::resources::default_player_skin;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::network::chat::style::Style;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::world::item::component::resolvable_profile::ResolvableProfile;
use mcrust::net::minecraft::world::item::items;
use mcrust::org::joml::matrix3x2f_stack::Matrix3x2fStack;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final Identifier PARTY_CARD_TEXTURE = SkyblockerMod.id("textures/gui/party_card.png");
pub static PARTY_CARD_TEXTURE: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("textures/gui/party_card.png"));
/// java: private static final Identifier PARTY_CARD_TEXTURE_HOVER = SkyblockerMod.id("textures/gui/party_card_hover.png");
pub static PARTY_CARD_TEXTURE_HOVER: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("textures/gui/party_card_hover.png"));
/// java: private static final Map<String, ResolvableProfile> SKULL_CACHE = new Object2ObjectOpenHashMap<>();
pub static SKULL_CACHE: compat::MainThread<std::collections::HashMap<String, ResolvableProfile>> = compat::MainThread::new(|| std::collections::HashMap::new());
/// java: private static final Pattern NUMBERS_PATTERN = Pattern.compile("\\d+$");
pub static NUMBERS_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("\\d+$"));
/// java: public static final Component JOIN_TEXT = Component.translatable("skyblocker.partyFinder.join");
pub static JOIN_TEXT: compat::MainThread<MutableComponent> = compat::MainThread::new(|| component::translatable("skyblocker.partyFinder.join"));
/// java: public static final Component DE_LIST_TEXT = Component.translatable("skyblocker.partyFinder.deList");
pub static YOUR_PARTY_DE_LIST_TEXT: compat::MainThread<MutableComponent> = compat::MainThread::new(|| component::translatable("skyblocker.partyFinder.deList"));
/// java: public static final Component YOUR_PARTY_TEXT = Component.translatable("skyblocker.partyFinder.yourParty");
pub static YOUR_PARTY_YOUR_PARTY_TEXT: compat::MainThread<MutableComponent> = compat::MainThread::new(|| component::translatable("skyblocker.partyFinder.yourParty"));

/// java: public class PartyEntry extends ContainerObjectSelectionList.Entry<PartyEntry>
pub struct PartyEntryBase {
    /// java: extends net.minecraft.client.gui.components.ContainerObjectSelectionList$Entry
    pub base: mcrust::net::minecraft::client::gui::components::abstract_selection_list::Entry,
    /// java: protected final PartyFinderScreen screen;
    /// (`None` is Java's null: `NoParties` passes none)
    pub screen: Option<compat::Shared<PartyFinderScreen>>,
    /// java: protected final int slotID;
    pub slot_id: i32,
    /// java: Player partyLeader;
    pub party_leader: Option<compat::Shared<Player>>,
    /// java: String floor = "???";
    pub floor: String,
    /// java: String dungeon = "???";
    pub dungeon: String,
    /// java: String note = "";
    pub note: String,
    /// java: PropertyMap floorSkullProperties = PropertyMap.EMPTY;
    pub floor_skull_properties: PropertyMap,
    /// java: Identifier partyLeaderSkin = DefaultPlayerSkin.getDefaultTexture();
    pub party_leader_skin: Identifier,
    /// java: Player[] partyMembers = new Player[4];
    /// (`None` is an empty member slot - the array's null)
    pub party_members: Vec<Option<compat::Shared<Player>>>,
    /// java: int minClassLevel = -1;
    pub min_class_level: i32,
    /// java: int minCatacombsLevel = -1;
    pub min_catacombs_level: i32,
    /// java: boolean isLocked = false;
    pub is_locked: bool,
    /// java: Component lockReason = Component.empty();
    pub lock_reason: MutableComponent,
    /// the row's widgets (`children()`): lent to the object by its `compat::EntryAdapter`
    pub __row: mcrust::net::minecraft::client::gui::screens::screen::Screen,
    /// the screen's widgets as this object last saw them (see `compat::WidgetCache`)
    pub __widgets: compat::WidgetCache,
}

impl std::ops::Deref for PartyEntryBase {
    type Target = mcrust::net::minecraft::client::gui::components::abstract_selection_list::Entry;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for PartyEntryBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl PartyEntryBase {
    /// java: public PartyEntry(Component title, List<Component> tooltips, PartyFinderScreen screen, int slotID) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_in<__R>(mut title: MutableComponent, mut tooltips: Vec<MutableComponent>, mut screen: Option<compat::Shared<PartyFinderScreen>>, mut slot_id: i32, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn PartyEntry>, __R)) -> (compat::Shared<dyn PartyEntry>, __R) {
        let __init_screen = screen.clone();
        let __init_slot_id = slot_id;
        let this = Self {
            base: mcrust::net::minecraft::client::gui::components::abstract_selection_list::Entry::new(0),
            __widgets: Default::default(),
            __row: mcrust::net::minecraft::client::gui::screens::screen::Screen::new(String::new()),
            screen: __init_screen,
            slot_id: __init_slot_id,
            party_leader: None,
            floor: "???".to_string(),
            dungeon: "???".to_string(),
            note: "".to_string(),
            floor_skull_properties: mcrust::com::mojang::authlib::properties::property_map::PropertyMap::empty(),
            // DefaultPlayerSkin.getDefaultTexture(): the default skin's `texturePath()` (`textures/<id>.png`)
            party_leader_skin: Identifier::with_default_namespace(&format!("textures/{}.png", default_player_skin::get_default_texture())),
            party_members: vec![None, None, None, None],
            min_class_level: -1,
            min_catacombs_level: -1,
            is_locked: false,
            lock_reason: component::empty(),
        };
        let (__h, __r) = __wrap(this);
        let __this = __h;
        __this.lock().borrow_mut().party_entry_base_mut().party_members.fill(None);
        if tooltips.is_empty() {
            return (__this, __r);
        }
        let client = compat::minecraft();
        let mut party_host: String = (compat::regex::split(&title.get_string(), "'s"))[(0) as usize].clone();
        let mut members_index: i32 = -1;
        for i in 0..(tooltips.len() as i32) {
            let text = &tooltips[(i) as usize];
            let mut tooltip_text: Option<String> = Some(chat_formatting::strip_formatting(&text.get_string()));
            debug_assert!(tooltip_text.is_some());
            let mut lower_case: String = tooltip_text.as_ref().unwrap().to_lowercase();
            if lower_case.contains("members:") && members_index == -1 {
                members_index = i.wrapping_add(1);
            } else if lower_case.contains("class level") {
                let mut matcher: compat::regex::Matcher = NUMBERS_PATTERN.matcher(&lower_case);
                if matcher.find() {
                    { let __v = matcher.group(0).unwrap().parse::<i32>().expect("NumberFormatException"); compat::write(&__this, |__t| __t.party_entry_base_mut().min_class_level = __v); }
                }
            } else if lower_case.contains("dungeon level") {
                let mut matcher: compat::regex::Matcher = NUMBERS_PATTERN.matcher(&lower_case);
                if matcher.find() {
                    { let __v = matcher.group(0).unwrap().parse::<i32>().expect("NumberFormatException"); compat::write(&__this, |__t| __t.party_entry_base_mut().min_catacombs_level = __v); }
                }
            } else if lower_case.contains("floor:") {
                { let __v = compat::trim(&((compat::regex::split(&tooltip_text.as_ref().unwrap(), ":"))[(1) as usize])); compat::write(&__this, |__t| __t.party_entry_base_mut().floor = __v); }
                if compat::read(&__this, |__t| __t.party_entry_base().dungeon.clone()) == "???" {
                    continue;
                }
                if party_finder_screen::FLOOR_ICONS_MASTER.get_mut().is_none() || party_finder_screen::FLOOR_ICONS_NORMAL.get_mut().is_none() {
                    continue;
                }
                if __this.lock().borrow().party_entry_base().dungeon.contains("Master Mode") {
                    // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
                    let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        { let __v = party_finder_screen::FLOOR_ICONS_MASTER.get_mut().as_ref().unwrap().get(&__this.lock().borrow().party_entry_base().floor.to_lowercase()).cloned().unwrap_or_else(|| mcrust::com::mojang::authlib::properties::property_map::PropertyMap::empty()); compat::write(&__this, |__t| __t.party_entry_base_mut().floor_skull_properties = __v); }
                    }));
                    if let Err(e) = __result {
                        panic!("RuntimeException: {}", compat::panic_message(&e));
                    }
                } else {
                    // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
                    let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        { let __v = party_finder_screen::FLOOR_ICONS_NORMAL.get_mut().as_ref().unwrap().get(&__this.lock().borrow().party_entry_base().floor.to_lowercase()).cloned().unwrap_or_else(|| mcrust::com::mojang::authlib::properties::property_map::PropertyMap::empty()); compat::write(&__this, |__t| __t.party_entry_base_mut().floor_skull_properties = __v); }
                    }));
                    if let Err(e) = __result {
                        panic!("RuntimeException: {}", compat::panic_message(&e));
                    }
                }
            } else if lower_case.contains("dungeon:") {
                { let __v = compat::trim(&((compat::regex::split(&tooltip_text.as_ref().unwrap(), ":"))[(1) as usize])); compat::write(&__this, |__t| __t.party_entry_base_mut().dungeon = __v); }
            } else if !(text.get_siblings().is_empty()) && text.get_siblings()[0].get_style().get_color().unwrap() == mcrust::net::minecraft::network::chat::text_color::TextColor::RED && !(lower_case.starts_with(" ")) {
                { let __v = true; compat::write(&__this, |__t| __t.party_entry_base_mut().is_locked = __v); }
                { let __v = text.clone(); compat::write(&__this, |__t| __t.party_entry_base_mut().lock_reason = __v); }
            } else if lower_case.contains("note:") {
                let mut split: Vec<String> = compat::regex::split_limit(&tooltip_text.as_ref().unwrap(), ":", 2);
                if (split.len() as i32) == 2 {
                    { let __v = compat::trim(&(split[(1) as usize])); compat::write(&__this, |__t| __t.party_entry_base_mut().note = __v); }
                } else {
                    { let __v = "???".to_string(); compat::write(&__this, |__t| __t.party_entry_base_mut().note = __v); }
                }
            }
        }
        if members_index != -1 {
            {
                let mut i: i32 = members_index;
                let mut j: i32 = 0;
                while i < members_index.wrapping_add(5) {
                    'continue_2: {
                        if i >= (tooltips.len() as i32) {
                            break 'continue_2;
                        }
                        let text = &tooltips[(i) as usize];
                        let mut member_text: String = text.get_string();
                        if !(member_text.starts_with(" ")) {
                            break 'continue_2;
                        }
                        let mut parts: Vec<String> = compat::regex::split_limit(&member_text, ":", 2);
                        if (parts.len() as i32) != 2 {
                            break 'continue_2;
                        }
                        let mut player_name_trim: String = compat::trim(&(parts[(0) as usize]));
                        if player_name_trim == "Empty" {
                            break 'continue_2;
                        }
                        let siblings = text.get_siblings();
                        let mut name_style: Style = (if !siblings.is_empty() { siblings[0].get_style().clone() } else { text.get_style().clone() });
                        let mut player_name: MutableComponent = component::literal(&player_name_trim).set_style(name_style.clone());
                        let mut class_name: String = (compat::regex::split(&(compat::trim(&(parts[(1) as usize]))), " "))[(0) as usize].clone();
                        let mut class_level: i32 = -1;
                        // PERF (port-only, same result): compiled once, not two regexes per party member.
                        static CLASS_LEVEL_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("\\((\\d+)\\)"));
                        let mut matcher: compat::regex::Matcher = CLASS_LEVEL_PATTERN.matcher(&parts[(1) as usize]);
                        if matcher.find() {
                            class_level = matcher.group(1).unwrap().parse::<i32>().expect("NumberFormatException");
                        }
                        let mut player: compat::Shared<Player> = Player::new(player_name.clone(), class_name.clone(), class_level);
                        // java: client.playerSkinRenderCache().lookup(ResolvableProfile.createUnresolved(playerNameTrim)).thenAccept(
                        //           gameProfile -> gameProfile.ifPresent(entry -> player.skinTexture = entry.playerSkin().body().texturePath()));
                        {
                            let __player = compat::RenderThread::new(player.clone());
                            lookup_skin_texture(&player_name_trim, move |texture| { compat::write(__player.get(), |__t| __t.skin_texture = texture); });
                        }
                        if player_name_trim == party_host {
                            { let __v = Some(player.clone()); compat::write(&__this, |__t| __t.party_entry_base_mut().party_leader = __v); }
                            j -= 1;
                        } else if j > 3 {
                            { let __v = Some(player.clone()); compat::write(&__this, |__t| __t.party_entry_base_mut().party_leader = __v); }
                        } else {
                            __this.lock().borrow_mut().party_entry_base_mut().party_members[(j) as usize] = Some(player.clone());
                        }
                    }
                    i += 1;
                    j += 1;
                }
            }
        }
        if compat::read(&__this, |__t| __t.party_entry_base().party_leader.clone()).is_none() {
            {
                let mut i: i32 = (__this.lock().borrow().party_entry_base().party_members.len() as i32).wrapping_sub(1);
                while i >= 0 {
                    if __this.lock().borrow().party_entry_base().party_members[(i) as usize].is_some() {
                        { let __v = __this.lock().borrow().party_entry_base().party_members[(i) as usize].clone(); compat::write(&__this, |__t| __t.party_entry_base_mut().party_leader = __v); }
                        __this.lock().borrow_mut().party_entry_base_mut().party_members[(i) as usize] = None;
                        break;
                    }
                    i -= 1;
                }
            }
        }
        if compat::read(&__this, |__t| __t.party_entry_base().party_leader.clone()).is_none() {
            { let __v = Some(Player::new(component::literal("Error"), "Error".to_string(), -1)); compat::write(&__this, |__t| __t.party_entry_base_mut().party_leader = __v); }
        }
        // java: client.playerSkinRenderCache().lookup(ResolvableProfile.createUnresolved(partyLeader.name.getString())).thenAccept(
        //           gameProfile -> gameProfile.ifPresent(entry -> partyLeaderSkin = entry.playerSkin().body().texturePath()));
        {
            let __leader_name: String = compat::read(&__this, |__t| __t.party_entry_base().party_leader.clone()).map(|leader| compat::read(&leader, |__l| __l.name.get_string())).unwrap_or_default();
            let __entry = compat::RenderThread::new(__this.clone());
            lookup_skin_texture(&__leader_name, move |texture| { compat::write(__entry.get(), |__t| __t.party_entry_base_mut().party_leader_skin = texture); });
        }
        (__this, __r)
    }

    /// java: public PartyEntry(Component title, List<Component> tooltips, PartyFinderScreen screen, int slotID)
    pub fn new(mut title: MutableComponent, mut tooltips: Vec<MutableComponent>, mut screen: compat::Shared<PartyFinderScreen>, mut slot_id: i32) -> compat::Shared<Self> {
        Self::new_in(title, tooltips, Some(screen), slot_id, |this| {
            let __h = compat::shared(this);
            ((__h.clone() as compat::Shared<dyn PartyEntry>), __h)
        }).1
    }

}

/// java: the virtual methods of PartyEntry (implemented by PartyEntryBase and every subclass)
pub trait PartyEntry: compat::DynAny {
    fn party_entry_base(&self) -> &PartyEntryBase;
    fn party_entry_base_mut(&mut self) -> &mut PartyEntryBase;
    fn as_any(&self) -> &dyn std::any::Any;

    /// java: public boolean isLocked() (dispatch: the body for this class)
    fn is_locked(&self) -> fn(&compat::Shared<dyn PartyEntry>) -> bool {
        <dyn PartyEntry>::is_locked_body
    }

    /// java: @Override public List<? extends NarratableEntry> narratables() (dispatch: the body for this class)
    /// (the list's element type is `NarrationSupplier`, `NarratableEntry`'s supertrait: the port's `NarratableEntry`
    /// is not usable as a trait object - as `utils/render/gui/dropdown_widget.rs` types it)
    fn narratables(&self) -> fn(&compat::Shared<dyn PartyEntry>) -> Vec<Box<dyn mcrust::net::minecraft::client::gui::narration::narration_supplier::NarrationSupplier>> {
        <dyn PartyEntry>::narratables_body
    }

    /// java: @Override public List<? extends GuiEventListener> children() (dispatch: the body for this class)
    fn children(&self) -> fn(&compat::Shared<dyn PartyEntry>) -> Vec<Box<dyn GuiEventListener>> {
        <dyn PartyEntry>::children_body
    }

    /// java: @Override public void extractContent(GuiGraphicsExtractor graphics, int mouseX, int mouseY, boolean hovered, float a) (dispatch: the body for this class)
    fn extract_content(&self) -> fn(&compat::Shared<dyn PartyEntry>, &mut GuiGraphicsExtractor, i32, i32, bool, f32) {
        <dyn PartyEntry>::extract_content_body
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled) (dispatch: the body for this class)
    fn mouse_clicked(&self) -> fn(&compat::Shared<dyn PartyEntry>, &MouseButtonEvent, bool) -> bool {
        <dyn PartyEntry>::mouse_clicked_body
    }

}

impl dyn PartyEntry {
    /// java: public boolean isLocked()
    pub fn is_locked_body(__this: &compat::Shared<dyn PartyEntry>) -> bool {
        return compat::read(__this, |__t| __t.party_entry_base().is_locked);
    }

    /// java: @Override public List<? extends NarratableEntry> narratables()
    pub fn narratables_body(__this: &compat::Shared<dyn PartyEntry>) -> Vec<Box<dyn mcrust::net::minecraft::client::gui::narration::narration_supplier::NarrationSupplier>> {
        return vec![];
    }

    /// java: @Override public List<? extends GuiEventListener> children()
    pub fn children_body(__this: &compat::Shared<dyn PartyEntry>) -> Vec<Box<dyn GuiEventListener>> {
        return vec![];
    }

    /// java: @Override public void extractContent(GuiGraphicsExtractor graphics, int mouseX, int mouseY, boolean hovered, float a)
    pub fn extract_content_body(__this: &compat::Shared<dyn PartyEntry>, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut hovered: bool, mut a: f32) {
        let mut x: i32 = __this.lock().borrow().party_entry_base().get_x();
        let mut y: i32 = __this.lock().borrow().party_entry_base().get_y();
        let mut entry_width: i32 = __this.lock().borrow().party_entry_base().get_width();
        let mut entry_height: i32 = __this.lock().borrow().party_entry_base().get_height();
        let __pose_1 = gui_graphics_extractor::push_pose();
        gui_graphics_extractor::pose_translate((x) as f32, (y) as f32);
        let mut text_renderer: Font = compat::font();
        if hovered && !(compat::read(__this, |__t| __t.party_entry_base().is_locked)) {
            gui_graphics_extractor::blit_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), PARTY_CARD_TEXTURE_HOVER.get_mut(), 0, 0, ((0) as f32) as f32, ((0) as f32) as f32, 336, 64, 336, 64);
            if !(__this.lock().borrow().as_any().is::<compat::Shared<YourParty>>()) {
                gui_graphics_extractor::draw_formatted_text_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(JOIN_TEXT.get_mut().clone()), 148, 6, mcrust::net::minecraft::util::common_colors::WHITE, false);
            }
        } else {
            gui_graphics_extractor::blit_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), PARTY_CARD_TEXTURE.get_mut(), 0, 0, ((0) as f32) as f32, ((0) as f32) as f32, 336, 64, 336, 64);
        }
        let mut mouse_x_local: i32 = mouse_x.wrapping_sub(x);
        let mut mouse_y_local: i32 = mouse_y.wrapping_sub(y);
        gui_graphics_extractor::draw_formatted_text_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(Player::to_text(&compat::read(__this, |__t| __t.party_entry_base().party_leader.clone()).unwrap())), 18, 6, mcrust::net::minecraft::util::common_colors::WHITE, true);
        if compat::get_state(&party_finder_screen::DEBUG) {
            gui_graphics_extractor::text_string_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), Some(&compat::read(__this, |__t| __t.party_entry_base().slot_id).to_string()), 166, 6, mcrust::net::minecraft::util::common_colors::WHITE, true);
            if hovered {
                gui_graphics_extractor::text_string_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), Some("H"), 160, 6, mcrust::net::minecraft::util::common_colors::WHITE, true);
            }
        }
        // PlayerFaceExtractor.extractRenderState(graphics, partyLeaderSkin, ..): the port's takes the texture the id names
        { let __client = compat::minecraft(); let __skin = compat::read(__this, |__t| __t.party_entry_base().party_leader_skin.clone()); if let Some(__texture) = __client.game_renderer.texture_manager().get_texture_or_request(&__skin) { player_face_extractor::extract_render_state(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*__client, __texture, 6, 6, 8, true, false, -1); } }
        for i in { 0..(__this.lock().borrow().party_entry_base().party_members.len() as i32) } {
            let mut party_member: Option<compat::Shared<Player>> = __this.lock().borrow().party_entry_base().party_members[(i) as usize].clone();
            let Some(mut party_member) = party_member.clone() else {
                continue;
            };
            gui_graphics_extractor::draw_formatted_text(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), 17i32.wrapping_add(136i32.wrapping_mul(i.wrapping_rem(2))), 24i32.wrapping_add(14i32.wrapping_mul(i.wrapping_div(2))), &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(Player::to_text(&party_member)), mcrust::net::minecraft::util::common_colors::WHITE);
            { let __client = compat::minecraft(); let __skin = compat::read(&party_member, |__t| __t.skin_texture.clone()); if let Some(__texture) = __client.game_renderer.texture_manager().get_texture_or_request(&__skin) { player_face_extractor::extract_render_state(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*__client, __texture, 6i32.wrapping_add(136i32.wrapping_mul(i.wrapping_rem(2))), 24i32.wrapping_add(14i32.wrapping_mul(i.wrapping_div(2))), 8, true, false, -1); } }
        }
        if compat::read(__this, |__t| __t.party_entry_base().min_class_level) > 0 {
            gui_graphics_extractor::draw_formatted_text(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), 278, 25, &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::null_to_empty(Some(&format!("Class {}", compat::read(__this, |__t| __t.party_entry_base().min_class_level))))), mcrust::net::minecraft::util::common_colors::WHITE);
            if !(compat::read(__this, |__t| __t.party_entry_base().is_locked)) && hovered && mouse_x_local >= 276 && mouse_x_local <= 331 && mouse_y_local >= 22 && mouse_y_local <= 35 {
                // java: graphics.setTooltipForNextFrame(textRenderer, Component.translatable("skyblocker.partyFinder.partyCard.minClassLevel", minClassLevel), mouseX, mouseY);
                compat::batch_s_set_tooltip_for_next_frame(vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::translatable_args("skyblocker.partyFinder.partyCard.minClassLevel", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of(compat::read(__this, |__t| __t.party_entry_base().min_class_level).to_string())]))]);
            }
        }
        if compat::read(__this, |__t| __t.party_entry_base().min_catacombs_level) > 0 {
            gui_graphics_extractor::draw_formatted_text(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), 278, 43, &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::null_to_empty(Some(&format!("Cata {}", compat::read(__this, |__t| __t.party_entry_base().min_catacombs_level))))), mcrust::net::minecraft::util::common_colors::WHITE);
            if !(compat::read(__this, |__t| __t.party_entry_base().is_locked)) && hovered && mouse_x_local >= 276 && mouse_x_local <= 331 && mouse_y_local >= 40 && mouse_y_local <= 53 {
                // java: graphics.setTooltipForNextFrame(textRenderer, Component.translatable("skyblocker.partyFinder.partyCard.minDungeonLevel", minCatacombsLevel), mouseX, mouseY);
                compat::batch_s_set_tooltip_for_next_frame(vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::translatable_args("skyblocker.partyFinder.partyCard.minDungeonLevel", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of(compat::read(__this, |__t| __t.party_entry_base().min_catacombs_level).to_string())]))]);
            }
        }
        let mut stack: mcrust::net::minecraft::world::item::item_stack::ItemStack = mcrust::net::minecraft::world::item::item_stack::ItemStack::of_one(items::items().get("player_head"));
        // stack.set(DataComponents.PROFILE, SKULL_CACHE.computeIfAbsent("SkyblockerCustomPFSkull" + dungeon + floor,
        //     name -> ResolvableProfile.createResolved(new GameProfile(UUID.randomUUID(), name, floorSkullProperties))));
        let __floor_skull_properties = compat::read(__this, |__t| __t.party_entry_base().floor_skull_properties.clone());
        let __key = format!("SkyblockerCustomPFSkull{}{}", compat::read(__this, |__t| __t.party_entry_base().dungeon.clone()), compat::read(__this, |__t| __t.party_entry_base().floor.clone()));
        let __profile: ResolvableProfile = SKULL_CACHE.get_mut().entry(__key).or_insert_with_key(|name| ResolvableProfile::create_resolved(GameProfile::new(Uuid::random_uuid(), name, __floor_skull_properties.clone())).expect("createResolved")).clone();
        stack.set(mcrust::net::minecraft::core::component::data_components::profile().clone(), Some(mcrust::com::mojang::serialization::obj(__profile)));
        graphics.item(&stack, 317, 3);
        let mut text_width: i32 = text_renderer.width(&compat::read(__this, |__t| __t.party_entry_base().floor.clone()));
        gui_graphics_extractor::text_string_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), Some(&compat::read(__this, |__t| __t.party_entry_base().floor.clone())), 314i32.wrapping_sub(text_width), 7, 0xA0000000u32 as i32, false);
        gui_graphics_extractor::text_string_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), Some(&compat::read(__this, |__t| __t.party_entry_base().note.clone())), 5, 52, mcrust::net::minecraft::util::common_colors::WHITE, true);
        if compat::read(__this, |__t| __t.party_entry_base().is_locked) {
            graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), 0, 0, entry_width, entry_height, 0x90000000u32 as i32);
            let __pose_2 = gui_graphics_extractor::push_pose();
            gui_graphics_extractor::pose_translate(((((entry_width) as f32) / (2 as f32))) as f32, ((((entry_height) as f32) / (2 as f32))) as f32);
            let mut lock_width: i32 = (text_renderer.width_of_cached(&mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(compat::read(__this, |__t| __t.party_entry_base().lock_reason.clone())))).wrapping_add(6);
            let mut text_height: i32 = mcrust::net::minecraft::client::gui::font::LINE_HEIGHT;
            graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), (-lock_width).wrapping_div(2), -2, lock_width.wrapping_div(2), text_height, 0x7F000000);
            gui_graphics_extractor::centered_text(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(compat::read(__this, |__t| __t.party_entry_base().lock_reason.clone())), 0, 0, mcrust::net::minecraft::util::common_colors::SOFT_RED);
            gui_graphics_extractor::pop_pose(__pose_2);
        }
        gui_graphics_extractor::pop_pose(__pose_1);
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    pub fn mouse_clicked_body(__this: &compat::Shared<dyn PartyEntry>, click: &MouseButtonEvent, mut doubled: bool) -> bool {
        if compat::read(__this, |__t| __t.party_entry_base().slot_id) == -1 {
            logger().error(&format!("[Skyblocker] Slot ID is null for {}'s party", compat::read(__this, |__t| __t.party_entry_base().party_leader.clone()).unwrap().lock().borrow().name.get_string()));
        }
        if click.button() == 1 && !(PartyFinderScreen::is_waiting_for_server(&compat::read(__this, |__t| __t.party_entry_base().screen.clone()).expect("NullPointerException"))) && compat::read(__this, |__t| __t.party_entry_base().slot_id) != -1 {
            PartyFinderScreen::click_and_wait_for_server(&compat::read(__this, |__t| __t.party_entry_base().screen.clone()).expect("NullPointerException"), compat::read(__this, |__t| __t.party_entry_base().slot_id));
            return true;
        }
        // java: return super.mouseClicked(click, doubled); - `ContainerObjectSelectionList.Entry.mouseClicked` is
        // `ContainerEventHandler.mouseClicked` over `children()`, which this class answers with `List.of()`: no child is
        // under the cursor, so `false`
        let _ = doubled;
        return false;
    }

}

impl PartyEntry for PartyEntryBase {
    fn party_entry_base(&self) -> &PartyEntryBase {
        self
    }
    fn party_entry_base_mut(&mut self) -> &mut PartyEntryBase {
        self
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

}

/// PartyEntry's row box and its widgets, lent by its `compat::EntryAdapter` for each call
impl compat::RowHalf for dyn PartyEntry {
    fn entry_half(&mut self) -> &mut mcrust::net::minecraft::client::gui::components::abstract_selection_list::Entry {
        &mut self.party_entry_base_mut().base
    }
    fn row_parts(&mut self) -> (&mut mcrust::net::minecraft::client::gui::screens::screen::Screen, &mut compat::WidgetCache) {
        let __s = self.party_entry_base_mut();
        (&mut __s.__row, &mut __s.__widgets)
    }
}

/// PartyEntry as a row of the game's list (`addEntry` attaches it)
impl ModListEntry for compat::EntryAdapter<dyn PartyEntry> {
    /// java: @Override public void extractContent(GuiGraphicsExtractor graphics, int mouseX, int mouseY, boolean hovered, float a)
    fn extract_content(&mut self, list: &mut ModSelectionList, index: usize, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, hovered: bool, a: f32) {
        // `extractContent` is virtual: the row's own class answers (`PartyEntry`, `NoParties`, `YourParty`)
        self.lend(Some(list.list.entry(index)), |__h| { let __f = compat::dispatch(__h, |__t| __t.extract_content()); __f(__h, &mut context.graphics, mouse_x, mouse_y, hovered, a) });
        // the row's widgets (Java's `child.extractRenderState(..)` over `children()`)
        context.extract_widgets(&mut self.row, mouse_x, mouse_y);
    }

    fn children(&mut self) -> Option<&mut mcrust::net::minecraft::client::gui::screens::screen::Screen> {
        Some(&mut self.row)
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    fn mouse_clicked(&mut self, list: &mut ModSelectionList, index: usize, event: &MouseButtonEvent, double_click: bool) -> bool {
        // `mouseClicked` is virtual: the row's own class answers (`PartyEntry`, `NoParties`, `YourParty`)
        self.lend(Some(list.list.entry(index)), |__h| { let __f = compat::dispatch(__h, |__t| __t.mouse_clicked()); __f(__h, event, double_click) })
    }

}

/// java: public static class Player
#[derive(Clone)]
pub struct Player {
    /// java: public final Component name;
    pub name: MutableComponent,
    /// java: public final String dungeonClass;
    pub dungeon_class: String,
    /// java: public final int classLevel;
    pub class_level: i32,
    /// java: public Identifier skinTexture = DefaultPlayerSkin.getDefaultTexture();
    pub skin_texture: Identifier,
}

impl Player {
    /// java: Player(Component name, String dungeonClass, int classLevel)
    pub fn new(mut name: MutableComponent, mut dungeon_class: String, mut class_level: i32) -> compat::Shared<Self> {
        let __init_name = name.clone();
        let __init_dungeon_class = dungeon_class.clone();
        let __init_class_level = class_level;
        let __this = compat::shared(Self {
            name: __init_name,
            dungeon_class: __init_dungeon_class,
            class_level: __init_class_level,
            // DefaultPlayerSkin.getDefaultTexture(): the default skin's `texturePath()` (`textures/<id>.png`)
            skin_texture: Identifier::with_default_namespace(&format!("textures/{}.png", default_player_skin::get_default_texture())),
        });
        __this
    }

    /// java: public Component toText()
    pub fn to_text(__this: &compat::Shared<Self>) -> MutableComponent {
        let mut d_class: char = (if __this.lock().borrow().dungeon_class.is_empty() { '?' } else { compat::char_at(&__this.lock().borrow().dungeon_class, 0) });
        return { let (__q0,) = ((component::literal(&format!(" {} {}", d_class, compat::read(__this, |__t| __t.class_level)))).with_style_format(ChatFormatting::Yellow),); __this.lock().borrow().name.copy().append(__q0) };
    }

}

/// java: public static class NoParties extends PartyEntry
pub struct NoParties {
    /// java: extends PartyEntry
    pub base: PartyEntryBase,
}

impl std::ops::Deref for NoParties {
    type Target = PartyEntryBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for NoParties {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl NoParties {
    /// java: public NoParties()
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = PartyEntryBase::new_in(component::empty(), vec![], None, -1, move |__base| {
            let __h = compat::shared(Self {
                base: __base,
            });
            ((__h.clone() as compat::Shared<dyn PartyEntry>), __h)
        });
        __this
    }

}

impl PartyEntry for NoParties {
    fn party_entry_base(&self) -> &PartyEntryBase {
        &self.base
    }
    fn party_entry_base_mut(&mut self) -> &mut PartyEntryBase {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled) (dispatch: the body for this class)
    fn mouse_clicked(&self) -> fn(&compat::Shared<dyn PartyEntry>, &MouseButtonEvent, bool) -> bool {
        NoParties::mouse_clicked_dyn
    }

    /// java: @Override public void extractContent(GuiGraphicsExtractor graphics, int mouseX, int mouseY, boolean hovered, float a) (dispatch: the body for this class)
    fn extract_content(&self) -> fn(&compat::Shared<dyn PartyEntry>, &mut GuiGraphicsExtractor, i32, i32, bool, f32) {
        NoParties::extract_content_dyn
    }

}

impl NoParties {
    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    pub fn mouse_clicked_impl(__this: &compat::Shared<Self>, click: &MouseButtonEvent, mut doubled: bool) -> bool {
        return false;
    }

    /// NoParties.mouseClicked as the dispatch of PartyEntry.mouseClicked
    pub fn mouse_clicked_dyn(__this: &compat::Shared<dyn PartyEntry>, __p0: &MouseButtonEvent, __p1: bool) -> bool {
        Self::mouse_clicked_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1)
    }

    /// java: @Override public void extractContent(GuiGraphicsExtractor graphics, int mouseX, int mouseY, boolean hovered, float a)
    pub fn extract_content_impl(__this: &compat::Shared<Self>, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut hovered: bool, mut a: f32) {
        let mut text_renderer: Font = compat::font();
        gui_graphics_extractor::centered_text(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::translatable("skyblocker.partyFinder.noParties")), (__this.lock().borrow().get_x()).wrapping_add((__this.lock().borrow().get_width()).wrapping_div(2)), ((__this.lock().borrow().get_y()).wrapping_add((__this.lock().borrow().get_height()).wrapping_div(2))).wrapping_sub((mcrust::net::minecraft::client::gui::font::LINE_HEIGHT).wrapping_div(2)), mcrust::net::minecraft::util::common_colors::WHITE);
    }

    /// NoParties.extractContent as the dispatch of PartyEntry.extractContent
    pub fn extract_content_dyn(__this: &compat::Shared<dyn PartyEntry>, __p0: &mut GuiGraphicsExtractor, __p1: i32, __p2: i32, __p3: bool, __p4: f32) {
        Self::extract_content_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2, __p3, __p4)
    }

}

/// NoParties's row box and its widgets, lent by its `compat::EntryAdapter` for each call
impl compat::RowHalf for NoParties {
    fn entry_half(&mut self) -> &mut mcrust::net::minecraft::client::gui::components::abstract_selection_list::Entry {
        &mut self.base.base
    }
    fn row_parts(&mut self) -> (&mut mcrust::net::minecraft::client::gui::screens::screen::Screen, &mut compat::WidgetCache) {
        let __s = &mut self.base;
        (&mut __s.__row, &mut __s.__widgets)
    }
}

/// NoParties as a row of the game's list (`addEntry` attaches it)
impl ModListEntry for compat::EntryAdapter<NoParties> {
    /// java: @Override public void extractContent(GuiGraphicsExtractor graphics, int mouseX, int mouseY, boolean hovered, float a)
    fn extract_content(&mut self, list: &mut ModSelectionList, index: usize, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, hovered: bool, a: f32) {
        self.lend(Some(list.list.entry(index)), |__h| ({ let __f = compat::dispatch(__h, |__t| __t.extract_content()); __f(&compat::view(__h, |a| a as compat::Shared<dyn PartyEntry>), &mut context.graphics, mouse_x, mouse_y, hovered, a) }));
        // the row's widgets (Java's `child.extractRenderState(..)` over `children()`)
        context.extract_widgets(&mut self.row, mouse_x, mouse_y);
    }

    fn children(&mut self) -> Option<&mut mcrust::net::minecraft::client::gui::screens::screen::Screen> {
        Some(&mut self.row)
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    fn mouse_clicked(&mut self, list: &mut ModSelectionList, index: usize, event: &MouseButtonEvent, double_click: bool) -> bool {
        self.lend(Some(list.list.entry(index)), |__h| ({ let __f = compat::dispatch(__h, |__t| __t.mouse_clicked()); __f(&compat::view(__h, |a| a as compat::Shared<dyn PartyEntry>), event, double_click) }))
    }

}

/// java: public static class YourParty extends PartyEntry
pub struct YourParty {
    /// java: extends PartyEntry
    pub base: PartyEntryBase,
}

impl std::ops::Deref for YourParty {
    type Target = PartyEntryBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for YourParty {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl YourParty {
    /// java: public YourParty(Component title, List<Component> tooltips, PartyFinderScreen screen, int deListSlotId)
    pub fn new(mut title: MutableComponent, mut tooltips: Vec<MutableComponent>, mut screen: compat::Shared<PartyFinderScreen>, mut de_list_slot_id: i32) -> compat::Shared<Self> {
        let (_, __this) = PartyEntryBase::new_in(title.clone(), tooltips.clone(), Some(screen.clone()), de_list_slot_id, move |__base| {
            let __h = compat::shared(Self {
                base: __base,
            });
            ((__h.clone() as compat::Shared<dyn PartyEntry>), __h)
        });
        __this
    }

}

impl PartyEntry for YourParty {
    fn party_entry_base(&self) -> &PartyEntryBase {
        &self.base
    }
    fn party_entry_base_mut(&mut self) -> &mut PartyEntryBase {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void extractContent(GuiGraphicsExtractor graphics, int mouseX, int mouseY, boolean hovered, float a) (dispatch: the body for this class)
    fn extract_content(&self) -> fn(&compat::Shared<dyn PartyEntry>, &mut GuiGraphicsExtractor, i32, i32, bool, f32) {
        YourParty::extract_content_dyn
    }

}

impl YourParty {
    /// java: @Override public void extractContent(GuiGraphicsExtractor graphics, int mouseX, int mouseY, boolean hovered, float a)
    pub fn extract_content_impl(__this: &compat::Shared<Self>, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut hovered: bool, mut a: f32) {
        <dyn PartyEntry>::extract_content_body(&compat::view(__this, |a| a as compat::Shared<dyn PartyEntry>), graphics, mouse_x, mouse_y, hovered, a);
        let __pose_1 = gui_graphics_extractor::push_pose();
        gui_graphics_extractor::pose_translate((__this.lock().borrow().get_x()) as f32, (__this.lock().borrow().get_y()) as f32);
        hovered = (hovered & (compat::read(__this, |__t| __t.slot_id) != -1));
        let mut text_renderer: Font = compat::font();
        gui_graphics_extractor::draw_formatted_text_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from((if hovered { YOUR_PARTY_DE_LIST_TEXT.get_mut().clone() } else { YOUR_PARTY_YOUR_PARTY_TEXT.get_mut().clone() })), 148, 6, mcrust::net::minecraft::util::common_colors::WHITE, false);
        gui_graphics_extractor::pop_pose(__pose_1);
    }

    /// YourParty.extractContent as the dispatch of PartyEntry.extractContent
    pub fn extract_content_dyn(__this: &compat::Shared<dyn PartyEntry>, __p0: &mut GuiGraphicsExtractor, __p1: i32, __p2: i32, __p3: bool, __p4: f32) {
        Self::extract_content_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2, __p3, __p4)
    }

}

/// YourParty's row box and its widgets, lent by its `compat::EntryAdapter` for each call
impl compat::RowHalf for YourParty {
    fn entry_half(&mut self) -> &mut mcrust::net::minecraft::client::gui::components::abstract_selection_list::Entry {
        &mut self.base.base
    }
    fn row_parts(&mut self) -> (&mut mcrust::net::minecraft::client::gui::screens::screen::Screen, &mut compat::WidgetCache) {
        let __s = &mut self.base;
        (&mut __s.__row, &mut __s.__widgets)
    }
}

/// YourParty as a row of the game's list (`addEntry` attaches it)
impl ModListEntry for compat::EntryAdapter<YourParty> {
    /// java: @Override public void extractContent(GuiGraphicsExtractor graphics, int mouseX, int mouseY, boolean hovered, float a)
    fn extract_content(&mut self, list: &mut ModSelectionList, index: usize, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, hovered: bool, a: f32) {
        self.lend(Some(list.list.entry(index)), |__h| ({ let __f = compat::dispatch(__h, |__t| __t.extract_content()); __f(&compat::view(__h, |a| a as compat::Shared<dyn PartyEntry>), &mut context.graphics, mouse_x, mouse_y, hovered, a) }));
        // the row's widgets (Java's `child.extractRenderState(..)` over `children()`)
        context.extract_widgets(&mut self.row, mouse_x, mouse_y);
    }

    fn children(&mut self) -> Option<&mut mcrust::net::minecraft::client::gui::screens::screen::Screen> {
        Some(&mut self.row)
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    fn mouse_clicked(&mut self, list: &mut ModSelectionList, index: usize, event: &MouseButtonEvent, double_click: bool) -> bool {
        self.lend(Some(list.list.entry(index)), |__h| ({ let __f = compat::dispatch(__h, |__t| __t.mouse_clicked()); __f(&compat::view(__h, |a| a as compat::Shared<dyn PartyEntry>), event, double_click) }))
    }

}

/// `client.playerSkinRenderCache().lookup(ResolvableProfile.createUnresolved(name)).thenAccept(gameProfile ->
/// gameProfile.ifPresent(entry -> action(entry.playerSkin().body().texturePath())))`: the action runs on the client
/// thread (the objects it writes are the screen's) once the lookup completes with an entry.
fn lookup_skin_texture(name: &str, action: impl FnOnce(Identifier) + 'static) {
    use mcrust::net::minecraft::core::client_asset::Texture as _;
    let Some(cache) = compat::minecraft().game_renderer.player_skin_render_cache.as_ref() else { return };
    let Ok(profile) = mcrust::net::minecraft::world::item::component::resolvable_profile::ResolvableProfile::create_unresolved_name(name) else { return };
    let action = compat::RenderThread::new(std::cell::RefCell::new(Some(Box::new(action) as Box<dyn FnOnce(Identifier)>)));
    let render_thread: mcrust::java::util::concurrent::completable_future::Executor = Arc::new(|__t: Box<dyn FnOnce() + Send>| crate::compat::execute(__t));
    let _ = cache.lookup(&profile).when_complete_async(move |outcome| {
        if let Ok(Some(entry)) = outcome {
            let texture: Identifier = entry.player_skin().body().texture_path().clone();
            if let Some(action) = action.get().borrow_mut().take() {
                action(texture);
            }
        }
    }, render_thread);
}
