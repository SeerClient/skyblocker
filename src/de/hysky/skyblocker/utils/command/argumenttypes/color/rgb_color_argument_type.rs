//! Ported from de.hysky.skyblocker.utils.command.argumenttypes.color.RgbColorArgumentType by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::com::mojang::brigadier::arguments::integer_argument_type::IntegerArgumentType;
use mcrust::com::mojang::brigadier::context::command_context::CommandContext;
use mcrust::com::mojang::brigadier::exceptions::simple_command_exception_type::SimpleCommandExceptionType;
use mcrust::com::mojang::brigadier::string_reader::StringReader;
use mcrust::net::minecraft::network::chat::component;
use mcrust::com::mojang::brigadier::arguments::argument_type::ArgumentType as _;
use mcrust::com::mojang::brigadier::immutable_string_reader::ImmutableStringReader as _;

/// java: public static final SimpleCommandExceptionType INCOMPLETE_EXCEPTION = new SimpleCommandExceptionType(Component.translatable("argument.color.rgb.incomplete"));
pub static INCOMPLETE_EXCEPTION: LazyLock<Arc<SimpleCommandExceptionType>> = LazyLock::new(|| SimpleCommandExceptionType::new(Arc::new(component::translatable("argument.color.rgb.incomplete"))));

/// java: public final class RgbColorArgumentType implements ArgumentType<Integer>
#[derive(Clone)]
pub struct RgbColorArgumentType {}

impl RgbColorArgumentType {
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
        let mut i: i32 = reader.get_cursor();
        let mut red_argument: i32 = *(IntegerArgumentType::integer_range(0x00, 0xFF)).parse(reader).unwrap().downcast_ref::<i32>().expect("IntegerArgumentType answers an Integer");
        if reader.can_read() && reader.peek() == ' ' {
            reader.skip();
            let mut green_argument: i32 = *(IntegerArgumentType::integer_range(0x00, 0xFF)).parse(reader).unwrap().downcast_ref::<i32>().expect("IntegerArgumentType answers an Integer");
            if reader.can_read() && reader.peek() == ' ' {
                reader.skip();
                let mut blue_argument: i32 = *(IntegerArgumentType::integer_range(0x00, 0xFF)).parse(reader).unwrap().downcast_ref::<i32>().expect("IntegerArgumentType answers an Integer");
                return ((red_argument.wrapping_shl((16) as u32) | green_argument.wrapping_shl((8) as u32)) | blue_argument);
            } else {
                reader.set_cursor(i);
                panic!("{:?}", INCOMPLETE_EXCEPTION.create_with_context(&*reader));
            }
        } else {
            reader.set_cursor(i);
            panic!("{:?}", INCOMPLETE_EXCEPTION.create_with_context(&*reader));
        }
    }

}
