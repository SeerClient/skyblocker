//! Ported from de.hysky.skyblocker.skyblock.dwarven.MiningLocationLabel by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_collector::PrimitiveCollector;
use crate::de::hysky::skyblocker::utils::render::renderable::Renderable;
use crate::de::hysky::skyblocker::utils::waypoint::distanced_named_waypoint::DistancedNamedWaypoint;
use crate::de::hysky::skyblocker::utils::waypoint::distanced_named_waypoint::DistancedNamedWaypointBase;
use crate::de::hysky::skyblocker::utils::waypoint::named_waypoint::NamedWaypoint;
use crate::de::hysky::skyblocker::utils::waypoint::named_waypoint::NamedWaypointBase;
use crate::de::hysky::skyblocker::utils::waypoint::waypoint::Type;
use crate::de::hysky::skyblocker::utils::waypoint::waypoint::Waypoint;
use crate::de::hysky::skyblocker::utils::waypoint::waypoint::WaypointBase;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::java::awt::color::Color;
use mcrust::net::minecraft::core::block_pos::BlockPos;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::world::item::dye_color::DyeColor;

/// java: private static final String CRYSTALS_SPACER = " ";
pub const CRYSTALS_SPACER: &str = "                                ";
// TODO(fabric2seer): `from_values` takes 2 argument(s) in the port (`values: Vec<E>, serialized_name: impl Fn(&E) -> String + Send + Sync + Copy + 'static`), Java passes 1
//   java: public static final Codec<CrystalHollowsLocationsCategory> CODEC = StringRepresentable.fromValues(CrystalHollowsLocationsCategory::values);
/// java: public static final Codec<CrystalHollowsLocationsCategory> CODEC = StringRepresentable.fromValues(CrystalHollowsLocationsCategory::values);
/// The port's `fromValues` is handed the values and the `getSerializedName` it calls on each.
pub static CRYSTAL_HOLLOWS_LOCATIONS_CATEGORY_CODEC: compat::MainThread<CodecRef<CrystalHollowsLocationsCategory>> = compat::MainThread::new(|| mcrust::net::minecraft::util::string_representable::from_values(CrystalHollowsLocationsCategory::values(), |value: &CrystalHollowsLocationsCategory| value.get_serialized_name()));

/// java: public class MiningLocationLabel extends DistancedNamedWaypoint
#[derive(Clone)]
pub struct MiningLocationLabel {
    /// java: extends DistancedNamedWaypoint
    pub base: DistancedNamedWaypointBase,
    /// java: private final Category category;
    pub category: compat::Shared<dyn Category>,
}

impl std::ops::Deref for MiningLocationLabel {
    type Target = DistancedNamedWaypointBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for MiningLocationLabel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl MiningLocationLabel {
    /// java: public MiningLocationLabel(Category category, BlockPos pos)
    pub fn new(mut category: compat::Shared<dyn Category>, mut pos: BlockPos) -> compat::Shared<Self> {
        let __init_category = category.clone();
        let (_, __this) = DistancedNamedWaypointBase::new_in(pos.clone(), MiningLocationLabel::get_name(&category), vec![((0) as f32) as f32, ((0) as f32) as f32, ((0) as f32) as f32], false, move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                category: __init_category,
            });
            ((__h.clone() as compat::Shared<dyn DistancedNamedWaypoint>), __h)
        });
        __this
    }

    /// java: private static Component getName(Category category)
    pub fn get_name(category: &compat::Shared<dyn Category>) -> MutableComponent {
        if SkyblockerConfigManager::get().mining.commission_waypoints.use_color {
            return (component::literal(&({ let __f = compat::dispatch(category, |__t| __t.get_name()); __f(category) }))).with_color(({ let __f = compat::dispatch(category, |__t| __t.get_color()); __f(category) }));
        }
        return component::literal(&({ let __f = compat::dispatch(category, |__t| __t.get_name()); __f(category) }));
    }

    /// java: public Category category()
    pub fn category(__this: &compat::Shared<Self>) -> compat::Shared<dyn Category> {
        return compat::read(__this, |__t| __t.category.clone());
    }

}

