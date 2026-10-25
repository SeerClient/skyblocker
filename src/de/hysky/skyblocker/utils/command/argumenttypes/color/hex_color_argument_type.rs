//! Ported from de.hysky.skyblocker.utils.command.argumenttypes.color.HexColorArgumentType by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::com::mojang::brigadier::immutable_string_reader::ImmutableStringReader as _;
use mcrust::com::mojang::brigadier::context::command_context::CommandContext;
use mcrust::com::mojang::brigadier::exceptions::dynamic_command_exception_type::DynamicCommandExceptionType;
use mcrust::com::mojang::brigadier::string_reader::StringReader;
use mcrust::net::minecraft::network::chat::component;
use mcrust::com::mojang::brigadier::message::Message;

/// java: public static final DynamicCommandExceptionType WRONG_INPUT_WIDTH = new DynamicCommandExceptionType(found -> Component.translatable("argument.color.hex.invalidString", ((String) found).length()));
pub static WRONG_INPUT_WIDTH: LazyLock<Arc<DynamicCommandExceptionType>> = LazyLock::new(|| {
    DynamicCommandExceptionType::new(|found: &str| {
        Arc::new(component::translatable_args("argument.color.hex.invalidString", vec![FormattedText::Plain(compat::str_len(found).to_string())])) as Arc<dyn Message>
    })
});
/// java: public static final DynamicCommandExceptionType INVALID_CHARACTER = new DynamicCommandExceptionType(character -> Component.translatable("argument.color.hex.invalidChar", (String) character));
pub static INVALID_CHARACTER: LazyLock<Arc<DynamicCommandExceptionType>> = LazyLock::new(|| {
    DynamicCommandExceptionType::new(|character: &str| {
        Arc::new(component::translatable_args("argument.color.hex.invalidChar", vec![FormattedText::Plain(character.to_string())])) as Arc<dyn Message>
    })
});

/// java: @SuppressWarnings("RedundantCast") public final class HexColorArgumentType implements ArgumentType<Integer>
#[derive(Clone)]
pub struct HexColorArgumentType {}

impl HexColorArgumentType {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static int getInt(CommandContext<FabricClientCommandSource> context, String name)
    pub fn get_int(context: &CommandContext<crate::unresolved::FabricClientCommandSource>, name: &str) -> i32 {
        return *context.get_argument::<i32>(name).unwrap();
    }

    /// java: @Override public Integer parse(StringReader reader) throws CommandSyntaxException
    pub fn parse(&self, reader: &mut StringReader) -> i32 {
        let mut input: String = reader.read_string().unwrap_or_else(|e| std::panic::panic_any(e));
        // java: Strings.CI.startsWith(input, "0x") - a case-insensitive prefix
        if input.len() >= 2 && input.is_char_boundary(2) && input[..2].eq_ignore_ascii_case("0x") {
            input = compat::substring(&input, 2);
        }
        if compat::str_len(&input) != 6 {
            std::panic::panic_any(WRONG_INPUT_WIDTH.create(&input));
        }
        for i in 0..compat::str_len(&input) {
            let mut character: char = compat::char_at(&input, i);
            if (character < '0' || character > '9') && (character < 'a' || character > 'f') && (character < 'A' || character > 'F') {
                std::panic::panic_any(INVALID_CHARACTER.create(&character.to_string()));
            }
        }
        // java: Integer.decode("#" + input) - '#' is the hexadecimal radix prefix
        return i32::from_str_radix(&input, 16).expect("NumberFormatException");
    }

}

/// java: `implements ArgumentType<Integer>` - the parsed value is the `i32` colour
impl std::fmt::Debug for HexColorArgumentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("HexColorArgumentType")
    }
}

impl mcrust::com::mojang::brigadier::arguments::argument_type::ArgumentType for HexColorArgumentType {
    fn parse(&self, reader: &mut StringReader) -> Result<Arc<dyn std::any::Any + Send + Sync>, mcrust::com::mojang::brigadier::exceptions::command_syntax_exception::CommandSyntaxException> {
        // the parse throws `CommandSyntaxException`, carried as the panic's payload
        let start = reader.get_cursor();
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| HexColorArgumentType::parse(self, reader))) {
            Ok(color) => Ok(Arc::new(color)),
            Err(payload) => {
                reader.set_cursor(start);
                match payload.downcast::<mcrust::com::mojang::brigadier::exceptions::command_syntax_exception::CommandSyntaxException>() {
                    Ok(exception) => Err(*exception),
                    Err(payload) => std::panic::resume_unwind(payload),
                }
            }
        }
    }
}
