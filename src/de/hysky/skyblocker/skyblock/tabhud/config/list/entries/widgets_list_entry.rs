//! Ported from de.hysky.skyblocker.skyblock.tabhud.config.list.entries.WidgetsListEntry by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::tabhud::config::list::entries::slot::boolean_slot_entry::BooleanSlotEntry;
use crate::de::hysky::skyblocker::skyblock::tabhud::config::list::entries::slot::default_slot_entry::DefaultSlotEntry;
use crate::de::hysky::skyblocker::skyblock::tabhud::config::list::entries::slot::editable_slot_entry::EditableSlotEntry;
use crate::de::hysky::skyblocker::skyblock::tabhud::config::list::entries::slot::widget_slot_entry::WidgetSlotEntry;
use crate::de::hysky::skyblocker::utils::render::gui_helper::GuiHelper;
use mcrust::mcrust::modding::widgets::ModListEntry;
use mcrust::mcrust::modding::widgets::ModSelectionList;
use mcrust::mcrust::modding::widgets::WidgetRenderContext;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::font::Font;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::narration::narratable_entry::NarratableEntry;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: public static final Component ENABLED_TEXT = Component.literal("ENABLED").withStyle(ChatFormatting.GREEN);
pub static ENABLED_TEXT: compat::MainThread<MutableComponent> = compat::MainThread::new(|| component::literal("ENABLED").with_style_format(ChatFormatting::Green));
/// java: public static final Component DISABLED_TEXT = Component.literal("DISABLED").withStyle(ChatFormatting.RED);
pub static DISABLED_TEXT: compat::MainThread<MutableComponent> = compat::MainThread::new(|| component::literal("DISABLED").with_style_format(ChatFormatting::Red));

/// java: public abstract class WidgetsListEntry extends ContainerObjectSelectionList.Entry<WidgetsListEntry>
pub struct WidgetsListEntryBase {
    /// java: extends net.minecraft.client.gui.components.ContainerObjectSelectionList$Entry
    pub base: mcrust::net::minecraft::client::gui::components::abstract_selection_list::Entry,
    /// the row's widgets (`children()`): lent to the object by its `compat::EntryAdapter`
    pub __row: mcrust::net::minecraft::client::gui::screens::screen::Screen,
    /// the screen's widgets as this object last saw them (see `compat::WidgetCache`)
    pub __widgets: compat::WidgetCache,
}

impl std::ops::Deref for WidgetsListEntryBase {
    type Target = mcrust::net::minecraft::client::gui::components::abstract_selection_list::Entry;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for WidgetsListEntryBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl WidgetsListEntryBase {
    /// java: the implicit no-argument constructor (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_in<__R>(__wrap: impl FnOnce(Self) -> (compat::Shared<dyn WidgetsListEntry>, __R)) -> (compat::Shared<dyn WidgetsListEntry>, __R) {
        let this = Self {
            base: mcrust::net::minecraft::client::gui::components::abstract_selection_list::Entry::new(0),
            __widgets: Default::default(),
            __row: mcrust::net::minecraft::client::gui::screens::screen::Screen::new(String::new()),
        };
        let (__h, __r) = __wrap(this);
        let __this = __h;
        (__this, __r)
    }

}

/// java: the virtual methods of WidgetsListEntry (implemented by WidgetsListEntryBase and every subclass)
pub trait WidgetsListEntry: compat::DynAny {
    fn widgets_list_entry_base(&self) -> &WidgetsListEntryBase;
    fn widgets_list_entry_base_mut(&mut self) -> &mut WidgetsListEntryBase;
    fn as_any(&self) -> &dyn std::any::Any;

    /// java: public void extractTooltip(GuiGraphicsExtractor graphics, int x, int y, int entryWidth, int entryHeight, int mouseX, int mouseY) (dispatch: the body for this class)
    fn extract_tooltip(&self) -> fn(&compat::Shared<dyn WidgetsListEntry>, &mut GuiGraphicsExtractor, i32, i32, i32, i32, i32, i32) {
        <dyn WidgetsListEntry>::extract_tooltip_body
    }

    /// java: public void extractBorder(GuiGraphicsExtractor graphics, int index, int y, int x, int entryWidth, int entryHeight, int mouseX, int mouseY, boolean hovered, float tickDelta) (dispatch: the body for this class)
    fn extract_border(&self) -> fn(&compat::Shared<dyn WidgetsListEntry>, &mut GuiGraphicsExtractor, i32, i32, i32, i32, i32, i32, i32, bool, f32) {
        <dyn WidgetsListEntry>::extract_border_body
    }

    /// java: @Override public List<? extends NarratableEntry> narratables() (dispatch: the body for this class)
    fn narratables(&self) -> fn(&compat::Shared<dyn WidgetsListEntry>) -> Vec<Box<dyn NarratableEntry>> {
        <dyn WidgetsListEntry>::narratables_body
    }

    /// java: protected void extractIconAndText(GuiGraphicsExtractor graphics, ItemStack icon, int y, int x, int entryHeight) (dispatch: the body for this class)
    fn extract_icon_and_text(&self) -> fn(&compat::Shared<dyn WidgetsListEntry>, &GuiGraphicsExtractor, &mcrust::net::minecraft::world::item::item_stack::ItemStack, i32, i32, i32) {
        <dyn WidgetsListEntry>::extract_icon_and_text_body
    }

}

