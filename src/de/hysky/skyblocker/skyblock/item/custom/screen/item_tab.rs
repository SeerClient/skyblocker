//! Ported from de.hysky.skyblocker.skyblock.item.custom.screen.ItemTab by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::item::custom::screen::customize_screen::CustomizeScreen;
use crate::de::hysky::skyblocker::skyblock::item::custom::screen::identifier_text_field::IdentifierTextField;
use crate::de::hysky::skyblocker::skyblock::item::custom::screen::item_select_popup::ItemSelectPopup;
use crate::de::hysky::skyblocker::skyblock::item::custom::screen::model_selection_popup::ModelSelectionPopup;
use crate::de::hysky::skyblocker::skyblock::item::custom::screen::name::customize_name_widget::CustomizeNameWidget;
use crate::de::hysky::skyblocker::skyblock::item::skyblock_inventory_screen::SkyblockInventoryScreen;
use mcrust::mcrust::modding::widgets::ModWidget;
use mcrust::mcrust::modding::widgets::WidgetRenderContext;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::components::abstract_container_widget::AbstractContainerWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::abstract_widget::PressLambda;
use mcrust::net::minecraft::client::gui::components::abstract_widget::WidgetKind;
use mcrust::net::minecraft::client::gui::components::button;
use mcrust::net::minecraft::client::gui::components::events::gui_event_listener::GuiEventListener;
use mcrust::net::minecraft::client::gui::components::tabs::grid_layout_tab::GridLayoutTab;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::layouts::layout_element::LayoutElement;
use mcrust::net::minecraft::client::gui::layouts::linear_layout::LinearLayout;
use mcrust::net::minecraft::client::gui::layouts::spacer_element::SpacerElement;
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::client::input::mouse_button_info::MouseButtonInfo;
use mcrust::net::minecraft::client::player::local_player::LocalPlayer;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::util::tri_state::TriState;
use mcrust::org::joml::matrix3x2f_stack::Matrix3x2fStack;
use crate::de::hysky::skyblocker::injected::skyblocker_stack::SkyblockerStack as _;
use mcrust::net::minecraft::client::gui::components::renderable::Renderable as _;
use mcrust::net::minecraft::network::chat::component::Component as _;
use mcrust::net::minecraft::world::entity::living_entity::LivingEntity as _;

/// java: private static final Identifier INNER_SPACE_TEXTURE = SkyblockerMod.id("menu_inner_space");
pub static INNER_SPACE_TEXTURE: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("menu_inner_space"));
/// java: private static final int PADDING = 3;
pub const ITEM_SELECTOR_PADDING: i32 = 3;
/// java: private static final int ADDED_ITEM_OFFSET = 7;
pub const ITEM_SELECTOR_ADDED_ITEM_OFFSET: i32 = 7;

/// java: public class ItemTab extends GridLayoutTab
pub struct ItemTab {
    /// java: extends net.minecraft.client.gui.components.tabs.GridLayoutTab
    pub base: GridLayoutTab,
    /// java: private final CustomizeScreen parentScreen;
    pub parent_screen: Option<compat::Shared<CustomizeScreen>>,
    /// java: private final CustomizeNameWidget nameWidget;
    pub name_widget: Option<CustomizeNameWidget>,
    /// java: private final Button glintButton;
    pub glint_button: Option<AbstractWidget>,
    /// java: private final IdentifierTextField modelField;
    pub model_field: Option<compat::Shared<IdentifierTextField>>,
    /// java: private ItemStack currentItem = ItemStack.EMPTY;
    pub current_item: mcrust::net::minecraft::world::item::item_stack::ItemStack,
    /// java: private TriState glintState = TriState.DEFAULT;
    pub glint_state: TriState,
}

