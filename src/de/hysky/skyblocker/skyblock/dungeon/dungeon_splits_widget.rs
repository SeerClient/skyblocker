//! Ported from de.hysky.skyblocker.skyblock.dungeon.DungeonSplitsWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::events::dungeon_events;
use crate::de::hysky::skyblocker::events::skyblock_events;
use crate::de::hysky::skyblocker::skyblock::tabhud::config::option_widget_collector::OptionWidgetCollector;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::element::Element;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::element_collector::ElementCollector;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::plain_text_element::PlainTextElement;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::table_element::TableElement;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element_based_widget::ElementBasedWidget;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element_based_widget::ElementBasedWidgetBase;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::hud_widget::HudWidget;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::hud_widget::HudWidgetBase;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::hud_widget::Information;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::table_widget::Row;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::table_widget::TableWidget;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::table_widget::TableWidgetBase;
use crate::de::hysky::skyblocker::utils::codec_utils::CodecUtils;
use crate::de::hysky::skyblocker::utils::data::profiled_data::ProfiledData;
use crate::de::hysky::skyblocker::utils::json_value_input::JsonValueInput;
use crate::de::hysky::skyblocker::utils::json_value_output::JsonValueOutput;
use crate::de::hysky::skyblocker::utils::location::Location;
use crate::de::hysky::skyblocker::utils::utils;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use crate::entry::skyblocker_mod;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::net::minecraft::chat_formatting;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use crate::de::hysky::skyblocker::utils::data::json_data::JsonData as _;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final Pattern FLOOR_PATTERN = Pattern.compile(".*?(?=T)The Catacombs \\((?<floor>[EFM]\\D*\\d*)\\)");
pub static FLOOR_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile(".*?(?=T)The Catacombs \\((?<floor>[EFM]\\D*\\d*)\\)"));
/// java: private static final Pattern DUNGEON_START = Pattern.compile("\\[NPC] Mort: Here, I found this map when I first entered the dungeon\\.|\\[NPC] Mort: Right-click the Orb for spells, and Left-click \\(o
pub static DUNGEON_START: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("\\[NPC] Mort: Here, I found this map when I first entered the dungeon\\.|\\[NPC] Mort: Right-click the Orb for spells, and Left-click \\(or Drop\\) to use your Ultimate!"));
/// java: private static final Pattern BLOOD_OPEN = Pattern.compile("^\\[BOSS] The Watcher: (Congratulations, you made it through the Entrance\\.|Ah, you've finally arrived\\.|Ah, we meet again\\.\\.\\.|So you 
pub static BLOOD_OPEN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("^\\[BOSS] The Watcher: (Congratulations, you made it through the Entrance\\.|Ah, you've finally arrived\\.|Ah, we meet again\\.\\.\\.|So you made it this far\\.\\.\\. interesting\\.|You've managed to scratch and claw your way here, eh\\?|I'm starting to get tired of seeing you around here\\.\\.|Oh\\.\\. hello\\?|Things feel a little more roomy now, eh\\?)$|^The BLOOD DOOR has been opened!$"));
/// java: private static final Pattern BLOOD_CLEAR = Pattern.compile("\\[BOSS] The Watcher: You have proven yourself\\. You may pass\\.");
pub static BLOOD_CLEAR: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("\\[BOSS] The Watcher: You have proven yourself\\. You may pass\\."));
/// java: private static final Pattern DUNGEON_END = Pattern.compile("^\\s*☠ Defeated (.+) in 0?([\\dhms ]+?)\\s*(\\(NEW RECORD!\\))?$");
pub static DUNGEON_END: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("^\\s*☠ Defeated (.+) in 0?([\\dhms ]+?)\\s*(\\(NEW RECORD!\\))?$"));
/// java: private static final Pattern F1_ENTRY = Pattern.compile("^\\[BOSS] Bonzo: Gratz for making it this far, but I'm basically unbeatable\\.$");
pub static F1_ENTRY: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("^\\[BOSS] Bonzo: Gratz for making it this far, but I'm basically unbeatable\\.$"));
/// java: private static final Pattern F2_ENTRY = Pattern.compile("^\\[BOSS] Scarf: This is where the journey ends for you, Adventurers\\.$");
pub static F2_ENTRY: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("^\\[BOSS] Scarf: This is where the journey ends for you, Adventurers\\.$"));
/// java: private static final Pattern F3_ENTRY = Pattern.compile("^\\[BOSS] The Professor: I was burdened with terrible news recently\\.\\.\\.$");
pub static F3_ENTRY: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("^\\[BOSS] The Professor: I was burdened with terrible news recently\\.\\.\\.$"));
/// java: private static final Pattern F4_ENTRY = Pattern.compile("^\\[BOSS] Thorn: Welcome Adventurers! I am Thorn, the Spirit! And host of the Vegan Trials!$");
pub static F4_ENTRY: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("^\\[BOSS] Thorn: Welcome Adventurers! I am Thorn, the Spirit! And host of the Vegan Trials!$"));
/// java: private static final Pattern F5_ENTRY = Pattern.compile("^\\[BOSS] Livid: Welcome, you've arrived right on time\\. I am Livid, the Master of Shadows\\.$");
pub static F5_ENTRY: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("^\\[BOSS] Livid: Welcome, you've arrived right on time\\. I am Livid, the Master of Shadows\\.$"));
/// java: private static final Pattern F6_ENTRY = Pattern.compile("^\\[BOSS] Sadan: So you made it all the way here\\.\\.\\. Now you wish to defy me\\? Sadan\\?!$");
pub static F6_ENTRY: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("^\\[BOSS] Sadan: So you made it all the way here\\.\\.\\. Now you wish to defy me\\? Sadan\\?!$"));
/// java: private static final Pattern F7_ENTRY = Pattern.compile("^\\[BOSS] Maxor: WELL! WELL! WELL! LOOK WHO'S HERE!$");
pub static F7_ENTRY: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("^\\[BOSS] Maxor: WELL! WELL! WELL! LOOK WHO'S HERE!$"));
/// java: private static final Pattern F7_MAXOR = Pattern.compile("^\\[BOSS] Storm: Pathetic Maxor, just like expected\\.$");
pub static F7_MAXOR: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("^\\[BOSS] Storm: Pathetic Maxor, just like expected\\.$"));
/// java: private static final Pattern F7_STORM = Pattern.compile("^\\[BOSS] Goldor: Who dares trespass into my domain\\?$");
pub static F7_STORM: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("^\\[BOSS] Goldor: Who dares trespass into my domain\\?$"));
/// java: private static final Pattern F7_TERMINALS = Pattern.compile("^The Core entrance is opening!$");
pub static F7_TERMINALS: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("^The Core entrance is opening!$"));
/// java: private static final Pattern F7_GOLDOR = Pattern.compile("^\\[BOSS] Necron: You went further than any human before, congratulations\\.$");
pub static F7_GOLDOR: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("^\\[BOSS] Necron: You went further than any human before, congratulations\\.$"));
/// java: private static final Pattern F7_NECRON = Pattern.compile("^\\[BOSS] Necron: All this, for nothing\\.\\.\\.$");
pub static F7_NECRON: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("^\\[BOSS] Necron: All this, for nothing\\.\\.\\.$"));
/// java: private static final Pattern BONZO_SIKE = Pattern.compile("\\[BOSS] Bonzo: Oh I'm dead!");
pub static BONZO_SIKE: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("\\[BOSS] Bonzo: Oh I'm dead!"));
/// java: private static final Pattern SCARF_MINIONS = Pattern.compile("^\\[BOSS] Scarf: Did you forget\\? I was taught by the best! Let's dance\\.$");
pub static SCARF_MINIONS: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("^\\[BOSS] Scarf: Did you forget\\? I was taught by the best! Let's dance\\.$"));
/// java: private static final Pattern GUARDIANS = Pattern.compile("^\\[BOSS] The Professor: Oh\\? You found my Guardians' one weakness\\?$");
pub static GUARDIANS: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("^\\[BOSS] The Professor: Oh\\? You found my Guardians' one weakness\\?$"));
/// java: private static final Pattern PROFESSOR = Pattern.compile("^\\[BOSS] The Professor: I see. You have forced me to use my ultimate technique\\.$");
pub static PROFESSOR: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("^\\[BOSS] The Professor: I see. You have forced me to use my ultimate technique\\.$"));
/// java: private static final Pattern TERRACOTTAS = Pattern.compile("^\\[BOSS] Sadan: ENOUGH!$");
pub static TERRACOTTAS: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("^\\[BOSS] Sadan: ENOUGH!$"));
/// java: private static final Pattern SADAN_GIANTS = Pattern.compile("^\\[BOSS] Sadan: You did it. I understand now, you have earned my respect\\.$");
pub static SADAN_GIANTS: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("^\\[BOSS] Sadan: You did it. I understand now, you have earned my respect\\.$"));
/// java: private static final int[] SPLIT_COLORS = { 0xFD5858, 0xFDAB58, 0xFDFD58, 0xABFD58, 0x58FD58, 0x58FDAB, 0x58ABFD, 0x5858FD, 0xAB58FD, 0xFD58FD };
pub static SPLIT_COLORS: LazyLock<Vec<i32>> = LazyLock::new(|| vec![0xFD5858, 0xFDAB58, 0xFDFD58, 0xABFD58, 0x58FD58, 0x58FDAB, 0x58ABFD, 0x5858FD, 0xAB58FD, 0xFD58FD]);
/// java: private static final Map<String, List<Split>> FLOOR_SPLITS = new HashMap<>();
pub static FLOOR_SPLITS: LazyLock<std::collections::HashMap<String, Vec<compat::Shared<Split>>>> = LazyLock::new(|| std::collections::HashMap::new());
/// java: private static final Path BEST_FILE = SkyblockerMod.CONFIG_DIR.resolve("dungeon_split_bests.json");
pub static BEST_FILE: compat::MainThread<std::path::PathBuf> = compat::MainThread::new(|| skyblocker_mod::CONFIG_DIR.get_mut().join("dungeon_split_bests.json"));
/// java: private static final Codec<Object2ObjectMap<String, Object2LongMap<String>>> BEST_CODEC = CodecUtils.object2ObjectMapCodec(Codec.STRING, CodecUtils.object2LongMapCodec(Codec.STRING));
pub static BEST_CODEC: compat::MainThread<CodecRef<std::collections::HashMap<String, std::collections::HashMap<String, i64>>>> = compat::MainThread::new(|| CodecUtils::object2_object_map_codec(&mcrust::com::mojang::serialization::codec::string_codec(), &CodecUtils::object2_long_map_codec(&mcrust::com::mojang::serialization::codec::string_codec())));
/// java: private static final ProfiledData<Object2ObjectMap<String, Object2LongMap<String>>> BEST_SPLITS = new ProfiledData<>(BEST_FILE, BEST_CODEC);
pub static BEST_SPLITS: compat::MainThread<compat::Shared<ProfiledData<std::collections::HashMap<String, std::collections::HashMap<String, i64>>>>> = compat::MainThread::new(|| ProfiledData::new(BEST_FILE.get_mut().clone(), BEST_CODEC.get_mut().clone()));
/// java: private static @Nullable DungeonSplitsWidget instance;
pub static INSTANCE: compat::MainThread<Option<compat::Shared<DungeonSplitsWidget>>> = compat::MainThread::new(|| None);

