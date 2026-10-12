//! Ported from de.hysky.skyblocker.skyblock.fancybars.BarPositioner by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::fancybars::fancy_status_bars;
use crate::de::hysky::skyblocker::skyblock::fancybars::fancy_status_bars::FancyStatusBars;
use crate::de::hysky::skyblocker::skyblock::fancybars::status_bar::StatusBar;
use mcrust::net::minecraft::client::gui::navigation::screen_position::ScreenPosition;
use mcrust::net::minecraft::client::gui::navigation::screen_rectangle::ScreenRectangle;

// TODO(fabric2seer): call to `values(..)` (not a method of this class)
//   java: private static final List<BarAnchor> cached = List.of(values());
/// java: private static final List<BarAnchor> cached = List.of(values());
pub static BAR_ANCHOR_CACHED: LazyLock<Vec<BarAnchor>> = LazyLock::new(|| todo!("fabric2seer: the initialiser of `cached` - see the TODO above"));
/// java: public static final BarLocation NULL = new BarLocation(null, -1, -1);
pub static BAR_LOCATION_NULL: LazyLock<BarLocation> = LazyLock::new(|| BarLocation::new(None, -1, -1));

/// java: public class BarPositioner
#[derive(Clone)]
pub struct BarPositioner {
    /// java: private final Map<BarAnchor, LinkedList<LinkedList<StatusBar>>> map = new EnumMap<>(BarAnchor.class);
    pub map: std::collections::HashMap<BarAnchor, Vec<Vec<compat::Shared<dyn StatusBar>>>>,
}

impl BarPositioner {
    /// java: public BarPositioner()
    pub fn new() -> Self {
        let mut this = Self {
            map: std::any::TypeId::of::<BarAnchor>(),
        };
        for value in BarAnchor::values() {
            this.map.insert(value, Vec::new());
        }
        this
    }

    /// java: public int getRowCount(BarAnchor barAnchor)
    pub fn get_row_count(&self, mut bar_anchor: BarAnchor) -> i32 {
        return (self.map.get(&bar_anchor).cloned().unwrap().len() as i32);
    }

    /// java: public void addRow(BarAnchor barAnchor)
    pub fn add_row(&self, mut bar_anchor: BarAnchor) {
        self.map.get(&bar_anchor).cloned().unwrap().push(Vec::new());
    }

    /// java: public void addRow(BarAnchor barAnchor, int row)
    pub fn add_row_2(&self, mut bar_anchor: BarAnchor, mut row: i32) {
        let mut rows: Vec<Vec<compat::Shared<dyn StatusBar>>> = self.map.get(&bar_anchor).cloned().unwrap();
        for i in row..(rows.len() as i32) {
            for bar in (rows[(i) as usize]).iter() {
                { let __v = compat::read(bar, |__t| __t.status_bar_base().grid_y) + 1; compat::write(bar, |__t| __t.status_bar_base_mut().grid_y = __v); }
            }
        }
        rows.insert((row) as usize, Vec::new());
    }

    /// java: public void addBar(BarAnchor barAnchor, int row, StatusBar bar)
    pub fn add_bar(&self, mut bar_anchor: BarAnchor, mut row: i32, mut bar: compat::Shared<dyn StatusBar>) {
        let status_bars = &mut (self.map.get(&bar_anchor).cloned().unwrap()[(row) as usize]);
        status_bars.push(bar.clone());
        { let __v = row; compat::write(&bar, |__t| __t.status_bar_base_mut().grid_y = __v); }
        // TODO(fabric2seer): List.lastIndexOf(..)
        //   java: bar.gridX = statusBars.lastIndexOf(bar);
        { let __v = Some(bar_anchor); compat::write(&bar, |__t| __t.status_bar_base_mut().anchor = __v); }
    }

    /// java: public void addBar(BarAnchor barAnchor, int row, int x, StatusBar bar)
    pub fn add_bar_4(&self, mut bar_anchor: BarAnchor, mut row: i32, mut x: i32, mut bar: compat::Shared<dyn StatusBar>) {
        let status_bars = &mut (self.map.get(&bar_anchor).cloned().unwrap()[(row) as usize]);
        for i in x..(status_bars.len() as i32) {
            { let __v = compat::read(&status_bars[(i) as usize], |__t| __t.status_bar_base().grid_x) + 1; compat::write(&status_bars[(i) as usize], |__t| __t.status_bar_base_mut().grid_x = __v); }
        }
        status_bars.insert((x) as usize, bar.clone());
        { let __v = row; compat::write(&bar, |__t| __t.status_bar_base_mut().grid_y = __v); }
        { let __v = status_bars.iter().position(|e| *e == bar.clone()).map_or(-1, |i| i as i32); compat::write(&bar, |__t| __t.status_bar_base_mut().grid_x = __v); }
        { let __v = Some(bar_anchor); compat::write(&bar, |__t| __t.status_bar_base_mut().anchor = __v); }
    }

