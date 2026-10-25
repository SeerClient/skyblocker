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
/// (the screen's identity: the `SCREEN_GENERATION` of the init the search bar was shown on)
pub static OPENED_HANDLED_SCREEN: compat::MainThread<Option<u64>> = compat::MainThread::new(|| None);
/// Every `AFTER_INIT` is a new screen identity for `handledScreen == openedHandledScreen` (Java's `AFTER_INIT` also
/// nulls `openedHandledScreen`, so a re-initialised screen counts as a new one there too).
static SCREEN_GENERATION: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
/// `showSearchBar` asked for by a click on the `SearchTextWidget` or the key: run on the screen's widget tree by the next
/// event that hands it over (`AFTER_MOUSE_CLICK`).
static PENDING_SHOW: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
/// java: private static final BitSet slots = new BitSet(90);
pub static SLOTS: compat::MainThread<BitSet> = compat::MainThread::new(|| BitSet::with_nbits(90));
/// java: private static final BitSet computedSlots = new BitSet(90);
pub static COMPUTED_SLOTS: compat::MainThread<BitSet> = compat::MainThread::new(|| BitSet::with_nbits(90));
/// Whether the screen `AFTER_INIT` last saw passed `init`'s checks: Fabric registers the key listener on that screen
/// (`ScreenKeyboardEvents.allowKeyPress(handledScreen)`), the port's key event is global (see `init`).
pub static SCREEN_ACCEPTED: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));
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
        // Fabric registers the key listener on the screen instance (`ScreenKeyboardEvents.allowKeyPress(handledScreen)`),
        // so it lives as long as that screen; the port's key event is global, so it is registered once and answers for
        // the screen `AFTER_INIT` last accepted (`SCREEN_ACCEPTED`, reset by every screen's init).
        screen_events::AFTER_INIT.register(Arc::new(|screen: &mut Screen, width: i32, height: i32| {
            let _ = (width, height);
            compat::set_state(&SCREEN_ACCEPTED, false);
            // a new screen object (or the same one re-initialised): `handledScreen == openedHandledScreen` is false again
            SCREEN_GENERATION.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            PENDING_SHOW.store(false, std::sync::atomic::Ordering::Relaxed);
            let inventory_search_config = &SkyblockerConfigManager::get().ui_and_visuals.inventory_search;
            //   java: if (!inventorySearchConfig.enabled.isEnabled() || !(screen instanceof AbstractContainerScreen<?> handledScreen) || CatharsisCompatibility.isGuiElementHidden("skyblocker:inventorySearch")) return;
            if !inventory_search_config.enabled.is_enabled() || !compat::container_screen_open() || crate::entry::catharsis_compatibility::is_gui_element_hidden("skyblocker:inventorySearch") {
                return;
            }
            *OPENED_HANDLED_SCREEN.get_mut() = None;
            if inventory_search_config.clickable_text {
                //   java: Screens.getWidgets(handledScreen).add(new SearchTextWidget(handledScreen));
                screen.add_renderable_widget(SearchTextWidget::new(screen));
            }
            compat::set_state(&SCREEN_ACCEPTED, true);
            // java: ScreenEvents.remove(handledScreen).register(InventorySearch::onScreenClosed); - registered with the
            // search bar in Java; the port's screen removal is one hook for every screen (see `install_screen_closed`)
            InventorySearch::install_screen_closed();
        }));
        //   java: ScreenKeyboardEvents.allowKeyPress(handledScreen).register((_, input) -> { .. });
        // The game's `ALLOW_SCREEN_KEY` hands the open screen's widget tree with every key the screen is handed;
        // `SCREEN_ACCEPTED` is "this screen registered the listener" (its `AFTER_INIT` passed the checks).
        screen_keyboard_events::ALLOW_SCREEN_KEY.register(Arc::new(|screen: &mut Screen, action: i32, key_event: &KeyEvent| {
            // allowKeyPress: a press or a repeat, not a release
            if action == mcrust::com::mojang::blaze3d::platform::input_constants::RELEASE || !compat::get_state(&SCREEN_ACCEPTED) {
                return true;
            }
            let inventory_search_config = &SkyblockerConfigManager::get().ui_and_visuals.inventory_search;
            //   java: if (input.key() == (inventorySearchConfig.ctrlK ? InputConstants.KEY_K : InputConstants.KEY_F) && input.hasControlDownWithQuirk()) {
            if key_event.key() == (if inventory_search_config.ctrl_k { mcrust::com::mojang::blaze3d::platform::input_constants::KEY_K } else { mcrust::com::mojang::blaze3d::platform::input_constants::KEY_F }) && key_event.has_control_down_with_quirk() {
                //   java: InventorySearch.showSearchBar(handledScreen); return false;
                InventorySearch::show_search_bar(screen);
                return false;
            }
            return true;
        }));
        // `SearchTextWidget.onClick` -> `showSearchBar(screen)`: the widget's click is not handed its screen, the event that
        // follows the click in the same press is (`ScreenMouseEvents.afterMouseClick`, the screen's own widget tree).
        mcrust::mcrust::modding::events::screen_mouse_events::AFTER_MOUSE_CLICK.register(Arc::new(|screen: &mut Screen, _mouse: (i32, i32), _pressed: bool| {
            if compat::get_state(&SCREEN_ACCEPTED) && PENDING_SHOW.swap(false, std::sync::atomic::Ordering::Relaxed) {
                InventorySearch::show_search_bar(screen);
            }
        }));
    }

    /// `ScreenEvents.remove(handledScreen).register(InventorySearch::onScreenClosed)`: a HEAD injector on `Screen#removed`
    /// (every screen's removal; only an open search bar has anything to reset).
    fn install_screen_closed() {
        static ONCE: std::sync::Once = std::sync::Once::new();
        ONCE.call_once(|| {
            let hooked = mcrust::mcrust::modding::hook::inject("net.minecraft.client.gui.screens.Screen#removed", mcrust::mcrust::modding::hook::At::Head, |ci: &mut mcrust::mcrust::modding::hook::CallbackInfo<'_>| {
                if OPENED_HANDLED_SCREEN.get().is_none() {
                    return;
                }
                if let Some(screen) = ci.find_arg::<Screen>() {
                    InventorySearch::on_screen_closed(screen);
                }
            });
            if let Err(e) = hooked {
                logger().error(&format!("[Skyblocker Inventory Search] Failed to hook the screen removal: {:?}", e));
            }
        });
    }

    /// java: public static void showSearchBar(AbstractContainerScreen<?> handledScreen)
    /// (`handledScreen` is the open container screen's widget tree; its identity is the init it was last given,
    /// `SCREEN_GENERATION`)
    pub fn show_search_bar(handled_screen: &mut Screen) {
        let generation: u64 = SCREEN_GENERATION.load(std::sync::atomic::Ordering::Relaxed);
        // `handledScreen == openedHandledScreen`: identity (false while none is open)
        if *OPENED_HANDLED_SCREEN.get() == Some(generation) {
            return;
        }
        // java: openedHandledScreen = handledScreen;
        *OPENED_HANDLED_SCREEN.get_mut() = Some(generation);
        let text_field_widget: AbstractWidget = InventorySearch::get_text_field_widget(handled_screen);
        // java: Screens.getWidgets(handledScreen).removeIf(button -> button instanceof SearchTextWidget);
        // (removed first so the indices below stay the box's)
        let search_texts: Vec<usize> = handled_screen.children().iter().enumerate().filter(|(_, w)| w.mod_widget().is_some_and(|slot| slot.is::<SearchTextWidget>())).map(|(i, _)| i).collect();
        for index in search_texts.into_iter().rev() {
            handled_screen.remove_widget(index);
        }
        // java: Screens.getWidgets(handledScreen).addFirst(textFieldWidget);
        // (the port's screen appends; the box is the screen's only text input, so its place in the list orders nothing)
        let index: usize = handled_screen.add_renderable_widget(text_field_widget);
        // java: handledScreen.setFocused(textFieldWidget);
        handled_screen.set_focused(Some(index));
    }

    /// java: private static EditBox getTextFieldWidget(AbstractContainerScreen<?> handledScreen)
    /// (the box as the game's widget; `SearchTextFieldWidget`'s overrides are not dispatched to an `EditBox` - see that
    /// class)
    pub fn get_text_field_widget(handled_screen: &Screen) -> AbstractWidget {
        //   java: EditBox textFieldWidget = new SearchTextFieldWidget(handledScreen.getFont(), Component.translatable("skyblocker.inventorySearch.searchInventory"));
        // (`SearchTextFieldWidget(font, message)` is `super(textRenderer, 120, 20, message)`)
        let font = compat::font();
        let mut text_field_widget: AbstractWidget = compat::edit_box(0, 0, 120, 20, component::translatable("skyblocker.inventorySearch.searchInventory"));
        //   java: textFieldWidget.setPosition((handledScreen.width - textFieldWidget.getWidth()) / 2, 15);
        let __x = (handled_screen.width.wrapping_sub(text_field_widget.get_width())).wrapping_div(2);
        text_field_widget.set_position(__x, 15);
        compat::with_edit_box(&mut text_field_widget, |__e| {
            //   java: textFieldWidget.setHint(Component.translatable("gui.socialInteractions.search_hint"));
            __e.set_hint_component(mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::translatable("gui.socialInteractions.search_hint")));
            //   java: textFieldWidget.setValue(search);
            __e.set_value(&font.splitter(), &compat::get_state(&SEARCH));
            //   java: textFieldWidget.setResponder(InventorySearch::onSearchTyped);
            __e.set_responder(Responder::Lambda(ResponderLambda::new(move |__a0| { InventorySearch::on_search_typed(__a0); })));
            // java: `new SearchTextFieldWidget(..)` - the subclass's overrides, dispatched by the game's box
            __e.mod_subclass = mcrust::mcrust::modding::widgets::EditBoxSubclassSlot::new(Arc::new(SearchTextFieldSubclass {
                message: component::translatable("skyblocker.inventorySearch.searchInventory"),
            }));
        });
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
        let _ = screen;
        *OPENED_HANDLED_SCREEN.get_mut() = None;
        COMPUTED_SLOTS.get_mut().clear();
    }

    /// java: public static void refreshSlot(int slotId)
    pub fn refresh_slot(mut slot_id: i32) {
        //   java: computedSlots.clear(slotId);
        COMPUTED_SLOTS.get_mut().clear_bit(slot_id as usize);
    }

}