/// java: @RegisterWidget public class DungeonSplitsWidget extends TableWidget
#[derive(Clone)]
pub struct DungeonSplitsWidget {
    /// java: extends TableWidget
    pub base: TableWidgetBase,
    /// java: private final List<Split> splits = new ArrayList<>();
    pub splits: Vec<compat::Shared<Split>>,
    /// java: private long startTime = 0L;
    pub start_time: i64,
    /// java: private long elapsedTime = 0L;
    pub elapsed_time: i64,
    /// java: private boolean running = false;
    pub running: bool,
    /// java: private ChatFormatting timerColor = ChatFormatting.YELLOW;
    pub timer_color: ChatFormatting,
    /// java: private String floor = "?";
    pub floor: String,
    /// java: private @Nullable String loadedFloor = null;
    pub loaded_floor: Option<String>,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl std::ops::Deref for DungeonSplitsWidget {
    type Target = TableWidgetBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for DungeonSplitsWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl DungeonSplitsWidget {
    /// java: static { .. } - Java runs it when the class is first used; every static method and
    /// constructor calls this first.
    fn clinit() {
        static ONCE: std::sync::Once = std::sync::Once::new();
        ONCE.call_once(|| {
            let mut floor1_splits: Vec<compat::Shared<Split>> = vec![Split::new("skyblocker.dungeons.splits.bloodOpen".to_string(), BLOOD_OPEN.clone()), Split::new("skyblocker.dungeons.splits.bloodClear".to_string(), BLOOD_CLEAR.clone()), Split::new("skyblocker.dungeons.splits.portalEntry".to_string(), F1_ENTRY.clone()), Split::new("skyblocker.dungeons.splits.bonzoSike".to_string(), BONZO_SIKE.clone()), Split::new("skyblocker.dungeons.splits.bonzo".to_string(), DUNGEON_END.clone())];
            FLOOR_SPLITS.insert("F1".to_string(), floor1_splits.clone());
            FLOOR_SPLITS.insert("M1".to_string(), floor1_splits.clone());
            let mut floor2_splits: Vec<compat::Shared<Split>> = vec![Split::new("skyblocker.dungeons.splits.bloodOpen".to_string(), BLOOD_OPEN.clone()), Split::new("skyblocker.dungeons.splits.bloodClear".to_string(), BLOOD_CLEAR.clone()), Split::new("skyblocker.dungeons.splits.portalEntry".to_string(), F2_ENTRY.clone()), Split::new("skyblocker.dungeons.splits.scarfMinions".to_string(), SCARF_MINIONS.clone()), Split::new("skyblocker.dungeons.splits.scarf".to_string(), DUNGEON_END.clone())];
            FLOOR_SPLITS.insert("F2".to_string(), floor2_splits.clone());
            FLOOR_SPLITS.insert("M2".to_string(), floor2_splits.clone());
            let mut floor3_splits: Vec<compat::Shared<Split>> = vec![Split::new("skyblocker.dungeons.splits.bloodOpen".to_string(), BLOOD_OPEN.clone()), Split::new("skyblocker.dungeons.splits.bloodClear".to_string(), BLOOD_CLEAR.clone()), Split::new("skyblocker.dungeons.splits.portalEntry".to_string(), F3_ENTRY.clone()), Split::new("skyblocker.dungeons.splits.guardians".to_string(), GUARDIANS.clone()), Split::new("skyblocker.dungeons.splits.professor".to_string(), PROFESSOR.clone()), Split::new("skyblocker.dungeons.splits.transformedProfessor".to_string(), DUNGEON_END.clone())];
            FLOOR_SPLITS.insert("F3".to_string(), floor3_splits.clone());
            FLOOR_SPLITS.insert("M3".to_string(), floor3_splits.clone());
            let mut floor4_splits: Vec<compat::Shared<Split>> = vec![Split::new("skyblocker.dungeons.splits.bloodOpen".to_string(), BLOOD_OPEN.clone()), Split::new("skyblocker.dungeons.splits.bloodClear".to_string(), BLOOD_CLEAR.clone()), Split::new("skyblocker.dungeons.splits.portalEntry".to_string(), F4_ENTRY.clone()), Split::new("skyblocker.dungeons.splits.thorn".to_string(), DUNGEON_END.clone())];
            FLOOR_SPLITS.insert("F4".to_string(), floor4_splits.clone());
            FLOOR_SPLITS.insert("M4".to_string(), floor4_splits.clone());
            let mut floor5_splits: Vec<compat::Shared<Split>> = vec![Split::new("skyblocker.dungeons.splits.bloodOpen".to_string(), BLOOD_OPEN.clone()), Split::new("skyblocker.dungeons.splits.bloodClear".to_string(), BLOOD_CLEAR.clone()), Split::new("skyblocker.dungeons.splits.portalEntry".to_string(), F5_ENTRY.clone()), Split::new("skyblocker.dungeons.splits.livid".to_string(), DUNGEON_END.clone())];
            FLOOR_SPLITS.insert("F5".to_string(), floor5_splits.clone());
            FLOOR_SPLITS.insert("M5".to_string(), floor5_splits.clone());
            let mut floor6_splits: Vec<compat::Shared<Split>> = vec![Split::new("skyblocker.dungeons.splits.bloodOpen".to_string(), BLOOD_OPEN.clone()), Split::new("skyblocker.dungeons.splits.bloodClear".to_string(), BLOOD_CLEAR.clone()), Split::new("skyblocker.dungeons.splits.portalEntry".to_string(), F6_ENTRY.clone()), Split::new("skyblocker.dungeons.splits.terracottas".to_string(), TERRACOTTAS.clone()), Split::new("skyblocker.dungeons.splits.sadanGiants".to_string(), SADAN_GIANTS.clone()), Split::new("skyblocker.dungeons.splits.sadan".to_string(), DUNGEON_END.clone())];
            FLOOR_SPLITS.insert("F6".to_string(), floor6_splits.clone());
            FLOOR_SPLITS.insert("M6".to_string(), floor6_splits.clone());
            FLOOR_SPLITS.insert("F7".to_string(), vec![Split::new("skyblocker.dungeons.splits.bloodOpen".to_string(), BLOOD_OPEN.clone()), Split::new("skyblocker.dungeons.splits.bloodClear".to_string(), BLOOD_CLEAR.clone()), Split::new("skyblocker.dungeons.splits.portalEntry".to_string(), F7_ENTRY.clone()), Split::new("skyblocker.dungeons.splits.maxor".to_string(), F7_MAXOR.clone()), Split::new("skyblocker.dungeons.splits.storm".to_string(), F7_STORM.clone()), Split::new("skyblocker.dungeons.splits.terminals".to_string(), F7_TERMINALS.clone()), Split::new("skyblocker.dungeons.splits.goldor".to_string(), F7_GOLDOR.clone()), Split::new("skyblocker.dungeons.splits.necron".to_string(), DUNGEON_END.clone())]);
            FLOOR_SPLITS.insert("M7".to_string(), vec![Split::new("skyblocker.dungeons.splits.bloodOpen".to_string(), BLOOD_OPEN.clone()), Split::new("skyblocker.dungeons.splits.bloodClear".to_string(), BLOOD_CLEAR.clone()), Split::new("skyblocker.dungeons.splits.portalEntry".to_string(), F7_ENTRY.clone()), Split::new("skyblocker.dungeons.splits.maxor".to_string(), F7_MAXOR.clone()), Split::new("skyblocker.dungeons.splits.storm".to_string(), F7_STORM.clone()), Split::new("skyblocker.dungeons.splits.terminals".to_string(), F7_TERMINALS.clone()), Split::new("skyblocker.dungeons.splits.goldor".to_string(), F7_GOLDOR.clone()), Split::new("skyblocker.dungeons.splits.necron".to_string(), F7_NECRON.clone()), Split::new("skyblocker.dungeons.splits.witherKing".to_string(), DUNGEON_END.clone())]);
        });
    }

    /// java: public DungeonSplitsWidget()
    pub fn new() -> compat::Shared<Self> {
        Self::clinit();
        let (_, __this) = TableWidgetBase::new_in(component::literal("Splits").with_style_formats(&[ChatFormatting::Gold, ChatFormatting::Bold]), mcrust::net::minecraft::network::chat::text_color::TextColor::GOLD.get_value(), 3, 0, false, Information::new3("dungeon_splits".to_string(), component::literal("Dungeon Splits"), Location::Dungeon), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                self_ref: std::sync::Weak::new(),
                splits: Vec::new(),
                start_time: 0,
                elapsed_time: 0,
                running: false,
                timer_color: ChatFormatting::Yellow,
                floor: "?".to_string(),
                loaded_floor: None,
            });
            compat::write(&__h, |__t| __t.self_ref = std::sync::Arc::downgrade(&__h));
            ((__h.clone() as compat::Shared<dyn TableWidget>), __h)
        });
        *INSTANCE.get_mut() = Some(__this.clone());
        BEST_SPLITS.get_mut().lock().borrow_mut().init();
        let __this_1 = compat::RenderThread::new(__this.clone());
        dungeon_events::DUNGEON_LOADED.register(Arc::new(move || { DungeonSplitsWidget::on_dungeon_loaded(&(*__this_1.get())); }));
        // TODO(fabric2seer): ClientReceiveMessageEvents.ALLOW_GAME.register: no SeerClient equivalent yet
        //   java: ClientReceiveMessageEvents.ALLOW_GAME.register(this::onChatMessage);
        let __this_2 = compat::RenderThread::new(__this.clone());
        skyblock_events::LOCATION_CHANGE.register(Arc::new(move |__a0| { DungeonSplitsWidget::on_location_change(&(*__this_2.get()), __a0); }));
        __this
    }

    /// java: public static DungeonSplitsWidget getInstance()
    pub fn get_instance() -> compat::Shared<DungeonSplitsWidget> {
        Self::clinit();
        return INSTANCE.get_mut().clone().unwrap();
    }

    /// java: private static String formatTime(long millis)
    pub fn format_time(mut millis: i64) -> String {
        Self::clinit();
        let mut total_seconds: i64 = millis.wrapping_div(1000 as i64);
        let mut minutes: i64 = total_seconds.wrapping_div(60 as i64);
        let mut seconds: i64 = total_seconds.wrapping_rem(60 as i64);
        let mut hundredths: i64 = (millis.wrapping_rem(1000 as i64)).wrapping_div(10 as i64);
        return format!("{:02}:{:02}.{:02}", minutes, seconds, hundredths);
    }

    /// java: private void onDungeonLoaded()
    pub fn on_dungeon_loaded(__this: &compat::Shared<Self>) {
        { let __v = false; compat::write(__this, |__t| __t.running = __v); }
        { let __v = 0; compat::write(__this, |__t| __t.elapsed_time = __v); }
        { let __v = 0; compat::write(__this, |__t| __t.start_time = __v); }
        { let __v = ChatFormatting::Yellow; compat::write(__this, |__t| __t.timer_color = __v); }
        { let __v = None; compat::write(__this, |__t| __t.loaded_floor = __v); }
        DungeonSplitsWidget::update_floor(__this);
        DungeonSplitsWidget::load_floor_splits(__this);
    }

    /// java: private void onLocationChange(Location location)
    pub fn on_location_change(__this: &compat::Shared<Self>, mut location: Location) {
        if location != Location::Dungeon {
            if compat::read(__this, |__t| __t.running) {
                DungeonSplitsWidget::stop_timer(__this, false);
            }
            { let __v = false; compat::write(__this, |__t| __t.running = __v); }
            { let __v = 0; compat::write(__this, |__t| __t.elapsed_time = __v); }
            { let __v = 0; compat::write(__this, |__t| __t.start_time = __v); }
            { let __v = "?".to_string(); compat::write(__this, |__t| __t.floor = __v); }
            { let __v = None; compat::write(__this, |__t| __t.loaded_floor = __v); }
            __this.lock().borrow_mut().splits.clear();
        }
    }

    /// java: @SuppressWarnings("SameReturnValue") private boolean onChatMessage(Component text, boolean overlay)
    pub fn on_chat_message(__this: &compat::Shared<Self>, text: &MutableComponent, mut overlay: bool) -> bool {
        if !(Utils::is_in_dungeons()) || overlay {
            return true;
        }
        let mut stripped: String = chat_formatting::strip_formatting(&text.get_string());
        if !(compat::read(__this, |__t| __t.running)) && DUNGEON_START.matcher(&stripped).matches() {
            { let __v = compat::millis(); compat::write(__this, |__t| __t.start_time = __v); }
            { let __v = 0; compat::write(__this, |__t| __t.elapsed_time = __v); }
            { let __v = true; compat::write(__this, |__t| __t.running = __v); }
            { let __v = ChatFormatting::Yellow; compat::write(__this, |__t| __t.timer_color = __v); }
            for split in compat::read(__this, |__t| __t.splits.clone()) {
                Split::reset(&split);
            }
            return true;
        }
        if !(compat::read(__this, |__t| __t.running)) {
            return true;
        }
        for i in 0..(__this.lock().borrow().splits.len() as i32) {
            let mut split: compat::Shared<Split> = __this.lock().borrow().splits[(i) as usize].clone();
            if !(compat::read(&split, |__t| __t.completed)) && split.lock().borrow().trigger.as_ref().unwrap().matcher(&stripped).matches() {
                let mut time: i64 = (compat::millis()).wrapping_sub(compat::read(__this, |__t| __t.start_time));
                Split::complete(&split, time);
                let mut prev: i64 = (if i == 0 { 0 } else { compat::read(&__this.lock().borrow().splits[(i.wrapping_sub(1)) as usize], |__t| __t.completed_time) });
                let mut segment: i64 = time.wrapping_sub(prev);
                DungeonSplitsWidget::update_best(__this, &split, segment);
                if i == (__this.lock().borrow().splits.len() as i32).wrapping_sub(1) {
                    DungeonSplitsWidget::stop_timer(__this, true);
                }
                return true;
            }
        }
        if stripped.contains("EXTRA STATS") {
            let mut all_completed: bool = true;
            for split in compat::read(__this, |__t| __t.splits.clone()) {
                if !(compat::read(&split, |__t| __t.completed)) {
                    all_completed = false;
                    break;
                }
            }
            if !all_completed {
                DungeonSplitsWidget::stop_timer(__this, false);
            }
        }
        return true;
    }

    /// java: private void updateFloor()
    pub fn update_floor(__this: &compat::Shared<Self>) {
        let mut old: String = compat::read(__this, |__t| __t.floor.clone());
        for line in compat::get_state(&utils::STRING_SCOREBOARD) {
            let mut m: compat::regex::Matcher = FLOOR_PATTERN.matcher(&line);
            if m.matches() {
                { let __v = m.group_named("floor").unwrap(); compat::write(__this, |__t| __t.floor = __v); }
                break;
            }
        }
        if !(compat::read(__this, |__t| __t.floor.clone()) == old) {
            DungeonSplitsWidget::load_floor_splits(__this);
        }
    }

    /// java: private void loadFloorSplits()
    pub fn load_floor_splits(__this: &compat::Shared<Self>) {
        if Some(&*(compat::read(__this, |__t| __t.floor.clone()))) == compat::read(__this, |__t| __t.loaded_floor.clone()).as_deref() || compat::read(__this, |__t| __t.floor.clone()) == "?" {
            return;
        }
        { let __v = Some(compat::read(__this, |__t| __t.floor.clone())); compat::write(__this, |__t| __t.loaded_floor = __v); }
        __this.lock().borrow_mut().splits.clear();
        let mut group: Option<Vec<compat::Shared<Split>>> = FLOOR_SPLITS.get(&compat::read(__this, |__t| __t.floor.clone())).cloned();
        if let Some(group) = &group {
            let mut data: std::collections::HashMap<String, std::collections::HashMap<String, i64>> = ProfiledData::compute_if_absent(BEST_SPLITS.get_mut(), Arc::new(move || std::collections::HashMap::new()));
            let mut floor_data: Option<std::collections::HashMap<String, i64>> = data.get(&compat::read(__this, |__t| __t.floor.clone())).cloned();
            for s in group.iter() {
                let mut best: i64 = (if floor_data.is_some() { floor_data.as_ref().unwrap().get(&compat::read(s, |__t| __t.key.clone())).cloned().unwrap_or_else(|| 0) } else { 0 });
                let __h0 = Split::new_3(compat::read(s, |__t| __t.key.clone()), compat::read(s, |__t| __t.trigger.clone()).unwrap(), best);
                __this.lock().borrow_mut().splits.push(__h0);
            }
        }
    }

    /// java: private void stopTimer(boolean success)
    pub fn stop_timer(__this: &compat::Shared<Self>, mut success: bool) {
        if compat::read(__this, |__t| __t.running) {
            { let __v = (compat::millis()).wrapping_sub(compat::read(__this, |__t| __t.start_time)); compat::write(__this, |__t| __t.elapsed_time = __v); }
            { let __v = false; compat::write(__this, |__t| __t.running = __v); }
            if success {
                let mut data: std::collections::HashMap<String, std::collections::HashMap<String, i64>> = ProfiledData::compute_if_absent(BEST_SPLITS.get_mut(), Arc::new(move || std::collections::HashMap::new()));
                let mut floor_data: std::collections::HashMap<String, i64> = data.get(&compat::read(__this, |__t| __t.floor.clone())).cloned().unwrap_or_else(|| std::collections::HashMap::new());
                let mut updated: bool = false;
                for split in compat::read(__this, |__t| __t.splits.clone()) {
                    if compat::read(&split, |__t| __t.completed) {
                        let mut current_best: i64 = floor_data.get(&compat::read(&split, |__t| __t.key.clone())).cloned().unwrap_or_else(|| 0);
                        if current_best == 0 || compat::read(&split, |__t| __t.completed_time) < current_best {
                            floor_data.insert(compat::read(&split, |__t| __t.key.clone()), compat::read(&split, |__t| __t.completed_time));
                            updated = true;
                        }
                    }
                }
                if updated {
                    data.insert(compat::read(__this, |__t| __t.floor.clone()), floor_data.clone());
                    ProfiledData::put(BEST_SPLITS.get_mut(), data.clone());
                    BEST_SPLITS.get_mut().lock().borrow().save();
                }
            }
        }
        { let __v = (if success { ChatFormatting::Green } else { ChatFormatting::Red }); compat::write(__this, |__t| __t.timer_color = __v); }
    }

    /// java: private void updateBest(Split split, long completionTime)
    pub fn update_best(__this: &compat::Shared<Self>, split: &compat::Shared<Split>, mut completion_time: i64) {
        let mut data: std::collections::HashMap<String, std::collections::HashMap<String, i64>> = ProfiledData::compute_if_absent(BEST_SPLITS.get_mut(), Arc::new(move || std::collections::HashMap::new()));
        let mut floor_data: std::collections::HashMap<String, i64> = data.get(&compat::read(__this, |__t| __t.floor.clone())).cloned().unwrap_or_else(|| std::collections::HashMap::new());
        let mut current_best: i64 = floor_data.get(&compat::read(split, |__t| __t.key.clone())).cloned().unwrap_or_else(|| 0);
        if current_best == 0 || completion_time < current_best {
            floor_data.insert(compat::read(split, |__t| __t.key.clone()), completion_time);
            data.insert(compat::read(__this, |__t| __t.floor.clone()), floor_data.clone());
            ProfiledData::put(BEST_SPLITS.get_mut(), data.clone());
            BEST_SPLITS.get_mut().lock().borrow().save();
        }
    }

}

