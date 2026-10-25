//! Ported from de.hysky.skyblocker.skyblock.tabhud.screenbuilder.pipeline.PositionRule by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::com::mojang::serialization::codec;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::com::mojang::serialization::codecs::record_codec_builder;

/// java: public static final PositionRule DEFAULT = new PositionRule(Optional.empty(), Point.DEFAULT, Point.DEFAULT, 5, 5);
pub static DEFAULT: compat::MainThread<PositionRule> = compat::MainThread::new(|| PositionRule::new_canonical(None, POINT_DEFAULT.clone(), POINT_DEFAULT.clone(), 5, 5));
/// java: public static final Codec<PositionRule> CODEC = RecordCodecBuilder.create(instance -> instance.group( Codec.STRING.optionalFieldOf("parent").forGetter(PositionRule::parent), Point.CODEC.fieldOf("paren
pub static CODEC: compat::MainThread<CodecRef<PositionRule>> = compat::MainThread::new(|| record_codec_builder::create(record_codec_builder::apply5(record_codec_builder::of(|o: &PositionRule| o.parent(), codec::optional_field_of_opt(mcrust::com::mojang::serialization::codec::string_codec(), "parent")), record_codec_builder::of(|o: &PositionRule| o.parent_point(), codec::field_of(POINT_CODEC.get_mut().clone(), "parent_anchor")), record_codec_builder::of(|o: &PositionRule| o.this_point(), codec::field_of(POINT_CODEC.get_mut().clone(), "this_anchor")), record_codec_builder::of(|o: &PositionRule| o.relative_x(), codec::field_of(mcrust::com::mojang::serialization::codec::int_codec(), "relative_x")), record_codec_builder::of(|o: &PositionRule| o.relative_y(), codec::field_of(mcrust::com::mojang::serialization::codec::int_codec(), "relative_y")), move |__a0, __a1, __a2, __a3, __a4| PositionRule::new_canonical(__a0, __a1, __a2, __a3, __a4))));
/// java: public static final Codec<HorizontalPoint> CODEC = Codec.STRING.xmap(HorizontalPoint::valueOf, HorizontalPoint::name);
pub static HORIZONTAL_POINT_CODEC: compat::MainThread<CodecRef<HorizontalPoint>> = compat::MainThread::new(|| codec::xmap(codec::string_codec(), |s: String| HorizontalPoint::value_of(&s), |p: &HorizontalPoint| p.name().to_string()));
/// java: public static final Codec<VerticalPoint> CODEC = Codec.STRING.xmap(VerticalPoint::valueOf, VerticalPoint::name);
pub static VERTICAL_POINT_CODEC: compat::MainThread<CodecRef<VerticalPoint>> = compat::MainThread::new(|| codec::xmap(codec::string_codec(), |s: String| VerticalPoint::value_of(&s), |p: &VerticalPoint| p.name().to_string()));
/// java: public static final Point DEFAULT = new Point(VerticalPoint.TOP, HorizontalPoint.LEFT);
pub static POINT_DEFAULT: LazyLock<Point> = LazyLock::new(|| Point::new(VerticalPoint::Top, HorizontalPoint::Left));
/// java: public static final Codec<Point> CODEC = RecordCodecBuilder.create(instance -> instance.group( VerticalPoint.CODEC.fieldOf("v").forGetter(Point::verticalPoint), HorizontalPoint.CODEC.fieldOf("h").forG
pub static POINT_CODEC: compat::MainThread<CodecRef<Point>> = compat::MainThread::new(|| record_codec_builder::create(record_codec_builder::apply2(record_codec_builder::of(|o: &Point| o.vertical_point(), codec::field_of(VERTICAL_POINT_CODEC.get_mut().clone(), "v")), record_codec_builder::of(|o: &Point| o.horizontal_point(), codec::field_of(HORIZONTAL_POINT_CODEC.get_mut().clone(), "h")), move |__a0, __a1| Point::new(__a0.clone(), __a1.clone()))));

/// java: public record PositionRule(Optional<String> parent, Point parentPoint, Point thisPoint, int relativeX, int relativeY)
#[derive(Clone, Debug, PartialEq)]
pub struct PositionRule {
    /// java: Optional<String> parent
    pub parent: Option<String>,
    /// java: Point parentPoint
    pub parent_point: Point,
    /// java: Point thisPoint
    pub this_point: Point,
    /// java: int relativeX
    pub relative_x: i32,
    /// java: int relativeY
    pub relative_y: i32,
}

impl PositionRule {
    /// java: public PositionRule(@Nullable String parent, Point parentPoint, Point thisPoint, int relativeX, int relativeY)
    pub fn new(parent: Option<String>, parent_point: Point, this_point: Point, relative_x: i32, relative_y: i32) -> Self {
        // java: this(parent == null || parent.equals("screen") ? Optional.empty() : Optional.of(parent), parentPoint, thisPoint, relativeX, relativeY);
        Self::new_canonical(parent.filter(|p| p != "screen"), parent_point, this_point, relative_x, relative_y)
    }

