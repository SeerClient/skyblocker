//! Ported from de.hysky.skyblocker.skyblock.item.custom.screen.TrimSelectionWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::item::custom::custom_armor_trims::ArmorTrimId;
use crate::de::hysky::skyblocker::skyblock::item::custom::screen::trim_element_button::Material;
use crate::de::hysky::skyblocker::skyblock::item::custom::screen::trim_element_button::Pattern;
use crate::de::hysky::skyblocker::skyblock::item::custom::screen::trim_element_button::TrimElementButton;
use crate::de::hysky::skyblocker::utils::registry_utils::RegistryUtils;
use mcrust::mcrust::modding::widgets::WidgetRenderContext;
use mcrust::net::minecraft::client::gui::components::abstract_container_widget::AbstractContainerWidget;
use mcrust::net::minecraft::client::gui::components::abstract_scroll_area::ScrollArea;
use mcrust::net::minecraft::client::gui::components::abstract_scroll_area::ScrollbarSettings;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::layouts::frame_layout::FrameLayout;
use mcrust::net::minecraft::client::gui::layouts::grid_layout::GridLayout;
use mcrust::net::minecraft::client::gui::layouts::grid_layout::RowHelper;
use mcrust::net::minecraft::client::gui::layouts::layout_element::LayoutElement;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::formatted_text::FormattedText;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::world::item::equipment::trim::trim_material::TrimMaterial;
use mcrust::net::minecraft::world::item::equipment::trim::trim_pattern::TrimPattern;
use crate::de::hysky::skyblocker::injected::skyblocker_stack::SkyblockerStack as _;
use mcrust::net::minecraft::core::holder_lookup::HolderLookup as _;
use mcrust::net::minecraft::core::holder_lookup::Provider as _;

/// java: private static final int PADDING = 3;
pub const PADDING: i32 = 3;
/// java: private static final Identifier INNER_SPACE_TEXTURE = SkyblockerMod.id("menu_inner_space");
pub static INNER_SPACE_TEXTURE: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("menu_inner_space"));
/// java: private static final int MAX_BUTTONS_PER_ROW_PATTERN = 7;
pub const MAX_BUTTONS_PER_ROW_PATTERN: i32 = 7;
/// java: private static final int MAX_BUTTONS_PER_ROW_MATERIAL = 6;
pub const MAX_BUTTONS_PER_ROW_MATERIAL: i32 = 6;

/// A button's `onPress` (`this::onClickPattern` / `this::onClickMaterial`): the button calls back while this widget
/// is handing it the click, so the press is recorded here and run as the click returns (`run_presses`).
enum TrimPress {
    Pattern(compat::Shared<dyn TrimElementButton>),
    Material(compat::Shared<dyn TrimElementButton>),
}

/// java: public class TrimSelectionWidget extends AbstractContainerWidget
pub struct TrimSelectionWidget {
    /// java: extends net.minecraft.client.gui.components.AbstractContainerWidget
    pub base: AbstractContainerWidget,
    /// `AbstractScrollArea`'s scroll state (`AbstractScrollArea.defaultSettings(8)`)
    pub scroll: ScrollArea,
    /// java: private final List<TrimElementButton.Pattern> patternButtons = new ArrayList<>();
    pub pattern_buttons: Vec<compat::Shared<Pattern>>,
    /// java: private final List<TrimElementButton> materialButtons = new ArrayList<>();
    pub material_buttons: Vec<compat::Shared<dyn TrimElementButton>>,
    /// java: private FrameLayout layout;
    pub layout: LayoutElement,
    /// java: private final List<AbstractWidget> children = new ArrayList<>(); - the pattern buttons, then the material
    /// buttons, as the game's widgets
    pub children: mcrust::net::minecraft::client::gui::screens::screen::Screen,
    /// java: private ItemStack currentItem = null;
    pub current_item: Option<mcrust::net::minecraft::world::item::item_stack::ItemStack>,
    /// java: private Identifier selectedPattern = null;
    pub selected_pattern: Option<Identifier>,
    /// java: private Identifier selectedMaterial = null;
    pub selected_material: Option<Identifier>,
    /// the buttons' presses during this widget's own click (see `TrimPress`)
    presses: std::rc::Rc<std::cell::RefCell<Vec<TrimPress>>>,
}

