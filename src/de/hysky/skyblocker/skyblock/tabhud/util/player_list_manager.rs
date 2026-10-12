//! Ported from de.hysky.skyblocker.skyblock.tabhud.util.PlayerListManager by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::tabhud::config::option_widget_collector::OptionWidgetCollector;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::widget_manager::WidgetManager;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::element::Element;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::element_collector::ElementCollector;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::plain_text_element::PlainTextElement;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element_based_widget::ElementBasedWidget;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element_based_widget::ElementBasedWidgetBase;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::hud_widget::HudWidget;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::hud_widget::HudWidgetBase;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::hud_widget::Information;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::tab_hud_widget::TabHudWidget;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::tab_hud_widget::TabHudWidgetBase;
use crate::de::hysky::skyblocker::utils::json_value_input::JsonValueInput;
use crate::de::hysky::skyblocker::utils::json_value_output::JsonValueOutput;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::com::mojang::authlib::game_profile::GameProfile;
use mcrust::java::text::decimal_format_symbols::Locale;
use mcrust::java::util::uuid::Uuid;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::minecraft;
use mcrust::net::minecraft::client::multiplayer::player_info::PlayerInfo;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: public static boolean shouldUpdateNextTick = false;
pub static SHOULD_UPDATE_NEXT_TICK: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));
/// java: private static boolean playerListLoaded = false;
pub static PLAYER_LIST_LOADED: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));
/// java: private static int loadingTicks = 0;
pub static LOADING_TICKS: LazyLock<Mutex<i32>> = LazyLock::new(|| Mutex::new(0));
/// java: private static final Pattern PLAYERS_COLUMN_PATTERN = Pattern.compile("\\s*(Players \\(\\d+\\)|Island|Coop \\(\\d+\\))\\s*");
pub static PLAYERS_COLUMN_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("\\s*(Players \\(\\d+\\)|Island|Coop \\(\\d+\\))\\s*"));
/// java: private static final Pattern INFO_COLUMN_PATTERN = Pattern.compile("\\s*Info\\s*");
pub static INFO_COLUMN_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("\\s*Info\\s*"));
/// java: public static final Pattern PLAYER_NAME_PATTERN = Pattern.compile("\\[(?<level>\\d+)] (?:\\[[A-Za-z]+] )?(?<name>[A-Za-z0-9_]+)(?: .+)?");
pub static PLAYER_NAME_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("\\[(?<level>\\d+)] (?:\\[[A-Za-z]+] )?(?<name>[A-Za-z0-9_]+)(?: .+)?"));
/// java: private static List<PlayerInfo> playerList = new ArrayList<>();
pub static PLAYER_LIST: compat::MainThread<Vec<PlayerInfo>> = compat::MainThread::new(|| Vec::new());
/// java: private static List<String> playerStringList = new ArrayList<>();
pub static PLAYER_STRING_LIST: LazyLock<Mutex<Vec<String>>> = LazyLock::new(|| Mutex::new(Vec::new()));
/// java: private static @Nullable String footer;
pub static FOOTER: LazyLock<Mutex<Option<String>>> = LazyLock::new(|| Mutex::new(None));
/// java: static final Map<String, Widget> WIDGET_MAP = new Object2ObjectLinkedOpenHashMap<>();
pub static WIDGET_MAP: compat::MainThread<std::collections::HashMap<String, Widget>> = compat::MainThread::new(|| std::collections::HashMap::new());
/// java: private static final Set<Runnable> TAB_LISTENERS = new ObjectOpenHashSet<>();
pub static TAB_LISTENERS: compat::MainThread<std::collections::HashSet<Arc<dyn Fn()>>> = compat::MainThread::new(|| std::collections::HashSet::new());
// TODO(fabric2seer): static call MultimapBuilder.hashKeys(..)
//   java: private static final Multimap<String, Consumer<Widget>> TAB_WIDGET_LISTENERS = MultimapBuilder.hashKeys().arrayListValues().build();
/// java: private static final Multimap<String, Consumer<Widget>> TAB_WIDGET_LISTENERS = MultimapBuilder.hashKeys().arrayListValues().build();
pub static TAB_WIDGET_LISTENERS: compat::MainThread<crate::unresolved::Multimap> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `TAB_WIDGET_LISTENERS` - see the TODO above"));
/// java: private static final Set<Runnable> FOOTER_LISTENERS = new ObjectOpenHashSet<>();
pub static FOOTER_LISTENERS: compat::MainThread<std::collections::HashSet<Arc<dyn Fn()>>> = compat::MainThread::new(|| std::collections::HashSet::new());
/// java: static final Map<String, HudWidget> HANDLED_TAB_WIDGETS = new Object2ObjectOpenHashMap<>();
pub static HANDLED_TAB_WIDGETS: compat::MainThread<std::collections::HashMap<String, compat::Shared<dyn HudWidget>>> = compat::MainThread::new(|| std::collections::HashMap::new());
/// java: public static final Widget EMPTY = new Widget(Component.empty(), List.of(), List.of());
pub static WIDGET_EMPTY: compat::MainThread<Widget> = compat::MainThread::new(|| Widget::new(component::empty(), vec![], vec![]));

