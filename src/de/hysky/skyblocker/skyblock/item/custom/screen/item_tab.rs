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
use mcrust::mcrust::modding::widgets::WidgetRenderContext;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::components::abstract_container_widget::AbstractContainerWidget;
use mcrust::net::minecraft::client::gui::components::abstract_scroll_area::ScrollbarSettings;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::abstract_widget::PressLambda;
use mcrust::net::minecraft::client::gui::components::button;
use mcrust::net::minecraft::client::gui::components::tabs::grid_layout_tab::GridLayoutTab;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::layouts::grid_layout::GridLayout;
use mcrust::net::minecraft::client::gui::layouts::layout_element::LayoutElement;
use mcrust::net::minecraft::client::gui::layouts::linear_layout::LinearLayout;
use mcrust::net::minecraft::client::gui::layouts::spacer_element::SpacerElement;
use mcrust::net::minecraft::client::gui::navigation::screen_rectangle::ScreenRectangle;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::key_event::KeyEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::network::chat::common_components;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::formatted_text::FormattedText;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::util::tri_state::TriState;
use crate::de::hysky::skyblocker::injected::skyblocker_stack::SkyblockerStack as _;

/// java: private static final Identifier INNER_SPACE_TEXTURE = SkyblockerMod.id("menu_inner_space");
pub static INNER_SPACE_TEXTURE: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("menu_inner_space"));
/// java: private static final int PADDING = 3; (ItemSelector)
pub const ITEM_SELECTOR_PADDING: i32 = 3;
/// java: private static final int ADDED_ITEM_OFFSET = 7; (ItemSelector) - "Offset added to ensure this is aligned with the item model field area"
pub const ITEM_SELECTOR_ADDED_ITEM_OFFSET: i32 = 7;

/// The tab's widgets' places in the screen's widget list.
#[derive(Clone, Debug)]
pub struct ItemTabWidgets {
    pub item_selector: usize,
    pub background_renderer: usize,
    pub glint_button: usize,
    pub select_model_button: usize,
    pub model_field: usize,
    pub name_widget: usize,
    pub nothing_customizable: Option<usize>,
}

/// What `setCurrentItem` writes on the screen's own widgets (`visible`, the glint button's message, the model field's
/// value) while the screen is not up (a popup is - the item and model pickers call back from there): written at the
/// screen's next call that holds its widgets (`ItemTab::apply`).
#[derive(Clone, Debug, Default)]
pub struct ItemTabPending {
    pub children_visible: Option<bool>,
    pub glint_message: Option<FormattedText>,
    pub model_value: Option<String>,
}

/// java: public class ItemTab extends GridLayoutTab
pub struct ItemTab {
    /// java: extends net.minecraft.client.gui.components.tabs.GridLayoutTab
    pub base: GridLayoutTab,
    /// java: private final CustomizeScreen parentScreen; (the screen holds this tab, so the tab holds it weakly)
    pub parent_screen: compat::WeakShared<CustomizeScreen>,
    /// java: private final CustomizeNameWidget nameWidget;
    pub name_widget: compat::Shared<CustomizeNameWidget>,
    /// java: private final IdentifierTextField modelField; - its state (the `EditBox` is the screen's widget)
    pub model_field: IdentifierTextField,
    /// java: private ItemStack currentItem = ItemStack.EMPTY;
    pub current_item: mcrust::net::minecraft::world::item::item_stack::ItemStack,
    /// java: private TriState glintState = TriState.DEFAULT;
    pub glint_state: TriState,
    /// the `ItemSelector` and `BackgroundRenderer` objects
    pub item_selector: compat::Shared<ItemSelector>,
    pub background_renderer: compat::Shared<BackgroundRenderer>,
    /// where the widgets are in the screen
    pub widgets: ItemTabWidgets,
    /// `visitChildren(w -> w.visible = ..)`'s value for the screen's own widgets
    pub children_visible: bool,
    pub pending: ItemTabPending,
    /// whether the model field had the screen's focus at the last look (`IdentifierTextField.setFocused`)
    pub model_field_focused: bool,
    /// `glintButton.getX()`, `getY()`, `modelField.getRight()`, `getBottom()` after the last layout, which
    /// `BackgroundRenderer` draws from
    pub background_box: (i32, i32, i32, i32),
}

