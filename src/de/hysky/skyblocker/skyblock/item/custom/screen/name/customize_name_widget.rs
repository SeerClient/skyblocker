//! Ported from de.hysky.skyblocker.skyblock.item.custom.screen.name.CustomizeNameWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::config_utils;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::item::custom::screen::name::visitor::get_clicked_position_visitor::GetClickedPositionVisitor;
use crate::de::hysky::skyblocker::skyblock::item::custom::screen::name::visitor::get_render_width_visitor::GetRenderWidthVisitor;
use crate::de::hysky::skyblocker::skyblock::item::custom::screen::name::visitor::get_style_visitor::GetStyleVisitor;
use crate::de::hysky::skyblocker::skyblock::item::custom::screen::name::visitor::insert_text_visitor::InsertTextVisitor;
use crate::de::hysky::skyblocker::skyblock::item::custom::screen::name::visitor::set_style_visitor::SetStyleVisitor;
use crate::de::hysky::skyblocker::utils::ok_lab_color::OkLabColor;
use crate::de::hysky::skyblocker::utils::render::gui_helper::GuiHelper;
use crate::de::hysky::skyblocker::utils::sky_block_colors::SkyBlockColors;
use mcrust::com::mojang::blaze3d::platform::cursor::cursor_types::CursorTypes;
use mcrust::mcrust::modding::widgets::ModWidget;
use mcrust::mcrust::modding::widgets::WidgetRenderContext;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::components::abstract_container_widget::AbstractContainerWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::abstract_widget::WidgetKind;
use mcrust::net::minecraft::client::gui::components::events::gui_event_listener::GuiEventListener;
use mcrust::net::minecraft::client::gui::components::ime_preedit_overlay::IMEPreeditOverlay;
use mcrust::net::minecraft::client::gui::components::tooltip::Tooltip;
use mcrust::net::minecraft::client::gui::font::Font;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::layouts::grid_layout::GridLayout;
use mcrust::net::minecraft::client::gui::layouts::layout_element::LayoutElement;
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::input_with_modifiers::InputWithModifiers;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::client::input::mouse_button_info::MouseButtonInfo;
use mcrust::net::minecraft::client::input::preedit_event::PreeditEvent;
use mcrust::net::minecraft::client::minecraft;
use mcrust::net::minecraft::client::sounds::sound_manager::SoundManager;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::network::chat::style::Style;
use mcrust::net::minecraft::network::chat::text_color::TextColor;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::util::argb;
use mcrust::net::minecraft::util::string_util;
use mcrust::net::minecraft::util::util;
use crate::de::hysky::skyblocker::injected::skyblocker_stack::SkyblockerStack as _;
use mcrust::net::minecraft::client::gui::components::renderable::Renderable as _;
use mcrust::net::minecraft::client::gui::layouts::abstract_layout::AbstractLayout as _;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final Identifier INNER_SPACE_TEXTURE = SkyblockerMod.id("menu_inner_space");
pub static INNER_SPACE_TEXTURE: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("menu_inner_space"));
/// java: private static final int PADDING = 3;
pub const PADDING: i32 = 3;

/// java: public class CustomizeNameWidget extends AbstractContainerWidget
pub struct CustomizeNameWidget {
    /// java: extends net.minecraft.client.gui.components.AbstractContainerWidget
    pub base: AbstractContainerWidget,
    /// java: private final Font textRenderer = client.font;
    pub text_renderer: Option<Font<'static>>,
    /// java: private String uuid = "";
    pub uuid: String,
    /// java: private Component text = Component.empty();
    pub text: MutableComponent,
    /// java: private String textString = "";
    pub text_string: String,
    /// java: private final TextField textField;
    pub text_field: Option<compat::Shared<TextField>>,
    /// java: private final StringWidget previewWidget;
    pub preview_widget: Option<AbstractWidget>,
    /// java: private final FormattingButton[] formattingButtons;
    pub formatting_buttons: Vec<compat::Shared<FormattingButton>>,
    /// java: private final List<AbstractWidget> widgets;
    pub widgets: Vec<AbstractWidget>,
    /// java: private final GridLayout grid = new GridLayout();
    pub grid: GridLayout,
    /// java: private int selectionStart;
    pub selection_start: i32,
    /// java: private int selectionEnd;
    pub selection_end: i32,
    /// java: private @Nullable Style insertAs;
    pub insert_as: Option<Style>,
}