/// java: public class PlayerListManager
#[derive(Clone)]
pub struct PlayerListManager {}

impl PlayerListManager {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static PlayerListManager.@Nullable Widget getListWidget(String name)
    pub fn get_list_widget(name: &str) -> Option<Widget> {
        return WIDGET_MAP.get_mut().get(name).cloned();
    }

    /// java: public static @Nullable HudWidget getTabWidget(String name)
    pub fn get_tab_widget(name: &str) -> Option<compat::Shared<dyn HudWidget>> {
        return HANDLED_TAB_WIDGETS.get_mut().get(name).cloned();
    }

    /// java: public static Set<String> getCurrentWidgets()
    pub fn get_current_widgets() -> std::collections::HashSet<String> {
        return WIDGET_MAP.get_mut().keys().cloned().collect::<std::collections::HashSet<_>>();
    }

    /// java: public static void addHandledTabWidget(String name, HudWidget widget)
    pub fn add_handled_tab_widget(mut name: String, mut widget: compat::Shared<dyn HudWidget>) {
        HANDLED_TAB_WIDGETS.get_mut().insert(name.clone(), widget.clone());
    }

    /// java: public static void registerTabListener(Runnable listener)
    pub fn register_tab_listener(listener: Arc<dyn Fn()>) {
        TAB_LISTENERS.get_mut().insert(listener.clone());
    }

    /// java: public static void registerFooterListener(Runnable listener)
    pub fn register_footer_listener(listener: Arc<dyn Fn()>) {
        FOOTER_LISTENERS.get_mut().insert(listener.clone());
    }

    /// java: public static void registerTabWidgetListener(String widgetName, Consumer<Widget> listener)
    pub fn register_tab_widget_listener(mut widget_name: String, listener: Arc<dyn Fn(Widget)>) {
        // TODO(fabric2seer): `.put(..)` on a Multimap
        //   java: TAB_WIDGET_LISTENERS.put(widgetName, listener);
    }

    /// java: public static boolean isPlayerListLoaded()
    pub fn is_player_list_loaded() -> bool {
        return compat::get_state(&PLAYER_LIST_LOADED);
    }

    /// java: @Init public static void init()
    pub fn init() {
        client_play_connection_events::JOIN.register(Arc::new(|listener: &mut ClientPacketListener| {
            compat::set_state(&PLAYER_LIST_LOADED, false);
            compat::set_state(&LOADING_TICKS, 0);
        }));
    }

