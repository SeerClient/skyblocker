//! Ported from de.hysky.skyblocker.skyblock.item.custom.screen.ColorSelectionWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::item::custom::custom_armor_animated_dyes::AnimatedDye;
use crate::de::hysky::skyblocker::skyblock::item::custom::custom_armor_animated_dyes::Keyframe;
use crate::de::hysky::skyblocker::skyblock::item::custom::screen::animated_dye_timeline_widget::AnimatedDyeTimelineWidget;
use crate::de::hysky::skyblocker::skyblock::item::custom::screen::dye_select_popup::DyeSelectPopup;
use crate::de::hysky::skyblocker::utils::formatters;
use crate::de::hysky::skyblocker::utils::render::gui::argb_text_input::ARGBTextInput;
use crate::de::hysky::skyblocker::utils::render::gui::color_picker_widget::ColorPickerWidget;
use mcrust::com::mojang::blaze3d::platform::cursor::cursor_type::CursorType;
use mcrust::com::mojang::blaze3d::platform::cursor::cursor_types::CursorTypes;
use mcrust::mcrust::modding::widgets::WidgetRenderContext;
use mcrust::net::minecraft::client::gui::components::abstract_container_widget::AbstractContainerWidget;
use mcrust::net::minecraft::client::gui::components::abstract_scroll_area::ScrollbarSettings;
use mcrust::net::minecraft::client::gui::components::abstract_slider_button;
use mcrust::net::minecraft::client::gui::components::abstract_slider_button::SliderState;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::abstract_widget::PressLambda;
use mcrust::net::minecraft::client::gui::components::button;
use mcrust::net::minecraft::client::gui::font::Font;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::layouts::frame_layout::FrameLayout;
use mcrust::net::minecraft::client::gui::layouts::grid_layout::GridLayout;
use mcrust::net::minecraft::client::gui::layouts::layout_element::LayoutElement;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::key_event::KeyEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::client::resources::model::equipment_client_info::LayerType;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::formatted_text::FormattedText;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::resources::resource_key::ResourceKey;
use mcrust::net::minecraft::util::argb;
use mcrust::net::minecraft::world::item::component::dyed_item_color;
use mcrust::net::minecraft::world::item::equipment::equippable::Equippable;
use crate::de::hysky::skyblocker::injected::skyblocker_stack::SkyblockerStack as _;
use mcrust::net::minecraft::core::component::data_component_map::DataComponentGetter as _;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final int PADDING = 3;
pub const PADDING: i32 = 3;
/// java: private static final Identifier INNER_SPACE_TEXTURE = SkyblockerMod.id("menu_inner_space");
pub static INNER_SPACE_TEXTURE: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("menu_inner_space"));
/// java: private static final Component RESET_COLOR_TEXT = Component.translatable("skyblocker.customization.armor.resetColor");
pub static RESET_COLOR_TEXT: compat::MainThread<MutableComponent> = compat::MainThread::new(|| component::translatable("skyblocker.customization.armor.resetColor"));
/// java: private static final Component PICK_DYE_COLOR_TEXT = Component.translatable("skyblocker.customization.armor.pickDye");
pub static PICK_DYE_COLOR_TEXT: compat::MainThread<MutableComponent> = compat::MainThread::new(|| component::translatable("skyblocker.customization.armor.pickDye"));
/// java: private static final Component CANNOT_CUSTOMIZE_COLOR_TEXT = Component.translatable("skyblocker.customization.armor.cannotCustomizeColor");
pub static CANNOT_CUSTOMIZE_COLOR_TEXT: compat::MainThread<MutableComponent> = compat::MainThread::new(|| component::translatable("skyblocker.customization.armor.cannotCustomizeColor"));
/// java: private static final Component ANIMATED_TEXT = Component.translatable("skyblocker.customization.armor.animated");
pub static ANIMATED_TEXT: compat::MainThread<MutableComponent> = compat::MainThread::new(|| component::translatable("skyblocker.customization.armor.animated"));
/// java: private static final Component CYCLE_BACK_TEXT = Component.translatable("skyblocker.customization.armor.cycleBack");
pub static CYCLE_BACK_TEXT: compat::MainThread<MutableComponent> = compat::MainThread::new(|| component::translatable("skyblocker.customization.armor.cycleBack"));
/// java: private static final Component DURATION_TOOLTIP_TEXT = Component.translatable("skyblocker.customization.armor.durationTooltip");
pub static DURATION_TOOLTIP_TEXT: compat::MainThread<MutableComponent> = compat::MainThread::new(|| component::translatable("skyblocker.customization.armor.durationTooltip"));
/// java: private static final Component DELAY_TOOLTIP_TEXT = Component.translatable("skyblocker.customization.armor.delayTooltip");
pub static DELAY_TOOLTIP_TEXT: compat::MainThread<MutableComponent> = compat::MainThread::new(|| component::translatable("skyblocker.customization.armor.delayTooltip"));
/// java: private static final String DURATION_TEXT = "skyblocker.customization.armor.duration";
pub const DURATION_TEXT: &str = "skyblocker.customization.armor.duration";
/// java: private static final String DELAY_TEXT = "skyblocker.customization.armor.delay";
pub const DELAY_TEXT: &str = "skyblocker.customization.armor.delay";

/// The `children` list's order (Java's `List.of(colorPicker, argbTextInput, timelineWidget, resetColorButton,
/// pickDyeButton, animatedCheckbox, notCustomizableText, cycleBackCheckbox, delaySlider, durationSlider)`): each
/// child's index in `children`.
const COLOR_PICKER: usize = 0;
const ARGB_TEXT_INPUT: usize = 1;
const TIMELINE: usize = 2;
const RESET_COLOR_BUTTON: usize = 3;
const PICK_DYE_BUTTON: usize = 4;
const ANIMATED_CHECKBOX: usize = 5;
const NOT_CUSTOMIZABLE_TEXT: usize = 6;
const CYCLE_BACK_CHECKBOX: usize = 7;
const DELAY_SLIDER: usize = 8;
const DURATION_SLIDER: usize = 9;