impl std::ops::Deref for CustomizeNameWidget {
    type Target = AbstractContainerWidget;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for CustomizeNameWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl CustomizeNameWidget {
    /// java: public CustomizeNameWidget(Screen parent)
    pub fn new(mut parent: mcrust::net::minecraft::client::gui::screens::screen::Screen) -> Self {
        // TODO(fabric2seer): static call ImmutableList.builder(..)
        //   java: ImmutableList.Builder<AbstractWidget> builder = ImmutableList.builder();
        let mut builder: crate::unresolved::Builder = todo!("fabric2seer: the initialiser of `builder` (TODO above)");
        // TODO(fabric2seer): static `AbstractScrollArea.defaultSettings` has no Rust counterpart in the port
        //   java: super(0, 0, 0, 0, Component.literal("Customize Item Name"), AbstractScrollArea.defaultSettings(4));
        let mut this: Self = todo!("constructor");
        // TODO(fabric2seer): a widget put in a layout outside a mod screen's own methods: the port's layouts hold a widget by its index in the screen's widget list
        // rust: this.text_field = Some(TextField::new());
        // rust: this.grid.add_child_span(this.text_field.clone().unwrap(), 1, 0, 1, 20).unwrap();
        // TODO(fabric2seer): `.add(..)` on a Builder
        //   java: builder.add(textField);
        this.formatting_buttons = vec![FormattingButton::new2("B".to_string(), ChatFormatting::Bold, Arc::new(move |__a0| __a0.is_bold())), FormattingButton::new2("I".to_string(), ChatFormatting::Italic, Arc::new(move |__a0| __a0.is_italic())), FormattingButton::new2("U".to_string(), ChatFormatting::Underline, Arc::new(move |__a0| __a0.is_underlined())), FormattingButton::new2("S".to_string(), ChatFormatting::Strikethrough, Arc::new(move |__a0| __a0.is_strikethrough())), FormattingButton::new2("|||".to_string(), ChatFormatting::Obfuscated, Arc::new(move |__a0| __a0.is_obfuscated()))];
        this.add_formatting_buttons(&mut builder);
        // TODO(fabric2seer): `.add(..)` on a Builder
        //   java: builder.add(grid.addChild(Button.builder(Component.translatable("skyblocker.customItemNames.screen.customColor"), _ ->
        //   java: 				client.gui.setScreen(ColorPopup.create(parent, color -> setStyle(Style.EMPTY.withColor(color))))
        //   java: 		).size(48, 16).build(), 2, 17, 1, 3));
        // TODO(fabric2seer): `.add(..)` on a Builder
        //   java: builder.add(grid.addChild(Button.builder(Component.translatable("skyblocker.customItemNames.screen.gradientColor"), _ ->
        //   java: 				client.gui.setScreen(ColorPopup.createGradient(parent, this::createGradient))
        //   java: 		).size(48, 16).build(), 3, 17, 1, 3));
        // TODO(fabric2seer): `.add(..)` on a Builder
        //   java: builder.add(grid.addChild(new StringWidget(20 * 16, textRenderer.lineHeight, Component.translatable("skyblocker.customItemNames.screen.howToRemove").withStyle(ChatFormatting.ITALIC, ChatFormatting.GRAY), textRenderer)/*.alignLeft()*/, 4, 0, 1, 20, LayoutSettings.defaults().paddingTop(2)));
        // TODO(fabric2seer): `.add(..)` on a Builder
        //   java: builder.add(previewWidget = grid.addChild(new StringWidget(20 * 16, textRenderer.lineHeight, Component.empty(), textRenderer).setMaxWidth(20 * 16, StringWidget.TextOverflow.SCROLLING), 5, 0, 1, 20, LayoutSettings.defaults().paddingVertical(2).alignHorizontallyCenter()));
        // TODO(fabric2seer): `.build(..)` on a Builder
        //   java: widgets = builder.build();
        this.grid.arrange_elements();
        // TODO(fabric2seer): `LayoutElement::set_position` ports `setPosition`, but the port's `GridLayout` holds no `LayoutElement` (no Deref, no `base` field) to call it on
        // rust: this.grid.set_position(this.get_x().wrapping_add(PADDING), this.get_y().wrapping_add(PADDING));
        this.set_size(this.grid.get_width().wrapping_add(PADDING.wrapping_mul(2)), this.grid.get_height().wrapping_add(PADDING.wrapping_mul(2)));
        this.selection_start = { this.selection_end = compat::str_len(&this.text_string); this.selection_end };
        this
    }

    /// java: private void addFormattingButtons(ImmutableList.Builder<AbstractWidget> builder)
    pub fn add_formatting_buttons(&mut self, builder: &mut crate::unresolved::Builder) {
        for i in 0..(self.formatting_buttons.len() as i32) {
            let mut button: compat::Shared<FormattingButton> = self.formatting_buttons[(i) as usize].clone();
            // TODO(fabric2seer): `.add(..)` on a Builder
            //   java: builder.add(grid.addChild(button, 0, i));
        }
        let mut color_button_index: i32 = 0;
        for formatting in mcrust::net::minecraft::chat_formatting::VALUES {
            if TextColor::from_legacy_format(formatting).is_some() {
                // TODO(fabric2seer): `.add(..)` on a Builder
                //   java: builder.add(grid.addChild(new ColorButton(formatting), 2, colorButtonIndex++));
            }
        }
    }

    /// java: @Override public void setX(int x)
    pub fn set_x(&mut self, mut x: i32) {
        self.base.set_x(x);
        { let __s0 = self.get_x().wrapping_add(PADDING); self.grid.set_x(__s0) };
    }

    /// java: @Override public void setY(int y)
    pub fn set_y(&mut self, mut y: i32) {
        self.base.set_y(y);
        { let __s0 = self.get_y().wrapping_add(PADDING); self.grid.set_y(__s0) };
    }

    /// java: public void setItem(ItemStack stack)
    pub fn set_item(&mut self, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack) {
        self.uuid = stack.get_uuid();
        self.set_text(compat::component_of(stack.get_hover_name()).copy(), false);
    }

    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float deltaTicks)
    pub fn extract_widget_render_state(&self, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut delta_ticks: f32) {
        gui_graphics_extractor::blit_sprite_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), INNER_SPACE_TEXTURE.get_mut(), self.get_x(), self.get_y(), self.get_width(), self.get_height());
        if crate::de::hysky::skyblocker::debug::debug::Debug::debug_enabled() {
            gui_graphics_extractor::draw_formatted_char_sequence(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::literal(&format!("Selection Start: {}, Selection End: {}", self.selection_start, self.selection_end))), self.get_x(), self.get_bottom(), -1);
            // TODO(fabric2seer): `fmt` takes 1 argument(s) in the port (`f: &mut std::fmt::Formatter<'_>`), Java passes 0
            // rust: gui_graphics_extractor::draw_formatted_char_sequence(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::literal(&format!("Insert Style: {}", (if self.insert_as.is_none() { "null".to_string() } else { self.insert_as.as_ref().unwrap().fmt() })))), self.get_x(), self.get_bottom().wrapping_add(10), -1);
        }
        for widget in self.widgets.iter() {
            // TODO(fabric2seer): `extract_render_state` is a method of the port's trait `Renderable`, which the port's `AbstractWidget` does not implement
            // rust: widget.extract_render_state(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mouse_x, mouse_y, delta_ticks);
        }
    }

    /// java: private void createGradient(int startColor, int endColor)
    pub fn create_gradient(&mut self, mut start_color: i32, mut end_color: i32) {
        let mut previous_selection_start: i32 = self.selection_start;
        let mut previous_selection_end: i32 = self.selection_end;
        let mut sel_start: i32 = self.selection_start.min(self.selection_end);
        let mut sel_size: i32 = (self.selection_end.wrapping_sub(self.selection_start)).wrapping_abs();
        if sel_size == 0 {
            return;
        }
        if sel_size == 1 {
            self.set_style(&mcrust::net::minecraft::network::chat::style::Style::EMPTY.with_color(start_color));
        } else {
            for i in 0..sel_size {
                self.selection_start = sel_start.wrapping_add(i);
                self.selection_end = (sel_start.wrapping_add(i)).wrapping_add(1);
                let mut color: i32 = OkLabColor::interpolate(start_color, end_color, (((i) as f32) / (sel_size.wrapping_sub(1) as f32)));
                self.set_style(&mcrust::net::minecraft::network::chat::style::Style::EMPTY.with_color(color));
            }
        }
        self.selection_start = previous_selection_start;
        self.selection_end = previous_selection_end;
    }

    /// java: private void setStyle(Style style)
    pub fn set_style(&mut self, style: &Style) {
        if self.selection_start == self.selection_end {
            self.insert_as = Some(style.apply_to(&(if self.insert_as.is_none() { Some(mcrust::net::minecraft::network::chat::style::Style::EMPTY) } else { self.insert_as.clone() }).unwrap()));
            return;
        }
        let mut set_style_visitor: compat::Shared<SetStyleVisitor> = SetStyleVisitor::new(style.clone(), self.selection_start, self.selection_end);
        self.text.visit_styled(&mut set_style_visitor, &mcrust::net::minecraft::network::chat::style::Style::EMPTY);
        self.set_text_1(&SetStyleVisitor::get_new_text(&set_style_visitor));
    }

    /// java: private void updateStyleButtons()
    pub fn update_style_buttons(&self) {
        let mut style_visitor: compat::Shared<GetStyleVisitor> = GetStyleVisitor::new(self.selection_start, self.selection_end);
        self.text.visit_styled(&mut style_visitor, &mcrust::net::minecraft::network::chat::style::Style::EMPTY);
        let mut style: Style = GetStyleVisitor::get_style(&style_visitor);
        for button in self.formatting_buttons.iter_mut() {
            FormattingButton::update(button, &style);
        }
    }

    /// java: public void setText(Component text, boolean updateConfig)
    pub fn set_text(&mut self, mut text: MutableComponent, mut update_config: bool) {
        self.text = text.clone();
        self.text_string = text.get_string();
        if update_config && !self.uuid.is_empty() {
            // the lambda reads this widget's fields: their values now (it runs before update_only returns)
            let __text_string = self.text_string.clone();
            let __uuid = self.uuid.clone();
            let text = text.clone();
            SkyblockerConfigManager::update_only(Arc::new(move |mut config| {
                if compat::is_blank(&__text_string) {
                    config.general.custom_item_names.remove(&__uuid);
                } else {
                    config.general.custom_item_names.insert(__uuid.clone(), text.copy().set_style(mcrust::net::minecraft::network::chat::style::Style::EMPTY.with_italic(Some(false)).with_color_format(Some(ChatFormatting::White))));
                }
            }));
        }
        self.preview_widget.as_mut().unwrap().set_message_text(mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(text.clone()));
        self.grid.arrange_elements();
        if self.text_field.is_some() {
            { let __v = true; compat::write(self.text_field.as_ref().unwrap(), |__t| __t.update_me_pretty_please = __v); }
        }
    }

    /// java: public void setText(Component text)
    pub fn set_text_1(&mut self, text: &MutableComponent) {
        self.set_text(text.clone(), true);
    }

    /// java: @Override public List<? extends GuiEventListener> children()
    pub fn children(&self) -> Vec<Box<dyn GuiEventListener>> {
        return self.widgets.clone();
    }

    /// java: @Override public boolean charTyped(CharacterEvent input)
    pub fn char_typed(&self, input: &CharacterEvent) -> bool {
        // TODO(fabric2seer): `char_typed` takes 2 argument(s) in the port (`_codepoint: char, _modifiers: i32`), Java passes 1
        // TODO(fabric2seer): `char_typed` is a method of the port's trait `GuiEventListener`, which the port's `AbstractContainerWidget` does not implement
        let __o1 = self.text_field.clone().unwrap();
        let __o2 = self.text_field.clone().unwrap();
        if self.base.char_typed(input.clone()) || compat::write(&__o2, |__t| __t.is_focused()) {
            return true;
        }
        self.set_focused(self.text_field.clone().unwrap());
        return TextField::char_typed(self.text_field.as_ref().unwrap(), input);
    }

    /// java: public void insertText(String str)
    pub fn insert_text(&mut self, mut str: String) {
        str = string_util::filter_text(&str);
        if self.text_string.is_empty() {
            { let __s0 = (component::literal(&str).set_style((if self.insert_as.is_some() { self.insert_as.clone() } else { Some(mcrust::net::minecraft::network::chat::style::Style::EMPTY) }).unwrap())).clone(); self.set_text_1(&__s0) };
        } else {
            let mut visitor: compat::Shared<InsertTextVisitor> = InsertTextVisitor::new(str.clone(), self.insert_as.clone(), self.selection_start, self.selection_end);
            self.text.visit_styled(&mut visitor, &mcrust::net::minecraft::network::chat::style::Style::EMPTY);
            self.set_text_1(&InsertTextVisitor::get_new_text(&visitor));
            self.insert_as = None;
        }
        self.selection_start = (self.selection_start.min(self.selection_end)).wrapping_add(compat::str_len(&str));
        self.selection_end = self.selection_start;
        self.update_style_buttons();
    }

    /// java: private void moveCursor(boolean left, boolean shiftHeld, boolean ctrlHeld)
    pub fn move_cursor(&mut self, mut left: bool, mut shift_held: bool, mut ctrl_held: bool) {
        if left && self.selection_start == 0 || !left && self.selection_start == compat::str_len(&self.text_string) {
            return;
        }
        if ctrl_held {
            self.selection_start = self.get_word_skip_position(left);
        } else {
            // Util.offsetByCodepoints(textString, selectionStart, left ? -1 : 1) - over the mod's string indices (one per
            // code point here; the port's `offset_by_codepoints_str` walks UTF-8 byte positions)
            self.selection_start = { let __pos = self.selection_start; let __len = compat::str_len(&self.text_string); if left { if __pos > 0 { __pos - 1 } else { __pos } } else if __pos < __len { __pos + 1 } else { __pos } };
        }
        if !shift_held {
            self.selection_end = self.selection_start;
        }
        self.insert_as = None;
        self.update_style_buttons();
    }

    /// java: private void erase(boolean left, boolean ctrlHeld)
    pub fn erase(&mut self, mut left: bool, mut ctrl_held: bool) {
        if self.selection_start != self.selection_end {
            self.insert_text("".to_string());
            return;
        }
        self.move_cursor(left, true, ctrl_held);
        self.insert_text("".to_string());
    }

    /// java: private int getWordSkipPosition(boolean left)
    pub fn get_word_skip_position(&self, mut left: bool) -> i32 {
        let mut i: i32 = self.selection_start;
        if !left {
            let mut l: i32 = compat::str_len(&self.text_string);
            i = compat::index_of_from(&self.text_string, char::from_u32((32) as u32).unwrap_or('\0'), i);
            if i == -1 {
                i = l;
            } else {
                while i < l && compat::char_at(&self.text_string, i) == ' ' {
                    i += 1;
                }
            }
        } else {
            while i > 0 && compat::char_at(&self.text_string, i.wrapping_sub(1)) == ' ' {
                i -= 1;
            }
            while i > 0 && compat::char_at(&self.text_string, i.wrapping_sub(1)) != ' ' {
                i -= 1;
            }
        }
        return i;
    }

    /// java: @Override public void setFocused(boolean focused)
    pub fn set_focused(&self, mut focused: bool) {
        if !focused {
            self.set_focused(None);
        }
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    pub fn update_widget_narration(&self, builder: &dyn NarrationElementOutput) {
    }

    /// java: @Override protected int contentHeight()
    pub fn content_height(&self) -> i32 {
        return 0;
    }

    /// java: @Override protected double scrollRate()
    pub fn scroll_rate(&self) -> f64 {
        return ((0) as f64) as f64;
    }

}