    /// java: public void removeBar(BarAnchor barAnchor, int row, int x)
    pub fn remove_bar(&self, mut bar_anchor: BarAnchor, mut row: i32, mut x: i32) {
        let status_bars = &mut (self.map.get(&bar_anchor).cloned().unwrap()[(row) as usize]);
        let mut remove: compat::Shared<dyn StatusBar> = status_bars.remove((x) as usize);
        { let __v = None; compat::write(&remove, |__t| __t.status_bar_base_mut().anchor = __v); }
        for i in x..(status_bars.len() as i32) {
            { let __v = compat::read(&status_bars[(i) as usize], |__t| __t.status_bar_base().grid_x) - 1; compat::write(&status_bars[(i) as usize], |__t| __t.status_bar_base_mut().grid_x = __v); }
        }
        if status_bars.is_empty() {
            self.remove_row(bar_anchor, row);
        }
    }

    /// java: public void removeBar(BarAnchor barAnchor, int row, StatusBar bar)
    pub fn remove_bar_bar_anchor_int_status_bar(&self, mut bar_anchor: BarAnchor, mut row: i32, bar: &compat::Shared<dyn StatusBar>) {
        let bar_row = &mut (self.map.get(&bar_anchor).cloned().unwrap()[(row) as usize]);
        let mut x: i32 = bar_row.iter().position(|e| *e == bar.clone()).map_or(-1, |i| i as i32);
        if x < 0 {
            return;
        }
        compat::remove_item(&mut *bar_row, &bar.clone());
        { let __v = None; compat::write(bar, |__t| __t.status_bar_base_mut().anchor = __v); }
        for i in x..(bar_row.len() as i32) {
            { let __v = compat::read(&bar_row[(i) as usize], |__t| __t.status_bar_base().grid_x) - 1; compat::write(&bar_row[(i) as usize], |__t| __t.status_bar_base_mut().grid_x = __v); }
        }
        if bar_row.is_empty() {
            self.remove_row(bar_anchor, row);
        }
    }

    /// java: public void removeRow(BarAnchor barAnchor, int row)
    pub fn remove_row(&self, mut bar_anchor: BarAnchor, mut row: i32) {
        let bar_row = &self.map.get(&bar_anchor).cloned().unwrap()[(row) as usize];
        if !bar_row.is_empty() {
            panic!("IllegalStateException: {}", format!("Can't remove a non-empty row ({},{})", compat::jstring(&bar_anchor), row).to_string());
        }
        self.map.get(&bar_anchor).cloned().unwrap().remove((row) as usize);
        for i in row..(self.map.get(&bar_anchor).cloned().unwrap().len() as i32) {
            for status_bar in (self.map.get(&bar_anchor).cloned().unwrap()[(i) as usize]).iter() {
                { let __v = compat::read(status_bar, |__t| __t.status_bar_base().grid_y) - 1; compat::write(status_bar, |__t| __t.status_bar_base_mut().grid_y = __v); }
            }
        }
    }

    /// java: public LinkedList<StatusBar> getRow(BarAnchor barAnchor, int row)
    pub fn get_row(&self, mut bar_anchor: BarAnchor, mut row: i32) -> Vec<compat::Shared<dyn StatusBar>> {
        return self.map.get(&bar_anchor).cloned().unwrap()[(row) as usize].clone();
    }

    /// java: public StatusBar getBar(BarAnchor barAnchor, int row, int x)
    pub fn get_bar(&self, mut bar_anchor: BarAnchor, mut row: i32, mut x: i32) -> compat::Shared<dyn StatusBar> {
        return (self.map.get(&bar_anchor).cloned().unwrap()[(row) as usize])[(x) as usize].clone();
    }

