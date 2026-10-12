//! Ported from de.hysky.skyblocker.skyblock.dungeon.partyfinder.PartyEntryListWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::dungeon::partyfinder::party_entry::NoParties;
use crate::de::hysky::skyblocker::skyblock::dungeon::partyfinder::party_entry::PartyEntry;
use crate::de::hysky::skyblocker::skyblock::dungeon::partyfinder::party_entry::YourParty;
use mcrust::java::text::decimal_format_symbols::Locale;
use mcrust::mcrust::modding::widgets::ModList;
use mcrust::mcrust::modding::widgets::ModSelectionList;
use mcrust::mcrust::modding::widgets::WidgetRenderContext;
use mcrust::net::minecraft::client::gui::components::abstract_selection_list::AbstractSelectionList;
use mcrust::net::minecraft::client::gui::font::Font;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: public static String BASE_SKULL_NBT = """ { "SkullOwner": { "Id": [ 1215241996, -1849412511, -1161255720, -889217537 ], "Properties": { "textures": [ { "Value": "%TEXTURE%" } ] } } } """;
pub static BASE_SKULL_NBT: LazyLock<Mutex<String>> = LazyLock::new(|| Mutex::new("{\n\"SkullOwner\": {\n\t\"Id\": [\n\t\t\t1215241996,\n\t\t\t-1849412511,\n\t\t\t-1161255720,\n\t\t\t-889217537\n\t\t],\n\t\"Properties\": {\n\t\"textures\": [\n\t\t{\n\t\t\"Value\": \"%TEXTURE%\"\n\t\t}\n\t]\n\t}\n}\n}\n".to_string()));

/// java: public class PartyEntryListWidget extends ContainerObjectSelectionList<PartyEntry>
pub struct PartyEntryListWidget {
    /// java: extends net.minecraft.client.gui.components.ContainerObjectSelectionList
    pub base: ModSelectionList,
    /// java: protected List<PartyEntry> partyEntries;
    pub party_entries: Vec<compat::Shared<dyn PartyEntry>>,
    /// java: protected boolean isActive = true;
    pub is_active: bool,
    /// java: private String search = "";
    pub search: String,
}

impl std::ops::Deref for PartyEntryListWidget {
    type Target = AbstractSelectionList;
    fn deref(&self) -> &Self::Target {
        &self.base.list
    }
}

impl std::ops::DerefMut for PartyEntryListWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base.list
    }
}

