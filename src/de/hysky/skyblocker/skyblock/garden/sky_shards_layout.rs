//! Ported from de.hysky.skyblocker.skyblock.garden.SkyShardsLayout by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::java::text::decimal_format_symbols::Locale;

/// java: private static final int GRID_SIZE = 10;
pub const GRID_SIZE: i32 = 10;
/// java: private static final int TOTAL_CELLS = GRID_SIZE * GRID_SIZE;
pub static TOTAL_CELLS: LazyLock<i32> = LazyLock::new(|| GRID_SIZE.wrapping_mul(GRID_SIZE));
/// java: private static final int LETTERS = 26;
pub const LETTERS: i32 = 26;
/// java: private static final int MAX_DECOMPRESSED_BYTES = 4096;
pub const MAX_DECOMPRESSED_BYTES: i32 = 4096;
/// java: private static final String[] PALETTE_CROPS = { "Wheat Seeds", "Potato", "Carrot", "Pumpkin Seeds", "Melon Seeds", "Cocoa Beans", "Sugar Cane", "Cactus", "Nether Wart", "Red Mushroom", "Brown Mushroom
pub static PALETTE_CROPS: LazyLock<Vec<String>> = LazyLock::new(|| vec!["Wheat Seeds".to_string(), "Potato".to_string(), "Carrot".to_string(), "Pumpkin Seeds".to_string(), "Melon Seeds".to_string(), "Cocoa Beans".to_string(), "Sugar Cane".to_string(), "Cactus".to_string(), "Nether Wart".to_string(), "Red Mushroom".to_string(), "Brown Mushroom".to_string(), "Moonflower".to_string(), "Sunflower".to_string(), "Wild Rose".to_string(), "Fire".to_string(), "Dead Plants".to_string(), "Fermento".to_string()]);
/// java: private static final int MUTATION_COUNT = 40;
pub const MUTATION_COUNT: i32 = 40;

/// java: public final class SkyShardsLayout
#[derive(Clone)]
pub struct SkyShardsLayout {}

impl SkyShardsLayout {
    /// java: private SkyShardsLayout()
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: private static @Nullable String decompress(String code) throws Exception
    pub fn decompress(code: &str) -> Option<String> {
        // rust: port gap - `Base64.getUrlDecoder()` and `new Inflater(true)` (raw deflate) are not ported (GAPS_batchG)
        // TODO(fabric2seer): `.decode(..)` on a value of unknown type
        //   java: byte[] compressed = Base64.getUrlDecoder().decode(code);
        let mut compressed: Vec<u8> = todo!("fabric2seer: the initialiser of `compressed` (TODO above)");
        {
            // TODO(fabric2seer): `new InflaterInputStream(..)`
            //   java: InputStream in = new InflaterInputStream(new ByteArrayInputStream(compressed), new Inflater(true));
            let mut r#in: crate::unresolved::InputStream = todo!("fabric2seer: the initialiser of `in` (TODO above)");
            // TODO(fabric2seer): `.readNBytes(..)` on a InputStream
            //   java: byte[] decompressed = in.readNBytes(MAX_DECOMPRESSED_BYTES);
            let mut decompressed: Vec<u8> = todo!("fabric2seer: the initialiser of `decompressed` (TODO above)");
            // TODO(fabric2seer): the condition: `.read(..)` on a InputStream
            //   java: in.read() != -1
            if todo!("fabric2seer: the condition (TODO above)") {
                return None;
            }
            // new String(decompressed, StandardCharsets.UTF_8): malformed input becomes U+FFFD
            return Some(String::from_utf8_lossy(&decompressed).into_owned());
        }
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: private static int paletteIndexOf(String chars)
    pub fn palette_index_of(chars: &str) -> i32 {
        let mut index: i32 = 0;
        for i in 0..compat::str_len(&chars) {
            let mut c: char = compat::char_lower(compat::char_at(&chars, i));
            if c < 'a' || c > 'z' {
                return -1;
            }
            index = (index.wrapping_mul(LETTERS)).wrapping_add((c as u32 as i32).wrapping_sub('a' as u32 as i32));
        }
        if !(chars == chars.to_uppercase()) && !(chars == chars.to_lowercase()) {
            return -1;
        }
        return index;
    }

    /// java: private static int sizeOf(int cropId)
    pub fn size_of(mut crop_id: i32) -> i32 {
        return match crop_id {
            23 | 37 => 3,
            22 | 26 | 36 => 2,
            _ => 1,
        };
    }

}
