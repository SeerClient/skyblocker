//! Ported from de.hysky.skyblocker.utils.render.gui.ARGBTextInput by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::com::mojang::blaze3d::platform::cursor::cursor_types::CursorTypes;
use mcrust::java::text::decimal_format_symbols::Locale;
use mcrust::mcrust::modding::widgets::ModWidget;
use mcrust::mcrust::modding::widgets::WidgetRenderContext;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::abstract_widget::WidgetKind;
use mcrust::net::minecraft::client::gui::font::Font;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::client::input::mouse_button_info::MouseButtonInfo;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::client::gui::components::events::gui_event_listener::GuiEventListener as _;
use mcrust::net::minecraft::network::chat::component::Component as _;
use mcrust::net::minecraft::util::formatted_char_sink::FormattedCharSink as _;

/// java: private static final ChatFormatting[] FORMATTINGS = new ChatFormatting[] {ChatFormatting.WHITE, ChatFormatting.RED, ChatFormatting.GREEN, ChatFormatting.BLUE};
pub static FORMATTINGS: compat::MainThread<Vec<ChatFormatting>> = compat::MainThread::new(|| vec![ChatFormatting::White, ChatFormatting::Red, ChatFormatting::Green, ChatFormatting::Blue]);
/// java: private static final String HEXADECIMAL_CHARS = "0123456789aAbBcCdDeEfF";
pub const HEXADECIMAL_CHARS: &str = "0123456789aAbBcCdDeEfF";

/// java: public class ARGBTextInput extends AbstractWidget
pub struct ARGBTextInput {
    /// java: extends net.minecraft.client.gui.components.AbstractWidget
    pub base: AbstractWidget,
    /// java: private final int length;
    pub length: i32,
    /// java: private final boolean drawBackground;
    pub draw_background: bool,
    /// java: private final Font textRenderer;
    pub text_renderer: Option<Font<'static>>,
    /// java: private final boolean hasAlpha;
    pub has_alpha: bool,
    /// java: private final int alphaMask;
    pub alpha_mask: i32,
    /// java: private String input;
    pub input: String,
    /// java: int index = 0;
    pub index: i32,
    /// java: private @Nullable IntConsumer onChange = null;
    pub on_change: Option<Arc<dyn Fn(i32)>>,
}

