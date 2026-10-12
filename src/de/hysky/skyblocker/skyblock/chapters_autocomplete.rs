//! Ported from de.hysky.skyblocker.skyblock.ChaptersAutocomplete by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::entry::skyblocker_mod;
use mcrust::com::mojang::brigadier::builder::argument_builder::ArgumentBuilder as _;
use mcrust::com::mojang::brigadier::arguments::string_argument_type::StringArgumentType;
use mcrust::com::mojang::brigadier::command::Command;
use mcrust::com::mojang::brigadier::context::command_context::CommandContext;
use mcrust::com::mojang::brigadier::suggestion::suggestions_builder::SuggestionsBuilder;
use mcrust::com::mojang::brigadier::tree::command_node::CommandNode;
use mcrust::mcrust::modding::events::client_command_registration_callback::FabricClientCommandSource;
use mcrust::net::minecraft::commands::shared_suggestion_provider;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::com::mojang::serialization::codecs::record_codec_builder;
use mcrust::java::util::concurrent::completable_future::CompletableFuture;
use mcrust::com::mojang::serialization::decoder::Decoder as _;

/// java: private static List<String> locations = List.of();
pub static LOCATIONS: LazyLock<Mutex<Vec<String>>> = LazyLock::new(|| Mutex::new(vec![]));
/// java: public static @Nullable LiteralCommandNode<FabricClientCommandSource> singularCommandNode;
/// (written by the async loader, read by `ClientboundCommandsPacketMixin`: a thread-safe cell)
pub static SINGULAR_COMMAND_NODE: LazyLock<Mutex<Option<ClientCommandNode>>> = LazyLock::new(|| Mutex::new(None));
/// java: public static @Nullable LiteralCommandNode<FabricClientCommandSource> pluralCommandNode;
/// (written by the async loader, read by `ClientboundCommandsPacketMixin`: a thread-safe cell)
pub static PLURAL_COMMAND_NODE: LazyLock<Mutex<Option<ClientCommandNode>>> = LazyLock::new(|| Mutex::new(None));
//   java: private static final Codec<ChaptersData> CODEC = RecordCodecBuilder.create(instance -> instance.group(
//   java: 				Codec.STRING.listOf().fieldOf("locations").forGetter(ChaptersData::locations)
//   java: 		).apply(instance, ChaptersData::new));
/// java: private static final Codec<ChaptersData> CODEC = RecordCodecBuilder.create(instance -> instance.group( Codec.STRING.listOf().fieldOf("locations").forGetter(ChaptersData::locations) ).apply(instance, C
/// (read on the executor's thread: a codec is `Send + Sync`, so a `LazyLock`)
pub static CHAPTERS_DATA_CODEC: LazyLock<CodecRef<ChaptersData>> = LazyLock::new(|| record_codec_builder::create(record_codec_builder::apply1(record_codec_builder::of(|o: &ChaptersData| o.locations(), mcrust::com::mojang::serialization::codec::field_of(mcrust::com::mojang::serialization::codec::list(mcrust::com::mojang::serialization::codec::string_codec()), "locations")), move |locations| ChaptersData::new(locations))));

/// java: public final class ChaptersAutocomplete
#[derive(Clone)]
pub struct ChaptersAutocomplete {}

impl ChaptersAutocomplete {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        CompletableFuture::<()>::run_async(move || (|| {
            // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
            let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                //   java: String json = Http.sendGetRequest("https://hysky.de/api/chapters");
                let mut json: String = crate::de::hysky::skyblocker::utils::http::Http::send_get_request("https://hysky.de/api/chapters");
                let mut data: ChaptersData = (CHAPTERS_DATA_CODEC.parse(&mcrust::com::mojang::serialization::json_ops::JsonOps::instance(), &compat::json_to_port(&serde_json::from_str::<serde_json::Value>(&json).expect("JsonSyntaxException")))).get_or_throw().unwrap();
                compat::set_state(&LOCATIONS, data.locations.clone());
                compat::set_state(&SINGULAR_COMMAND_NODE, Some(ChaptersAutocomplete::create_command_node("chapter")));
                compat::set_state(&PLURAL_COMMAND_NODE, Some(ChaptersAutocomplete::create_command_node("chapters")));
            }));
            if let Err(e) = __result {
                logger().error(&format!("[Skyblocker Chapters Autocomplete] Failed to load Chapters data..."));
            }
        })(), compat::executor);
    }

    /// java: private static LiteralCommandNode<FabricClientCommandSource> createCommandNode(String command)
    pub fn create_command_node(command: &str) -> ClientCommandNode {
        //   java: return literal(command)
        //   java: 				.requires(_ -> Utils.isOnSkyblock())
        //   java: 				.executes(CommandUtils.noOp)
        //   java: 				.then(argument("island", StringArgumentType.string())
        //   java: 						.suggests((_, builder) -> SharedSuggestionProvider.suggest(locations, builder))
        //   java: 						.executes(CommandUtils.noOp)
        //   java: 				).build();
        let mut island = client_command_registration_callback::argument("island", Arc::new(StringArgumentType::string()));
        island.suggests(Some(Arc::new(|_: &CommandContext<FabricClientCommandSource>, builder: &mut SuggestionsBuilder| Ok(shared_suggestion_provider::suggest(&compat::get_state(&LOCATIONS), builder)))));
        island.executes(no_op());
        let mut node = client_command_registration_callback::literal(command);
        node.requires(Arc::new(|_: &FabricClientCommandSource| crate::de::hysky::skyblocker::utils::utils::Utils::is_on_skyblock()));
        node.executes(no_op());
        node.then(&island);
        return node.build();
    }

}

/// java: private record ChaptersData(List<String> locations)
#[derive(Clone, Debug, PartialEq)]
pub struct ChaptersData {
    /// java: List<String> locations
    pub locations: Vec<String>,
}

impl ChaptersData {
    /// java: the canonical record constructor
    pub fn new(mut locations: Vec<String>) -> Self {
        Self { locations }
    }

    /// java: the record accessor `locations()`
    pub fn locations(&self) -> Vec<String> {
        self.locations.clone()
    }

}

/// `LiteralCommandNode<FabricClientCommandSource>`: the port's `ArgumentBuilder.build()` answers the node as a
/// `CommandNode` (its `NodeKind::Literal`), the type `ClientboundCommandsPacketMixin` hands back.
pub type ClientCommandNode = Arc<CommandNode<FabricClientCommandSource>>;

/// `CommandUtils.noOp` (`_ -> -1`). The nodes are built on the executor's thread and `command_utils::NO_OP` is a
/// render-thread cell (`compat::MainThread`), so the command is made here, the same body.
fn no_op() -> Command<FabricClientCommandSource> {
    Arc::new(|_: &CommandContext<FabricClientCommandSource>| Ok(-1))
}