/// java: private static class SearchTextWidget extends StringWidget
/// (a mod widget: its box - position, size, message, `active`, hover - is the game's `AbstractWidget`)
pub struct SearchTextWidget {
    /// java: private final Component underlinedText;
    pub underlined_text: Option<MutableComponent>,
    /// java: private final Component normalText;
    pub normal_text: Option<MutableComponent>,
    /// java: private boolean hoveredState = false;
    pub hovered_state: bool,
    /// the message the widget shows (`setMessage(..)`): the draw is handed the box read-only, so the object keeps it
    pub message: MutableComponent,
}

impl SearchTextWidget {
    /// java: private SearchTextWidget(AbstractContainerScreen<?> handledScreen)
    /// (answers the widget, its object attached; `handledScreen` is the container screen's widget tree, of which only
    /// `width` is read - `onClick`'s `showSearchBar(screen)` is run on the screen the click lands on)
    pub fn new(handled_screen: &Screen) -> AbstractWidget {
        //   java: super(Component.translatable("skyblocker.inventorySearch.clickHereToSearch"), handledScreen.getFont());
        // (`StringWidget(Component, Font)` = `this(0, 0, font.width(message), 9, message, font)`)
        let message: MutableComponent = component::translatable("skyblocker.inventorySearch.clickHereToSearch");
        let font = compat::font();
        let width: i32 = mcrust::net::minecraft::client::gui::font::width(font.splitter().string_width_formatted(&mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(message.clone())));
        //   java: underlinedText = getMessage().copy().withStyle(ChatFormatting.UNDERLINE);
        let underlined_text: MutableComponent = message.copy().with_style_format(ChatFormatting::Underline);
        //   java: normalText = getMessage().copy().withStyle(ChatFormatting.GRAY);
        let normal_text: MutableComponent = message.copy().with_style_format(ChatFormatting::Gray);
        //   java: setPosition((handledScreen.width - getWidth()) / 2, 15);
        let x: i32 = (handled_screen.width.wrapping_sub(width)).wrapping_div(2);
        //   java: setMessage(normalText);
        let this = SearchTextWidget { underlined_text: Some(underlined_text), normal_text: Some(normal_text.clone()), hovered_state: false, message: normal_text.clone() };
        mcrust::mcrust::modding::widgets::ModWidgetSlot::widget(x, 15, width, mcrust::net::minecraft::client::gui::font::LINE_HEIGHT, normal_text.get_string(), Box::new(this))
    }
}