impl ItemTab {
    /// java: public ItemTab(CustomizeScreen parentScreen)
    pub fn new(parent_screen: &compat::Shared<CustomizeScreen>, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) -> compat::Shared<Self> {
        // java: super(Component.translatable("skyblocker.customization.item")); layout.spacing(5);
        let parent_ref = Arc::downgrade(parent_screen);
        let this: compat::Shared<ItemTab> = Arc::new_cyclic(|weak: &compat::WeakShared<ItemTab>| {
            // java: glintButton = Button.builder(Component.empty(), b -> {..}).width(Button.SMALL_WIDTH).build();
            let tab = compat::RenderThread::new(weak.clone());
            let glint_button = button::builder(String::new(), OnPress::Lambda(PressLambda::new(move |screen, index| {
                let Some(tab) = tab.get().upgrade() else {
                    return;
                };
                let (glint_state, uuid, message) = compat::write(&tab, |__t| {
                    let states = mcrust::net::minecraft::util::tri_state::VALUES;
                    let ordinal = states.iter().position(|s| *s == __t.glint_state).expect("TriState.ordinal()");
                    __t.glint_state = states[(ordinal + 1) % states.len()];
                    (__t.glint_state, __t.current_item.get_uuid(), __t.get_glint_text())
                });
                // java: b.setMessage(getGlintText());
                screen.widget_mut(index).set_message_text(FormattedText::from(message));
                SkyblockerConfigManager::update_only(Arc::new(move |config: &mut crate::de::hysky::skyblocker::config::skyblocker_config::SkyblockerConfig| {
                    let custom_glint = &mut config.general.custom_glint;
                    match glint_state {
                        TriState::Default => {
                            custom_glint.remove(&uuid);
                        }
                        TriState::True => {
                            custom_glint.insert(uuid.clone(), true);
                        }
                        TriState::False => {
                            custom_glint.insert(uuid.clone(), false);
                        }
                    }
                }));
            })))
            .width(button::SMALL_WIDTH)
            .build();
            // java: modelField = new IdentifierTextField(120, 20, identifier -> {..});
            let tab = weak.clone();
            let (model_field, mut model_field_widget) = IdentifierTextField::new(
                120,
                20,
                Arc::new(move |identifier: Option<Identifier>| {
                    let Some(tab) = tab.upgrade() else {
                        return;
                    };
                    let uuid: String = compat::read(&tab, |__t| __t.current_item.get_uuid());
                    if uuid.is_empty() {
                        return;
                    }
                    SkyblockerConfigManager::update_only(Arc::new(move |config: &mut crate::de::hysky::skyblocker::config::skyblocker_config::SkyblockerConfig| match &identifier {
                        None => {
                            config.general.custom_item_model.remove(&uuid);
                        }
                        Some(identifier) => {
                            config.general.custom_item_model.insert(uuid.clone(), identifier.clone());
                        }
                    }));
                }),
            );
            // java: modelField.setHint(Component.translatable("skyblocker.customization.item.modelOverride").withStyle(ChatFormatting.ITALIC).withStyle(ChatFormatting.GRAY));
            compat::with_edit_box(&mut model_field_widget, |e| e.set_hint_component(FormattedText::from(component::translatable("skyblocker.customization.item.modelOverride").with_style_format(ChatFormatting::Italic).with_style_format(ChatFormatting::Gray))));
            // java: nameWidget = new CustomizeNameWidget(parentScreen);
            let name_widget = CustomizeNameWidget::new();

            // java: layout.addChild(new ItemSelector(), 0, 0, 1, 1); layout.addChild(new BackgroundRenderer(), 0, 1);
            let item_selector = ItemSelector::new(weak.clone());
            let item_selector_index = screen.add_widget(compat::object_widget(item_selector.clone(), "Item Selector"));
            let background_renderer = BackgroundRenderer::new(weak.clone());
            let background_renderer_index = screen.add_widget(compat::object_widget(background_renderer.clone(), ""));
            let glint_index = screen.add_widget(glint_button);
            // java: Button.builder(Component.translatable("skyblocker.customization.item.selectModel"), _ -> {..}).width(120).build()
            let tab = compat::RenderThread::new(weak.clone());
            let select_model = button::builder(component::translatable("skyblocker.customization.item.selectModel").get_string(), OnPress::Lambda(PressLambda::new(move |_screen, _index| {
                let tab = tab.get().clone();
                // java: Consumer<@Nullable Identifier> applyItemModel = stack -> { if (stack != null) modelField.setValue(stack.toString()); };
                let apply_item_model: Arc<dyn Fn(Option<Identifier>)> = Arc::new(move |stack: Option<Identifier>| {
                    if let Some(stack) = stack
                        && let Some(tab) = tab.upgrade()
                    {
                        compat::write(&tab, |__t| __t.pending.model_value = Some(stack.to_string()));
                    }
                });
                // java: minecraft.gui.setScreen(new ModelSelectionPopup(parentScreen, applyItemModel));
                // rust: port gap (GAPS_batchA #19) - the popup base keeps its background as the game's `Screen` value;
                //   the screen that is up is held by the game (`set_screen_keeping_parent`) and an empty screen stands in
                let popup = ModelSelectionPopup::new(mcrust::net::minecraft::client::gui::screens::screen::Screen::new(String::new()), apply_item_model);
                mcrust::mcrust::modding::game::set_screen_keeping_parent(Box::new(compat::ScreenAdapter::new(popup)));
            })))
            .width(120)
            .build();
            let select_model_index = screen.add_widget(select_model);
            let model_field_index = screen.add_widget(model_field_widget);
            let name_widget_index = screen.add_widget(compat::object_widget(name_widget.clone(), "Customize Item Name"));

            parking_lot::ReentrantMutex::new(std::cell::RefCell::new(ItemTab {
                base: GridLayoutTab::new(FormattedText::from(component::translatable("skyblocker.customization.item"))),
                parent_screen: parent_ref,
                name_widget,
                model_field,
                current_item: mcrust::net::minecraft::world::item::item_stack::empty(),
                glint_state: TriState::Default,
                item_selector,
                background_renderer,
                widgets: ItemTabWidgets {
                    item_selector: item_selector_index,
                    background_renderer: background_renderer_index,
                    glint_button: glint_index,
                    select_model_button: select_model_index,
                    model_field: model_field_index,
                    name_widget: name_widget_index,
                    nothing_customizable: None,
                },
                children_visible: true,
                pending: ItemTabPending::default(),
                model_field_focused: false,
                background_box: (0, 0, 0, 0),
            }))
        });

        // java: LocalPlayer player = Minecraft.getInstance().player; ItemStack handStack = player.getMainHandItem();
        let hand_stack = mcrust::mcrust::modding::game::with_player(|player| player.get_main_hand_item().clone()).expect("Minecraft.player");
        let mut found: Option<mcrust::net::minecraft::world::item::item_stack::ItemStack> = None;
        if !hand_stack.get_uuid().is_empty() {
            found = Some(hand_stack);
        }
        if found.is_none() {
            found = SkyblockInventoryScreen::get_current_equipment_set().into_iter().find(|stack| !stack.get_uuid().is_empty());
        }
        if found.is_none() {
            // for (ItemStack stack : player.getInventory()) - the inventory's stacks, read through the player
            let stacks: Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack> = mcrust::mcrust::modding::game::with_player(|player| {
                use mcrust::net::minecraft::world::container::Container as _;
                use mcrust::net::minecraft::world::entity::player::player::Player as _;
                let inventory = player.player().get_inventory();
                (0..inventory.get_container_size()).map(|i| inventory.get_item(i).duplicate()).collect()
            })
            .unwrap_or_default();
            found = stacks.into_iter().find(|stack| !stack.get_uuid().is_empty());
        }
        match found {
            Some(stack) => ItemTab::set_current_item(&this, stack),
            None => {
                // java: visitChildren(clickableWidget -> clickableWidget.visible = false);
                ItemTab::set_children_visible(&this, false);
                // java: layout.addChild(new StringWidget(..nothingCustomizable..), 0, 0, 3, 2, p -> p.alignHorizontallyCenter().alignVerticallyMiddle());
                let index = screen.add_widget(compat::string_widget(0, 0, -1, 9, component::translatable("skyblocker.customization.nothingCustomizable")));
                compat::write(&this, |__t| __t.widgets.nothing_customizable = Some(index));
            }
        }
        let grid = compat::read(&this, |__t| __t.build_grid(screen));
        compat::write(&this, |__t| __t.base.layout = grid);
        ItemTab::apply(&this, screen);
        this
    }

