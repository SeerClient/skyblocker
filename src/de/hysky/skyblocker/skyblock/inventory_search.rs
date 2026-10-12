//! Ported from de.hysky.skyblocker.skyblock.InventorySearch by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::configs::ui_and_visuals_config::InventorySearchConfig;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::utils::item_utils::ItemUtils;
use mcrust::java::text::decimal_format_symbols::Locale;
use mcrust::java::util::bit_set::BitSet;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::edit_box;
use mcrust::net::minecraft::client::gui::components::edit_box::EditBox;
use mcrust::net::minecraft::client::gui::components::edit_box::Responder;
use mcrust::net::minecraft::client::gui::components::edit_box::ResponderLambda;
use mcrust::net::minecraft::client::gui::components::string_widget;
use mcrust::net::minecraft::client::gui::font::Font;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::screens::inventory::abstract_container_screen::AbstractContainerScreen;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::world::inventory::slot::Slot;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static @Nullable AbstractContainerScreen<?> openedHandledScreen = null;
pub static OPENED_HANDLED_SCREEN: compat::MainThread<Option<AbstractContainerScreen>> = compat::MainThread::new(|| None);
// TODO(fabric2seer): `new` takes 0 argument(s) in the port (``), Java passes 1
//   java: private static final BitSet slots = new BitSet(90);
/// java: private static final BitSet slots = new BitSet(90);
pub static SLOTS: compat::MainThread<BitSet> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `slots` - see the TODO above"));
// TODO(fabric2seer): `new` takes 0 argument(s) in the port (``), Java passes 1
//   java: private static final BitSet computedSlots = new BitSet(90);
/// java: private static final BitSet computedSlots = new BitSet(90);
pub static COMPUTED_SLOTS: compat::MainThread<BitSet> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `computedSlots` - see the TODO above"));
/// java: private static String search = "";
pub static SEARCH: LazyLock<Mutex<String>> = LazyLock::new(|| Mutex::new("".to_string()));

/// java: public class InventorySearch
#[derive(Clone)]
pub struct InventorySearch {}