// NOTE(fabric2seer): charTyped, children, contentHeight, extractWidgetRenderState, scrollRate, setFocused, setX, setY, updateWidgetNarration override(s) of AbstractContainerWidget - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them

/// java: private class FormattingButton extends AbstractButton
pub struct FormattingButton {
    /// java: extends net.minecraft.client.gui.components.AbstractButton
    pub base: AbstractWidget,
    /// java: private boolean enabled;
    pub enabled: bool,
    /// java: private final ChatFormatting format;
    pub format: Option<ChatFormatting>,
    /// java: private final Predicate<Style> isEnabled;
    pub is_enabled: Option<Arc<dyn Fn(Style) -> bool>>,
}

impl std::ops::Deref for FormattingButton {
    type Target = AbstractWidget;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for FormattingButton {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl FormattingButton {
    /// java: protected FormattingButton(Component message, ChatFormatting format, Predicate<Style> isEnabled)
    pub fn new(mut message: MutableComponent, mut format: ChatFormatting, is_enabled: Arc<dyn Fn(Style) -> bool>) -> compat::Shared<Self> {
        let __init_format = Some(format);
        let __init_is_enabled = Some(is_enabled.clone());
        let __this = compat::shared(Self {
            base: AbstractWidget::new(0, 0, 16, 16, (&message.get_string()).to_string(), WidgetKind::PlainText, OnPress::Inert),
            enabled: false,
            format: __init_format,
            is_enabled: __init_is_enabled,
        });
        { let (__q0,) = (Tooltip::create((config_utils::FORMATTING_FORMATTER.get_mut())(format).get_string()),); __this.lock().borrow_mut().set_tooltip(__q0) };
        __this
    }

    /// java: protected FormattingButton(String str, ChatFormatting format, Predicate<Style> isEnabled)
    pub fn new2(mut str: String, mut format: ChatFormatting, is_enabled: Arc<dyn Fn(Style) -> bool>) -> compat::Shared<Self> {
        let __this = Self::new(component::literal(&str).with_style_format(format), format, is_enabled.clone());
        __this
    }

    /// java: private void update(Style style)
    pub fn update(__this: &compat::Shared<Self>, style: &Style) {
        FormattingButton::set_enabled(__this, ((compat::read(__this, |__t| __t.is_enabled.clone()).unwrap()))(style.clone()));
    }

    /// java: @Override public void onPress(InputWithModifiers input)
    pub fn on_press(__this: &compat::Shared<Self>, input: &dyn InputWithModifiers) {
        FormattingButton::set_enabled(__this, !(compat::read(__this, |__t| __t.enabled)));
        match compat::read(__this, |__t| __t.format.clone()) {
            Some(ChatFormatting::Bold) => {
                // TODO(fabric2seer): `setStyle(..)` is a method of the enclosing instance (CustomizeNameWidget.this) - an inner class has no outer reference in Rust
                //   java: setStyle(Style.EMPTY.withBold(enabled));
            }
            Some(ChatFormatting::Italic) => {
                // TODO(fabric2seer): `setStyle(..)` is a method of the enclosing instance (CustomizeNameWidget.this) - an inner class has no outer reference in Rust
                //   java: setStyle(Style.EMPTY.withItalic(enabled));
            }
            Some(ChatFormatting::Underline) => {
                // TODO(fabric2seer): `setStyle(..)` is a method of the enclosing instance (CustomizeNameWidget.this) - an inner class has no outer reference in Rust
                //   java: setStyle(Style.EMPTY.withUnderlined(enabled));
            }
            Some(ChatFormatting::Strikethrough) => {
                // TODO(fabric2seer): `setStyle(..)` is a method of the enclosing instance (CustomizeNameWidget.this) - an inner class has no outer reference in Rust
                //   java: setStyle(Style.EMPTY.withStrikethrough(enabled));
            }
            Some(ChatFormatting::Obfuscated) => {
                // TODO(fabric2seer): `setStyle(..)` is a method of the enclosing instance (CustomizeNameWidget.this) - an inner class has no outer reference in Rust
                //   java: setStyle(Style.EMPTY.withObfuscated(enabled));
            }
            _ => {
                panic!("IllegalStateException: {}", format!("Unexpected value: {}", compat::opt_str(&compat::read(__this, |__t| __t.format.clone()))).to_string());
            }
        }
    }

    /// java: private void setEnabled(boolean enabled)
    pub fn set_enabled(__this: &compat::Shared<Self>, mut enabled: bool) {
        { let __v = enabled; compat::write(__this, |__t| __t.enabled = __v); }
        { let (__q0,) = (mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from({ let (__q0,) = ((if enabled { mcrust::net::minecraft::util::common_colors::YELLOW } else { mcrust::net::minecraft::util::common_colors::WHITE }),); compat::component_of(__this.lock().borrow().get_message_component()).copy().with_color(__q0) }),); __this.lock().borrow_mut().set_message_text(__q0) };
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    pub fn update_widget_narration(__this: &compat::Shared<Self>, builder: &dyn NarrationElementOutput) {
    }

    /// java: @Override protected void extractContents(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_contents(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        // TODO(fabric2seer): `AbstractButton.extractDefaultSprite` has no Rust method in the port (looked for `extract_default_sprite` in AbstractButton and its supertypes)
        // rust: __this.lock().borrow().extract_default_sprite(graphics);
        // TODO(fabric2seer): the port's `text_renderer` ports this Java method with other parameters (mouse_x, mouse_y for Java's )
        // TODO(fabric2seer): `text_renderer` takes 2 argument(s) in the port (`mouse_x: f64, mouse_y: f64`), Java passes 0
        // TODO(fabric2seer): `AbstractButton.extractDefaultLabel` has no Rust method in the port (looked for `extract_default_label` in AbstractButton and its supertypes)
        // rust: let __h0 = &gui_graphics_extractor::text_renderer();
        // rust: __this.lock().borrow().extract_default_label(__h0);
    }

}

/// FormattingButton's widget half (`super`'s fields: x, y, size, message, ...), lent by the game for each call
impl compat::WidgetHalf for FormattingButton {
    fn widget_half(&mut self) -> &mut AbstractWidget {
        &mut self.base
    }
}

/// FormattingButton as the game's widget: `compat::WidgetAdapter::widget(handle)` where Java hands it on
impl ModWidget for compat::WidgetAdapter<FormattingButton> {
    /// java: extractWidgetRenderState - not overridden here (the base class's body has no entry point in the port)
    fn extract_widget_render_state(&mut self, widget: &AbstractWidget, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        let _ = widget;
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    fn update_widget_narration(&mut self, widget: &AbstractWidget, output: &mut dyn NarrationElementOutput) {
        self.lend_ref(widget, |__h| FormattingButton::update_widget_narration(__h, output))
    }

    /// java: @Override public void onPress(InputWithModifiers input)
    fn on_click(&mut self, widget: &mut AbstractWidget, event: &MouseButtonEvent, double_click: bool) {
        self.lend(widget, |__h| FormattingButton::on_press(__h, event))
    }

}

// NOTE(fabric2seer): extractContents, onPress, updateWidgetNarration override(s) of AbstractButton - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them

/// java: private class ColorButton extends AbstractButton
pub struct ColorButton {
    /// java: extends net.minecraft.client.gui.components.AbstractButton
    pub base: AbstractWidget,
    /// java: private final ChatFormatting color;
    pub color: Option<ChatFormatting>,
    /// java: private final int intColor;
    pub int_color: i32,
}

impl std::ops::Deref for ColorButton {
    type Target = AbstractWidget;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for ColorButton {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl ColorButton {
    /// java: private ColorButton(ChatFormatting format)
    pub fn new(mut format: ChatFormatting) -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            base: AbstractWidget::new(0, 0, 16, 16, (&(config_utils::FORMATTING_FORMATTER.get_mut())(format).get_string()).to_string(), WidgetKind::PlainText, OnPress::Inert),
            color: None,
            int_color: 0,
        });
        { let (__q0,) = (Tooltip::create(compat::component_of(__this.lock().borrow().get_message_component()).get_string()),); __this.lock().borrow_mut().set_tooltip(__q0) };
        { let __v = Some(format); compat::write(&__this, |__t| __t.color = __v); }
        { let __v = argb::opaque(SkyBlockColors::from_vanilla((TextColor::from_legacy_format(compat::read(&__this, |__t| __t.color.clone()).unwrap()).unwrap()).get_value())); compat::write(&__this, |__t| __t.int_color = __v); }
        __this
    }

    /// java: @Override public void onPress(InputWithModifiers input)
    pub fn on_press(__this: &compat::Shared<Self>, input: &dyn InputWithModifiers) {
        // TODO(fabric2seer): `setStyle(..)` is a method of the enclosing instance (CustomizeNameWidget.this) - an inner class has no outer reference in Rust
        //   java: setStyle(Style.EMPTY.withColor(color));
    }

    /// java: @Override public void extractContents(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_contents(__this: &compat::Shared<Self>, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        // TODO(fabric2seer): `AbstractButton.extractDefaultSprite` has no Rust method in the port (looked for `extract_default_sprite` in AbstractButton and its supertypes)
        // rust: __this.lock().borrow().extract_default_sprite(graphics);
        graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), (__this.lock().borrow().get_x()).wrapping_add(2), (__this.lock().borrow().get_y()).wrapping_add(2), (__this.lock().borrow().get_right()).wrapping_sub(2), (__this.lock().borrow().get_bottom()).wrapping_sub(2), compat::read(__this, |__t| __t.int_color));
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    pub fn update_widget_narration(__this: &compat::Shared<Self>, builder: &dyn NarrationElementOutput) {
    }

}