    /// The tab's grid (`layout.spacing(5)` and the constructor's `addChild`s), over the widgets' sizes now (the port's
    /// layout leaves keep the size they were given, Java's read the widget's).
    fn build_grid(&self, screen: &mcrust::net::minecraft::client::gui::screens::screen::Screen) -> GridLayout {
        let leaf = |index: usize| {
            let widget = screen.widget(index);
            LayoutElement::widget(index, widget.get_width(), widget.get_height())
        };
        let mut layout = GridLayout::new_default();
        layout.spacing(5);
        layout.add_child_span(leaf(self.widgets.item_selector), 0, 0, 1, 1).expect("GridLayout.addChild");
        layout.add_child_at(leaf(self.widgets.background_renderer), 0, 1).expect("GridLayout.addChild");

        // java: LinearLayout linearLayout = layout.addChild(LinearLayout.vertical(), 0, 1, p -> p.alignHorizontallyRight().paddingRight(3).paddingVertical(3));
        let mut linear_layout = LinearLayout::vertical();
        linear_layout.add_child(leaf(self.widgets.glint_button));
        let settings = linear_layout.new_cell_settings().padding_top(4);
        linear_layout.add_child_with(leaf(self.widgets.select_model_button), settings);
        linear_layout.add_child(leaf(self.widgets.model_field));
        layout.add_child_at_adjusted(LayoutElement::Grid(linear_layout.wrapped), 0, 1, &mut |p| *p = p.clone().align_horizontally_right().padding_right(3).padding_vertical(3)).expect("GridLayout.addChild");

        // java: layout.addChild(nameWidget, 1, 0, 1, 2);
        layout.add_child_span(leaf(self.widgets.name_widget), 1, 0, 1, 2).expect("GridLayout.addChild");

        if let Some(text) = self.widgets.nothing_customizable {
            layout.add_child_span_adjusted(leaf(text), 0, 0, 3, 2, &mut |p| *p = p.clone().align_horizontally_center().align_vertically_middle()).expect("GridLayout.addChild");
        }
        layout
    }

