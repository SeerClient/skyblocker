//! Ported from de.hysky.skyblocker.skyblock.AuctionBazaarAutocomplete by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::command::command_utils::no_op;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::searchoverlay::search_over_manager::SearchOverManager;
use crate::de::hysky::skyblocker::utils::neu_repo_manager::NEURepoManager;
use mcrust::com::mojang::brigadier::suggestion::suggestions::Suggestions;
use mcrust::com::mojang::brigadier::builder::argument_builder::ArgumentBuilder as _;
use mcrust::com::mojang::brigadier::arguments::string_argument_type::StringArgumentType;
use mcrust::com::mojang::brigadier::command::Command;
use mcrust::com::mojang::brigadier::context::command_context::CommandContext;
use mcrust::com::mojang::brigadier::suggestion::suggestions_builder::SuggestionsBuilder;
use mcrust::com::mojang::brigadier::tree::command_node::CommandNode;
use mcrust::mcrust::modding::events::client_command_registration_callback::FabricClientCommandSource;
use mcrust::net::minecraft::commands::shared_suggestion_provider;

/// java: public static @Nullable LiteralCommandNode<FabricClientCommandSource> ahsNode;
/// (written by the async loader, read by `ClientboundCommandsPacketMixin`: a thread-safe cell)
pub static AHS_NODE: LazyLock<Mutex<Option<ClientCommandNode>>> = LazyLock::new(|| Mutex::new(None));
/// java: public static @Nullable LiteralCommandNode<FabricClientCommandSource> ahsearchNode;
/// (written by the async loader, read by `ClientboundCommandsPacketMixin`: a thread-safe cell)
pub static AHSEARCH_NODE: LazyLock<Mutex<Option<ClientCommandNode>>> = LazyLock::new(|| Mutex::new(None));
/// java: public static @Nullable LiteralCommandNode<FabricClientCommandSource> bzNode;
/// (written by the async loader, read by `ClientboundCommandsPacketMixin`: a thread-safe cell)
pub static BZ_NODE: LazyLock<Mutex<Option<ClientCommandNode>>> = LazyLock::new(|| Mutex::new(None));
/// java: public static @Nullable LiteralCommandNode<FabricClientCommandSource> bazaarNode;
/// (written by the async loader, read by `ClientboundCommandsPacketMixin`: a thread-safe cell)
pub static BAZAAR_NODE: LazyLock<Mutex<Option<ClientCommandNode>>> = LazyLock::new(|| Mutex::new(None));

/// java: public class AuctionBazaarAutocomplete
#[derive(Clone)]
pub struct AuctionBazaarAutocomplete {}

impl AuctionBazaarAutocomplete {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init(priority = 100) // Load after SearchOverManager public static void init()
    pub fn init() {
        NEURepoManager::run_async_after_load(Arc::new(move || { AuctionBazaarAutocomplete::create_nodes(); }));
    }

    /// java: public static CompletableFuture<Suggestions> suggestBzs(CommandContext<FabricClientCommandSource> context, SuggestionsBuilder builder)
    pub fn suggest_bzs(context: &CommandContext<FabricClientCommandSource>, builder: &mut SuggestionsBuilder) -> mcrust::java::util::concurrent::completable_future::CompletableFuture<Suggestions> {
        return shared_suggestion_provider::suggest(&SearchOverManager::get_bazaar_items().into_iter().collect::<Vec<String>>(), builder);
    }

    /// java: private static void createNodes()
    pub fn create_nodes() {
        if SkyblockerConfigManager::get().ui_and_visuals.search_overlay.command_autocomplete {
            compat::set_state(&AHS_NODE, Some(AuctionBazaarAutocomplete::create_node("ahs", Arc::new(move || SearchOverManager::get_auction_items().into_iter().collect::<Vec<String>>()))));
            compat::set_state(&AHSEARCH_NODE, Some(AuctionBazaarAutocomplete::create_node("ahsearch", Arc::new(move || SearchOverManager::get_auction_items().into_iter().collect::<Vec<String>>()))));
            compat::set_state(&BZ_NODE, Some(AuctionBazaarAutocomplete::create_node("bz", Arc::new(move || SearchOverManager::get_bazaar_items().into_iter().collect::<Vec<String>>()))));
            compat::set_state(&BAZAAR_NODE, Some(AuctionBazaarAutocomplete::create_node("bazaar", Arc::new(move || SearchOverManager::get_bazaar_items().into_iter().collect::<Vec<String>>()))));
        }
    }

    /// java: private static LiteralCommandNode<FabricClientCommandSource> createNode(String command, Supplier<Iterable<String>> suggester)
    pub fn create_node(command: &str, suggester: Arc<dyn Fn() -> Vec<String> + Send + Sync>) -> ClientCommandNode {
        //   java: return literal(command)
        //   java: 				.requires(_ -> Utils.isOnSkyblock())
        //   java: 				.executes(CommandUtils.noOp)
        //   java: 				.then(argument("item", StringArgumentType.greedyString())
        //   java: 						.suggests((_, builder) -> SharedSuggestionProvider.suggest(suggester.get(), builder))
        //   java: 						.executes(CommandUtils.noOp))
        //   java: 				.build();
        let mut item = client_command_registration_callback::argument("item", Arc::new(StringArgumentType::greedy_string()));
        item.suggests(Some(Arc::new(move |_: &CommandContext<FabricClientCommandSource>, builder: &mut SuggestionsBuilder| Ok(shared_suggestion_provider::suggest(&suggester(), builder)))));
        item.executes(no_op());
        let mut node = client_command_registration_callback::literal(command);
        node.requires(Arc::new(|_: &FabricClientCommandSource| crate::de::hysky::skyblocker::utils::utils::Utils::is_on_skyblock()));
        node.executes(no_op());
        node.then(&item);
        return node.build();
    }

}

/// `LiteralCommandNode<FabricClientCommandSource>`: the port's `ArgumentBuilder.build()` answers the node as a
/// `CommandNode` (its `NodeKind::Literal`), the type `ClientboundCommandsPacketMixin` hands back.
pub type ClientCommandNode = Arc<CommandNode<FabricClientCommandSource>>;