impl std::ops::Deref for ARGBTextInput {
    type Target = AbstractWidget;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for ARGBTextInput {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl ARGBTextInput {
    /// java: public ARGBTextInput(int x, int y, Font textRenderer, boolean drawBackground, boolean hasAlpha)
    pub fn new(mut x: i32, mut y: i32, mut text_renderer: Font, mut draw_background: bool, mut has_alpha: bool) -> compat::Shared<Self> {
        let __init_draw_background = draw_background;
        let __init_text_renderer = Some(compat::font());
        let __init_length = (if has_alpha { 8 } else { 6 });
        let __init_has_alpha = has_alpha;
        let __init_alpha_mask = (if has_alpha { 0 } else { 0xFF000000u32 as i32 });
        let __init_input = (if has_alpha { "FFFFFFFF" } else { "FFFFFF" }).to_string();
        let __this = compat::shared(Self {
            base: AbstractWidget::new(x, y, (text_renderer.width((if has_alpha { "AAAAAAAA" } else { "AAAAAA" }))).wrapping_add(if draw_background { 6 } else { 0 }), 10i32.wrapping_add(if draw_background { 4 } else { 0 }), (&component::null_to_empty(Some("ARGBTextInput")).get_string()).to_string(), WidgetKind::PlainText, OnPress::Inert),
            length: __init_length,
            draw_background: __init_draw_background,
            text_renderer: __init_text_renderer,
            has_alpha: __init_has_alpha,
            alpha_mask: __init_alpha_mask,
            input: __init_input,
            index: 0,
            on_change: None,
        });
        __this
    }

    /// java: public ARGBTextInput(int x, int y, Font textRenderer, boolean drawBackground)
    pub fn new_4(mut x: i32, mut y: i32, mut text_renderer: Font, mut draw_background: bool) -> compat::Shared<Self> {
        let __this = Self::new(x, y, compat::font(), draw_background, false);
        __this
    }

    /// java: protected OptionalInt getOptionalARGBColor(String input)
    pub fn get_optional_argb_color(__this: &compat::Shared<Self>, input: &str) -> Option<i32> {
        // java: try { int i = Integer.parseUnsignedInt(input, 16); return OptionalInt.of(alphaMask | i); }
        //       catch (NumberFormatException e) { LOGGER.error("Could not parse rgb color", e); }
        // (`parseUnsignedInt` takes an optional `+` and the full unsigned 32-bit range, as `u32::from_str_radix` does;
        // the NumberFormatException is its `Err`)
        match u32::from_str_radix(input, 16) {
            Ok(__parsed) => {
                let i: i32 = __parsed as i32;
                return Some((compat::read(__this, |__t| __t.alpha_mask) | i));
            }
            Err(e) => {
                logger().error(&format!("Could not parse rgb color: {}", e));
            }
        }
        return None;
    }

    /// java: public int getARGBColor()
    pub fn get_argb_color(__this: &compat::Shared<Self>) -> i32 {
        return ARGBTextInput::get_optional_argb_color(__this, &compat::read(__this, |__t| __t.input.clone())).unwrap_or(mcrust::net::minecraft::util::common_colors::WHITE);
    }

    /// java: public void setARGBColor(int argb)
    pub fn set_argb_color(__this: &compat::Shared<Self>, mut argb: i32) {
        { let __v = (if compat::read(__this, |__t| __t.has_alpha) { format!("{:08X}", (argb & !(compat::read(__this, |__t| __t.alpha_mask)))) } else { format!("{:06X}", (argb & !(compat::read(__this, |__t| __t.alpha_mask)))) }); compat::write(__this, |__t| __t.input = __v); }
    }

    /// java: public void setOnChange(@Nullable IntConsumer onChange)
    pub fn set_on_change(__this: &compat::Shared<Self>, on_change: Arc<dyn Fn(i32)>) {
        { let __v = Some(on_change.clone()); compat::write(__this, |__t| __t.on_change = __v); }
    }

    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor context, int mouseX, int mouseY, float delta)
    pub fn extract_widget_render_state(__this: &compat::Shared<Self>, context: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut delta: f32) {
        let mut selection_start: i32 = { let (__q0,) = (&compat::substring2(&__this.lock().borrow().input, 0, compat::read(__this, |__t| __t.index)),); __this.lock().borrow().text_renderer.as_ref().unwrap().width(__q0) };
        let mut selection_end: i32 = { let (__q0,) = (&compat::substring2(&__this.lock().borrow().input, 0, (compat::read(__this, |__t| __t.index)).wrapping_add(1)),); __this.lock().borrow().text_renderer.as_ref().unwrap().width(__q0) };
        let mut text_x: i32 = (__this.lock().borrow().get_x()).wrapping_add(if compat::read(__this, |__t| __t.draw_background) { 3 } else { 0 });
        let mut text_y: i32 = (__this.lock().borrow().get_y()).wrapping_add(((__this.lock().borrow().get_height()).wrapping_sub(mcrust::net::minecraft::client::gui::font::LINE_HEIGHT)).wrapping_div(2));
        if compat::read(__this, |__t| __t.draw_background) {
            context.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), __this.lock().borrow().get_x(), __this.lock().borrow().get_y(), __this.lock().borrow().get_right(), __this.lock().borrow().get_bottom(), (if __this.lock().borrow().is_focused() { mcrust::net::minecraft::util::common_colors::WHITE } else { mcrust::net::minecraft::util::common_colors::GRAY }));
            context.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), (__this.lock().borrow().get_x()).wrapping_add(1), (__this.lock().borrow().get_y()).wrapping_add(1), (__this.lock().borrow().get_right()).wrapping_sub(1), (__this.lock().borrow().get_bottom()).wrapping_sub(1), mcrust::net::minecraft::util::common_colors::BLACK);
        }
        if __this.lock().borrow().is_focused() {
            context.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), text_x.wrapping_add(selection_start), text_y, text_x.wrapping_add(selection_end), text_y.wrapping_add(mcrust::net::minecraft::client::gui::font::LINE_HEIGHT), 0xFF00BBFFu32 as i32);
            context.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), text_x.wrapping_add(selection_start), (text_y.wrapping_add(mcrust::net::minecraft::client::gui::font::LINE_HEIGHT)).wrapping_sub(1), text_x.wrapping_add(selection_end), text_y.wrapping_add(mcrust::net::minecraft::client::gui::font::LINE_HEIGHT), mcrust::net::minecraft::util::common_colors::WHITE);
        }
        // java: context.text(textRenderer, visitor -> { int start = hasAlpha ? 0 : 1; for (int i = 0; i < input.length(); i++) {
        //   if (!visitor.accept(i, isHoveredOrFocused() ? Style.EMPTY.applyFormat(FORMATTINGS[i / 2 + start]) : Style.EMPTY, input.charAt(i))) return false; }
        //   return true; }, textX, textY, CommonColors.WHITE, true);
        // (the port's text draw takes the sequence as `FormattedText`: one styled piece per character, as the visitor
        // hands them)
        let (start, input, hovered_or_focused): (i32, String, bool) = compat::read(__this, |__t| (if __t.has_alpha { 0 } else { 1 }, __t.input.clone(), __t.is_hovered_or_focused()));
        let mut sequence: mcrust::net::minecraft::network::chat::mutable_component::MutableComponent = component::empty();
        for (i, c) in input.chars().enumerate() {
            let style = if hovered_or_focused { mcrust::net::minecraft::network::chat::style::Style::EMPTY.apply_format(FORMATTINGS.get_mut()[((i as i32).wrapping_div(2)).wrapping_add(start) as usize]) } else { mcrust::net::minecraft::network::chat::style::Style::EMPTY };
            sequence = sequence.append(component::literal(&c.to_string()).with_style(&style));
        }
        gui_graphics_extractor::draw_formatted_char_sequence_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(sequence), text_x, text_y, mcrust::net::minecraft::util::common_colors::WHITE, true);
        if __this.lock().borrow().is_hovered() {
            // java: context.requestCursor(CursorTypes.IBEAM);
            compat::batch_w_request_cursor(compat::BatchWCursor::Ibeam);
        }
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    pub fn update_widget_narration(__this: &compat::Shared<Self>, builder: &dyn NarrationElementOutput) {
    }

    /// java: @Override public boolean keyPressed(KeyEvent keyInput)
    pub fn key_pressed(__this: &compat::Shared<Self>, key_input: &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        if !(__this.lock().borrow().is_focused()) {
            return false;
        }
        let mut bl: bool = match key_input.key() {
            261 => 'yield_1: {
                let mut builder: String = compat::read(__this, |__t| __t.input.clone());
                // java: builder.setCharAt(index, '0');
                ARGBTextInput::set_char_at(&mut builder, compat::read(__this, |__t| __t.index), '0');
                { let __v = builder.clone(); compat::write(__this, |__t| __t.input = __v); }
                break 'yield_1 true;
            }
            259 => 'yield_2: {
                let mut builder: String = compat::read(__this, |__t| __t.input.clone());
                // java: builder.setCharAt(index, '0');
                ARGBTextInput::set_char_at(&mut builder, compat::read(__this, |__t| __t.index), '0');
                { let __v = builder.clone(); compat::write(__this, |__t| __t.input = __v); }
                { let __v = 0i32.max((compat::read(__this, |__t| __t.index)).wrapping_sub(1)); compat::write(__this, |__t| __t.index = __v); }
                break 'yield_2 true;
            }
            263 => 'yield_3: {
                { let __v = 0i32.max((compat::read(__this, |__t| __t.index)).wrapping_sub(1)); compat::write(__this, |__t| __t.index = __v); }
                break 'yield_3 true;
            }
            262 => 'yield_4: {
                { let __v = ((compat::read(__this, |__t| __t.length)).wrapping_sub(1)).min((compat::read(__this, |__t| __t.index)).wrapping_add(1)); compat::write(__this, |__t| __t.index = __v); }
                break 'yield_4 true;
            }
            _ => false,
        };
        if bl {
            ARGBTextInput::call_on_change(__this);
            return true;
        } else {
            if key_input.is_copy() {
                // java: Minecraft.getInstance().keyboardHandler.setClipboard(input);
                let __input = compat::read(__this, |__t| __t.input.clone());
                let mut __memory = mcrust::net::minecraft::client::gui::components::edit_box::MemoryClipboard::default();
                match mcrust::net::minecraft::client::minecraft::SdlClipboard::published() {
                    Some(mut __glfw) => compat::minecraft().keyboard_handler.set_clipboard(&mut __glfw, &__input),
                    None => compat::minecraft().keyboard_handler.set_clipboard(&mut __memory, &__input),
                }
                return true;
            } else if key_input.is_paste() {
                // java: String clipboard = Minecraft.getInstance().keyboardHandler.getClipboard();
                let mut clipboard: String = {
                    let mut __memory = mcrust::net::minecraft::client::gui::components::edit_box::MemoryClipboard::default();
                    match mcrust::net::minecraft::client::minecraft::SdlClipboard::published() {
                        Some(mut __glfw) => compat::minecraft().keyboard_handler.get_clipboard(&mut __glfw),
                        None => compat::minecraft().keyboard_handler.get_clipboard(&mut __memory),
                    }
                };
                if clipboard.starts_with("#") {
                    clipboard = compat::substring(&clipboard, 1);
                }
                let mut s: String = compat::substring2(&clipboard, 0, (if compat::read(__this, |__t| __t.has_alpha) { 8 } else { 6 }).min(compat::str_len(&clipboard)));
                if let Some(color) = ARGBTextInput::get_optional_argb_color(__this, &s.to_uppercase()) {
                    ARGBTextInput::set_argb_color(__this, color);
                    ARGBTextInput::call_on_change(__this);
                }
                return true;
            }
        }
        // java: return super.keyPressed(keyInput); - `AbstractWidget` keeps `GuiEventListener.keyPressed:34-36`: `false`
        return false;
    }

    /// java: @Override public boolean charTyped(CharacterEvent input)
    pub fn char_typed(__this: &compat::Shared<Self>, input: &CharacterEvent) -> bool {
        if !(__this.lock().borrow().is_focused()) {
            return false;
        }
        if HEXADECIMAL_CHARS.contains(&input.codepoint_as_string()) {
            // java: this.input = new StringBuilder(this.input).replace(index, index+1, input.codepointAsString().toUpperCase(Locale.ENGLISH)).toString();
            {
                let (mut __input, __index) = compat::read(__this, |__t| (__t.input.clone(), __t.index));
                let __end = (__index.wrapping_add(1)).min(compat::str_len(&__input));
                __input = format!("{}{}{}", compat::substring2(&__input, 0, __index), input.codepoint_as_string().to_uppercase(), compat::substring(&__input, __end));
                compat::write(__this, |__t| __t.input = __input);
            }
            { let __v = ((compat::read(__this, |__t| __t.length)).wrapping_sub(1)).min((compat::read(__this, |__t| __t.index)).wrapping_add(1)); compat::write(__this, |__t| __t.index = __v); }
            ARGBTextInput::call_on_change(__this);
            return true;
        }
        // java: return super.charTyped(input); - `AbstractWidget` keeps `GuiEventListener.charTyped:42-44`: `false`
        return false;
    }

    /// `StringBuilder.setCharAt(index, c)` (`StringIndexOutOfBoundsException` outside the string, as Java's)
    fn set_char_at(builder: &mut String, index: i32, c: char) {
        let mut chars: Vec<char> = builder.chars().collect();
        if index < 0 || index as usize >= chars.len() {
            panic!("StringIndexOutOfBoundsException: index {}, length {}", index, chars.len());
        }
        chars[index as usize] = c;
        *builder = chars.into_iter().collect();
    }

    /// java: protected void callOnChange()
    pub fn call_on_change(__this: &compat::Shared<Self>) {
        if compat::read(__this, |__t| __t.on_change.clone()).is_some() {
            ((compat::read(__this, |__t| __t.on_change.clone()).unwrap()))(ARGBTextInput::get_argb_color(__this));
        }
    }

    /// java: @Override public void onClick(MouseButtonEvent click, boolean doubled)
    pub fn on_click(__this: &compat::Shared<Self>, click: &MouseButtonEvent, mut doubled: bool) {
        // java: super.onClick(click, doubled); - `AbstractWidget.onClick:100-101` is empty
        { let __v = ARGBTextInput::find_clicked_char(__this, ((click.x()) as i32)); compat::write(__this, |__t| __t.index = __v); }
    }

    /// java: @Override protected void onDrag(MouseButtonEvent click, double deltaX, double deltaY)
    pub fn on_drag(__this: &compat::Shared<Self>, click: &MouseButtonEvent, mut delta_x: f64, mut delta_y: f64) {
        // java: super.onDrag(click, deltaX, deltaY); - `AbstractWidget.onDrag:106-107` is empty
        { let __v = ARGBTextInput::find_clicked_char(__this, ((click.x()) as i32)); compat::write(__this, |__t| __t.index = __v); }
    }

    /// java: @Override public boolean mouseScrolled(double mouseX, double mouseY, double horizontalAmount, double verticalAmount)
    pub fn mouse_scrolled(__this: &compat::Shared<Self>, mut mouse_x: f64, mut mouse_y: f64, mut horizontal_amount: f64, mut vertical_amount: f64) -> bool {
        if !(__this.lock().borrow().is_mouse_over(mouse_x, mouse_y)) {
            return false;
        }
        let mut i: i32 = (ARGBTextInput::find_clicked_char(__this, ((mouse_x) as i32))).wrapping_div(2);
        let mut begin_index: i32 = i.wrapping_mul(2);
        let an_int: Option<i32>;
        // java: try { int parsedInt = Integer.parseUnsignedInt(input.substring(beginIndex, beginIndex + 2), 16); anInt = OptionalInt.of(parsedInt); }
        //       catch (NumberFormatException e) { anInt = OptionalInt.empty(); LOGGER.error("[Skyblocker] Failed to parse integer", e); }
        // (`substring` throws IndexOutOfBoundsException, which the catch does not take: `substring2` panics alike)
        let __digits: String = compat::substring2(&compat::read(__this, |__t| __t.input.clone()), begin_index, begin_index.wrapping_add(2));
        match u32::from_str_radix(&__digits, 16) {
            Ok(__parsed) => {
                let parsed_int: i32 = __parsed as i32;
                an_int = Some(parsed_int);
            }
            Err(e) => {
                an_int = None;
                logger().error(&format!("[Skyblocker] Failed to parse integer: {}", e));
            }
        }
        if an_int.is_some() {
            let mut prev: i32 = an_int.clone().unwrap();
            let mut new_int: i32 = prev.wrapping_add((((((vertical_amount.signum()) as i32) as f64) * vertical_amount.abs().ceil())) as i32);
            new_int = new_int.clamp(0, 255);
            if new_int != prev {
                let mut format: String = format!("{:02X}", new_int);
                // java: input = new StringBuilder(input).replace(beginIndex, beginIndex + 2, format).toString();
                {
                    let __input: String = compat::read(__this, |__t| __t.input.clone());
                    let __v = format!("{}{}{}", compat::substring2(&__input, 0, begin_index), format, compat::substring(&__input, begin_index.wrapping_add(2)));
                    compat::write(__this, |__t| __t.input = __v);
                }
                ARGBTextInput::call_on_change(__this);
            }
        }
        return true;
    }

    /// java: private int findClickedChar(int mouseX)
    pub fn find_clicked_char(__this: &compat::Shared<Self>, mut mouse_x: i32) -> i32 {
        return (compat::str_len(&({ let (__q0, __q1,) = (&compat::read(__this, |__t| __t.input.clone()), (mouse_x.wrapping_sub(__this.lock().borrow().get_x())).wrapping_sub(if compat::read(__this, |__t| __t.draw_background) { 3 } else { 0 }),); __this.lock().borrow().text_renderer.as_ref().unwrap().plain_substr_by_width(__q0, __q1) }))).clamp(0, (compat::read(__this, |__t| __t.length)).wrapping_sub(1));
    }

}

