//! Ported from de.hysky.skyblocker.skyblock.item.custom.screen.name.CustomizeNameWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::config_utils;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::item::custom::screen::name::color_popup::ColorPopup;
use crate::de::hysky::skyblocker::skyblock::item::custom::screen::name::visitor::base_visitor::BaseVisitor;
use crate::de::hysky::skyblocker::skyblock::item::custom::screen::name::visitor::get_clicked_position_visitor::GetClickedPositionVisitor;
use crate::de::hysky::skyblocker::skyblock::item::custom::screen::name::visitor::get_render_width_visitor::GetRenderWidthVisitor;
use crate::de::hysky::skyblocker::skyblock::item::custom::screen::name::visitor::get_style_visitor::GetStyleVisitor;
use crate::de::hysky::skyblocker::skyblock::item::custom::screen::name::visitor::insert_text_visitor::InsertTextVisitor;
use crate::de::hysky::skyblocker::skyblock::item::custom::screen::name::visitor::set_style_visitor::SetStyleVisitor;
use crate::de::hysky::skyblocker::utils::ok_lab_color::OkLabColor;
use crate::de::hysky::skyblocker::utils::render::gui_helper::GuiHelper;
use crate::de::hysky::skyblocker::utils::sky_block_colors::SkyBlockColors;
use mcrust::com::mojang::blaze3d::platform::cursor::cursor_type::CursorType;
use mcrust::com::mojang::blaze3d::platform::cursor::cursor_types::CursorTypes;
use mcrust::com::mojang::blaze3d::platform::input_constants;
use mcrust::mcrust::modding::widgets::WidgetRenderContext;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::components::abstract_container_widget::AbstractContainerWidget;
use mcrust::net::minecraft::client::gui::components::abstract_scroll_area::ScrollbarSettings;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::abstract_widget::PressLambda;
use mcrust::net::minecraft::client::gui::components::button;
use mcrust::net::minecraft::client::gui::components::edit_box::ClipboardAccess;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::layouts::grid_layout::GridLayout;
use mcrust::net::minecraft::client::gui::layouts::layout_element::LayoutElement;
use mcrust::net::minecraft::client::gui::layouts::layout_settings::LayoutSettings;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::key_event::KeyEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::formatted_text::FormattedText;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::network::chat::style::Style;
use mcrust::net::minecraft::network::chat::text_color::TextColor;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::util::argb;
use mcrust::net::minecraft::util::common_colors;
use crate::de::hysky::skyblocker::injected::skyblocker_stack::SkyblockerStack as _;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final Identifier INNER_SPACE_TEXTURE = SkyblockerMod.id("menu_inner_space");
pub static INNER_SPACE_TEXTURE: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("menu_inner_space"));
/// java: private static final int PADDING = 3;
pub const PADDING: i32 = 3;

/// What the children call back with (the buttons' `setStyle(..)`, the colour buttons' popups, the text field's input -
/// "Most logic is done in the screen itself"). They call back while this widget hands them input, so each call is
/// recorded and run once that call returns (`run_events`), in order.
enum NameEvent {
    SetStyle(Style),
    CustomColor,
    GradientColor,
    MoveCursor(bool, bool, bool),
    Erase(bool, bool),
    SelectAll,
    Copy,
    Paste,
    Cut,
    Insert(String),
    Click(f64),
    Drag(f64),
}

type NameEvents = std::rc::Rc<std::cell::RefCell<Vec<NameEvent>>>;

/// The children's places in `widgets` (Java's `widgets` list, built in this order).
const TEXT_FIELD: usize = 0;

/// java: public class CustomizeNameWidget extends AbstractContainerWidget
pub struct CustomizeNameWidget {
    /// java: extends net.minecraft.client.gui.components.AbstractContainerWidget
    pub base: AbstractContainerWidget,
    /// java: private String uuid = "";
    pub uuid: String,
    /// java: private Component text = Component.empty();
    pub text: MutableComponent,
    /// java: private String textString = "";
    pub text_string: String,
    /// java: private final TextField textField; - its state (the widget is `widgets`' first child)
    pub text_field: compat::Shared<TextField>,
    /// java: private final StringWidget previewWidget; (its place in `widgets`)
    pub preview_widget: usize,
    /// java: private final FormattingButton[] formattingButtons;
    pub formatting_buttons: Vec<compat::Shared<FormattingButton>>,
    /// java: private final List<AbstractWidget> widgets;
    pub widgets: mcrust::net::minecraft::client::gui::screens::screen::Screen,
    /// java: private final GridLayout grid = new GridLayout();
    pub grid: LayoutElement,
    /// java: private int selectionStart;
    pub selection_start: i32,
    /// java: private int selectionEnd;
    pub selection_end: i32,
    /// java: private @Nullable Style insertAs;
    pub insert_as: Option<Style>,
    /// the children's call-backs (see `NameEvent`)
    events: NameEvents,
    /// Java's `this`, for the colour popups' call-backs
    self_ref: compat::WeakShared<Self>,
}