impl TableWidget for DungeonSplitsWidget {
    fn table_widget_base(&self) -> &TableWidgetBase {
        &self.base
    }
    fn table_widget_base_mut(&mut self) -> &mut TableWidgetBase {
        &mut self.base
    }

    /// java: @Override protected List<Row> buildRows() (dispatch: the body for this class)
    fn build_rows(&self) -> fn(&compat::Shared<dyn TableWidget>) -> Vec<Row> {
        DungeonSplitsWidget::build_rows_dyn
    }

}

impl ElementBasedWidget for DungeonSplitsWidget {
    fn element_based_widget_base(&self) -> &ElementBasedWidgetBase {
        &self.base.base
    }
    fn element_based_widget_base_mut(&mut self) -> &mut ElementBasedWidgetBase {
        &mut self.base.base
    }

    /// java: @Override public boolean shouldUpdateBeforeRendering() (dispatch: the body for this class)
    fn should_update_before_rendering(&self) -> fn(&compat::Shared<dyn ElementBasedWidget>) -> bool {
        DungeonSplitsWidget::should_update_before_rendering_dyn
    }

    /// java: @Override public void updateContent() (dispatch: the body for this class)
    fn update_content(&self) -> fn(&compat::Shared<dyn ElementBasedWidget>) {
        DungeonSplitsWidget::update_content_dyn
    }