impl DistancedNamedWaypoint for MiningLocationLabel {
    fn distanced_named_waypoint_base(&self) -> &DistancedNamedWaypointBase {
        &self.base
    }
    fn distanced_named_waypoint_base_mut(&mut self) -> &mut DistancedNamedWaypointBase {
        &mut self.base
    }

}

impl NamedWaypoint for MiningLocationLabel {
    fn named_waypoint_base(&self) -> &NamedWaypointBase {
        &self.base.base
    }
    fn named_waypoint_base_mut(&mut self) -> &mut NamedWaypointBase {
        &mut self.base.base
    }

    /// java: @Override protected boolean shouldRenderName() (dispatch: the body for this class)
    fn should_render_name(&self) -> fn(&compat::Shared<dyn NamedWaypoint>) -> bool {
        MiningLocationLabel::should_render_name_dyn
    }

}

impl Waypoint for MiningLocationLabel {
    fn waypoint_base(&self) -> &WaypointBase {
        &self.base.base.base
    }
    fn waypoint_base_mut(&mut self) -> &mut WaypointBase {
        &mut self.base.base.base
    }

    /// java: @Override public NamedWaypoint withX(int x) (dispatch: the body for this class)
    fn with_x(&self) -> fn(&compat::Shared<dyn Waypoint>, i32) -> compat::Shared<dyn Waypoint> {
        <dyn NamedWaypoint>::with_x_from::<Self>
    }

    /// java: @Override public NamedWaypoint withY(int y) (dispatch: the body for this class)
    fn with_y(&self) -> fn(&compat::Shared<dyn Waypoint>, i32) -> compat::Shared<dyn Waypoint> {
        <dyn NamedWaypoint>::with_y_from::<Self>
    }

    /// java: @Override public NamedWaypoint withZ(int z) (dispatch: the body for this class)
    fn with_z(&self) -> fn(&compat::Shared<dyn Waypoint>, i32) -> compat::Shared<dyn Waypoint> {
        <dyn NamedWaypoint>::with_z_from::<Self>
    }

    /// java: @Override public NamedWaypoint withColor(float[] colorComponents, float alpha) (dispatch: the body for this class)
    fn with_color(&self) -> fn(&compat::Shared<dyn Waypoint>, &Vec<f32>, f32) -> compat::Shared<dyn Waypoint> {
        <dyn NamedWaypoint>::with_color_from::<Self>
    }

    /// java: @Override public NamedWaypoint withThroughWalls(boolean throughWalls) (dispatch: the body for this class)
    fn with_through_walls(&self) -> fn(&compat::Shared<dyn Waypoint>, bool) -> compat::Shared<dyn Waypoint> {
        <dyn NamedWaypoint>::with_through_walls_from::<Self>
    }

    /// java: @Override public NamedWaypoint withTypeSupplier(Supplier<Type> typeSupplier) (dispatch: the body for this class)
    fn with_type_supplier(&self) -> fn(&compat::Shared<dyn Waypoint>, Arc<dyn Fn() -> Type>) -> compat::Shared<dyn Waypoint> {
        <dyn NamedWaypoint>::with_type_supplier_from::<Self>
    }

    /// java: @Override public int hashCode() (dispatch: the body for this class)
    fn hash_code(&self) -> fn(&compat::Shared<dyn Waypoint>) -> i32 {
        <dyn NamedWaypoint>::hash_code_from::<Self>
    }

    /// java: @Override public boolean equals(Object obj) (dispatch: the body for this class)
    fn equals(&self) -> fn(&compat::Shared<dyn Waypoint>, &compat::Object) -> bool {
        <dyn NamedWaypoint>::equals_from::<Self>
    }

}

impl Renderable for MiningLocationLabel {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void extractRendering(PrimitiveCollector collector) (dispatch: the body for this class)
    fn extract_rendering(&self) -> fn(&compat::Shared<dyn Renderable>, &compat::Shared<dyn PrimitiveCollector>) {
        <dyn DistancedNamedWaypoint>::extract_rendering_from::<Self>
    }

}

