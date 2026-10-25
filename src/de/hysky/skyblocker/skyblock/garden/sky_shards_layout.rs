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

    /// java: public static int @Nullable [][] decode(String code)
    /// (dropped by the converter; `int[][]` indexed `[x][z]`, `None` for Java's `null`)
    pub fn decode(code: &str) -> Option<Vec<Vec<i32>>> {
        // java: try {..} catch (Exception _) { return null; } - an exception here is a panic, caught by `catch_unwind`
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| -> Option<Vec<Vec<i32>>> {
            let grid_string: String = SkyShardsLayout::decompress(code)?;
            // java: String[] parts = gridString.split("\\|", -1);
            let parts: Vec<String> = compat::regex::split_limit(&grid_string, "\\|", -1);
            if parts.len() != 3 {
                return None;
            }
            let input_palette: Vec<i32> = SkyShardsLayout::parse_palette(&parts[0])?;
            let target_palette: Vec<i32> = SkyShardsLayout::parse_palette(&parts[1])?;
            let grid: &String = &parts[2];
            let cell_width: i32 = if compat::str_len(grid) == *TOTAL_CELLS { 1 } else if compat::str_len(grid) == TOTAL_CELLS.wrapping_mul(2) { 2 } else { 0 };
            if cell_width == 0 {
                return None;
            }
            // Decoded in screen space first, so placements expand from their top left corner
            let mut screen: Vec<Vec<i32>> = vec![vec![-1; GRID_SIZE as usize]; GRID_SIZE as usize];
            for cell in 0..*TOTAL_CELLS {
                let chars: String = compat::substring2(grid, cell.wrapping_mul(cell_width), (cell.wrapping_add(1)).wrapping_mul(cell_width));
                let palette_index: i32 = SkyShardsLayout::palette_index_of(&chars);
                if palette_index < 0 {
                    continue;
                }
                // java: boolean target = chars.equals(chars.toUpperCase(Locale.ENGLISH));
                let target: bool = chars == chars.to_uppercase();
                let palette: &Vec<i32> = if target { &target_palette } else { &input_palette };
                if palette_index >= palette.len() as i32 {
                    continue;
                }
                let crop_id: i32 = palette[palette_index as usize];
                let value: i32 = if target { 0 } else { crop_id }; // Desired mutation spot should be empty
                let row: i32 = cell / GRID_SIZE;
                let col: i32 = cell % GRID_SIZE;
                let size: i32 = SkyShardsLayout::size_of(crop_id);
                let mut dr: i32 = 0;
                while dr < size && row + dr < GRID_SIZE {
                    let mut dc: i32 = 0;
                    while dc < size && col + dc < GRID_SIZE {
                        screen[(row + dr) as usize][(col + dc) as usize] = value;
                        dc += 1;
                    }
                    dr += 1;
                }
            }
            // The top of the layout faces north, so rows run along +z and columns along +x
            let mut target: Vec<Vec<i32>> = vec![vec![0; GRID_SIZE as usize]; GRID_SIZE as usize];
            for row in 0..GRID_SIZE as usize {
                for col in 0..GRID_SIZE as usize {
                    target[col][row] = screen[row][col];
                }
            }
            Some(target)
        })) {
            Ok(layout) => layout,
            Err(_) => None,
        }
    }

    /// java: private static int @Nullable [] parsePalette(String palette)
    /// (dropped by the converter)
    pub fn parse_palette(palette: &str) -> Option<Vec<i32>> {
        if palette.is_empty() {
            return Some(Vec::new());
        }
        // java: String[] indices = palette.split(",", -1);
        let indices: Vec<String> = compat::regex::split_limit(palette, ",", -1);
        let mut crop_ids: Vec<i32> = vec![0; indices.len()];
        for i in 0..indices.len() {
            // java: index = Integer.parseInt(indices[i], 36); catch (NumberFormatException _) { return null; }
            // (`parseInt` takes a leading sign and no surrounding blanks, as `from_str_radix`)
            let index: i32 = match i32::from_str_radix(&indices[i], 36) {
                Ok(index) => index,
                Err(_) => return None,
            };
            if index < 0 || index >= (PALETTE_CROPS.len() as i32).wrapping_add(MUTATION_COUNT) {
                return None;
            }
            if index < PALETTE_CROPS.len() as i32 {
                let name: &String = &PALETTE_CROPS[index as usize];
                // java: Objects.requireNonNull(GreenhouseCrops.CROP_ID_MAP.get(name), name).id()
                crop_ids[i] = crate::de::hysky::skyblocker::skyblock::garden::greenhouse_crops::CROP_ID_MAP.get().get(name).unwrap_or_else(|| panic!("NullPointerException: {}", name)).id();
            } else {
                crop_ids[i] = (index - PALETTE_CROPS.len() as i32) + 1;
            }
        }
        Some(crop_ids)
    }

    /// java: private static @Nullable String decompress(String code) throws Exception
    /// (`None` is Java's `null`, and also an exception, which `decode`'s `catch (Exception _)` turns into `null`)
    pub fn decompress(code: &str) -> Option<String> {
        // java: byte[] compressed = Base64.getUrlDecoder().decode(code);
        // `Decoder.RFC4648_URLSAFE` is the basic decoder over the URL alphabet (`fromBase64URL`): `-` and `_` where the
        // basic one has `+` and `/`, which it rejects
        if let Some(illegal) = code.chars().find(|c| *c == '+' || *c == '/') {
            let _ = illegal;
            return None; // IllegalArgumentException: Illegal base64 character
        }
        let basic: String = code.chars().map(|c| match c { '-' => '+', '_' => '/', c => c }).collect();
        let compressed: Vec<u8> = mcrust::java::util::base64::get_decoder().decode(&basic).ok()?;
        {
            // java: InputStream in = new InflaterInputStream(new ByteArrayInputStream(compressed), new Inflater(true));
            // (`nowrap`: raw DEFLATE, no zlib header or trailer)
            let inflated: Vec<u8> = mcrust::lwjgl::inflate::inflate(&compressed).ok()?;
            // java: byte[] decompressed = in.readNBytes(MAX_DECOMPRESSED_BYTES);
            // java: if (in.read() != -1) return null;
            if inflated.len() > MAX_DECOMPRESSED_BYTES as usize {
                return None;
            }
            let decompressed: Vec<u8> = inflated;
            // new String(decompressed, StandardCharsets.UTF_8): malformed input becomes U+FFFD
            return Some(String::from_utf8_lossy(&decompressed).into_owned());
        }
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
