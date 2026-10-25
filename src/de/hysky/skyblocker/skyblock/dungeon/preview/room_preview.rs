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
                //   java: dispatcher.register(ClientCommands.literal(NAMESPACE).then(ClientCommands.literal("dungeons").then(ClientCommands.literal("preview").then(ClientCommands.literal("loadRoom")
                //   java:     .then(argument("type", StringArgumentType.string()).suggests(DungeonManager::suggestRoomTypes)
                //   java:         .then(argument("room", StringArgumentType.string()).suggests((ctx, sB) -> DungeonManager.suggestRooms(ctx.getArgument("type", String.class), sB))
                //   java:             .executes(RoomPreview::startPreview)))))));
                use mcrust::com::mojang::brigadier::arguments::string_argument_type::StringArgumentType;
                use mcrust::com::mojang::brigadier::builder::argument_builder::ArgumentBuilder as _;
                use crate::prelude::client_command_registration_callback as ccr;
                type Ctx = CommandContext<ccr::FabricClientCommandSource>;
                let mut room = ccr::argument("room", Arc::new(StringArgumentType::string()));
                room.suggests(Some(Arc::new(|ctx: &Ctx, builder: &mut mcrust::com::mojang::brigadier::suggestion::suggestions_builder::SuggestionsBuilder| {
                    let room_type: String = ctx.get_argument::<String>("type").cloned().unwrap_or_default();
                    Ok(DungeonManager::suggest_rooms(&room_type, builder))
                })));
                room.executes(Arc::new(|ctx: &Ctx| Ok(RoomPreview::start_preview(ctx))));
                let mut room_type = ccr::argument("type", Arc::new(StringArgumentType::string()));
                room_type.suggests(Some(Arc::new(|ctx: &Ctx, builder: &mut mcrust::com::mojang::brigadier::suggestion::suggestions_builder::SuggestionsBuilder| Ok(DungeonManager::suggest_room_types(ctx, builder)))));
                room_type.then(&room);
                let mut load_room = ccr::literal("loadRoom");
                load_room.then(&room_type);
                let mut preview = ccr::literal("preview");
                preview.then(&load_room);
                let mut dungeons = ccr::literal("dungeons");
                dungeons.then(&preview);
                let mut root = ccr::literal(crate::entry::skyblocker_mod::NAMESPACE);
                root.then(&dungeons);
                dispatcher.dispatcher().register(&root);
            },
        ));
    }

    /// java: private static int startPreview(CommandContext<FabricClientCommandSource> ctx)
    pub fn start_preview(ctx: &CommandContext<crate::prelude::client_command_registration_callback::FabricClientCommandSource>) -> i32 {
        if DungeonManager::get_room_block_data(ctx.get_argument::<String>("type").as_deref().unwrap(), ctx.get_argument::<String>("room").as_deref().unwrap()).is_none() {
            // java: ctx.getSource().sendFeedback(Component.translatable("skyblocker.dungeons.roomPreview.invalidRoom").withStyle(ChatFormatting.RED));
            compat::batch_w_send_feedback(&mcrust::net::minecraft::network::chat::component::translatable("skyblocker.dungeons.roomPreview.invalidRoom").with_style_format(mcrust::net::minecraft::chat_formatting::ChatFormatting::Red));
            return -1;
        }
        // java: CLIENT.disconnectFromWorld(ClientLevel.DEFAULT_QUIT_MESSAGE); - `game::disconnect_from_world` parks it for the
        // loop, with the game's own quit message (`multiplayer.status.quitting`, `ClientLevel.DEFAULT_QUIT_MESSAGE`)
        mcrust::mcrust::modding::game::disconnect_from_world();
        // TODO(fabric2seer): `Minecraft.getSingleplayerServer` has no Rust method in the port (looked for `get_singleplayer_server` in Minecraft and its supertypes)
        if compat::minecraft().has_singleplayer_server() && true /* java: compat::minecraft().get_singleplayer_server() != null: never null here */ {
            // TODO(fabric2seer): `Minecraft.getSingleplayerServer` has no Rust method in the port (looked for `get_singleplayer_server` in Minecraft and its supertypes)
            // TODO(fabric2seer): `IntegratedServer.stopServer` has no Rust method in the port (looked for `stop_server` in IntegratedServer and its supertypes)
            // java: CLIENT.getSingleplayerServer().stopServer();
            // NEEDS-GAME-API: IntegratedServer::stop_server (no integrated server, NEXT_STEPS 000rr; `has_singleplayer_server` is always false)
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