impl std::ops::Deref for ItemTab {
    type Target = GridLayoutTab;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for ItemTab {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl ItemTab {
    /// java: public ItemTab(CustomizeScreen parentScreen)
    pub fn new(mut parent_screen: compat::Shared<CustomizeScreen>) -> Self {
        let mut this = Self {
            base: GridLayoutTab::new(mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::translatable("skyblocker.customization.item"))),
            parent_screen: None,
            name_widget: None,
            glint_button: None,
            model_field: None,
            current_item: mcrust::net::minecraft::world::item::item_stack::empty(),
            glint_state: TriState::Default,
        };
        { this.layout.spacing(5); &mut this.layout };
        this.parent_screen = Some(parent_screen.clone());
        this.glint_button = Some(((button::builder(component::empty().get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| {
            // TODO(fabric2seer): static `TriState.values` has no Rust counterpart in the port
            // rust: let mut states: Vec<TriState> = TriState::values();
            let mut states: Vec<TriState> = todo!("fabric2seer: the initialiser of `states` (TODO above)");
            // TODO(fabric2seer): `TriState.ordinal` has no Rust method in the port (looked for `ordinal` in TriState and JavaSrc)
            // rust: this.glint_state = states[((this.glint_state.ordinal().wrapping_add(1)).wrapping_rem(states.len() as i32)) as usize];
            (__screen.widget_mut(__index)).set_message_text(mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(this.get_glint_text()));
            let mut uuid: String = this.current_item.get_uuid();
            SkyblockerConfigManager::update_only(Arc::new(move |mut config| {
                let custom_glint = &mut config.general.custom_glint;
                match this.glint_state {
                    TriState::Default => {
                        // TODO(fabric2seer): Map.removeBoolean(..)
                        //   java: customGlint.removeBoolean(uuid);
                    }
                    TriState::True => {
                        custom_glint.insert(uuid.clone(), true);
                    }
                    TriState::False => {
                        custom_glint.insert(uuid.clone(), false);
                    }
                    _ => {}
                }
            }));
        })))).width(mcrust::net::minecraft::client::gui::components::button::SMALL_WIDTH)).build());
        this.model_field = Some(IdentifierTextField::new(120, 20, Arc::new(move |identifier| {
            let mut uuid: String = this.current_item.get_uuid();
            if uuid.is_empty() {
                return;
            }
            SkyblockerConfigManager::update_only(Arc::new(move |mut config| {
                if identifier.is_none() {
                    config.general.custom_item_model.remove(&uuid);
                } else {
                    config.general.custom_item_model.insert(uuid.clone(), identifier.clone().unwrap());
                }
            }));
        })));
        let __wa1 = component::translatable("skyblocker.customization.item.modelOverride").with_style_format(ChatFormatting::Italic).with_style_format(ChatFormatting::Gray);
        let __o1 = this.model_field.clone().unwrap();
        compat::write(&__o1, |__t| __t.set_hint_component(__wa1.clone()));
        this.name_widget = Some(CustomizeNameWidget::new(parent_screen.clone()));
        this.layout.add_child_span(ItemSelector::new(), 0, 0, 1, 1).unwrap();
        // TODO(fabric2seer): a widget put in a layout outside a mod screen's own methods: the port's layouts hold a widget by its index in the screen's widget list
        // rust: this.layout.add_child_at(BackgroundRenderer::new(), 0, 1).unwrap();
        let mut linear_layout: LinearLayout = this.layout.add_child_at_adjusted(LayoutElement::Grid(LinearLayout::vertical().wrapped), 0, 1, &mut |p| { p.align_horizontally_right().padding_right(3).padding_vertical(3); }).unwrap();
        // TODO(fabric2seer): a widget put in a layout outside a mod screen's own methods: the port's layouts hold a widget by its index in the screen's widget list
        // rust: linear_layout.add_child(this.glint_button.clone().unwrap());
        // TODO(fabric2seer): a widget put in a layout outside a mod screen's own methods: the port's layouts hold a widget by its index in the screen's widget list
        // rust: linear_layout.add_child_adjusted((button::builder(component::translatable("skyblocker.customization.item.selectModel").get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| {
        // rust: let minecraft = compat::minecraft();
        // rust: let mut apply_item_model: Arc<dyn Fn(Option<Identifier>)> = Arc::new(move |stack| {
        // rust: if stack.is_some() {
        // rust: // TODO(fabric2seer): `fmt` takes 1 argument(s) in the port (`f: &mut fmt::Formatter<'_>`), Java passes 0
        // rust: // TODO(fabric2seer): `set_value` takes 2 argument(s) in the port (`splitter: &StringSplitter<W>, value: &str`), Java passes 1
        // rust: // rust: let __wa3 = stack.as_ref().unwrap().fmt();
        // rust: // rust: let __o3 = this.model_field.clone().unwrap();
        // rust: // rust: compat::write(&__o3, |__t| __t.set_value(&__wa3));
        // rust: }
        // rust: });
        // rust: mcrust::mcrust::modding::game::set_screen(Box::new(compat::ScreenAdapter::new(ModelSelectionPopup::new(parent_screen.clone(), apply_item_model.clone()))));
        // rust: })))).width(120).build(), &mut |p| { p.padding_top(4); });
        linear_layout.add_child(this.model_field.clone().unwrap());
        this.layout.add_child_span(this.name_widget.clone().unwrap(), 1, 0, 1, 2).unwrap();
        let mut hand_stack: mcrust::net::minecraft::world::item::item_stack::ItemStack = mcrust::mcrust::modding::game::with_player(|player| player.get_main_hand_item().clone()).unwrap();
        if !(hand_stack.get_uuid().is_empty()) {
            this.set_current_item(hand_stack.clone());
            return this;
        }
        for stack in SkyblockInventoryScreen::get_current_equipment_set() {
            if !(stack.get_uuid().is_empty()) {
                this.set_current_item(stack.clone());
                return this;
            }
        }
        // for (ItemStack stack : client.player.getInventory()) - the inventory's stacks, read through the player
        let __stacks: Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack> = mcrust::mcrust::modding::game::with_player(|player| {
            use mcrust::net::minecraft::world::container::Container as _;
            use mcrust::net::minecraft::world::entity::player::player::Player as _;
            let inventory = player.player().get_inventory();
            (0..inventory.get_container_size()).map(|i| inventory.get_item(i).duplicate()).collect()
        }).unwrap_or_default();
        for stack in __stacks.iter() {
            if !(stack.get_uuid().is_empty()) {
                this.set_current_item(stack.clone());
                return this;
            }
        }
        // TODO(fabric2seer): `visit_children` takes 0 argument(s) in the port (``), Java passes 1
        // TODO(fabric2seer): a lambda for a parameter the port does not type as a closure (`Consumer<net.minecraft.client.gui.components.AbstractWidget>`)
        // rust: this.visit_children(|clickable_widget| { clickable_widget.visible = false; });
        // TODO(fabric2seer): a widget put in a layout outside a mod screen's own methods: the port's layouts hold a widget by its index in the screen's widget list
        // rust: this.layout.add_child_span_adjusted(compat::string_widget(0, 0, -1, 9, component::translatable("skyblocker.customization.nothingCustomizable")), 0, 0, 3, 2, &mut |p| { p.align_horizontally_center().align_vertically_middle(); }).unwrap();
        this
    }

    /// java: private void setCurrentItem(ItemStack itemStack)
    pub fn set_current_item(&mut self, mut item_stack: mcrust::net::minecraft::world::item::item_stack::ItemStack) {
        self.current_item = item_stack.clone();
        let mut uuid: String = self.current_item.get_uuid();
        let mut empty: bool = uuid.is_empty();
        // TODO(fabric2seer): `visit_children` takes 0 argument(s) in the port (``), Java passes 1
        // TODO(fabric2seer): a lambda for a parameter the port does not type as a closure (`Consumer<net.minecraft.client.gui.components.AbstractWidget>`)
        // rust: self.visit_children(|clickable_widget| { clickable_widget.visible = !empty; });
        if empty {
            return;
        }
        CustomizeScreen::backup_configs(self.parent_screen.as_ref().unwrap(), &item_stack);
        self.name_widget.as_mut().unwrap().set_item(&item_stack);
        if SkyblockerConfigManager::get().general.custom_item_model.contains_key(&uuid) {
            let mut identifier: Identifier = SkyblockerConfigManager::get().general.custom_item_model.get(&uuid).cloned().unwrap();
            // TODO(fabric2seer): `fmt` takes 1 argument(s) in the port (`f: &mut fmt::Formatter<'_>`), Java passes 0
            // rust: let mut string: String = identifier.fmt();
            let mut string: String = todo!("fabric2seer: the initialiser of `string` (TODO above)");
            // TODO(fabric2seer): `set_value` takes 2 argument(s) in the port (`splitter: &StringSplitter<W>, value: &str`), Java passes 1
            // rust: let __wa1 = string.clone();
            // rust: let __o1 = self.model_field.clone().unwrap();
            // rust: compat::write(&__o1, |__t| __t.set_value(&__wa1));
        } else {
            // TODO(fabric2seer): `set_value` takes 2 argument(s) in the port (`splitter: &StringSplitter<W>, value: &str`), Java passes 1
            // rust: let __o2 = self.model_field.clone().unwrap();
            // rust: compat::write(&__o2, |__t| __t.set_value(""));
        }
        let custom_glint = &SkyblockerConfigManager::get().general.custom_glint;
        let mut item_uuid: String = item_stack.get_uuid();
        if custom_glint.contains_key(&item_uuid) {
            self.glint_state = (if custom_glint.get(&item_uuid).copied().unwrap_or_default() { TriState::True } else { TriState::False });
        } else {
            self.glint_state = TriState::Default;
        }
        { let __s0 = mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(self.get_glint_text()); self.glint_button.as_mut().unwrap().set_message_text(__s0) };
    }

    /// java: private Component getGlintText()
    pub fn get_glint_text(&self) -> MutableComponent {
        return component::translatable_args("skyblocker.customization.item.glint", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(match self.glint_state {
            TriState::Default => mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::translatable("skyblocker.customization.item.glint.default")),
            TriState::True => mcrust::net::minecraft::network::chat::common_components::option_on(),
            TriState::False => mcrust::net::minecraft::network::chat::common_components::option_off(),
            _ => unreachable!("switch expression: no case matched"),
        })]);
    }

}