impl PartyEntryListWidget {
    /// java: public PartyEntryListWidget(Minecraft minecraftClient, int width, int height, int y, int itemHeight)
    pub fn new(minecraft_client: &mut mcrust::net::minecraft::client::minecraft::Minecraft, mut width: i32, mut height: i32, mut y: i32, mut item_height: i32) -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            base: mcrust::mcrust::modding::widgets::ModSelectionList::new(mcrust::mcrust::modding::widgets::ListKind::Container, width, height, y, item_height),
            party_entries: Default::default(),
            is_active: true,
            search: "".to_string(),
        });
        __this
    }

    /// java: @Override public int getRowWidth()
    pub fn get_row_width(__this: &compat::Shared<Self>) -> i32 {
        return 336;
    }

    /// java: public void setEntries(List<PartyEntry> partyEntries)
    pub fn set_entries(__this: &compat::Shared<Self>, mut party_entries: Vec<compat::Shared<dyn PartyEntry>>) {
        { let __v = party_entries.clone(); compat::write(__this, |__t| __t.party_entries = __v); }
        PartyEntryListWidget::update_display(__this);
    }

    /// java: public void updateDisplay()
    pub fn update_display(__this: &compat::Shared<Self>) {
        let mut entries: Vec<compat::Shared<dyn PartyEntry>> = compat::read(__this, |__t| __t.party_entries.clone()).into_iter().collect::<Vec<_>>();
        let __this_w1 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        entries.retain(|e| !(|party_entry| { let Some(__this_1) = __this_w1.get().upgrade() else { return Default::default(); }; (!(party_entry.lock().borrow().party_entry_base().note.to_lowercase().contains(&compat::read((&__this_1), |__t| __t.search.clone()))) && !(party_entry.lock().borrow().as_any().is::<compat::Shared<YourParty>>())) })(e));
        entries.sort_by(|a, b| { let key = |__a0| ({ let __f = compat::dispatch(__a0, |__t| __t.is_locked()); __f(__a0) }); key(a).cmp(&key(b)) });
        entries.sort_by(|a, b| { let key = |party_entry| !(party_entry.lock().borrow().as_any().is::<compat::Shared<YourParty>>()); key(a).cmp(&key(b)) });
        if entries.is_empty() && !(__this.lock().borrow().party_entries.is_empty()) {
            entries.push((NoParties::new() as compat::Shared<dyn PartyEntry>));
        }
        { let (__q0,) = (entries.clone(),); __this.lock().borrow_mut().replace_entries(__q0) };
    }

    /// java: public void setSearch(String s)
    pub fn set_search(__this: &compat::Shared<Self>, s: &str) {
        { let __v = s.to_lowercase(); compat::write(__this, |__t| __t.search = __v); }
        PartyEntryListWidget::update_display(__this);
        __this.lock().borrow_mut().refresh_scroll_amount();
    }

    /// java: @Override protected int scrollBarX()
    pub fn scroll_bar_x(__this: &compat::Shared<Self>) -> i32 {
        return (((compat::read(__this, |__t| __t.width)).wrapping_div(2)).wrapping_add((PartyEntryListWidget::get_row_width(__this)).wrapping_div(2))).wrapping_add(2);
    }

    /// java: public void setWidgetActive(boolean active)
    pub fn set_widget_active(__this: &compat::Shared<Self>, mut active: bool) {
        { let __v = active; compat::write(__this, |__t| __t.is_active = __v); }
    }

    /// java: public boolean isWidgetActive()
    pub fn is_widget_active(__this: &compat::Shared<Self>) -> bool {
        return compat::read(__this, |__t| __t.is_active);
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    pub fn mouse_clicked(__this: &compat::Shared<Self>, click: &MouseButtonEvent, mut doubled: bool) -> bool {
        // TODO(fabric2seer): `AbstractContainerWidget.visible` is a field of the port's `AbstractContainerWidget`, which the port's `AbstractSelectionList` does not hold (no Deref, no `base` field)
        if !(compat::read(__this, |__t| __t.visible)) {
            return false;
        }
        // TODO(fabric2seer): `AbstractContainerWidget::mouse_clicked` ports `mouseClicked`, but the port's `AbstractSelectionList` holds no `AbstractContainerWidget` (no Deref, no `base` field) to call it on
        // rust: return { let (__q0, __q1,) = (click.clone(), doubled,); __this.lock().borrow_mut().base.list.mouse_clicked(__q0, __q1) };
        return todo!("fabric2seer: see the TODO above");
    }

    /// java: @Override public void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_widget_render_state(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        if (compat::write(__this, |__t| { let __l = &mut __t.base; compat::list_entries::<dyn PartyEntry>(__l) })).is_empty() {
            let mut string: MutableComponent = component::translatable("skyblocker.partyFinder.loadingError");
            let mut text_renderer: Font = compat::font();
            gui_graphics_extractor::text_with_word_wrap_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(string.clone()), __this.lock().borrow().get_row_left(), (__this.lock().borrow().get_y()).wrapping_add(10), PartyEntryListWidget::get_row_width(__this), mcrust::net::minecraft::util::common_colors::WHITE, false);
        } else {
            // TODO(fabric2seer): `ContainerObjectSelectionList.extractWidgetRenderState` has no Rust method in the port (looked for `extract_widget_render_state` in ContainerObjectSelectionList and its supertypes)
            // rust: __this.lock().borrow_mut().base.list.extract_widget_render_state(graphics, mouse_x, mouse_y, a);
        }
    }

    /// java: @Override protected void extractListSeparators(GuiGraphicsExtractor graphics)
    pub fn extract_list_separators(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor) {
    }

    /// java: @Override protected void extractListBackground(GuiGraphicsExtractor graphics)
    pub fn extract_list_background(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor) {
    }

}

/// PartyEntryListWidget's list (the port's `ModSelectionList`) until the game takes it (`compat::attach_list`)
impl compat::ListHalf for PartyEntryListWidget {
    fn list_half(&mut self) -> &mut ModSelectionList {
        &mut self.base
    }
}

/// PartyEntryListWidget's overrides of the list's own methods, asked by the port's list bodies
impl ModList for compat::ListClass<PartyEntryListWidget> {
    /// java: @Override public int getRowWidth()
    fn get_row_width(&self, list: &AbstractSelectionList, super_answer: i32) -> i32 {
        self.ask(list, super_answer, |__h| PartyEntryListWidget::get_row_width(__h))
    }

    /// java: @Override protected int scrollBarX()
    fn scroll_bar_x(&self, list: &AbstractSelectionList, super_answer: i32) -> i32 {
        self.ask(list, super_answer, |__h| PartyEntryListWidget::scroll_bar_x(__h))
    }

    /// java: @Override protected void extractListBackground(GuiGraphicsExtractor graphics)
    fn extract_list_background(&self, list: &AbstractSelectionList, context: &mut WidgetRenderContext<'_>, super_extract_list_background: &mut dyn FnMut(&mut WidgetRenderContext<'_>)) {
        let _ = super_extract_list_background; // the override replaces the base's body (it never calls it)
        self.ask(list, (), |__h| PartyEntryListWidget::extract_list_background(__h, &context.graphics))
    }

    /// java: @Override protected void extractListSeparators(GuiGraphicsExtractor graphics)
    fn extract_list_separators(&self, list: &AbstractSelectionList, context: &mut WidgetRenderContext<'_>, super_extract_list_separators: &mut dyn FnMut(&mut WidgetRenderContext<'_>)) {
        let _ = super_extract_list_separators; // the override replaces the base's body (it never calls it)
        self.ask(list, (), |__h| PartyEntryListWidget::extract_list_separators(__h, &context.graphics))
    }

}

// NOTE(fabric2seer): extractListBackground, extractListSeparators, extractWidgetRenderState, getRowWidth, mouseClicked, scrollBarX override(s) of ContainerObjectSelectionList - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them