impl InventorySearch {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        screen_events::AFTER_INIT.register(Arc::new(|screen: &mut Screen, width: i32, height: i32| {
            let inventory_search_config = &SkyblockerConfigManager::get().ui_and_visuals.inventory_search;
            // TODO(fabric2seer): an `instanceof` pattern under `||` / `!`
            //   java: if (!inventorySearchConfig.enabled.isEnabled() || !(screen instanceof AbstractContainerScreen<?> handledScreen) || CatharsisCompatibility.isGuiElementHidden("skyblocker:inventorySearch")) return;
            *OPENED_HANDLED_SCREEN.get_mut() = None;
            if inventory_search_config.clickable_text {
                // TODO(fabric2seer): Fabric API `Screens.getWidgets` has no SeerClient equivalent
                //   java: Screens.getWidgets(handledScreen).add(new SearchTextWidget(handledScreen));
            }
            screen_keyboard_events::ALLOW_KEY_PRESS.register(Arc::new(|action: i32, key_event: &KeyEvent| {
                // TODO(fabric2seer): the condition: `.key(..)` on a value of unknown type
                //   java: input.key() == (inventorySearchConfig.ctrlK ? InputConstants.KEY_K : InputConstants.KEY_F) && input.hasControlDownWithQuirk()
                if todo!("fabric2seer: the condition (TODO above)") {
                    // TODO(fabric2seer): unknown name `handledScreen`
                    //   java: InventorySearch.showSearchBar(handledScreen);
                    return false;
                }
                return true;
                true
            }));
        }));
    }

    /// java: public static void showSearchBar(AbstractContainerScreen<?> handledScreen)
    pub fn show_search_bar(handled_screen: &AbstractContainerScreen) {
        // `handledScreen == openedHandledScreen`: identity (false while none is open)
        if OPENED_HANDLED_SCREEN.get().as_ref().is_some_and(|opened| std::ptr::eq(handled_screen, opened)) {
            return;
        }
        // TODO(fabric2seer): the mod keeps the game's `AbstractContainerScreen` (Java holds a reference to the live object); the port's `AbstractContainerScreen` is not a value to copy and the port lends the mod no handle to it
        // rust: *OPENED_HANDLED_SCREEN.get_mut() = Some(handled_screen.clone());
        let mut text_field_widget: EditBox = InventorySearch::get_text_field_widget(handled_screen);
        // TODO(fabric2seer): Fabric API `Screens.getWidgets` has no SeerClient equivalent
        //   java: Screens.getWidgets(handledScreen).addFirst(textFieldWidget);
        // TODO(fabric2seer): Fabric API `Screens.getWidgets` has no SeerClient equivalent
        //   java: Screens.getWidgets(handledScreen).removeIf(button -> button instanceof SearchTextWidget);
        // TODO(fabric2seer): `AbstractContainerScreen.setFocused` has no Rust method in the port (looked for `set_focused` in AbstractContainerScreen and its supertypes)
        // rust: handled_screen.set_focused(&text_field_widget);
        // TODO(fabric2seer): ScreenEvents.remove(..).register: no SeerClient equivalent yet
        //   java: ScreenEvents.remove(handledScreen).register(InventorySearch::onScreenClosed);
    }

    /// java: private static EditBox getTextFieldWidget(AbstractContainerScreen<?> handledScreen)
    pub fn get_text_field_widget(handled_screen: &AbstractContainerScreen) -> EditBox {
        // TODO(fabric2seer): `AbstractContainerScreen.getFont` has no Rust method in the port (looked for `get_font` in AbstractContainerScreen and its supertypes)
        // rust: let mut text_field_widget: EditBox = SearchTextFieldWidget::new(handled_screen.get_font(), component::translatable("skyblocker.inventorySearch.searchInventory"));
        let mut text_field_widget: EditBox = todo!("fabric2seer: the initialiser of `textFieldWidget` (TODO above)");
        // TODO(fabric2seer): `set_position` takes 3 argument(s) in the port (`x: i32, y: i32, splitter: &StringSplitter<W>`), Java passes 2
        // rust: text_field_widget.set_position((handled_screen.screen.width.wrapping_sub(text_field_widget.get_width())).wrapping_div(2), 15);
        text_field_widget.set_hint_component(component::translatable("gui.socialInteractions.search_hint"));
        // TODO(fabric2seer): `set_value` takes 2 argument(s) in the port (`splitter: &StringSplitter<W>, value: &str`), Java passes 1
        // rust: text_field_widget.set_value(&compat::get_state(&SEARCH));
        text_field_widget.set_responder(Responder::Lambda(ResponderLambda::new(move |__a0| { InventorySearch::on_search_typed(__a0); })));
        return text_field_widget;
    }

    /// java: public static boolean isSearching()
    pub fn is_searching() -> bool {
        return OPENED_HANDLED_SCREEN.get_mut().is_some();
    }

    /// java: private static boolean computeSlot(Slot slot)
    pub fn compute_slot(slot: &Slot) -> bool {
        // `slot.hasItem()` / `slot.getItem()` of the open menu's slot
        if !compat::slot_has_item(slot) {
            return false;
        }
        let item = compat::slot_item(slot);
        if compat::component_of(item.get_hover_name()).get_string().to_lowercase().contains(&compat::get_state(&SEARCH)) {
            return true;
        }
        return ItemUtils::get_lore_line_if(&item, Arc::new(move |s| s.to_lowercase().contains(&compat::get_state(&SEARCH)))).is_some();
    }

    /// java: public static boolean slotMatches(Slot slot)
    pub fn slot_matches(slot: &Slot) -> bool {
        if !(COMPUTED_SLOTS.get_mut().get((slot.index) as usize)) {
            COMPUTED_SLOTS.get_mut().set((slot.index) as usize);
            SLOTS.get_mut().set_to((slot.index) as usize, InventorySearch::compute_slot(slot));
        }
        return SLOTS.get_mut().get((slot.index) as usize);
    }

    /// java: private static void onSearchTyped(String text)
    pub fn on_search_typed(text: &str) {
        COMPUTED_SLOTS.get_mut().clear();
        compat::set_state(&SEARCH, text.to_lowercase());
    }

    /// java: private static void onScreenClosed(Screen screen)
    pub fn on_screen_closed(screen: &mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        *OPENED_HANDLED_SCREEN.get_mut() = None;
        COMPUTED_SLOTS.get_mut().clear();
    }

    /// java: public static void refreshSlot(int slotId)
    pub fn refresh_slot(mut slot_id: i32) {
        // TODO(fabric2seer): `clear` takes 0 argument(s) in the port (``), Java passes 1
        // rust: COMPUTED_SLOTS.get_mut().clear(slot_id);
    }

}