impl MiningLocationLabel {
    /// java: @Override protected boolean shouldRenderName()
    pub fn should_render_name_impl(__this: &compat::Shared<Self>) -> bool {
        return true;
    }

    /// MiningLocationLabel.shouldRenderName as the dispatch of NamedWaypoint.shouldRenderName
    pub fn should_render_name_dyn(__this: &compat::Shared<dyn NamedWaypoint>) -> bool {
        Self::should_render_name_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}

/// java: public interface Category
pub trait Category: compat::DynAny {
    fn as_any(&self) -> &dyn std::any::Any;
    /// java: String getName(); (dispatch: the body for this class)
    fn get_name(&self) -> fn(&compat::Shared<dyn Category>) -> String;

    /// java: int getColor(); (dispatch: the body for this class)
    fn get_color(&self) -> fn(&compat::Shared<dyn Category>) -> i32;

}

/// java: enum DwarvenCategory implements Category
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum DwarvenCategory {
    LavaSprings,
    CliffsideVeins,
    RampartsQuarry,
    UpperMines,
    RoyalMines,
    GlaciteWalker,
}

impl DwarvenCategory {
    pub const VALUES: [DwarvenCategory; 6] = [DwarvenCategory::LavaSprings, DwarvenCategory::CliffsideVeins, DwarvenCategory::RampartsQuarry, DwarvenCategory::UpperMines, DwarvenCategory::RoyalMines, DwarvenCategory::GlaciteWalker];

    /// java: values()
    pub fn values() -> Vec<DwarvenCategory> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            DwarvenCategory::LavaSprings => "LAVA_SPRINGS",
            DwarvenCategory::CliffsideVeins => "CLIFFSIDE_VEINS",
            DwarvenCategory::RampartsQuarry => "RAMPARTS_QUARRY",
            DwarvenCategory::UpperMines => "UPPER_MINES",
            DwarvenCategory::RoyalMines => "ROYAL_MINES",
            DwarvenCategory::GlaciteWalker => "GLACITE_WALKER",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> DwarvenCategory {
        match name {
            "LAVA_SPRINGS" => DwarvenCategory::LavaSprings,
            "CLIFFSIDE_VEINS" => DwarvenCategory::CliffsideVeins,
            "RAMPARTS_QUARRY" => DwarvenCategory::RampartsQuarry,
            "UPPER_MINES" => DwarvenCategory::UpperMines,
            "ROYAL_MINES" => DwarvenCategory::RoyalMines,
            "GLACITE_WALKER" => DwarvenCategory::GlaciteWalker,
            _ => panic!("IllegalArgumentException: No enum constant DwarvenCategory.{name}"),
        }
    }

    /// the constant's constructor data (Java keeps it in the constant's fields)
    pub fn fields(self) -> &'static DwarvenCategoryFields {
        static FIELDS: LazyLock<Vec<DwarvenCategoryFields>> = LazyLock::new(|| vec![
            DwarvenCategoryFields::new("Lava Springs".to_string(), BlockPos::new(60, 197, -15)),
            DwarvenCategoryFields::new("Cliffside Veins".to_string(), BlockPos::new(40, 128, 40)),
            DwarvenCategoryFields::new("Rampart's Quarry".to_string(), BlockPos::new(-100, 150, -20)),
            DwarvenCategoryFields::new("Upper Mines".to_string(), BlockPos::new(-130, 174, -50)),
            DwarvenCategoryFields::new("Royal Mines".to_string(), BlockPos::new(130, 154, 30)),
            DwarvenCategoryFields::new("Glacite Walker".to_string(), BlockPos::new(0, 128, 150)),
        ]);
        &FIELDS[self as usize]
    }

    /// java: public BlockPos getLocation()
    pub fn get_location(&self) -> BlockPos {
        return self.fields().location.clone();
    }

    /// java: @Override public String toString()
    pub fn to_string(&self) -> String {
        return self.fields().name.clone();
    }

    /// java: @Override public String getName()
    pub fn get_name(&self) -> String {
        return self.fields().name.clone();
    }

    /// java: @Override public int getColor()
    pub fn get_color(&self) -> i32 {
        if self.fields().is_titanium.load(std::sync::atomic::Ordering::Relaxed) {
            return 0xD8D6D8;
        }
        return 0x45BDE0;
    }

}