/// What the children call back with (`this::onPickerColorChanged`, the buttons' and checkboxes' handlers, the
/// sliders' consumers). They call back while this widget is handing them input (or calling into them), so each call
/// is recorded and run once that call returns (`run_events`), in order.
enum ColorEvent {
    PickerColorChanged(i32, bool),
    TextInputColorChanged(i32),
    TimelineFrameSelected(i32, f32),
    RemoveCustomColor,
    ClickPickDye,
    AnimatedCheckbox(bool),
    CycleBackCheckbox(bool),
    Delay(f32),
    Duration(f32),
}

type ColorEvents = std::rc::Rc<std::cell::RefCell<Vec<ColorEvent>>>;

/// java: public class ColorSelectionWidget extends AbstractContainerWidget implements Closeable
pub struct ColorSelectionWidget {
    /// java: extends net.minecraft.client.gui.components.AbstractContainerWidget
    pub base: AbstractContainerWidget,
    /// java: private final ColorPickerWidget colorPicker;
    pub color_picker: compat::Shared<ColorPickerWidget>,
    /// java: private final ARGBTextInput argbTextInput;
    pub argb_text_input: compat::Shared<ARGBTextInput>,
    /// java: private final AnimatedDyeTimelineWidget timelineWidget;
    pub timeline_widget: compat::Shared<AnimatedDyeTimelineWidget>,
    /// java: private final Slider delaySlider;
    pub delay_slider: compat::Shared<Slider>,
    /// java: private final Slider durationSlider;
    pub duration_slider: compat::Shared<Slider>,
    /// java: private final List<AbstractWidget> children; - every widget above and `cycleBackCheckbox`,
    /// `resetColorButton`, `pickDyeButton`, `animatedCheckbox`, `notCustomizableText`, as the game's widgets (see the
    /// index constants)
    pub children: mcrust::net::minecraft::client::gui::screens::screen::Screen,
    /// java: private final FrameLayout layout; - its minimum size (`new FrameLayout(w, h)`); the layout itself is built
    /// from the children's live sizes each time it is arranged (`arrange`)
    pub layout_min: (i32, i32),
    /// the arranged layout
    pub layout: LayoutElement,
    /// java: private ItemStack currentItem;
    pub current_item: Option<mcrust::net::minecraft::world::item::item_stack::ItemStack>,
    /// java: private boolean animated;
    pub animated: bool,
    /// java: private boolean customizable = false;
    pub customizable: bool,
    /// the children's call-backs during this widget's own calls (see `ColorEvent`)
    events: ColorEvents,
    /// Java's `this`, for the dye popup's call-backs
    self_ref: compat::WeakShared<Self>,
}

