//! Ported from de.hysky.skyblocker.skyblock.CallAutocomplete by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::neu_repo_manager::NEURepoManager;
use mcrust::com::mojang::brigadier::tree::literal_command_node::LiteralCommandNode;
use mcrust::java::text::decimal_format_symbols::Locale;

/// java: public static @Nullable LiteralCommandNode<FabricClientCommandSource> commandNode;
pub static COMMAND_NODE: compat::MainThread<Option<LiteralCommandNode>> = compat::MainThread::new(|| None);

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
        let mut suggestions: Vec<String> = Vec::new();
        // TODO(fabric2seer): `.getAbiphoneContacts(..)` on a NEUConstants
        //   java: for (Map.Entry<String, AbiphoneContact> entry : NEURepoManager.getConstants().getAbiphoneContacts().entrySet()) {
        //   java: 			String npcName = entry.getKey();
        //   java: 			List<String> callNameOverrides = entry.getValue().getCallNames();
        //   java:
        //   java: 			suggestions.addAll(computeCallNames(npcName, callNameOverrides));
        //   java: 		}
        // TODO(fabric2seer): a command builder outside dispatcher.register(..)
        //   java: commandNode = literal("call")
        //   java: 				.requires(_ -> Utils.isOnSkyblock())
        //   java: 				.executes(CommandUtils.noOp)
        //   java: 				.then(argument("contact", StringArgumentType.greedyString())
        //   java: 						.suggests((_, builder) -> SharedSuggestionProvider.suggest(suggestions, builder))
        //   java: 						.executes(CommandUtils.noOp))
        //   java: 				.build();
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