    /// `visitChildren(w -> w.visible = visible)` - the objects now, the screen's own widgets at `apply`
    fn set_children_visible(this: &compat::Shared<Self>, visible: bool) {
        let (item_selector, background_renderer, name_widget) = compat::write(this, |__t| {
            __t.children_visible = visible;
            __t.pending.children_visible = Some(visible);
            (__t.item_selector.clone(), __t.background_renderer.clone(), __t.name_widget.clone())
        });
        compat::write(&item_selector, |__t| __t.base.visible = visible);
        compat::write(&background_renderer, |__t| __t.base.visible = visible);
        compat::write(&name_widget, |__t| __t.base.visible = visible);
    }

    /// java: private void setCurrentItem(ItemStack itemStack)
    pub fn set_current_item(this: &compat::Shared<Self>, item_stack: mcrust::net::minecraft::world::item::item_stack::ItemStack) {
        compat::write(this, |__t| __t.current_item = item_stack.clone());
        let uuid: String = item_stack.get_uuid();
        let empty: bool = uuid.is_empty();
        ItemTab::set_children_visible(this, !empty);
        if empty {
            return;
        }
        if let Some(parent) = compat::read(this, |__t| __t.parent_screen.upgrade()) {
            CustomizeScreen::backup_configs(&parent, &item_stack);
        }
        let name_widget = compat::read(this, |__t| __t.name_widget.clone());
        compat::write(&name_widget, |__t| __t.set_item(&item_stack));
        // java: modelField.setValue(customItemModel.containsKey(uuid) ? identifier.toString() : "");
        let model_value = match SkyblockerConfigManager::get().general.custom_item_model.get(&uuid) {
            Some(identifier) => identifier.to_string(),
            None => String::new(),
        };

        // glint
        let custom_glint = &SkyblockerConfigManager::get().general.custom_glint;
        let glint_state = match custom_glint.get(&uuid) {
            Some(true) => TriState::True,
            Some(false) => TriState::False,
            None => TriState::Default,
        };
        compat::write(this, |__t| {
            __t.pending.model_value = Some(model_value);
            __t.glint_state = glint_state;
            // java: glintButton.setMessage(getGlintText());
            __t.pending.glint_message = Some(FormattedText::from(__t.get_glint_text()));
        });
    }

