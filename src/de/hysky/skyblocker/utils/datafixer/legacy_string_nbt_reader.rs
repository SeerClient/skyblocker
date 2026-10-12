//! Ported from de.hysky.skyblocker.utils.datafixer.LegacyStringNbtReader by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::com::mojang::brigadier::exceptions::dynamic2_command_exception_type::Dynamic2CommandExceptionType;
use mcrust::com::mojang::brigadier::exceptions::dynamic_command_exception_type::DynamicCommandExceptionType;
use mcrust::com::mojang::brigadier::exceptions::simple_command_exception_type::SimpleCommandExceptionType;
use mcrust::com::mojang::brigadier::string_reader::StringReader;
use mcrust::java::lang::number::Number;
use mcrust::net::minecraft::nbt::byte_tag;
use mcrust::net::minecraft::nbt::double_tag;
use mcrust::net::minecraft::nbt::float_tag;
use mcrust::net::minecraft::nbt::int_tag;
use mcrust::net::minecraft::nbt::list_tag;
use mcrust::net::minecraft::nbt::long_tag;
use mcrust::net::minecraft::nbt::short_tag;
use mcrust::net::minecraft::nbt::string_tag;
use mcrust::net::minecraft::nbt::tag::Tag;
use mcrust::net::minecraft::nbt::tag_type::TagType;
use mcrust::net::minecraft::network::chat::component;
use mcrust::com::mojang::brigadier::immutable_string_reader::ImmutableStringReader as _;

