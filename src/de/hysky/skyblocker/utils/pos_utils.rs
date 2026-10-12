//! Ported from de.hysky.skyblocker.utils.PosUtils by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::com::mojang::serialization::codec;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::com::mojang::serialization::codecs::record_codec_builder;
use mcrust::net::minecraft::core::block_pos::BlockPos;

/// java: public static final Codec<BlockPos> ALT_BLOCK_POS_CODEC = RecordCodecBuilder.create(instance -> instance.group( Codec.INT.fieldOf("x").forGetter(BlockPos::getX), Codec.INT.fieldOf("y").forGetter(Block
pub static ALT_BLOCK_POS_CODEC: compat::MainThread<CodecRef<BlockPos>> = compat::MainThread::new(|| record_codec_builder::create(record_codec_builder::apply3(record_codec_builder::of(|o: &BlockPos| o.get_x(), codec::field_of(mcrust::com::mojang::serialization::codec::int_codec(), "x")), record_codec_builder::of(|o: &BlockPos| o.get_y(), codec::field_of(mcrust::com::mojang::serialization::codec::int_codec(), "y")), record_codec_builder::of(|o: &BlockPos| o.get_z(), codec::field_of(mcrust::com::mojang::serialization::codec::int_codec(), "z")), move |__a0, __a1, __a2| BlockPos::new(__a0.clone(), __a1.clone(), __a2.clone()))));

/// java: public final class PosUtils
#[derive(Clone)]
pub struct PosUtils {}

impl PosUtils {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static BlockPos parsePosString(String posData)
    pub fn parse_pos_string(pos_data: &str) -> BlockPos {
        let mut pos_array: Vec<String> = compat::regex::split(&pos_data, ",");
        return BlockPos::new((pos_array[(0) as usize]).parse::<i32>().expect("NumberFormatException"), (pos_array[(1) as usize]).parse::<i32>().expect("NumberFormatException"), (pos_array[(2) as usize]).parse::<i32>().expect("NumberFormatException"));
    }

    /// java: public static BlockPos parsePosJson(JsonObject posJson)
    pub fn parse_pos_json(pos_json: &serde_json::Map<String, serde_json::Value>) -> BlockPos {
        return BlockPos::new(compat::json::get_as_int(&pos_json.get("x").cloned().unwrap()), compat::json::get_as_int(&pos_json.get("y").cloned().unwrap()), compat::json::get_as_int(&pos_json.get("z").cloned().unwrap()));
    }

    /// java: public static String getPosString(BlockPos blockPos)
    pub fn get_pos_string(block_pos: &BlockPos) -> String {
        return format!("{},{},{}", block_pos.get_x(), block_pos.get_y(), block_pos.get_z());
    }

    /// java: public static String toSpaceSeparatedString(BlockPos blockPos)
    pub fn to_space_separated_string(block_pos: &BlockPos) -> String {
        return format!("{} {} {}", block_pos.get_x(), block_pos.get_y(), block_pos.get_z());
    }

}