impl CustomizeNameWidget {
    /// java: public CustomizeNameWidget(Screen parent)
    ///
    /// (`parent` is what the colour popups draw behind; see `open_color_popup`)
    pub fn new() -> compat::Shared<Self> {
        // java: super(0, 0, 0, 0, Component.literal("Customize Item Name"), AbstractScrollArea.defaultSettings(4));
        let base = AbstractContainerWidget::new_with_scrollbar_settings(0, 0, 0, 0, FormattedText::of("Customize Item Name"), ScrollbarSettings::default_settings(4.0));
        let events: NameEvents = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let mut widgets = mcrust::net::minecraft::client::gui::screens::screen::Screen::new(String::new());
        // the gui is a grid of 20 columns, should be 16 px each
        let mut grid = GridLayout::new_default();
        // java: textField = grid.addChild(new TextField(), 1, 0, 1, 20); builder.add(textField);
        let text_field = TextField::new(events.clone());
        let index = widgets.add_renderable_widget(compat::object_widget(text_field.clone(), "TextField"));
        grid.add_child_span(LayoutElement::widget(index, 320, 20), 1, 0, 1, 20).expect("GridLayout.addChild");

        let formatting_buttons: Vec<compat::Shared<FormattingButton>> = vec![
            FormattingButton::new_str("B", ChatFormatting::Bold, Style::is_bold, events.clone()),
            FormattingButton::new_str("I", ChatFormatting::Italic, Style::is_italic, events.clone()),
            FormattingButton::new_str("U", ChatFormatting::Underline, Style::is_underlined, events.clone()),
            FormattingButton::new_str("S", ChatFormatting::Strikethrough, Style::is_strikethrough, events.clone()),
            FormattingButton::new_str("|||", ChatFormatting::Obfuscated, Style::is_obfuscated, events.clone()),
        ];
        CustomizeNameWidget::add_formatting_buttons(&formatting_buttons, &mut widgets, &mut grid, &events);

        // java: builder.add(grid.addChild(Button.builder(Component.translatable("skyblocker.customItemNames.screen.customColor"), _ -> client.gui.setScreen(ColorPopup.create(parent, color -> setStyle(Style.EMPTY.withColor(color))))).size(48, 16).build(), 2, 17, 1, 3));
        let e = compat::RenderThread::new(events.clone());
        let custom_color = button::builder(component::translatable("skyblocker.customItemNames.screen.customColor").get_string(), OnPress::Lambda(PressLambda::new(move |_screen, _index| e.get().borrow_mut().push(NameEvent::CustomColor)))).size(48, 16).build();
        let index = widgets.add_renderable_widget(custom_color);
        grid.add_child_span(LayoutElement::widget(index, 48, 16), 2, 17, 1, 3).expect("GridLayout.addChild");
        // java: builder.add(grid.addChild(Button.builder(Component.translatable("skyblocker.customItemNames.screen.gradientColor"), _ -> client.gui.setScreen(ColorPopup.createGradient(parent, this::createGradient))).size(48, 16).build(), 3, 17, 1, 3));
        let e = compat::RenderThread::new(events.clone());
        let gradient_color = button::builder(component::translatable("skyblocker.customItemNames.screen.gradientColor").get_string(), OnPress::Lambda(PressLambda::new(move |_screen, _index| e.get().borrow_mut().push(NameEvent::GradientColor)))).size(48, 16).build();
        let index = widgets.add_renderable_widget(gradient_color);
        grid.add_child_span(LayoutElement::widget(index, 48, 16), 3, 17, 1, 3).expect("GridLayout.addChild");
        // java: builder.add(grid.addChild(new StringWidget(20 * 16, textRenderer.lineHeight, Component.translatable("skyblocker.customItemNames.screen.howToRemove").withStyle(ChatFormatting.ITALIC, ChatFormatting.GRAY), textRenderer), 4, 0, 1, 20, LayoutSettings.defaults().paddingTop(2)));
        let how_to_remove = compat::string_widget(0, 0, 20 * 16, mcrust::net::minecraft::client::gui::font::LINE_HEIGHT, component::translatable("skyblocker.customItemNames.screen.howToRemove").with_style_formats(&[ChatFormatting::Italic, ChatFormatting::Gray]));
        let index = widgets.add_renderable_widget(how_to_remove);
        grid.add_child(LayoutElement::widget(index, 20 * 16, mcrust::net::minecraft::client::gui::font::LINE_HEIGHT), 4, 0, 1, 20, LayoutSettings::defaults().padding_top(2)).expect("GridLayout.addChild");
        // java: builder.add(previewWidget = grid.addChild(new StringWidget(20 * 16, textRenderer.lineHeight, Component.empty(), textRenderer).setMaxWidth(20 * 16, StringWidget.TextOverflow.SCROLLING), 5, 0, 1, 20, LayoutSettings.defaults().paddingVertical(2).alignHorizontallyCenter()));
        let preview = compat::string_widget(0, 0, 20 * 16, mcrust::net::minecraft::client::gui::font::LINE_HEIGHT, component::empty());
        let preview_widget = widgets.add_renderable_widget(preview);
        grid.add_child(LayoutElement::widget(preview_widget, 20 * 16, mcrust::net::minecraft::client::gui::font::LINE_HEIGHT), 5, 0, 1, 20, LayoutSettings::defaults().padding_vertical(2).align_horizontally_center()).expect("GridLayout.addChild");

        // java: grid.arrangeElements(); grid.setPosition(getX() + PADDING, getY() + PADDING); setSize(grid.getWidth() + PADDING * 2, grid.getHeight() + PADDING * 2);
        let mut grid = LayoutElement::Grid(grid);
        grid.arrange_elements();
        grid.set_position(base.get_x() + PADDING, base.get_y() + PADDING);
        let mut this = CustomizeNameWidget {
            base,
            uuid: String::new(),
            text: component::empty(),
            text_string: String::new(),
            text_field,
            preview_widget,
            formatting_buttons,
            widgets,
            grid,
            selection_start: 0,
            selection_end: 0,
            insert_as: None,
            events,
            self_ref: std::sync::Weak::new(),
        };
        this.place_children();
        let (w, h) = (this.grid.get_width() + PADDING * 2, this.grid.get_height() + PADDING * 2);
        this.base.set_size(w, h);
        // java: selectionStart = selectionEnd = textString.length();
        this.selection_end = utf16_len(&this.text_string);
        this.selection_start = this.selection_end;
        let this = compat::shared(this);
        compat::write(&this, |__t| __t.self_ref = Arc::downgrade(&this));
        this
    }

