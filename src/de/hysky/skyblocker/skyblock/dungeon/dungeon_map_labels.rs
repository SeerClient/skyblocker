//! Ported from de.hysky.skyblocker.skyblock.dungeon.DungeonMapLabels by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::events::dungeon_events;
use crate::de::hysky::skyblocker::skyblock::dungeon::dungeon_score::DungeonScore;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::dungeon_manager::DungeonManager;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::dungeon_map_utils::DungeonMapUtils;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::room::ClearState;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::room::Direction;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::room::Room;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::room::Shape;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::room::Type;
use crate::de::hysky::skyblocker::utils::fun_utils::FunUtils;
use crate::de::hysky::skyblocker::utils::render::gui_helper::GuiHelper;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler::Scheduler;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::net::minecraft::client::gui::font::Font;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::util::formatted_char_sequence::FormattedCharSequence;
use mcrust::net::minecraft::util::formatted_char_sequence::Seq;
use mcrust::net::minecraft::world::phys::vec3::Vec3;
use mcrust::org::joml::vector2i::Vector2i;

/// java: private static final Object2ObjectOpenHashMap<String, RoomLabel> LABELS = new Object2ObjectOpenHashMap<>();
pub static LABELS: compat::MainThread<std::collections::HashMap<String, RoomLabel>> = compat::MainThread::new(|| std::collections::HashMap::new());
/// java: private static final float LABEL_SCALE = 0.55f;
pub const LABEL_SCALE: f32 = 0.55f32;
/// java: private static final float MAX_WIDTH_SCALAR = 1.5f;
pub const MAX_WIDTH_SCALAR: f32 = 1.5f32;

/// java: public class DungeonMapLabels
#[derive(Clone)]
pub struct DungeonMapLabels {}

