//! Ported from de.hysky.skyblocker.utils.command.argumenttypes.EggTypeArgumentType by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::com::mojang::brigadier::immutable_string_reader::ImmutableStringReader as _;
use crate::de::hysky::skyblocker::skyblock::chocolatefactory::egg_finder;
use crate::de::hysky::skyblocker::skyblock::chocolatefactory::egg_finder::EggType;
use mcrust::com::mojang::brigadier::context::command_context::CommandContext;
use mcrust::com::mojang::brigadier::string_reader::StringReader;
use mcrust::com::mojang::brigadier::suggestion::suggestions::Suggestions;
use mcrust::com::mojang::brigadier::suggestion::suggestions_builder::SuggestionsBuilder;
use mcrust::net::minecraft::commands::shared_suggestion_provider::SharedSuggestionProvider;
use mcrust::com::mojang::brigadier::exceptions::built_in_exception_provider::BuiltInExceptionProvider as _;

/// java: public final class EggTypeArgumentType implements ArgumentType<EggFinder.EggType>
#[derive(Clone)]
pub struct EggTypeArgumentType {}

impl EggTypeArgumentType {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static EggTypeArgumentType eggType()
    pub fn egg_type() -> EggTypeArgumentType {
        return EggTypeArgumentType::new();
    }

    /// java: @Override public EggFinder.EggType parse(StringReader reader) throws CommandSyntaxException
    pub fn parse(&self, reader: &mut StringReader) -> EggType {
        let mut name: String = reader.read_unquoted_string();
        for r#type in egg_finder::EGG_TYPE_ENTRIES.iter().copied() {
            if compat::equals_ignore_case(&(r#type).name(), &name) {
                return r#type;
            }
        }
        // java: throw CommandSyntaxException.BUILT_IN_EXCEPTIONS.dispatcherUnknownArgument().create(); - the exception is the
        // panic's payload, which the `ArgumentType` impl below turns back into the parse's `Err`
        std::panic::panic_any(mcrust::com::mojang::brigadier::exceptions::command_syntax_exception::built_in_exceptions().dispatcher_unknown_argument().create());
    }

    /// java: @Override public <S> CompletableFuture<Suggestions> listSuggestions(CommandContext<S> context, SuggestionsBuilder builder)
    /// `builder` is `&mut`: `SharedSuggestionProvider.suggest` adds to it.
    pub fn list_suggestions<S: 'static>(&self, context: &CommandContext<S>, builder: &mut SuggestionsBuilder) -> mcrust::java::util::concurrent::completable_future::CompletableFuture<Suggestions> {
        // java: return context.getSource() instanceof SharedSuggestionProvider
        //           ? SharedSuggestionProvider.suggest(EggFinder.EggType.entries.stream().map(EggFinder.EggType::name).map(String::toLowerCase), builder)
        //           : Suggestions.empty();
        // (`instanceof` an interface: the port's implementors of `SharedSuggestionProvider` are the client's
        // `ClientSuggestionProvider`, which is also Fabric's `FabricClientCommandSource`)
        let _ = context;
        let is_shared_suggestion_provider: bool = std::any::TypeId::of::<S>() == std::any::TypeId::of::<mcrust::net::minecraft::client::multiplayer::client_suggestion_provider::ClientSuggestionProvider>()
            || std::any::TypeId::of::<S>() == std::any::TypeId::of::<std::sync::Arc<mcrust::net::minecraft::client::multiplayer::client_suggestion_provider::ClientSuggestionProvider>>();
        return if is_shared_suggestion_provider {
            mcrust::net::minecraft::commands::shared_suggestion_provider::suggest(&self.get_examples(), builder)
        } else {
            Suggestions::empty()
        };
    }

    /// java: @Override public Collection<String> getExamples()
    pub fn get_examples(&self) -> Vec<String> {
        // java: return EggFinder.EggType.entries.stream().map(EggFinder.EggType::name).map(String::toLowerCase).toList();
        return egg_finder::EGG_TYPE_ENTRIES.iter().map(|r#type| r#type.name()).map(|name| name.to_lowercase()).collect();
    }

}

/// java: `implements ArgumentType<EggFinder.EggType>` - the parsed value is the `EggType`
impl std::fmt::Debug for EggTypeArgumentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("EggTypeArgumentType")
    }
}

impl mcrust::com::mojang::brigadier::arguments::argument_type::ArgumentType for EggTypeArgumentType {
    fn parse(&self, reader: &mut StringReader) -> Result<Arc<dyn std::any::Any + Send + Sync>, mcrust::com::mojang::brigadier::exceptions::command_syntax_exception::CommandSyntaxException> {
        let start = reader.get_cursor();
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| EggTypeArgumentType::parse(self, reader))) {
            Ok(egg_type) => Ok(Arc::new(egg_type)),
            Err(payload) => {
                reader.set_cursor(start);
                match payload.downcast::<mcrust::com::mojang::brigadier::exceptions::command_syntax_exception::CommandSyntaxException>() {
                    Ok(exception) => Err(*exception),
                    Err(payload) => std::panic::resume_unwind(payload),
                }
            }
        }
    }

    fn list_suggestions(&self, context: &dyn std::any::Any, builder: &mut SuggestionsBuilder) -> mcrust::java::util::concurrent::completable_future::CompletableFuture<Suggestions> {
        // the client command source is a `SharedSuggestionProvider` (see `list_suggestions` above)
        let _ = context;
        mcrust::net::minecraft::commands::shared_suggestion_provider::suggest(&EggTypeArgumentType::get_examples(self), builder)
    }

    fn get_examples(&self) -> Vec<String> {
        EggTypeArgumentType::get_examples(self)
    }
}