    /// java: private void addFormattingButtons(ImmutableList.Builder<AbstractWidget> builder) - "Makes it easier for Aaron Mod to add a chroma colour button"
    fn add_formatting_buttons(formatting_buttons: &[compat::Shared<FormattingButton>], widgets: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, grid: &mut GridLayout, events: &NameEvents) {
        for (i, button) in formatting_buttons.iter().enumerate() {
            let tooltip = compat::read(button, |__t| FormattedText::from((config_utils::FORMATTING_FORMATTER.get())(__t.format)));
            let mut widget = compat::object_widget(button.clone(), "");
            // java: setTooltip(Tooltip.create(ConfigUtils.FORMATTING_FORMATTER.apply(format)));
            widget.set_tooltip_text(tooltip);
            let index = widgets.add_renderable_widget(widget);
            grid.add_child_at(LayoutElement::widget(index, 16, 16), 0, i as i32).expect("GridLayout.addChild");
        }

        let mut color_button_index = 0;
        for formatting in mcrust::net::minecraft::chat_formatting::VALUES {
            if TextColor::from_legacy_format(formatting).is_some() {
                let button = ColorButton::new(formatting, events.clone());
                let tooltip = compat::read(&button, |__t| FormattedText::from(__t.message.clone()));
                let mut widget = compat::object_widget(button, "");
                // java: setTooltip(Tooltip.create(getMessage()));
                widget.set_tooltip_text(tooltip);
                let index = widgets.add_renderable_widget(widget);
                grid.add_child_at(LayoutElement::widget(index, 16, 16), 2, color_button_index).expect("GridLayout.addChild");
                color_button_index += 1;
            }
        }
    }

    /// The grid's positions written onto the children.
    fn place_children(&mut self) {
        let mut placed = Vec::new();
        self.grid.visit_widgets(&mut placed);
        for (index, x, y) in placed {
            self.widgets.widget_mut(index).set_position(x, y);
        }
    }

    /// java: public void setItem(ItemStack stack)
    pub fn set_item(&mut self, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack) {
        self.uuid = stack.get_uuid();
        self.set_text(compat::component_of(stack.get_hover_name()), false);
    }

    /// java: private void createGradient(int startColor, int endColor)
    pub fn create_gradient(&mut self, start_color: i32, end_color: i32) {
        let previous_selection_start = self.selection_start;
        let previous_selection_end = self.selection_end;

        let sel_start = self.selection_start.min(self.selection_end);
        let sel_size = (self.selection_end - self.selection_start).abs();
        if sel_size == 0 {
            return;
        }
        if sel_size == 1 {
            self.set_style(&Style::EMPTY.with_color(start_color));
        } else {
            for i in 0..sel_size {
                self.selection_start = sel_start + i;
                self.selection_end = sel_start + i + 1;
                let color = OkLabColor::interpolate(start_color, end_color, (i as f32) / ((sel_size - 1) as f32));
                self.set_style(&Style::EMPTY.with_color(color));
            }
        }
        self.selection_start = previous_selection_start;
        self.selection_end = previous_selection_end;
    }

    /// java: private void setStyle(Style style)
    fn set_style(&mut self, style: &Style) {
        if self.selection_start == self.selection_end {
            self.insert_as = Some(style.apply_to(self.insert_as.as_ref().unwrap_or(&Style::EMPTY)));
            return;
        }
        let set_style_visitor = SetStyleVisitor::new(style.clone(), self.selection_start, self.selection_end);
        visit(&self.text, &compat::view(&set_style_visitor, |a| a as compat::Shared<dyn BaseVisitor>));
        let new_text = SetStyleVisitor::get_new_text(&set_style_visitor);
        self.set_text_1(new_text);
    }

    /// java: private void updateStyleButtons()
    fn update_style_buttons(&mut self) {
        let style_visitor = GetStyleVisitor::new(self.selection_start, self.selection_end);
        visit(&self.text, &compat::view(&style_visitor, |a| a as compat::Shared<dyn BaseVisitor>));
        let style = GetStyleVisitor::get_style(&style_visitor);
        for button in self.formatting_buttons.iter() {
            compat::write(button, |__t| __t.update(&style));
        }
    }

    /// java: public void setText(Component text, boolean updateConfig)
    pub fn set_text(&mut self, text: MutableComponent, update_config: bool) {
        self.text = text;
        self.text_string = self.text.get_string();
        if update_config && !self.uuid.is_empty() {
            let (uuid, text_string, text) = (self.uuid.clone(), self.text_string.clone(), self.text.clone());
            SkyblockerConfigManager::update_only(Arc::new(move |config: &mut crate::de::hysky::skyblocker::config::skyblocker_config::SkyblockerConfig| {
                if compat::is_blank(&text_string) {
                    config.general.custom_item_names.remove(&uuid);
                } else {
                    config.general.custom_item_names.insert(uuid.clone(), text.clone().set_style(Style::EMPTY.with_italic(Some(false)).with_color_format(Some(ChatFormatting::White))));
                }
            }));
        }
        let preview = FormattedText::from(self.text.clone());
        self.widgets.widget_mut(self.preview_widget).set_message_text(preview);
        self.grid.arrange_elements();
        self.place_children();

        // called before init
        compat::write(&self.text_field, |__t| __t.update_me_pretty_please = true);
    }

