//! Ported from de.hysky.skyblocker.skyblock.dungeon.puzzle.waterboard.Waterboard by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::com::mojang::brigadier::context::command_context::CommandContext;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::net::minecraft::commands::arguments::string_representable_argument::StringRepresentableArgument;
use mcrust::net::minecraft::core::block_pos::BlockPos;
use mcrust::net::minecraft::world::item::dye_color::DyeColor;
use mcrust::net::minecraft::world::level::block::block::Block;
use mcrust::net::minecraft::world::level::block::blocks;

/// java: public static final int BOARD_MIN_X = 6;
pub const BOARD_MIN_X: i32 = 6;
/// java: public static final int BOARD_MAX_X = 24;
pub const BOARD_MAX_X: i32 = 24;
/// java: public static final int BOARD_MIN_Y = 58;
pub const BOARD_MIN_Y: i32 = 58;
/// java: public static final int BOARD_MAX_Y = 81;
pub const BOARD_MAX_Y: i32 = 81;
/// java: public static final int BOARD_Z = 26;
pub const BOARD_Z: i32 = 26;
/// java: public static final BlockPos WATER_ENTRANCE_POSITION = new BlockPos(15, 78, 26);
pub static WATER_ENTRANCE_POSITION: compat::MainThread<BlockPos> = compat::MainThread::new(|| BlockPos::new(15, 78, 26));
// TODO(fabric2seer): `from_enum` takes 2 argument(s) in the port (`values: Vec<E>, serialized_name: impl Fn(&E) -> String + Send + Sync + Copy + 'static`), Java passes 1
//   java: private static final Codec<LeverType> CODEC = StringRepresentable.fromEnum(LeverType::values);
/// java: private static final Codec<LeverType> CODEC = StringRepresentable.fromEnum(LeverType::values);
pub static LEVER_TYPE_CODEC: compat::MainThread<CodecRef<LeverType>> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `CODEC` - see the TODO above"));

/// java: public class Waterboard
#[derive(Clone)]
pub struct Waterboard {}

impl Waterboard {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

}

/// java: public enum LeverType implements StringRepresentable
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum LeverType {
    Coal,
    Gold,
    Quartz,
    Diamond,
    Emerald,
    Terracotta,
    Water,
}

impl LeverType {
    pub const VALUES: [LeverType; 7] = [LeverType::Coal, LeverType::Gold, LeverType::Quartz, LeverType::Diamond, LeverType::Emerald, LeverType::Terracotta, LeverType::Water];

    /// java: values()
    pub fn values() -> Vec<LeverType> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            LeverType::Coal => "COAL",
            LeverType::Gold => "GOLD",
            LeverType::Quartz => "QUARTZ",
            LeverType::Diamond => "DIAMOND",
            LeverType::Emerald => "EMERALD",
            LeverType::Terracotta => "TERRACOTTA",
            LeverType::Water => "WATER",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> LeverType {
        match name {
            "COAL" => LeverType::Coal,
            "GOLD" => LeverType::Gold,
            "QUARTZ" => LeverType::Quartz,
            "DIAMOND" => LeverType::Diamond,
            "EMERALD" => LeverType::Emerald,
            "TERRACOTTA" => LeverType::Terracotta,
            "WATER" => LeverType::Water,
            _ => panic!("IllegalArgumentException: No enum constant LeverType.{name}"),
        }
    }