/// java: public static final SimpleCommandExceptionType TRAILING = new SimpleCommandExceptionType(Component.translatable("argument.nbt.trailing"));
pub static TRAILING: compat::MainThread<SimpleCommandExceptionType> = compat::MainThread::new(|| SimpleCommandExceptionType::new(component::translatable("argument.nbt.trailing")));
/// java: public static final SimpleCommandExceptionType EXPECTED_KEY = new SimpleCommandExceptionType(Component.translatable("argument.nbt.expected.key"));
pub static EXPECTED_KEY: compat::MainThread<SimpleCommandExceptionType> = compat::MainThread::new(|| SimpleCommandExceptionType::new(component::translatable("argument.nbt.expected.key")));
/// java: public static final SimpleCommandExceptionType EXPECTED_VALUE = new SimpleCommandExceptionType(Component.translatable("argument.nbt.expected.value"));
pub static EXPECTED_VALUE: compat::MainThread<SimpleCommandExceptionType> = compat::MainThread::new(|| SimpleCommandExceptionType::new(component::translatable("argument.nbt.expected.value")));
// TODO(fabric2seer): `new Dynamic2CommandExceptionType(..)`: the port has no `Dynamic2CommandExceptionType::new`
//   java: public static final Dynamic2CommandExceptionType LIST_MIXED = new Dynamic2CommandExceptionType(
//   java: 		(receivedType, expectedType) -> Component.translatableEscape("argument.nbt.list.mixed", receivedType, expectedType)
//   java: 	);
/// java: public static final Dynamic2CommandExceptionType LIST_MIXED = new Dynamic2CommandExceptionType( (receivedType, expectedType) -> Component.translatableEscape("argument.nbt.list.mixed", receivedType, ex
pub static LIST_MIXED: compat::MainThread<Dynamic2CommandExceptionType> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `LIST_MIXED` - see the TODO above"));
// TODO(fabric2seer): `new Dynamic2CommandExceptionType(..)`: the port has no `Dynamic2CommandExceptionType::new`
//   java: public static final Dynamic2CommandExceptionType ARRAY_MIXED = new Dynamic2CommandExceptionType(
//   java: 		(receivedType, expectedType) -> Component.translatableEscape("argument.nbt.array.mixed", receivedType, expectedType)
//   java: 	);
/// java: public static final Dynamic2CommandExceptionType ARRAY_MIXED = new Dynamic2CommandExceptionType( (receivedType, expectedType) -> Component.translatableEscape("argument.nbt.array.mixed", receivedType, 
pub static ARRAY_MIXED: compat::MainThread<Dynamic2CommandExceptionType> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `ARRAY_MIXED` - see the TODO above"));
// TODO(fabric2seer): `new DynamicCommandExceptionType(..)`: the port has no `DynamicCommandExceptionType::new`
//   java: public static final DynamicCommandExceptionType ARRAY_INVALID = new DynamicCommandExceptionType(
//   java: 		type -> Component.translatableEscape("argument.nbt.array.invalid", type)
//   java: 	);
/// java: public static final DynamicCommandExceptionType ARRAY_INVALID = new DynamicCommandExceptionType( type -> Component.translatableEscape("argument.nbt.array.invalid", type) );
pub static ARRAY_INVALID: compat::MainThread<DynamicCommandExceptionType> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `ARRAY_INVALID` - see the TODO above"));
/// java: private static final Pattern DOUBLE_PATTERN_IMPLICIT = Pattern.compile("[-+]?(?:[0-9]+[.]|[0-9]*[.][0-9]+)(?:e[-+]?[0-9]+)?", Pattern.CASE_INSENSITIVE);
pub static DOUBLE_PATTERN_IMPLICIT: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile_with_flags("[-+]?(?:[0-9]+[.]|[0-9]*[.][0-9]+)(?:e[-+]?[0-9]+)?", mcrust::java::util::regex::pattern::CASE_INSENSITIVE));
/// java: private static final Pattern DOUBLE_PATTERN = Pattern.compile("[-+]?(?:[0-9]+[.]?|[0-9]*[.][0-9]+)(?:e[-+]?[0-9]+)?d", Pattern.CASE_INSENSITIVE);
pub static DOUBLE_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile_with_flags("[-+]?(?:[0-9]+[.]?|[0-9]*[.][0-9]+)(?:e[-+]?[0-9]+)?d", mcrust::java::util::regex::pattern::CASE_INSENSITIVE));
/// java: private static final Pattern FLOAT_PATTERN = Pattern.compile("[-+]?(?:[0-9]+[.]?|[0-9]*[.][0-9]+)(?:e[-+]?[0-9]+)?f", Pattern.CASE_INSENSITIVE);
pub static FLOAT_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile_with_flags("[-+]?(?:[0-9]+[.]?|[0-9]*[.][0-9]+)(?:e[-+]?[0-9]+)?f", mcrust::java::util::regex::pattern::CASE_INSENSITIVE));
/// java: private static final Pattern BYTE_PATTERN = Pattern.compile("[-+]?(?:0|[1-9][0-9]*)b", Pattern.CASE_INSENSITIVE);
pub static BYTE_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile_with_flags("[-+]?(?:0|[1-9][0-9]*)b", mcrust::java::util::regex::pattern::CASE_INSENSITIVE));
/// java: private static final Pattern LONG_PATTERN = Pattern.compile("[-+]?(?:0|[1-9][0-9]*)l", Pattern.CASE_INSENSITIVE);
pub static LONG_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile_with_flags("[-+]?(?:0|[1-9][0-9]*)l", mcrust::java::util::regex::pattern::CASE_INSENSITIVE));
/// java: private static final Pattern SHORT_PATTERN = Pattern.compile("[-+]?(?:0|[1-9][0-9]*)s", Pattern.CASE_INSENSITIVE);
pub static SHORT_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile_with_flags("[-+]?(?:0|[1-9][0-9]*)s", mcrust::java::util::regex::pattern::CASE_INSENSITIVE));
/// java: private static final Pattern INT_PATTERN = Pattern.compile("[-+]?(?:0|[1-9][0-9]*)");
pub static INT_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("[-+]?(?:0|[1-9][0-9]*)"));

/// java: public class LegacyStringNbtReader
#[derive(Clone)]
pub struct LegacyStringNbtReader {
    /// java: public final StringReader reader;
    pub reader: StringReader,
}

impl LegacyStringNbtReader {
    /// java: public LegacyStringNbtReader(StringReader reader)
    pub fn new(mut reader: StringReader) -> Self {
        let __init_reader = reader.clone();
        let mut this = Self {
            reader: __init_reader,
        };
        this
    }