    /// java: private Component getGlintText()
    fn get_glint_text(&self) -> MutableComponent {
        let state: FormattedText = match self.glint_state {
            TriState::Default => FormattedText::from(component::translatable("skyblocker.customization.item.glint.default")),
            TriState::True => common_components::option_on(),
            TriState::False => common_components::option_off(),
        };
        component::translatable_args("skyblocker.customization.item.glint", vec![state])
    }

    /// The screen's own widgets brought up to what the tab wrote while it could not reach them (`ItemTabPending`).
    pub fn apply(this: &compat::Shared<Self>, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        let (pending, widgets) = compat::write(this, |__t| (std::mem::take(&mut __t.pending), __t.widgets.clone()));
        if let Some(visible) = pending.children_visible {
            for index in [widgets.glint_button, widgets.select_model_button, widgets.model_field] {
                screen.widget_mut(index).visible = visible;
            }
        }
        if let Some(message) = pending.glint_message {
            screen.widget_mut(widgets.glint_button).set_message_text(message);
        }
        if let Some(value) = pending.model_value {
            // (the field's responder reads this tab: no borrow of it is held here)
            compat::set_edit_box_value(screen.widget_mut(widgets.model_field), &value);
        }
    }

    /// `IdentifierTextField.setFocused(false)` when the screen's focus has left the model field since the last look.
    pub fn check_model_field_focus(this: &compat::Shared<Self>, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        let index = compat::read(this, |__t| __t.widgets.model_field);
        let focused = screen.get_focused() == Some(index);
        let was_focused = compat::write(this, |__t| std::mem::replace(&mut __t.model_field_focused, focused));
        if was_focused && !focused {
            // (a shared borrow: the field's callback reads this tab too)
            compat::read(this, |__t| __t.model_field.set_focused(screen.widget_mut(index), false));
        }
    }

    /// java: Tab.doLayout - `GridLayoutTab.doLayout` over the grid as the widgets now are
    pub fn do_layout(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, tab_area: ScreenRectangle) {
        self.base.layout = self.build_grid(screen);
        self.base.do_layout(screen, tab_area);
        let glint = screen.widget(self.widgets.glint_button);
        let model = screen.widget(self.widgets.model_field);
        self.background_box = (glint.get_x(), glint.get_y(), model.get_right(), model.get_bottom());
    }

    /// java: Tab.visitChildren - `layout.visitWidgets(..)`
    pub fn visit_children(&self) -> Vec<usize> {
        self.base.visit_children()
    }

    /// Java's `visible` of child `index` (the tab's `visitChildren` writes the screen's own widgets' flag)
    pub fn child_visible(&self, index: usize) -> bool {
        if index == self.widgets.glint_button || index == self.widgets.select_model_button || index == self.widgets.model_field {
            self.children_visible
        } else {
            true
        }
    }
}

/// java: private class ItemSelector extends AbstractContainerWidget
pub struct ItemSelector {
    /// java: extends net.minecraft.client.gui.components.AbstractContainerWidget
    pub base: AbstractContainerWidget,
    /// java: ItemTab.this
    pub tab: compat::WeakShared<ItemTab>,
    /// java: private final LinearLayout layout = LinearLayout.vertical().spacing(5);
    pub layout: LayoutElement,
    /// java: private final Button selectItemButton; - `children`'s widget 0
    pub children: mcrust::net::minecraft::client::gui::screens::screen::Screen,
}

