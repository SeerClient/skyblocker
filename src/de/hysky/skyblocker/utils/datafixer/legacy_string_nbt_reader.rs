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
use mcrust::com::mojang::brigadier::message::Message;
use mcrust::com::mojang::brigadier::immutable_string_reader::ImmutableStringReader as _;

/// java: public static final SimpleCommandExceptionType TRAILING = new SimpleCommandExceptionType(Component.translatable("argument.nbt.trailing"));
pub static TRAILING: LazyLock<Arc<SimpleCommandExceptionType>> = LazyLock::new(|| SimpleCommandExceptionType::new(Arc::new(component::translatable("argument.nbt.trailing"))));
/// java: public static final SimpleCommandExceptionType EXPECTED_KEY = new SimpleCommandExceptionType(Component.translatable("argument.nbt.expected.key"));
pub static EXPECTED_KEY: LazyLock<Arc<SimpleCommandExceptionType>> = LazyLock::new(|| SimpleCommandExceptionType::new(Arc::new(component::translatable("argument.nbt.expected.key"))));
/// java: public static final SimpleCommandExceptionType EXPECTED_VALUE = new SimpleCommandExceptionType(Component.translatable("argument.nbt.expected.value"));
pub static EXPECTED_VALUE: LazyLock<Arc<SimpleCommandExceptionType>> = LazyLock::new(|| SimpleCommandExceptionType::new(Arc::new(component::translatable("argument.nbt.expected.value"))));
// TODO(fabric2seer): `new Dynamic2CommandExceptionType(..)`: the port has no `Dynamic2CommandExceptionType::new`
//   java: public static final Dynamic2CommandExceptionType LIST_MIXED = new Dynamic2CommandExceptionType(
//   java: 		(receivedType, expectedType) -> Component.translatableEscape("argument.nbt.list.mixed", receivedType, expectedType)
//   java: 	);
/// java: public static final Dynamic2CommandExceptionType LIST_MIXED = new Dynamic2CommandExceptionType( (receivedType, expectedType) -> Component.translatableEscape("argument.nbt.list.mixed", receivedType, ex
pub static LIST_MIXED: LazyLock<Arc<Dynamic2CommandExceptionType>> = LazyLock::new(|| {
    Dynamic2CommandExceptionType::new(|received_type: &str, expected_type: &str| {
        Arc::new(component::translatable_escape("argument.nbt.list.mixed", vec![FormattedText::Plain(received_type.to_string()), FormattedText::Plain(expected_type.to_string())])) as Arc<dyn Message>
    })
});
// TODO(fabric2seer): `new Dynamic2CommandExceptionType(..)`: the port has no `Dynamic2CommandExceptionType::new`
//   java: public static final Dynamic2CommandExceptionType ARRAY_MIXED = new Dynamic2CommandExceptionType(
//   java: 		(receivedType, expectedType) -> Component.translatableEscape("argument.nbt.array.mixed", receivedType, expectedType)
//   java: 	);
/// java: public static final Dynamic2CommandExceptionType ARRAY_MIXED = new Dynamic2CommandExceptionType( (receivedType, expectedType) -> Component.translatableEscape("argument.nbt.array.mixed", receivedType, 
pub static ARRAY_MIXED: LazyLock<Arc<Dynamic2CommandExceptionType>> = LazyLock::new(|| {
    Dynamic2CommandExceptionType::new(|received_type: &str, expected_type: &str| {
        Arc::new(component::translatable_escape("argument.nbt.array.mixed", vec![FormattedText::Plain(received_type.to_string()), FormattedText::Plain(expected_type.to_string())])) as Arc<dyn Message>
    })
});
// TODO(fabric2seer): `new DynamicCommandExceptionType(..)`: the port has no `DynamicCommandExceptionType::new`
//   java: public static final DynamicCommandExceptionType ARRAY_INVALID = new DynamicCommandExceptionType(
//   java: 		type -> Component.translatableEscape("argument.nbt.array.invalid", type)
//   java: 	);
/// java: public static final DynamicCommandExceptionType ARRAY_INVALID = new DynamicCommandExceptionType( type -> Component.translatableEscape("argument.nbt.array.invalid", type) );
pub static ARRAY_INVALID: LazyLock<Arc<DynamicCommandExceptionType>> = LazyLock::new(|| {
    DynamicCommandExceptionType::new(|r#type: &str| {
        Arc::new(component::translatable_escape("argument.nbt.array.invalid", vec![FormattedText::Plain(r#type.to_string())])) as Arc<dyn Message>
    })
});
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
            bytes[(i) as usize] = numbers[(i) as usize].byte_value();
        }
        return bytes;
    }

    /// java: @VisibleForTesting protected CompoundTag readCompound() throws CommandSyntaxException
    pub fn read_compound(&mut self) -> Tag {
        let mut nbt_compound: Tag = self.parse_compound();
        self.reader.skip_whitespace();
        if self.reader.can_read() {
            panic!("{:?}", TRAILING.create_with_context(&self.reader));
        } else {
            return nbt_compound;
        }
    }

    /// java: protected String readString() throws CommandSyntaxException
    pub fn read_string(&mut self) -> String {
        self.reader.skip_whitespace();
        if !self.reader.can_read() {
            panic!("{:?}", EXPECTED_KEY.create_with_context(&self.reader));
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
                panic!("{:?}", EXPECTED_VALUE.create_with_context(&self.reader));
            } else {
                return self.parse_primitive(&string);
            }
        }
    }

    /// java: private Tag parsePrimitive(String input)
    pub fn parse_primitive(&self, input: &str) -> Tag {
        // java: try {..} catch (NumberFormatException _) {} - a number that does not parse falls through to the StringTag
        let __parsed: Option<Tag> = (|| {
            if FLOAT_PATTERN.matcher(input).matches() {
                return Some(float_tag::value_of(compat::substring2(&input, 0, (compat::str_len(&input)).wrapping_sub(1)).parse::<f32>().ok()?));
            }
            if BYTE_PATTERN.matcher(input).matches() {
                return Some(byte_tag::value_of(compat::substring2(&input, 0, (compat::str_len(&input)).wrapping_sub(1)).parse::<i8>().ok()?));
            }
            if LONG_PATTERN.matcher(input).matches() {
                return Some(long_tag::value_of(compat::substring2(&input, 0, (compat::str_len(&input)).wrapping_sub(1)).parse::<i64>().ok()?));
            }
            if SHORT_PATTERN.matcher(input).matches() {
                return Some(short_tag::value_of(compat::substring2(&input, 0, (compat::str_len(&input)).wrapping_sub(1)).parse::<i16>().ok()?));
            }
            if INT_PATTERN.matcher(input).matches() {
                return Some(int_tag::value_of(input.parse::<i32>().ok()?));
            }
            if DOUBLE_PATTERN.matcher(input).matches() {
                return Some(double_tag::value_of(compat::substring2(&input, 0, (compat::str_len(&input)).wrapping_sub(1)).parse::<f64>().ok()?));
            }
            if DOUBLE_PATTERN_IMPLICIT.matcher(input).matches() {
                return Some(double_tag::value_of(input.parse::<f64>().ok()?));
            }
            if compat::equals_ignore_case(&"true", input) {
                return Some(mcrust::net::minecraft::nbt::byte_tag::one());
            }
            if compat::equals_ignore_case(&"false", input) {
                return Some(mcrust::net::minecraft::nbt::byte_tag::zero());
            }
            None
        })();
        if let Some(tag) = __parsed {
            return tag;
        }
        return string_tag::value_of(input.to_string());
    }

    /// java: public Tag parseElement() throws CommandSyntaxException
    pub fn parse_element(&mut self) -> Tag {
        self.reader.skip_whitespace();
        if !self.reader.can_read() {
            panic!("{:?}", EXPECTED_VALUE.create_with_context(&self.reader));
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
        let mut nbt_compound: Tag = Tag::compound();
        self.reader.skip_whitespace();
        while self.reader.can_read() && self.reader.peek() != '}' {
            let mut i: i32 = self.reader.get_cursor();
            let mut string: String = self.read_string();
            if string.is_empty() {
                self.reader.set_cursor(i);
                panic!("{:?}", EXPECTED_KEY.create_with_context(&self.reader));
            }
            self.expect(':');
            let element: Tag = self.parse_element();
            nbt_compound.put(&string, element);
            if !self.read_comma() {
                break;
            }
            if !self.reader.can_read() {
                panic!("{:?}", EXPECTED_KEY.create_with_context(&self.reader));
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
            panic!("{:?}", EXPECTED_VALUE.create_with_context(&self.reader));
        } else {
            // java: ListTag nbtList = new ListTag(); - its elements (the port's list functions work on them); the ListTag
            // is built from them where it is returned
            let mut nbt_list: Vec<Tag> = Vec::new();
            let mut list_type: Option<&'static dyn TagType> = None;
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
                let element_type: &'static dyn TagType = element.get_type();
                if list_type.is_none() {
                    list_type = Some(element_type);
                } else if !std::ptr::addr_eq(element_type as *const dyn TagType, list_type.unwrap() as *const dyn TagType) {
                    self.reader.set_cursor(cursor_position);
                    panic!("{:?}", LIST_MIXED.create_with_context(&self.reader, &element_type.get_pretty_name(), &list_type.unwrap().get_pretty_name()));
                }
                // java: nbtList.add(element); - AbstractList.add(e) is add(size(), e)
                let __end: usize = nbt_list.len();
                list_tag::add(&mut nbt_list, __end, element);
                if !self.read_comma() {
                    break;
                }
                if !self.reader.can_read() {
                    panic!("{:?}", EXPECTED_VALUE.create_with_context(&self.reader));
                }
            }
            self.expect(']');
            return list_tag::of_list(nbt_list);
        }
    }

    /// java: private Tag parseElementPrimitiveArray() throws CommandSyntaxException
    pub fn parse_element_primitive_array(&mut self) -> Tag {
        self.expect('[');
        let mut i: i32 = self.reader.get_cursor();
        let mut c: char = self.reader.read();
        self.reader.read();
        self.reader.skip_whitespace();
        if !self.reader.can_read() {
            panic!("{:?}", EXPECTED_VALUE.create_with_context(&self.reader));
        } else if c == 'B' {
            // java: return new ByteArrayTag(mapToByteArray(this.readArray(ByteArrayTag.TYPE, ByteTag.TYPE)));
            return Tag::ByteArray(LegacyStringNbtReader::map_to_byte_array(&self.read_array(mcrust::net::minecraft::nbt::byte_array_tag::get_type(), mcrust::net::minecraft::nbt::byte_tag::get_type())));
        } else if c == 'L' {
            // java: return new LongArrayTag(this.readArray(LongArrayTag.TYPE, LongTag.TYPE).stream().mapToLong(Number::longValue).toArray());
            return Tag::LongArray(self.read_array(mcrust::net::minecraft::nbt::long_array_tag::get_type(), mcrust::net::minecraft::nbt::long_tag::get_type()).into_iter().map(|n| n.long_value()).collect());
        } else if c == 'I' {
            // java: return new IntArrayTag(this.readArray(IntArrayTag.TYPE, IntTag.TYPE).stream().mapToInt(Number::intValue).toArray());
            return Tag::IntArray(self.read_array(mcrust::net::minecraft::nbt::int_array_tag::get_type(), mcrust::net::minecraft::nbt::int_tag::get_type()).into_iter().map(|n| n.int_value()).collect());
        } else {
            self.reader.set_cursor(i);
            panic!("{:?}", ARRAY_INVALID.create_with_context(&self.reader, &c.to_string()));
        }
    }

    /// java: @SuppressWarnings("unchecked") private <T extends Number> List<T> readArray(TagType<?> arrayTypeReader, TagType<?> typeReader) throws CommandSyntaxException
    pub fn read_array(&mut self, array_type_reader: &'static dyn TagType, type_reader: &'static dyn TagType) -> Vec<Number> {
        let mut list: Vec<Number> = vec![];
        while self.reader.peek() != ']' {
            let mut i: i32 = self.reader.get_cursor();
            let mut nbt_element: Tag = self.parse_element();
            let nbt_type: &'static dyn TagType = nbt_element.get_type();
            if !std::ptr::addr_eq(nbt_type as *const dyn TagType, type_reader as *const dyn TagType) {
                self.reader.set_cursor(i);
                panic!("{:?}", ARRAY_MIXED.create_with_context(&self.reader, &nbt_type.get_pretty_name(), &array_type_reader.get_pretty_name()));
            }
            // java: list.add((T) ((ByteTag) nbtElement).byteValue()) / LongTag.longValue() / IntTag.intValue() (boxed)
            if std::ptr::addr_eq(type_reader as *const dyn TagType, mcrust::net::minecraft::nbt::byte_tag::get_type() as *const dyn TagType) {
                list.push(Number::Byte(nbt_element.byte_value().expect("ClassCastException: not a ByteTag")));
            } else if std::ptr::addr_eq(type_reader as *const dyn TagType, mcrust::net::minecraft::nbt::long_tag::get_type() as *const dyn TagType) {
                list.push(Number::Long(nbt_element.long_value().expect("ClassCastException: not a LongTag")));
            } else {
                list.push(Number::Int(nbt_element.int_value().expect("ClassCastException: not an IntTag")));
            }
            if !self.read_comma() {
                break;
            }
            if !self.reader.can_read() {
                panic!("{:?}", EXPECTED_VALUE.create_with_context(&self.reader));
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