impl DungeonMapLabels {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        client_play_connection_events::JOIN.register(Arc::new(|listener: &mut ClientPacketListener| {
            DungeonMapLabels::clear_labels();
        }));
        dungeon_events::DUNGEON_ENDED.register(Arc::new(move || { DungeonMapLabels::clear_labels(); }));
        dungeon_events::ROOM_MATCHED.register(Arc::new(move |__a0| { DungeonMapLabels::on_room_matched(&__a0); }));
        ({ let __r = scheduler::INSTANCE.get_mut(); let __f = compat::dispatch(__r, |__t| __t.schedule_cyclic()); __f(__r, Arc::new(move || { DungeonMapLabels::update_room_names(None); }), 20) });
    }

    /// java: private static boolean shouldProcess()
    pub fn should_process() -> bool {
        return (Utils::is_in_dungeons() && DungeonScore::is_dungeon_started() && !(DungeonManager::is_in_boss()) && SkyblockerConfigManager::get().dungeons.dungeon_map.show_room_labels);
    }

    /// java: private static void clearLabels()
    pub fn clear_labels() {
        if !(LABELS.get_mut().is_empty()) {
            LABELS.get_mut().clear();
        }
    }

    /// java: private static void onRoomMatched(Room room)
    pub fn on_room_matched(room: &compat::Shared<dyn Room>) {
        DungeonMapLabels::update_room_names(Some(room.clone()));
    }

    /// java: private static void updateRoomNames(@Nullable Room newRoom)
    pub fn update_room_names(mut new_room: Option<compat::Shared<dyn Room>>) {
        if !(DungeonMapLabels::should_process()) {
            return;
        }
        let mut entrance_pos: Option<Vector2i> = DungeonManager::get_physical_entrance_pos();
        let mut map_entrance_pos: Option<Vector2i> = DungeonManager::get_map_entrance_pos();
        if entrance_pos.is_none() || map_entrance_pos.is_none() {
            return;
        }
        let mut map_room_size: i32 = DungeonManager::get_map_room_size();
        let mut text_renderer: Font = compat::font();
        if let Some(new_room) = &new_room {
            DungeonMapLabels::add_room_label(new_room, entrance_pos.as_ref().unwrap(), map_entrance_pos.as_ref().unwrap(), map_room_size, &text_renderer);
            return;
        }
        LABELS.get_mut().clear();
        for room in DungeonManager::get_rooms_stream().filter(|__a0| ({ let __f = compat::dispatch(__a0, |__t| __t.is_matched()); __f(__a0) })) {
            DungeonMapLabels::add_room_label(&room, entrance_pos.as_ref().unwrap(), map_entrance_pos.as_ref().unwrap(), map_room_size, &text_renderer);
        }
    }

    /// java: private static void addRoomLabel(Room room, Vector2ic entrancePos, Vector2ic mapEntrancePos, int mapRoomSize, Font textRenderer)
    pub fn add_room_label(room: &compat::Shared<dyn Room>, entrance_pos: &Vector2i, map_entrance_pos: &Vector2i, mut map_room_size: i32, text_renderer: &Font) {
        if LABELS.get_mut().contains_key(({ let __f = compat::dispatch(room, |__t| __t.get_name()); __f(room) }).as_ref().unwrap()) {
            return;
        }
        let Some(mut label_pos) = DungeonMapLabels::get_pos_for_label(room, map_room_size, mcrust::net::minecraft::client::gui::font::LINE_HEIGHT) else {
            return;
        };
        let mut map_pos: crate::unresolved::Vector2dc = DungeonMapUtils::get_map_pos_from_physical_vector2ic_vector2ic_int_position(entrance_pos, map_entrance_pos, map_room_size, &label_pos);
        let Some(mut room_info) = DungeonManager::get_room_metadata(({ let __f = compat::dispatch(room, |__t| __t.get_name()); __f(room) }).as_deref().unwrap()) else {
            return;
        };
        let mut room_name: String = room_info.name();
        let mut color: i32 = match compat::read(room, |__t| __t.room_base().clear_state) {
            ClearState::GreenChecked => mcrust::net::minecraft::util::common_colors::GREEN,
            ClearState::WhiteChecked => mcrust::net::minecraft::util::common_colors::WHITE,
            ClearState::Uncleared => mcrust::net::minecraft::util::common_colors::GRAY,
            ClearState::Failed => mcrust::net::minecraft::util::common_colors::RED,
            _ => unreachable!("switch expression: no case matched"),
        };
        let mut width: f32 = ((DungeonMapLabels::get_max_width(room, map_room_size) as f32) / LABEL_SCALE);
        let mut text: MutableComponent = DungeonMapLabels::get_label_for_room(room, &room_name);
        // Font.split(text, width) = Language.getInstance().getVisualOrder(splitter.splitLines(text, width, Style.EMPTY));
        // the port's `Font::split` answers the lines before the visual order
        let mut lines: Vec<Seq> = mcrust::net::minecraft::locale::language::get_instance().get_visual_order_list(&text_renderer.split(&mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(text.clone()), ((width) as i32)));
        LABELS.get_mut().insert(({ let __f = compat::dispatch(room, |__t| __t.get_name()); __f(room) }).unwrap(), RoomLabel::new(lines, (map_pos.x()) as i32, (map_pos.y()) as i32, color));
    }

    /// java: private static Component getLabelForRoom(Room room, String roomName)
    pub fn get_label_for_room(room: &compat::Shared<dyn Room>, room_name: &str) -> MutableComponent {
        if ({ let __f = compat::dispatch(room, |__t| __t.get_type()); __f(room) }) != Type::Room && ({ let __f = compat::dispatch(room, |__t| __t.get_type()); __f(room) }) != Type::Trap {
            return component::literal(room_name);
        }
        let mut room_label_type: RoomLabelType = SkyblockerConfigManager::get().dungeons.dungeon_map.room_label_type;
        return match room_label_type {
            RoomLabelType::RoomName => component::literal(room_name),
            RoomLabelType::SecretsFound => component::literal(&format!("{}/{}", ({ let __f = compat::dispatch(room, |__t| __t.get_found_secret_count()); __f(room) }), ({ let __f = compat::dispatch(room, |__t| __t.get_max_secret_count()); __f(room) }))),
            RoomLabelType::RoomNameAndSecretsFound => 'yield_1: {
                if FunUtils::should_enable_fun() {
                    break 'yield_1 (component::literal(&format!("{}/{}", ({ let __f = compat::dispatch(room, |__t| __t.get_found_secret_count()); __f(room) }), ({ let __f = compat::dispatch(room, |__t| __t.get_max_secret_count()); __f(room) })))).append_str("\n").append_str(room_name);
                }
                break 'yield_1 (component::literal(room_name).append_str("\n")).append_str(&format!("{}/{}", ({ let __f = compat::dispatch(room, |__t| __t.get_found_secret_count()); __f(room) }), ({ let __f = compat::dispatch(room, |__t| __t.get_max_secret_count()); __f(room) })));
            }
            _ => unreachable!("switch expression: no case matched"),
        };
    }

    /// java: protected static void extractRoomNames(GuiGraphicsExtractor graphics)
    pub fn extract_room_names(graphics: &GuiGraphicsExtractor) {
        if !(SkyblockerConfigManager::get().dungeons.dungeon_map.show_room_labels) {
            return;
        }
        let mut text_renderer: Font = compat::font();
        for label in LABELS.get().values() {
            let __pose_1 = gui_graphics_extractor::push_pose();
            gui_graphics_extractor::pose_translate((label.x) as f32, (label.y) as f32);
            gui_graphics_extractor::pose_scale(LABEL_SCALE, LABEL_SCALE);
            DungeonMapLabels::extract_text(graphics, &text_renderer, &label.text_lines, label.color);
            gui_graphics_extractor::pop_pose(__pose_1);
        }
    }

    /// java: @SuppressWarnings("incomplete-switch") private static @Nullable Vec3 getPosForLabel(Room room, int mapRoomSize, int fontHeight)
    pub fn get_pos_for_label(room: &compat::Shared<dyn Room>, mut map_room_size: i32, mut font_height: i32) -> Option<Vec3> {
        match ({ let __f = compat::dispatch(room, |__t| __t.get_type()); __f(room) }) {
            Type::Blood | Type::Fairy => {
                return None;
            }
            _ => {}
        }
        let mut segments: std::collections::HashSet<Vector2i> = ({ let __f = compat::dispatch(room, |__t| __t.get_segments()); __f(room) });
        let mut segments_x: Vec<i32> = compat::sorted(segments.iter().cloned().map(|__a0| __a0.x())).collect::<Vec<_>>();
        let mut segments_y: Vec<i32> = compat::sorted(segments.iter().cloned().map(|__a0| __a0.y())).collect::<Vec<_>>();
        let mut result: Option<Vector2i> = None;
        match ({ let __f = compat::dispatch(room, |__t| __t.get_shape()); __f(room) }) {
            Shape::Trap | Shape::Puzzle | Shape::Miniboss | Shape::OneByOne => {
                result = Some(Vector2i::new(((segments_x[(0) as usize]).wrapping_add(map_room_size)).wrapping_sub(2), ((((((segments_y[(0) as usize]).wrapping_add(map_room_size) as f64) - (((font_height) as f64) / (2 as f64))) + (1 as f64))) as i32)));
                if map_room_size == 18 {
                    result.as_mut().unwrap().x = (result.as_mut().unwrap().x).wrapping_sub(2);
                }
            }
            Shape::LShape => {
                let mut med_x: i32;
                let mut med_y: i32 = ((segments_y[(1) as usize]).wrapping_add(map_room_size)).wrapping_sub(2);
                if ({ let __f = compat::dispatch(room, |__t| __t.get_direction()); __f(room) }).unwrap() == Direction::Se || ({ let __f = compat::dispatch(room, |__t| __t.get_direction()); __f(room) }).unwrap() == Direction::Sw {
                    med_x = (segments_x[(1) as usize]).wrapping_sub(2);
                } else {
                    med_x = ((segments_x[(2) as usize]).wrapping_sub(map_room_size.wrapping_div(4))).wrapping_add(2);
                }
                result = Some(Vector2i::new(med_x, med_y));
            }
            Shape::TwoByTwo | Shape::OneByTwo | Shape::OneByThree | Shape::OneByFour => {
                let mut avg_x: i32 = (((segments_x[(0) as usize]).wrapping_add(segments_x[((segments_x.len() as i32).wrapping_sub(1)) as usize])).wrapping_add(map_room_size)).wrapping_div(2);
                if ({ let __f = compat::dispatch(room, |__t| __t.get_direction()); __f(room) }).unwrap() == Direction::Se || ({ let __f = compat::dispatch(room, |__t| __t.get_direction()); __f(room) }).unwrap() == Direction::Sw {
                    avg_x = avg_x.wrapping_add(map_room_size.wrapping_div(2));
                } else {
                    avg_x = avg_x.wrapping_add((map_room_size.wrapping_div(4)).wrapping_add(1));
                }
                let mut avg_y: i32 = (((((segments_y[(0) as usize]).wrapping_add(segments_y[((segments_y.len() as i32).wrapping_sub(1)) as usize])).wrapping_add(map_room_size)).wrapping_div(2)).wrapping_add(map_room_size.wrapping_div(3))).wrapping_add(2);
                result = Some(Vector2i::new(avg_x, avg_y));
            }
            _ => {}
        }
        result.as_mut().unwrap().sub(0, font_height.wrapping_div(2));
        return Some(Vec3::new((result.as_ref().unwrap().x()) as f64, (0) as f64, (result.as_ref().unwrap().y()) as f64));
    }

    /// java: private static int getMaxWidth(Room room, int mapRoomSize)
    pub fn get_max_width(room: &compat::Shared<dyn Room>, mut map_room_size: i32) -> i32 {
        let mut max_width: i32 = match ({ let __f = compat::dispatch(room, |__t| __t.get_shape()); __f(room) }) {
            Shape::OneByOne => 'yield_1: {
                if map_room_size == 16 {
                    break 'yield_1 map_room_size.wrapping_add(2);
                } else {
                    break 'yield_1 map_room_size;
                }
            }
            Shape::Trap | Shape::Puzzle | Shape::Miniboss => map_room_size,
            Shape::OneByTwo | Shape::OneByThree | Shape::OneByFour => 'yield_2: {
                if ({ let __f = compat::dispatch(room, |__t| __t.get_direction()); __f(room) }).unwrap() == Direction::Ne {
                    break 'yield_2 map_room_size;
                }
                break 'yield_2 map_room_size.wrapping_mul(3);
            }
            Shape::LShape | Shape::TwoByTwo => map_room_size.wrapping_mul(2),
            _ => unreachable!("switch expression: no case matched"),
        };
        return ((((max_width as f32) * MAX_WIDTH_SCALAR)) as i32);
    }

    /// java: private static void extractText(GuiGraphicsExtractor graphics, Font textRenderer, List<FormattedCharSequence> lines, int color)
    pub fn extract_text(graphics: &GuiGraphicsExtractor, text_renderer: &Font, lines: &Vec<Seq>, mut color: i32) {
        let mut y: i32 = (if (lines.len() as i32) > 1 { (-(mcrust::net::minecraft::client::gui::font::LINE_HEIGHT).wrapping_div(2)).wrapping_mul((lines.len() as i32).wrapping_sub(1)) } else { 0 });
        for ordered_text in lines.iter() {
            let mut text_width: i32 = (text_renderer.width_of_sequence(&**ordered_text)).wrapping_div(2);
            GuiHelper::outlined_text_gui_graphics_extractor_formatted_char_sequence_int_int_int_int(graphics, ordered_text, (-text_width), y, color, mcrust::net::minecraft::util::common_colors::BLACK);
            y = y.wrapping_add(9);
        }
    }

}

