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
use mcrust::net::minecraft::client::gui::components::abstract_container_widget::AbstractContainerWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::abstract_widget::PressLambda;
use mcrust::net::minecraft::client::gui::components::button;
use mcrust::net::minecraft::client::gui::components::checkbox::Checkbox;
use mcrust::net::minecraft::client::gui::components::events::container_event_handler;
use mcrust::net::minecraft::client::gui::components::events::gui_event_listener::GuiEventListener;
use mcrust::net::minecraft::client::gui::components::tooltip::Tooltip;
use mcrust::net::minecraft::client::gui::font::Font;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::layouts::frame_layout;
use mcrust::net::minecraft::client::gui::layouts::frame_layout::FrameLayout;
use mcrust::net::minecraft::client::gui::layouts::grid_layout::GridLayout;
use mcrust::net::minecraft::client::gui::layouts::layout_element::LayoutElement;
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::client::resources::model::equipment_client_info::EquipmentClientInfo;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::resources::resource_key::ResourceKey;
use mcrust::net::minecraft::util::argb;
use mcrust::net::minecraft::world::item::component::dyed_item_color;
use mcrust::net::minecraft::world::item::equipment::equipment_asset::EquipmentAsset;
use mcrust::net::minecraft::world::item::equipment::equippable::Equippable;
use crate::de::hysky::skyblocker::injected::skyblocker_stack::SkyblockerStack as _;
use mcrust::net::minecraft::client::gui::components::renderable::Renderable as _;
use mcrust::net::minecraft::client::gui::layouts::abstract_layout::AbstractLayout as _;
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

/// java: public class ColorSelectionWidget extends AbstractContainerWidget implements Closeable
pub struct ColorSelectionWidget {
    /// java: extends net.minecraft.client.gui.components.AbstractContainerWidget
    pub base: AbstractContainerWidget,
    /// java: private final ColorPickerWidget colorPicker;
    pub color_picker: compat::Shared<ColorPickerWidget>,
    /// java: private final ARGBTextInput argbTextInput;
    pub argb_text_input: Option<compat::Shared<ARGBTextInput>>,
    /// java: private final AnimatedDyeTimelineWidget timelineWidget;
    pub timeline_widget: Option<compat::Shared<AnimatedDyeTimelineWidget>>,
    /// java: private final Checkbox cycleBackCheckbox;
    pub cycle_back_checkbox: Option<Checkbox>,
    /// java: private final Slider delaySlider;
    pub delay_slider: Option<compat::Shared<Slider>>,
    /// java: private final Slider durationSlider;
    pub duration_slider: Option<compat::Shared<Slider>>,
    /// java: private final Button resetColorButton;
    pub reset_color_button: Option<AbstractWidget>,
    /// java: private final Button pickDyeButton;
    pub pick_dye_button: Option<AbstractWidget>,
    /// java: private final Checkbox animatedCheckbox;
    pub animated_checkbox: Option<Checkbox>,
    /// java: private final StringWidget notCustomizableText;
    pub not_customizable_text: Option<AbstractWidget>,
    /// java: private final FrameLayout layout;
    pub layout: Option<FrameLayout>,
    /// java: private ItemStack currentItem;
    pub current_item: Option<mcrust::net::minecraft::world::item::item_stack::ItemStack>,
    /// java: private boolean animated;
    pub animated: bool,
    /// java: private boolean customizable = false;
    pub customizable: bool,
    /// java: private final List<AbstractWidget> children;
    pub children: Vec<AbstractWidget>,
}