    /// java: @Override protected void updateConfigContent(ElementCollector collector) (dispatch: the body for this class)
    fn update_config_content(&self) -> fn(&compat::Shared<dyn ElementBasedWidget>, &compat::Shared<dyn ElementCollector>) {
        DungeonSplitsWidget::update_config_content_dyn
    }

}

impl HudWidget for DungeonSplitsWidget {
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

    /// java: @Override public void load(JsonValueInput input) (dispatch: the body for this class)
    fn load(&self) -> fn(&compat::Shared<dyn HudWidget>, &JsonValueInput) {
        <dyn ElementBasedWidget>::load_from::<Self>
    }

    /// java: @Override public void save(JsonValueOutput output) (dispatch: the body for this class)
    fn save(&self) -> fn(&compat::Shared<dyn HudWidget>, &mut JsonValueOutput) {
        <dyn ElementBasedWidget>::save_from::<Self>
    }

    /// java: @Override public void getOptionWidgets(OptionWidgetCollector collector) (dispatch: the body for this class)
    fn get_option_widgets(&self) -> fn(&compat::Shared<dyn HudWidget>, &mut OptionWidgetCollector) {
        <dyn ElementBasedWidget>::get_option_widgets_from::<Self>
    }

    /// java: @Override public void updateConfigPreview() (dispatch: the body for this class)
    fn update_config_preview(&self) -> fn(&compat::Shared<dyn HudWidget>) {
        <dyn ElementBasedWidget>::update_config_preview_from::<Self>
    }

}