impl ColorSelectionWidget {
    /// java: public ColorSelectionWidget(int x, int y, int width, int height, Font textRenderer)
    pub fn new(x: i32, y: i32, width: i32, height: i32, text_renderer: &Font) -> compat::Shared<Self> {
        // java: super(x, y, width, height, Component.nullToEmpty("ColorSelectionWidget"), AbstractScrollArea.defaultSettings(4));
        let base = AbstractContainerWidget::new_with_scrollbar_settings(x, y, width, height, FormattedText::of("ColorSelectionWidget"), ScrollbarSettings::default_settings(4.0));
        let events: ColorEvents = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let height1: i32 = ((2 * height / 3).min(width / 5)).min(height - 40); // 40 is the height of slider + timeline + some padding/margin

        // java: colorPicker = new ColorPickerWidget(0, 0, height1 * 2, height1); colorPicker.setOnColorChange(this::onPickerColorChanged);
        let color_picker = ColorPickerWidget::new(0, 0, height1 * 2, height1);
        let e = events.clone();
        ColorPickerWidget::set_on_color_change(&color_picker, Arc::new(move |argb: i32, release: bool| e.borrow_mut().push(ColorEvent::PickerColorChanged(argb, release))));
        // java: argbTextInput = new ARGBTextInput(0, 0, textRenderer, true); argbTextInput.setOnChange(this::onTextInputColorChanged);
        // (the port's `Font` is a borrowed view of the game's one font, which `compat::font()` hands out again)
        let _ = text_renderer;
        let argb_text_input = ARGBTextInput::new_4(0, 0, compat::font(), true);
        let e = events.clone();
        ARGBTextInput::set_on_change(&argb_text_input, Arc::new(move |argb: i32| e.borrow_mut().push(ColorEvent::TextInputColorChanged(argb))));
        // java: timelineWidget = new AnimatedDyeTimelineWidget(0, 0, getWidth() - 6, 15, this::onTimelineFrameSelected);
        let e = events.clone();
        let timeline_widget = AnimatedDyeTimelineWidget::new(0, 0, base.get_width() - 6, 15, Arc::new(move |color: i32, time: f32| e.borrow_mut().push(ColorEvent::TimelineFrameSelected(color, time))));

        // java: resetColorButton = Button.builder(RESET_COLOR_TEXT, this::onRemoveCustomColor).width(75).build();
        let e = compat::RenderThread::new(events.clone());
        let reset_color_button = button::builder(RESET_COLOR_TEXT.get().get_string(), OnPress::Lambda(PressLambda::new(move |_screen, _index| e.get().borrow_mut().push(ColorEvent::RemoveCustomColor)))).width(75).build();
        // java: pickDyeButton = Button.builder(PICK_DYE_COLOR_TEXT, this::onClickPickDye).width(75).build();
        let e = compat::RenderThread::new(events.clone());
        let pick_dye_button = button::builder(PICK_DYE_COLOR_TEXT.get().get_string(), OnPress::Lambda(PressLambda::new(move |_screen, _index| e.get().borrow_mut().push(ColorEvent::ClickPickDye)))).width(75).build();

        // java: notCustomizableText = new StringWidget(CANNOT_CUSTOMIZE_COLOR_TEXT, textRenderer);
        let mut not_customizable_text = compat::string_widget(0, 0, -1, 9, CANNOT_CUSTOMIZE_COLOR_TEXT.get().clone());
        // java: FrameLayout.centerInRectangle(notCustomizableText, getX(), getY(), getWidth(), getHeight());
        let mut centered = LayoutElement::widget(0, not_customizable_text.get_width(), not_customizable_text.get_height());
        mcrust::net::minecraft::client::gui::layouts::frame_layout::center_in_rectangle(&mut centered, base.get_x(), base.get_y(), base.get_width(), base.get_height());
        not_customizable_text.set_position(centered.get_x(), centered.get_y());

        // java: animatedCheckbox = Checkbox.builder(ANIMATED_TEXT, textRenderer).pos(colorPicker.getRight() + 5, resetColorButton.getBottom()).maxWidth(80).onValueChange(this::onAnimatedCheckbox).build();
        let picker_right = compat::read(&color_picker, |__t| __t.base.get_right());
        let e = compat::RenderThread::new(events.clone());
        let animated_checkbox = compat::checkbox(FormattedText::from(ANIMATED_TEXT.get().clone()), picker_right + 5, reset_color_button.get_bottom(), Some(80), false, move |checked| e.get().borrow_mut().push(ColorEvent::AnimatedCheckbox(checked)));
        // java: cycleBackCheckbox = Checkbox.builder(CYCLE_BACK_TEXT, textRenderer).pos(colorPicker.getRight() + 5, animatedCheckbox.getBottom() + 3).maxWidth(80).onValueChange(this::onCycleBackCheckbox).build();
        let e = compat::RenderThread::new(events.clone());
        let cycle_back_checkbox = compat::checkbox(FormattedText::from(CYCLE_BACK_TEXT.get().clone()), picker_right + 5, animated_checkbox.get_bottom() + 3, Some(80), false, move |checked| e.get().borrow_mut().push(ColorEvent::CycleBackCheckbox(checked)));

        let slider_width: i32 = ((width as f32) * 0.35f32) as i32;
        // java: delaySlider = new Slider(0, 0, sliderWidth, 0.0f, 2.0f, 0.02f, true, DELAY_TEXT, f -> {..});
        let e = events.clone();
        let delay_slider = Slider::new(0, 0, slider_width, 0.0, 2.0, 0.02, true, DELAY_TEXT, Arc::new(move |f: f32| e.borrow_mut().push(ColorEvent::Delay(f))));
        // java: durationSlider = new Slider(0, 0, sliderWidth, 0.1f, 10.0f, 0.1f, true, DURATION_TEXT, f -> {..});
        let e = events.clone();
        let duration_slider = Slider::new(0, 0, slider_width, 0.1, 10.0, 0.1, true, DURATION_TEXT, Arc::new(move |f: f32| e.borrow_mut().push(ColorEvent::Duration(f))));

        // java: children = List.of(colorPicker, argbTextInput, timelineWidget, resetColorButton, pickDyeButton, animatedCheckbox, notCustomizableText, cycleBackCheckbox, delaySlider, durationSlider);
        let mut children = mcrust::net::minecraft::client::gui::screens::screen::Screen::new(String::new());
        children.add_renderable_widget(compat::WidgetAdapter::widget(color_picker.clone()));
        children.add_renderable_widget(compat::WidgetAdapter::widget(argb_text_input.clone()));
        children.add_renderable_widget(compat::object_widget(timeline_widget.clone(), "Animated Dye Timeline"));
        children.add_renderable_widget(reset_color_button);
        children.add_renderable_widget(pick_dye_button);
        children.add_renderable_widget(animated_checkbox);
        children.add_renderable_widget(not_customizable_text);
        children.add_renderable_widget(cycle_back_checkbox);
        let mut delay_widget = compat::object_widget(delay_slider.clone(), String::new());
        // java: delaySlider.setTooltip(Tooltip.create(DELAY_TOOLTIP_TEXT));
        delay_widget.set_tooltip_text(FormattedText::from(DELAY_TOOLTIP_TEXT.get().clone()));
        children.add_renderable_widget(delay_widget);
        let mut duration_widget = compat::object_widget(duration_slider.clone(), String::new());
        // java: durationSlider.setTooltip(Tooltip.create(DURATION_TOOLTIP_TEXT));
        duration_widget.set_tooltip_text(FormattedText::from(DURATION_TOOLTIP_TEXT.get().clone()));
        children.add_renderable_widget(duration_widget);

        // java: int w = getWidth() - PADDING * 2; int h = getHeight() - PADDING * 2; layout = new FrameLayout(w, h);
        let w = base.get_width() - PADDING * 2;
        let h = base.get_height() - PADDING * 2;
        let this = compat::shared(ColorSelectionWidget {
            base,
            color_picker,
            argb_text_input,
            timeline_widget,
            delay_slider,
            duration_slider,
            children,
            layout_min: (w, h),
            layout: LayoutElement::Frame(FrameLayout::of(w, h)),
            current_item: None,
            animated: false,
            customizable: false,
            events,
            self_ref: std::sync::Weak::new(),
        });
        compat::write(&this, |__t| {
            __t.self_ref = Arc::downgrade(&this);
            __t.update_widget_dimensions();
        });
        this
    }

    /// The child at `index` as a layout leaf over its live size.
    fn leaf(&self, index: usize) -> LayoutElement {
        let widget = self.children.widget(index);
        LayoutElement::widget(index, widget.get_width(), widget.get_height())
    }

    /// The constructor's layout (`layout.addChild(timelineWidget, ..)`, the grid, `notCustomizableText`), over the
    /// children's sizes now (the port's layout leaves keep the size they were given, Java's read the widget's).
    fn build_layout(&self) -> LayoutElement {
        let mut layout = FrameLayout::of(self.layout_min.0, self.layout_min.1);
        // java: layout.addChild(timelineWidget, LayoutSettings::alignVerticallyBottom);
        layout.add_child_adjusted(self.leaf(TIMELINE), &mut |s| *s = s.clone().align_vertically_bottom());

        // java: GridLayout grid = new GridLayout().spacing(3);
        let mut grid = GridLayout::new_default();
        grid.spacing(3);
        grid.add_child_at(self.leaf(ARGB_TEXT_INPUT), 0, 1).expect("GridLayout.addChild");
        grid.add_child_span_adjusted(self.leaf(RESET_COLOR_BUTTON), 0, 1, 1, 3, &mut |s| *s = s.clone().align_horizontally_right()).expect("GridLayout.addChild");
        grid.add_child_span_adjusted(self.leaf(PICK_DYE_BUTTON), 0, 2, 1, 4, &mut |s| *s = s.clone().align_horizontally_right()).expect("GridLayout.addChild");
        grid.add_child_span(self.leaf(ANIMATED_CHECKBOX), 1, 1, 1, 2).expect("GridLayout.addChild");
        grid.add_child_span_adjusted(self.leaf(DELAY_SLIDER), 1, 3, 1, 2, &mut |s| *s = s.clone().align_horizontally_right()).expect("GridLayout.addChild");
        grid.add_child_span(self.leaf(CYCLE_BACK_CHECKBOX), 2, 1, 1, 2).expect("GridLayout.addChild");
        grid.add_child_span_adjusted(self.leaf(DURATION_SLIDER), 2, 3, 1, 2, &mut |s| *s = s.clone().align_horizontally_right()).expect("GridLayout.addChild");
        grid.add_child_span(self.leaf(COLOR_PICKER), 0, 0, 3, 1).expect("GridLayout.addChild");
        // java: layout.addChild(grid, LayoutSettings::alignVerticallyTop);
        layout.add_child_adjusted(LayoutElement::Grid(grid), &mut |s| *s = s.clone().align_vertically_top());
        // java: layout.addChild(notCustomizableText);
        layout.add_child(self.leaf(NOT_CUSTOMIZABLE_TEXT));
        LayoutElement::Frame(layout)
    }

