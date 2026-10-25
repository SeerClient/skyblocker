//! Ported from de.hysky.skyblocker.skyblock.dungeon.preview.PreviewRoom by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::dungeon::preview::room_preview_server;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::dungeon_manager::DungeonManager;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::dungeon_manager::RoomWaypoint;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::dungeon_map_utils::DungeonMapUtils;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::room;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::room::Direction;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::room::MatchState;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::room::Room;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::room::RoomBase;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::room::Type;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::secret_waypoint::SecretWaypoint;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_collector::PrimitiveCollector;
use crate::de::hysky::skyblocker::utils::render::renderable::Renderable;
use crate::de::hysky::skyblocker::utils::tickable::Tickable;
use mcrust::net::minecraft::core::block_pos::BlockPos;
use mcrust::net::minecraft::world::level::block::block::Block;
use mcrust::net::minecraft::world::level::block::blocks;
use mcrust::net::minecraft::world::level::block::state::block_state::BlockState;
use mcrust::net::minecraft::world::level::level::Level;
use mcrust::org::joml::vector2i::Vector2i;
use mcrust::net::minecraft::util::thread::blockable_event_loop::BlockableEventLoop as _;
use mcrust::net::minecraft::world::level::collision_getter::CollisionGetter as _;
use mcrust::net::minecraft::world::level::level_writer::LevelWriter as _;

/// java: private static final Vector2ic ORIGIN = new Vector2i(0, 0);
pub static ORIGIN: compat::MainThread<Vector2i> = compat::MainThread::new(|| Vector2i::new(0, 0));

/// java: public class PreviewRoom extends Room
pub struct PreviewRoom {
    /// java: extends Room
    pub base: RoomBase,
}

