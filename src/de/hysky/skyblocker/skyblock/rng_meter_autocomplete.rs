//! Ported from de.hysky.skyblocker.skyblock.RngMeterAutocomplete by fabric2seer.

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
use mcrust::java::util::concurrent::completable_future::CompletableFuture;
use mcrust::com::mojang::serialization::decoder::Decoder as _;

//   java: private static final Codec<Map<String, List<String>>> CODEC = Codec.unboundedMap(Codec.STRING, Codec.STRING.listOf());
/// java: private static final Codec<Map<String, List<String>>> CODEC = Codec.unboundedMap(Codec.STRING, Codec.STRING.listOf());
/// (read on the executor's thread: a codec is `Send + Sync`, so a `LazyLock`; the port's `unboundedMap` answers the
/// entries, collected into the map)
pub static CODEC: LazyLock<CodecRef<std::collections::HashMap<String, Vec<String>>>> = LazyLock::new(|| mcrust::com::mojang::serialization::codec::xmap(
    mcrust::com::mojang::serialization::codec::unbounded_map(mcrust::com::mojang::serialization::codec::string_codec(), mcrust::com::mojang::serialization::codec::list(mcrust::com::mojang::serialization::codec::string_codec())),
    |entries: Vec<(String, Vec<String>)>| entries.into_iter().collect::<std::collections::HashMap<String, Vec<String>>>(),
    |map: &std::collections::HashMap<String, Vec<String>>| map.iter().map(|(k, v)| (k.clone(), v.clone())).collect::<Vec<(String, Vec<String>)>>(),
));
/// java: private static Map<String, List<String>> rngMeters = new HashMap<>();
pub static RNG_METERS: LazyLock<Mutex<std::collections::HashMap<String, Vec<String>>>> = LazyLock::new(|| Mutex::new(std::collections::HashMap::new()));
/// java: public static @Nullable LiteralCommandNode<FabricClientCommandSource> longCommand;
/// (written by the async loader, read by `ClientboundCommandsPacketMixin`: a thread-safe cell)
pub static LONG_COMMAND: LazyLock<Mutex<Option<ClientCommandNode>>> = LazyLock::new(|| Mutex::new(None));
/// java: public static @Nullable LiteralCommandNode<FabricClientCommandSource> shortCommand;
/// (written by the async loader, read by `ClientboundCommandsPacketMixin`: a thread-safe cell)
pub static SHORT_COMMAND: LazyLock<Mutex<Option<ClientCommandNode>>> = LazyLock::new(|| Mutex::new(None));

/// java: public class RngMeterAutocomplete
#[derive(Clone)]
pub struct RngMeterAutocomplete {}

impl RngMeterAutocomplete {
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
                //   java: String json = Http.sendGetRequest("https://hysky.de/api/rngmeters");
                let mut json: String = crate::de::hysky::skyblocker::utils::http::Http::send_get_request("https://hysky.de/api/rngmeters");
                compat::set_state(&RNG_METERS, (CODEC.parse(&mcrust::com::mojang::serialization::json_ops::JsonOps::instance(), &compat::json_to_port(&serde_json::from_str::<serde_json::Value>(&json).expect("JsonSyntaxException")))).get_or_throw().unwrap());
                compat::set_state(&LONG_COMMAND, Some(RngMeterAutocomplete::create_command_node("rngmeter")));
                compat::set_state(&SHORT_COMMAND, Some(RngMeterAutocomplete::create_command_node("rng")));
            }));
            if let Err(e) = __result {
                logger().error(&format!("[Skyblocker RNG Meter Autocomplete] Failed to load RNG Meter data."));
            }
        })(), compat::executor);
    }

    /// java: private static LiteralCommandNode<FabricClientCommandSource> createCommandNode(String command)
    pub fn create_command_node(command: &str) -> ClientCommandNode {
        //   java: return literal(command)
        //   java: 				.requires(_ -> Utils.isOnSkyblock())
        //   java: 				.executes(CommandUtils.noOp)
        //   java: 				.then(argument("type", StringArgumentType.string())
        //   java: 						.suggests((_, builder) -> SharedSuggestionProvider.suggest(rngMeters.keySet(), builder))
        //   java: 						.executes(CommandUtils.noOp)
        //   java: 						.then(argument("subtype", StringArgumentType.string())
        //   java: 								.suggests((context, builder) -> SharedSuggestionProvider.suggest(rngMeters.getOrDefault(StringArgumentType.getString(context, "type"), List.of()), builder))
        //   java: 								.executes(CommandUtils.noOp)
        //   java: 						)
        //   java: 				).build();
        let mut subtype = client_command_registration_callback::argument("subtype", Arc::new(StringArgumentType::string()));
        subtype.suggests(Some(Arc::new(|context: &CommandContext<FabricClientCommandSource>, builder: &mut SuggestionsBuilder| {
            let r#type: String = StringArgumentType::get_string(context, "type");
            let subtypes: Vec<String> = compat::lock(&RNG_METERS).get(&r#type).cloned().unwrap_or_default();
            Ok(shared_suggestion_provider::suggest(&subtypes, builder))
        })));
        subtype.executes(no_op());
        let mut r#type = client_command_registration_callback::argument("type", Arc::new(StringArgumentType::string()));
        r#type.suggests(Some(Arc::new(|_: &CommandContext<FabricClientCommandSource>, builder: &mut SuggestionsBuilder| {
            let types: Vec<String> = compat::lock(&RNG_METERS).keys().cloned().collect();
            Ok(shared_suggestion_provider::suggest(&types, builder))
        })));
        r#type.executes(no_op());
        r#type.then(&subtype);
        let mut node = client_command_registration_callback::literal(command);
        node.requires(Arc::new(|_: &FabricClientCommandSource| crate::de::hysky::skyblocker::utils::utils::Utils::is_on_skyblock()));
        node.executes(no_op());
        node.then(&r#type);
        return node.build();
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