impl ElementCollector for DungeonSplitsWidget {
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

impl DungeonSplitsWidget {
    /// java: @Override protected List<Row> buildRows()
    pub fn build_rows_impl(__this: &compat::Shared<Self>) -> Vec<Row> {
        let mut now: i64 = (if compat::read(__this, |__t| __t.running) { (compat::millis()).wrapping_sub(compat::read(__this, |__t| __t.start_time)) } else { (if compat::read(__this, |__t| __t.start_time) == 0 { 0 } else { compat::read(__this, |__t| __t.elapsed_time) }) });
        let mut previous: i64 = 0;
        let mut showing_current: bool = false;
        let mut rows: Vec<Row> = Vec::new();
        for idx in 0..(__this.lock().borrow().splits.len() as i32) {
            let mut split: compat::Shared<Split> = __this.lock().borrow().splits[(idx) as usize].clone();
            let mut name: compat::Shared<dyn Element> = (PlainTextElement::new(Some((component::translatable(&compat::read(&split, |__t| __t.key.clone()))).with_color(SPLIT_COLORS[(idx.wrapping_rem(SPLIT_COLORS.len() as i32)) as usize]))) as compat::Shared<dyn Element>);
            let mut best_str: String = DungeonSplitsWidget::format_time(compat::read(&split, |__t| __t.best_time));
            let mut mid: compat::Shared<dyn Element>;
            let mut right: compat::Shared<dyn Element>;
            let mut border: i32 = 0;
            if compat::read(&split, |__t| __t.completed) {
                let mut segment_time: i64 = (compat::read(&split, |__t| __t.completed_time)).wrapping_sub(previous);
                let mut diff: i64 = segment_time.wrapping_sub(compat::read(&split, |__t| __t.best_time));
                let mut fmt: ChatFormatting = (if diff <= (0 as i64) { ChatFormatting::Green } else { ChatFormatting::Red });
                let mut diff_str: String = format!("%+.2fs");
                mid = (PlainTextElement::new(Some(component::literal(&diff_str).with_style_format(fmt))) as compat::Shared<dyn Element>);
                right = (PlainTextElement::new(Some((component::literal(&DungeonSplitsWidget::format_time(compat::read(&split, |__t| __t.completed_time)))).with_style_format(ChatFormatting::Yellow))) as compat::Shared<dyn Element>);
                previous = compat::read(&split, |__t| __t.completed_time);
            } else if !showing_current && compat::read(__this, |__t| __t.running) || compat::read(__this, |__t| __t.start_time) != 0 {
                let mut segment_time: i64 = now.wrapping_sub(previous);
                mid = (PlainTextElement::new(Some(component::literal(&DungeonSplitsWidget::format_time(segment_time)))) as compat::Shared<dyn Element>);
                right = (PlainTextElement::new(Some(component::literal(&best_str))) as compat::Shared<dyn Element>);
                showing_current = true;
                border = SPLIT_COLORS[(idx.wrapping_rem(SPLIT_COLORS.len() as i32)) as usize];
            } else {
                mid = (PlainTextElement::new(Some(component::literal("--"))) as compat::Shared<dyn Element>);
                right = (PlainTextElement::new(Some(component::literal(&best_str))) as compat::Shared<dyn Element>);
            }
            rows.push(Row::new(vec![name.clone(), mid.clone(), right.clone()], border));
        }
        return rows;
    }