impl std::fmt::Display for DwarvenCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", DwarvenCategory::to_string(self))
    }
}

/// the constructor data of each DwarvenCategory constant
pub struct DwarvenCategoryFields {
    /// java: public boolean isTitanium; - a mutable field of the enum constant (written by `CommissionLabels.update`).
    pub is_titanium: std::sync::atomic::AtomicBool,
    pub name: String,
    pub location: BlockPos,
}

impl DwarvenCategoryFields {
    fn new(mut name: String, mut location: BlockPos) -> Self {
        let mut this = Self {
            is_titanium: std::sync::atomic::AtomicBool::new(false),
            name,
            location,
        };
        this
    }

}

impl Category for DwarvenCategory {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn get_name(&self) -> fn(&compat::Shared<dyn Category>) -> String {
        DwarvenCategory::get_name_dyn
    }

    fn get_color(&self) -> fn(&compat::Shared<dyn Category>) -> i32 {
        DwarvenCategory::get_color_dyn
    }

}

impl DwarvenCategory {
    /// DwarvenCategory.getName as the dispatch of Category.getName
    pub fn get_name_dyn(__this: &compat::Shared<dyn Category>) -> String {
        compat::read_as::<DwarvenCategory, _, _>(__this, |__e| __e.get_name())
    }

    /// DwarvenCategory.getColor as the dispatch of Category.getColor
    pub fn get_color_dyn(__this: &compat::Shared<dyn Category>) -> i32 {
        compat::read_as::<DwarvenCategory, _, _>(__this, |__e| __e.get_color())
    }

}

/// java: enum DwarvenEmissaries implements Category
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum DwarvenEmissaries {
    LavaSprings,
    CliffsideVeins,
    RampartsQuarry,
    UpperMines,
    RoyalMines,
    DwarvenVillage,
    DwarvenMines,
}

impl DwarvenEmissaries {
    pub const VALUES: [DwarvenEmissaries; 7] = [DwarvenEmissaries::LavaSprings, DwarvenEmissaries::CliffsideVeins, DwarvenEmissaries::RampartsQuarry, DwarvenEmissaries::UpperMines, DwarvenEmissaries::RoyalMines, DwarvenEmissaries::DwarvenVillage, DwarvenEmissaries::DwarvenMines];

    /// java: values()
    pub fn values() -> Vec<DwarvenEmissaries> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            DwarvenEmissaries::LavaSprings => "LAVA_SPRINGS",
            DwarvenEmissaries::CliffsideVeins => "CLIFFSIDE_VEINS",
            DwarvenEmissaries::RampartsQuarry => "RAMPARTS_QUARRY",
            DwarvenEmissaries::UpperMines => "UPPER_MINES",
            DwarvenEmissaries::RoyalMines => "ROYAL_MINES",
            DwarvenEmissaries::DwarvenVillage => "DWARVEN_VILLAGE",
            DwarvenEmissaries::DwarvenMines => "DWARVEN_MINES",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> DwarvenEmissaries {
        match name {
            "LAVA_SPRINGS" => DwarvenEmissaries::LavaSprings,
            "CLIFFSIDE_VEINS" => DwarvenEmissaries::CliffsideVeins,
            "RAMPARTS_QUARRY" => DwarvenEmissaries::RampartsQuarry,
            "UPPER_MINES" => DwarvenEmissaries::UpperMines,
            "ROYAL_MINES" => DwarvenEmissaries::RoyalMines,
            "DWARVEN_VILLAGE" => DwarvenEmissaries::DwarvenVillage,
            "DWARVEN_MINES" => DwarvenEmissaries::DwarvenMines,
            _ => panic!("IllegalArgumentException: No enum constant DwarvenEmissaries.{name}"),
        }
    }

