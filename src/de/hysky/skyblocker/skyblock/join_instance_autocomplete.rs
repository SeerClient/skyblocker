//! Ported from de.hysky.skyblocker.skyblock.JoinInstanceAutocomplete by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::command::command_utils::no_op;
use crate::entry::skyblocker_mod;
use mcrust::com::mojang::brigadier::builder::argument_builder::ArgumentBuilder as _;
use mcrust::com::mojang::brigadier::arguments::string_argument_type::StringArgumentType;
use mcrust::com::mojang::brigadier::command::Command;
use mcrust::com::mojang::brigadier::context::command_context::CommandContext;
use mcrust::com::mojang::brigadier::suggestion::suggestions_builder::SuggestionsBuilder;
use mcrust::com::mojang::brigadier::tree::command_node::CommandNode;
use mcrust::mcrust::modding::events::client_command_registration_callback::FabricClientCommandSource;
use mcrust::net::minecraft::commands::shared_suggestion_provider;
use mcrust::java::util::concurrent::completable_future::CompletableFuture;

/// java: public static @Nullable LiteralCommandNode<FabricClientCommandSource> joinInstanceCommand;
/// (written by the async loader, read by `ClientboundCommandsPacketMixin`: a thread-safe cell)
pub static JOIN_INSTANCE_COMMAND: LazyLock<Mutex<Option<ClientCommandNode>>> = LazyLock::new(|| Mutex::new(None));
/// java: public static @Nullable LiteralCommandNode<FabricClientCommandSource> dungeonCommand;
/// (written by the async loader, read by `ClientboundCommandsPacketMixin`: a thread-safe cell)
pub static DUNGEON_COMMAND: LazyLock<Mutex<Option<ClientCommandNode>>> = LazyLock::new(|| Mutex::new(None));
/// java: public static @Nullable LiteralCommandNode<FabricClientCommandSource> kuudraCommand;
/// (written by the async loader, read by `ClientboundCommandsPacketMixin`: a thread-safe cell)
pub static KUUDRA_COMMAND: LazyLock<Mutex<Option<ClientCommandNode>>> = LazyLock::new(|| Mutex::new(None));
/// java: private static Map<String, String> instanceMap = Map.of();
pub static INSTANCE_MAP: LazyLock<Mutex<std::collections::HashMap<String, String>>> = LazyLock::new(|| Mutex::new(std::collections::HashMap::from([])));

/// java: public class JoinInstanceAutocomplete
#[derive(Clone)]
pub struct JoinInstanceAutocomplete {}

impl JoinInstanceAutocomplete {
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
                //   java: String json = Http.sendGetRequest("https://hysky.de/api/joininstances");
                let mut json: String = crate::de::hysky::skyblocker::utils::http::Http::send_get_request("https://hysky.de/api/joininstances");
                let mut obj: serde_json::Map<String, serde_json::Value> = compat::json::get_as_json_object(&(serde_json::from_str::<serde_json::Value>(&json).expect("JsonSyntaxException")));
                //   java: instanceMap = obj.entrySet().stream().collect(Collectors.toMap(Map.Entry::getKey, e -> e.getValue().getAsString()));
                compat::set_state(&INSTANCE_MAP, obj.iter().map(|(key, value)| (key.clone(), compat::json::get_as_string(value))).collect::<std::collections::HashMap<String, String>>());
                compat::set_state(&JOIN_INSTANCE_COMMAND, Some(JoinInstanceAutocomplete::build_command("joininstance", Arc::new(move |_: &str| true))));
                compat::set_state(&DUNGEON_COMMAND, Some(JoinInstanceAutocomplete::build_command("joindungeon", Arc::new(move |s: &str| compat::equals_ignore_case(&compat::lock(&INSTANCE_MAP).get(s).cloned().unwrap(), "Catacombs")))));
                compat::set_state(&KUUDRA_COMMAND, Some(JoinInstanceAutocomplete::build_command("joinkuudra", Arc::new(move |s: &str| compat::equals_ignore_case(&compat::lock(&INSTANCE_MAP).get(s).cloned().unwrap(), "Kuudra")))));
            }));
            if let Err(e) = __result {
                logger().error(&format!("[Skyblocker] Failed to load joininstance list"));
            }
        })(), compat::executor);
    }

    /// java: private static LiteralCommandNode<FabricClientCommandSource> buildCommand(String command, java.util.function.Predicate<String> filter)
    pub fn build_command(command: &str, filter: Arc<dyn Fn(&str) -> bool + Send + Sync>) -> ClientCommandNode {
        //   java: return literal(command)
        //   java: 				.requires(_ -> Utils.isOnSkyblock())
        //   java: 				.executes(CommandUtils.noOp)
        //   java: 				.then(argument("instance", StringArgumentType.word())
        //   java: 						.suggests((_, builder) -> SharedSuggestionProvider.suggest(
        //   java: 								instanceMap.keySet().stream().filter(filter).sorted(),
        //   java: 								builder))
        //   java: 						.executes(CommandUtils.noOp))
        //   java: 				.build();
        let mut instance = client_command_registration_callback::argument("instance", Arc::new(StringArgumentType::word()));
        instance.suggests(Some(Arc::new(move |_: &CommandContext<FabricClientCommandSource>, builder: &mut SuggestionsBuilder| {
            let keys: Vec<String> = compat::lock(&INSTANCE_MAP).keys().cloned().collect();
            // `.sorted()` - `String.compareTo`
            let mut instances: Vec<String> = keys.into_iter().filter(|key| filter(key.as_str())).collect();
            instances.sort_by(|a, b| compat::compare_to(a, b).cmp(&0));
            Ok(shared_suggestion_provider::suggest(&instances, builder))
        })));
        instance.executes(no_op());
        let mut node = client_command_registration_callback::literal(command);
        node.requires(Arc::new(|_: &FabricClientCommandSource| crate::de::hysky::skyblocker::utils::utils::Utils::is_on_skyblock()));
        node.executes(no_op());
        node.then(&instance);
        return node.build();
    }

}

/// `LiteralCommandNode<FabricClientCommandSource>`: the port's `ArgumentBuilder.build()` answers the node as a
/// `CommandNode` (its `NodeKind::Literal`), the type `ClientboundCommandsPacketMixin` hands back.
pub type ClientCommandNode = Arc<CommandNode<FabricClientCommandSource>>;