    /// DungeonSplitsWidget.buildRows as the dispatch of TableWidget.buildRows
    pub fn build_rows_dyn(__this: &compat::Shared<dyn TableWidget>) -> Vec<Row> {
        Self::build_rows_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public boolean shouldUpdateBeforeRendering()
    pub fn should_update_before_rendering_impl(__this: &compat::Shared<Self>) -> bool {
        return true;
    }

    /// DungeonSplitsWidget.shouldUpdateBeforeRendering as the dispatch of ElementBasedWidget.shouldUpdateBeforeRendering
    pub fn should_update_before_rendering_dyn(__this: &compat::Shared<dyn ElementBasedWidget>) -> bool {
        Self::should_update_before_rendering_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public void updateContent()
    pub fn update_content_impl(__this: &compat::Shared<Self>) {
        DungeonSplitsWidget::update_floor(__this);
        DungeonSplitsWidget::load_floor_splits(__this);
        let __r0 = compat::read(__this, |__t| __t.floor.clone());
        { let (__a0,) = ((PlainTextElement::new(Some(component::literal(&format!("Floor: {}", __r0)))) as compat::Shared<dyn Element>),); __this.lock().borrow_mut().add_element(__a0) };
        <dyn TableWidget>::update_content_body(&compat::view(__this, |a| a as compat::Shared<dyn TableWidget>));
        let mut now: i64 = (if compat::read(__this, |__t| __t.running) { (compat::millis()).wrapping_sub(compat::read(__this, |__t| __t.start_time)) } else { (if compat::read(__this, |__t| __t.start_time) == 0 { 0 } else { compat::read(__this, |__t| __t.elapsed_time) }) });
        let __r0 = compat::read(__this, |__t| __t.timer_color);
        { let (__a0,) = ((PlainTextElement::new(Some(component::literal(&DungeonSplitsWidget::format_time(now)).with_style_format(__r0))) as compat::Shared<dyn Element>),); __this.lock().borrow_mut().add_element(__a0) };
    }

    /// DungeonSplitsWidget.updateContent as the dispatch of ElementBasedWidget.updateContent
    pub fn update_content_dyn(__this: &compat::Shared<dyn ElementBasedWidget>) {
        Self::update_content_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override protected void updateConfigContent(ElementCollector collector)
    pub fn update_config_content_impl(__this: &compat::Shared<Self>, collector: &compat::Shared<dyn ElementCollector>) {
        { let (__a0,) = ((PlainTextElement::new(Some(component::literal("Floor: ???"))) as compat::Shared<dyn Element>),); collector.lock().borrow_mut().add_element(__a0) };
        let mut element: compat::Shared<TableElement> = TableElement::new(3, 5, 0, false);
        for i in 0..5 {
            TableElement::add_to_cell(&element, 0, i, (PlainTextElement::new(Some(component::literal(&format!("Split {}", i.wrapping_add(1))))) as compat::Shared<dyn Element>));
            if i < 3 {
                TableElement::add_to_cell(&element, 1, i, (PlainTextElement::new(Some((component::literal("-5.24s")).with_style_format(ChatFormatting::Green))) as compat::Shared<dyn Element>));
                TableElement::add_to_cell(&element, 2, i, (PlainTextElement::new(Some((component::literal(&format!("00:{}.00", (i.wrapping_add(1)).wrapping_mul(10)))).with_style_format(ChatFormatting::Yellow))) as compat::Shared<dyn Element>));
            } else if i < 4 {
                TableElement::add_to_cell(&element, 1, i, (PlainTextElement::new(Some(component::literal("00:05.45"))) as compat::Shared<dyn Element>));
                TableElement::add_to_cell(&element, 2, i, (PlainTextElement::new(Some(component::literal("00:40.00"))) as compat::Shared<dyn Element>));
            } else {
                TableElement::add_to_cell(&element, 1, i, (PlainTextElement::new(Some(component::literal("--"))) as compat::Shared<dyn Element>));
                TableElement::add_to_cell(&element, 2, i, (PlainTextElement::new(Some(component::literal("00:50.00"))) as compat::Shared<dyn Element>));
            }
        }
        { let (__a0,) = ((element.clone() as compat::Shared<dyn Element>),); collector.lock().borrow_mut().add_element(__a0) };
        { let (__a0,) = ((PlainTextElement::new(Some(component::literal("00:34.55").with_style_format(ChatFormatting::Yellow))) as compat::Shared<dyn Element>),); collector.lock().borrow_mut().add_element(__a0) };
    }

    /// DungeonSplitsWidget.updateConfigContent as the dispatch of ElementBasedWidget.updateConfigContent
    pub fn update_config_content_dyn(__this: &compat::Shared<dyn ElementBasedWidget>, __p0: &compat::Shared<dyn ElementCollector>) {
        Self::update_config_content_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

}

/// java: private static class Split
#[derive(Clone)]
pub struct Split {
    /// java: final String key;
    pub key: String,
    /// java: final Pattern trigger;
    pub trigger: Option<compat::regex::Pattern>,
    /// java: long bestTime;
    pub best_time: i64,
    /// java: long completedTime;
    pub completed_time: i64,
    /// java: boolean completed;
    pub completed: bool,
}

impl Split {
    /// java: Split(String key, Pattern trigger)
    pub fn new(mut key: String, mut trigger: compat::regex::Pattern) -> compat::Shared<Self> {
        let __this = Self::new_3(key.clone(), trigger.clone(), ((0) as i64) as i64);
        __this
    }

    /// java: Split(String key, Pattern trigger, long bestTime)
    pub fn new_3(mut key: String, mut trigger: compat::regex::Pattern, mut best_time: i64) -> compat::Shared<Self> {
        let __init_key = key.clone();
        let __init_trigger = Some(trigger.clone());
        let __init_best_time = best_time;
        let __this = compat::shared(Self {
            key: __init_key,
            trigger: __init_trigger,
            best_time: __init_best_time,
            completed_time: 0,
            completed: false,
        });
        __this
    }

    /// java: void complete(long time)
    pub fn complete(__this: &compat::Shared<Self>, mut time: i64) {
        { let __v = true; compat::write(__this, |__t| __t.completed = __v); }
        { let __v = time; compat::write(__this, |__t| __t.completed_time = __v); }
    }

    /// java: void reset()
    pub fn reset(__this: &compat::Shared<Self>) {
        { let __v = false; compat::write(__this, |__t| __t.completed = __v); }
        { let __v = 0; compat::write(__this, |__t| __t.completed_time = __v); }
    }

}
