//! Ported from de.hysky.skyblocker.skyblock.dungeon.DungeonMap by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::dungeon::dungeon_map_labels::DungeonMapLabels;
use crate::de::hysky::skyblocker::skyblock::dungeon::dungeon_map_texture::DungeonMapTexture;
use crate::de::hysky::skyblocker::skyblock::dungeon::dungeon_score::DungeonScore;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::dungeon_manager::DungeonManager;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::dungeon_map_utils::DungeonMapUtils;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::dungeon_player_manager::DungeonPlayer;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::dungeon_player_manager::DungeonPlayerManager;
use crate::de::hysky::skyblocker::utils::render::gui_helper::GuiHelper;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::java::util::uuid::Uuid;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::world::entity::player::player::Player;
use mcrust::net::minecraft::world::item::items;
use mcrust::net::minecraft::world::item::map_item;
use mcrust::net::minecraft::world::level::level::Level;
use mcrust::net::minecraft::world::level::saveddata::maps::map_decoration::MapDecoration;
use mcrust::net::minecraft::world::level::saveddata::maps::map_id::MapId;
use mcrust::net::minecraft::world::level::saveddata::maps::map_item_saved_data::MapItemSavedData;
use mcrust::net::minecraft::core::component::data_component_map::DataComponentGetter as _;

/// java: private static final Identifier DUNGEON_MAP = SkyblockerMod.id("dungeon_map");
pub static DUNGEON_MAP: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("dungeon_map"));
/// java: private static final MapId DEFAULT_MAP_ID_COMPONENT = new MapId(1024);
pub static DEFAULT_MAP_ID_COMPONENT: compat::MainThread<MapId> = compat::MainThread::new(|| MapId::new(1024));
/// java: private static @Nullable MapId cachedMapIdComponent = null;
pub static CACHED_MAP_ID_COMPONENT: compat::MainThread<Option<MapId>> = compat::MainThread::new(|| None);

/// java: public class DungeonMap
#[derive(Clone)]
pub struct DungeonMap {}