    /// java: public static void tryUpdateList()
    pub fn try_update_list() {
        if !(compat::get_state(&PLAYER_LIST_LOADED)) {
            if compat::get_state(&LOADING_TICKS) > 40 {
                compat::set_state(&PLAYER_LIST_LOADED, true);
            } else {
                compat::set_state(&LOADING_TICKS, compat::get_state(&LOADING_TICKS) + 1);
            }
        }
        if compat::get_state(&SHOULD_UPDATE_NEXT_TICK) {
            PlayerListManager::update_list_now();
        }
    }

    /// java: public static void updateListNow()
    pub fn update_list_now() {
        PlayerListManager::update_list();
        compat::set_state(&SHOULD_UPDATE_NEXT_TICK, false);
    }

    /// java: public static void updateList()
    pub fn update_list() {
        if !(Utils::is_on_skyblock()) {
            return;
        }
        let mut network_handler: Option<mcrust::net::minecraft::client::multiplayer::client_packet_listener::ClientPacketListener> = minecraft::get_connection(compat::minecraft());
        if let Some(network_handler) = &network_handler {
            // TODO(fabric2seer): static call PlayerTabOverlayAccessor.getOrdering(..)
            //   java: playerList = networkHandler.getOnlinePlayers()
            //   java: 					.stream()
            //   java: 					.sorted(PlayerTabOverlayAccessor.getOrdering())
            //   java: 					.toList();
            compat::set_state(&PLAYER_STRING_LIST, PLAYER_LIST.get_mut().iter().cloned().map(|__a0| __a0.get_tab_list_display_name()).filter(|__a0| true).map(|__a0| __a0.get_string()).map(|__a0| __a0.trim().to_string()).collect::<Vec<_>>());
        }
        if (PLAYER_LIST.get_mut().len() as i32) >= 80 {
            compat::set_state(&PLAYER_LIST_LOADED, true);
        }
        if Utils::is_in_dungeons() {
            PlayerListManager::update_dungeons(None);
        } else {
            PlayerListManager::update_widgets_from(PLAYER_LIST.get_mut());
        }
    }

    /// java: public static void updateDungeons(@Nullable List<Component> lines)
    pub fn update_dungeons(mut lines: Option<Vec<MutableComponent>>) {
        if let Some(lines) = &lines {
            *PLAYER_LIST.get_mut() = Vec::new();
            for i in 0..(lines.len() as i32) {
                // TODO(fabric2seer): `new` takes 3 argument(s) in the port (`id: Uuid, name: &str, properties: PropertyMap`), Java passes 2
                // rust: PLAYER_LIST.get_mut().push(PlayerInfo::new(GameProfile::new(Uuid::random_uuid(), &i.to_string()), false));
                (PLAYER_LIST.get_mut()[PLAYER_LIST.get_mut().len() - 1]).set_tab_list_display_name(Some(lines[(i) as usize].clone()));
            }
        }
        WIDGET_MAP.get_mut().clear();
        WIDGET_MAP.get_mut().insert("Dungeon Buffs".to_string(), WIDGET_EMPTY.get_mut().clone());
        WIDGET_MAP.get_mut().insert("Dungeon Deaths".to_string(), WIDGET_EMPTY.get_mut().clone());
        WIDGET_MAP.get_mut().insert("Dungeon Downed".to_string(), WIDGET_EMPTY.get_mut().clone());
        WIDGET_MAP.get_mut().insert("Dungeon Puzzles".to_string(), WIDGET_EMPTY.get_mut().clone());
        WIDGET_MAP.get_mut().insert("Dungeon Discoveries".to_string(), WIDGET_EMPTY.get_mut().clone());
        WIDGET_MAP.get_mut().insert("Dungeon Info".to_string(), WIDGET_EMPTY.get_mut().clone());
        for i in 1..6 {
            WIDGET_MAP.get_mut().insert(format!("Dungeon Player {}", i), WIDGET_EMPTY.get_mut().clone());
        }
        for __a0 in TAB_LISTENERS.get_mut().iter() {
            (__a0)();
        }
        for (key, value) in WIDGET_MAP.get_mut().iter() {
            // TODO(fabric2seer): `.get(..)` on a Multimap
            //   java: TAB_WIDGET_LISTENERS.get(..).forEach(..);
        }
    }

