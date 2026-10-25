//! Ported from de.hysky.skyblocker.utils.render.text.GridTooltipComponent by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::render::text::grid_component_manager::GridComponentManager;
use crate::de::hysky::skyblocker::utils::render::text::grid_formatted_char_sequence::GridFormattedCharSequence;
use mcrust::net::minecraft::client::gui::active_text_collector::ActiveTextCollector;
use mcrust::net::minecraft::client::gui::font::Font;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: public class GridTooltipComponent implements ClientTooltipComponent
pub struct GridTooltipComponent {
    /// java: private final GridFormattedCharSequence sequence;
    pub sequence: GridFormattedCharSequence,
    /// java: private @Nullable GridComponentManager manager;
    pub manager: Option<compat::Shared<GridComponentManager>>,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl GridTooltipComponent {
    /// java: public GridTooltipComponent(final GridFormattedCharSequence sequence)
    pub fn new(mut sequence: GridFormattedCharSequence) -> compat::Shared<Self> {
        let __init_sequence = sequence.clone();
        let __this = compat::shared(Self {
            self_ref: std::sync::Weak::new(),
            sequence: __init_sequence,
            manager: None,
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        __this
    }

    /// java: public void setManager(@Nullable GridComponentManager manager)
    pub fn set_manager(__this: &compat::Shared<Self>, mut manager: Option<compat::Shared<GridComponentManager>>) {
        { let __v = manager.clone(); compat::write(__this, |__t| __t.manager = __v); }
    }

    /// java: @Override public int getHeight(Font font)
    pub fn get_height(__this: &compat::Shared<Self>, font: &Font) -> i32 {
        return (mcrust::net::minecraft::client::gui::font::LINE_HEIGHT).wrapping_add(1);
    }

    /// java: public GridFormattedCharSequence sequence()
    pub fn sequence(__this: &compat::Shared<Self>) -> GridFormattedCharSequence {
        return compat::read(__this, |__t| __t.sequence.clone());
    }

    /// java: @Override public int getWidth(Font font)
    pub fn get_width(__this: &compat::Shared<Self>, font: &Font) -> i32 {
        if compat::read(__this, |__t| __t.manager.clone()).is_some() {
            GridComponentManager::update_widths(&compat::read(__this, |__t| __t.manager.clone()).unwrap(), __this.clone(), font);
            return GridComponentManager::get_total_width(&compat::read(__this, |__t| __t.manager.clone()).unwrap(), &__this.lock().borrow().sequence.grid_contents().group());
        }
        return font.width_of_sequence(&compat::read(__this, |__t| __t.sequence.clone()));
    }

    /// java: @Override public void extractText(GuiGraphicsExtractor graphics, Font font, int x, int y)
    pub fn extract_text(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor, font: &Font, mut x: i32, mut y: i32) {
        // java: ActiveTextCollector textRenderer = graphics.textRenderer(); - its `accept(x, y, text)` below is
        // `RenderingTextCollector.accept`'s draw: the text left-aligned at (x, y), `ARGB.white(opacity)` (opacity 1), with
        // its drop shadow
        for i in { 0..(__this.lock().borrow().sequence.grid_contents().components().len() as i32) } {
            let column = &__this.lock().borrow().sequence.grid_contents().components()[(i) as usize];
            // java: textRenderer.accept(x, y, column);
            gui_graphics_extractor::draw_formatted_text_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(column.clone()), x, y, -1, true);
            if compat::read(__this, |__t| __t.manager.clone()).is_none() {
                x = x.wrapping_add(font.width_of_cached(&mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(column.clone())));
            } else {
                let mut width: i32 = GridComponentManager::get_width(&compat::read(__this, |__t| __t.manager.clone()).unwrap(), &__this.lock().borrow().sequence.grid_contents().group(), i);
                x = x.wrapping_add(if width == 0 { font.width_of_cached(&mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(column.clone())) } else { width });
            }
        }
    }

}

/// java: `GridTooltipComponent implements ClientTooltipComponent` - the object the game's tooltip list holds
/// (`List<ClientTooltipComponent>`): it carries the mod's shared object, so `initializeGrids` can find it again
/// (`as_any`) and hand it its manager, and every draw goes to the bodies above.
pub struct GridTooltipComponentLine(pub compat::Shared<GridTooltipComponent>);

impl mcrust::net::minecraft::client::gui::screens::inventory::tooltip::client_tooltip_component::ClientTooltipComponent for GridTooltipComponentLine {
    fn as_any(&self) -> Option<&dyn std::any::Any> {
        Some(self)
    }

    /// java: @Override public int getHeight(Font font)
    fn get_height(&self, _font: &dyn mcrust::net::minecraft::client::gui::screens::inventory::tooltip::client_tooltip_component::TooltipFont) -> i32 {
        GridTooltipComponent::get_height(&self.0, &compat::font())
    }

    /// java: @Override public int getWidth(Font font) (the game's `Font`: the tooltip's font is always `Minecraft.font`)
    fn get_width(&self, _font: &dyn mcrust::net::minecraft::client::gui::screens::inventory::tooltip::client_tooltip_component::TooltipFont) -> i32 {
        GridTooltipComponent::get_width(&self.0, &compat::font())
    }

    fn extracts_text(&self) -> bool {
        true
    }

    /// java: @Override public void extractText(GuiGraphicsExtractor graphics, Font font, int x, int y)
    fn extract_text(&self, _graphics: &mut mcrust::net::minecraft::client::gui::screens::inventory::tooltip::client_tooltip_component::TooltipGraphics<'_>, _font: &dyn mcrust::net::minecraft::client::gui::screens::inventory::tooltip::client_tooltip_component::TooltipFont, x: i32, y: i32) {
        GridTooltipComponent::extract_text(&self.0, &GuiGraphicsExtractor::new(), &compat::font(), x, y);
    }
}