    /// the constant's constructor data (Java keeps it in the constant's fields)
    pub fn fields(self) -> &'static LeverTypeFields {
        static FIELDS: LazyLock<Vec<LeverTypeFields>> = LazyLock::new(|| vec![
            LeverTypeFields::new(blocks::blocks().get("coal_block"), BlockPos::new(20, 61, 10), DyeColor::Red, Some(vec![Some(BlockPos::new(0, -2, 0)), Some(BlockPos::new(2, -1, 1)), None, Some(BlockPos::new(5, -1, 0))])),
            LeverTypeFields::new(blocks::blocks().get("gold_block"), BlockPos::new(20, 61, 15), DyeColor::Yellow, Some(vec![Some(BlockPos::new(1, -1, 0)), Some(BlockPos::new(3, -2, 0)), Some(BlockPos::new(-4, -1, 1)), Some(BlockPos::new(1, 0, 0))])),
            LeverTypeFields::new(blocks::blocks().get("quartz_block"), BlockPos::new(20, 61, 20), DyeColor::LightGray, Some(vec![Some(BlockPos::new(1, -4, 1)), Some(BlockPos::new(-1, 0, 0)), Some(BlockPos::new(1, 0, 0)), Some(BlockPos::new(-1, 0, 1))])),
            LeverTypeFields::new(blocks::blocks().get("diamond_block"), BlockPos::new(10, 61, 20), DyeColor::Cyan, Some(vec![Some(BlockPos::new(0, -5, 1)), Some(BlockPos::new(-2, -1, 0)), Some(BlockPos::new(-1, 0, 1)), Some(BlockPos::new(-3, -4, 1))])),
            LeverTypeFields::new(blocks::blocks().get("emerald_block"), BlockPos::new(10, 61, 15), DyeColor::Lime, Some(vec![Some(BlockPos::new(-1, -10, 1)), Some(BlockPos::new(1, 0, 1)), Some(BlockPos::new(-6, 0, 0)), Some(BlockPos::new(1, -4, 0))])),
            LeverTypeFields::new(blocks::blocks().get("terracotta"), BlockPos::new(10, 61, 10), DyeColor::Orange, Some(vec![Some(BlockPos::new(-1, -1, 1)), Some(BlockPos::new(0, -3, 1)), None, Some(BlockPos::new(-4, -5, 1))])),
            LeverTypeFields::new(blocks::blocks().get("lava"), BlockPos::new(15, 60, 5), DyeColor::LightBlue, None),
        ]);
        &FIELDS[self as usize]
    }

    /// java: public static @Nullable LeverType fromName(String name)
    pub fn from_name(name: &str) -> Option<LeverType> {
        for lever_type in LeverType::values() {
            if compat::equals_ignore_case(&lever_type.name(), name) {
                return Some(lever_type);
            }
        }
        return None;
    }

    /// java: public static @Nullable LeverType fromBlock(Block block)
    pub fn from_block(block: &Arc<Block>) -> Option<LeverType> {
        for lever_type in LeverType::values() {
            if std::ptr::addr_eq(std::sync::Arc::as_ptr(&lever_type.fields().block), std::sync::Arc::as_ptr(block)) {
                return Some(lever_type);
            }
        }
        return None;
    }

    /// java: public static @Nullable LeverType fromPos(BlockPos leverPos)
    pub fn from_pos(lever_pos: &BlockPos) -> Option<LeverType> {
        for lever_type in LeverType::values() {
            if *lever_pos == lever_type.fields().lever_pos {
                return Some(lever_type);
            }
        }
        return None;
    }

    /// java: @Override public String getSerializedName()
    pub fn get_serialized_name(&self) -> String {
        // TODO(fabric2seer): call to `name(..)` (not a method of this class)
        //   java: return name().toLowerCase(Locale.ENGLISH);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}

impl std::fmt::Display for LeverType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

/// the constructor data of each LeverType constant
pub struct LeverTypeFields {
    pub block: Arc<Block>,
    pub lever_pos: BlockPos,
    pub color: DyeColor,
    /// java: public final @Nullable BlockPos[] initialPositions - elements may be null too
    pub initial_positions: Option<Vec<Option<BlockPos>>>,
}

impl LeverTypeFields {
    fn new(mut block: Arc<Block>, mut lever_pos: BlockPos, mut color: DyeColor, mut initial_positions: Option<Vec<Option<BlockPos>>>) -> Self {
        let mut this = Self {
            block: block.clone(),
            lever_pos: lever_pos.clone(),
            color: color,
            initial_positions: initial_positions,
        };
        this
    }

}

/// java: public static class LeverTypeArgumentType extends StringRepresentableArgument<LeverType>
pub struct LeverTypeArgumentType {
    /// java: extends net.minecraft.commands.arguments.StringRepresentableArgument
    pub base: StringRepresentableArgument<compat::Object>,
}

impl std::ops::Deref for LeverTypeArgumentType {
    type Target = StringRepresentableArgument<compat::Object>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for LeverTypeArgumentType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl LeverTypeArgumentType {
    /// java: private LeverTypeArgumentType()
    pub fn new() -> Self {
        // TODO(fabric2seer): `new` takes 3 argument(s) in the port (`values: Vec<T>, codec_name: fn(&T) -> String, convert_id: fn(&str) -> String`), Java passes 2
        //   java: super(CODEC, LeverType::values);
        let mut this: Self = todo!("constructor");
        this
    }

    /// java: public static LeverTypeArgumentType leverType()
    pub fn lever_type() -> LeverTypeArgumentType {
        return LeverTypeArgumentType::new();
    }

    /// java: public static <S> LeverType getLeverType(CommandContext<S> context, String name)
    pub fn get_lever_type<S: 'static>(context: &CommandContext<S>, name: &str) -> LeverType {
        return context.get_argument::<LeverType>(name).cloned().unwrap();
    }

}