impl DungeonMap {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        hud_render_callback::EVENT.register(Arc::new(|ctx: &mut hud_render_callback::HudRenderContext<'_>| {
            // SAFETY: `ctx` outlives every call made through `graphics` (this callback).
            let graphics = unsafe { ScriptGraphics::new(ctx.gl, &*ctx.minecraft, ctx.gui_width, ctx.gui_height) };
            let _session = graphics.session();
            DungeonMap::extract(&mut graphics);
        }));
        client_command_registration_callback::EVENT.register(Arc::new(
            |dispatcher: &mut client_command_registration_callback::ClientCommandDispatcher| {
                dispatcher.register("skyblocker", |ctx| {
                    #[allow(unreachable_code)]
                    Ok(())
                });
            },
        ));
        client_play_connection_events::JOIN.register(Arc::new(|listener: &mut ClientPacketListener| {
            DungeonMap::reset();
        }));
    }

    /// java: private static boolean shouldProcess()
    pub fn should_process() -> bool {
        return (Utils::is_in_dungeons() && DungeonScore::is_dungeon_started() && !(DungeonManager::is_in_boss()));
    }

    /// java: private static void extract(GuiGraphicsExtractor graphics)
    pub fn extract(graphics: &mut GuiGraphicsExtractor) {
        let dungeon_map = &SkyblockerConfigManager::get().dungeons.dungeon_map;
        if DungeonMap::should_process() && dungeon_map.enable_map {
            DungeonMap::extract_5(graphics, dungeon_map.map_x, dungeon_map.map_y, dungeon_map.map_scaling, dungeon_map.fancy_map);
        }
    }

    /// java: public static void extract(GuiGraphicsExtractor context, int x, int y, float scale, boolean fancy)
    pub fn extract_5(context: &mut GuiGraphicsExtractor, mut x: i32, mut y: i32, mut scale: f32, mut fancy: bool) {
        DungeonMap::extract_render_state(context, x, y, scale, fancy, i32::MIN, i32::MIN, None);
    }

    /// java: public static @Nullable UUID extractRenderState(GuiGraphicsExtractor graphics, int x, int y, float scale, boolean fancy, int mouseX, int mouseY, @Nullable UUID enlarge)
    pub fn extract_render_state(graphics: &mut GuiGraphicsExtractor, mut x: i32, mut y: i32, mut scale: f32, mut fancy: bool, mut mouse_x: i32, mut mouse_y: i32, mut enlarge: Option<Uuid>) -> Option<Uuid> {
        let client = compat::minecraft();
        let dungeon_map = &SkyblockerConfigManager::get().dungeons.dungeon_map;
        if mcrust::mcrust::modding::game::with_player(|_| ()).is_none() || mcrust::mcrust::modding::game::level().is_none() {
            return None;
        }
        let mut map_id: MapId = DungeonMap::get_map_id_component(mcrust::mcrust::modding::game::with_player(|player| player.base.base.get_inventory().get_non_equipment_items()[(8) as usize].clone()));
        let Some(mut state) = map_item::get_saved_data(Some(map_id.clone()), &mcrust::mcrust::modding::game::level().unwrap()) else {
            return None;
        };
        let __pose_1 = gui_graphics_extractor::push_pose();
        gui_graphics_extractor::pose_translate((x) as f32, (y) as f32);
        gui_graphics_extractor::pose_scale((scale) as f32, (scale) as f32);
        if dungeon_map.background_blur {
            GuiHelper::blurred_rectangle(graphics, 0, 0, 128, 128, 5);
        }
        if dungeon_map.show_outline {
            GuiHelper::border(graphics, 0, 0, 128, 128, mcrust::net::minecraft::util::common_colors::LIGHT_GRAY);
        }
        DungeonMapTexture::blit_map(graphics);
        DungeonMapLabels::extract_room_names(graphics);
        let mut hovered_head: Option<Uuid> = None;
        if fancy {
            hovered_head = DungeonMap::extract_player_heads(graphics, &mcrust::mcrust::modding::game::level().unwrap(), &state, (((mouse_x as f32) / scale)) as f64, (((mouse_y as f32) / scale)) as f64, enlarge.clone());
        }
        gui_graphics_extractor::pop_pose(__pose_1);
        return hovered_head;
    }

    /// java: public static MapId getMapIdComponent(@Nullable ItemStack stack)
    pub fn get_map_id_component(mut stack: Option<mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> MapId {
        if stack.is_some() && stack.as_ref().unwrap().is_item(&items::items().get("filled_map")) && stack.as_ref().unwrap().has(mcrust::net::minecraft::core::component::data_components::map_id()) {
            let mut map_id_component: MapId = stack.as_ref().unwrap().get(mcrust::net::minecraft::core::component::data_components::map_id()).unwrap();
            *CACHED_MAP_ID_COMPONENT.get_mut() = Some(map_id_component.clone());
            return map_id_component;
        } else {
            return (if CACHED_MAP_ID_COMPONENT.get_mut().is_some() { CACHED_MAP_ID_COMPONENT.get_mut().clone() } else { Some(DEFAULT_MAP_ID_COMPONENT.get_mut().clone()) }).unwrap();
        }
    }

    /// java: private static @Nullable UUID extractPlayerHeads(GuiGraphicsExtractor graphics, Level level, MapItemSavedData state, double mouseX, double mouseY, @Nullable UUID enlarge)
    pub fn extract_player_heads(graphics: &mut GuiGraphicsExtractor, level: &Level, state: &MapItemSavedData, mut mouse_x: f64, mut mouse_y: f64, mut enlarge: Option<Uuid>) -> Option<Uuid> {
        if !(DungeonManager::is_clearing_dungeon()) {
            return None;
        }
        let mut i: i32 = 1;
        let mut hovered: Option<Uuid> = None;
        for map_decoration in state.decorations.iter().map(|(k, v)| (k.clone(), v.clone())) {
            let mut dungeon_player: Option<compat::Shared<DungeonPlayer>> = None;
            // TODO(fabric2seer): static `MapDecorationTypes.FRAME` has no Rust counterpart in the port's MapDecorationTypes file
            if map_decoration.1.type_().value() == crate::unresolved::MapDecorationTypes::FRAME.value() {
                if !(SkyblockerConfigManager::get().dungeons.dungeon_map.show_self_head) {
                    continue;
                }
                dungeon_player = Some(DungeonPlayerManager::get_players().unwrap()[(0) as usize].clone());
            } else {
                while i < (DungeonPlayerManager::get_players().unwrap().len() as i32) && dungeon_player.is_none() || !(DungeonPlayer::alive(dungeon_player.as_ref().unwrap())) {
                    dungeon_player = Some(DungeonPlayerManager::get_players().unwrap()[(i) as usize].clone());
                    i += 1;
                }
            }
            if let Some(dungeon_player) = &dungeon_player {
                if !(DungeonPlayer::alive(dungeon_player)) {
                    DungeonMap::dungeon_player_error(&map_decoration.0, "not alive", i.wrapping_sub(1), &DungeonPlayerManager::get_players().unwrap(), &state.decorations);
                    continue;
                } else if DungeonPlayer::uuid(dungeon_player).is_none() {
                    DungeonMap::dungeon_player_error(&map_decoration.0, "has null uuid", i.wrapping_sub(1), &DungeonPlayerManager::get_players().unwrap(), &state.decorations);
                    continue;
                }
            } else {
                DungeonMap::dungeon_player_error(&map_decoration.0, "not found", i.wrapping_sub(1), &DungeonPlayerManager::get_players().unwrap(), &state.decorations);
                continue;
            }
            let mut player: PlayerRenderState = PlayerRenderState::of(level, dungeon_player.as_ref().unwrap(), &map_decoration.1);
            let __pose_1 = gui_graphics_extractor::push_pose();
            // TODO(fabric2seer): `.x(..)` on a Vector2dc
            //   java: graphics.pose().translate((float) player.mapPos().x(), (float) player.mapPos().y());
            gui_graphics_extractor::pose_rotate((((((player.deg() + 180.0f32) as f64).to_radians()) as f32)) as f32);
            if player.uuid() == enlarge.clone().unwrap() {
                gui_graphics_extractor::pose_scale((2) as f32, (2) as f32);
            } else if hovered.is_none() && DungeonMap::is_player_hovered(&player, mouse_x, mouse_y) {
                gui_graphics_extractor::pose_scale((2) as f32, (2) as f32);
                hovered = Some(player.uuid());
            }
            GuiHelper::player_head(graphics, -4, -4, 8, &player.uuid());
            GuiHelper::border(graphics, -5, -5, 10, 10, DungeonPlayer::dungeon_class(dungeon_player.as_ref().unwrap()).color());
            graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), -1, -7, 1, -5, DungeonPlayer::dungeon_class(dungeon_player.as_ref().unwrap()).color());
            gui_graphics_extractor::pop_pose(__pose_1);
        }
        return hovered;
    }

    /// java: private static void dungeonPlayerError(String decorationId, String reason, int i, DungeonPlayerManager.DungeonPlayer[] dungeonPlayers, Map<String, MapDecoration> mapDecorations)
    pub fn dungeon_player_error(decoration_id: &str, reason: &str, mut i: i32, dungeon_players: &Vec<compat::Shared<DungeonPlayer>>, map_decorations: &std::collections::HashMap<String, MapDecoration>) {
        logger().error(&format!("[Skyblocker Dungeon Map] Dungeon player for map decoration '{}' {}. Player list index (zero-indexed): {}. Player list: {}. Map decorations: {}", decoration_id, reason, i, format!("{:?}", dungeon_players), compat::jstring(&map_decorations)));
    }

    /// java: public static boolean isPlayerHovered(PlayerRenderState player, double mouseX, double mouseY)
    pub fn is_player_hovered(player: &PlayerRenderState, mut mouse_x: f64, mut mouse_y: f64) -> bool {
        // TODO(fabric2seer): `.distanceSquared(..)` on a Vector2dc
        //   java: return player.mapPos().distanceSquared(mouseX, mouseY) < 16;
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: private static void reset()
    pub fn reset() {
        *CACHED_MAP_ID_COMPONENT.get_mut() = None;
    }

}

