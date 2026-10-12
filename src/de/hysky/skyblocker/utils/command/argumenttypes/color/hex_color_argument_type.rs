//! Ported from de.hysky.skyblocker.utils.command.argumenttypes.color.HexColorArgumentType by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::com::mojang::brigadier::context::command_context::CommandContext;
use mcrust::com::mojang::brigadier::exceptions::dynamic_command_exception_type::DynamicCommandExceptionType;
use mcrust::com::mojang::brigadier::string_reader::StringReader;
use mcrust::net::minecraft::network::chat::component;

// TODO(fabric2seer): `new DynamicCommandExceptionType(..)`: the port has no `DynamicCommandExceptionType::new`
//   java: public static final DynamicCommandExceptionType WRONG_INPUT_WIDTH = new DynamicCommandExceptionType(found -> Component.translatable("argument.color.hex.invalidString", ((String) found).length()));
/// java: public static final DynamicCommandExceptionType WRONG_INPUT_WIDTH = new DynamicCommandExceptionType(found -> Component.translatable("argument.color.hex.invalidString", ((String) found).length()));
pub static WRONG_INPUT_WIDTH: compat::MainThread<DynamicCommandExceptionType> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `WRONG_INPUT_WIDTH` - see the TODO above"));
// TODO(fabric2seer): `new DynamicCommandExceptionType(..)`: the port has no `DynamicCommandExceptionType::new`
//   java: public static final DynamicCommandExceptionType INVALID_CHARACTER = new DynamicCommandExceptionType(character -> Component.translatable("argument.color.hex.invalidChar", (String) character));
/// java: public static final DynamicCommandExceptionType INVALID_CHARACTER = new DynamicCommandExceptionType(character -> Component.translatable("argument.color.hex.invalidChar", (String) character));
pub static INVALID_CHARACTER: compat::MainThread<DynamicCommandExceptionType> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `INVALID_CHARACTER` - see the TODO above"));

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
        return context.get_argument::<i32>(name).unwrap();
    }

    /// java: @Override public Integer parse(StringReader reader) throws CommandSyntaxException
    pub fn parse(&self, reader: &mut StringReader) -> i32 {
        let mut input: String = reader.read_string().unwrap();
        // TODO(fabric2seer): the condition: static field Strings.CI
        //   java: Strings.CI.startsWith(..)
        if todo!("fabric2seer: the condition (TODO above)") {
            input = compat::substring(&input, 2);
        }
        if compat::str_len(&input) != 6 {
            panic!("{:?}", WRONG_INPUT_WIDTH.get_mut().create(&input));
        }
        for i in 0..compat::str_len(&input) {
            let mut character: char = compat::char_at(&input, i);
            if character < '0' || character > '9' && character < 'a' || character > 'f' && character < 'A' || character > 'F' {
                panic!("{:?}", INVALID_CHARACTER.get_mut().create(&character.to_string()));
            }
        }
        return (format!("#{}", input)).trim().parse::<i32>().expect("NumberFormatException");
    }

}
