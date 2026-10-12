//! Ported from de.hysky.skyblocker.utils.render.text.GridComponentManager by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::render::text::grid_formatted_char_sequence::GridFormattedCharSequence;
use crate::de::hysky::skyblocker::utils::render::text::grid_tooltip_component::GridTooltipComponent;
use mcrust::net::minecraft::client::gui::font::Font;

/// java: private static final int PADDING = 4;
pub const PADDING: i32 = 4;

/// java: public class GridComponentManager
#[derive(Clone)]
pub struct GridComponentManager {
    /// java: private final Object2ObjectMap<String, IntList> widths = new Object2ObjectOpenHashMap<>();
    /// (its `defaultReturnValue(IntList.of())` is the empty list `get` answers for a missing group)
    pub widths: std::collections::HashMap<String, Vec<i32>>,
    /// java: private final Set<GridTooltipComponent> handledComponents = new ObjectOpenHashSet<>();
    /// `GridTooltipComponent` keeps `Object`'s equals/hashCode, so the set is an identity set: membership by
    /// `Arc::ptr_eq`, holding the components as Java's set does.
    pub handled_components: Vec<compat::Shared<GridTooltipComponent>>,
}

impl GridComponentManager {
    /// java: public GridComponentManager()
    pub fn new() -> compat::Shared<Self> {
        // java: widths.defaultReturnValue(IntList.of()); - `widths_of` answers the empty list for a missing group
        let __this = compat::shared(Self {
            widths: std::collections::HashMap::new(),
            handled_components: Vec::new(),
        });
        __this
    }

    /// java: `widths.get(group)` with the map's default return value `IntList.of()`
    fn widths_of(&self, group: &str) -> Vec<i32> {
        self.widths.get(group).cloned().unwrap_or_default()
    }

    /// java: void updateWidths(GridTooltipComponent component, Font font)
    pub fn update_widths(__this: &compat::Shared<Self>, component: compat::Shared<GridTooltipComponent>, font: &Font) {
        // java: if (!handledComponents.add(component)) return;
        let added: bool = compat::write(__this, |__t| {
            if __t.handled_components.iter().any(|handled| std::sync::Arc::ptr_eq(handled, &component)) {
                false
            } else {
                __t.handled_components.push(component.clone());
                true
            }
        });
        if !added {
            return;
        }
        let sequence: GridFormattedCharSequence = GridTooltipComponent::sequence(&component);
        let components = sequence.grid_contents().components();
        // java: font.width(sequence.gridContents().components().get(i)), measured before the map is borrowed
        let column_widths: Vec<i32> = components.iter().map(|column| font.width_of_cached(&mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(column.clone()))).collect();
        compat::write(__this, |__t| {
            // java: IntList list = widths.computeIfAbsent(sequence.gridContents().group(), _ -> new IntArrayList());
            let list: &mut Vec<i32> = __t.widths.entry(sequence.grid_contents().group()).or_insert_with(Vec::new);
            // java: list.size(sequence.gridContents().components().size()); - grows with 0s or truncates
            list.resize(components.len(), 0);
            for i in 0..components.len() {
                // java: list.set(i, Math.max(list.getInt(i), font.width(..)));
                list[i] = list[i].max(column_widths[i]);
            }
        });
    }

    /// java: int getTotalWidth(String group)
    pub fn get_total_width(__this: &compat::Shared<Self>, group: &str) -> i32 {
        // java: return widths.get(group).intStream().sum() + (widths.size() - 1) * PADDING;
        compat::read(__this, |__t| {
            let sum: i32 = __t.widths_of(group).iter().fold(0i32, |acc, w| acc.wrapping_add(*w));
            sum.wrapping_add(((__t.widths.len() as i32).wrapping_sub(1)).wrapping_mul(PADDING))
        })
    }

    /// java: int getWidth(String group, int column)
    pub fn get_width(__this: &compat::Shared<Self>, group: &str, column: i32) -> i32 {
        let list: Vec<i32> = compat::read(__this, |__t| __t.widths_of(group));
        if (list.len() as i32) <= column {
            return 0;
        }
        return (list[(column) as usize]).wrapping_add(PADDING);
    }

}