    /// the constant's constructor data (Java keeps it in the constant's fields)
    pub fn fields(self) -> &'static DwarvenEmissariesFields {
        static FIELDS: LazyLock<Vec<DwarvenEmissariesFields>> = LazyLock::new(|| vec![
            DwarvenEmissariesFields::new(BlockPos::new(58, 198, -8)),
            DwarvenEmissariesFields::new(BlockPos::new(42, 134, 22)),
            DwarvenEmissariesFields::new(BlockPos::new(-72, 153, -10)),
            DwarvenEmissariesFields::new(BlockPos::new(-132, 174, -50)),
            DwarvenEmissariesFields::new(BlockPos::new(171, 150, 31)),
            DwarvenEmissariesFields::new(BlockPos::new(-37, 200, -131)),
            DwarvenEmissariesFields::new(BlockPos::new(89, 198, -92)),
        ]);
        &FIELDS[self as usize]
    }

    /// java: public BlockPos getLocation()
    pub fn get_location(&self) -> BlockPos {
        return self.fields().location.clone();
    }

    /// java: @Override public String toString()
    pub fn to_string(&self) -> String {
        return "Emissary".to_string();
    }

    /// java: @Override public String getName()
    pub fn get_name(&self) -> String {
        return "Emissary".to_string();
    }

    /// java: @Override public int getColor()
    pub fn get_color(&self) -> i32 {
        return 0xFFFFFF;
    }

}

impl std::fmt::Display for DwarvenEmissaries {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", DwarvenEmissaries::to_string(self))
    }
}

/// the constructor data of each DwarvenEmissaries constant
pub struct DwarvenEmissariesFields {
    pub location: BlockPos,
}

impl DwarvenEmissariesFields {
    fn new(mut location: BlockPos) -> Self {
        let mut this = Self {
            location,
        };
        this
    }

}

impl Category for DwarvenEmissaries {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn get_name(&self) -> fn(&compat::Shared<dyn Category>) -> String {
        DwarvenEmissaries::get_name_dyn
    }

    fn get_color(&self) -> fn(&compat::Shared<dyn Category>) -> i32 {
        DwarvenEmissaries::get_color_dyn
    }

}

impl DwarvenEmissaries {
    /// DwarvenEmissaries.getName as the dispatch of Category.getName
    pub fn get_name_dyn(__this: &compat::Shared<dyn Category>) -> String {
        compat::read_as::<DwarvenEmissaries, _, _>(__this, |__e| __e.get_name())
    }

    /// DwarvenEmissaries.getColor as the dispatch of Category.getColor
    pub fn get_color_dyn(__this: &compat::Shared<dyn Category>) -> i32 {
        compat::read_as::<DwarvenEmissaries, _, _>(__this, |__e| __e.get_color())
    }

}

/// java: enum GlaciteCategory implements Category
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum GlaciteCategory {
    Aquamarine,
    Onyx,
    Peridot,
    Citrine,
    Campfire,
}

impl GlaciteCategory {
    pub const VALUES: [GlaciteCategory; 5] = [GlaciteCategory::Aquamarine, GlaciteCategory::Onyx, GlaciteCategory::Peridot, GlaciteCategory::Citrine, GlaciteCategory::Campfire];

    /// java: values()
    pub fn values() -> Vec<GlaciteCategory> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            GlaciteCategory::Aquamarine => "AQUAMARINE",
            GlaciteCategory::Onyx => "ONYX",
            GlaciteCategory::Peridot => "PERIDOT",
            GlaciteCategory::Citrine => "CITRINE",
            GlaciteCategory::Campfire => "CAMPFIRE",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> GlaciteCategory {
        match name {
            "AQUAMARINE" => GlaciteCategory::Aquamarine,
            "ONYX" => GlaciteCategory::Onyx,
            "PERIDOT" => GlaciteCategory::Peridot,
            "CITRINE" => GlaciteCategory::Citrine,
            "CAMPFIRE" => GlaciteCategory::Campfire,
            _ => panic!("IllegalArgumentException: No enum constant GlaciteCategory.{name}"),
        }
    }