    /// java: public boolean hasNeighbor(BarAnchor barAnchor, int row, int x, boolean right)
    pub fn has_neighbor(&self, mut bar_anchor: BarAnchor, mut row: i32, mut x: i32, mut right: bool) -> bool {
        let status_bars = &self.map.get(&bar_anchor).cloned().unwrap()[(row) as usize];
        if bar_anchor.is_right() {
            return (right && x < (status_bars.len() as i32).wrapping_sub(1) || !right && x > 0);
        } else {
            return (right && x > 0 || !right && x < (status_bars.len() as i32).wrapping_sub(1));
        }
    }

    /// java: public void clear()
    pub fn clear(&mut self) {
        // TODO(fabric2seer): Map.replaceAll(..)
        //   java: map.replaceAll((_, _) -> new LinkedList<>());
    }

}

/// java: public enum BarAnchor
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum BarAnchor {
    HotbarLeft,
    HotbarRight,
    HotbarTop,
    ScreenTopLeft,
    ScreenTopRight,
    ScreenBottomLeft,
    ScreenBottomRight,
    ScreenCenterLeft,
    ScreenCenterRight,
}

impl BarAnchor {
    pub const VALUES: [BarAnchor; 9] = [BarAnchor::HotbarLeft, BarAnchor::HotbarRight, BarAnchor::HotbarTop, BarAnchor::ScreenTopLeft, BarAnchor::ScreenTopRight, BarAnchor::ScreenBottomLeft, BarAnchor::ScreenBottomRight, BarAnchor::ScreenCenterLeft, BarAnchor::ScreenCenterRight];

    /// java: values()
    pub fn values() -> Vec<BarAnchor> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            BarAnchor::HotbarLeft => "HOTBAR_LEFT",
            BarAnchor::HotbarRight => "HOTBAR_RIGHT",
            BarAnchor::HotbarTop => "HOTBAR_TOP",
            BarAnchor::ScreenTopLeft => "SCREEN_TOP_LEFT",
            BarAnchor::ScreenTopRight => "SCREEN_TOP_RIGHT",
            BarAnchor::ScreenBottomLeft => "SCREEN_BOTTOM_LEFT",
            BarAnchor::ScreenBottomRight => "SCREEN_BOTTOM_RIGHT",
            BarAnchor::ScreenCenterLeft => "SCREEN_CENTER_LEFT",
            BarAnchor::ScreenCenterRight => "SCREEN_CENTER_RIGHT",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> BarAnchor {
        match name {
            "HOTBAR_LEFT" => BarAnchor::HotbarLeft,
            "HOTBAR_RIGHT" => BarAnchor::HotbarRight,
            "HOTBAR_TOP" => BarAnchor::HotbarTop,
            "SCREEN_TOP_LEFT" => BarAnchor::ScreenTopLeft,
            "SCREEN_TOP_RIGHT" => BarAnchor::ScreenTopRight,
            "SCREEN_BOTTOM_LEFT" => BarAnchor::ScreenBottomLeft,
            "SCREEN_BOTTOM_RIGHT" => BarAnchor::ScreenBottomRight,
            "SCREEN_CENTER_LEFT" => BarAnchor::ScreenCenterLeft,
            "SCREEN_CENTER_RIGHT" => BarAnchor::ScreenCenterRight,
            _ => panic!("IllegalArgumentException: No enum constant BarAnchor.{name}"),
        }
    }

