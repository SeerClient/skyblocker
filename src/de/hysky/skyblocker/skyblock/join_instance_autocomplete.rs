//! Ported from de.hysky.skyblocker.skyblock.JoinInstanceAutocomplete by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::entry::skyblocker_mod;
use mcrust::com::mojang::brigadier::tree::literal_command_node::LiteralCommandNode;
use mcrust::java::util::concurrent::completable_future::CompletableFuture;

/// java: public static @Nullable LiteralCommandNode<FabricClientCommandSource> joinInstanceCommand;
pub static JOIN_INSTANCE_COMMAND: compat::MainThread<Option<LiteralCommandNode>> = compat::MainThread::new(|| None);
/// java: public static @Nullable LiteralCommandNode<FabricClientCommandSource> dungeonCommand;
pub static DUNGEON_COMMAND: compat::MainThread<Option<LiteralCommandNode>> = compat::MainThread::new(|| None);
/// java: public static @Nullable LiteralCommandNode<FabricClientCommandSource> kuudraCommand;
pub static KUUDRA_COMMAND: compat::MainThread<Option<LiteralCommandNode>> = compat::MainThread::new(|| None);
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
                // TODO(fabric2seer): `Http.sendGetRequest` is not a static method of Http
                //   java: String json = Http.sendGetRequest("https://hysky.de/api/joininstances");
                let mut json: String = todo!("fabric2seer: the initialiser of `json` (TODO above)");
                let mut obj: serde_json::Map<String, serde_json::Value> = compat::json::get_as_json_object(&(serde_json::from_str::<serde_json::Value>(&json).expect("JsonSyntaxException")));
                // TODO(fabric2seer): static field Map.Entry
                //   java: instanceMap = obj.entrySet().stream().collect(Collectors.toMap(Map.Entry::getKey, e -> e.getValue().getAsString()));
                *JOIN_INSTANCE_COMMAND.get_mut() = Some(JoinInstanceAutocomplete::build_command("joininstance", Arc::new(move |_| true)));
                *DUNGEON_COMMAND.get_mut() = Some(JoinInstanceAutocomplete::build_command("joindungeon", Arc::new(move |s| compat::equals_ignore_case(&compat::lock(&INSTANCE_MAP).get(&s).cloned().unwrap(), "Catacombs"))));
                *KUUDRA_COMMAND.get_mut() = Some(JoinInstanceAutocomplete::build_command("joinkuudra", Arc::new(move |s| compat::equals_ignore_case(&compat::lock(&INSTANCE_MAP).get(&s).cloned().unwrap(), "Kuudra"))));
            }));
            if let Err(e) = __result {
                logger().error(&format!("[Skyblocker] Failed to load joininstance list"));
            }
        })(), compat::executor);
    }

    /// java: private static LiteralCommandNode<FabricClientCommandSource> buildCommand(String command, java.util.function.Predicate<String> filter)
    pub fn build_command(command: &str, filter: Arc<dyn Fn(String) -> bool>) -> LiteralCommandNode {
        // TODO(fabric2seer): a command builder outside dispatcher.register(..)
        //   java: return literal(command)
        //   java: 				.requires(_ -> Utils.isOnSkyblock())
        //   java: 				.executes(CommandUtils.noOp)
        //   java: 				.then(argument("instance", StringArgumentType.word())
        //   java: 						.suggests((_, builder) -> SharedSuggestionProvider.suggest(
        //   java: 								instanceMap.keySet().stream().filter(filter).sorted(),
        //   java: 								builder))
        //   java: 						.executes(CommandUtils.noOp))
        //   java: 				.build();
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}