/// ColorButton's widget half (`super`'s fields: x, y, size, message, ...), lent by the game for each call
impl compat::WidgetHalf for ColorButton {
    fn widget_half(&mut self) -> &mut AbstractWidget {
        &mut self.base
    }
}

/// ColorButton as the game's widget: `compat::WidgetAdapter::widget(handle)` where Java hands it on
impl ModWidget for compat::WidgetAdapter<ColorButton> {
    /// java: extractWidgetRenderState - not overridden here (the base class's body has no entry point in the port)
    fn extract_widget_render_state(&mut self, widget: &AbstractWidget, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        let _ = widget;
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    fn update_widget_narration(&mut self, widget: &AbstractWidget, output: &mut dyn NarrationElementOutput) {
        self.lend_ref(widget, |__h| ColorButton::update_widget_narration(__h, output))
    }

    /// java: @Override public void onPress(InputWithModifiers input)
    fn on_click(&mut self, widget: &mut AbstractWidget, event: &MouseButtonEvent, double_click: bool) {
        self.lend(widget, |__h| ColorButton::on_press(__h, event))
    }

}

// NOTE(fabric2seer): extractContents, onPress, updateWidgetNarration override(s) of AbstractButton - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them

/// java: private class TextField extends AbstractWidget
#[derive(Clone)]
pub struct TextField {
    /// java: extends net.minecraft.client.gui.components.AbstractWidget
    pub base: AbstractWidget,
    /// java: private @Nullable IMEPreeditOverlay preeditOverlay;
    pub preedit_overlay: Option<IMEPreeditOverlay>,
    /// java: private int renderedSelectionStart;
    pub rendered_selection_start: i32,
    /// java: private int renderedSelectionEnd;
    pub rendered_selection_end: i32,
    /// java: private boolean updateMePrettyPlease = false;
    pub update_me_pretty_please: bool,
    /// java: private int renderStart;
    pub render_start: i32,
    /// java: private int renderEnd;
    pub render_end: i32,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl std::ops::Deref for TextField {
    type Target = AbstractWidget;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for TextField {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl TextField {
    /// java: private TextField()
    pub fn new() -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            base: AbstractWidget::new(0, 0, 320, 20, (&component::literal("TextField").get_string()).to_string(), WidgetKind::PlainText, OnPress::Inert),
            self_ref: std::sync::Weak::new(),
            preedit_overlay: None,
            rendered_selection_start: 0,
            rendered_selection_end: 0,
            update_me_pretty_please: false,
            render_start: 0,
            render_end: 0,
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        __this
    }

    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_widget_render_state(__this: &compat::Shared<Self>, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        // TODO(fabric2seer): the condition: `selectionStart` is a field of the enclosing instance (CustomizeNameWidget.this) - an inner class has no outer reference in Rust
        //   java: renderedSelectionStart != selectionStart || renderedSelectionEnd != selectionEnd || updateMePrettyPlease
        if todo!("fabric2seer: the condition (TODO above)") {
            // TODO(fabric2seer): `selectionStart` is a field of the enclosing instance (CustomizeNameWidget.this) - an inner class has no outer reference in Rust
            //   java: renderedSelectionStart = selectionStart;
            // TODO(fabric2seer): `selectionEnd` is a field of the enclosing instance (CustomizeNameWidget.this) - an inner class has no outer reference in Rust
            //   java: renderedSelectionEnd = selectionEnd;
            { let __v = false; compat::write(__this, |__t| __t.update_me_pretty_please = __v); }
            // TODO(fabric2seer): `selectionStart` is a field of the enclosing instance (CustomizeNameWidget.this) - an inner class has no outer reference in Rust
            //   java: GetRenderWidthVisitor getRenderWidthVisitor = new GetRenderWidthVisitor(selectionStart, selectionEnd);
            let mut get_render_width_visitor: compat::Shared<GetRenderWidthVisitor> = todo!("fabric2seer: the initialiser of `getRenderWidthVisitor` (TODO above)");
            // TODO(fabric2seer): `text` is a field of the enclosing instance (CustomizeNameWidget.this) - an inner class has no outer reference in Rust
            //   java: text.visit(getRenderWidthVisitor, Style.EMPTY);
            { let __v = GetRenderWidthVisitor::get_widths(&get_render_width_visitor).0; compat::write(__this, |__t| __t.render_start = __v); }
            { let __v = GetRenderWidthVisitor::get_widths(&get_render_width_visitor).1; compat::write(__this, |__t| __t.render_end = __v); }
        }
        graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), __this.lock().borrow().get_x(), __this.lock().borrow().get_y(), __this.lock().borrow().get_right(), __this.lock().borrow().get_bottom(), mcrust::net::minecraft::util::common_colors::BLACK);
        GuiHelper::border(graphics, __this.lock().borrow().get_x(), __this.lock().borrow().get_y(), __this.lock().borrow().get_width(), __this.lock().borrow().get_height(), (if __this.lock().borrow().is_focused() { mcrust::net::minecraft::util::common_colors::WHITE } else { mcrust::net::minecraft::util::common_colors::GRAY }));
        let mut text_x: i32 = TextField::get_text_x(__this);
        // TODO(fabric2seer): `textRenderer` is a field of the enclosing instance (CustomizeNameWidget.this) - an inner class has no outer reference in Rust
        //   java: int textY = getY() + (getHeight() - textRenderer.lineHeight) / 2;
        let mut text_y: i32 = todo!("fabric2seer: the initialiser of `textY` (TODO above)");
        if compat::read(__this, |__t| __t.render_start) != compat::read(__this, |__t| __t.render_end) {
            // TODO(fabric2seer): `textRenderer` is a field of the enclosing instance (CustomizeNameWidget.this) - an inner class has no outer reference in Rust
            //   java: graphics.fill(textX + renderStart, textY, textX + renderEnd, textY + textRenderer.lineHeight, CommonColors.BLUE);
        }
        if __this.lock().borrow().is_focused() {
            // TODO(fabric2seer): `selectionStart` is a field of the enclosing instance (CustomizeNameWidget.this) - an inner class has no outer reference in Rust
            //   java: graphics.verticalLine(textX + (selectionStart < selectionEnd ? renderStart : renderEnd) - 1, textY - 1, textY + textRenderer.lineHeight, CommonColors.WHITE);
        }
        // TODO(fabric2seer): `textRenderer` is a field of the enclosing instance (CustomizeNameWidget.this) - an inner class has no outer reference in Rust
        //   java: graphics.text(textRenderer, text, textX, textY, -1, false);
        if compat::read(__this, |__t| __t.preedit_overlay.clone()).is_some() {
            __this.lock().borrow_mut().preedit_overlay.as_mut().unwrap().update_input_position(text_x, text_y);
            gui_graphics_extractor::set_preedit_overlay(compat::read(__this, |__t| __t.preedit_overlay.clone()).unwrap());
        }
        TextField::handle_cursor(__this, graphics);
    }