    /// java: public static void updateWidgetsFrom(List<PlayerInfo> lines)
    pub fn update_widgets_from(lines: &Vec<PlayerInfo>) {
        let mut players_column_predicate: Arc<dyn Fn(String) -> bool> = Arc::new(move |s: &String| PLAYERS_COLUMN_PATTERN.matches_all(s));
        let mut info_column_predicate: Arc<dyn Fn(String) -> bool> = Arc::new(move |s: &String| INFO_COLUMN_PATTERN.matches_all(s));
        let mut side_thing: MutableComponent = component::empty();
        let mut contents: Vec<MutableComponent> = Vec::new();
        let mut player_list_entries: Vec<PlayerInfo> = Vec::new();
        let mut doing_players: bool = false;
        let mut players_done: bool = false;
        let mut hypixel_widget_name: (i32, String) = (0xFFFF00, "".to_string());
        WIDGET_MAP.get_mut().clear();
        for player_list_entry in lines.iter() {
            let Some(mut display_name) = player_list_entry.get_tab_list_display_name() else {
                continue;
            };
            let mut string: String = display_name.get_string();
            if compat::is_blank(&string) {
                continue;
            }
            if !players_done {
                if (players_column_predicate)(string.clone()) {
                    if !doing_players {
                        doing_players = true;
                        hypixel_widget_name = (mcrust::net::minecraft::network::chat::text_color::TextColor::AQUA.get_value(), "Players".to_string());
                    }
                    continue;
                }
                if (info_column_predicate)(string.clone()) {
                    players_done = true;
                    WIDGET_MAP.get_mut().insert(hypixel_widget_name.1.clone(), Widget::new(component::empty(), contents.clone(), player_list_entries.clone()));
                    contents.clear();
                    player_list_entries.clear();
                    continue;
                }
            } else {
                if (info_column_predicate)(string.clone()) {
                    continue;
                }
                if !(string.starts_with(" ")) && string.contains(":") && !(hypixel_widget_name.1.starts_with("Mining Event")) || !(string.to_lowercase().starts_with("ends in")) {
                    if !contents.is_empty() || !(side_thing.get_string().is_empty()) {
                        WIDGET_MAP.get_mut().insert(hypixel_widget_name.1.clone(), Widget::new(side_thing.clone(), contents.clone(), player_list_entries.clone()));
                    }
                    side_thing = component::empty();
                    contents.clear();
                    player_list_entries.clear();
                    let mut name_and_info: ((i32, String), MutableComponent) = PlayerListManager::get_name_and_info(&display_name);
                    hypixel_widget_name = name_and_info.0.clone();
                    if !(HANDLED_TAB_WIDGETS.get_mut().contains_key(&hypixel_widget_name.1)) {
                        WidgetManager::add_widget_instance((DefaultTabHudWidget::new(hypixel_widget_name.1.clone(), component::literal(&hypixel_widget_name.1).with_style_format(ChatFormatting::Bold), hypixel_widget_name.0) as compat::Shared<dyn HudWidget>));
                    }
                    if !(compat::is_blank(&name_and_info.1.get_string())) {
                        side_thing = PlayerListManager::trim(&name_and_info.1);
                        // TODO(fabric2seer): the mod keeps the game's `PlayerInfo` (Java holds a reference to the live object); the port's `PlayerInfo` is not a value to copy and the port lends the mod no handle to it
                        // rust: player_list_entries.push(player_list_entry.clone());
                    }
                    continue;
                }
            }
            contents.push(PlayerListManager::trim(&display_name));
            // TODO(fabric2seer): the mod keeps the game's `PlayerInfo` (Java holds a reference to the live object); the port's `PlayerInfo` is not a value to copy and the port lends the mod no handle to it
            // rust: player_list_entries.push(player_list_entry.clone());
        }
        if !contents.is_empty() || !(side_thing.get_string().is_empty()) {
            WIDGET_MAP.get_mut().insert(hypixel_widget_name.1.clone(), Widget::new(side_thing.clone(), contents.clone(), player_list_entries.clone()));
        }
        for __a0 in TAB_LISTENERS.get_mut().iter() {
            (__a0)();
        }
        for (key, value) in WIDGET_MAP.get_mut().iter() {
            // TODO(fabric2seer): `.get(..)` on a Multimap
            //   java: TAB_WIDGET_LISTENERS.get(..).forEach(..);
        }
    }

