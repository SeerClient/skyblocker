//! Ported from de.hysky.skyblocker.skyblock.AuctionBazaarAutocomplete by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::searchoverlay::search_over_manager::SearchOverManager;
use crate::de::hysky::skyblocker::utils::neu_repo_manager::NEURepoManager;
use mcrust::com::mojang::brigadier::context::command_context::CommandContext;
use mcrust::com::mojang::brigadier::suggestion::suggestions::Suggestions;
use mcrust::com::mojang::brigadier::suggestion::suggestions_builder::SuggestionsBuilder;
use mcrust::com::mojang::brigadier::tree::literal_command_node::LiteralCommandNode;
use mcrust::net::minecraft::commands::shared_suggestion_provider;

/// java: public static @Nullable LiteralCommandNode<FabricClientCommandSource> ahsNode;
pub static AHS_NODE: compat::MainThread<Option<LiteralCommandNode>> = compat::MainThread::new(|| None);
/// java: public static @Nullable LiteralCommandNode<FabricClientCommandSource> ahsearchNode;
pub static AHSEARCH_NODE: compat::MainThread<Option<LiteralCommandNode>> = compat::MainThread::new(|| None);
/// java: public static @Nullable LiteralCommandNode<FabricClientCommandSource> bzNode;
pub static BZ_NODE: compat::MainThread<Option<LiteralCommandNode>> = compat::MainThread::new(|| None);
/// java: public static @Nullable LiteralCommandNode<FabricClientCommandSource> bazaarNode;
pub static BAZAAR_NODE: compat::MainThread<Option<LiteralCommandNode>> = compat::MainThread::new(|| None);

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
    pub fn suggest_bzs(context: &CommandContext<crate::unresolved::FabricClientCommandSource>, builder: &SuggestionsBuilder) -> mcrust::java::util::concurrent::completable_future::CompletableFuture<Suggestions> {
        return shared_suggestion_provider::suggest(&SearchOverManager::get_bazaar_items(), &mut builder);
    }

    /// java: private static void createNodes()
    pub fn create_nodes() {
        if SkyblockerConfigManager::get().ui_and_visuals.search_overlay.command_autocomplete {
            *AHS_NODE.get_mut() = Some(AuctionBazaarAutocomplete::create_node("ahs", Arc::new(move || SearchOverManager::get_auction_items())));
            *AHSEARCH_NODE.get_mut() = Some(AuctionBazaarAutocomplete::create_node("ahsearch", Arc::new(move || SearchOverManager::get_auction_items())));
            *BZ_NODE.get_mut() = Some(AuctionBazaarAutocomplete::create_node("bz", Arc::new(move || SearchOverManager::get_bazaar_items())));
            *BAZAAR_NODE.get_mut() = Some(AuctionBazaarAutocomplete::create_node("bazaar", Arc::new(move || SearchOverManager::get_bazaar_items())));
        }
    }

    /// java: private static LiteralCommandNode<FabricClientCommandSource> createNode(String command, Supplier<Iterable<String>> suggester)
    pub fn create_node(command: &str, suggester: Arc<dyn Fn() -> Vec<String>>) -> LiteralCommandNode {
        // TODO(fabric2seer): a command builder outside dispatcher.register(..)
        //   java: return literal(command)
        //   java: 				.requires(_ -> Utils.isOnSkyblock())
        //   java: 				.executes(CommandUtils.noOp)
        //   java: 				.then(argument("item", StringArgumentType.greedyString())
        //   java: 						.suggests((_, builder) -> SharedSuggestionProvider.suggest(suggester.get(), builder))
        //   java: 						.executes(CommandUtils.noOp))
        //   java: 				.build();
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}
