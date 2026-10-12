//! Ported from de.hysky.skyblocker.skyblock.fancybars.BarPositioner by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::fancybars::fancy_status_bars;
use crate::de::hysky::skyblocker::skyblock::fancybars::fancy_status_bars::FancyStatusBars;
use crate::de::hysky::skyblocker::skyblock::fancybars::status_bar::StatusBar;
use mcrust::net::minecraft::client::gui::navigation::screen_position::ScreenPosition;
use mcrust::net::minecraft::client::gui::navigation::screen_rectangle::ScreenRectangle;

/// java: private static final List<BarAnchor> cached = List.of(values());
pub static BAR_ANCHOR_CACHED: LazyLock<Vec<BarAnchor>> = LazyLock::new(|| BarAnchor::values());
/// java: public static final BarLocation NULL = new BarLocation(null, -1, -1);
pub static BAR_LOCATION_NULL: LazyLock<BarLocation> = LazyLock::new(|| BarLocation::new(None, -1, -1));

/// java: `list.indexOf(bar)` / `lastIndexOf(bar)` over status bars - `StatusBar` keeps `Object.equals`, so identity.
fn index_of(list: &[compat::Shared<dyn StatusBar>], bar: &compat::Shared<dyn StatusBar>) -> i32 {
    list.iter().position(|e| Arc::ptr_eq(e, bar)).map_or(-1, |i| i as i32)
}

/// java: public class BarPositioner
#[derive(Clone)]
pub struct BarPositioner {
    /// java: private final Map<BarAnchor, LinkedList<LinkedList<StatusBar>>> map = new EnumMap<>(BarAnchor.class);
    /// (an EnumMap iterates in ordinal order: `BarAnchor`'s `Ord` is its declaration order)
    pub map: std::collections::BTreeMap<BarAnchor, Vec<Vec<compat::Shared<dyn StatusBar>>>>,
}

impl BarPositioner {
    /// java: public BarPositioner()
    pub fn new() -> Self {
        let mut this = Self {
            map: std::collections::BTreeMap::new(),
        };
        for value in BarAnchor::values() {
            this.map.insert(value, Vec::new());
        }
        this
    }

    /// java: map.get(barAnchor) - every anchor is put by the constructor
    fn rows(&self, bar_anchor: BarAnchor) -> &Vec<Vec<compat::Shared<dyn StatusBar>>> {
        self.map.get(&bar_anchor).expect("java.lang.NullPointerException: BarPositioner.map")
    }

    /// java: map.get(barAnchor), for a mutation of the list itself
    fn rows_mut(&mut self, bar_anchor: BarAnchor) -> &mut Vec<Vec<compat::Shared<dyn StatusBar>>> {
        self.map.get_mut(&bar_anchor).expect("java.lang.NullPointerException: BarPositioner.map")
    }

    /// java: public int getRowCount(BarAnchor barAnchor)
    pub fn get_row_count(&self, bar_anchor: BarAnchor) -> i32 {
        return self.rows(bar_anchor).len() as i32;
    }

    /// Adds a row to the end of an anchor
    /// java: public void addRow(BarAnchor barAnchor)
    pub fn add_row(&mut self, bar_anchor: BarAnchor) {
        self.rows_mut(bar_anchor).push(Vec::new());
    }

    /// Adds a row at the specified index
    /// java: public void addRow(BarAnchor barAnchor, int row)
    pub fn add_row_2(&mut self, bar_anchor: BarAnchor, row: i32) {
        let rows = self.rows_mut(bar_anchor);
        for i in row..(rows.len() as i32) {
            for bar in rows[i as usize].iter() {
                compat::write(bar, |__t| __t.status_bar_base_mut().grid_y += 1);
            }
        }
        rows.insert(row as usize, Vec::new());
    }

