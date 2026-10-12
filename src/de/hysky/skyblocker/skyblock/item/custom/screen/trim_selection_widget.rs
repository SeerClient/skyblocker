//! Ported from de.hysky.skyblocker.skyblock.item.custom.screen.TrimSelectionWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::item::custom::custom_armor_trims::ArmorTrimId;
use crate::de::hysky::skyblocker::skyblock::item::custom::screen::trim_element_button::TrimElementButton;
use crate::de::hysky::skyblocker::utils::registry_utils::RegistryUtils;
use mcrust::net::minecraft::client::gui::components::abstract_container_widget::AbstractContainerWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::events::gui_event_listener::GuiEventListener;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::layouts::frame_layout::FrameLayout;
use mcrust::net::minecraft::client::gui::layouts::grid_layout::GridLayout;
use mcrust::net::minecraft::client::gui::layouts::grid_layout::RowHelper;
use mcrust::net::minecraft::client::gui::layouts::layout_element::LayoutElement;
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::resources::identifier::Identifier;
use crate::de::hysky::skyblocker::injected::skyblocker_stack::SkyblockerStack as _;
use mcrust::net::minecraft::client::gui::components::renderable::Renderable as _;
use mcrust::net::minecraft::client::gui::layouts::abstract_layout::AbstractLayout as _;
use mcrust::net::minecraft::core::holder_lookup::HolderLookup as _;

/// java: private static final int PADDING = 3;
pub const PADDING: i32 = 3;
/// java: private static final Identifier INNER_SPACE_TEXTURE = SkyblockerMod.id("menu_inner_space");
pub static INNER_SPACE_TEXTURE: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("menu_inner_space"));
/// java: private static final int MAX_BUTTONS_PER_ROW_PATTERN = 7;
pub const MAX_BUTTONS_PER_ROW_PATTERN: i32 = 7;
/// java: private static final int MAX_BUTTONS_PER_ROW_MATERIAL = 6;
pub const MAX_BUTTONS_PER_ROW_MATERIAL: i32 = 6;

/// java: public class TrimSelectionWidget extends AbstractContainerWidget
#[derive(Clone)]
pub struct TrimSelectionWidget {
    /// java: extends net.minecraft.client.gui.components.AbstractContainerWidget
    pub base: AbstractContainerWidget,
    /// java: private final List<TrimElementButton.Pattern> patternButtons = new ArrayList<>();
    pub pattern_buttons: Vec<compat::regex::Pattern>,
    /// java: private final List<TrimElementButton> materialButtons = new ArrayList<>();
    pub material_buttons: Vec<compat::Shared<dyn TrimElementButton>>,
    /// java: private FrameLayout layout;
    pub layout: Option<FrameLayout>,
    /// java: private final List<AbstractWidget> children = new ArrayList<>();
    pub children: Vec<AbstractWidget>,
    /// java: private ItemStack currentItem = null;
    pub current_item: Option<mcrust::net::minecraft::world::item::item_stack::ItemStack>,
    /// java: private Identifier selectedPattern = null;
    pub selected_pattern: Option<Identifier>,
    /// java: private Identifier selectedMaterial = null;
    pub selected_material: Option<Identifier>,
}