    /// java: public void setText(Component text)
    pub fn set_text_1(&mut self, text: MutableComponent) {
        self.set_text(text, true);
    }

    /// java: public void insertText(String str)
    pub fn insert_text(&mut self, str: &str) {
        let str = mcrust::net::minecraft::util::string_util::filter_text(str);
        if self.text_string.is_empty() {
            let style = self.insert_as.clone().unwrap_or(Style::EMPTY);
            self.set_text_1(component::literal(&str).set_style(style));
        } else {
            let visitor = InsertTextVisitor::new(str.clone(), self.insert_as.clone(), self.selection_start, self.selection_end);
            visit(&self.text, &compat::view(&visitor, |a| a as compat::Shared<dyn BaseVisitor>));
            let new_text = InsertTextVisitor::get_new_text(&visitor);
            self.set_text_1(new_text);
            self.insert_as = None;
        }

        self.selection_start = self.selection_start.min(self.selection_end) + utf16_len(&str);
        self.selection_end = self.selection_start;
        self.update_style_buttons();
    }

    /// java: private void moveCursor(boolean left, boolean shiftHeld, boolean ctrlHeld)
    fn move_cursor(&mut self, left: bool, shift_held: bool, ctrl_held: bool) {
        if left && self.selection_start == 0 || (!left && self.selection_start == utf16_len(&self.text_string)) {
            return;
        }
        if ctrl_held {
            self.selection_start = self.get_word_skip_position(left);
        } else {
            let units: Vec<u16> = self.text_string.encode_utf16().collect();
            self.selection_start = mcrust::net::minecraft::util::util::offset_by_codepoints(&units, self.selection_start, if left { -1 } else { 1 });
        }
        if !shift_held {
            self.selection_end = self.selection_start;
        }
        self.insert_as = None;
        self.update_style_buttons();
    }

    /// java: private void erase(boolean left, boolean ctrlHeld)
    fn erase(&mut self, left: bool, ctrl_held: bool) {
        if self.selection_start != self.selection_end {
            self.insert_text("");
            return;
        }
        self.move_cursor(left, true, ctrl_held);
        self.insert_text("");
    }

    /// java: private int getWordSkipPosition(boolean left)
    fn get_word_skip_position(&self, left: bool) -> i32 {
        let units: Vec<u16> = self.text_string.encode_utf16().collect();
        let mut i: i32 = self.selection_start;
        if !left {
            let l: i32 = units.len() as i32;
            // java: i = this.textString.indexOf(32, i);
            i = units.iter().skip(i.max(0) as usize).position(|&c| c == 32).map(|p| p as i32 + i.max(0)).unwrap_or(-1);
            if i == -1 {
                i = l;
            } else {
                while i < l && units[i as usize] == ' ' as u16 {
                    i += 1;
                }
            }
        } else {
            while i > 0 && units[(i - 1) as usize] == ' ' as u16 {
                i -= 1;
            }
            while i > 0 && units[(i - 1) as usize] != ' ' as u16 {
                i -= 1;
            }
        }
        i
    }

    /// `textString.substring(selectionStart, selectionEnd)` over UTF-16 units, as Java's
    fn selected_substring(&self) -> String {
        let units: Vec<u16> = self.text.get_string().encode_utf16().collect();
        let (start, end) = (self.selection_start.max(0) as usize, self.selection_end.max(0) as usize);
        if start > end || end > units.len() {
            panic!("StringIndexOutOfBoundsException: begin {}, end {}, length {}", start, end, units.len());
        }
        String::from_utf16_lossy(&units[start..end])
    }

    /// java: TextField.getTextX()
    fn text_x(&self) -> i32 {
        self.widgets.widget(TEXT_FIELD).get_x() + 2
    }

    /// The children's call-backs recorded during the last call, run in order.
    fn run_events(&mut self) {
        loop {
            let events: Vec<NameEvent> = std::mem::take(&mut *self.events.borrow_mut());
            if events.is_empty() {
                return;
            }
            for event in events {
                match event {
                    NameEvent::SetStyle(style) => self.set_style(&style),
                    NameEvent::CustomColor => self.open_color_popup(false),
                    NameEvent::GradientColor => self.open_color_popup(true),
                    NameEvent::MoveCursor(left, shift, ctrl) => self.move_cursor(left, shift, ctrl),
                    NameEvent::Erase(left, ctrl) => self.erase(left, ctrl),
                    NameEvent::SelectAll => {
                        self.selection_start = 0;
                        self.selection_end = utf16_len(&self.text_string);
                        self.update_style_buttons();
                    }
                    NameEvent::Copy => {
                        let selected = self.selected_substring();
                        set_clipboard(&selected);
                    }
                    NameEvent::Paste => {
                        let clipboard = get_clipboard();
                        if !clipboard.is_empty() {
                            self.insert_text(&clipboard);
                        }
                    }
                    NameEvent::Cut => {
                        let selected = self.selected_substring();
                        set_clipboard(&selected);
                        self.insert_text("");
                    }
                    NameEvent::Insert(str) => self.insert_text(&str),
                    NameEvent::Click(x) => {
                        // java: TextField.onClick
                        let mut visitor = GetClickedPositionVisitor::new_1((x as i32) - self.text_x());
                        visit_clicked(&self.text, &mut visitor);
                        let position = visitor.get_position();
                        self.selection_end = if position < 0 { utf16_len(&self.text_string) } else { position };
                        self.selection_start = self.selection_end;
                        self.update_style_buttons();
                    }
                    NameEvent::Drag(x) => {
                        // java: TextField.onDrag
                        let mut visitor = GetClickedPositionVisitor::new_1((x as i32) - self.text_x());
                        visit_clicked(&self.text, &mut visitor);
                        let position = visitor.get_position();
                        self.selection_start = if position < 0 { utf16_len(&self.text_string) } else { position };
                        self.update_style_buttons();
                    }
                }
            }
        }
    }