impl dyn WidgetsListEntry {
    /// java: public void extractTooltip(GuiGraphicsExtractor graphics, int x, int y, int entryWidth, int entryHeight, int mouseX, int mouseY)
    pub fn extract_tooltip_body(__this: &compat::Shared<dyn WidgetsListEntry>, graphics: &mut GuiGraphicsExtractor, mut x: i32, mut y: i32, mut entry_width: i32, mut entry_height: i32, mut mouse_x: i32, mut mouse_y: i32) {
    }

    /// java: public void extractBorder(GuiGraphicsExtractor graphics, int index, int y, int x, int entryWidth, int entryHeight, int mouseX, int mouseY, boolean hovered, float tickDelta)
    pub fn extract_border_body(__this: &compat::Shared<dyn WidgetsListEntry>, graphics: &mut GuiGraphicsExtractor, mut index: i32, mut y: i32, mut x: i32, mut entry_width: i32, mut entry_height: i32, mut mouse_x: i32, mut mouse_y: i32, mut hovered: bool, mut tick_delta: f32) {
        if hovered {
            GuiHelper::border(graphics, x, y, entry_width, entry_height, -1);
        }
    }

    /// java: @Override public List<? extends NarratableEntry> narratables()
    pub fn narratables_body(__this: &compat::Shared<dyn WidgetsListEntry>) -> Vec<Box<dyn NarratableEntry>> {
        return vec![];
    }

    /// java: protected void extractIconAndText(GuiGraphicsExtractor graphics, ItemStack icon, int y, int x, int entryHeight)
    pub fn extract_icon_and_text_body(__this: &compat::Shared<dyn WidgetsListEntry>, graphics: &GuiGraphicsExtractor, icon: &mcrust::net::minecraft::world::item::item_stack::ItemStack, mut y: i32, mut x: i32, mut entry_height: i32) {
        let mut text_renderer: Font = compat::font();
        graphics.item(icon, x.wrapping_add(2), y.wrapping_add((entry_height.wrapping_sub(16)).wrapping_div(2)));
        gui_graphics_extractor::draw_formatted_text_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(compat::component_of(icon.get_hover_name())), x.wrapping_add(20), y.wrapping_add((entry_height.wrapping_sub(9)).wrapping_div(2)), mcrust::net::minecraft::util::common_colors::WHITE, true);
    }

}

/// WidgetsListEntry's row box and its widgets, lent by its `compat::EntryAdapter` for each call
impl compat::RowHalf for dyn WidgetsListEntry {
    fn entry_half(&mut self) -> &mut mcrust::net::minecraft::client::gui::components::abstract_selection_list::Entry {
        &mut self.widgets_list_entry_base_mut().base
    }
    fn row_parts(&mut self) -> (&mut mcrust::net::minecraft::client::gui::screens::screen::Screen, &mut compat::WidgetCache) {
        let __s = self.widgets_list_entry_base_mut();
        (&mut __s.__row, &mut __s.__widgets)
    }
}

/// WidgetsListEntry as a row of the game's list (`addEntry` attaches it)
impl ModListEntry for compat::EntryAdapter<dyn WidgetsListEntry> {
    /// java: @Override public void extractContent(GuiGraphicsExtractor graphics, int mouseX, int mouseY, boolean hovered, float deltaTicks)
    fn extract_content(&mut self, list: &mut ModSelectionList, index: usize, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, hovered: bool, a: f32) {
        self.lend(Some(list.list.entry(index)), |__h| { if let Some(__c) = compat::try_view::<BooleanSlotEntry, _>(__h) { return BooleanSlotEntry::extract_content(&*__c, &context.graphics, mouse_x, mouse_y, hovered, a); } if let Some(__c) = compat::try_view::<DefaultSlotEntry, _>(__h) { return DefaultSlotEntry::extract_content(&*__c, &context.graphics, mouse_x, mouse_y, hovered, a); } if let Some(__c) = compat::try_view::<EditableSlotEntry, _>(__h) { return EditableSlotEntry::extract_content(&*__c, &context.graphics, mouse_x, mouse_y, hovered, a); } if let Some(__c) = compat::try_view::<WidgetSlotEntry, _>(__h) { return WidgetSlotEntry::extract_content(&*__c, &context.graphics, mouse_x, mouse_y, hovered, a); }  });
        // the row's widgets (Java's `child.extractRenderState(..)` over `children()`)
        context.extract_widgets(&mut self.row, mouse_x, mouse_y);
    }

    fn children(&mut self) -> Option<&mut mcrust::net::minecraft::client::gui::screens::screen::Screen> {
        Some(&mut self.row)
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent event, boolean doubled)
    fn mouse_clicked(&mut self, list: &mut ModSelectionList, index: usize, event: &MouseButtonEvent, double_click: bool) -> bool {
        self.lend(Some(list.list.entry(index)), |__h| { if let Some(__c) = compat::try_view::<WidgetSlotEntry, _>(__h) { return WidgetSlotEntry::mouse_clicked(&*__c, event, double_click); } false })
    }

}