    /// java: @Override protected void handleCursor(GuiGraphicsExtractor graphics)
    pub fn handle_cursor(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor) {
        if __this.lock().borrow().is_hovered() {
            // TODO(fabric2seer): static `CursorTypes.IBEAM` has no Rust counterpart in the port's CursorTypes file
            // TODO(fabric2seer): `GuiGraphicsExtractor.requestCursor` has no Rust method in the port (looked for `request_cursor` in GuiGraphicsExtractor and its supertypes)
            // rust: graphics.request_cursor(&CursorTypes::IBEAM);
        }
    }

    /// java: @Override public boolean keyPressed(KeyEvent input)
    pub fn key_pressed(__this: &compat::Shared<Self>, input: &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        let mut captured: bool = true;
        match input.key() {
            263 => {
                // TODO(fabric2seer): `moveCursor(..)` is a method of the enclosing instance (CustomizeNameWidget.this) - an inner class has no outer reference in Rust
                //   java: moveCursor(true, input.hasShiftDown(), input.hasControlDownWithQuirk());
            }
            262 => {
                // TODO(fabric2seer): `moveCursor(..)` is a method of the enclosing instance (CustomizeNameWidget.this) - an inner class has no outer reference in Rust
                //   java: moveCursor(false, input.hasShiftDown(), input.hasControlDownWithQuirk());
            }
            259 => {
                // TODO(fabric2seer): `erase(..)` is a method of the enclosing instance (CustomizeNameWidget.this) - an inner class has no outer reference in Rust
                //   java: erase(true, input.hasControlDownWithQuirk());
            }
            261 => {
                // TODO(fabric2seer): `erase(..)` is a method of the enclosing instance (CustomizeNameWidget.this) - an inner class has no outer reference in Rust
                //   java: erase(false, input.hasControlDownWithQuirk());
            }
            _ => {
                captured = false;
            }
        }
        if captured {
            return true;
        }
        // TODO(fabric2seer): `client` is a field of the enclosing instance (CustomizeNameWidget.this) - an inner class has no outer reference in Rust
        //   java: assert client != null;
        if input.is_select_all() {
            // TODO(fabric2seer): `selectionStart` is a field of the enclosing instance (CustomizeNameWidget.this) - an inner class has no outer reference in Rust
            //   java: selectionStart = 0;
            // TODO(fabric2seer): `selectionEnd` is a field of the enclosing instance (CustomizeNameWidget.this) - an inner class has no outer reference in Rust
            //   java: selectionEnd = textString.length();
            // TODO(fabric2seer): `updateStyleButtons(..)` is a method of the enclosing instance (CustomizeNameWidget.this) - an inner class has no outer reference in Rust
            //   java: updateStyleButtons();
            captured = true;
        } else if input.is_copy() {
            // TODO(fabric2seer): `client` is a field of the enclosing instance (CustomizeNameWidget.this) - an inner class has no outer reference in Rust
            //   java: client.keyboardHandler.setClipboard(text.getString().substring(selectionStart, selectionEnd));
            captured = true;
        } else if input.is_paste() {
            // TODO(fabric2seer): `client` is a field of the enclosing instance (CustomizeNameWidget.this) - an inner class has no outer reference in Rust
            //   java: String clipboard = client.keyboardHandler.getClipboard();
            let mut clipboard: String = todo!("fabric2seer: the initialiser of `clipboard` (TODO above)");
            if !clipboard.is_empty() {
                // TODO(fabric2seer): `insertText(..)` is a method of the enclosing instance (CustomizeNameWidget.this) - an inner class has no outer reference in Rust
                //   java: insertText(clipboard);
            }
            captured = true;
        } else if input.is_cut() {
            // TODO(fabric2seer): `client` is a field of the enclosing instance (CustomizeNameWidget.this) - an inner class has no outer reference in Rust
            //   java: client.keyboardHandler.setClipboard(text.getString().substring(selectionStart, selectionEnd));
            // TODO(fabric2seer): `insertText(..)` is a method of the enclosing instance (CustomizeNameWidget.this) - an inner class has no outer reference in Rust
            //   java: insertText("");
            captured = true;
        }
        return captured;
    }