impl ItemSelector {
    /// java: private ItemSelector()
    fn new(tab: compat::WeakShared<ItemTab>) -> compat::Shared<Self> {
        // java: super(0, 20, 0, 0, Component.literal("Item Selector"), AbstractScrollArea.defaultSettings(8));
        let base = AbstractContainerWidget::new_with_scrollbar_settings(0, 20, 0, 0, FormattedText::of("Item Selector"), ScrollbarSettings::default_settings(8.0));
        let mut children = mcrust::net::minecraft::client::gui::screens::screen::Screen::new(String::new());
        // java: selectItemButton = layout.addChild(Button.builder(Component.literal("Select Item"), _ -> Minecraft.getInstance().gui.setScreen(new ItemSelectPopup(parentScreen, ItemTab.this::setCurrentItem))).width(Button.SMALL_WIDTH).build());
        let press_tab = compat::RenderThread::new(tab.clone());
        let select_item_button = button::builder("Select Item", OnPress::Lambda(PressLambda::new(move |_screen, _index| {
            let tab = press_tab.get().clone();
            let callback: Arc<dyn Fn(mcrust::net::minecraft::world::item::item_stack::ItemStack)> = Arc::new(move |stack: mcrust::net::minecraft::world::item::item_stack::ItemStack| {
                if let Some(tab) = tab.upgrade() {
                    ItemTab::set_current_item(&tab, stack);
                }
            });
            // rust: port gap (GAPS_batchA #19) - the popup base keeps its background as the game's `Screen` value;
            //   the screen that is up is held by the game (`set_screen_keeping_parent`) and an empty screen stands in
            let popup = ItemSelectPopup::new(mcrust::net::minecraft::client::gui::screens::screen::Screen::new(String::new()), callback);
            mcrust::mcrust::modding::game::set_screen_keeping_parent(Box::new(compat::ScreenAdapter::new(popup)));
        })))
        .width(button::SMALL_WIDTH)
        .build();
        let (bw, bh) = (select_item_button.get_width(), select_item_button.get_height());
        let index = children.add_renderable_widget(select_item_button);
        // java: layout.addChild(SpacerElement.height(32 + ADDED_ITEM_OFFSET)); .. layout.arrangeElements(); layout.setPosition(PADDING, PADDING);
        let mut layout = LinearLayout::vertical().spacing(5);
        layout.add_child(SpacerElement::height(32 + ITEM_SELECTOR_ADDED_ITEM_OFFSET).as_layout_element());
        layout.add_child(LayoutElement::widget(index, bw, bh));
        let mut layout = LayoutElement::Grid(layout.wrapped);
        layout.arrange_elements();
        layout.set_position(ITEM_SELECTOR_PADDING, ITEM_SELECTOR_PADDING);
        let mut this = ItemSelector { base, tab, layout, children };
        this.place_children();
        // java: setSize(layout.getWidth() + PADDING * 2, layout.getHeight() + PADDING * 2);
        let (w, h) = (this.layout.get_width() + ITEM_SELECTOR_PADDING * 2, this.layout.get_height() + ITEM_SELECTOR_PADDING * 2);
        this.base.set_size(w, h);
        compat::shared(this)
    }

    fn place_children(&mut self) {
        let mut placed = Vec::new();
        self.layout.visit_widgets(&mut placed);
        for (index, x, y) in placed {
            self.children.widget_mut(index).set_position(x, y);
        }
    }
}

impl compat::ObjectWidget for ItemSelector {
    fn object_box(&mut self) -> &mut AbstractContainerWidget {
        &mut self.base
    }

    fn object_box_ref(&self) -> &AbstractContainerWidget {
        &self.base
    }

    /// java: @Override public void setX(int x)
    fn set_x(&mut self, x: i32) {
        self.base.set_x(x);
        self.layout.set_x(self.base.get_x() + ITEM_SELECTOR_PADDING);
        self.place_children();
    }