impl mcrust::mcrust::modding::widgets::ModWidget for SearchTextWidget {
    /// java: @Override public void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_widget_render_state(&mut self, widget: &AbstractWidget, context: &mut mcrust::mcrust::modding::widgets::WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        let _ = (mouse_x, mouse_y, a);
        //   java: if (isHovered() != hoveredState) { hoveredState = active = isHovered(); setMessage(hoveredState ? underlinedText : normalText); }
        // (`active` follows the hover in Java: the box is read-only here, and an inactive widget is one the mouse is not
        // over, which the click hit test already answers)
        if widget.is_hovered() != self.hovered_state {
            self.hovered_state = widget.is_hovered();
            self.message = if self.hovered_state { self.underlined_text.clone().unwrap() } else { self.normal_text.clone().unwrap() };
        }
        //   java: super.extractWidgetRenderState(graphics, mouseX, mouseY, a); - `StringWidget`: the message at the box's
        //   left, centred in its height, in the widget's colour (white) with the font's shadow
        let (x, y) = mcrust::net::minecraft::client::gui::components::string_widget::text_position(widget.get_x(), widget.get_y(), widget.get_height());
        gui_graphics_extractor::draw_formatted_text_with_drop_shadow(context.gl, context.minecraft, &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(self.message.clone()), x, y, -1, true);
    }

    /// `StringWidget` narrates nothing of its own (`AbstractStringWidget.updateWidgetNarration` is empty).
    fn update_widget_narration(&mut self, widget: &AbstractWidget, output: &mut dyn mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput) {
        let _ = (widget, output);
    }

    /// java: @Override public void onClick(MouseButtonEvent click, boolean doubled)
    fn on_click(&mut self, widget: &mut AbstractWidget, event: &MouseButtonEvent, double_click: bool) {
        let _ = (widget, event, double_click);
        // java: InventorySearch.showSearchBar(screen); - run on the screen by the click's `afterMouseClick` (see `init`)
        PENDING_SHOW.store(true, std::sync::atomic::Ordering::Relaxed);
    }
}

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
        // java: super(textRenderer, 120, 20, message); - `EditBox(font, width, height, narration)` = `this(font, 0, 0, width, height, narration)`
        // (the narration is the widget's message, which the game's `AbstractWidget` around the box keeps)
        let __base = { let __splitter = __init_text_renderer.splitter(); EditBox::new(&__splitter, 0, 0, 120, 20) };
        let mut this: Self = Self {
            base: __base,
            text_renderer: __init_text_renderer,
            message: __init_message,
        };
        this
    }

    /// java: @Override public void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_widget_render_state(&self, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        //   java: super.extractWidgetRenderState(graphics, mouseX, mouseY, a); - the game draws the box itself
        // (the box the screen holds is the game's `EditBox` widget; this override reaches it as
        // `SearchTextFieldSubclass::extract_widget_render_state`, the same body)
        gui_graphics_extractor::centered_text(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(self.message.clone()), self.get_x().wrapping_add(self.width.wrapping_div(2)), (self.get_y().wrapping_sub(1)).wrapping_sub(mcrust::net::minecraft::client::gui::font::LINE_HEIGHT), mcrust::net::minecraft::util::common_colors::WHITE);
    }

    /// java: @Override public boolean keyPressed(KeyEvent input)
    pub fn key_pressed(&self, input: &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        //   java: return super.keyPressed(input) || !input.isEscape() && isFocused();
        // The box the screen holds is the game's `EditBox` widget, which owns the focus, the active flag and the
        // clipboard `super.keyPressed` reads: this override reaches it as `SearchTextFieldSubclass::key_pressed`.
        // This object is not a widget the game hands keys to.
        let _ = input;
        return false;
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    pub fn mouse_clicked(&self, click: &MouseButtonEvent, mut doubled: bool) -> bool {
        //   java: if (isFocused() && !isMouseOver(click.x(), click.y())) { setFocused(false); return false; }
        //   java: if (super.mouseClicked(click, doubled)) { setFocused(true); return true; }
        //   java: return false;
        // The box the screen holds is the game's `EditBox` widget (focus, hover, click pass): this override reaches it
        // as `SearchTextFieldSubclass::mouse_clicked`. This object is not a widget the game hands clicks to.
        let _ = (click, doubled);
        return false;
    }

}