    /// java: private static Component trim(Component text)
    pub fn trim(text: &MutableComponent) -> MutableComponent {
        let mut trimmed_parts: Vec<MutableComponent> = Vec::new();
        let mut leading_space_found: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
        // TODO(fabric2seer): a lambda for a parameter the port does not type as a closure (`net.minecraft.network.chat.FormattedText.StyledContentConsumer<T>`)
        // rust: text.visit_styled(|style, as_string| {
        // rust: let mut trimmed: String = as_string.clone();
        // rust: if !(leading_space_found.load(std::sync::atomic::Ordering::SeqCst)) {
        // rust: trimmed = trimmed.trim_start().to_string();
        // rust: if !(compat::is_blank(&trimmed)) {
        // rust: leading_space_found.store(true, std::sync::atomic::Ordering::SeqCst);
        // rust: } else {
        // rust: return None;
        // rust: }
        // rust: }
        // rust: trimmed_parts.push(component::literal(&trimmed).set_style(style.clone()));
        // rust: return None;
        // rust: }, &mcrust::net::minecraft::network::chat::style::Style::EMPTY);
        for i in 0..(trimmed_parts.len() as i32) {
            let mut last: MutableComponent = trimmed_parts.pop().unwrap();
            let mut trimmed: String = last.get_string().trim_end().to_string();
            if !(compat::is_blank(&trimmed)) {
                trimmed_parts.push(component::literal(&trimmed).set_style(last.get_style().clone()));
                break;
            }
        }
        let mut out: MutableComponent = component::empty();
        for __a0 in trimmed_parts.iter() {
            out.append(__a0.clone());
        }
        return out;
    }