    /// adds a bar to the end of a row
    /// java: public void addBar(BarAnchor barAnchor, int row, StatusBar bar)
    pub fn add_bar(&mut self, bar_anchor: BarAnchor, row: i32, bar: compat::Shared<dyn StatusBar>) {
        let status_bars = &mut self.rows_mut(bar_anchor)[row as usize];
        status_bars.push(bar.clone());
        compat::write(&bar, |__t| __t.status_bar_base_mut().grid_y = row);
        // java: bar.gridX = statusBars.lastIndexOf(bar); // optimization baby, start with the end!
        let grid_x = status_bars.iter().rposition(|e| Arc::ptr_eq(e, &bar)).map_or(-1, |i| i as i32);
        compat::write(&bar, |__t| __t.status_bar_base_mut().grid_x = grid_x);
        compat::write(&bar, |__t| __t.status_bar_base_mut().anchor = Some(bar_anchor));
    }

    /// adds a bar to the specified x in a row
    /// java: public void addBar(BarAnchor barAnchor, int row, int x, StatusBar bar)
    pub fn add_bar_4(&mut self, bar_anchor: BarAnchor, row: i32, x: i32, bar: compat::Shared<dyn StatusBar>) {
        let status_bars = &mut self.rows_mut(bar_anchor)[row as usize];
        for i in x..(status_bars.len() as i32) {
            compat::write(&status_bars[i as usize], |__t| __t.status_bar_base_mut().grid_x += 1);
        }
        status_bars.insert(x as usize, bar.clone());
        compat::write(&bar, |__t| __t.status_bar_base_mut().grid_y = row);
        let grid_x = index_of(status_bars, &bar);
        compat::write(&bar, |__t| __t.status_bar_base_mut().grid_x = grid_x);
        compat::write(&bar, |__t| __t.status_bar_base_mut().anchor = Some(bar_anchor));
    }

    /// removes the specified bar at x on the row. If it's row is empty after being removed, the row will be auto removed
    /// java: public void removeBar(BarAnchor barAnchor, int row, int x)
    pub fn remove_bar(&mut self, bar_anchor: BarAnchor, row: i32, x: i32) {
        let status_bars = &mut self.rows_mut(bar_anchor)[row as usize];
        let remove: compat::Shared<dyn StatusBar> = status_bars.remove(x as usize);
        compat::write(&remove, |__t| __t.status_bar_base_mut().anchor = None);
        for i in x..(status_bars.len() as i32) {
            compat::write(&status_bars[i as usize], |__t| __t.status_bar_base_mut().grid_x -= 1);
        }
        if status_bars.is_empty() {
            self.remove_row(bar_anchor, row);
        }
    }

    /// removes the specified bar on the row. If it's row is empty after being removed, the row will be auto removed
    /// java: public void removeBar(BarAnchor barAnchor, int row, StatusBar bar)
    pub fn remove_bar_bar_anchor_int_status_bar(&mut self, bar_anchor: BarAnchor, row: i32, bar: &compat::Shared<dyn StatusBar>) {
        let bar_row = &mut self.rows_mut(bar_anchor)[row as usize];
        let x: i32 = index_of(bar_row, bar);
        if x < 0 {
            return;
        }
        bar_row.remove(x as usize);
        compat::write(bar, |__t| __t.status_bar_base_mut().anchor = None);
        for i in x..(bar_row.len() as i32) {
            compat::write(&bar_row[i as usize], |__t| __t.status_bar_base_mut().grid_x -= 1);
        }
        if bar_row.is_empty() {
            self.remove_row(bar_anchor, row);
        }
    }

    /// row must be empty
    /// java: public void removeRow(BarAnchor barAnchor, int row)
    pub fn remove_row(&mut self, bar_anchor: BarAnchor, row: i32) {
        let bar_row = &self.rows(bar_anchor)[row as usize];
        if !bar_row.is_empty() {
            panic!("IllegalStateException: {}", format!("Can't remove a non-empty row ({},{})", compat::jstring(&bar_anchor), row));
        }
        let rows = self.rows_mut(bar_anchor);
        rows.remove(row as usize);
        for i in row..(rows.len() as i32) {
            for status_bar in rows[i as usize].iter() {
                compat::write(status_bar, |__t| __t.status_bar_base_mut().grid_y -= 1);
            }
        }
    }

    /// java: public LinkedList<StatusBar> getRow(BarAnchor barAnchor, int row)
    pub fn get_row(&self, bar_anchor: BarAnchor, row: i32) -> Vec<compat::Shared<dyn StatusBar>> {
        return self.rows(bar_anchor)[row as usize].clone();
    }