    /// java: @Override public boolean charTyped(CharacterEvent input)
    pub fn char_typed(__this: &compat::Shared<Self>, input: &CharacterEvent) -> bool {
        if !(compat::read(__this, |__t| __t.active)) {
            return false;
        } else if input.is_allowed_chat_character() {
            // TODO(fabric2seer): `insertText(..)` is a method of the enclosing instance (CustomizeNameWidget.this) - an inner class has no outer reference in Rust
            //   java: insertText(input.codepointAsString());
            return true;
        } else {
            return false;
        }
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public boolean preeditUpdated(@Nullable final PreeditEvent event)
    pub fn preedit_updated(__this: &compat::Shared<Self>, mut event: Option<PreeditEvent>) -> bool {
        if __this.lock().borrow().is_active() {
            // TODO(fabric2seer): `CustomizeNameWidget.this` (the enclosing instance of an inner class)
            //   java: this.preeditOverlay = event != null ? new IMEPreeditOverlay(event, CustomizeNameWidget.this.textRenderer, 9 + 1) : null;
            return true;
        }
        // TODO(fabric2seer): `AbstractWidget.preeditUpdated` has no Rust method in the port (looked for `preedit_updated` in AbstractWidget and its supertypes)
        // rust: let __h0 = event.as_ref().unwrap();
        // rust: return __this.lock().borrow_mut().base.preedit_updated(__h0);
        return todo!("fabric2seer: see the TODO above");
    }

    /// java: @Override public void onClick(MouseButtonEvent click, boolean doubled)
    pub fn on_click(__this: &compat::Shared<Self>, click: &MouseButtonEvent, mut doubled: bool) {
        let mut get_clicked_position_visitor: GetClickedPositionVisitor = GetClickedPositionVisitor::new_1(((click.x()) as i32).wrapping_sub(TextField::get_text_x(__this)));
        // TODO(fabric2seer): `text` is a field of the enclosing instance (CustomizeNameWidget.this) - an inner class has no outer reference in Rust
        //   java: text.visit(getClickedPositionVisitor, Style.EMPTY);
        // TODO(fabric2seer): `selectionStart` is a field of the enclosing instance (CustomizeNameWidget.this) - an inner class has no outer reference in Rust
        //   java: selectionStart = selectionEnd = getClickedPositionVisitor.getPosition() < 0 ? textString.length() : getClickedPositionVisitor.getPosition();
        // TODO(fabric2seer): `updateStyleButtons(..)` is a method of the enclosing instance (CustomizeNameWidget.this) - an inner class has no outer reference in Rust
        //   java: updateStyleButtons();
    }

    /// java: @Override protected void onDrag(MouseButtonEvent click, double offsetX, double offsetY)
    pub fn on_drag(__this: &compat::Shared<Self>, click: &MouseButtonEvent, mut offset_x: f64, mut offset_y: f64) {
        let mut get_clicked_position_visitor: GetClickedPositionVisitor = GetClickedPositionVisitor::new_1(((click.x()) as i32).wrapping_sub(TextField::get_text_x(__this)));
        // TODO(fabric2seer): `text` is a field of the enclosing instance (CustomizeNameWidget.this) - an inner class has no outer reference in Rust
        //   java: text.visit(getClickedPositionVisitor, Style.EMPTY);
        // TODO(fabric2seer): `selectionStart` is a field of the enclosing instance (CustomizeNameWidget.this) - an inner class has no outer reference in Rust
        //   java: selectionStart = getClickedPositionVisitor.getPosition() < 0 ? textString.length() : getClickedPositionVisitor.getPosition();
        // TODO(fabric2seer): `updateStyleButtons(..)` is a method of the enclosing instance (CustomizeNameWidget.this) - an inner class has no outer reference in Rust
        //   java: updateStyleButtons();
    }

    /// java: private int getTextX()
    pub fn get_text_x(__this: &compat::Shared<Self>) -> i32 {
        return (__this.lock().borrow().get_x()).wrapping_add(2);
    }

    /// java: @Override public void setFocused(final boolean focused)
    pub fn set_focused(__this: &compat::Shared<Self>, mut focused: bool) {
        // TODO(fabric2seer): `set_focused` takes 2 argument(s) in the port (`focused: bool, last_input_type: InputType`), Java passes 1
        // rust: __this.lock().borrow_mut().base.set_focused(focused);
        if focused {
            // TODO(fabric2seer): `on_text_input_focus_change` takes 6 argument(s) in the port (`gfx: &mut Minecraft, g: &crate::lwjgl::glfw::Glfw, is_focused: bool, screen_present: bool, screen: &mut crate::net::minecraft::client::gui::screens::screen::Screen, screen_extra: &mut crate::net::minecraft::client::gui::gui::ScreenExtra`), Java passes 2
            // rust: minecraft::on_text_input_focus_change(&mut __this, &focused);
        }
    }

    /// java: @Override public void playDownSound(SoundManager soundManager)
    pub fn play_down_sound(__this: &compat::Shared<Self>, sound_manager: &SoundManager) {
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    pub fn update_widget_narration(__this: &compat::Shared<Self>, builder: &dyn NarrationElementOutput) {
    }

}

/// TextField's widget half (`super`'s fields: x, y, size, message, ...), lent by the game for each call
impl compat::WidgetHalf for TextField {
    fn widget_half(&mut self) -> &mut AbstractWidget {
        &mut self.base
    }
}

/// TextField as the game's widget: `compat::WidgetAdapter::widget(handle)` where Java hands it on
impl ModWidget for compat::WidgetAdapter<TextField> {
    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_widget_render_state(&mut self, widget: &AbstractWidget, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        self.lend_ref(widget, |__h| TextField::extract_widget_render_state(__h, &mut context.graphics, mouse_x, mouse_y, a))
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    fn update_widget_narration(&mut self, widget: &AbstractWidget, output: &mut dyn NarrationElementOutput) {
        self.lend_ref(widget, |__h| TextField::update_widget_narration(__h, output))
    }

    /// java: @Override public void onClick(MouseButtonEvent click, boolean doubled)
    fn on_click(&mut self, widget: &mut AbstractWidget, event: &MouseButtonEvent, double_click: bool) {
        self.lend(widget, |__h| TextField::on_click(__h, event, double_click))
    }

    /// java: @Override protected void onDrag(MouseButtonEvent click, double offsetX, double offsetY)
    fn on_drag(&mut self, widget: &mut AbstractWidget, event: &MouseButtonEvent, dx: f64, dy: f64) {
        self.lend(widget, |__h| TextField::on_drag(__h, event, dx, dy))
    }

    /// java: @Override public boolean keyPressed(KeyEvent input)
    fn key_pressed(&mut self, widget: &mut AbstractWidget, event: &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        self.lend(widget, |__h| TextField::key_pressed(__h, event))
    }

    /// java: @Override public boolean charTyped(CharacterEvent input)
    fn char_typed(&mut self, widget: &mut AbstractWidget, event: &CharacterEvent) -> bool {
        self.lend(widget, |__h| TextField::char_typed(__h, event))
    }

    /// java: @Override public void setFocused(final boolean focused)
    fn set_focused(&mut self, widget: &mut AbstractWidget, focused: bool) {
        self.lend(widget, |__h| TextField::set_focused(__h, focused))
    }

}

// NOTE(fabric2seer): charTyped, extractWidgetRenderState, handleCursor, keyPressed, onClick, onDrag, playDownSound, preeditUpdated, setFocused, updateWidgetNarration override(s) of AbstractWidget - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them