    /// java: private static Pair<IntObjectPair<String>, ? extends Component> getNameAndInfo(Component text)
    pub fn get_name_and_info(text: &MutableComponent) -> ((i32, String), MutableComponent) {
        let mut to_return: (String, MutableComponent) = ("".to_string(), component::empty());
        let mut in_info: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
        let mut color_output: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0xFFFF00);
        // TODO(fabric2seer): a lambda for a parameter the port does not type as a closure (`net.minecraft.network.chat.FormattedText.StyledContentConsumer<T>`)
        // rust: text.visit_styled(|style, as_string| {
        // rust: if in_info.load(std::sync::atomic::Ordering::SeqCst) {
        // rust: to_return.1.append(component::literal(&as_string).with_style(&style));
        // rust: } else {
        // rust: if as_string.contains(":") {
        // rust: in_info.store(true, std::sync::atomic::Ordering::SeqCst);
        // rust: let mut split: Vec<String> = compat::regex::split_limit(&as_string, ":", 2);
        // rust: to_return.0;
        // rust: to_return.1.append((component::literal(&split[(1) as usize])).with_style(&style));
        // rust: if style.get_color().is_some() {
        // rust: color_output.store(style.get_color().unwrap().get_value(), std::sync::atomic::Ordering::SeqCst);
        // rust: }
        // rust: } else {
        // rust: to_return.0;
        // rust: }
        // rust: }
        // rust: return None;
        // rust: }, &mcrust::net::minecraft::network::chat::style::Style::EMPTY);
        return ((color_output.load(std::sync::atomic::Ordering::SeqCst), to_return.0.clone()), to_return.1.clone());
    }

    /// java: public static List<PlayerInfo> getPlayerList()
    pub fn get_player_list() -> Vec<PlayerInfo> {
        return PLAYER_LIST.get_mut().clone();
    }

    /// java: public static List<String> getPlayerStringList()
    pub fn get_player_string_list() -> Vec<String> {
        return compat::get_state(&PLAYER_STRING_LIST);
    }

    /// java: public static void updateFooter(@Nullable Component f)
    pub fn update_footer(mut f: Option<MutableComponent>) {
        if let Some(f) = &f {
            compat::set_state(&FOOTER, Some(f.get_string()));
            if compat::lock(&FOOTER).unwrap().is_empty() {
                compat::set_state(&FOOTER, None);
            }
            for __a0 in FOOTER_LISTENERS.get_mut().iter() {
                (__a0)();
            }
        } else {
            compat::set_state(&FOOTER, None);
        }
    }

    /// java: public static @Nullable String getFooter()
    pub fn get_footer() -> Option<String> {
        return compat::get_state(&FOOTER);
    }

    /// java: public static @Nullable Matcher regexAt(int idx, Pattern p)
    pub fn regex_at(mut idx: i32, p: &compat::regex::Pattern) -> Option<compat::regex::Matcher> {
        let Some(mut str) = PlayerListManager::str_at(idx) else {
            return None;
        };
        let mut m: compat::regex::Matcher = p.matcher(&str);
        if !m.matches() {
            logger().error(&format!("no match: \"{}\" against \"{}\"", str, compat::jstring(&p)));
            return None;
        } else {
            return Some(m);
        }
    }

    /// java: public static @Nullable String strAt(int idx)
    pub fn str_at(mut idx: i32) -> Option<String> {
        if (PLAYER_LIST.get_mut().len() as i32) <= idx {
            return None;
        }
        let Some(mut txt) = (PLAYER_LIST.get_mut()[(idx) as usize]).get_tab_list_display_name() else {
            return None;
        };
        let mut str: String = compat::trim(&txt.get_string());
        if str.is_empty() {
            return None;
        }
        return Some(str);
    }

    /// java: public static @Nullable Component textAt(int idx)
    pub fn text_at(mut idx: i32) -> Option<MutableComponent> {
        if (PLAYER_LIST.get_mut().len() as i32) <= idx {
            return None;
        }
        let Some(mut txt) = (PLAYER_LIST.get_mut()[(idx) as usize]).get_tab_list_display_name() else {
            return None;
        };
        let mut new_text: MutableComponent = component::empty();
        let mut size: i32 = (txt.get_siblings().len() as i32);
        for i in 0..size {
            let current = &txt.get_siblings()[(i) as usize];
            let mut text_to_append: String = current.get_string();
            if i == 0 {
                text_to_append = text_to_append.trim_start().to_string();
            }
            if i == size.wrapping_sub(1) {
                text_to_append = text_to_append.trim_end().to_string();
            }
            new_text.append(component::literal(&text_to_append).set_style(current.get_style().clone()));
        }
        if new_text.get_string().is_empty() {
            return None;
        }
        return Some(new_text);
    }

    /// java: public static PlayerInfo getRaw(int idx)
    pub fn get_raw(mut idx: i32) -> PlayerInfo {
        // TODO(fabric2seer): the mod keeps the game's `PlayerInfo` (Java holds a reference to the live object); the port's `PlayerInfo` is not a value to copy and the port lends the mod no handle to it
        // rust: return PLAYER_LIST.get_mut()[(idx) as usize].clone();
        return todo!("fabric2seer: see the TODO above");
    }

    /// java: public static int getSize()
    pub fn get_size() -> i32 {
        return (PLAYER_LIST.get_mut().len() as i32);
    }

}