    /// The layout's positions written onto the children - through `setX` / `setY` for the ones that override them
    /// (Java's layouts call those on the widget).
    fn place_children(&mut self) {
        let mut placed = Vec::new();
        self.layout.visit_widgets(&mut placed);
        for (index, x, y) in placed {
            self.children.widget_mut(index).set_position(x, y);
            if index == COLOR_PICKER {
                ColorPickerWidget::set_x(&self.color_picker, x);
                ColorPickerWidget::set_y(&self.color_picker, y);
            }
        }
    }

    /// java: private void updateWidgetDimensions()
    fn update_widget_dimensions(&mut self) {
        let w = self.base.get_width() - PADDING * 2;
        let h = self.base.get_height() - PADDING * 2;
        // java: timelineWidget.setWidth(w);
        compat::write(&self.timeline_widget, |__t| compat::ObjectWidget::set_width(__t, w));
        compat::push_object_box(&self.timeline_widget, self.children.widget_mut(TIMELINE));
        // java: colorPicker.setHeight(Math.min(h - timelineWidget.getHeight() - 5, w / 3 / 2)); colorPicker.setWidth(colorPicker.getHeight() * 2);
        let timeline_height = compat::read(&self.timeline_widget, |__t| __t.base.get_height());
        ColorPickerWidget::set_height(&self.color_picker, (h - timeline_height - 5).min(w / 3 / 2));
        let picker_height = compat::read(&self.color_picker, |__t| __t.base.get_height());
        ColorPickerWidget::set_width(&self.color_picker, picker_height * 2);
        self.children.widget_mut(COLOR_PICKER).set_size(picker_height * 2, picker_height);
        // java: delaySlider.setWidth((int) (w * 0.35f)); durationSlider.setWidth((int) (w * 0.35f));
        let slider_width = ((w as f32) * 0.35f32) as i32;
        compat::write(&self.delay_slider, |__t| compat::ObjectWidget::set_width(__t, slider_width));
        compat::push_object_box(&self.delay_slider, self.children.widget_mut(DELAY_SLIDER));
        compat::write(&self.duration_slider, |__t| compat::ObjectWidget::set_width(__t, slider_width));
        compat::push_object_box(&self.duration_slider, self.children.widget_mut(DURATION_SLIDER));
        // java: layout.arrangeElements(); layout.setPosition(getX() + PADDING, getY() + PADDING);
        self.layout = self.build_layout();
        self.layout.arrange_elements();
        self.layout.set_position(self.base.get_x() + PADDING, self.base.get_y() + PADDING);
        self.place_children();
        // java: width = layout.getWidth() + PADDING * 2; height = layout.getHeight() + PADDING * 2;
        self.base.width = self.layout.get_width() + PADDING * 2;
        self.base.height = self.layout.get_height() + PADDING * 2;
    }

    /// java: public AnimatedDyeTimelineWidget getTimelineWidget()
    pub fn get_timeline_widget(&self) -> compat::Shared<AnimatedDyeTimelineWidget> {
        return self.timeline_widget.clone();
    }

    /// The call-backs recorded during the last call, run in order.
    fn run_events(&mut self) {
        loop {
            let events: Vec<ColorEvent> = std::mem::take(&mut *self.events.borrow_mut());
            if events.is_empty() {
                return;
            }
            for event in events {
                match event {
                    ColorEvent::PickerColorChanged(argb, release) => self.on_picker_color_changed(argb, release),
                    ColorEvent::TextInputColorChanged(argb) => self.on_text_input_color_changed(argb),
                    ColorEvent::TimelineFrameSelected(color, time) => self.on_timeline_frame_selected(color, time),
                    ColorEvent::RemoveCustomColor => self.on_remove_custom_color(),
                    ColorEvent::ClickPickDye => self.on_click_pick_dye(),
                    ColorEvent::AnimatedCheckbox(checked) => self.on_animated_checkbox(checked),
                    ColorEvent::CycleBackCheckbox(checked) => self.on_cycle_back_checkbox(checked),
                    ColorEvent::Delay(f) => self.on_delay_changed(f),
                    ColorEvent::Duration(f) => self.on_duration_changed(f),
                }
            }
        }
    }

    /// `currentItem.getUuid()`
    fn item_uuid(&self) -> String {
        self.current_item.as_ref().expect("NullPointerException: ColorSelectionWidget.currentItem").get_uuid()
    }

    /// `DyedItemColor.getOrDefault(currentItem, -1)`
    fn dyed_color_or_default(&self) -> i32 {
        let stack = self.current_item.as_ref().expect("NullPointerException: ColorSelectionWidget.currentItem");
        let dyed = stack.get(mcrust::net::minecraft::core::component::data_components::dyed_color()).and_then(|__value| mcrust::com::mojang::serialization::cast::<dyed_item_color::DyedItemColor>(&__value).copied());
        dyed_item_color::get_or_default(dyed.as_ref(), -1)
    }