impl std::ops::Deref for TrimSelectionWidget {
    type Target = AbstractContainerWidget;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for TrimSelectionWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl TrimSelectionWidget {
    /// java: public TrimSelectionWidget(int x, int y, int width, int height)
    pub fn new(mut x: i32, mut y: i32, mut width: i32, mut height: i32) -> Self {
        // TODO(fabric2seer): static `AbstractScrollArea.defaultSettings` has no Rust counterpart in the port
        //   java: super(x, y, width, height, Component.nullToEmpty("Trim Selection"), AbstractScrollArea.defaultSettings(8));
        let mut this: Self = todo!("constructor");
        let mut pattern_none_button: compat::regex::Pattern = crate::de::hysky::skyblocker::skyblock::item::custom::screen::trim_element_button::Pattern::new(None, None, Arc::new(move |__a0| { this.on_click_pattern(&__a0); }));
        ({ let __f = compat::dispatch(&pattern_none_button, |__t| __t.set_message()); __f(&compat::view(&pattern_none_button, |a| a as compat::Shared<dyn TrimElementButton>), &component::translatable("gui.none")) });
        this.pattern_buttons.push(pattern_none_button.clone());
        // TODO(fabric2seer): `.description(..)` on a value of unknown type
        //   java: RegistryUtils.getRegistryWrapperLookup().lookupOrThrow(Registries.TRIM_PATTERN).listElements()
        //   java: 				// Sort them in alphabetical order
        //   java: 				.sorted(Comparator.comparing(reference -> reference.value().description().getString()))
        //   java: 				.map(reference -> new TrimElementButton.Pattern(
        //   java: 						reference.key().identifier(),
        //   java: 						reference.value(),
        //   java: 						this::onClickPattern
        //   java: 				)).forEachOrdered(patternButtons::add);
        this.children.extend(this.pattern_buttons.iter().cloned());
        // TODO(fabric2seer): `.description(..)` on a value of unknown type
        //   java: RegistryUtils.getRegistryWrapperLookup().lookupOrThrow(Registries.TRIM_MATERIAL).listElements()
        //   java: 				// Sort them in alphabetical order
        //   java: 				.sorted(Comparator.comparing(reference -> reference.value().description().getString()))
        //   java: 				.map(reference -> new TrimElementButton.Material(
        //   java: 						reference.key().identifier(),
        //   java: 						reference.value(),
        //   java: 						this::onClickMaterial
        //   java: 				)).forEachOrdered(materialButtons::add);
        this.children.extend(this.material_buttons.iter().cloned());
        this.position_buttons(width, height);
        this.layout.as_mut().unwrap().set_position(x.wrapping_add(PADDING), y.wrapping_add(PADDING));
        this
    }

    /// java: private void positionButtons(int width, int height)
    pub fn position_buttons(&mut self, mut width: i32, mut height: i32) {
        let mut buttons_per_row: i32 = (width.wrapping_sub(9)).wrapping_div(20);
        // TODO(fabric2seer): Math.ceilDiv
        //   java: int patternButtonsPerRow = Math.min(Math.ceilDiv(buttonsPerRow, 2), MAX_BUTTONS_PER_ROW_PATTERN);
        let mut pattern_buttons_per_row: i32 = todo!("fabric2seer: the initialiser of `patternButtonsPerRow` (TODO above)");
        let mut material_buttons_per_row: i32 = (compat::floor_div_i32(buttons_per_row, 2)).min(MAX_BUTTONS_PER_ROW_MATERIAL);
        let mut max_height: i32 = self.get_height().wrapping_sub(PADDING.wrapping_mul(2));
        let mut overflow: bool = ((((self.pattern_buttons.len() as i32).wrapping_div(pattern_buttons_per_row)).wrapping_add(1)).wrapping_mul(20) > max_height || (((self.material_buttons.len() as i32).wrapping_div(material_buttons_per_row)).wrapping_add(1)).wrapping_mul(20) > max_height);
        if overflow {
            buttons_per_row = (width.wrapping_sub(15)).wrapping_div(20);
            // TODO(fabric2seer): Math.ceilDiv
            //   java: patternButtonsPerRow = Math.min(Math.ceilDiv(buttonsPerRow, 2), MAX_BUTTONS_PER_ROW_PATTERN);
            material_buttons_per_row = (compat::floor_div_i32(buttons_per_row, 2)).min(MAX_BUTTONS_PER_ROW_MATERIAL);
        }
        let mut patterns_grid: GridLayout = GridLayout::new_default();
        let mut pattern_adder: RowHelper = patterns_grid.create_row_helper(pattern_buttons_per_row);
        for __a0 in self.pattern_buttons.iter() {
            // TODO(fabric2seer): the port's `add_child` ports this Java method with other parameters (grid, child for Java's widget)
            // TODO(fabric2seer): `add_child` takes 2 argument(s) in the port (`grid: &mut GridLayout, child: LayoutElement`), Java passes 1
            // rust: pattern_adder.add_child(&mut __a0).unwrap();
        }
        let mut materials_grid: GridLayout = GridLayout::new_default();
        for i in 0..(self.material_buttons.len() as i32) {
            let mut button: compat::Shared<dyn TrimElementButton> = self.material_buttons[(i) as usize].clone();
            let mut row: i32 = i.wrapping_div(material_buttons_per_row);
            let mut column: i32 = (material_buttons_per_row.wrapping_sub(i.wrapping_rem(material_buttons_per_row))).wrapping_sub(1);
            materials_grid.add_child_at(button.clone(), row, column).unwrap();
        }
        self.layout = Some(FrameLayout::of((width.wrapping_sub(PADDING.wrapping_mul(2))).wrapping_sub(if overflow { 6 } else { 0 }), height.wrapping_sub(PADDING.wrapping_mul(2))));
        self.layout.as_mut().unwrap().default_child_layout_setting().align_vertically_top();
        self.layout.as_mut().unwrap().add_child_adjusted(LayoutElement::Grid(patterns_grid.clone()), &mut |__a0| { __a0.align_horizontally_left(); });
        self.layout.as_mut().unwrap().add_child_adjusted(LayoutElement::Grid(materials_grid.clone()), &mut |__a0| { __a0.align_horizontally_right(); });
        self.layout.as_mut().unwrap().arrange_elements();
    }

    /// java: private void onClickPattern(TrimElementButton button)
    pub fn on_click_pattern(&mut self, button: &compat::Shared<dyn TrimElementButton>) {
        for pattern_button in self.pattern_buttons.iter_mut() {
            { let __v = true; compat::write(pattern_button, |__t| __t.trim_element_button_base_mut().active = __v); }
        }
        { let __v = false; compat::write(button, |__t| __t.trim_element_button_base_mut().active = __v); }
        self.selected_pattern = ({ let __f = compat::dispatch(button, |__t| __t.get_element()); __f(button) });
        self.update_config();
    }

    /// java: private void onClickMaterial(TrimElementButton button)
    pub fn on_click_material(&mut self, button: &compat::Shared<dyn TrimElementButton>) {
        for material_button in self.material_buttons.iter_mut() {
            { let __v = true; compat::write(material_button, |__t| __t.trim_element_button_base_mut().active = __v); }
        }
        { let __v = false; compat::write(button, |__t| __t.trim_element_button_base_mut().active = __v); }
        self.selected_material = ({ let __f = compat::dispatch(button, |__t| __t.get_element()); __f(button) });
        self.update_config();
    }

    /// java: private void updateConfig()
    pub fn update_config(&self) {
        if self.current_item.is_none() {
            return;
        }
        SkyblockerConfigManager::update_only(Arc::new(move |config| {
            let trims = &mut config.general.custom_armor_trims;
            let mut item_uuid: String = self.current_item.as_ref().unwrap().get_uuid();
            if self.selected_pattern.is_none() {
                trims.remove(&item_uuid);
            } else {
                trims.insert(item_uuid.clone(), ArmorTrimId::new(self.selected_material.clone().unwrap(), self.selected_pattern.clone().unwrap()));
            }
        }));
    }

    /// java: @Override public List<? extends GuiEventListener> children()
    pub fn children(&self) -> Vec<Box<dyn GuiEventListener>> {
        return self.children.clone();
    }

    /// java: @Override protected int contentHeight()
    pub fn content_height(&self) -> i32 {
        return (self.layout.as_ref().unwrap().get_height()).wrapping_add(PADDING.wrapping_mul(2));
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
        { let __s0 = self.get_width(); let __s1 = self.get_height(); self.position_buttons(__s0, __s1) };
    }

    /// java: @Override public void setHeight(int height)
    pub fn set_height(&mut self, mut height: i32) {
        self.base.set_height(height);
        { let __s0 = self.get_width(); let __s1 = self.get_height(); self.position_buttons(__s0, __s1) };
    }

    /// java: @Override protected double scrollRate()
    pub fn scroll_rate(&self) -> f64 {
        return ((10) as f64) as f64;
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    pub fn mouse_clicked(&self, click: &MouseButtonEvent, mut doubled: bool) -> bool {
        // TODO(fabric2seer): `ScrollArea::scroll_amount` ports `scrollAmount`, but the port's `AbstractContainerWidget` holds no `ScrollArea` (no Deref, no `base` field) to call it on
        // rust: return { let __s0 = MouseButtonEvent::new(click.x(), (click.y() + self.scroll_amount()), click.button_info()); self.base.mouse_clicked(__s0, doubled) };
        return todo!("fabric2seer: see the TODO above");
    }

    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_widget_render_state(&self, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        gui_graphics_extractor::blit_sprite_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), INNER_SPACE_TEXTURE.get_mut(), self.get_x(), self.get_y(), self.get_width(), self.get_height());
        graphics.enable_scissor(self.get_x().wrapping_add(2), self.get_y().wrapping_add(2), (self.get_x().wrapping_add(self.get_width())).wrapping_sub(2), (self.get_y().wrapping_add(self.get_height())).wrapping_sub(2));
        // TODO(fabric2seer): `ScrollArea::scroll_amount` ports `scrollAmount`, but the port's `AbstractContainerWidget` holds no `ScrollArea` (no Deref, no `base` field) to call it on
        // rust: let mut scroll_y: i32 = ((self.scroll_amount()) as i32);
        let mut scroll_y: i32 = todo!("fabric2seer: the initialiser of `scrollY` (TODO above)");
        for widget in self.children.iter_mut() {
            widget.set_y(widget.get_y().wrapping_sub(scroll_y));
            // TODO(fabric2seer): `extract_render_state` is a method of the port's trait `Renderable`, which the port's `AbstractWidget` does not implement
            // rust: widget.extract_render_state(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mouse_x, mouse_y, a);
            widget.set_y(widget.get_y().wrapping_add(scroll_y));
        }
        // TODO(fabric2seer): `AbstractContainerWidget.extractScrollbar` has no Rust method in the port (looked for `extract_scrollbar` in AbstractContainerWidget and its supertypes)
        // rust: self.extract_scrollbar(graphics, mouse_x, mouse_y);
        graphics.disable_scissor();
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    pub fn update_widget_narration(&self, builder: &dyn NarrationElementOutput) {
    }

    /// java: public void setCurrentItem(ItemStack currentItem)
    pub fn set_current_item(&mut self, mut current_item: mcrust::net::minecraft::world::item::item_stack::ItemStack) {
        self.current_item = Some(current_item.clone());
        let trims = &SkyblockerConfigManager::get().general.custom_armor_trims;
        let mut item_uuid: String = current_item.get_uuid();
        for button in self.pattern_buttons.iter_mut() {
            crate::de::hysky::skyblocker::skyblock::item::custom::screen::trim_element_button::Pattern::set_stack(button, current_item.clone());
        }
        if !(trims.contains_key(&item_uuid)) {
            self.selected_pattern = None;
            self.selected_material = ({ let __r = &self.material_buttons[0]; let __f = compat::dispatch(__r, |__t| __t.get_element()); __f(__r) });
            for i in 0..(self.material_buttons.len() as i32) {
                { let __v = (i != 0); compat::write(&self.material_buttons[(i) as usize], |__t| __t.trim_element_button_base_mut().active = __v); }
            }
            for i in 0..(self.pattern_buttons.len() as i32) {
                { let __v = (i != 0); compat::write(&self.pattern_buttons[(i) as usize], |__t| __t.active = __v); }
            }
        } else {
            let mut id: ArmorTrimId = trims.get(&item_uuid).cloned().unwrap();
            self.selected_material = Some(id.material());
            self.selected_pattern = Some(id.pattern());
            for material_button in self.material_buttons.iter_mut() {
                { let __v = !(self.selected_material.clone().unwrap() == ({ let __f = compat::dispatch(material_button, |__t| __t.get_element()); __f(material_button) }).unwrap()); compat::write(material_button, |__t| __t.trim_element_button_base_mut().active = __v); }
            }
            for pattern_button in self.pattern_buttons.iter_mut() {
                { let __v = !(self.selected_pattern.clone().unwrap() == ({ let __f = compat::dispatch(pattern_button, |__t| __t.get_element()); __f(pattern_button) }).unwrap()); compat::write(pattern_button, |__t| __t.trim_element_button_base_mut().active = __v); }
            }
        }
    }

}

// NOTE(fabric2seer): children, contentHeight, extractWidgetRenderState, mouseClicked, scrollRate, setHeight, setWidth, setX, setY, updateWidgetNarration override(s) of AbstractContainerWidget - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them