    /// java: public static CompoundTag parse(String string) throws CommandSyntaxException
    pub fn parse(string: &str) -> Tag {
        return LegacyStringNbtReader::new(StringReader::new(string)).read_compound();
    }

    /// java: private static byte[] mapToByteArray(List<Number> numbers)
    pub fn map_to_byte_array(numbers: &Vec<Number>) -> Vec<i8> {
        let mut bytes: Vec<i8> = vec![0; ((numbers.len() as i32)) as usize];
        for i in 0..(numbers.len() as i32) {
            // TODO(fabric2seer): `.byteValue(..)` on a Number
            //   java: bytes[i] = numbers.get(i).byteValue();
        }
        return bytes;
    }

    /// java: @VisibleForTesting protected CompoundTag readCompound() throws CommandSyntaxException
    pub fn read_compound(&mut self) -> Tag {
        let mut nbt_compound: Tag = self.parse_compound();
        self.reader.skip_whitespace();
        if self.reader.can_read() {
            panic!("{:?}", TRAILING.get_mut().create_with_context(&self.reader));
        } else {
            return nbt_compound;
        }
    }

    /// java: protected String readString() throws CommandSyntaxException
    pub fn read_string(&mut self) -> String {
        self.reader.skip_whitespace();
        if !self.reader.can_read() {
            panic!("{:?}", EXPECTED_KEY.get_mut().create_with_context(&self.reader));
        } else {
            return self.reader.read_string().unwrap();
        }
    }

    /// java: protected Tag parseElementPrimitive() throws CommandSyntaxException
    pub fn parse_element_primitive(&mut self) -> Tag {
        self.reader.skip_whitespace();
        let mut i: i32 = self.reader.get_cursor();
        if StringReader::is_quoted_string_start(self.reader.peek()) {
            return string_tag::value_of(self.reader.read_quoted_string().unwrap());
        } else {
            let mut string: String = self.reader.read_unquoted_string();
            if string.is_empty() {
                self.reader.set_cursor(i);
                panic!("{:?}", EXPECTED_VALUE.get_mut().create_with_context(&self.reader));
            } else {
                return self.parse_primitive(&string);
            }
        }
    }

    /// java: private Tag parsePrimitive(String input)
    pub fn parse_primitive(&self, input: &str) -> Tag {
        // java: try {..} catch (NumberFormatException _) - the body runs inline: a Rust body does not throw checked exceptions (a panic in it propagates)
        {
            if FLOAT_PATTERN.matcher(input).matches() {
                return float_tag::value_of((compat::substring2(&input, 0, (compat::str_len(&input)).wrapping_sub(1))).trim().parse::<f32>().expect("NumberFormatException"));
            }
            if BYTE_PATTERN.matcher(input).matches() {
                return byte_tag::value_of((compat::substring2(&input, 0, (compat::str_len(&input)).wrapping_sub(1))).trim().parse::<i8>().expect("NumberFormatException"));
            }
            if LONG_PATTERN.matcher(input).matches() {
                return long_tag::value_of((compat::substring2(&input, 0, (compat::str_len(&input)).wrapping_sub(1))).trim().parse::<i64>().expect("NumberFormatException"));
            }
            if SHORT_PATTERN.matcher(input).matches() {
                return short_tag::value_of((compat::substring2(&input, 0, (compat::str_len(&input)).wrapping_sub(1))).trim().parse::<i16>().expect("NumberFormatException"));
            }
            if INT_PATTERN.matcher(input).matches() {
                return int_tag::value_of(input.trim().parse::<i32>().expect("NumberFormatException"));
            }
            if DOUBLE_PATTERN.matcher(input).matches() {
                return double_tag::value_of((compat::substring2(&input, 0, (compat::str_len(&input)).wrapping_sub(1))).trim().parse::<f64>().expect("NumberFormatException"));
            }
            if DOUBLE_PATTERN_IMPLICIT.matcher(input).matches() {
                return double_tag::value_of(input.trim().parse::<f64>().expect("NumberFormatException"));
            }
            if compat::equals_ignore_case(&"true", input) {
                return mcrust::net::minecraft::nbt::byte_tag::one();
            }
            if compat::equals_ignore_case(&"false", input) {
                return mcrust::net::minecraft::nbt::byte_tag::zero();
            }
        }
        // TODO(fabric2seer): the `catch (NumberFormatException _)` block is not reached in Rust - the try body returns, breaks or continues, so it cannot run inside `catch_unwind`
        //   java: {
        //   java: 		}
        return string_tag::value_of(input.to_string());
    }