    /// the constant's constructor data (Java keeps it in the constant's fields)
    pub fn fields(self) -> &'static BarAnchorFields {
        static FIELDS: LazyLock<Vec<BarAnchorFields>> = LazyLock::new(|| vec![
            BarAnchorFields::new_4(true, false, Arc::new(move |scaled_width, scaled_height| ScreenPosition::new(((scaled_width.wrapping_div(2)).wrapping_sub(91)).wrapping_sub(2), scaled_height.wrapping_sub(5))), SizeRule::free_size(25, 2, 6)),
            BarAnchorFields::new_4(true, true, Arc::new(move |scaled_width, scaled_height| ScreenPosition::new(((scaled_width.wrapping_div(2)).wrapping_add(91)).wrapping_add(2), scaled_height.wrapping_sub(5))), SizeRule::free_size(25, 2, 6)),
            BarAnchorFields::new(true, true, Arc::new(move |scaled_width, scaled_height| ScreenPosition::new((scaled_width.wrapping_div(2)).wrapping_sub(91), scaled_height.wrapping_sub(if FancyStatusBars::is_experience_fancy_bar_enabled(fancy_status_bars::INSTANCE.get_mut()) { 23 } else { 35 }))), SizeRule::target_size(12, 182, 2), Arc::new(move |anchor_position| ScreenRectangle::new_at(anchor_position.x(), anchor_position.y().wrapping_sub(20), 182, 20))),
            BarAnchorFields::new_4(false, true, Arc::new(move |_, _| ScreenPosition::new(5, 5)), SizeRule::free_size(25, 2, 6)),
            BarAnchorFields::new_4(false, false, Arc::new(move |scaled_width, _| ScreenPosition::new(scaled_width.wrapping_sub(5), 5)), SizeRule::free_size(25, 2, 6)),
            BarAnchorFields::new_4(true, true, Arc::new(move |_, scaled_height| ScreenPosition::new(5, scaled_height.wrapping_sub(5))), SizeRule::free_size(25, 2, 6)),
            BarAnchorFields::new_4(true, false, Arc::new(move |scaled_width, scaled_height| ScreenPosition::new(scaled_width.wrapping_sub(5), scaled_height.wrapping_sub(5))), SizeRule::free_size(25, 2, 6)),
            BarAnchorFields::new_4(false, false, Arc::new(move |scaled_width, scaled_height| ScreenPosition::new((scaled_width.wrapping_div(2)).wrapping_sub(8), (scaled_height.wrapping_div(2)).wrapping_sub(4))), SizeRule::free_size(15, 3, 8)),
            BarAnchorFields::new_4(false, true, Arc::new(move |scaled_width, scaled_height| ScreenPosition::new((scaled_width.wrapping_div(2)).wrapping_add(8), (scaled_height.wrapping_div(2)).wrapping_sub(4))), SizeRule::free_size(15, 3, 8)),
        ]);
        &FIELDS[self as usize]
    }

    /// java: public ScreenPosition getAnchorPosition(int scaledWidth, int scaledHeight)
    pub fn get_anchor_position(&self, mut scaled_width: i32, mut scaled_height: i32) -> ScreenPosition {
        return (self.fields().position_provider)(scaled_width, scaled_height);
    }

    /// java: public ScreenRectangle getAnchorHitbox(ScreenPosition anchorPosition)
    pub fn get_anchor_hitbox(&self, anchor_position: &ScreenPosition) -> ScreenRectangle {
        return (self.fields().hitbox_provider)(anchor_position.clone());
    }

    /// java: public boolean isUp()
    pub fn is_up(&self) -> bool {
        return self.fields().up;
    }

    /// java: public boolean isRight()
    pub fn is_right(&self) -> bool {
        return self.fields().right;
    }

    /// java: public SizeRule getSizeRule()
    pub fn get_size_rule(&self) -> SizeRule {
        return self.fields().size_rule.clone();
    }

    /// java: public static List<BarAnchor> allAnchors()
    pub fn all_anchors() -> Vec<BarAnchor> {
        return BAR_ANCHOR_CACHED.clone();
    }

}

impl std::fmt::Display for BarAnchor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

/// the constructor data of each BarAnchor constant
pub struct BarAnchorFields {
    pub position_provider: Option<AnchorPositionProvider>,
    pub hitbox_provider: Option<AnchorHitboxProvider>,
    pub up: bool,
    pub right: bool,
    pub size_rule: Option<SizeRule>,
}

impl BarAnchorFields {
    fn new(mut up: bool, mut right: bool, position_provider: AnchorPositionProvider, mut size_rule: SizeRule, hitbox_provider: AnchorHitboxProvider) -> Self {
        let mut this = Self {
            position_provider: None,
            hitbox_provider: None,
            up: false,
            right: false,
            size_rule: None,
        };
        this.position_provider = position_provider.clone();
        this.up = up;
        this.right = right;
        this.hitbox_provider = hitbox_provider.clone();
        this.size_rule = size_rule.clone();
        this
    }

    fn new_4(mut up: bool, mut right: bool, position_provider: AnchorPositionProvider, mut size_rule: SizeRule) -> Self {
        let mut this = Self {
            position_provider: None,
            hitbox_provider: None,
            up: false,
            right: false,
            size_rule: None,
        };
        // TODO(fabric2seer): this statement
        //   java: this(up, right, positionProvider, sizeRule,
        //   java: 					anchorPosition -> new ScreenRectangle(anchorPosition.x() - (right ? 0 : 20), anchorPosition.y() - (up ? 20 : 0), 20, 20));
        this
    }

}