    /// the constant's constructor data (Java keeps it in the constant's fields)
    pub fn fields(self) -> &'static GlaciteCategoryFields {
        static FIELDS: LazyLock<Vec<GlaciteCategoryFields>> = LazyLock::new(|| vec![
            GlaciteCategoryFields::new("Aquamarine".to_string(), 0x334CB1, vec![BlockPos::new(20, 136, 370), BlockPos::new(-14, 132, 386), BlockPos::new(6, 137, 411), BlockPos::new(50, 117, 302)]),
            GlaciteCategoryFields::new("Onyx".to_string(), 0x191919, vec![BlockPos::new(4, 127, 307), BlockPos::new(-3, 139, 434), BlockPos::new(77, 118, 411), BlockPos::new(-68, 130, 404)]),
            GlaciteCategoryFields::new("Peridot".to_string(), 0x667F33, vec![BlockPos::new(66, 144, 284), BlockPos::new(94, 154, 284), BlockPos::new(-62, 147, 303), BlockPos::new(-77, 119, 283), BlockPos::new(87, 122, 394), BlockPos::new(-73, 122, 456)]),
            GlaciteCategoryFields::new("Citrine".to_string(), 0x664C33, vec![BlockPos::new(-86, 143, 261), BlockPos::new(74, 150, 327), BlockPos::new(63, 137, 343), BlockPos::new(38, 119, 386), BlockPos::new(55, 150, 400), BlockPos::new(-45, 127, 415), BlockPos::new(-60, 144, 424), BlockPos::new(-54, 132, 410)]),
            GlaciteCategoryFields::new("Base Camp".to_string(), 0x983333, vec![BlockPos::new(-7, 126, 229)]),
        ]);
        &FIELDS[self as usize]
    }

    /// java: public BlockPos[] getLocations()
    pub fn get_locations(&self) -> Vec<BlockPos> {
        return self.fields().location.clone();
    }

    /// java: @Override public String toString()
    pub fn to_string(&self) -> String {
        return self.fields().name.clone();
    }

    /// java: @Override public String getName()
    pub fn get_name(&self) -> String {
        return self.fields().name.clone();
    }

    /// java: @Override public int getColor()
    pub fn get_color(&self) -> i32 {
        return self.fields().color;
    }

}

impl std::fmt::Display for GlaciteCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", GlaciteCategory::to_string(self))
    }
}

/// the constructor data of each GlaciteCategory constant
pub struct GlaciteCategoryFields {
    pub name: String,
    pub color: i32,
    pub location: Vec<BlockPos>,
}

impl GlaciteCategoryFields {
    fn new(mut name: String, mut color: i32, mut location: Vec<BlockPos>) -> Self {
        let mut this = Self {
            name,
            color,
            location,
        };
        this
    }

}

impl Category for GlaciteCategory {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn get_name(&self) -> fn(&compat::Shared<dyn Category>) -> String {
        GlaciteCategory::get_name_dyn
    }

    fn get_color(&self) -> fn(&compat::Shared<dyn Category>) -> i32 {
        GlaciteCategory::get_color_dyn
    }

}

impl GlaciteCategory {
    /// GlaciteCategory.getName as the dispatch of Category.getName
    pub fn get_name_dyn(__this: &compat::Shared<dyn Category>) -> String {
        compat::read_as::<GlaciteCategory, _, _>(__this, |__e| __e.get_name())
    }

    /// GlaciteCategory.getColor as the dispatch of Category.getColor
    pub fn get_color_dyn(__this: &compat::Shared<dyn Category>) -> i32 {
        compat::read_as::<GlaciteCategory, _, _>(__this, |__e| __e.get_color())
    }

}

/// java: public enum CrystalHollowsLocationsCategory implements Category, StringRepresentable
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CrystalHollowsLocationsCategory {
    Unknown,
    JungleTemple,
    MinesOfDivan,
    GoblinQueensDen,
    LostPrecursorCity,
    KhazadDum,
    FairyGrotto,
    DragonsLair,
    Corleone,
    KingYolkar,
    Odawa,
    KeyGuardian,
    Xalx,
}