/// java: private record RoomLabel(List<FormattedCharSequence> textLines, int x, int y, int color)
pub struct RoomLabel {
    /// java: List<FormattedCharSequence> textLines
    pub text_lines: Vec<Seq>,
    /// java: int x
    pub x: i32,
    /// java: int y
    pub y: i32,
    /// java: int color
    pub color: i32,
}

impl RoomLabel {
    /// java: the canonical record constructor
    pub fn new(mut text_lines: Vec<Seq>, mut x: i32, mut y: i32, mut color: i32) -> Self {
        Self { text_lines, x, y, color }
    }

    /// java: the record accessor `textLines()`
    pub fn text_lines(&self) -> Vec<Seq> {
        self.text_lines.clone()
    }

    /// java: the record accessor `x()`
    pub fn x(&self) -> i32 {
        self.x
    }

    /// java: the record accessor `y()`
    pub fn y(&self) -> i32 {
        self.y
    }

    /// java: the record accessor `color()`
    pub fn color(&self) -> i32 {
        self.color
    }

}

/// java: public enum RoomLabelType implements StringRepresentable
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RoomLabelType {
    RoomName,
    SecretsFound,
    RoomNameAndSecretsFound,
}

impl RoomLabelType {
    pub const VALUES: [RoomLabelType; 3] = [RoomLabelType::RoomName, RoomLabelType::SecretsFound, RoomLabelType::RoomNameAndSecretsFound];

    /// java: values()
    pub fn values() -> Vec<RoomLabelType> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            RoomLabelType::RoomName => "ROOM_NAME",
            RoomLabelType::SecretsFound => "SECRETS_FOUND",
            RoomLabelType::RoomNameAndSecretsFound => "ROOM_NAME_AND_SECRETS_FOUND",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> RoomLabelType {
        match name {
            "ROOM_NAME" => RoomLabelType::RoomName,
            "SECRETS_FOUND" => RoomLabelType::SecretsFound,
            "ROOM_NAME_AND_SECRETS_FOUND" => RoomLabelType::RoomNameAndSecretsFound,
            _ => panic!("IllegalArgumentException: No enum constant RoomLabelType.{name}"),
        }
    }

    /// java: @Override public String getSerializedName()
    pub fn get_serialized_name(&self) -> String {
        return self.name().to_string();
    }

    /// java: @Override public String toString()
    pub fn to_string(&self) -> String {
        return mcrust::net::minecraft::client::resources::language::i18n::get(&format!("skyblocker.config.dungeons.map.roomLabelType.{}", self.name()), &[]);
    }

}

impl std::fmt::Display for RoomLabelType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", RoomLabelType::to_string(self))
    }
}