    /// java: private void onPickerColorChanged(int argb, boolean release)
    fn on_picker_color_changed(&mut self, argb: i32, release: bool) {
        ARGBTextInput::set_argb_color(&self.argb_text_input, argb);
        if !self.animated {
            let uuid = self.item_uuid();
            SkyblockerConfigManager::update_only(Arc::new(move |config: &mut crate::de::hysky::skyblocker::config::skyblocker_config::SkyblockerConfig| {
                config.general.custom_dye_colors.insert(uuid.clone(), argb::opaque(argb));
            }));
        } else if release {
            compat::write(&self.timeline_widget, |__t| __t.set_color(argb));
        }
    }

    /// java: private void onTextInputColorChanged(int argb)
    fn on_text_input_color_changed(&mut self, argb: i32) {
        ColorPickerWidget::set_argb_color(&self.color_picker, argb);
        if self.animated {
            compat::write(&self.timeline_widget, |__t| __t.set_color(argb));
        } else {
            let uuid = self.item_uuid();
            SkyblockerConfigManager::update_only(Arc::new(move |config: &mut crate::de::hysky::skyblocker::config::skyblocker_config::SkyblockerConfig| {
                config.general.custom_dye_colors.insert(uuid.clone(), argb::opaque(argb));
            }));
        }
    }

    /// java: private void onTimelineFrameSelected(int color, float time)
    fn on_timeline_frame_selected(&mut self, color: i32, _time: f32) {
        ARGBTextInput::set_argb_color(&self.argb_text_input, color);
        ColorPickerWidget::set_argb_color(&self.color_picker, color);
    }

    /// java: private void onRemoveCustomColor(Button button)
    fn on_remove_custom_color(&mut self) {
        self.animated = false;
        compat::set_checkbox_selected(self.children.widget_mut(ANIMATED_CHECKBOX), false);
        self.change_visibilities();

        let item_uuid = self.item_uuid();
        let uuid = item_uuid.clone();
        SkyblockerConfigManager::update_only(Arc::new(move |config: &mut crate::de::hysky::skyblocker::config::skyblocker_config::SkyblockerConfig| {
            config.general.custom_dye_colors.remove(&uuid);
        }));
        let uuid = item_uuid;
        SkyblockerConfigManager::update_only(Arc::new(move |config: &mut crate::de::hysky::skyblocker::config::skyblocker_config::SkyblockerConfig| {
            config.general.custom_animated_dyes.remove(&uuid);
        }));

        let color: i32 = self.dyed_color_or_default();
        ARGBTextInput::set_argb_color(&self.argb_text_input, color);
        ColorPickerWidget::set_argb_color(&self.color_picker, color);
    }

    /// java: private void onAnimatedCheckbox(Checkbox checkbox, boolean checked)
    fn on_animated_checkbox(&mut self, checked: bool) {
        self.animated = checked;
        self.change_visibilities();
        let item_uuid = self.item_uuid();
        if self.animated {
            let uuid = item_uuid.clone();
            SkyblockerConfigManager::update_only(Arc::new(move |config: &mut crate::de::hysky::skyblocker::config::skyblocker_config::SkyblockerConfig| {
                config.general.custom_animated_dyes.insert(
                    uuid.clone(),
                    AnimatedDye::new(vec![Keyframe::new(mcrust::net::minecraft::util::common_colors::RED, 0.0), Keyframe::new(mcrust::net::minecraft::util::common_colors::BLUE, 1.0)], true, 0.0, 1.0f32),
                );
            }));
            compat::write(&self.timeline_widget, |__t| __t.set_animated_dye(&item_uuid));
            compat::write(&self.delay_slider, |__t| __t.set_value(0.0));
            compat::write(&self.duration_slider, |__t| __t.set_value(1.0));
            compat::set_checkbox_selected(self.children.widget_mut(CYCLE_BACK_CHECKBOX), true);
        } else {
            let color: i32 = SkyblockerConfigManager::get().general.custom_dye_colors.get(&item_uuid).copied().unwrap_or_else(|| self.dyed_color_or_default());
            ColorPickerWidget::set_argb_color(&self.color_picker, color);
            ARGBTextInput::set_argb_color(&self.argb_text_input, color);
            SkyblockerConfigManager::update_only(Arc::new(move |config: &mut crate::de::hysky::skyblocker::config::skyblocker_config::SkyblockerConfig| {
                config.general.custom_animated_dyes.remove(&item_uuid);
            }));
        }
    }

    /// java: private void onCycleBackCheckbox(Checkbox checkbox, boolean checked)
    fn on_cycle_back_checkbox(&mut self, checked: bool) {
        let item_uuid = self.item_uuid();
        let dye: AnimatedDye = SkyblockerConfigManager::get().general.custom_animated_dyes.get(&item_uuid).cloned().expect("NullPointerException: the animated dye");
        let new_dye = AnimatedDye::new(dye.keyframes(), checked, dye.delay(), dye.duration());
        SkyblockerConfigManager::update_only(Arc::new(move |config: &mut crate::de::hysky::skyblocker::config::skyblocker_config::SkyblockerConfig| {
            config.general.custom_animated_dyes.insert(item_uuid.clone(), new_dye.clone());
        }));
    }

    /// java: the delay slider's `f -> {..}`
    fn on_delay_changed(&mut self, f: f32) {
        let item_uuid = self.item_uuid();
        let dye: AnimatedDye = SkyblockerConfigManager::get().general.custom_animated_dyes.get(&item_uuid).cloned().expect("NullPointerException: the animated dye");
        let new_dye = AnimatedDye::new(dye.keyframes(), dye.cycle_back(), f, dye.duration());
        SkyblockerConfigManager::update_only(Arc::new(move |config: &mut crate::de::hysky::skyblocker::config::skyblocker_config::SkyblockerConfig| {
            config.general.custom_animated_dyes.insert(item_uuid.clone(), new_dye.clone());
        }));
    }

    /// java: the duration slider's `f -> {..}`
    fn on_duration_changed(&mut self, f: f32) {
        let item_uuid = self.item_uuid();
        let dye: AnimatedDye = SkyblockerConfigManager::get().general.custom_animated_dyes.get(&item_uuid).cloned().expect("NullPointerException: the animated dye");
        let new_dye = AnimatedDye::new(dye.keyframes(), dye.cycle_back(), dye.delay(), f);
        SkyblockerConfigManager::update_only(Arc::new(move |config: &mut crate::de::hysky::skyblocker::config::skyblocker_config::SkyblockerConfig| {
            config.general.custom_animated_dyes.insert(item_uuid.clone(), new_dye.clone());
        }));
    }