impl TrimSelectionWidget {
    /// java: public TrimSelectionWidget(int x, int y, int width, int height)
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> compat::Shared<Self> {
        // java: super(x, y, width, height, Component.nullToEmpty("Trim Selection"), AbstractScrollArea.defaultSettings(8));
        let settings = ScrollbarSettings::default_settings(8.0);
        let mut this = TrimSelectionWidget {
            base: AbstractContainerWidget::new_with_scrollbar_settings(x, y, width, height, FormattedText::of("Trim Selection"), settings),
            scroll: ScrollArea::new(y, height, settings),
            pattern_buttons: Vec::new(),
            material_buttons: Vec::new(),
            layout: LayoutElement::Frame(FrameLayout::new()),
            children: mcrust::net::minecraft::client::gui::screens::screen::Screen::new(String::new()),
            current_item: None,
            selected_pattern: None,
            selected_material: None,
            presses: std::rc::Rc::new(std::cell::RefCell::new(Vec::new())),
        };

        // Patterns
        let presses = this.presses.clone();
        let on_click_pattern: Arc<dyn Fn(compat::Shared<dyn TrimElementButton>)> = Arc::new(move |button: compat::Shared<dyn TrimElementButton>| presses.borrow_mut().push(TrimPress::Pattern(button)));
        let pattern_none_button: compat::Shared<Pattern> = Pattern::new(None, None, on_click_pattern.clone());
        ({ let __f = compat::dispatch(&pattern_none_button, |__t| __t.set_message()); __f(&compat::view(&pattern_none_button, |a| a as compat::Shared<dyn TrimElementButton>), &component::translatable("gui.none")) });
        this.pattern_buttons.push(pattern_none_button);

        let lookup = RegistryUtils::get_registry_wrapper_lookup();
        let mut patterns: Vec<(Identifier, TrimPattern)> = lookup
            .lookup_registry_or_throw(&mcrust::net::minecraft::core::registries::registries::trim_pattern())
            .expect("Registries.TRIM_PATTERN")
            .list_elements()
            .into_iter()
            .filter_map(|reference| mcrust::com::mojang::serialization::cast::<TrimPattern>(&reference.value()).cloned().map(|pattern| (reference.key().identifier().clone(), pattern)))
            .collect();
        // Sort them in alphabetical order
        patterns.sort_by(|a, b| compat::compare_to(&a.1.description().get_string(), &b.1.description().get_string()).cmp(&0));
        for (identifier, pattern) in patterns {
            this.pattern_buttons.push(Pattern::new(Some(identifier), Some(pattern), on_click_pattern.clone()));
        }
        // java: children.addAll(patternButtons);
        for button in this.pattern_buttons.iter() {
            this.children.add_renderable_widget(compat::WidgetAdapter::widget(button.clone()));
        }

        // Materials
        let presses = this.presses.clone();
        let on_click_material: Arc<dyn Fn(compat::Shared<dyn TrimElementButton>)> = Arc::new(move |button: compat::Shared<dyn TrimElementButton>| presses.borrow_mut().push(TrimPress::Material(button)));
        let mut materials: Vec<(Identifier, TrimMaterial)> = lookup
            .lookup_registry_or_throw(&mcrust::net::minecraft::core::registries::registries::trim_material())
            .expect("Registries.TRIM_MATERIAL")
            .list_elements()
            .into_iter()
            .filter_map(|reference| mcrust::com::mojang::serialization::cast::<TrimMaterial>(&reference.value()).cloned().map(|material| (reference.key().identifier().clone(), material)))
            .collect();
        // Sort them in alphabetical order
        materials.sort_by(|a, b| compat::compare_to(&a.1.description().get_string(), &b.1.description().get_string()).cmp(&0));
        for (identifier, material) in materials {
            let button = Material::new(identifier, material, on_click_material.clone());
            this.children.add_renderable_widget(compat::WidgetAdapter::widget(button.clone()));
            this.material_buttons.push(button as compat::Shared<dyn TrimElementButton>);
        }

        this.position_buttons(width, height);
        this.layout.set_position(x + PADDING, y + PADDING);
        this.place_children();
        compat::shared(this)
    }

    /// The layout's positions written onto the buttons (Java's layouts move the widgets themselves).
    fn place_children(&mut self) {
        let mut placed = Vec::new();
        self.layout.visit_widgets(&mut placed);
        for (index, x, y) in placed {
            self.children.widget_mut(index).set_position(x, y);
        }
    }