    /// java: @Override public void setY(int y)
    fn set_y(&mut self, y: i32) {
        self.base.set_y(y);
        self.layout.set_y(self.base.get_y() + ITEM_SELECTOR_PADDING);
        self.place_children();
    }

    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_widget_render_state(&mut self, _widget: &AbstractWidget, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, _a: f32) {
        gui_graphics_extractor::blit_sprite_identifier(context.gl, context.minecraft, mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), INNER_SPACE_TEXTURE.get(), self.base.get_x(), self.base.get_y(), self.base.get_width(), self.base.get_height());
        let x: f32 = self.layout.get_x() as f32 + self.layout.get_width() as f32 / 2f32 - 16.0;
        let y: i32 = self.layout.get_y() + ITEM_SELECTOR_ADDED_ITEM_OFFSET;
        if let Some(tab) = self.tab.upgrade() {
            compat::read(&tab, |tab| {
                if (mouse_x as f32) >= x && (mouse_x as f32) < x + 32.0 && mouse_y >= y && mouse_y < y + 32 {
                    // java: graphics.setTooltipForNextFrame(currentItem.getHoverName(), mouseX, mouseY);
                    // rust: port gap - the frame's deferred tooltip is not reachable from a mod widget (GAPS_batchE.md)
                }
                // java: matrices.pushMatrix(); matrices.translate(x, y); matrices.scale(2); graphics.item(currentItem, 0, 0); matrices.popMatrix();
                let saved = gui_graphics_extractor::push_pose();
                gui_graphics_extractor::pose_translate(x, y as f32);
                gui_graphics_extractor::pose_scale(2.0, 2.0);
                context.graphics.item(&tab.current_item, 0, 0);
                gui_graphics_extractor::pop_pose(saved);
            });
        }
        // java: selectItemButton.extractRenderState(graphics, mouseX, mouseY, a);
        context.extract_widgets(&mut self.children, mouse_x, mouse_y);
    }

    /// ContainerEventHandler.mouseClicked:39-55 over `List.of(selectItemButton)`
    fn mouse_clicked(&mut self, _widget: &mut AbstractWidget, event: &MouseButtonEvent, _double_click: bool) -> bool {
        compat::children_mouse_clicked(&mut self.children, event)
    }

    /// ContainerEventHandler.mouseReleased:57-66
    fn mouse_released(&mut self, _widget: &mut AbstractWidget, event: &MouseButtonEvent) -> bool {
        compat::children_mouse_released(&mut self.children, event);
        false
    }

    /// ContainerEventHandler.keyPressed - the focused child
    fn key_pressed(&mut self, _widget: &mut AbstractWidget, event: &KeyEvent) -> bool {
        compat::children_key_pressed(&mut self.children, event)
    }

    /// ContainerEventHandler.charTyped - the focused child
    fn char_typed(&mut self, _widget: &mut AbstractWidget, event: &CharacterEvent) -> bool {
        compat::children_char_typed(&mut self.children, event)
    }
}

/// java: private class BackgroundRenderer extends AbstractWidget
pub struct BackgroundRenderer {
    /// java: extends net.minecraft.client.gui.components.AbstractWidget
    pub base: AbstractContainerWidget,
    /// java: ItemTab.this
    pub tab: compat::WeakShared<ItemTab>,
}

impl BackgroundRenderer {
    /// java: BackgroundRenderer()
    fn new(tab: compat::WeakShared<ItemTab>) -> compat::Shared<Self> {
        // java: super(0, 0, 0, 0, Component.empty()); active = false;
        let mut base = AbstractContainerWidget::new(0, 0, 0, 0, FormattedText::Empty);
        base.active = false;
        compat::shared(BackgroundRenderer { base, tab })
    }
}

impl compat::ObjectWidget for BackgroundRenderer {
    fn object_box(&mut self) -> &mut AbstractContainerWidget {
        &mut self.base
    }

    fn object_box_ref(&self) -> &AbstractContainerWidget {
        &self.base
    }

    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_widget_render_state(&mut self, _widget: &AbstractWidget, context: &mut WidgetRenderContext<'_>, _mouse_x: i32, _mouse_y: i32, _a: f32) {
        let Some(tab) = self.tab.upgrade() else {
            return;
        };
        // java: glintButton.getX() - 3, getY() - 3, modelField.getRight() + 3 - x, modelField.getBottom() + 3 - y
        let (glint_x, glint_y, model_right, model_bottom) = compat::read(&tab, |__t| __t.background_box);
        let x = glint_x - 3;
        let y = glint_y - 3;
        gui_graphics_extractor::blit_sprite_identifier(context.gl, context.minecraft, mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), INNER_SPACE_TEXTURE.get(), x, y, model_right + 3 - x, model_bottom + 3 - y);
    }
}
