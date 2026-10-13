//! Ported from de.hysky.skyblocker.skyblock.CallAutocomplete by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::neu_repo_manager::NEURepoManager;
use crate::de::hysky::skyblocker::skyblock::viewstash_autocomplete::ClientCommandNode;
use crate::de::hysky::skyblocker::utils::command::command_utils::no_op;
use mcrust::java::text::decimal_format_symbols::Locale;

/// java: public static @Nullable LiteralCommandNode<FabricClientCommandSource> commandNode;
/// (set on the executor's thread once the NEU repo loaded, read on the network thread: a `LazyLock<Mutex<..>>`)
pub static COMMAND_NODE: LazyLock<Mutex<Option<ClientCommandNode>>> = LazyLock::new(|| Mutex::new(None));

/// java: public class CallAutocomplete
#[derive(Clone)]
pub struct CallAutocomplete {}

impl CallAutocomplete {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        NEURepoManager::run_async_after_load(Arc::new(move || { CallAutocomplete::load_call_names(); }));
    }

    /// java: private static void loadCallNames()
    pub fn load_call_names() {
        let suggestions: Vec<String> = Vec::new();
        //   java: for (Map.Entry<String, AbiphoneContact> entry : NEURepoManager.getConstants().getAbiphoneContacts().entrySet()) {
        //   java: 			suggestions.addAll(computeCallNames(entry.getKey(), entry.getValue().getCallNames()));
        //   java: 		}
        // rust: port gap (NEU repo library) - `NEUConstants.getAbiphoneContacts()` is not ported (crate::unresolved::NEUConstants),
        // so the contacts add no names
        //   java: commandNode = literal("call")
        //   java: 				.requires(_ -> Utils.isOnSkyblock())
        //   java: 				.executes(CommandUtils.noOp)
        //   java: 				.then(argument("contact", StringArgumentType.greedyString())
        //   java: 						.suggests((_, builder) -> SharedSuggestionProvider.suggest(suggestions, builder))
        //   java: 						.executes(CommandUtils.noOp))
        //   java: 				.build();
        use mcrust::com::mojang::brigadier::arguments::string_argument_type::StringArgumentType;
        use mcrust::com::mojang::brigadier::builder::argument_builder::ArgumentBuilder as _;
        use mcrust::com::mojang::brigadier::context::command_context::CommandContext;
        use mcrust::com::mojang::brigadier::suggestion::suggestions_builder::SuggestionsBuilder;
        use mcrust::mcrust::modding::events::client_command_registration_callback::{self, FabricClientCommandSource};
        let suggestions = Arc::new(suggestions);
        let mut contact = client_command_registration_callback::argument("contact", Arc::new(StringArgumentType::greedy_string()));
        contact.suggests(Some(Arc::new(move |_: &CommandContext<FabricClientCommandSource>, builder: &mut SuggestionsBuilder| Ok(mcrust::net::minecraft::commands::shared_suggestion_provider::suggest(&suggestions, builder)))));
        contact.executes(no_op());
        let mut node = client_command_registration_callback::literal("call");
        node.requires(Arc::new(|_: &FabricClientCommandSource| crate::de::hysky::skyblocker::utils::utils::Utils::is_on_skyblock()));
        node.executes(no_op());
        node.then(&contact);
        compat::set_state(&COMMAND_NODE, Some(node.build()));
    }

    /// java: private static List<String> computeCallNames(String npcName, @Nullable List<String> callNameOverrides)
    pub fn compute_call_names(npc_name: &str, mut call_name_overrides: Option<Vec<String>>) -> Vec<String> {
        return (if call_name_overrides.is_some() { call_name_overrides.clone() } else { Some(vec![CallAutocomplete::format_default_call_name(npc_name)]) }).unwrap();
    }

    /// java: private static String formatDefaultCallName(String npcName)
    pub fn format_default_call_name(npc_name: &str) -> String {
        return npc_name.to_lowercase();
    }

}