    /// java: private void onClickPickDye(Button button)
    fn on_click_pick_dye(&mut self) {
        // java: if (client.gui.screen() == null) return;
        if compat::minecraft().gui.screen().is_none() {
            return;
        }
        let solid = compat::RenderThread::new(self.self_ref.clone());
        let animated = compat::RenderThread::new(self.self_ref.clone());
        // java: client.gui.setScreen(new DyeSelectPopup(client.gui.screen(), this::setFromSolidDye, this::setFromAnimatedDye));
        // rust: port gap (GAPS_batchA #19) - the popup base keeps its background as the game's `Screen` value and the
        //   port lends no handle to the live one; the screen that is up is held by the game (`set_screen_keeping_parent`)
        //   and an empty screen stands in for the base's field
        let popup = DyeSelectPopup::new(
            mcrust::net::minecraft::client::gui::screens::screen::Screen::new(String::new()),
            Arc::new(move |button: AbstractWidget, hex: i32| {
                if let Some(this) = solid.get().upgrade() {
                    compat::write(&this, |__t| __t.set_from_solid_dye(&button, hex));
                }
            }),
            Arc::new(move |button: AbstractWidget, key_frames: Vec<Keyframe>, cycle_back: bool| {
                if let Some(this) = animated.get().upgrade() {
                    compat::write(&this, |__t| __t.set_from_animated_dye(&button, &key_frames, cycle_back));
                }
            }),
        );
        mcrust::mcrust::modding::game::set_screen_keeping_parent(Box::new(compat::ScreenAdapter::new(popup)));
    }

    /// java: private void setFromSolidDye(Button button, int hex)
    pub fn set_from_solid_dye(&mut self, _button: &AbstractWidget, hex: i32) {
        self.on_remove_custom_color();
        self.on_picker_color_changed(hex, true);
        self.run_events();
    }

    /// java: private void setFromAnimatedDye(Button button, List<CustomArmorAnimatedDyes.Keyframe> keyFrames, boolean cycleBack)
    pub fn set_from_animated_dye(&mut self, _button: &AbstractWidget, key_frames: &Vec<Keyframe>, cycle_back: bool) {
        self.on_remove_custom_color();
        self.animated = true;
        compat::set_checkbox_selected(self.children.widget_mut(ANIMATED_CHECKBOX), true);
        compat::set_checkbox_selected(self.children.widget_mut(CYCLE_BACK_CHECKBOX), cycle_back);
        compat::write(&self.duration_slider, |__t| __t.set_value(10.0));
        self.change_visibilities();

        let item_uuid = self.item_uuid();
        let new_dye = AnimatedDye::new(key_frames.clone(), cycle_back, 0.0, 10.0);
        let uuid = item_uuid.clone();
        SkyblockerConfigManager::update_only(Arc::new(move |config: &mut crate::de::hysky::skyblocker::config::skyblocker_config::SkyblockerConfig| {
            config.general.custom_animated_dyes.insert(uuid.clone(), new_dye.clone());
        }));
        compat::write(&self.timeline_widget, |__t| {
            __t.set_animated_dye(&item_uuid);
            __t.recreate_image(); // have to do this too for some reason
        });
        self.run_events();
    }

    /// java: private void changeVisibilities()
    fn change_visibilities(&mut self) {
        let (customizable, animated) = (self.customizable, self.animated);
        self.children.widget_mut(COLOR_PICKER).visible = customizable;
        self.children.widget_mut(ARGB_TEXT_INPUT).visible = customizable;

        compat::write(&self.timeline_widget, |__t| __t.base.visible = customizable && animated);
        self.children.widget_mut(CYCLE_BACK_CHECKBOX).visible = customizable && animated;
        compat::write(&self.delay_slider, |__t| __t.base.visible = customizable && animated);
        compat::write(&self.duration_slider, |__t| __t.base.visible = customizable && animated);

        self.children.widget_mut(RESET_COLOR_BUTTON).visible = customizable;
        self.children.widget_mut(PICK_DYE_BUTTON).visible = customizable;
        self.children.widget_mut(ANIMATED_CHECKBOX).visible = customizable;
        self.children.widget_mut(NOT_CUSTOMIZABLE_TEXT).visible = !customizable;
    }

    /// java: @Override public void close()
    pub fn close(&mut self) {
        compat::write(&self.timeline_widget, |__t| __t.close());
    }

    /// java: public void setCurrentItem(ItemStack currentItem)
    pub fn set_current_item(&mut self, current_item: mcrust::net::minecraft::world::item::item_stack::ItemStack) {
        self.current_item = Some(current_item);
        self.refresh();
    }

    /// java: public void refresh()
    pub fn refresh(&mut self) {
        let item_uuid = self.item_uuid();
        let general = &SkyblockerConfigManager::get().general;
        let stack = self.current_item.as_ref().expect("NullPointerException: ColorSelectionWidget.currentItem");
        let key: Option<ResourceKey> = if let Some(model) = general.custom_armor_model.get(&item_uuid) {
            // java: ResourceKey.create(EquipmentAssets.ROOT_ID, SkyblockerConfigManager.get().general.customArmorModel.get(itemUuid))
            Some(ResourceKey::create(&mcrust::net::minecraft::world::item::equipment::equipment_assets::root_id(), model.clone()))
        } else if stack.has(mcrust::net::minecraft::core::component::data_components::equippable()) {
            let component: Option<Equippable> = stack.get(mcrust::net::minecraft::core::component::data_components::equippable()).and_then(|__o| mcrust::com::mojang::serialization::cast::<Equippable>(&__o).cloned());
            component.and_then(|component| component.asset_id().cloned())
        } else {
            None
        };
        match key {
            None => self.customizable = false,
            Some(key) => {
                // java: ((EntityRenderDispatcherAccessor) Minecraft.getInstance().getEntityRenderDispatcher()).getEquipmentAssets().get(key)
                let model = match compat::minecraft().game_renderer.equipment_assets.as_ref() {
                    Some(assets) => assets.get(&key),
                    None => mcrust::net::minecraft::client::resources::model::equipment_asset_manager::missing(),
                };
                self.customizable = [LayerType::Humanoid, LayerType::HumanoidLeggings, LayerType::Wings].into_iter().flat_map(|l| model.get_layers(l).iter()).any(|layer| layer.dyeable().is_some());
            }
        }
        if !self.customizable {
            self.animated = false;
            compat::set_checkbox_selected(self.children.widget_mut(ANIMATED_CHECKBOX), false);
            self.change_visibilities();
            self.run_events();
            return;
        }
        if let Some(animated_dye) = general.custom_animated_dyes.get(&item_uuid) {
            self.animated = true;
            compat::set_checkbox_selected(self.children.widget_mut(CYCLE_BACK_CHECKBOX), animated_dye.cycle_back());
            compat::write(&self.delay_slider, |__t| __t.set_value(animated_dye.delay()));
            compat::write(&self.duration_slider, |__t| __t.set_value(animated_dye.duration()));
            compat::write(&self.timeline_widget, |__t| __t.set_animated_dye(&item_uuid));
        } else if let Some(color) = general.custom_dye_colors.get(&item_uuid).copied() {
            self.animated = false;
            ARGBTextInput::set_argb_color(&self.argb_text_input, color);
            ColorPickerWidget::set_argb_color(&self.color_picker, color);
        } else {
            self.animated = false;
            let color: i32 = self.dyed_color_or_default();
            ARGBTextInput::set_argb_color(&self.argb_text_input, color);
            ColorPickerWidget::set_argb_color(&self.color_picker, color);
        }
        self.change_visibilities();
        compat::set_checkbox_selected(self.children.widget_mut(ANIMATED_CHECKBOX), self.animated);
        self.run_events();
    }
}