/// java: private static class SearchTextWidget extends StringWidget
pub struct SearchTextWidget {
    /// java: extends net.minecraft.client.gui.components.StringWidget
    pub base: AbstractWidget,
    /// java: private final Component underlinedText;
    pub underlined_text: Option<MutableComponent>,
    /// java: private final Component normalText;
    pub normal_text: Option<MutableComponent>,
    /// java: private final AbstractContainerScreen<?> screen;
    pub screen: Option<AbstractContainerScreen>,
    /// java: private boolean hoveredState = false;
    pub hovered_state: bool,
}

impl std::ops::Deref for SearchTextWidget {
    type Target = AbstractWidget;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for SearchTextWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl SearchTextWidget {
    /// java: private SearchTextWidget(AbstractContainerScreen<?> handledScreen)
    pub fn new(mut handled_screen: AbstractContainerScreen) -> Self {
        // TODO(fabric2seer): `AbstractContainerScreen.getFont` has no Rust method in the port (looked for `get_font` in AbstractContainerScreen and its supertypes)
        //   java: super(Component.translatable("skyblocker.inventorySearch.clickHereToSearch"), handledScreen.getFont());
        let mut this: Self = todo!("constructor");
        // TODO(fabric2seer): `width` takes 2 argument(s) in the port (`max_width: i32, text_width: i32`), Java passes 0
        // rust: this.set_position((handled_screen.screen.width.wrapping_sub(string_widget::width())).wrapping_div(2), 15);
        this.underlined_text = Some(compat::component_of(this.get_message_component()).copy().with_style_format(ChatFormatting::Underline));
        this.normal_text = Some(compat::component_of(this.get_message_component()).copy().with_style_format(ChatFormatting::Gray));
        // TODO(fabric2seer): the mod keeps the game's `AbstractContainerScreen` (Java holds a reference to the live object); the port's `AbstractContainerScreen` is not a value to copy and the port lends the mod no handle to it
        // rust: this.screen = Some(handled_screen.clone());
        this.set_message_text(mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(this.normal_text.clone().unwrap()));
        this
    }

    /// java: @Override public void onClick(MouseButtonEvent click, boolean doubled)
    pub fn on_click(&self, click: &MouseButtonEvent, mut doubled: bool) {
        InventorySearch::show_search_bar(self.screen.as_ref().unwrap());
    }

    /// java: @Override public void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_widget_render_state(&mut self, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        if self.is_hovered() != self.hovered_state {
            self.hovered_state = { self.active = self.is_hovered(); self.active };
            if self.hovered_state {
                { let __s0 = mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(self.underlined_text.clone().unwrap()); self.set_message_text(__s0) };
            } else {
                { let __s0 = mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(self.normal_text.clone().unwrap()); self.set_message_text(__s0) };
            }
        }
        // TODO(fabric2seer): `StringWidget.extractWidgetRenderState` has no Rust method in the port (looked for `extract_widget_render_state` in StringWidget and its supertypes)
        // rust: self.base.extract_widget_render_state(graphics, mouse_x, mouse_y, a);
    }

}

