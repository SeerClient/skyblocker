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
    pub widths: std::collections::HashMap<String, Vec<i32>>,
    /// java: private final Set<GridTooltipComponent> handledComponents = new ObjectOpenHashSet<>();
    pub handled_components: std::collections::HashSet<compat::Shared<GridTooltipComponent>>,
}

impl GridComponentManager {
    /// java: public GridComponentManager()
    pub fn new() -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            widths: std::collections::HashMap::new(),
            handled_components: std::collections::HashSet::new(),
        });
        // TODO(fabric2seer): Map.defaultReturnValue(..)
        //   java: widths.defaultReturnValue(IntList.of());
        __this
    }

    /// java: void updateWidths(GridTooltipComponent component, Font font)
    pub fn update_widths(__this: &compat::Shared<Self>, mut component: compat::Shared<GridTooltipComponent>, font: &Font) {
        if !(__this.lock().borrow_mut().handled_components.insert(component.clone())) {
            return;
        }
        let mut sequence: GridFormattedCharSequence = GridTooltipComponent::sequence(&component);
        let list = __this.lock().borrow_mut().widths.entry(sequence.grid_contents().group()).or_insert_with_key(|k| (|_| Vec::new())(k.clone()));
        (list.len() as i32);
        for i in 0..(sequence.grid_contents().components().len() as i32) {
            list[(i) as usize] = (list[(i) as usize]).max(font.width_of_cached(&mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(sequence.grid_contents().components()[(i) as usize].clone())));
        }
    }

    /// java: int getTotalWidth(String group)
    pub fn get_total_width(__this: &compat::Shared<Self>, group: &str) -> i32 {
        // TODO(fabric2seer): List.intStream(..)
        //   java: return widths.get(group).intStream().sum() + (widths.size() - 1) * PADDING;
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: int getWidth(String group, int column)
    pub fn get_width(__this: &compat::Shared<Self>, group: &str, mut column: i32) -> i32 {
        let mut list: Vec<i32> = __this.lock().borrow().widths.get(group).cloned().unwrap();
        if (list.len() as i32) <= column {
            return 0;
        }
        return (list[(column) as usize]).wrapping_add(PADDING);
    }

}