    /// java: public Tag parseElement() throws CommandSyntaxException
    pub fn parse_element(&mut self) -> Tag {
        self.reader.skip_whitespace();
        if !self.reader.can_read() {
            panic!("{:?}", EXPECTED_VALUE.get_mut().create_with_context(&self.reader));
        } else {
            let mut c: char = self.reader.peek();
            if c == '{' {
                return self.parse_compound();
            } else {
                return (if c == '[' { self.parse_array() } else { self.parse_element_primitive() });
            }
        }
    }

    /// java: protected Tag parseArray() throws CommandSyntaxException
    pub fn parse_array(&mut self) -> Tag {
        return (if self.reader.can_read_length(3) && !(StringReader::is_quoted_string_start(self.reader.peek_at(1))) && self.reader.peek_at(2) == ';' { self.parse_element_primitive_array() } else { self.parse_list() });
    }

    /// java: public CompoundTag parseCompound() throws CommandSyntaxException
    pub fn parse_compound(&mut self) -> Tag {
        self.expect('{');
        let mut nbt_compound: Tag = mcrust::net::minecraft::nbt::compound_tag::Tag::compound();
        self.reader.skip_whitespace();
        while self.reader.can_read() && self.reader.peek() != '}' {
            let mut i: i32 = self.reader.get_cursor();
            let mut string: String = self.read_string();
            if string.is_empty() {
                self.reader.set_cursor(i);
                panic!("{:?}", EXPECTED_KEY.get_mut().create_with_context(&self.reader));
            }
            self.expect(':');
            nbt_compound.put(&string, self.parse_element());
            if !self.read_comma() {
                break;
            }
            if !self.reader.can_read() {
                panic!("{:?}", EXPECTED_KEY.get_mut().create_with_context(&self.reader));
            }
        }
        self.expect('}');
        return nbt_compound;
    }