/// java: public record SizeRule(boolean isTargetSize, int targetSize, int totalWidth, int widthPerSize, int minSize, int maxSize)
#[derive(Clone, Debug, PartialEq)]
pub struct SizeRule {
    /// java: boolean isTargetSize
    pub is_target_size: bool,
    /// java: int targetSize
    pub target_size: i32,
    /// java: int totalWidth
    pub total_width: i32,
    /// java: int widthPerSize
    pub width_per_size: i32,
    /// java: int minSize
    pub min_size: i32,
    /// java: int maxSize
    pub max_size: i32,
}

impl SizeRule {
    /// java: the canonical record constructor
    pub fn new(mut is_target_size: bool, mut target_size: i32, mut total_width: i32, mut width_per_size: i32, mut min_size: i32, mut max_size: i32) -> Self {
        Self { is_target_size, target_size, total_width, width_per_size, min_size, max_size }
    }

    /// java: the record accessor `isTargetSize()`
    pub fn is_target_size(&self) -> bool {
        self.is_target_size
    }

    /// java: the record accessor `targetSize()`
    pub fn target_size(&self) -> i32 {
        self.target_size
    }

    /// java: the record accessor `totalWidth()`
    pub fn total_width(&self) -> i32 {
        self.total_width
    }

    /// java: the record accessor `widthPerSize()`
    pub fn width_per_size(&self) -> i32 {
        self.width_per_size
    }

    /// java: the record accessor `minSize()`
    pub fn min_size(&self) -> i32 {
        self.min_size
    }

    /// java: the record accessor `maxSize()`
    pub fn max_size(&self) -> i32 {
        self.max_size
    }

    /// java: public static SizeRule freeSize(int widthPerSize, int minSize, int maxSize)
    pub fn free_size(mut width_per_size: i32, mut min_size: i32, mut max_size: i32) -> SizeRule {
        return SizeRule::new(false, -1, -1, width_per_size, min_size, max_size);
    }

    /// java: public static SizeRule targetSize(int targetSize, int totalWidth, int minSize)
    pub fn target_size(mut target_size: i32, mut total_width: i32, mut min_size: i32) -> SizeRule {
        return SizeRule::new(true, target_size, total_width, -1, min_size, target_size);
    }

}

/// java: public record BarLocation(@Nullable BarAnchor barAnchor, int x, int y)
#[derive(Clone, Debug, PartialEq)]
pub struct BarLocation {
    /// java: BarAnchor barAnchor
    pub bar_anchor: Option<BarAnchor>,
    /// java: int x
    pub x: i32,
    /// java: int y
    pub y: i32,
}

impl BarLocation {
    /// java: the canonical record constructor
    pub fn new(mut bar_anchor: Option<BarAnchor>, mut x: i32, mut y: i32) -> Self {
        Self { bar_anchor, x, y }
    }

    /// java: the record accessor `barAnchor()`
    pub fn bar_anchor(&self) -> Option<BarAnchor> {
        self.bar_anchor
    }

    /// java: the record accessor `x()`
    pub fn x(&self) -> i32 {
        self.x
    }

    /// java: the record accessor `y()`
    pub fn y(&self) -> i32 {
        self.y
    }

    /// java: public static BarLocation of(StatusBar bar)
    pub fn of(bar: &compat::Shared<dyn StatusBar>) -> BarLocation {
        return BarLocation::new(compat::read(bar, |__t| __t.status_bar_base().anchor.clone()), compat::read(bar, |__t| __t.status_bar_base().grid_x), compat::read(bar, |__t| __t.status_bar_base().grid_y));
    }

    /// java: public boolean equals(BarAnchor barAnchor, int x, int y)
    pub fn equals(&self, mut bar_anchor: BarAnchor, mut x: i32, mut y: i32) -> bool {
        return (x == self.x && y == self.y && bar_anchor == self.bar_anchor.clone().unwrap());
    }

}

/// java: @FunctionalInterface interface AnchorPositionProvider
/// a functional interface: `getPosition(int scaledWidth, int scaledHeight)` - a lambda / method reference is this closure type
pub type AnchorPositionProvider = Arc<dyn Fn(i32, i32) -> ScreenPosition>;

/// java: @FunctionalInterface interface AnchorHitboxProvider
/// a functional interface: `getHitbox(ScreenPosition anchorPosition)` - a lambda / method reference is this closure type
pub type AnchorHitboxProvider = Arc<dyn Fn(ScreenPosition) -> ScreenRectangle>;