impl CrystalHollowsLocationsCategory {
    pub const VALUES: [CrystalHollowsLocationsCategory; 13] = [CrystalHollowsLocationsCategory::Unknown, CrystalHollowsLocationsCategory::JungleTemple, CrystalHollowsLocationsCategory::MinesOfDivan, CrystalHollowsLocationsCategory::GoblinQueensDen, CrystalHollowsLocationsCategory::LostPrecursorCity, CrystalHollowsLocationsCategory::KhazadDum, CrystalHollowsLocationsCategory::FairyGrotto, CrystalHollowsLocationsCategory::DragonsLair, CrystalHollowsLocationsCategory::Corleone, CrystalHollowsLocationsCategory::KingYolkar, CrystalHollowsLocationsCategory::Odawa, CrystalHollowsLocationsCategory::KeyGuardian, CrystalHollowsLocationsCategory::Xalx];

    /// java: values()
    pub fn values() -> Vec<CrystalHollowsLocationsCategory> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            CrystalHollowsLocationsCategory::Unknown => "UNKNOWN",
            CrystalHollowsLocationsCategory::JungleTemple => "JUNGLE_TEMPLE",
            CrystalHollowsLocationsCategory::MinesOfDivan => "MINES_OF_DIVAN",
            CrystalHollowsLocationsCategory::GoblinQueensDen => "GOBLIN_QUEENS_DEN",
            CrystalHollowsLocationsCategory::LostPrecursorCity => "LOST_PRECURSOR_CITY",
            CrystalHollowsLocationsCategory::KhazadDum => "KHAZAD_DUM",
            CrystalHollowsLocationsCategory::FairyGrotto => "FAIRY_GROTTO",
            CrystalHollowsLocationsCategory::DragonsLair => "DRAGONS_LAIR",
            CrystalHollowsLocationsCategory::Corleone => "CORLEONE",
            CrystalHollowsLocationsCategory::KingYolkar => "KING_YOLKAR",
            CrystalHollowsLocationsCategory::Odawa => "ODAWA",
            CrystalHollowsLocationsCategory::KeyGuardian => "KEY_GUARDIAN",
            CrystalHollowsLocationsCategory::Xalx => "XALX",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> CrystalHollowsLocationsCategory {
        match name {
            "UNKNOWN" => CrystalHollowsLocationsCategory::Unknown,
            "JUNGLE_TEMPLE" => CrystalHollowsLocationsCategory::JungleTemple,
            "MINES_OF_DIVAN" => CrystalHollowsLocationsCategory::MinesOfDivan,
            "GOBLIN_QUEENS_DEN" => CrystalHollowsLocationsCategory::GoblinQueensDen,
            "LOST_PRECURSOR_CITY" => CrystalHollowsLocationsCategory::LostPrecursorCity,
            "KHAZAD_DUM" => CrystalHollowsLocationsCategory::KhazadDum,
            "FAIRY_GROTTO" => CrystalHollowsLocationsCategory::FairyGrotto,
            "DRAGONS_LAIR" => CrystalHollowsLocationsCategory::DragonsLair,
            "CORLEONE" => CrystalHollowsLocationsCategory::Corleone,
            "KING_YOLKAR" => CrystalHollowsLocationsCategory::KingYolkar,
            "ODAWA" => CrystalHollowsLocationsCategory::Odawa,
            "KEY_GUARDIAN" => CrystalHollowsLocationsCategory::KeyGuardian,
            "XALX" => CrystalHollowsLocationsCategory::Xalx,
            _ => panic!("IllegalArgumentException: No enum constant CrystalHollowsLocationsCategory.{name}"),
        }
    }