    /// The colour buttons' `client.gui.setScreen(ColorPopup.create(parent, color -> setStyle(..)))` /
    /// `ColorPopup.createGradient(parent, this::createGradient)`.
    fn open_color_popup(&mut self, gradient: bool) {
        // rust: port gap (GAPS_batchA #19) - the popup base keeps its background as the game's `Screen` value and the
        //   port lends no handle to the live one; the screen that is up is held by the game (`set_screen_keeping_parent`)
        //   and an empty screen stands in for the base's field
        let parent = mcrust::net::minecraft::client::gui::screens::screen::Screen::new(String::new());
        let this = compat::RenderThread::new(self.self_ref.clone());
        let popup = if gradient {
            ColorPopup::create_gradient(
                parent,
                Arc::new(move |start: i32, end: i32| {
                    if let Some(this) = this.get().upgrade() {
                        compat::write(&this, |__t| __t.create_gradient(start, end));
                    }
                }),
            )
        } else {
            ColorPopup::create(
                parent,
                Arc::new(move |color: i32| {
                    if let Some(this) = this.get().upgrade() {
                        compat::write(&this, |__t| __t.set_style(&Style::EMPTY.with_color(color)));
                    }
                }),
            )
        };
        mcrust::mcrust::modding::game::set_screen_keeping_parent(Box::new(compat::ScreenAdapter::new(popup)));
    }

    /// java: TextField.extractWidgetRenderState - the field drawn from this widget's text and selection
    fn extract_text_field(&mut self, context: &mut WidgetRenderContext<'_>) {
        let field = self.widgets.widget(TEXT_FIELD);
        if !field.visible {
            return;
        }
        let (x, y, w, h, focused) = (field.get_x(), field.get_y(), field.get_width(), field.get_height(), field.is_focused());
        let (selection_start, selection_end) = (self.selection_start, self.selection_end);
        let text = self.text.clone();
        compat::write(&self.text_field, |tf| {
            if tf.rendered_selection_start != selection_start || tf.rendered_selection_end != selection_end || tf.update_me_pretty_please {
                tf.rendered_selection_start = selection_start;
                tf.rendered_selection_end = selection_end;
                tf.update_me_pretty_please = false;
                let get_render_width_visitor = GetRenderWidthVisitor::new_2(selection_start, selection_end);
                visit(&text, &compat::view(&get_render_width_visitor, |a| a as compat::Shared<dyn BaseVisitor>));
                let (render_start, render_end) = GetRenderWidthVisitor::get_widths(&get_render_width_visitor);
                tf.render_start = render_start;
                tf.render_end = render_end;
            }
        });
        let (render_start, render_end) = compat::read(&self.text_field, |tf| (tf.render_start, tf.render_end));

        let line_height = mcrust::net::minecraft::client::gui::font::LINE_HEIGHT;
        context.graphics.fill(context.gl, x, y, x + w, y + h, common_colors::BLACK);
        GuiHelper::border(&mut context.graphics, x, y, w, h, if focused { common_colors::WHITE } else { common_colors::GRAY });
        let text_x = x + 2;
        let text_y = y + (h - line_height) / 2;

        if render_start != render_end {
            context.graphics.fill(context.gl, text_x + render_start, text_y, text_x + render_end, text_y + line_height, common_colors::BLUE);
        }
        if focused {
            let caret = text_x + (if selection_start < selection_end { render_start } else { render_end }) - 1;
            gui_graphics_extractor::vertical_line(context.gl, context.minecraft, caret, text_y - 1, text_y + line_height, common_colors::WHITE);
        }

        // java: graphics.text(textRenderer, text, textX, textY, -1, false);
        gui_graphics_extractor::draw_formatted_text_no_shadow(context.gl, context.minecraft, text_x, text_y, &FormattedText::from(text), -1);

        // java: if (this.preeditOverlay != null) { .. graphics.setPreeditOverlay(this.preeditOverlay); }
        // rust: port gap - a mod widget is handed no `preeditUpdated` (IME composition), so `preeditOverlay` stays null
        //   (GAPS_batchE.md)
    }
}

/// `text.visit(visitor, Style.EMPTY)` for a `BaseVisitor` (whose `accept` answers `Optional.empty()`: keep going).
fn visit(text: &MutableComponent, visitor: &compat::Shared<dyn BaseVisitor>) {
    text.visit_styled(
        &mut |style: &Style, as_string: &str| {
            let accept = compat::dispatch(visitor, |__t| __t.accept());
            accept(visitor, style, as_string).is_none()
        },
        &Style::EMPTY,
    );
}

/// `text.visit(getClickedPositionVisitor, Style.EMPTY)`.
fn visit_clicked(text: &MutableComponent, visitor: &mut GetClickedPositionVisitor) {
    text.visit_styled(&mut |style: &Style, as_string: &str| visitor.accept(style, as_string).is_none(), &Style::EMPTY);
}

/// `String.length()` - UTF-16 units, as every index in this class is.
fn utf16_len(s: &str) -> i32 {
    s.encode_utf16().count() as i32
}

/// `client.keyboardHandler.setClipboard(value)`.
fn set_clipboard(value: &str) {
    if let Some(mut clipboard) = mcrust::net::minecraft::client::minecraft::GlfwClipboard::published() {
        clipboard.set_clipboard(value);
    }
}