impl compat::ObjectWidget for ColorSelectionWidget {
    fn object_box(&mut self) -> &mut AbstractContainerWidget {
        &mut self.base
    }

    fn object_box_ref(&self) -> &AbstractContainerWidget {
        &self.base
    }

    /// java: @Override public void setX(int x)
    fn set_x(&mut self, x: i32) {
        self.base.set_x(x);
        self.layout.set_x(self.base.get_x() + PADDING);
        self.place_children();
    }

    /// java: @Override public void setY(int y)
    fn set_y(&mut self, y: i32) {
        self.base.set_y(y);
        self.layout.set_y(self.base.get_y() + PADDING);
        self.place_children();
    }

    /// java: @Override public void setWidth(int width)
    fn set_width(&mut self, width: i32) {
        self.base.set_width(width);
        self.update_widget_dimensions();
    }

    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_widget_render_state(&mut self, _widget: &AbstractWidget, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, _a: f32) {
        gui_graphics_extractor::blit_sprite_identifier(context.gl, context.minecraft, mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), INNER_SPACE_TEXTURE.get(), self.base.get_x(), self.base.get_y(), self.base.get_width(), self.base.get_height());
        // java: for (AbstractWidget child : children) child.extractRenderState(graphics, mouseX, mouseY, a);
        context.extract_widgets(&mut self.children, mouse_x, mouse_y);
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    fn mouse_clicked(&mut self, _widget: &mut AbstractWidget, event: &MouseButtonEvent, _double_click: bool) -> bool {
        // java: super.mouseClicked - AbstractContainerWidget:62-66 (no scrollbar: contentHeight() is 0) over the children
        let consumed = compat::children_mouse_clicked(&mut self.children, event);
        self.run_events();
        if !consumed {
            // java: setFocused(null);
            self.children.set_focused(None);
            return false;
        }
        true
    }

    /// ContainerEventHandler.mouseReleased:57-66
    fn mouse_released(&mut self, _widget: &mut AbstractWidget, event: &MouseButtonEvent) -> bool {
        compat::children_mouse_released(&mut self.children, event);
        self.run_events();
        false
    }

    /// ContainerEventHandler.mouseDragged:68-71
    fn mouse_dragged(&mut self, _widget: &mut AbstractWidget, event: &MouseButtonEvent, dx: f64, dy: f64) -> bool {
        compat::children_mouse_dragged(&mut self.children, event, dx, dy);
        self.run_events();
        false
    }

    /// java: @Override public boolean mouseScrolled(double mouseX, double mouseY, double horizontalAmount, double verticalAmount)
    fn mouse_scrolled(&mut self, _widget: &mut AbstractWidget, x: f64, y: f64, scroll_x: f64, scroll_y: f64) -> bool {
        let child = compat::children_mouse_scrolled(&mut self.children, x, y, scroll_x, scroll_y);
        self.run_events();
        // java: || super.mouseScrolled(..) - AbstractScrollArea.mouseScrolled:27-35, `true` while visible
        child || self.base.visible
    }

    /// ContainerEventHandler.keyPressed - the focused child
    fn key_pressed(&mut self, _widget: &mut AbstractWidget, event: &KeyEvent) -> bool {
        let answer = compat::children_key_pressed(&mut self.children, event);
        self.run_events();
        answer
    }

    /// ContainerEventHandler.charTyped - the focused child
    fn char_typed(&mut self, _widget: &mut AbstractWidget, event: &CharacterEvent) -> bool {
        let answer = compat::children_char_typed(&mut self.children, event);
        self.run_events();
        answer
    }
}

/// java: private static class Slider extends AbstractSliderButton
pub struct Slider {
    /// java: extends AbstractSliderButton - AbstractWidget's fields (x, y, `width`, `height` 15, `active`, `visible`)
    pub base: AbstractContainerWidget,
    /// AbstractSliderButton's `value`, `canChangeValue`, `dragging`
    pub state: SliderState,
    /// AbstractWidget's `message`
    pub message: FormattedText,
    /// java: private final float minValue;
    pub min_value: f32,
    /// java: private final float maxValue;
    pub max_value: f32,
    /// java: private final float step;
    pub step: f32,
    /// java: private final boolean linear;
    pub linear: bool,
    /// java: private final String translatable;
    pub translatable: String,
    /// java: private final FloatConsumer onValueChanged;
    pub on_value_changed: Arc<dyn Fn(f32)>,
    /// java: private boolean clicked = false;
    pub clicked: bool,
    /// the game's slider look (`AbstractSliderButton.extractWidgetRenderState`)
    look: compat::SliderLook,
}