/// java: private class ItemSelector extends AbstractContainerWidget
pub struct ItemSelector {
    /// java: extends net.minecraft.client.gui.components.AbstractContainerWidget
    pub base: AbstractContainerWidget,
    /// java: private final Button selectItemButton;
    pub select_item_button: Option<AbstractWidget>,
    /// java: private final LinearLayout layout = LinearLayout.vertical().spacing(5);
    pub layout: LinearLayout,
}

impl std::ops::Deref for ItemSelector {
    type Target = AbstractContainerWidget;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for ItemSelector {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl ItemSelector {
    /// java: private ItemSelector()
    pub fn new() -> Self {
        // TODO(fabric2seer): static `AbstractScrollArea.defaultSettings` has no Rust counterpart in the port
        //   java: super(0, 20, 0, 0, Component.literal("Item Selector"), AbstractScrollArea.defaultSettings(8));
        let mut this: Self = todo!("constructor");
        this.layout.add_child(SpacerElement::height(32i32.wrapping_add(ITEM_SELECTOR_ADDED_ITEM_OFFSET)).as_layout_element());
        // TODO(fabric2seer): `parentScreen` is a field of the enclosing instance (ItemTab.this) - an inner class has no outer reference in Rust
        //   java: selectItemButton = layout.addChild(Button.builder(Component.literal("Select Item"), _ ->
        //   java: 					Minecraft.getInstance().gui.setScreen(new ItemSelectPopup(parentScreen, ItemTab.this::setCurrentItem))
        //   java: 			).width(Button.SMALL_WIDTH).build());
        this.layout.arrange_elements();
        // TODO(fabric2seer): `LayoutElement::set_position` ports `setPosition`, but the port's `LinearLayout` holds no `LayoutElement` (no Deref, no `base` field) to call it on
        // rust: this.layout.set_position(ITEM_SELECTOR_PADDING, ITEM_SELECTOR_PADDING);
        this.set_size(this.layout.get_width().wrapping_add(ITEM_SELECTOR_PADDING.wrapping_mul(2)), this.layout.get_height().wrapping_add(ITEM_SELECTOR_PADDING.wrapping_mul(2)));
        this
    }

    /// java: @Override public void setX(int x)
    pub fn set_x(&mut self, mut x: i32) {
        self.base.set_x(x);
        { let __s0 = self.get_x().wrapping_add(ITEM_SELECTOR_PADDING); self.layout.set_x(__s0) };
    }

    /// java: @Override public void setY(int y)
    pub fn set_y(&mut self, mut y: i32) {
        self.base.set_y(y);
        { let __s0 = self.get_y().wrapping_add(ITEM_SELECTOR_PADDING); self.layout.set_y(__s0) };
    }

    /// java: @Override public List<? extends GuiEventListener> children()
    pub fn children(&self) -> Vec<Box<dyn GuiEventListener>> {
        return vec![self.select_item_button.clone().unwrap()];
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
    pub fn extract_widget_render_state(&mut self, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        gui_graphics_extractor::blit_sprite_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), INNER_SPACE_TEXTURE.get_mut(), self.get_x(), self.get_y(), self.get_width(), self.get_height());
        let __pose_1 = gui_graphics_extractor::push_pose();
        let mut x: f32 = (((self.layout.get_x() as f32) + ((self.layout.get_width() as f32) / 2.0f32)) - (16 as f32));
        let mut y: i32 = self.layout.get_y().wrapping_add(ITEM_SELECTOR_ADDED_ITEM_OFFSET);
        if (mouse_x as f32) >= x && (mouse_x as f32) < (x + (32 as f32)) && mouse_y >= y && mouse_y < y.wrapping_add(32) {
            // TODO(fabric2seer): `currentItem` is a field of the enclosing instance (ItemTab.this) - an inner class has no outer reference in Rust
            //   java: graphics.setTooltipForNextFrame(currentItem.getHoverName(), mouseX, mouseY);
        }
        gui_graphics_extractor::pose_translate((x) as f32, (y) as f32);
        // TODO(fabric2seer): Matrix3x2fStack.scale
        //   java: matrices.scale(2);
        // TODO(fabric2seer): `currentItem` is a field of the enclosing instance (ItemTab.this) - an inner class has no outer reference in Rust
        //   java: graphics.item(currentItem, 0, 0);
        gui_graphics_extractor::pop_pose(__pose_1);
        // TODO(fabric2seer): `extract_render_state` is a method of the port's trait `Renderable`, which the port's `AbstractWidget` does not implement
        // rust: self.select_item_button.as_mut().unwrap().extract_render_state(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mouse_x, mouse_y, a);
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    pub fn update_widget_narration(&self, builder: &dyn NarrationElementOutput) {
    }

}

// NOTE(fabric2seer): children, contentHeight, extractWidgetRenderState, scrollRate, setX, setY, updateWidgetNarration override(s) of AbstractContainerWidget - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them

/// java: private class BackgroundRenderer extends AbstractWidget
#[derive(Clone)]
pub struct BackgroundRenderer {
    /// java: extends net.minecraft.client.gui.components.AbstractWidget
    pub base: AbstractWidget,
}

impl std::ops::Deref for BackgroundRenderer {
    type Target = AbstractWidget;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for BackgroundRenderer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl BackgroundRenderer {
    /// java: BackgroundRenderer()
    pub fn new() -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            base: AbstractWidget::new(0, 0, 0, 0, (&component::empty().get_string()).to_string(), WidgetKind::PlainText, OnPress::Inert),
        });
        { let __v = false; compat::write(&__this, |__t| __t.active = __v); }
        __this
    }

    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_widget_render_state(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        // TODO(fabric2seer): `glintButton` is a field of the enclosing instance (ItemTab.this) - an inner class has no outer reference in Rust
        //   java: int x = glintButton.getX() - 3;
        let mut x: i32 = todo!("fabric2seer: the initialiser of `x` (TODO above)");
        // TODO(fabric2seer): `glintButton` is a field of the enclosing instance (ItemTab.this) - an inner class has no outer reference in Rust
        //   java: int y = glintButton.getY() - 3;
        let mut y: i32 = todo!("fabric2seer: the initialiser of `y` (TODO above)");
        // TODO(fabric2seer): `modelField` is a field of the enclosing instance (ItemTab.this) - an inner class has no outer reference in Rust
        //   java: graphics.blitSprite(RenderPipelines.GUI_TEXTURED,
        //   java: 					INNER_SPACE_TEXTURE,
        //   java: 					x,
        //   java: 					y,
        //   java: 					modelField.getRight() + 3 - x,
        //   java: 					modelField.getBottom() + 3 - y
        //   java: 			);
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    pub fn update_widget_narration(__this: &compat::Shared<Self>, builder: &dyn NarrationElementOutput) {
    }

}

/// BackgroundRenderer's widget half (`super`'s fields: x, y, size, message, ...), lent by the game for each call
impl compat::WidgetHalf for BackgroundRenderer {
    fn widget_half(&mut self) -> &mut AbstractWidget {
        &mut self.base
    }
}

/// BackgroundRenderer as the game's widget: `compat::WidgetAdapter::widget(handle)` where Java hands it on
impl ModWidget for compat::WidgetAdapter<BackgroundRenderer> {
    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_widget_render_state(&mut self, widget: &AbstractWidget, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        self.lend_ref(widget, |__h| BackgroundRenderer::extract_widget_render_state(__h, &context.graphics, mouse_x, mouse_y, a))
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    fn update_widget_narration(&mut self, widget: &AbstractWidget, output: &mut dyn NarrationElementOutput) {
        self.lend_ref(widget, |__h| BackgroundRenderer::update_widget_narration(__h, output))
    }

}

// NOTE(fabric2seer): extractWidgetRenderState, updateWidgetNarration override(s) of AbstractWidget - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them