/// java: public record PlayerRenderState(UUID uuid, String name, Vector2dc mapPos, float deg)
pub struct PlayerRenderState {
    /// java: UUID uuid
    pub uuid: Uuid,
    /// java: String name
    pub name: String,
    /// java: Vector2dc mapPos
    pub map_pos: crate::unresolved::Vector2dc,
    /// java: float deg
    pub deg: f32,
}

impl PlayerRenderState {
    /// java: the canonical record constructor
    pub fn new(mut uuid: Uuid, mut name: String, mut map_pos: crate::unresolved::Vector2dc, mut deg: f32) -> Self {
        Self { uuid, name, map_pos, deg }
    }

    /// java: the record accessor `uuid()`
    pub fn uuid(&self) -> Uuid {
        self.uuid.clone()
    }

    /// java: the record accessor `name()`
    pub fn name(&self) -> String {
        self.name.clone()
    }

    /// java: the record accessor `mapPos()`
    pub fn map_pos(&self) -> crate::unresolved::Vector2dc {
        self.map_pos.clone()
    }

    /// java: the record accessor `deg()`
    pub fn deg(&self) -> f32 {
        self.deg
    }

    /// java: public static PlayerRenderState of(Level world, DungeonPlayerManager.DungeonPlayer dungeonPlayer, MapDecoration mapDecoration)
    pub fn of(world: &Level, dungeon_player: &compat::Shared<DungeonPlayer>, map_decoration: &MapDecoration) -> PlayerRenderState {
        // TODO(fabric2seer): `Level.getPlayerByUUID` has no Rust method in the port (looked for `get_player_by_uuid` in Level and its supertypes)
        // rust: let mut player_entity: Option<Box<dyn Player>> = Some(world.get_player_by_uuid(&DungeonPlayer::uuid(dungeon_player).unwrap()));
        let mut player_entity: Box<dyn Player> = todo!("fabric2seer: the initialiser of `playerEntity` (TODO above)");
        // TODO(fabric2seer): `new Vector2d(..)`
        //   java: Vector2dc mapPos = playerEntity != null ? DungeonMapUtils.getMapPosFromPhysical(DungeonManager.getPhysicalEntrancePos(), DungeonManager.getMapEntrancePos(), DungeonManager.getMapRoomSize(), playerEntity.position()) : new Vector2d(mapDecoration.x() / 2d + 64, mapDecoration.y() / 2d + 64);
        let mut map_pos: crate::unresolved::Vector2dc = todo!("fabric2seer: the initialiser of `mapPos` (TODO above)");
        let mut deg: f32 = (if true /* java: player_entity != null: never null here */ { player_entity.get_y_rot() } else { (((map_decoration.rot() as i32).wrapping_mul(360) as f32) / 16.0f32) });
        return PlayerRenderState::new(DungeonPlayer::uuid(dungeon_player).unwrap(), DungeonPlayer::name(dungeon_player), map_pos.clone(), deg);
    }

}