impl std::ops::Deref for PreviewRoom {
    type Target = RoomBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for PreviewRoom {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl PreviewRoom {
    /// java: public PreviewRoom(String name)
    pub fn new(mut name: String) -> compat::Shared<Self> {
        let (_, __this) = RoomBase::new_in(Type::Room, vec![ORIGIN.get_mut().clone()], move |__base| {
            let __h = compat::shared(Self {
                base: __base,
            });
            ((__h.clone() as compat::Shared<dyn Room>), __h)
        });
        { let __v = Some(Direction::Nw); compat::write(&__this, |__t| __t.direction = __v); }
        { let __v = MatchState::Matched; compat::write(&__this, |__t| __t.match_state = __v); }
        { let __v = Some(name.clone()); compat::write(&__this, |__t| __t.name = __v); }
        { let __v = Some(ORIGIN.get_mut().clone()); compat::write(&__this, |__t| __t.physical_corner_pos = __v); }
        PreviewRoom::load_secrets(&__this);
        __this
    }

    /// java: protected void loadSecrets()
    pub fn load_secrets(__this: &compat::Shared<Self>) {
        let mut room_waypoints: Option<Vec<RoomWaypoint>> = DungeonManager::get_room_waypoints(&compat::get_state(&room_preview_server::SELECTED_ROOM));
        if let Some(room_waypoints) = &room_waypoints {
            for waypoint in room_waypoints.iter() {
                let mut secret_name: String = waypoint.secret_name();
                let mut secret_index_matcher: compat::regex::Matcher = room::SECRET_INDEX.matcher(&secret_name);
                let mut secret_index: i32 = (if secret_index_matcher.find() { secret_index_matcher.group(1).unwrap().parse::<i32>().expect("NumberFormatException") } else { 0 });
                let mut pos: BlockPos = DungeonMapUtils::relative_to_actual(({ let __f = compat::dispatch(__this, |__t| __t.get_direction()); __f(&compat::view(__this, |a| a as compat::Shared<dyn Room>)) }).unwrap(), &({ let __f = compat::dispatch(__this, |__t| __t.get_physical_corner_pos()); __f(&compat::view(__this, |a| a as compat::Shared<dyn Room>)) }).unwrap(), waypoint);
                let mut secret_waypoint: compat::Shared<SecretWaypoint> = SecretWaypoint::new(secret_index, waypoint.category(), secret_name.clone(), pos.clone());
                // java: secretWaypoints.put(secretIndex, pos, secretWaypoint);
                { let __w = secret_waypoint.clone(); compat::write(__this, |__t| __t.room_base_mut().secret_waypoints.put(secret_index, pos.clone(), __w)); }
                PreviewRoom::show_secret_in_level(__this, &secret_waypoint);
            }
        }
        let mut custom_waypoints: indexmap::IndexMap<BlockPos, compat::Shared<SecretWaypoint>> = DungeonManager::get_custom_waypoints(&compat::get_state(&room_preview_server::SELECTED_ROOM));
        for (pos, secret_waypoint) in custom_waypoints.iter() {
            // java: customWaypoints.forEach((pos, secretWaypoint) -> secretWaypoints.put(secretWaypoint.secretIndex, pos, secretWaypoint));
            { let (__i, __w) = (compat::read(secret_waypoint, |__t| __t.secret_index), secret_waypoint.clone()); compat::write(__this, |__t| __t.room_base_mut().secret_waypoints.put(__i, pos.clone(), __w)); }
        }
    }

    /// java: protected void showSecretInLevel(SecretWaypoint waypoint)
    pub fn show_secret_in_level(__this: &compat::Shared<Self>, waypoint: &compat::Shared<SecretWaypoint>) {
        // java: IntegratedServer server = CLIENT.getSingleplayerServer(); if (server == null) return;
        // GAP (verified, RustSrc NEXT_STEPS 000rr): the port has no integrated server, so this is always null.
        // NEEDS-GAME-API: Minecraft::get_singleplayer_server / IntegratedServer::overworld
        let server: Option<crate::unresolved::IntegratedServer> = None;
        let Some(_server) = server else {
            return;
        };
        let mut block: Option<Arc<Block>> = match compat::read(waypoint, |__t| __t.category.clone()) {
            Some(crate::de::hysky::skyblocker::skyblock::dungeon::secrets::secret_waypoint::Category::Entrance) | Some(crate::de::hysky::skyblocker::skyblock::dungeon::secrets::secret_waypoint::Category::Stonk) | Some(crate::de::hysky::skyblocker::skyblock::dungeon::secrets::secret_waypoint::Category::Aotv) | Some(crate::de::hysky::skyblocker::skyblock::dungeon::secrets::secret_waypoint::Category::Pearl) | Some(crate::de::hysky::skyblocker::skyblock::dungeon::secrets::secret_waypoint::Category::Prince) | Some(crate::de::hysky::skyblocker::skyblock::dungeon::secrets::secret_waypoint::Category::Default) => None,
            Some(crate::de::hysky::skyblocker::skyblock::dungeon::secrets::secret_waypoint::Category::Bat) => Some(mcrust::net::minecraft::world::level::block::blocks::stained_glass().green().clone()),
            Some(crate::de::hysky::skyblocker::skyblock::dungeon::secrets::secret_waypoint::Category::Chest) => Some(blocks::blocks().get("CHEST")),
            Some(crate::de::hysky::skyblocker::skyblock::dungeon::secrets::secret_waypoint::Category::Fairysoul) => Some(mcrust::net::minecraft::world::level::block::blocks::stained_glass().pink().clone()),
            Some(crate::de::hysky::skyblocker::skyblock::dungeon::secrets::secret_waypoint::Category::Item) => Some(mcrust::net::minecraft::world::level::block::blocks::stained_glass().blue().clone()),
            Some(crate::de::hysky::skyblocker::skyblock::dungeon::secrets::secret_waypoint::Category::Lever) => Some(blocks::blocks().get("LEVER")),
            Some(crate::de::hysky::skyblocker::skyblock::dungeon::secrets::secret_waypoint::Category::Superboom) => Some(blocks::blocks().get("TNT")),
            Some(crate::de::hysky::skyblocker::skyblock::dungeon::secrets::secret_waypoint::Category::Wither) | Some(crate::de::hysky::skyblocker::skyblock::dungeon::secrets::secret_waypoint::Category::RedstoneKey) => Some(blocks::blocks().get("PLAYER_HEAD")),
            _ => unreachable!("switch expression: no case matched"),
        };
        let Some(mut block) = block.clone() else {
            return;
        };
        let mut state: Arc<BlockState> = block.default_block_state();
        // java: server.execute(() -> { level.setBlock(waypoint.pos, state, Block.UPDATE_ALL); level.updateNeighboursOnBlockSet(waypoint.pos, state); });
        // NEEDS-GAME-API: IntegratedServer::execute / ServerLevel::set_block / update_neighbours_on_block_set (NEXT_STEPS 000rr)
        let _ = state;
    }

}

impl Room for PreviewRoom {
    fn room_base(&self) -> &RoomBase {
        &self.base
    }
    fn room_base_mut(&mut self) -> &mut RoomBase {
        &mut self.base
    }

    /// java: @Override protected Direction[] getPossibleDirections(IntSortedSet segmentsX, IntSortedSet segmentsY) (dispatch: the body for this class)
    fn get_possible_directions(&self) -> fn(&compat::Shared<dyn Room>, &std::collections::BTreeSet<i32>, &std::collections::BTreeSet<i32>) -> Vec<Direction> {
        PreviewRoom::get_possible_directions_dyn
    }

    /// java: @Override protected void onUseBlock(Level level, BlockPos pos) (dispatch: the body for this class)
    fn on_use_block(&self) -> fn(&compat::Shared<dyn Room>, &Level, BlockPos) {
        PreviewRoom::on_use_block_dyn
    }

}

impl Tickable for PreviewRoom {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @SuppressWarnings("JavadocReference") @Override public void tick(Minecraft client) (dispatch: the body for this class)
    fn tick(&self) -> fn(&compat::Shared<dyn Tickable>, &mut mcrust::net::minecraft::client::minecraft::Minecraft) {
        <dyn Room>::tick_from::<Self>
    }

}

impl Renderable for PreviewRoom {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void extractRendering(PrimitiveCollector collector) (dispatch: the body for this class)
    fn extract_rendering(&self) -> fn(&compat::Shared<dyn Renderable>, &compat::Shared<dyn PrimitiveCollector>) {
        <dyn Room>::extract_rendering_from::<Self>
    }

}

impl PreviewRoom {
    /// java: @Override protected Direction[] getPossibleDirections(IntSortedSet segmentsX, IntSortedSet segmentsY)
    pub fn get_possible_directions_impl(__this: &compat::Shared<Self>, segments_x: &std::collections::BTreeSet<i32>, segments_y: &std::collections::BTreeSet<i32>) -> Vec<Direction> {
        return vec![Direction::Nw];
    }

    /// PreviewRoom.getPossibleDirections as the dispatch of Room.getPossibleDirections
    pub fn get_possible_directions_dyn(__this: &compat::Shared<dyn Room>, __p0: &std::collections::BTreeSet<i32>, __p1: &std::collections::BTreeSet<i32>) -> Vec<Direction> {
        Self::get_possible_directions_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1)
    }

    /// java: @Override protected void onUseBlock(Level level, BlockPos pos)
    pub fn on_use_block_impl(__this: &compat::Shared<Self>, level: &Level, mut pos: BlockPos) {
        if !level.is_client_side_level() {
            return;
        }
        <dyn Room>::on_use_block_body(&compat::view(__this, |a| a as compat::Shared<dyn Room>), level, pos.clone());
    }

    /// PreviewRoom.onUseBlock as the dispatch of Room.onUseBlock
    pub fn on_use_block_dyn(__this: &compat::Shared<dyn Room>, __p0: &Level, __p1: BlockPos) {
        Self::on_use_block_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1)
    }

}