impl std::ops::Deref for ColorSelectionWidget {
    type Target = AbstractContainerWidget;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for ColorSelectionWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl ColorSelectionWidget {
    /// java: public ColorSelectionWidget(int x, int y, int width, int height, Font textRenderer)
    pub fn new(mut x: i32, mut y: i32, mut width: i32, mut height: i32, mut text_renderer: Font) -> Self {
        let mut height1: i32 = (((2i32.wrapping_mul(height)).wrapping_div(3)).min(width.wrapping_div(5))).min(height.wrapping_sub(40));
        let __init_color_picker = ColorPickerWidget::new(0, 0, height1.wrapping_mul(2), height1);
        // TODO(fabric2seer): static `AbstractScrollArea.defaultSettings` has no Rust counterpart in the port
        //   java: super(x, y, width, height, Component.nullToEmpty("ColorSelectionWidget"), AbstractScrollArea.defaultSettings(4));
        let mut this: Self = todo!("constructor");
        ColorPickerWidget::set_on_color_change(&this.color_picker, Arc::new(move |__a0, __a1| { this.on_picker_color_changed(__a0, __a1); }));
        this.argb_text_input = Some(ARGBTextInput::new_4(0, 0, compat::font(), true));
        ARGBTextInput::set_on_change(this.argb_text_input.as_ref().unwrap(), Arc::new(move |__a0| { this.on_text_input_color_changed(__a0); }));
        this.timeline_widget = Some(AnimatedDyeTimelineWidget::new(0, 0, this.get_width().wrapping_sub(6), 15, Arc::new(move |__a0, __a1| { this.on_timeline_frame_selected(__a0, __a1); })));
        this.reset_color_button = Some((button::builder(RESET_COLOR_TEXT.get_mut().get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { this.on_remove_custom_color((__screen.widget_mut(__index))); })))).width(75).build());
        this.pick_dye_button = Some((button::builder(PICK_DYE_COLOR_TEXT.get_mut().get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { this.on_click_pick_dye((__screen.widget_mut(__index))); })))).width(75).build());
        this.not_customizable_text = Some(compat::string_widget(0, 0, -1, 9, CANNOT_CUSTOMIZE_COLOR_TEXT.get_mut().clone()));
        // TODO(fabric2seer): a widget put in a layout outside a mod screen's own methods: the port's layouts hold a widget by its index in the screen's widget list
        // rust: frame_layout::center_in_rectangle(this.not_customizable_text.as_mut().unwrap(), this.get_x(), this.get_y(), this.get_width(), this.get_height());
        // TODO(fabric2seer): no overload of `onAnimatedCheckbox` takes 1 argument(s)
        //   java: animatedCheckbox = Checkbox.builder(ANIMATED_TEXT, textRenderer)
        //   java: 				.pos(colorPicker.getRight() + 5, resetColorButton.getBottom())
        //   java: 				.maxWidth(80)
        //   java: 				.onValueChange(this::onAnimatedCheckbox)
        //   java: 				.build();
        // TODO(fabric2seer): no overload of `onCycleBackCheckbox` takes 1 argument(s)
        //   java: cycleBackCheckbox = Checkbox.builder(CYCLE_BACK_TEXT, textRenderer)
        //   java: 				.pos(colorPicker.getRight() + 5, animatedCheckbox.getBottom() + 3)
        //   java: 				.maxWidth(80)
        //   java: 				.onValueChange(this::onCycleBackCheckbox)
        //   java: 				.build();
        let mut slider_width: i32 = ((((width as f32) * 0.35f32)) as i32);
        this.delay_slider = Some(Slider::new(0, 0, slider_width, 0.0f32, 2.0f32, 0.02f32, true, DELAY_TEXT.to_string(), Arc::new(move |f| {
            let mut item_uuid: String = this.current_item.as_ref().unwrap().get_uuid();
            let mut dye: AnimatedDye = SkyblockerConfigManager::get().general.custom_animated_dyes.get(&item_uuid).cloned().unwrap();
            let mut new_dye: AnimatedDye = AnimatedDye::new(dye.keyframes(), dye.cycle_back(), f, dye.duration());
            SkyblockerConfigManager::update_only(Arc::new(move |mut config| { config.general.custom_animated_dyes.insert(item_uuid.clone(), new_dye.clone()); }));
        })));
        let __wa1 = Tooltip::create(DELAY_TOOLTIP_TEXT.get_mut().get_string());
        let __o5 = this.delay_slider.clone().unwrap();
        compat::write(&__o5, |__t| __t.set_tooltip(__wa1.clone()));
        this.duration_slider = Some(Slider::new(0, 0, slider_width, 0.1f32, 10.0f32, 0.1f32, true, DURATION_TEXT.to_string(), Arc::new(move |f| {
            let mut item_uuid: String = this.current_item.as_ref().unwrap().get_uuid();
            let mut dye: AnimatedDye = SkyblockerConfigManager::get().general.custom_animated_dyes.get(&item_uuid).cloned().unwrap();
            let mut new_dye: AnimatedDye = AnimatedDye::new(dye.keyframes(), dye.cycle_back(), dye.delay(), f);
            SkyblockerConfigManager::update_only(Arc::new(move |mut config| { config.general.custom_animated_dyes.insert(item_uuid.clone(), new_dye.clone()); }));
        })));
        let __wa2 = Tooltip::create(DURATION_TOOLTIP_TEXT.get_mut().get_string());
        let __o6 = this.duration_slider.clone().unwrap();
        compat::write(&__o6, |__t| __t.set_tooltip(__wa2.clone()));
        this.children = vec![this.color_picker.clone(), this.argb_text_input.clone().unwrap(), this.timeline_widget.clone().unwrap(), this.reset_color_button.clone().unwrap(), this.pick_dye_button.clone().unwrap(), this.animated_checkbox.clone().unwrap(), this.not_customizable_text.clone().unwrap(), this.cycle_back_checkbox.clone().unwrap(), this.delay_slider.clone().unwrap(), this.duration_slider.clone().unwrap()];
        let mut w: i32 = this.get_width().wrapping_sub(PADDING.wrapping_mul(2));
        let mut h: i32 = this.get_height().wrapping_sub(PADDING.wrapping_mul(2));
        this.layout = Some(FrameLayout::of(w, h));
        this.layout.as_mut().unwrap().add_child_adjusted(this.timeline_widget.clone().unwrap(), &mut |__a0| { __a0.align_vertically_bottom(); });
        let mut grid: GridLayout = GridLayout::new_default().spacing(3);
        // TODO(fabric2seer): a widget put in a layout outside a mod screen's own methods: the port's layouts hold a widget by its index in the screen's widget list
        // rust: grid.add_child_at(this.argb_text_input.clone().unwrap(), 0, 1).unwrap();
        // TODO(fabric2seer): a widget put in a layout outside a mod screen's own methods: the port's layouts hold a widget by its index in the screen's widget list
        // rust: grid.add_child_span_adjusted(this.reset_color_button.clone().unwrap(), 0, 1, 1, 3, &mut |__a0| { __a0.align_horizontally_right(); }).unwrap();
        // TODO(fabric2seer): a widget put in a layout outside a mod screen's own methods: the port's layouts hold a widget by its index in the screen's widget list
        // rust: grid.add_child_span_adjusted(this.pick_dye_button.clone().unwrap(), 0, 2, 1, 4, &mut |__a0| { __a0.align_horizontally_right(); }).unwrap();
        grid.add_child_span(this.animated_checkbox.clone().unwrap(), 1, 1, 1, 2).unwrap();
        grid.add_child_span_adjusted(this.delay_slider.clone().unwrap(), 1, 3, 1, 2, &mut |__a0| { __a0.align_horizontally_right(); }).unwrap();
        grid.add_child_span(this.cycle_back_checkbox.clone().unwrap(), 2, 1, 1, 2).unwrap();
        grid.add_child_span_adjusted(this.duration_slider.clone().unwrap(), 2, 3, 1, 2, &mut |__a0| { __a0.align_horizontally_right(); }).unwrap();
        // TODO(fabric2seer): a widget put in a layout outside a mod screen's own methods: the port's layouts hold a widget by its index in the screen's widget list
        // rust: grid.add_child_span(this.color_picker.clone(), 0, 0, 3, 1).unwrap();
        this.layout.as_mut().unwrap().add_child_adjusted(LayoutElement::Grid(grid.clone()), &mut |__a0| { __a0.align_vertically_top(); });
        // TODO(fabric2seer): a widget put in a layout outside a mod screen's own methods: the port's layouts hold a widget by its index in the screen's widget list
        // rust: this.layout.as_mut().unwrap().add_child(this.not_customizable_text.clone().unwrap());
        this.update_widget_dimensions();
        this
    }

    /// java: private void updateWidgetDimensions()
    pub fn update_widget_dimensions(&mut self) {
        let mut w: i32 = self.get_width().wrapping_sub(PADDING.wrapping_mul(2));
        let mut h: i32 = self.get_height().wrapping_sub(PADDING.wrapping_mul(2));
        AnimatedDyeTimelineWidget::set_width(self.timeline_widget.as_ref().unwrap(), w);
        let __o1 = self.timeline_widget.clone().unwrap();
        ColorPickerWidget::set_height(&self.color_picker, ((h.wrapping_sub(compat::write(&__o1, |__t| __t.get_height()))).wrapping_sub(5)).min((w.wrapping_div(3)).wrapping_div(2)));
        let __o2 = self.color_picker.clone();
        ColorPickerWidget::set_width(&self.color_picker, (compat::write(&__o2, |__t| __t.get_height())).wrapping_mul(2));
        let __wa1 = ((((w as f32) * 0.35f32)) as i32);
        let __o3 = self.delay_slider.clone().unwrap();
        compat::write(&__o3, |__t| __t.set_width(__wa1));
        let __wa2 = ((((w as f32) * 0.35f32)) as i32);
        let __o4 = self.duration_slider.clone().unwrap();
        compat::write(&__o4, |__t| __t.set_width(__wa2));
        self.layout.as_mut().unwrap().arrange_elements();
        { let __s0 = self.get_x().wrapping_add(PADDING); let __s1 = self.get_y().wrapping_add(PADDING); self.layout.as_mut().unwrap().set_position(__s0, __s1) };
        self.width = (self.layout.as_ref().unwrap().get_width()).wrapping_add(PADDING.wrapping_mul(2));
        self.height = (self.layout.as_ref().unwrap().get_height()).wrapping_add(PADDING.wrapping_mul(2));
    }

    /// java: @Override public void setX(int x)
    pub fn set_x(&mut self, mut x: i32) {
        self.base.set_x(x);
        { let __s0 = self.get_x().wrapping_add(PADDING); self.layout.as_mut().unwrap().set_x(__s0) };
    }

    /// java: @Override public void setY(int y)
    pub fn set_y(&mut self, mut y: i32) {
        self.base.set_y(y);
        { let __s0 = self.get_y().wrapping_add(PADDING); self.layout.as_mut().unwrap().set_y_translating(__s0) };
    }

    /// java: @Override public void setWidth(int width)
    pub fn set_width(&mut self, mut width: i32) {
        self.base.set_width(width);
        self.update_widget_dimensions();
    }

    /// java: public AnimatedDyeTimelineWidget getTimelineWidget()
    pub fn get_timeline_widget(&self) -> compat::Shared<AnimatedDyeTimelineWidget> {
        return self.timeline_widget.clone().unwrap();
    }

    /// java: private void onPickerColorChanged(int argb, boolean release)
    pub fn on_picker_color_changed(&mut self, mut argb: i32, mut release: bool) {
        ARGBTextInput::set_argb_color(self.argb_text_input.as_ref().unwrap(), argb);
        if !self.animated {
            let __uuid = self.current_item.as_ref().unwrap().get_uuid();
            SkyblockerConfigManager::update_only(Arc::new(move |mut config| { config.general.custom_dye_colors.insert(__uuid.clone(), argb::opaque(argb)); }));
        } else if release {
            AnimatedDyeTimelineWidget::set_color(self.timeline_widget.as_ref().unwrap(), argb);
        }
    }

    /// java: private void onTextInputColorChanged(int argb)
    pub fn on_text_input_color_changed(&mut self, mut argb: i32) {
        ColorPickerWidget::set_argb_color(&self.color_picker, argb);
        if self.animated {
            AnimatedDyeTimelineWidget::set_color(self.timeline_widget.as_ref().unwrap(), argb);
        } else {
            let __uuid = self.current_item.as_ref().unwrap().get_uuid();
            SkyblockerConfigManager::update_only(Arc::new(move |mut config| { config.general.custom_dye_colors.insert(__uuid.clone(), argb::opaque(argb)); }));
        }
    }

    /// java: @Override public boolean mouseScrolled(double mouseX, double mouseY, double horizontalAmount, double verticalAmount)
    pub fn mouse_scrolled(&self, mut mouse_x: f64, mut mouse_y: f64, mut horizontal_amount: f64, mut vertical_amount: f64) -> bool {
        // TODO(fabric2seer): Optional.mouseScrolled
        //   java: return getChildAt(mouseX, mouseY).filter(element -> element.mouseScrolled(mouseX, mouseY, horizontalAmount, verticalAmount)).isPresent() || super.mouseScrolled(mouseX, mouseY, horizontalAmount, verticalAmount);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: private void onTimelineFrameSelected(int color, float time)
    pub fn on_timeline_frame_selected(&mut self, mut color: i32, mut time: f32) {
        ARGBTextInput::set_argb_color(self.argb_text_input.as_ref().unwrap(), color);
        ColorPickerWidget::set_argb_color(&self.color_picker, color);
    }

    /// java: private void onRemoveCustomColor(Button button)
    pub fn on_remove_custom_color(&mut self, button: &AbstractWidget) {
        self.animated = false;
        self.animated_checkbox.selected = false;
        self.change_visibilities();
        let mut item_uuid: String = self.current_item.as_ref().unwrap().get_uuid();
        SkyblockerConfigManager::update_only(Arc::new(move |mut config| { config.general.custom_dye_colors.remove(&item_uuid); }));
        SkyblockerConfigManager::update_only(Arc::new(move |mut config| { config.general.custom_animated_dyes.remove(&item_uuid); }));
        let mut color: i32 = dyed_item_color::get_or_default(self.current_item.as_ref().unwrap().get(mcrust::net::minecraft::core::component::data_components::dyed_color()).and_then(|__value| mcrust::com::mojang::serialization::cast::<mcrust::net::minecraft::world::item::component::dyed_item_color::DyedItemColor>(&__value).copied()).as_ref(), -1);
        ARGBTextInput::set_argb_color(self.argb_text_input.as_ref().unwrap(), color);
        ColorPickerWidget::set_argb_color(&self.color_picker, color);
    }

    /// java: private void onAnimatedCheckbox(Checkbox checkbox, boolean checked)
    pub fn on_animated_checkbox(&mut self, checkbox: &Checkbox, mut checked: bool) {
        self.animated = checked;
        self.change_visibilities();
        let mut item_uuid: String = self.current_item.as_ref().unwrap().get_uuid();
        if self.animated {
            SkyblockerConfigManager::update_only(Arc::new(move |mut config| { config.general.custom_animated_dyes.insert(item_uuid.clone(), AnimatedDye::new(vec![Keyframe::new(mcrust::net::minecraft::util::common_colors::RED, ((0) as f32) as f32), Keyframe::new(mcrust::net::minecraft::util::common_colors::BLUE, ((1) as f32) as f32)], true, ((0) as f32) as f32, 1.0f32)); }));
            AnimatedDyeTimelineWidget::set_animated_dye(self.timeline_widget.as_ref().unwrap(), item_uuid.clone());
            Slider::set_value(self.delay_slider.as_ref().unwrap(), ((0) as f32) as f32);
            Slider::set_value(self.duration_slider.as_ref().unwrap(), ((1) as f32) as f32);
            self.cycle_back_checkbox.selected = true;
        } else {
            let mut color: i32 = SkyblockerConfigManager::get().general.custom_dye_colors.get(&item_uuid).cloned().unwrap_or_else(|| dyed_item_color::get_or_default(self.current_item.as_ref().unwrap().get(mcrust::net::minecraft::core::component::data_components::dyed_color()).and_then(|__value| mcrust::com::mojang::serialization::cast::<mcrust::net::minecraft::world::item::component::dyed_item_color::DyedItemColor>(&__value).copied()).as_ref(), -1));
            ColorPickerWidget::set_argb_color(&self.color_picker, color);
            ARGBTextInput::set_argb_color(self.argb_text_input.as_ref().unwrap(), color);
            SkyblockerConfigManager::update_only(Arc::new(move |mut config| { config.general.custom_animated_dyes.remove(&item_uuid); }));
        }
    }

    /// java: private void onCycleBackCheckbox(Checkbox checkbox, boolean checked)
    pub fn on_cycle_back_checkbox(&self, checkbox: &Checkbox, mut checked: bool) {
        let mut item_uuid: String = self.current_item.as_ref().unwrap().get_uuid();
        let mut dye: AnimatedDye = SkyblockerConfigManager::get().general.custom_animated_dyes.get(&item_uuid).cloned().unwrap();
        let mut new_dye: AnimatedDye = AnimatedDye::new(dye.keyframes(), checked, dye.delay(), dye.duration());
        SkyblockerConfigManager::update_only(Arc::new(move |mut config| { config.general.custom_animated_dyes.insert(item_uuid.clone(), new_dye.clone()); }));
    }

    /// java: private void onClickPickDye(Button button)
    pub fn on_click_pick_dye(&self, button: &AbstractWidget) {
        let client = compat::minecraft();
        if client.gui.screen().is_none() {
            return;
        }
        mcrust::mcrust::modding::game::set_screen_keeping_parent(Box::new(compat::ScreenAdapter::new(DyeSelectPopup::new(client.gui.screen().unwrap(), Arc::new(move |__a0, __a1| { self.set_from_solid_dye(&__a0, __a1); }), Arc::new(move |__a0, __a1, __a2| { self.set_from_animated_dye(&__a0, &__a1, __a2); })))));
    }

    /// java: private void setFromSolidDye(Button button, int hex)
    pub fn set_from_solid_dye(&mut self, button: &AbstractWidget, mut hex: i32) {
        self.on_remove_custom_color(button);
        self.on_picker_color_changed(hex, true);
    }

    /// java: private void setFromAnimatedDye(Button button, List<CustomArmorAnimatedDyes.Keyframe> keyFrames, boolean cycleBack)
    pub fn set_from_animated_dye(&mut self, button: &AbstractWidget, key_frames: &Vec<Keyframe>, mut cycle_back: bool) {
        self.on_remove_custom_color(button);
        self.animated = true;
        self.animated_checkbox.selected = true;
        self.cycle_back_checkbox.selected = cycle_back;
        Slider::set_value(self.duration_slider.as_ref().unwrap(), ((10) as f32) as f32);
        self.change_visibilities();
        let mut item_uuid: String = self.current_item.as_ref().unwrap().get_uuid();
        let mut new_dye: AnimatedDye = AnimatedDye::new(key_frames.clone(), cycle_back, ((0) as f32) as f32, ((10) as f32) as f32);
        SkyblockerConfigManager::update_only(Arc::new(move |mut config| { config.general.custom_animated_dyes.insert(item_uuid.clone(), new_dye.clone()); }));
        AnimatedDyeTimelineWidget::set_animated_dye(self.timeline_widget.as_ref().unwrap(), item_uuid.clone());
        AnimatedDyeTimelineWidget::recreate_image(self.timeline_widget.as_ref().unwrap());
    }

    /// java: private void changeVisibilities()
    pub fn change_visibilities(&mut self) {
        { let __v = self.customizable; compat::write(&self.color_picker, |__t| __t.visible = __v); }
        { let __v = self.customizable; compat::write(self.argb_text_input.as_ref().unwrap(), |__t| __t.visible = __v); }
        { let __v = (self.customizable && self.animated); compat::write(self.timeline_widget.as_ref().unwrap(), |__t| __t.visible = __v); }
        // TODO(fabric2seer): `AbstractWidget.visible` is a field of the port's `AbstractWidget`, which the port's `Checkbox` does not hold (no Deref, no `base` field)
        // rust: self.cycle_back_checkbox.as_mut().unwrap().visible = (self.customizable && self.animated);
        { let __v = (self.customizable && self.animated); compat::write(self.delay_slider.as_ref().unwrap(), |__t| __t.visible = __v); }
        { let __v = (self.customizable && self.animated); compat::write(self.duration_slider.as_ref().unwrap(), |__t| __t.visible = __v); }
        self.reset_color_button.as_mut().unwrap().visible = self.customizable;
        self.pick_dye_button.as_mut().unwrap().visible = self.customizable;
        // TODO(fabric2seer): `AbstractWidget.visible` is a field of the port's `AbstractWidget`, which the port's `Checkbox` does not hold (no Deref, no `base` field)
        // rust: self.animated_checkbox.as_mut().unwrap().visible = self.customizable;
        self.not_customizable_text.as_mut().unwrap().visible = !self.customizable;
    }

    /// java: @Override public List<? extends GuiEventListener> children()
    pub fn children(&self) -> Vec<Box<dyn GuiEventListener>> {
        return self.children.clone();
    }

    /// java: @Override protected int contentHeight()
    pub fn content_height(&self) -> i32 {
        return 0;
    }

    /// java: @Override protected double scrollRate()
    pub fn scroll_rate(&self) -> f64 {
        return ((0) as f64) as f64;
    }

    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_widget_render_state(&self, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        gui_graphics_extractor::blit_sprite_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), INNER_SPACE_TEXTURE.get_mut(), self.get_x(), self.get_y(), self.get_width(), self.get_height());
        for child in self.children.iter() {
            // TODO(fabric2seer): `extract_render_state` is a method of the port's trait `Renderable`, which the port's `AbstractWidget` does not implement
            // rust: child.extract_render_state(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mouse_x, mouse_y, a);
        }
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    pub fn update_widget_narration(&self, builder: &dyn NarrationElementOutput) {
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    pub fn mouse_clicked(&self, click: &MouseButtonEvent, mut doubled: bool) -> bool {
        if !(self.base.mouse_clicked(click.clone(), doubled)) {
            // TODO(fabric2seer): `AbstractContainerWidget.setFocused` has no Rust method in the port (looked for `set_focused` in AbstractContainerWidget and its supertypes)
            // rust: self.set_focused(None);
            return false;
        }
        return true;
    }

    /// java: @Override public void close()
    pub fn close(&mut self) {
        AnimatedDyeTimelineWidget::close(self.timeline_widget.as_ref().unwrap());
    }

    /// java: public void setCurrentItem(ItemStack currentItem)
    pub fn set_current_item(&mut self, mut current_item: mcrust::net::minecraft::world::item::item_stack::ItemStack) {
        self.current_item = Some(current_item.clone());
        self.refresh();
    }

    /// java: public void refresh()
    pub fn refresh(&mut self) {
        let mut item_uuid: String = self.current_item.as_ref().unwrap().get_uuid();
        let mut key: Option<ResourceKey> = None;
        if SkyblockerConfigManager::get().general.custom_armor_model.contains_key(&item_uuid) {
            key = Some(ResourceKey::create(&mcrust::net::minecraft::world::item::equipment::equipment_assets::root_id(), SkyblockerConfigManager::get().general.custom_armor_model.get(&item_uuid).cloned().unwrap()));
        } else if self.current_item.as_ref().unwrap().has(mcrust::net::minecraft::core::component::data_components::equippable()) {
            let mut component: Equippable = self.current_item.as_ref().unwrap().get(mcrust::net::minecraft::core::component::data_components::equippable()).and_then(|__value| mcrust::com::mojang::serialization::cast::<Equippable>(&__value).cloned()).expect("NullPointerException");
            key = component.asset_id().cloned();
        }
        if let Some(key) = &key {
            // TODO(fabric2seer): `.get(..)` on a EquipmentAssetManager
            //   java: EquipmentClientInfo model = ((EntityRenderDispatcherAccessor) Minecraft.getInstance().getEntityRenderDispatcher()).getEquipmentAssets().get(key);
            let mut model: EquipmentClientInfo = todo!("fabric2seer: the initialiser of `model` (TODO above)");
            // TODO(fabric2seer): field `HUMANOID` of a value of unknown type
            //   java: customizable = Stream.of(EquipmentClientInfo.LayerType.HUMANOID, EquipmentClientInfo.LayerType.HUMANOID_LEGGINGS, EquipmentClientInfo.LayerType.WINGS)
            //   java: 					.flatMap(l -> model.getLayers(l).stream())
            //   java: 					.anyMatch(layer -> layer.dyeable().isPresent());
        } else {
            self.customizable = false;
        }
        if !self.customizable {
            self.animated = false;
            self.animated_checkbox.selected = false;
            self.change_visibilities();
            return;
        }
        if SkyblockerConfigManager::get().general.custom_animated_dyes.contains_key(&item_uuid) {
            self.animated = true;
            let mut animated_dye: AnimatedDye = SkyblockerConfigManager::get().general.custom_animated_dyes.get(&item_uuid).cloned().unwrap();
            self.cycle_back_checkbox.selected = animated_dye.cycle_back();
            Slider::set_value(self.delay_slider.as_ref().unwrap(), animated_dye.delay());
            Slider::set_value(self.duration_slider.as_ref().unwrap(), animated_dye.duration());
            AnimatedDyeTimelineWidget::set_animated_dye(self.timeline_widget.as_ref().unwrap(), item_uuid.clone());
        } else if SkyblockerConfigManager::get().general.custom_dye_colors.contains_key(&item_uuid) {
            self.animated = false;
            let mut color: i32 = SkyblockerConfigManager::get().general.custom_dye_colors.get(&item_uuid).copied().unwrap_or_default();
            ARGBTextInput::set_argb_color(self.argb_text_input.as_ref().unwrap(), color);
            ColorPickerWidget::set_argb_color(&self.color_picker, color);
        } else {
            self.animated = false;
            let mut color: i32 = dyed_item_color::get_or_default(self.current_item.as_ref().unwrap().get(mcrust::net::minecraft::core::component::data_components::dyed_color()).and_then(|__value| mcrust::com::mojang::serialization::cast::<mcrust::net::minecraft::world::item::component::dyed_item_color::DyedItemColor>(&__value).copied()).as_ref(), -1);
            ARGBTextInput::set_argb_color(self.argb_text_input.as_ref().unwrap(), color);
            ColorPickerWidget::set_argb_color(&self.color_picker, color);
        }
        self.change_visibilities();
        self.animated_checkbox.selected = self.animated;
    }

}