/// ARGBTextInput's widget half (`super`'s fields: x, y, size, message, ...), lent by the game for each call
impl compat::WidgetHalf for ARGBTextInput {
    fn widget_half(&mut self) -> &mut AbstractWidget {
        &mut self.base
    }
}

/// ARGBTextInput as the game's widget: `compat::WidgetAdapter::widget(handle)` where Java hands it on
impl ModWidget for compat::WidgetAdapter<ARGBTextInput> {
    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor context, int mouseX, int mouseY, float delta)
    fn extract_widget_render_state(&mut self, widget: &AbstractWidget, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        self.lend_ref(widget, |__h| ARGBTextInput::extract_widget_render_state(__h, &mut context.graphics, mouse_x, mouse_y, a))
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    fn update_widget_narration(&mut self, widget: &AbstractWidget, output: &mut dyn NarrationElementOutput) {
        self.lend_ref(widget, |__h| ARGBTextInput::update_widget_narration(__h, output))
    }

    /// java: @Override public void onClick(MouseButtonEvent click, boolean doubled)
    fn on_click(&mut self, widget: &mut AbstractWidget, event: &MouseButtonEvent, double_click: bool) {
        self.lend(widget, |__h| ARGBTextInput::on_click(__h, event, double_click))
    }

    /// java: @Override protected void onDrag(MouseButtonEvent click, double deltaX, double deltaY)
    fn on_drag(&mut self, widget: &mut AbstractWidget, event: &MouseButtonEvent, dx: f64, dy: f64) {
        self.lend(widget, |__h| ARGBTextInput::on_drag(__h, event, dx, dy))
    }

    /// java: @Override public boolean mouseScrolled(double mouseX, double mouseY, double horizontalAmount, double verticalAmount)
    fn mouse_scrolled(&mut self, widget: &mut AbstractWidget, x: f64, y: f64, scroll_x: f64, scroll_y: f64) -> bool {
        self.lend(widget, |__h| ARGBTextInput::mouse_scrolled(__h, x, y, scroll_x, scroll_y))
    }

    /// java: @Override public boolean keyPressed(KeyEvent keyInput)
    fn key_pressed(&mut self, widget: &mut AbstractWidget, event: &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        self.lend(widget, |__h| ARGBTextInput::key_pressed(__h, event))
    }

    /// java: @Override public boolean charTyped(CharacterEvent input)
    fn char_typed(&mut self, widget: &mut AbstractWidget, event: &CharacterEvent) -> bool {
        self.lend(widget, |__h| ARGBTextInput::char_typed(__h, event))
    }

}

// NOTE(fabric2seer): charTyped, extractWidgetRenderState, keyPressed, mouseScrolled, onClick, onDrag, updateWidgetNarration override(s) of AbstractWidget - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them