    /// java: public StatusBar getBar(BarAnchor barAnchor, int row, int x)
    pub fn get_bar(&self, bar_anchor: BarAnchor, row: i32, x: i32) -> compat::Shared<dyn StatusBar> {
        return self.rows(bar_anchor)[row as usize][x as usize].clone();
    }

    /// java: public boolean hasNeighbor(BarAnchor barAnchor, int row, int x, boolean right)
    pub fn has_neighbor(&self, bar_anchor: BarAnchor, row: i32, x: i32, right: bool) -> bool {
        let status_bars = &self.rows(bar_anchor)[row as usize];
        if bar_anchor.is_right() {
            return (right && x < (status_bars.len() as i32).wrapping_sub(1)) || (!right && x > 0);
        } else {
            return (right && x > 0) || (!right && x < (status_bars.len() as i32).wrapping_sub(1));
        }
    }

    /// java: public void clear()
    pub fn clear(&mut self) {
        // java: map.replaceAll((_, _) -> new LinkedList<>());
        for rows in self.map.values_mut() {
            *rows = Vec::new();
        }
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
            BarAnchorFields::new(true, true, Arc::new(move |scaled_width, scaled_height| ScreenPosition::new((scaled_width.wrapping_div(2)).wrapping_sub(91), scaled_height.wrapping_sub(if FancyStatusBars::is_experience_fancy_bar_enabled(fancy_status_bars::INSTANCE.get_mut()) { 23 } else { 35 }))), SizeRule::target_size_3(12, 182, 2), Arc::new(move |anchor_position| ScreenRectangle::new_at(anchor_position.x(), anchor_position.y().wrapping_sub(20), 182, 20))),
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
    pub position_provider: AnchorPositionProvider,
    pub hitbox_provider: AnchorHitboxProvider,
    pub up: bool,
    pub right: bool,
    pub size_rule: SizeRule,
}

impl BarAnchorFields {
    /// java: BarAnchor(boolean up, boolean right, AnchorPositionProvider positionProvider, SizeRule sizeRule, AnchorHitboxProvider hitboxProvider)
    fn new(up: bool, right: bool, position_provider: AnchorPositionProvider, size_rule: SizeRule, hitbox_provider: AnchorHitboxProvider) -> Self {
        Self { position_provider, hitbox_provider, up, right, size_rule }
    }

    /// java: BarAnchor(boolean up, boolean right, AnchorPositionProvider positionProvider, SizeRule sizeRule)
    fn new_4(up: bool, right: bool, position_provider: AnchorPositionProvider, size_rule: SizeRule) -> Self {
        // java: this(up, right, positionProvider, sizeRule,
        //         anchorPosition -> new ScreenRectangle(anchorPosition.x() - (right ? 0 : 20), anchorPosition.y() - (up ? 20 : 0), 20, 20));
        Self::new(
            up,
            right,
            position_provider,
            size_rule,
            Arc::new(move |anchor_position: ScreenPosition| {
                ScreenRectangle::new_at(anchor_position.x().wrapping_sub(if right { 0 } else { 20 }), anchor_position.y().wrapping_sub(if up { 20 } else { 0 }), 20, 20)
            }),
        )
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
    /// (named `target_size_3`: the record accessor `targetSize()` is `target_size`)
    pub fn target_size_3(mut target_size: i32, mut total_width: i32, mut min_size: i32) -> SizeRule {
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
        // java: barAnchor == this.barAnchor (a null anchor is unequal, not a NullPointerException)
        return x == self.x && y == self.y && Some(bar_anchor) == self.bar_anchor;
    }

}

/// java: @FunctionalInterface interface AnchorPositionProvider
/// a functional interface: `getPosition(int scaledWidth, int scaledHeight)` - a lambda / method reference is this closure type
pub type AnchorPositionProvider = Arc<dyn Fn(i32, i32) -> ScreenPosition + Send + Sync>;

/// java: @FunctionalInterface interface AnchorHitboxProvider
/// a functional interface: `getHitbox(ScreenPosition anchorPosition)` - a lambda / method reference is this closure type
pub type AnchorHitboxProvider = Arc<dyn Fn(ScreenPosition) -> ScreenRectangle + Send + Sync>;