    /// java: private void positionButtons(int width, int height)
    pub fn position_buttons(&mut self, width: i32, height: i32) {
        // Layout button again and accommodate for the scrollbar if overflows
        // We need to do this since overflows always return false during the first calculation since the buttons per row hasn't been calculated yet

        // Calculate buttons per row
        // minus 9 because 3 pixels of left padding, right padding, and gap
        let mut buttons_per_row: i32 = (width - 9) / 20;
        // Try to allocate more buttons to patterns since there are more patterns than materials
        // TODO: Clamping these alone does not fix crash at small screen widths
        let mut pattern_buttons_per_row: i32 = ceil_div(buttons_per_row, 2).min(MAX_BUTTONS_PER_ROW_PATTERN);
        let mut material_buttons_per_row: i32 = compat::floor_div_i32(buttons_per_row, 2).min(MAX_BUTTONS_PER_ROW_MATERIAL);

        let max_height: i32 = self.base.get_height() - PADDING * 2;
        let overflow: bool = ((self.pattern_buttons.len() as i32) / pattern_buttons_per_row + 1) * 20 > max_height || ((self.material_buttons.len() as i32) / material_buttons_per_row + 1) * 20 > max_height;
        if overflow {
            // subtract 6 extra pixels for the scrollbar
            buttons_per_row = (width - 15) / 20;
            pattern_buttons_per_row = ceil_div(buttons_per_row, 2).min(MAX_BUTTONS_PER_ROW_PATTERN);
            material_buttons_per_row = compat::floor_div_i32(buttons_per_row, 2).min(MAX_BUTTONS_PER_ROW_MATERIAL);
        }

        // Set button positions
        let mut patterns_grid: GridLayout = GridLayout::new_default();
        let mut pattern_adder: RowHelper = patterns_grid.create_row_helper(pattern_buttons_per_row);
        // java: patternButtons.forEach(patternAdder::addChild); - the pattern buttons are children 0..
        for i in 0..self.pattern_buttons.len() {
            pattern_adder.add_child(&mut patterns_grid, LayoutElement::widget(i, 20, 20)).expect("GridLayout.addChild");
        }

        let mut materials_grid: GridLayout = GridLayout::new_default();
        let first_material = self.pattern_buttons.len();
        for i in 0..(self.material_buttons.len() as i32) {
            let row: i32 = i / material_buttons_per_row;
            let column: i32 = material_buttons_per_row - (i % material_buttons_per_row) - 1;
            materials_grid.add_child_at(LayoutElement::widget(first_material + i as usize, 20, 20), row, column).expect("GridLayout.addChild");
        }

        let mut layout = FrameLayout::of(width - PADDING * 2 - (if overflow { 6 } else { 0 }), height - PADDING * 2);
        // java: layout.defaultChildLayoutSetting().alignVerticallyTop(); - on the layout's own settings
        let settings = layout.default_child_layout_setting();
        *settings = settings.clone().align_vertically_top();
        layout.add_child_adjusted(LayoutElement::Grid(patterns_grid), &mut |s| *s = s.clone().align_horizontally_left());
        layout.add_child_adjusted(LayoutElement::Grid(materials_grid), &mut |s| *s = s.clone().align_horizontally_right());
        let mut layout = LayoutElement::Frame(layout);
        layout.arrange_elements();
        self.layout = layout;
    }

    /// java: private void onClickPattern(TrimElementButton button)
    pub fn on_click_pattern(&mut self, button: &compat::Shared<dyn TrimElementButton>) {
        for i in 0..self.pattern_buttons.len() {
            self.children.widget_mut(i).active = true;
        }
        if let Some(index) = self.pattern_buttons.iter().position(|b| std::ptr::addr_eq(Arc::as_ptr(b), Arc::as_ptr(button))) {
            self.children.widget_mut(index).active = false;
        }
        self.selected_pattern = ({ let __f = compat::dispatch(button, |__t| __t.get_element()); __f(button) });
        self.update_config();
    }

    /// java: private void onClickMaterial(TrimElementButton button)
    pub fn on_click_material(&mut self, button: &compat::Shared<dyn TrimElementButton>) {
        let first_material = self.pattern_buttons.len();
        for i in 0..self.material_buttons.len() {
            self.children.widget_mut(first_material + i).active = true;
        }
        if let Some(index) = self.material_buttons.iter().position(|b| std::ptr::addr_eq(Arc::as_ptr(b), Arc::as_ptr(button))) {
            self.children.widget_mut(first_material + index).active = false;
        }
        self.selected_material = ({ let __f = compat::dispatch(button, |__t| __t.get_element()); __f(button) });
        self.update_config();
    }