/// `client.keyboardHandler.getClipboard()`.
fn get_clipboard() -> String {
    match mcrust::net::minecraft::client::minecraft::GlfwClipboard::published() {
        Some(mut clipboard) => clipboard.get_clipboard(),
        None => String::new(),
    }
}

impl compat::ObjectWidget for CustomizeNameWidget {
    fn object_box(&mut self) -> &mut AbstractContainerWidget {
        &mut self.base
    }

    fn object_box_ref(&self) -> &AbstractContainerWidget {
        &self.base
    }

    /// java: @Override public void setX(int x)
    fn set_x(&mut self, x: i32) {
        self.base.set_x(x);
        self.grid.set_x(self.base.get_x() + PADDING);
        self.place_children();
    }

    /// java: @Override public void setY(int y)
    fn set_y(&mut self, y: i32) {
        self.base.set_y(y);
        self.grid.set_y(self.base.get_y() + PADDING);
        self.place_children();
    }

    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float deltaTicks)
    fn extract_widget_render_state(&mut self, _widget: &AbstractWidget, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, _a: f32) {
        gui_graphics_extractor::blit_sprite_identifier(context.gl, context.minecraft, mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), INNER_SPACE_TEXTURE.get(), self.base.get_x(), self.base.get_y(), self.base.get_width(), self.base.get_height());
        if crate::de::hysky::skyblocker::debug::debug::Debug::debug_enabled() {
            let selection = format!("Selection Start: {}, Selection End: {}", self.selection_start, self.selection_end);
            gui_graphics_extractor::draw_text(context.gl, context.minecraft, self.base.get_x(), self.base.get_bottom(), &selection, -1);
            let insert = format!("Insert Style: {}", match &self.insert_as {
                None => "null".to_string(),
                Some(style) => format!("{style:?}"),
            });
            gui_graphics_extractor::draw_text(context.gl, context.minecraft, self.base.get_x(), self.base.get_bottom() + 10, &insert, -1);
        }
        // java: for (AbstractWidget widget : widgets) widget.extractRenderState(..); - the text field first
        self.extract_text_field(context);
        context.extract_widgets(&mut self.widgets, mouse_x, mouse_y);
    }

    /// AbstractContainerWidget.mouseClicked:62-66 over `children()`
    fn mouse_clicked(&mut self, _widget: &mut AbstractWidget, event: &MouseButtonEvent, _double_click: bool) -> bool {
        let consumed = compat::children_mouse_clicked(&mut self.widgets, event);
        self.run_events();
        consumed
    }

    /// ContainerEventHandler.mouseReleased:57-66
    fn mouse_released(&mut self, _widget: &mut AbstractWidget, event: &MouseButtonEvent) -> bool {
        compat::children_mouse_released(&mut self.widgets, event);
        self.run_events();
        false
    }

    /// ContainerEventHandler.mouseDragged:68-71
    fn mouse_dragged(&mut self, _widget: &mut AbstractWidget, event: &MouseButtonEvent, dx: f64, dy: f64) -> bool {
        compat::children_mouse_dragged(&mut self.widgets, event, dx, dy);
        self.run_events();
        false
    }

    /// ContainerEventHandler.keyPressed - the focused child
    fn key_pressed(&mut self, _widget: &mut AbstractWidget, event: &KeyEvent) -> bool {
        let answer = compat::children_key_pressed(&mut self.widgets, event);
        self.run_events();
        answer
    }

    /// java: @Override public boolean charTyped(CharacterEvent input)
    fn char_typed(&mut self, _widget: &mut AbstractWidget, event: &CharacterEvent) -> bool {
        let answer = compat::children_char_typed(&mut self.widgets, event);
        self.run_events();
        if answer || self.widgets.widget(TEXT_FIELD).is_focused() {
            return true;
        }
        // java: setFocused(textField); return textField.charTyped(input);
        self.widgets.set_focused(Some(TEXT_FIELD));
        let answer = compat::children_char_typed(&mut self.widgets, event);
        self.run_events();
        answer
    }

    /// java: @Override public void setFocused(boolean focused)
    fn set_focused(&mut self, widget: &mut AbstractWidget, focused: bool) {
        widget.set_focused(focused, mcrust::net::minecraft::client::input_type::InputType::None);
        if !focused {
            self.widgets.set_focused(None);
        }
    }
}

/// java: private class FormattingButton extends AbstractButton
pub struct FormattingButton {
    /// java: extends AbstractButton - AbstractWidget's fields (16 x 16)
    pub base: AbstractContainerWidget,
    /// AbstractWidget's `message` (`setMessage` changes its colour)
    pub message: MutableComponent,
    /// java: private boolean enabled;
    pub enabled: bool,
    /// java: private final ChatFormatting format;
    pub format: ChatFormatting,
    /// java: private final Predicate<Style> isEnabled;
    pub is_enabled: fn(&Style) -> bool,
    /// `CustomizeNameWidget.this` - its `setStyle(..)`, recorded (see `NameEvent`)
    events: NameEvents,
    look: compat::ButtonLook,
}

impl FormattingButton {
    /// java: protected FormattingButton(Component message, ChatFormatting format, Predicate<Style> isEnabled)
    fn new(message: MutableComponent, format: ChatFormatting, is_enabled: fn(&Style) -> bool, events: NameEvents) -> compat::Shared<Self> {
        // java: super(0, 0, 16, 16, message);
        compat::shared(FormattingButton { base: AbstractContainerWidget::new(0, 0, 16, 16, FormattedText::from(message.clone())), message, enabled: false, format, is_enabled, events, look: compat::ButtonLook::new() })
    }