// NOTE(fabric2seer): children, close, contentHeight, extractWidgetRenderState, mouseClicked, mouseScrolled, scrollRate, setWidth, setX, setY, updateWidgetNarration override(s) of AbstractContainerWidget - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them

/// java: private static class Slider extends AbstractSliderButton
pub struct Slider {
    /// java: extends net.minecraft.client.gui.components.AbstractSliderButton
    pub base: AbstractWidget,
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
    pub on_value_changed: Option<Arc<dyn Fn(f32)>>,
    /// java: private boolean clicked = false;
    pub clicked: bool,
}

impl std::ops::Deref for Slider {
    type Target = AbstractWidget;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for Slider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl Slider {
    /// java: private Slider(int x, int y, int width, float min, float max, float step, boolean linear, @Translatable String translatable, FloatConsumer onValueChanged)
    pub fn new(mut x: i32, mut y: i32, mut width: i32, mut min: f32, mut max: f32, mut step: f32, mut linear: bool, mut translatable: String, on_value_changed: Arc<dyn Fn(f32)>) -> compat::Shared<Self> {
        // TODO(fabric2seer): `new` takes 1 argument(s) in the port (`initial_value: f64`), Java passes 6
        //   java: super(x, y, width, 15, Component.empty(), 0);
        let __this: compat::Shared<Self> = todo!("constructor");
        if min >= max || step <= (0 as f32) || step > (max - min) {
            panic!("IllegalArgumentException: {}", format!("Invalid slider parameters: min={}, max={}, step={}", compat::fstr(min as f64), compat::fstr(max as f64), compat::fstr(step as f64)).to_string());
        }
        { let __v = min; compat::write(&__this, |__t| __t.min_value = __v); }
        { let __v = max; compat::write(&__this, |__t| __t.max_value = __v); }
        { let __v = step; compat::write(&__this, |__t| __t.step = __v); }
        { let __v = linear; compat::write(&__this, |__t| __t.linear = __v); }
        { let __v = translatable.clone(); compat::write(&__this, |__t| __t.translatable = __v); }
        { let __v = Some(on_value_changed.clone()); compat::write(&__this, |__t| __t.on_value_changed = __v); }
        Slider::update_message(&__this);
        __this
    }

    /// java: private float trueValue()
    pub fn true_value(__this: &compat::Shared<Self>) -> f32 {
        // TODO(fabric2seer): `AbstractSliderButton.value` has no Rust field in the port (searched net.minecraft.client.gui.components.AbstractSliderButton and its supertypes)
        // rust: let mut v: f64 = (if compat::read(__this, |__t| __t.linear) { compat::read(__this, |__t| __t.value) } else { (compat::read(__this, |__t| __t.value) * compat::read(__this, |__t| __t.value)) });
        let mut v: f64 = todo!("fabric2seer: the initialiser of `v` (TODO above)");
        return Slider::round_to_step(__this, (v * ((compat::read(__this, |__t| __t.max_value) - compat::read(__this, |__t| __t.min_value)) as f64)));
    }

    /// java: @Override protected void updateMessage()
    pub fn update_message(__this: &compat::Shared<Self>) {
        let __r0 = compat::read(__this, |__t| __t.translatable.clone());
        { let (__q0,) = (mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::translatable_args(&__r0, vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of(formatters::DOUBLE_NUMBERS.get_mut().format((Slider::true_value(__this)) as f64))])),); __this.lock().borrow_mut().set_message_text(__q0) };
    }

    /// java: private void setValue(float val)
    pub fn set_value(__this: &compat::Shared<Self>, mut val: f32) {
        let mut v: f32 = ((val - compat::read(__this, |__t| __t.min_value)) / (compat::read(__this, |__t| __t.max_value) - compat::read(__this, |__t| __t.min_value)));
        // TODO(fabric2seer): `AbstractSliderButton.value` has no Rust field in the port (searched net.minecraft.client.gui.components.AbstractSliderButton and its supertypes)
        // rust: { let __v = (if compat::read(__this, |__t| __t.linear) { (v) as f64 } else { (v as f64).sqrt() }); compat::write(__this, |__t| __t.value = __v); }
        Slider::update_message(__this);
    }

    /// java: @Override public void onClick(MouseButtonEvent click, boolean doubled)
    pub fn on_click(__this: &compat::Shared<Self>, click: &MouseButtonEvent, mut doubled: bool) {
        // TODO(fabric2seer): `on_click` takes 4 argument(s) in the port (`mouse_x: f64, x: i32, width: i32, active: bool`), Java passes 2
        // TODO(fabric2seer): `SliderState::on_click` ports `onClick`, but the port's `AbstractWidget` holds no `SliderState` (no Deref, no `base` field) to call it on
        // rust: { let (__q0, __q1,) = (click.clone(), doubled,); __this.lock().borrow_mut().base.on_click(__q0, __q1) };
        { let __v = true; compat::write(__this, |__t| __t.clicked = __v); }
    }

    /// java: @Override public void onRelease(MouseButtonEvent click)
    pub fn on_release(__this: &compat::Shared<Self>, click: &MouseButtonEvent) {
        // TODO(fabric2seer): `on_release` takes 0 argument(s) in the port (``), Java passes 1
        // TODO(fabric2seer): `SliderState::on_release` ports `onRelease`, but the port's `AbstractWidget` holds no `SliderState` (no Deref, no `base` field) to call it on
        // rust: { let (__q0,) = (click.clone(),); __this.lock().borrow_mut().base.on_release(__q0) };
        if compat::read(__this, |__t| __t.clicked) {
            ((compat::read(__this, |__t| __t.on_value_changed.clone()).unwrap()))(Slider::true_value(__this));
            { let __v = false; compat::write(__this, |__t| __t.clicked = __v); }
        }
    }

    /// java: @Override public boolean keyPressed(KeyEvent input)
    pub fn key_pressed(__this: &compat::Shared<Self>, input: &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        // TODO(fabric2seer): `key_pressed` takes 3 argument(s) in the port (`_key: i32, _scancode: i32, _modifiers: i32`), Java passes 1
        // TODO(fabric2seer): `key_pressed` is a method of the port's trait `GuiEventListener`, which the port's `AbstractWidget` does not implement
        if { let (__q0,) = (input.clone(),); __this.lock().borrow_mut().base.key_pressed(__q0) } {
            ((compat::read(__this, |__t| __t.on_value_changed.clone()).unwrap()))(Slider::true_value(__this));
            return true;
        }
        return false;
    }

    /// java: @Override public boolean mouseScrolled(double mouseX, double mouseY, double horizontalAmount, double verticalAmount)
    pub fn mouse_scrolled(__this: &compat::Shared<Self>, mut mouse_x: f64, mut mouse_y: f64, mut horizontal_amount: f64, mut vertical_amount: f64) -> bool {
        if vertical_amount == (0 as f64) {
            return false;
        }
        let mut offset: f32 = (if vertical_amount > (0 as f64) { compat::read(__this, |__t| __t.step) } else { (-compat::read(__this, |__t| __t.step)) });
        Slider::set_value(__this, (Slider::true_value(__this) + offset).clamp(compat::read(__this, |__t| __t.min_value), compat::read(__this, |__t| __t.max_value)));
        ((compat::read(__this, |__t| __t.on_value_changed.clone()).unwrap()))(Slider::true_value(__this));
        return true;
    }

    /// java: @Override protected void applyValue()
    pub fn apply_value(__this: &compat::Shared<Self>) {
    }

    /// java: private float roundToStep(double value)
    pub fn round_to_step(__this: &compat::Shared<Self>, mut value: f64) -> f32 {
        return (compat::read(__this, |__t| __t.min_value) + (compat::read(__this, |__t| __t.step) * (compat::java_round_f64((value / (compat::read(__this, |__t| __t.step) as f64))) as f32))).clamp(compat::read(__this, |__t| __t.min_value), compat::read(__this, |__t| __t.max_value));
    }

}

// NOTE(fabric2seer): applyValue, keyPressed, mouseScrolled, onClick, onRelease, updateMessage override(s) of AbstractSliderButton - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them