/// java: private static final class DefaultTabHudWidget extends TabHudWidget
#[derive(Clone)]
pub struct DefaultTabHudWidget {
    /// java: extends TabHudWidget
    pub base: TabHudWidgetBase,
}

impl std::ops::Deref for DefaultTabHudWidget {
    type Target = TabHudWidgetBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for DefaultTabHudWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl DefaultTabHudWidget {
    /// java: private DefaultTabHudWidget(String hypixelWidgetName, MutableComponent title, int color)
    pub fn new(mut hypixel_widget_name: String, mut title: MutableComponent, mut color: i32) -> compat::Shared<Self> {
        let (_, __this) = TabHudWidgetBase::new_in(hypixel_widget_name.clone(), title.clone(), color, Information::new(Some(HudWidgetBase::name_to_id(&hypixel_widget_name)), Some(title.plain_copy().append(component::literal(" (auto)").with_style_formats(&[ChatFormatting::Gray, ChatFormatting::Italic])))), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
            });
            ((__h.clone() as compat::Shared<dyn TabHudWidget>), __h)
        });
        __this
    }

}

impl TabHudWidget for DefaultTabHudWidget {
    fn tab_hud_widget_base(&self) -> &TabHudWidgetBase {
        &self.base
    }
    fn tab_hud_widget_base_mut(&mut self) -> &mut TabHudWidgetBase {
        &mut self.base
    }

    /// java: @Override protected void updateContent(Widget widget) (dispatch: the body for this class)
    fn update_content_1(&self) -> fn(&compat::Shared<dyn TabHudWidget>, &Widget) {
        DefaultTabHudWidget::update_content_1_dyn
    }

}

impl ElementBasedWidget for DefaultTabHudWidget {
    fn element_based_widget_base(&self) -> &ElementBasedWidgetBase {
        &self.base.base
    }
    fn element_based_widget_base_mut(&mut self) -> &mut ElementBasedWidgetBase {
        &mut self.base.base
    }

    /// java: @Override public void updateContent() (dispatch: the body for this class)
    fn update_content(&self) -> fn(&compat::Shared<dyn ElementBasedWidget>) {
        <dyn TabHudWidget>::update_content_from::<Self>
    }

    /// java: @Override protected final void updateConfigContent(ElementCollector collector) (dispatch: the body for this class)
    fn update_config_content(&self) -> fn(&compat::Shared<dyn ElementBasedWidget>, &compat::Shared<dyn ElementCollector>) {
        <dyn TabHudWidget>::update_config_content_from::<Self>
    }

}