    /// java: protected FormattingButton(String str, ChatFormatting format, Predicate<Style> isEnabled)
    fn new_str(str: &str, format: ChatFormatting, is_enabled: fn(&Style) -> bool, events: NameEvents) -> compat::Shared<Self> {
        FormattingButton::new(component::literal(str).with_style_format(format), format, is_enabled, events)
    }

    /// java: private void update(Style style)
    fn update(&mut self, style: &Style) {
        let enabled = (self.is_enabled)(style);
        self.set_enabled(enabled);
    }

    /// java: private void setEnabled(boolean enabled)
    fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
        // java: setMessage(getMessage().copy().withColor(enabled ? CommonColors.YELLOW : CommonColors.WHITE));
        self.message = self.message.clone().with_color(if enabled { common_colors::YELLOW } else { common_colors::WHITE });
    }

    /// java: @Override public void onPress(InputWithModifiers input)
    fn on_press(&mut self) {
        self.set_enabled(!self.enabled);
        let enabled = Some(self.enabled);
        let style = match self.format {
            ChatFormatting::Bold => Style::EMPTY.with_bold(enabled),
            ChatFormatting::Italic => Style::EMPTY.with_italic(enabled),
            ChatFormatting::Underline => Style::EMPTY.with_underlined(enabled),
            ChatFormatting::Strikethrough => Style::EMPTY.with_strikethrough(enabled),
            ChatFormatting::Obfuscated => Style::EMPTY.with_obfuscated(enabled),
            other => panic!("IllegalStateException: Unexpected value: {other:?}"),
        };
        self.events.borrow_mut().push(NameEvent::SetStyle(style));
    }
}

impl compat::ObjectWidget for FormattingButton {
    fn object_box(&mut self) -> &mut AbstractContainerWidget {
        &mut self.base
    }

    fn object_box_ref(&self) -> &AbstractContainerWidget {
        &self.base
    }

    /// java: @Override protected void extractContents(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_widget_render_state(&mut self, widget: &AbstractWidget, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, _a: f32) {
        // java: this.extractDefaultSprite(graphics); this.extractDefaultLabel(graphics.textRenderer());
        let message = FormattedText::from(self.message.clone());
        self.look.draw_with_message(widget, &message, context, mouse_x, mouse_y);
    }

    /// AbstractButton.onClick - `onPress(event)`
    fn on_click(&mut self, _widget: &mut AbstractWidget, _event: &MouseButtonEvent, _double_click: bool) {
        self.on_press();
    }

    /// AbstractButton.keyPressed - a selection key presses an active button
    fn key_pressed(&mut self, _widget: &mut AbstractWidget, event: &KeyEvent) -> bool {
        if !self.base.is_active() || !event.is_selection() {
            return false;
        }
        self.on_press();
        true
    }

    /// AbstractButton's `handleCursor(graphics)` - `AbstractWidget.handleCursor:68-72`
    fn handle_cursor(&self, widget: &AbstractWidget, cursors: &CursorTypes) -> Option<CursorType> {
        widget.handle_cursor(cursors)
    }
}

/// java: private class ColorButton extends AbstractButton
pub struct ColorButton {
    /// java: extends AbstractButton - AbstractWidget's fields (16 x 16)
    pub base: AbstractContainerWidget,
    /// AbstractWidget's `message` - `ConfigUtils.FORMATTING_FORMATTER.apply(format)`
    pub message: MutableComponent,
    /// java: private final ChatFormatting color;
    pub color: ChatFormatting,
    /// java: private final int intColor;
    pub int_color: i32,
    /// `CustomizeNameWidget.this` - its `setStyle(..)`, recorded (see `NameEvent`)
    events: NameEvents,
    look: compat::ButtonLook,
}

impl ColorButton {
    /// java: private ColorButton(ChatFormatting format)
    fn new(format: ChatFormatting, events: NameEvents) -> compat::Shared<Self> {
        // java: super(0, 0, 16, 16, ConfigUtils.FORMATTING_FORMATTER.apply(format));
        let message = (config_utils::FORMATTING_FORMATTER.get())(format);
        // java: this.intColor = ARGB.opaque(SkyBlockColors.fromVanilla(TextColor.fromLegacyFormat(color).getValue()));
        let value = TextColor::from_legacy_format(format).expect("NullPointerException: TextColor.fromLegacyFormat").get_value();
        let int_color = argb::opaque(SkyBlockColors::from_vanilla(value));
        compat::shared(ColorButton { base: AbstractContainerWidget::new(0, 0, 16, 16, FormattedText::from(message.clone())), message, color: format, int_color, events, look: compat::ButtonLook::new() })
    }

    /// java: @Override public void onPress(InputWithModifiers input)
    fn on_press(&mut self) {
        self.events.borrow_mut().push(NameEvent::SetStyle(Style::EMPTY.with_color_format(Some(self.color))));
    }
}

impl compat::ObjectWidget for ColorButton {
    fn object_box(&mut self) -> &mut AbstractContainerWidget {
        &mut self.base
    }

    fn object_box_ref(&self) -> &AbstractContainerWidget {
        &self.base
    }

