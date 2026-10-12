//! Ported from de.hysky.skyblocker.RoomDataTest by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::dungeon_manager;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::dungeon_manager::DungeonManager;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::server::packs::resources::resource_provider::ResourceProvider as _;

/// java: @SuppressWarnings("UnstableApiUsage") public class RoomDataTest implements FabricClientGameTest
#[derive(Clone)]
pub struct RoomDataTest {}

impl RoomDataTest {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Override public void runTest(ClientGameTestContext clientGameTestContext)
    pub fn run_test(&self, client_game_test_context: &crate::unresolved::ClientGameTestContext) {
        // TODO(fabric2seer): `.waitFor(..)` on a ClientGameTestContext
        //   java: clientGameTestContext.waitFor(_ -> DungeonManager.isRoomsLoaded());
        // TODO(fabric2seer): `.waitTicks(..)` on a ClientGameTestContext
        //   java: clientGameTestContext.waitTicks(100);
        // TODO(fabric2seer): `.runOnClient(..)` on a ClientGameTestContext
        //   java: clientGameTestContext.runOnClient(this::testMain);
    }

    /// java: public void testMain(Minecraft client)
    pub fn test_main(&self, client: &mut mcrust::net::minecraft::client::minecraft::Minecraft) {
        let mut skeleton_files: Vec<String> = self.get_room_files_by_type(client, ".skeleton");
        let mut room_files: Vec<String> = self.get_room_files_by_type(client, ".json");
        logger().info(&format!("Found {} .skeleton files and {} .json files!", (skeleton_files.len() as i32), (room_files.len() as i32)));
        if !(self.check_rooms(&skeleton_files, &room_files)) {
            panic!("AssertionError: {}", "There are missing .skeleton or .json files!");
        }
        if !(self.check_room_json(client)) {
            panic!("AssertionError: {}", "There are invalid room .json files!");
        }
        self.check_for_duplicate_names(&skeleton_files);
        self.check_if_loaded_correctly((skeleton_files.len() as i32));
    }

    /// java: public boolean checkRooms(List<String> skeletonFiles, List<String> roomFiles)
    pub fn check_rooms(&self, skeleton_files: &Vec<String>, room_files: &Vec<String>) -> bool {
        let mut is_valid: bool = true;
        for room_name in skeleton_files.iter() {
            if !(room_files.iter().any(|e| *e == room_name.to_string())) {
                is_valid = false;
                logger().error(&format!("{} is missing a .json file!", room_name));
            }
        }
        for room_name in room_files.iter() {
            if !(skeleton_files.iter().any(|e| *e == room_name.to_string())) {
                is_valid = false;
                logger().error(&format!("{} is missing a .skeleton file!", room_name));
            }
        }
        return is_valid;
    }

    /// java: public boolean checkRoomJson(Minecraft client)
    pub fn check_room_json(&self, client: &mut mcrust::net::minecraft::client::minecraft::Minecraft) -> bool {
        let mut is_valid: bool = true;
        for file_path in self.get_room_json(client) {
            {
                let mut reader: crate::unresolved::BufferedReader = client.get_resource_manager().open_as_reader(&file_path).unwrap();
                // java: try {..} catch (Exception ex) - an exception here is a panic, caught by `catch_unwind`
                let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    // TODO(fabric2seer): static call JsonParser.parseReader(..)
                    //   java: DungeonManager.RoomData.CODEC.parse(JsonOps.INSTANCE, JsonParser.parseReader(reader)).getOrThrow();
                }));
                if let Err(ex) = __result {
                    let mut parts: Vec<String> = compat::regex::split(&file_path.path(), "/");
                    logger().error(&format!("Failed to load room: dungeon={}, shape={}, room={}", parts[(1) as usize], parts[(2) as usize], parts[(3) as usize]));
                    is_valid = false;
                }
            }
        }
        return is_valid;
    }

    /// java: public void checkForDuplicateNames(List<String> skeletonFiles)
    pub fn check_for_duplicate_names(&self, skeleton_files: &Vec<String>) {
        let mut temp: std::collections::HashSet<String> = std::collections::HashSet::new();
        for name in skeleton_files.iter() {
            if !(temp.insert(name.to_string())) {
                panic!("AssertionError: {}", format!("Duplicate room name (id): {}", name).to_string());
            }
        }
    }

    /// java: public void checkIfLoadedCorrectly(int expected)
    pub fn check_if_loaded_correctly(&self, mut expected: i32) {
        let mut count: i32 = DungeonManager::get_loaded_room_count();
        if count != expected {
            panic!("AssertionError: {}", format!("Expected {} room(s) but only {} room(s) loaded", expected, count).to_string());
        }
    }

    /// java: List<String> getRoomFilesByType(Minecraft client, String fileType)
    pub fn get_room_files_by_type(&self, client: &mut mcrust::net::minecraft::client::minecraft::Minecraft, file_type: &str) -> Vec<String> {
        return (client.get_resource_manager().list_resources(dungeon_manager::DUNGEONS_PATH, &|id| id.path().ends_with(file_type))).keys().cloned().map(|identifier| compat::regex::split(&identifier.path(), "/")).filter(|path| ((path.len() as i32) == 4)).map(|path| (path[(3) as usize]).replace(file_type, "")).collect::<Vec<_>>();
    }

    /// java: List<Identifier> getRoomJson(Minecraft client)
    pub fn get_room_json(&self, client: &mut mcrust::net::minecraft::client::minecraft::Minecraft) -> Vec<Identifier> {
        return (client.get_resource_manager().list_resources(dungeon_manager::DUNGEONS_PATH, &|id| (((compat::regex::split(&id.path(), "/")).len() as i32) == 4 && id.path().ends_with(".json")))).keys().cloned().collect::<Vec<_>>();
    }

}