    /// The presses the buttons recorded during this widget's click, in order.
    fn run_presses(&mut self) {
        let presses: Vec<TrimPress> = std::mem::take(&mut *self.presses.borrow_mut());
        for press in presses {
            match press {
                TrimPress::Pattern(button) => self.on_click_pattern(&button),
                TrimPress::Material(button) => self.on_click_material(&button),
            }
        }
    }

    /// java: private void updateConfig()
    pub fn update_config(&self) {
        let Some(current_item) = self.current_item.as_ref() else {
            return;
        };
        let item_uuid: String = current_item.get_uuid();
        let selected_pattern = self.selected_pattern.clone();
        let selected_material = self.selected_material.clone();
        SkyblockerConfigManager::update_only(Arc::new(move |config: &mut crate::de::hysky::skyblocker::config::skyblocker_config::SkyblockerConfig| {
            let trims = &mut config.general.custom_armor_trims;
            match &selected_pattern {
                None => {
                    trims.remove(&item_uuid);
                }
                Some(pattern) => {
                    trims.insert(item_uuid.clone(), ArmorTrimId::new(selected_material.clone().expect("TrimSelectionWidget.selectedMaterial"), pattern.clone()));
                }
            }
        }));
    }

    /// java: @Override protected int contentHeight()
    pub fn content_height(&self) -> i32 {
        // 3 pixels of padding on top and bottom
        return self.layout.get_height() + PADDING * 2;
    }

    /// java: @Override protected double scrollRate()
    pub fn scroll_rate(&self) -> f64 {
        return 10.0;
    }

    /// `AbstractScrollArea.scrollBarX()` - `getRight() - scrollbarWidth()`
    fn scroll_bar_x(&self) -> i32 {
        self.base.get_right() - self.scroll.scrollbar_width()
    }

    /// java: public void setCurrentItem(ItemStack currentItem)
    pub fn set_current_item(&mut self, current_item: mcrust::net::minecraft::world::item::item_stack::ItemStack) {
        self.current_item = Some(current_item.clone());
        let trims = &SkyblockerConfigManager::get().general.custom_armor_trims;
        let item_uuid: String = current_item.get_uuid();
        for button in self.pattern_buttons.iter() {
            Pattern::set_stack(button, current_item.clone());
        }
        let first_material = self.pattern_buttons.len();
        match trims.get(&item_uuid) {
            None => {
                self.selected_pattern = None;
                self.selected_material = ({ let __r = &self.material_buttons[0]; let __f = compat::dispatch(__r, |__t| __t.get_element()); __f(__r) });
                for i in 0..self.material_buttons.len() {
                    self.children.widget_mut(first_material + i).active = i != 0;
                }
                for i in 0..self.pattern_buttons.len() {
                    self.children.widget_mut(i).active = i != 0;
                }
            }
            Some(id) => {
                self.selected_material = Some(id.material());
                self.selected_pattern = Some(id.pattern());
                for i in 0..self.material_buttons.len() {
                    let element = ({ let __r = &self.material_buttons[i]; let __f = compat::dispatch(__r, |__t| __t.get_element()); __f(__r) });
                    // java: !selectedMaterial.equals(materialButton.getElement())
                    self.children.widget_mut(first_material + i).active = self.selected_material != element;
                }
                for i in 0..self.pattern_buttons.len() {
                    let element = compat::read(&self.pattern_buttons[i], |__t| __t.base.element.clone());
                    self.children.widget_mut(i).active = self.selected_pattern != element;
                }
            }
        }
    }
}

/// `Math.ceilDiv(x, y)`.
fn ceil_div(x: i32, y: i32) -> i32 {
    -compat::floor_div_i32(-x, y)
}

