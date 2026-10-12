//! Ported from de.hysky.skyblocker.skyblock.dungeon.preview.RoomPreview by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::dungeon::preview::preview_room::PreviewRoom;
use crate::de::hysky::skyblocker::skyblock::dungeon::preview::room_preview_server;
use crate::de::hysky::skyblocker::skyblock::dungeon::preview::room_preview_server::RoomPreviewServer;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::dungeon_manager::DungeonManager;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::room::Room;
use mcrust::com::mojang::brigadier::context::command_context::CommandContext;
use mcrust::net::minecraft::client::minecraft;
use mcrust::net::minecraft::client::multiplayer::client_level::ClientLevel;

/// java: public class RoomPreview
#[derive(Clone)]
pub struct RoomPreview {}

impl RoomPreview {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        client_command_registration_callback::EVENT.register(Arc::new(
            |dispatcher: &mut client_command_registration_callback::ClientCommandDispatcher| {
                // TODO(fabric2seer): a command tree this tool cannot read (arguments, redirects, requires) - SeerClient commands receive the words after the name in `ctx.args`
                //   java: dispatcher.register(..);
            },
        ));
    }

    /// java: private static int startPreview(CommandContext<FabricClientCommandSource> ctx)
    pub fn start_preview(ctx: &CommandContext<crate::unresolved::FabricClientCommandSource>) -> i32 {
        if DungeonManager::get_room_block_data(ctx.get_argument::<String>("type").as_deref().unwrap(), ctx.get_argument::<String>("room").as_deref().unwrap()).is_none() {
            // TODO(fabric2seer): `.sendFeedback(..)` on a FabricClientCommandSource
            //   java: ctx.getSource().sendFeedback(Component.translatable("skyblocker.dungeons.roomPreview.invalidRoom").withStyle(ChatFormatting.RED));
            return -1;
        }
        // TODO(fabric2seer): static `ClientLevel.DEFAULT_QUIT_MESSAGE` has no Rust counterpart in the port's ClientLevel file
        // TODO(fabric2seer): `disconnect_from_world` takes 12 argument(s) in the port (`gfx: &mut Minecraft, connection: &mut Option< Box<crate::net::minecraft::client::gui::screens::connect_screen::ConnectScreenState>, >, player: &mut Option< std::sync::Arc< std::sync::Mutex<crate::net::minecraft::client::player::local_player ..`), Java passes 1
        // rust: minecraft::disconnect_from_world(&mut ClientLevel::DEFAULT_QUIT_MESSAGE);
        // TODO(fabric2seer): `Minecraft.getSingleplayerServer` has no Rust method in the port (looked for `get_singleplayer_server` in Minecraft and its supertypes)
        if compat::minecraft().has_singleplayer_server() && true /* java: compat::minecraft().get_singleplayer_server() != null: never null here */ {
            // TODO(fabric2seer): `Minecraft.getSingleplayerServer` has no Rust method in the port (looked for `get_singleplayer_server` in Minecraft and its supertypes)
            // TODO(fabric2seer): `IntegratedServer.stopServer` has no Rust method in the port (looked for `stop_server` in IntegratedServer and its supertypes)
            // rust: compat::minecraft().get_singleplayer_server().stop_server();
        }
        RoomPreviewServer::create_server();
        RoomPreviewServer::load_room(ctx.get_argument::<String>("type").unwrap(), ctx.get_argument::<String>("room").unwrap().clone());
        return 1;
    }

    /// java: static void onJoin()
    pub fn on_join() {
        let mut preview_room: compat::Shared<PreviewRoom> = PreviewRoom::new(compat::get_state(&room_preview_server::SELECTED_ROOM));
        DungeonManager::start_from_room_preview((preview_room.clone() as compat::Shared<dyn Room>));
    }

}