impl HudWidget for DefaultTabHudWidget {
    fn hud_widget_base(&self) -> &HudWidgetBase {
        &self.base.base.base
    }
    fn hud_widget_base_mut(&mut self) -> &mut HudWidgetBase {
        &mut self.base.base.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override protected final void extractWidgetRenderState(GuiGraphicsExtractor context, float delta) (dispatch: the body for this class)
    fn extract_widget_render_state(&self) -> fn(&compat::Shared<dyn HudWidget>, &mut GuiGraphicsExtractor, f32) {
        <dyn ElementBasedWidget>::extract_widget_render_state_from::<Self>
    }

    /// java: @Override protected void extractWidgetRenderStateForConfig(GuiGraphicsExtractor graphics, float delta) (dispatch: the body for this class)
    fn extract_widget_render_state_for_config(&self) -> fn(&compat::Shared<dyn HudWidget>, &mut GuiGraphicsExtractor, f32) {
        <dyn ElementBasedWidget>::extract_widget_render_state_for_config_from::<Self>
    }

    /// java: @Override public boolean shouldRender() (dispatch: the body for this class)
    fn should_render(&self) -> fn(&compat::Shared<dyn HudWidget>) -> bool {
        <dyn TabHudWidget>::should_render_from::<Self>
    }

    /// java: @Override public void load(JsonValueInput input) (dispatch: the body for this class)
    fn load(&self) -> fn(&compat::Shared<dyn HudWidget>, &JsonValueInput) {
        <dyn TabHudWidget>::load_from::<Self>
    }

    /// java: @Override public void save(JsonValueOutput output) (dispatch: the body for this class)
    fn save(&self) -> fn(&compat::Shared<dyn HudWidget>, &mut JsonValueOutput) {
        <dyn TabHudWidget>::save_from::<Self>
    }

    /// java: @Override public void getOptionWidgets(OptionWidgetCollector collector) (dispatch: the body for this class)
    fn get_option_widgets(&self) -> fn(&compat::Shared<dyn HudWidget>, &mut OptionWidgetCollector) {
        <dyn TabHudWidget>::get_option_widgets_from::<Self>
    }

    /// java: @Override public void updateConfigPreview() (dispatch: the body for this class)
    fn update_config_preview(&self) -> fn(&compat::Shared<dyn HudWidget>) {
        <dyn TabHudWidget>::update_config_preview_from::<Self>
    }

}

impl ElementCollector for DefaultTabHudWidget {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: public <T extends Element> T addElement(T c)
    fn add_element(&mut self, mut c: compat::Shared<dyn Element>) -> compat::Shared<dyn Element> {
        ({ let __f = compat::dispatch(&c, |__t| __t.set_parent()); __f(&c, self.clone()) });
        self.elements.push(c.clone());
        return c;
    }

}

impl DefaultTabHudWidget {
    /// java: @Override protected void updateContent(Widget widget)
    pub fn update_content_1_impl(__this: &compat::Shared<Self>, widget: &Widget) {
        if !(widget.detail().get_string().is_empty()) {
            { let (__a0,) = ((PlainTextElement::new(Some(widget.detail())) as compat::Shared<dyn Element>),); __this.lock().borrow_mut().add_element(__a0) };
        }
        for line in widget.lines() {
            { let (__a0,) = ((PlainTextElement::new(Some(line.clone())) as compat::Shared<dyn Element>),); __this.lock().borrow_mut().add_element(__a0) };
        }
    }

    /// DefaultTabHudWidget.updateContent as the dispatch of TabHudWidget.updateContent
    pub fn update_content_1_dyn(__this: &compat::Shared<dyn TabHudWidget>, __p0: &Widget) {
        Self::update_content_1_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

}

/// java: public record Widget(Component detail, List<Component> lines, List<PlayerInfo> playerListEntries)
pub struct Widget {
    /// java: Component detail
    pub detail: MutableComponent,
    /// java: List<Component> lines
    pub lines: Vec<MutableComponent>,
    /// java: List<PlayerInfo> playerListEntries
    pub player_list_entries: Vec<PlayerInfo>,
}

impl Widget {
    /// java: public Widget(Component detail, List<Component> lines, List<PlayerInfo> playerListEntries)
    pub fn new(mut detail: MutableComponent, mut lines: Vec<MutableComponent>, mut player_list_entries: Vec<PlayerInfo>) -> Self {
        let __init_detail = detail.copy();
        let __init_lines = lines.iter().cloned().map(|__a0| __a0.copy()).collect::<Vec<_>>();
        let __init_player_list_entries = player_list_entries.iter().cloned().collect::<Vec<_>>();
        let mut this = Self {
            detail: __init_detail,
            lines: __init_lines,
            player_list_entries: __init_player_list_entries,
        };
        this
    }

    /// java: the record accessor `detail()`
    pub fn detail(&self) -> MutableComponent {
        self.detail.clone()
    }

    /// java: the record accessor `lines()`
    pub fn lines(&self) -> Vec<MutableComponent> {
        self.lines.clone()
    }

    /// java: the record accessor `playerListEntries()`
    pub fn player_list_entries(&self) -> Vec<PlayerInfo> {
        self.player_list_entries.clone()
    }

}