impl compat::ObjectWidget for TrimSelectionWidget {
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
        self.scroll.y = self.base.get_y();
        self.layout.set_y(self.base.get_y() + PADDING);
        self.place_children();
    }

    /// java: @Override public void setWidth(int width)
    fn set_width(&mut self, width: i32) {
        self.base.set_width(width);
        self.position_buttons(self.base.get_width(), self.base.get_height());
        // (Java's new layout sits at 0, 0 until the parent layout's `setX`/`setY`; the port's only re-runs those on
        // a move, so the new layout takes this widget's position here)
        self.layout.set_position(self.base.get_x() + PADDING, self.base.get_y() + PADDING);
        self.place_children();
    }

    /// java: @Override public void setHeight(int height)
    fn set_height(&mut self, height: i32) {
        self.base.set_height(height);
        self.scroll.height = self.base.get_height();
        self.position_buttons(self.base.get_width(), self.base.get_height());
        self.layout.set_position(self.base.get_x() + PADDING, self.base.get_y() + PADDING);
        self.place_children();
    }

    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_widget_render_state(&mut self, _widget: &AbstractWidget, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, _a: f32) {
        let (x, y, w, h) = (self.base.get_x(), self.base.get_y(), self.base.get_width(), self.base.get_height());
        gui_graphics_extractor::blit_sprite_identifier(context.gl, context.minecraft, mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), INNER_SPACE_TEXTURE.get(), x, y, w, h);
        context.graphics.enable_scissor(x + 2, y + 2, x + w - 2, y + h - 2);

        let scroll_y: i32 = self.scroll.scroll_amount() as i32;
        for i in 0..self.children.children().len() {
            let widget = self.children.widget_mut(i);
            widget.set_y(widget.get_y() - scroll_y);
        }
        // java: widget.extractRenderState(graphics, mouseX, mouseY, a);
        context.extract_widgets(&mut self.children, mouse_x, mouse_y);
        for i in 0..self.children.children().len() {
            let widget = self.children.widget_mut(i);
            widget.set_y(widget.get_y() + scroll_y);
        }

        // java: extractScrollbar(graphics, mouseX, mouseY);
        compat::extract_scrollbar(context, &self.scroll, self.content_height(), self.scroll_bar_x());
        context.graphics.disable_scissor();
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    fn mouse_clicked(&mut self, _widget: &mut AbstractWidget, event: &MouseButtonEvent, _double_click: bool) -> bool {
        // java: return super.mouseClicked(new MouseButtonEvent(click.x(), click.y() + this.scrollAmount(), click.buttonInfo()), doubled);
        let event = MouseButtonEvent::new(event.x(), event.y() + self.scroll.scroll_amount(), event.button_info());
        // AbstractContainerWidget.mouseClicked:62-66 - the scrollbar first, then the children
        let content_height = self.content_height();
        let scroll_bar_x = self.scroll_bar_x();
        let scrolling = self.scroll.update_scrolling(content_height, scroll_bar_x, event.button(), event.x(), event.y());
        let consumed = compat::children_mouse_clicked(&mut self.children, &event);
        self.run_presses();
        consumed || scrolling
    }

    /// AbstractContainerWidget.mouseReleased - the scrollbar lets go, then the children
    fn mouse_released(&mut self, _widget: &mut AbstractWidget, event: &MouseButtonEvent) -> bool {
        self.scroll.on_release();
        compat::children_mouse_released(&mut self.children, event);
        self.run_presses();
        false
    }

    /// AbstractContainerWidget.mouseDragged - the scrollbar, else the focused child
    fn mouse_dragged(&mut self, _widget: &mut AbstractWidget, event: &MouseButtonEvent, dx: f64, dy: f64) -> bool {
        let content_height = self.content_height();
        if self.scroll.mouse_dragged(content_height, event.y(), dy) {
            return true;
        }
        compat::children_mouse_dragged(&mut self.children, event, dx, dy);
        false
    }

    /// AbstractScrollArea.mouseScrolled:27-35 - `scrollAmount - scrollY * scrollRate()`
    fn mouse_scrolled(&mut self, _widget: &mut AbstractWidget, _x: f64, _y: f64, _scroll_x: f64, scroll_y: f64) -> bool {
        if !self.base.visible {
            return false;
        }
        let content_height = self.content_height();
        let target = self.scroll.scroll_amount() - scroll_y * self.scroll_rate();
        self.scroll.set_scroll_amount(content_height, target);
        true
    }

    /// `ContainerEventHandler.keyPressed` - the focused child
    fn key_pressed(&mut self, _widget: &mut AbstractWidget, event: &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        let answer = compat::children_key_pressed(&mut self.children, event);
        self.run_presses();
        answer
    }
}