    /// java: the canonical record constructor `PositionRule(Optional<String> parent, Point parentPoint, Point thisPoint, int relativeX, int relativeY)`
    pub fn new_canonical(parent: Option<String>, parent_point: Point, this_point: Point, relative_x: i32, relative_y: i32) -> Self {
        Self { parent, parent_point, this_point, relative_x, relative_y }
    }

    /// java: the record accessor `parent()`
    pub fn parent(&self) -> Option<String> {
        self.parent.clone()
    }

    /// java: the record accessor `parentPoint()`
    pub fn parent_point(&self) -> Point {
        self.parent_point.clone()
    }

    /// java: the record accessor `thisPoint()`
    pub fn this_point(&self) -> Point {
        self.this_point.clone()
    }

    /// java: the record accessor `relativeX()`
    pub fn relative_x(&self) -> i32 {
        self.relative_x
    }

    /// java: the record accessor `relativeY()`
    pub fn relative_y(&self) -> i32 {
        self.relative_y
    }

    /// java: public PositionRule withParent(@Nullable String parent)
    pub fn with_parent(&self, mut parent: Option<String>) -> PositionRule {
        return PositionRule::new(parent.clone(), self.parent_point.clone(), self.this_point.clone(), self.relative_x, self.relative_y);
    }

}

/// java: public enum HorizontalPoint
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum HorizontalPoint {
    Left,
    Center,
    Right,
}

impl HorizontalPoint {
    pub const VALUES: [HorizontalPoint; 3] = [HorizontalPoint::Left, HorizontalPoint::Center, HorizontalPoint::Right];

    /// java: values()
    pub fn values() -> Vec<HorizontalPoint> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            HorizontalPoint::Left => "LEFT",
            HorizontalPoint::Center => "CENTER",
            HorizontalPoint::Right => "RIGHT",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> HorizontalPoint {
        match name {
            "LEFT" => HorizontalPoint::Left,
            "CENTER" => HorizontalPoint::Center,
            "RIGHT" => HorizontalPoint::Right,
            _ => panic!("IllegalArgumentException: No enum constant HorizontalPoint.{name}"),
        }
    }

    /// java: public float getPercentage()
    pub fn get_percentage(&self) -> f32 {
        return match *self {
            HorizontalPoint::Left => 0.0f32,
            HorizontalPoint::Center => 0.5f32,
            HorizontalPoint::Right => 1.0f32,
            _ => unreachable!("switch expression: no case matched"),
        };
    }

}

impl std::fmt::Display for HorizontalPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

/// java: public enum VerticalPoint
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum VerticalPoint {
    Top,
    Center,
    Bottom,
}

impl VerticalPoint {
    pub const VALUES: [VerticalPoint; 3] = [VerticalPoint::Top, VerticalPoint::Center, VerticalPoint::Bottom];

    /// java: values()
    pub fn values() -> Vec<VerticalPoint> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            VerticalPoint::Top => "TOP",
            VerticalPoint::Center => "CENTER",
            VerticalPoint::Bottom => "BOTTOM",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> VerticalPoint {
        match name {
            "TOP" => VerticalPoint::Top,
            "CENTER" => VerticalPoint::Center,
            "BOTTOM" => VerticalPoint::Bottom,
            _ => panic!("IllegalArgumentException: No enum constant VerticalPoint.{name}"),
        }
    }

    /// java: public float getPercentage()
    pub fn get_percentage(&self) -> f32 {
        return match *self {
            VerticalPoint::Top => 0.0f32,
            VerticalPoint::Center => 0.5f32,
            VerticalPoint::Bottom => 1.0f32,
            _ => unreachable!("switch expression: no case matched"),
        };
    }

}

impl std::fmt::Display for VerticalPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

/// java: public record Point(VerticalPoint verticalPoint, HorizontalPoint horizontalPoint)
#[derive(Clone, Debug, PartialEq)]
pub struct Point {
    /// java: VerticalPoint verticalPoint
    pub vertical_point: VerticalPoint,
    /// java: HorizontalPoint horizontalPoint
    pub horizontal_point: HorizontalPoint,
}

impl Point {
    /// java: the canonical record constructor
    pub fn new(mut vertical_point: VerticalPoint, mut horizontal_point: HorizontalPoint) -> Self {
        Self { vertical_point, horizontal_point }
    }

    /// java: the record accessor `verticalPoint()`
    pub fn vertical_point(&self) -> VerticalPoint {
        self.vertical_point
    }

    /// java: the record accessor `horizontalPoint()`
    pub fn horizontal_point(&self) -> HorizontalPoint {
        self.horizontal_point
    }

}