    /// java: protected Tag parseList() throws CommandSyntaxException
    pub fn parse_list(&mut self) -> Tag {
        self.expect('[');
        self.reader.skip_whitespace();
        if !self.reader.can_read() {
            panic!("{:?}", EXPECTED_VALUE.get_mut().create_with_context(&self.reader));
        } else {
            // TODO(fabric2seer): `new ListTag(..)`: the port has no `ListTag::new`
            // rust: let mut nbt_list: Tag = Tag::new();
            let mut nbt_list: Tag = todo!("fabric2seer: the initialiser of `nbtList` (TODO above)");
            let mut list_type: Option<Box<dyn TagType<compat::Object>>> = None;
            while self.reader.peek() != ']' {
                let mut original_pos: i32 = self.reader.get_cursor();
                while self.reader.peek().is_numeric() {
                    self.reader.skip();
                }
                if self.reader.peek() == ':' {
                    self.reader.skip();
                    self.reader.skip_whitespace();
                } else {
                    self.reader.set_cursor(original_pos);
                }
                let mut cursor_position: i32 = self.reader.get_cursor();
                let mut element: Tag = self.parse_element();
                let element_type = element.get_type();
                if list_type.is_none() {
                    list_type = Some(element_type.clone());
                } else if !std::ptr::eq(element_type, list_type.as_ref().unwrap()) {
                    self.reader.set_cursor(cursor_position);
                    panic!("{:?}", LIST_MIXED.get_mut().create_with_context(&self.reader, &element_type.get_pretty_name(), &list_type.as_ref().unwrap().get_pretty_name()));
                }
                // TODO(fabric2seer): `add` takes 3 argument(s) in the port (`elements: &mut Vec<Tag>, index: usize, tag: Tag`), Java passes 1
                // rust: list_tag::add(&mut element);
                if !self.read_comma() {
                    break;
                }
                if !self.reader.can_read() {
                    panic!("{:?}", EXPECTED_VALUE.get_mut().create_with_context(&self.reader));
                }
            }
            self.expect(']');
            return nbt_list;
        }
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: private Tag parseElementPrimitiveArray() throws CommandSyntaxException
    pub fn parse_element_primitive_array(&mut self) -> Tag {
        self.expect('[');
        let mut i: i32 = self.reader.get_cursor();
        let mut c: char = self.reader.read();
        self.reader.read();
        self.reader.skip_whitespace();
        if !self.reader.can_read() {
            panic!("{:?}", EXPECTED_VALUE.get_mut().create_with_context(&self.reader));
        } else if c == 'B' {
            // TODO(fabric2seer): `new ByteArrayTag(..)`: the port has no `ByteArrayTag::new`
            // rust: return Tag::new(LegacyStringNbtReader::map_to_byte_array(&self.read_array(&mcrust::net::minecraft::nbt::byte_array_tag::TYPE, &mcrust::net::minecraft::nbt::byte_tag::TYPE)));
            return todo!("fabric2seer: see the TODO above");
        } else if c == 'L' {
            // TODO(fabric2seer): `.longValue(..)` on a T
            //   java: return new LongArrayTag(this.readArray(LongArrayTag.TYPE, LongTag.TYPE).stream().mapToLong(Number::longValue).toArray());
        } else if c == 'I' {
            // TODO(fabric2seer): `.intValue(..)` on a T
            //   java: return new IntArrayTag(this.readArray(IntArrayTag.TYPE, IntTag.TYPE).stream().mapToInt(Number::intValue).toArray());
        } else {
            self.reader.set_cursor(i);
            panic!("{:?}", ARRAY_INVALID.get_mut().create_with_context(&self.reader, &c.to_string()));
        }
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @SuppressWarnings("unchecked") private <T extends Number> List<T> readArray(TagType<?> arrayTypeReader, TagType<?> typeReader) throws CommandSyntaxException
    pub fn read_array<T: 'static>(&mut self, array_type_reader: &dyn TagType<compat::Object>, type_reader: &dyn TagType<compat::Object>) -> Vec<T> {
        let mut list: Vec<T> = vec![];
        while self.reader.peek() != ']' {
            let mut i: i32 = self.reader.get_cursor();
            let mut nbt_element: Tag = self.parse_element();
            let nbt_type = nbt_element.get_type();
            if !std::ptr::eq(nbt_type, type_reader) {
                self.reader.set_cursor(i);
                panic!("{:?}", ARRAY_MIXED.get_mut().create_with_context(&self.reader, &nbt_type.get_pretty_name(), &array_type_reader.get_pretty_name()));
            }
            if std::ptr::eq(type_reader, &mcrust::net::minecraft::nbt::byte_tag::TYPE) {
                list.push((nbt_element.as_any().downcast_ref::<Tag>().unwrap()).byte_value().unwrap());
            } else if std::ptr::eq(type_reader, &mcrust::net::minecraft::nbt::long_tag::TYPE) {
                list.push((nbt_element.as_any().downcast_ref::<Tag>().unwrap()).long_value().unwrap());
            } else {
                list.push((nbt_element.as_any().downcast_ref::<Tag>().unwrap()).int_value().unwrap());
            }
            if !self.read_comma() {
                break;
            }
            if !self.reader.can_read() {
                panic!("{:?}", EXPECTED_VALUE.get_mut().create_with_context(&self.reader));
            }
        }
        self.expect(']');
        return list;
    }

    /// java: protected boolean readComma()
    pub fn read_comma(&mut self) -> bool {
        self.reader.skip_whitespace();
        if self.reader.can_read() && self.reader.peek() == ',' {
            self.reader.skip();
            self.reader.skip_whitespace();
            return true;
        } else {
            return false;
        }
    }

    /// java: protected void expect(char c) throws CommandSyntaxException
    pub fn expect(&mut self, mut c: char) {
        self.reader.skip_whitespace();
        self.reader.expect(c).unwrap();
    }

}