    /// java: @Override public void extractContents(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_widget_render_state(&mut self, widget: &AbstractWidget, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, _a: f32) {
        // java: this.extractDefaultSprite(graphics);
        self.look.draw_sprite(widget, context, mouse_x, mouse_y);
        let (x, y) = (self.base.get_x(), self.base.get_y());
        context.graphics.fill(context.gl, x + 2, y + 2, self.base.get_right() - 2, self.base.get_bottom() - 2, self.int_color);
    }

    /// AbstractButton.onClick - `onPress(event)`
    fn on_click(&mut self, _widget: &mut AbstractWidget, _event: &MouseButtonEvent, _double_click: bool) {
        self.on_press();
    }

    /// AbstractButton.keyPressed - a selection key presses an active button
    fn key_pressed(&mut self, _widget: &mut AbstractWidget, event: &KeyEvent) -> bool {
        if !self.base.is_active() || !event.is_selection() {
            return false;
        }
        self.on_press();
        true
    }

    /// AbstractButton's `handleCursor(graphics)` - `AbstractWidget.handleCursor:68-72`
    fn handle_cursor(&self, widget: &AbstractWidget, cursors: &CursorTypes) -> Option<CursorType> {
        widget.handle_cursor(cursors)
    }
}

/// java: private class TextField extends AbstractWidget - "Used to capture inputs and render the text. Most logic is done
/// in the screen itself": its input goes to `CustomizeNameWidget` (`NameEvent`) and that widget draws it.
pub struct TextField {
    /// java: extends AbstractWidget - (320 x 20)
    pub base: AbstractContainerWidget,
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
    events: NameEvents,
}

impl TextField {
    /// java: private TextField()
    fn new(events: NameEvents) -> compat::Shared<Self> {
        // java: super(0, 0, 320, 20, Component.literal("TextField"));
        compat::shared(TextField { base: AbstractContainerWidget::new(0, 0, 320, 20, FormattedText::of("TextField")), rendered_selection_start: 0, rendered_selection_end: 0, update_me_pretty_please: false, render_start: 0, render_end: 0, events })
    }
}

impl compat::ObjectWidget for TextField {
    fn object_box(&mut self) -> &mut AbstractContainerWidget {
        &mut self.base
    }

    fn object_box_ref(&self) -> &AbstractContainerWidget {
        &self.base
    }

    /// java: @Override protected void extractWidgetRenderState(..) - drawn by `CustomizeNameWidget` (it reads that
    /// widget's text and selection)
    fn extract_widget_render_state(&mut self, _widget: &AbstractWidget, _context: &mut WidgetRenderContext<'_>, _mouse_x: i32, _mouse_y: i32, _a: f32) {}

    /// java: @Override protected void handleCursor(GuiGraphicsExtractor graphics) - `IBEAM` while hovered
    fn handle_cursor(&self, widget: &AbstractWidget, cursors: &CursorTypes) -> Option<CursorType> {
        if widget.is_hovered() { Some(cursors.ibeam) } else { None }
    }

    /// java: @Override public boolean keyPressed(KeyEvent input)
    fn key_pressed(&mut self, _widget: &mut AbstractWidget, event: &KeyEvent) -> bool {
        let mut events = self.events.borrow_mut();
        let captured = match event.key() {
            input_constants::KEY_LEFT => {
                events.push(NameEvent::MoveCursor(true, event.has_shift_down(), event.has_control_down_with_quirk()));
                true
            }
            input_constants::KEY_RIGHT => {
                events.push(NameEvent::MoveCursor(false, event.has_shift_down(), event.has_control_down_with_quirk()));
                true
            }
            input_constants::KEY_BACKSPACE => {
                events.push(NameEvent::Erase(true, event.has_control_down_with_quirk()));
                true
            }
            input_constants::KEY_DELETE => {
                events.push(NameEvent::Erase(false, event.has_control_down_with_quirk()));
                true
            }
            _ => false,
        };
        if captured {
            return true;
        }
        if event.is_select_all() {
            events.push(NameEvent::SelectAll);
            true
        } else if event.is_copy() {
            events.push(NameEvent::Copy);
            true
        } else if event.is_paste() {
            events.push(NameEvent::Paste);
            true
        } else if event.is_cut() {
            events.push(NameEvent::Cut);
            true
        } else {
            false
        }
    }

    /// java: @Override public boolean charTyped(CharacterEvent input)
    fn char_typed(&mut self, _widget: &mut AbstractWidget, event: &CharacterEvent) -> bool {
        if !self.base.active {
            false
        } else if event.is_allowed_chat_character() {
            self.events.borrow_mut().push(NameEvent::Insert(event.codepoint_as_string()));
            true
        } else {
            false
        }
    }

    /// java: @Override public void onClick(MouseButtonEvent click, boolean doubled)
    fn on_click(&mut self, _widget: &mut AbstractWidget, event: &MouseButtonEvent, _double_click: bool) {
        self.events.borrow_mut().push(NameEvent::Click(event.x()));
    }

    /// java: @Override protected void onDrag(MouseButtonEvent click, double offsetX, double offsetY)
    fn on_drag(&mut self, _widget: &mut AbstractWidget, event: &MouseButtonEvent, _dx: f64, _dy: f64) {
        self.events.borrow_mut().push(NameEvent::Drag(event.x()));
    }

    /// java: @Override public void setFocused(final boolean focused)
    fn set_focused(&mut self, widget: &mut AbstractWidget, focused: bool) {
        widget.set_focused(focused, mcrust::net::minecraft::client::input_type::InputType::None);
        // java: if (focused) Minecraft.getInstance().onTextInputFocusChange(this, focused); - "Required for IME support"
        // rust: port gap - a mod widget has no IME hook in the port (see `extract_text_field`)
    }

    /// java: @Override public void playDownSound(SoundManager soundManager) - empty
    fn play_down_sound(&self) -> Option<&'static mcrust::net::minecraft::sounds::sound_event::SoundEvent> {
        None
    }
}