/// `SearchTextFieldWidget`'s overrides as the game's `EditBoxSubclass` (the box the screen holds is the game's
/// `EditBox`; `getTextFieldWidget` puts this on it).
pub struct SearchTextFieldSubclass {
    /// java: the `message` handed to `super(textRenderer, 120, 20, message)`
    pub message: MutableComponent,
}

impl mcrust::mcrust::modding::widgets::EditBoxSubclass for SearchTextFieldSubclass {
    fn as_any(&self) -> Option<&dyn std::any::Any> {
        Some(self)
    }

    /// java: @Override public boolean keyPressed(KeyEvent input) { return super.keyPressed(input) || !input.isEscape() && isFocused(); }
    /// (the game hands a key to the box only while it is the focused child, so `isFocused()` is true here)
    fn key_pressed(
        &self,
        edit_box: &mut EditBox,
        event: &mcrust::net::minecraft::client::input::key_event::KeyEvent,
        super_key_pressed: &mut dyn FnMut(&mut EditBox) -> bool,
    ) -> bool {
        super_key_pressed(edit_box) || !event.is_escape()
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    fn mouse_clicked(
        &self,
        widget: &mut mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget,
        mouse_x: f64,
        mouse_y: f64,
        _button: i32,
        _double_click: bool,
        super_mouse_clicked: &mut dyn FnMut(&mut mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget) -> bool,
    ) -> bool {
        //   java: if (isFocused() && !isMouseOver(click.x(), click.y())) { setFocused(false); return false; }
        if widget.is_focused() && !widget.is_mouse_over(mouse_x, mouse_y) {
            widget.set_focused(false, mcrust::net::minecraft::client::input_type::InputType::Mouse);
            return false;
        }
        //   java: if (super.mouseClicked(click, doubled)) { setFocused(true); return true; }
        if super_mouse_clicked(widget) {
            widget.set_focused(true, mcrust::net::minecraft::client::input_type::InputType::Mouse);
            return true;
        }
        false
    }

    /// java: @Override public void extractWidgetRenderState(..) { super..; graphics.centeredText(textRenderer, message, getX() + width / 2, getY() - 1 - textRenderer.lineHeight, CommonColors.WHITE); }
    fn extract_widget_render_state(
        &self,
        widget: &mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget,
        _edit_box: &EditBox,
        _context: &mut mcrust::mcrust::modding::widgets::WidgetRenderContext<'_>,
        super_extract: &mut dyn FnMut(),
    ) {
        super_extract();
        gui_graphics_extractor::centered_text(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(self.message.clone()), widget.get_x().wrapping_add(widget.get_width().wrapping_div(2)), (widget.get_y().wrapping_sub(1)).wrapping_sub(mcrust::net::minecraft::client::gui::font::LINE_HEIGHT), mcrust::net::minecraft::util::common_colors::WHITE);
    }
}
