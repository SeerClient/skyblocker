//! Ported from de.hysky.skyblocker.utils.command.argumenttypes.EggTypeArgumentType by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
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
        panic!("{:?}", mcrust::com::mojang::brigadier::exceptions::command_syntax_exception::built_in_exceptions().dispatcher_unknown_argument().create());
    }

    /// java: @Override public <S> CompletableFuture<Suggestions> listSuggestions(CommandContext<S> context, SuggestionsBuilder builder)
    pub fn list_suggestions<S: 'static>(&self, context: &CommandContext<S>, builder: &SuggestionsBuilder) -> mcrust::java::util::concurrent::completable_future::CompletableFuture<Suggestions> {
        // TODO(fabric2seer): instanceof an interface (`SharedSuggestionProvider`): Rust cannot test for a trait at run time
        //   java: return context.getSource() instanceof SharedSuggestionProvider
        //   java: 			? SharedSuggestionProvider.suggest(EggFinder.EggType.entries.stream().map(EggFinder.EggType::name).map(String::toLowerCase), builder)
        //   java: 			: Suggestions.empty();
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public Collection<String> getExamples()
    pub fn get_examples(&self) -> Vec<String> {
        // TODO(fabric2seer): `EggType.name` is not a static method of EggType
        //   java: return EggFinder.EggType.entries.stream().map(EggFinder.EggType::name).map(String::toLowerCase).toList();
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}