impl Slider {
    /// java: private Slider(int x, int y, int width, float min, float max, float step, boolean linear, @Translatable String translatable, FloatConsumer onValueChanged)
    pub fn new(x: i32, y: i32, width: i32, min: f32, max: f32, step: f32, linear: bool, translatable: &str, on_value_changed: Arc<dyn Fn(f32)>) -> compat::Shared<Self> {
        // java: super(x, y, width, 15, Component.empty(), 0);
        if min >= max || step <= 0.0 || step > (max - min) {
            panic!("IllegalArgumentException: Invalid slider parameters: min={}, max={}, step={}", compat::fstr(min as f64), compat::fstr(max as f64), compat::fstr(step as f64));
        }
        let mut this = Slider {
            base: AbstractContainerWidget::new(x, y, width, 15, FormattedText::Empty),
            state: SliderState::new(0.0),
            message: FormattedText::Empty,
            min_value: min,
            max_value: max,
            step,
            linear, // old code stuff... is always true, keeping it, can maybe be useful
            translatable: translatable.to_string(),
            on_value_changed,
            clicked: false,
            look: compat::SliderLook::new(),
        };
        this.update_message();
        compat::shared(this)
    }

    /// java: private float trueValue()
    pub fn true_value(&self) -> f32 {
        let value = self.state.value;
        let v: f64 = if self.linear { value } else { value * value };
        return self.round_to_step(v * ((self.max_value - self.min_value) as f64));
    }

    /// java: @Override protected void updateMessage()
    pub fn update_message(&mut self) {
        let value = formatters::DOUBLE_NUMBERS.get().format(self.true_value() as f64);
        self.message = FormattedText::from(component::translatable_args(&self.translatable, vec![FormattedText::of(value)]));
    }

    /// java: private void setValue(float val)
    pub fn set_value(&mut self, val: f32) {
        let v: f32 = (val - self.min_value) / (self.max_value - self.min_value);
        self.state.value = if self.linear { v as f64 } else { (v as f64).sqrt() };
        self.update_message();
    }

    /// AbstractSliderButton.setValue(double) - clamped; `applyValue()` (empty here) when it changed; `updateMessage()`
    fn set_slider_value(&mut self, new_value: f64) {
        self.state.set_value(new_value);
        self.update_message();
    }

    /// java: private float roundToStep(double value)
    fn round_to_step(&self, value: f64) -> f32 {
        return (self.min_value + self.step * (compat::java_round_f64(value / (self.step as f64)) as f32)).clamp(self.min_value, self.max_value);
    }
}

impl compat::ObjectWidget for Slider {
    fn object_box(&mut self) -> &mut AbstractContainerWidget {
        &mut self.base
    }

    fn object_box_ref(&self) -> &AbstractContainerWidget {
        &self.base
    }

    /// AbstractSliderButton.extractWidgetRenderState - the track, the handle and the message
    fn extract_widget_render_state(&mut self, widget: &AbstractWidget, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, _a: f32) {
        self.look.draw(widget, &self.message, &self.state, context, mouse_x, mouse_y);
    }

    /// java: @Override public void onClick(MouseButtonEvent click, boolean doubled)
    fn on_click(&mut self, _widget: &mut AbstractWidget, event: &MouseButtonEvent, _double_click: bool) {
        // java: super.onClick(click, doubled); - `setValueFromMouse(event)` (dragging = active)
        let (x, width, active) = (self.base.get_x(), self.base.get_width(), self.base.is_active());
        self.state.on_click(event.x(), x, width, active);
        self.update_message();
        self.clicked = true;
    }

    /// AbstractSliderButton.onDrag - `setValueFromMouse(event)`
    fn on_drag(&mut self, _widget: &mut AbstractWidget, event: &MouseButtonEvent, _dx: f64, _dy: f64) {
        let (x, width) = (self.base.get_x(), self.base.get_width());
        self.state.on_drag(event.x(), x, width);
        self.update_message();
    }

    /// java: @Override public void onRelease(MouseButtonEvent click)
    fn on_release(&mut self, _widget: &mut AbstractWidget, _event: &MouseButtonEvent) {
        // java: super.onRelease(click);
        self.state.on_release();
        if self.clicked {
            (self.on_value_changed)(self.true_value());
            self.clicked = false;
        }
    }

    /// java: @Override public boolean keyPressed(KeyEvent input)
    fn key_pressed(&mut self, _widget: &mut AbstractWidget, event: &KeyEvent) -> bool {
        // java: super.keyPressed(input) - AbstractSliderButton.keyPressed
        let handled = if event.is_selection() {
            self.state.toggle_armed();
            true
        } else if self.state.can_change_value && (event.is_left() || event.is_right()) {
            let step = abstract_slider_button::key_step(self.base.get_width(), event.is_left());
            let value = self.state.value + step;
            self.set_slider_value(value);
            true
        } else {
            false
        };
        if handled {
            (self.on_value_changed)(self.true_value());
            return true;
        }
        false
    }

    /// java: @Override public boolean mouseScrolled(double mouseX, double mouseY, double horizontalAmount, double verticalAmount)
    fn mouse_scrolled(&mut self, _widget: &mut AbstractWidget, _x: f64, _y: f64, _scroll_x: f64, vertical_amount: f64) -> bool {
        if vertical_amount == 0.0 {
            return false;
        }
        let offset: f32 = if vertical_amount > 0.0 { self.step } else { -self.step };
        let value = (self.true_value() + offset).clamp(self.min_value, self.max_value);
        self.set_value(value);
        (self.on_value_changed)(self.true_value());
        true
    }

    /// AbstractSliderButton.setFocused - `canChangeValue` follows the input that focused it
    fn set_focused(&mut self, widget: &mut AbstractWidget, focused: bool) {
        let last_input_type = compat::minecraft().last_input_type;
        widget.set_focused(focused, last_input_type);
        self.state.set_focused(focused, last_input_type);
    }

    /// AbstractSliderButton.playDownSound - empty (the click is the release's)
    fn play_down_sound(&self) -> Option<&'static mcrust::net::minecraft::sounds::sound_event::SoundEvent> {
        None
    }

    /// AbstractSliderButton.handleCursor - RESIZE_EW while dragging
    fn handle_cursor(&self, widget: &AbstractWidget, cursors: &CursorTypes) -> Option<CursorType> {
        abstract_slider_button::cursor_for(cursors, widget.is_hovered(), self.base.is_active(), self.state.dragging)
    }
}