    /// the constant's constructor data (Java keeps it in the constant's fields)
    pub fn fields(self) -> &'static CrystalHollowsLocationsCategoryFields {
        static FIELDS: LazyLock<Vec<CrystalHollowsLocationsCategoryFields>> = LazyLock::new(|| vec![
            CrystalHollowsLocationsCategoryFields::new("Unknown".to_string(), mcrust::java::awt::color::white(), None),
            CrystalHollowsLocationsCategoryFields::new("Jungle Temple".to_string(), mcrust::java::awt::color::Color::from_rgb(DyeColor::Purple.get_text_color()), Some("[NPC] Kalhuiki Door Guardian:".to_string())),
            CrystalHollowsLocationsCategoryFields::new("Mines of Divan".to_string(), mcrust::java::awt::color::green(), Some(format!("{}Jade Crystal", CRYSTALS_SPACER))),
            CrystalHollowsLocationsCategoryFields::new("Goblin Queen's Den".to_string(), mcrust::java::awt::color::Color::from_rgb(DyeColor::Orange.get_text_color()), Some(format!("{}Amber Crystal", CRYSTALS_SPACER))),
            CrystalHollowsLocationsCategoryFields::new("Lost Precursor City".to_string(), mcrust::java::awt::color::cyan(), Some(format!("{}Sapphire Crystal", CRYSTALS_SPACER))),
            CrystalHollowsLocationsCategoryFields::new("Khazad-dûm".to_string(), mcrust::java::awt::color::yellow(), Some(format!("{}Topaz Crystal", CRYSTALS_SPACER))),
            CrystalHollowsLocationsCategoryFields::new("Fairy Grotto".to_string(), mcrust::java::awt::color::pink(), None),
            CrystalHollowsLocationsCategoryFields::new("Dragon's Lair".to_string(), mcrust::java::awt::color::Color::from_rgb(mcrust::net::minecraft::network::chat::text_color::TextColor::GOLD.get_value()), Some("[NPC] Golden Dragon:".to_string())),
            CrystalHollowsLocationsCategoryFields::new("Corleone".to_string(), mcrust::java::awt::color::white(), None),
            CrystalHollowsLocationsCategoryFields::new("King Yolkar".to_string(), mcrust::java::awt::color::red(), Some("[NPC] King Yolkar:".to_string())),
            CrystalHollowsLocationsCategoryFields::new("Odawa".to_string(), mcrust::java::awt::color::magenta(), Some("[NPC] Odawa:".to_string())),
            CrystalHollowsLocationsCategoryFields::new("Key Guardian".to_string(), mcrust::java::awt::color::light_gray(), None),
            CrystalHollowsLocationsCategoryFields::new("Xalx".to_string(), mcrust::java::awt::color::green(), Some("[NPC] Xalx:".to_string())),
        ]);
        &FIELDS[self as usize]
    }

    /// java: @Override public String getName()
    pub fn get_name(&self) -> String {
        return self.fields().name.clone();
    }

    /// java: @Override public int getColor()
    pub fn get_color(&self) -> i32 {
        return self.fields().color.get_rgb();
    }

    /// java: public @Nullable String getLinkedMessage()
    pub fn get_linked_message(&self) -> Option<String> {
        return self.fields().linked_message.clone();
    }

    /// java: @Override public String getSerializedName()
    pub fn get_serialized_name(&self) -> String {
        return self.name().to_string();
    }

}

impl std::fmt::Display for CrystalHollowsLocationsCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

/// the constructor data of each CrystalHollowsLocationsCategory constant
pub struct CrystalHollowsLocationsCategoryFields {
    pub color: Color,
    pub name: String,
    pub linked_message: Option<String>,
}

impl CrystalHollowsLocationsCategoryFields {
    fn new(mut name: String, mut color: Color, mut linked_message: Option<String>) -> Self {
        let mut this = Self {
            color,
            name,
            linked_message,
        };
        this
    }

}

impl Category for CrystalHollowsLocationsCategory {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn get_name(&self) -> fn(&compat::Shared<dyn Category>) -> String {
        CrystalHollowsLocationsCategory::get_name_dyn
    }

    fn get_color(&self) -> fn(&compat::Shared<dyn Category>) -> i32 {
        CrystalHollowsLocationsCategory::get_color_dyn
    }

}

impl CrystalHollowsLocationsCategory {
    /// CrystalHollowsLocationsCategory.getName as the dispatch of Category.getName
    pub fn get_name_dyn(__this: &compat::Shared<dyn Category>) -> String {
        compat::read_as::<CrystalHollowsLocationsCategory, _, _>(__this, |__e| __e.get_name())
    }

    /// CrystalHollowsLocationsCategory.getColor as the dispatch of Category.getColor
    pub fn get_color_dyn(__this: &compat::Shared<dyn Category>) -> i32 {
        compat::read_as::<CrystalHollowsLocationsCategory, _, _>(__this, |__e| __e.get_color())
    }

}