// NOTE(fabric2seer): extractWidgetRenderState, onClick override(s) of StringWidget - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them

/// java: public static class SearchTextFieldWidget extends EditBox
pub struct SearchTextFieldWidget {
    /// java: extends net.minecraft.client.gui.components.EditBox
    pub base: EditBox,
    /// java: Font textRenderer;
    pub text_renderer: Font<'static>,
    /// java: Component message;
    pub message: MutableComponent,
}

impl std::ops::Deref for SearchTextFieldWidget {
    type Target = EditBox;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for SearchTextFieldWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl SearchTextFieldWidget {
    /// java: public SearchTextFieldWidget(Font textRenderer, Component message)
    pub fn new(mut text_renderer: Font<'static>, mut message: MutableComponent) -> Self {
        let __init_text_renderer = compat::font();
        let __init_message = message.clone();
        // TODO(fabric2seer): the port's `new` ports this Java method with other parameters (splitter, x, y, width, height for Java's width, height, narration)
        //   java: super(textRenderer, 120, 20, message);
        let mut this: Self = todo!("constructor");
        this
    }

    /// java: @Override public void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_widget_render_state(&self, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        // TODO(fabric2seer): `extract_widget_render_state` takes 7 argument(s) in the port (`gl: &crate::lwjgl::opengl::Gl, gfx: &crate::net::minecraft::client::minecraft::Minecraft, w: &super::abstract_widget::AbstractWidget, e: &EditBox, millis: i64, color: i32, apply_format: Option<&dyn Fn(&str, i32) -> Option<crate::net::minecr ..`), Java passes 4
        // rust: edit_box::extract_widget_render_state(graphics, &mouse_x, &mouse_y, &a);
        gui_graphics_extractor::centered_text(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(self.message.clone()), self.get_x().wrapping_add(self.width.wrapping_div(2)), (self.get_y().wrapping_sub(1)).wrapping_sub(mcrust::net::minecraft::client::gui::font::LINE_HEIGHT), mcrust::net::minecraft::util::common_colors::WHITE);
    }

    /// java: @Override public boolean keyPressed(KeyEvent input)
    pub fn key_pressed(&self, input: &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        // TODO(fabric2seer): `key_pressed` takes 5 argument(s) in the port (`splitter: &StringSplitter<W>, event: KeyEvent, focused: bool, active: bool, clipboard: &mut dyn ClipboardAccess`), Java passes 1
        // TODO(fabric2seer): `AbstractWidget::is_focused` ports `isFocused`, but the port's `EditBox` holds no `AbstractWidget` (no Deref, no `base` field) to call it on
        // rust: return (self.base.key_pressed(input) || !input.is_escape() && self.is_focused());
        return todo!("fabric2seer: see the TODO above");
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    pub fn mouse_clicked(&self, click: &MouseButtonEvent, mut doubled: bool) -> bool {
        // TODO(fabric2seer): `AbstractWidget::is_focused` ports `isFocused`, but the port's `EditBox` holds no `AbstractWidget` (no Deref, no `base` field) to call it on
        // TODO(fabric2seer): `AbstractWidget::is_mouse_over` ports `isMouseOver`, but the port's `EditBox` holds no `AbstractWidget` (no Deref, no `base` field) to call it on
        //   java: if (isFocused() && !isMouseOver(click.x(), click.y())) { setFocused(false); return false; }
        //   java: if (super.mouseClicked(click, doubled)) { setFocused(true); return true; }
        //   java: return false;
        // rust: port gap - a mod subclass of `EditBox`: the port's `EditBox` is the `WidgetKind::EditBox` of a game
        // `AbstractWidget`, which holds the focus, the hover and the click pass this body reads; this class holds the
        // bare `EditBox`, so none of them is reachable (GAPS_batchG). Its constructor is the converter's todo, so no
        // object of this class exists to reach here.
        return false;
    }

}

// NOTE(fabric2seer): extractWidgetRenderState, keyPressed, mouseClicked override(s) of EditBox - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them
