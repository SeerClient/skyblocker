//! Ported from de.hysky.skyblocker.skyblock.item.custom.screen.ArmorTab by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::item::custom::screen::color_selection_widget::ColorSelectionWidget;
use crate::de::hysky::skyblocker::skyblock::item::custom::screen::customize_screen::CustomizeScreen;
use crate::de::hysky::skyblocker::skyblock::item::custom::screen::head_selection_widget::HeadSelectionWidget;
use crate::de::hysky::skyblocker::skyblock::item::custom::screen::identifier_text_field::IdentifierTextField;
use crate::de::hysky::skyblocker::skyblock::item::custom::screen::player_widget::PlayerWidget;
use crate::de::hysky::skyblocker::skyblock::item::custom::screen::trim_selection_widget::TrimSelectionWidget;
use crate::de::hysky::skyblocker::utils::item_utils::ItemUtils;
use mcrust::com::mojang::blaze3d::platform::cursor::cursor_type::CursorType;
use mcrust::com::mojang::blaze3d::platform::cursor::cursor_types::CursorTypes;
use mcrust::mcrust::modding::widgets::WidgetRenderContext;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::components::abstract_container_widget::AbstractContainerWidget;
use mcrust::net::minecraft::client::gui::components::abstract_scroll_area::ScrollbarSettings;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::tabs::grid_layout_tab::GridLayoutTab;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::layouts::frame_layout;
use mcrust::net::minecraft::client::gui::layouts::frame_layout::FrameLayout;
use mcrust::net::minecraft::client::gui::layouts::grid_layout::GridLayout;
use mcrust::net::minecraft::client::gui::layouts::layout_element::LayoutElement;
use mcrust::net::minecraft::client::gui::layouts::linear_layout::LinearLayout;
use mcrust::net::minecraft::client::gui::navigation::screen_rectangle::ScreenRectangle;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::key_event::KeyEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::client::player::remote_player::RemotePlayer;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::formatted_text::FormattedText;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::world::entity::equipment_slot::EquipmentSlot;
use mcrust::net::minecraft::world::item::items;
use crate::de::hysky::skyblocker::injected::skyblocker_stack::SkyblockerStack as _;
use mcrust::net::minecraft::world::entity::entity::Entity as _;
use mcrust::net::minecraft::world::entity::living_entity::LivingEntity as _;

/// java: private static final Identifier INNER_SPACE_TEXTURE = SkyblockerMod.id("menu_inner_space");
pub static INNER_SPACE_TEXTURE: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("menu_inner_space"));
/// java: private static final int PLAYER_WIDGET_WIDTH = 84;
pub const PLAYER_WIDGET_WIDTH: i32 = 84;
/// java: private static final int PADDING = 10;
pub const PADDING: i32 = 10;
/// java: private static final EquipmentSlot[] ARMOR_SLOTS = EquipmentSlot.VALUES.stream().filter(slot -> slot.getType() == EquipmentSlot.Type.HUMANOID_ARMOR).toArray(EquipmentSlot[]::new);
pub static ARMOR_SLOTS: LazyLock<Vec<EquipmentSlot>> = LazyLock::new(|| {
    mcrust::net::minecraft::world::entity::equipment_slot::VALUES
        .iter()
        .copied()
        .filter(|slot| slot.get_type() == mcrust::net::minecraft::world::entity::equipment_slot::Type::HumanoidArmor)
        .collect()
});
/// java: private static final ItemStack BARRIER = new ItemStack(Items.BARRIER);
pub static BARRIER: compat::MainThread<mcrust::net::minecraft::world::item::item_stack::ItemStack> = compat::MainThread::new(|| mcrust::net::minecraft::world::item::item_stack::ItemStack::of_one(items::items().get("barrier")));
/// java: private static final Identifier HOTBAR_TEXTURE = SkyblockerMod.id("armor_customization_screen/mini_hotbar");
pub static PIECE_SELECTION_WIDGET_HOTBAR_TEXTURE: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("armor_customization_screen/mini_hotbar"));
/// java: private static final Identifier HOTBAR_SELECTION_TEXTURE = SkyblockerMod.id("hotbar_selection_full");
pub static PIECE_SELECTION_WIDGET_HOTBAR_SELECTION_TEXTURE: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("hotbar_selection_full"));

/// The tab's widgets' places in the screen's widget list (the tab's grid holds them there, and
/// `CustomizeScreen` adds and removes them as the tab is switched).
#[derive(Clone, Debug)]
pub struct ArmorTabWidgets {
    pub player_widget: usize,
    pub piece_selection_widget: usize,
    pub head_selection_widget: usize,
    pub model_field_container: usize,
    pub trim_selection_widget: usize,
    pub color_selection_widget: usize,
    /// the "nothing customizable" text, only when nothing is
    pub nothing_customizable: Option<usize>,
}

/// java: public class ArmorTab extends GridLayoutTab implements Closeable
pub struct ArmorTab {
    /// java: extends net.minecraft.client.gui.components.tabs.GridLayoutTab
    pub base: GridLayoutTab,
    /// java: private final ItemStack[] armor = new ItemStack[4];
    pub armor: Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack>,
    /// java: private final CustomizeScreen parent; (the screen holds this tab, so the tab holds the screen weakly)
    pub parent: compat::WeakShared<CustomizeScreen>,
    /// java: private int selectedSlot = 0;
    pub selected_slot: i32,
    /// java: private final TrimSelectionWidget trimSelectionWidget;
    pub trim_selection_widget: compat::Shared<TrimSelectionWidget>,
    /// java: private final ColorSelectionWidget colorSelectionWidget;
    pub color_selection_widget: compat::Shared<ColorSelectionWidget>,
    /// java: private final HeadSelectionWidget headSelectionWidget;
    pub head_selection_widget: compat::Shared<HeadSelectionWidget>,
    /// java: private final ModelFieldContainer modelFieldContainer;
    pub model_field_container: compat::Shared<ModelFieldContainer>,
    /// java: private final boolean nothingCustomizable;
    pub nothing_customizable: bool,
    /// java: private final RemotePlayer player = new RemotePlayer(CLIENT.level, CLIENT.getGameProfile()) {..};
    pub player: compat::Shared<RemotePlayer>,
    /// where the widgets are in the screen
    pub widgets: ArmorTabWidgets,
}

impl ArmorTab {
    /// java: public ArmorTab(CustomizeScreen parent)
    ///
    /// (`screen`: the parent's widget list, where the tab's widgets go - Java's tab hands them to the screen through
    /// `visitChildren`)
    // TODO: The layout always calculates sizes based on all widgets in the layout, even if they are not visible.
    //  For example, in helmet customization, the layout uses the color selection widget's width
    //  as the head selection widget's width, even though the color selection widget is not visible.
    //  This results in the head selection widgets not being centered correctly on small screen widths.
    pub fn new(parent: &compat::Shared<CustomizeScreen>, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) -> compat::Shared<Self> {
        // java: super(Component.translatable("skyblocker.customization.armor"));

        // java: private final RemotePlayer player = new RemotePlayer(CLIENT.level, CLIENT.getGameProfile()) {..}
        // The anonymous subclass's overrides are the game's `RemotePlayerSubclass` (`entities::new_remote_player`);
        // `getId()` is `EntityUtils.PLACEHOLDER_ID`, the id the entity carries. `onEquipItem` is empty: the port's
        // `LivingEntity::set_item_slot` (below) is `PlayerEquipment.set` (the inventory's armour slots the renderer reads), with no equip sound or event either.
        let player = {
            let profile = mcrust::mcrust::modding::game::get_game_profile().expect("Minecraft.getGameProfile()");
            let mut player = mcrust::mcrust::modding::entities::new_remote_player(&profile, Some(Arc::new(ArmorTabPlayer)));
            {
                use mcrust::net::minecraft::world::entity::entity::Entity as _;
                player.base_mut().set_id(crate::de::hysky::skyblocker::utils::entity_utils::PLACEHOLDER_ID);
            }
            compat::shared(player)
        };

        // java: List<ItemStack> list = ItemUtils.getArmor(CLIENT.player);
        let mut list: Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack> = mcrust::mcrust::modding::game::with_player(|player| ItemUtils::get_armor(player)).expect("CLIENT.player");
        // new ItemStack[4] - every element is written below, before any read
        let mut armor: Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack> = (0..4).map(|_| mcrust::net::minecraft::world::item::item_stack::empty()).collect();
        for i in 0..list.len() {
            let copy = list[i].copy();
            armor[3 - i] = copy.clone();
            compat::write(&player, |p| mcrust::net::minecraft::world::entity::living_entity::LivingEntity::set_item_slot(p, ARMOR_SLOTS[i], copy));
        }
        let mut selected_slot: i32 = 0;
        while selected_slot < (armor.len() as i32) - 1 && !ArmorTab::can_edit(&armor[selected_slot as usize]) {
            selected_slot += 1;
        }
        let nothing_customizable = !ArmorTab::can_edit(&armor[selected_slot as usize]);

        // java: PlayerWidget playerWidget = new PlayerWidget(0, 0, 84, 165, player);
        let player_widget = PlayerWidget::new(0, 0, 84, 165, player.clone());
        let player_widget_index = screen.add_widget(compat::object_widget(player_widget, ""));

        // 444 results in a head selection widget of 330,
        // for which subtracting 10 pixels of padding and scrollbar,
        // gives a perfect 16 column head grid
        let width: i32 = 444.min(screen.width) - PLAYER_WIDGET_WIDTH - PADDING * 3;
        let head_selection_widget = HeadSelectionWidget::new(0, 0, width, 165);
        let head_selection_index = screen.add_widget(HeadSelectionWidget::widget(&head_selection_widget));

        let container_width: i32 = ((width as f32) * (1f32 / 3f32)) as i32;
        let trim_selection_widget = TrimSelectionWidget::new(0, 0, width - container_width - PADDING / 2, 80);

        let font = compat::font();
        let color_selection_widget = ColorSelectionWidget::new(0, 0, width, 100, &font);

        // the tab as one object (its inner classes hold it)
        let parent_ref = Arc::downgrade(parent);
        let this: compat::Shared<ArmorTab> = Arc::new_cyclic(|weak: &compat::WeakShared<ArmorTab>| {
            // java: PieceSelectionWidget pieceSelectionWidget = new PieceSelectionWidget(0, 0);
            let piece_selection_widget = PieceSelectionWidget::new(0, 0, weak.clone(), &armor);
            let piece_selection_index = screen.add_widget(compat::object_widget(piece_selection_widget, ""));
            // java: modelFieldContainer = layoutWidget.addChild(new ModelFieldContainer(containerWidth, 80));
            let model_field_container = ModelFieldContainer::new(container_width, 80, weak.clone());
            let model_field_index = screen.add_widget(compat::object_widget(model_field_container.clone(), ""));
            let trim_selection_index = screen.add_widget(compat::object_widget(trim_selection_widget.clone(), "Trim Selection"));
            let color_selection_index = screen.add_widget(compat::object_widget(color_selection_widget.clone(), "ColorSelectionWidget"));
            let nothing_customizable_index = if nothing_customizable {
                Some(screen.add_widget(compat::string_widget(0, 0, -1, 9, component::translatable("skyblocker.customization.nothingCustomizable"))))
            } else {
                None
            };
            parking_lot::ReentrantMutex::new(std::cell::RefCell::new(ArmorTab {
                base: GridLayoutTab::new(FormattedText::from(component::translatable("skyblocker.customization.armor"))),
                armor,
                parent: parent_ref,
                selected_slot,
                trim_selection_widget,
                color_selection_widget,
                head_selection_widget,
                model_field_container,
                nothing_customizable,
                player,
                widgets: ArmorTabWidgets {
                    player_widget: player_widget_index,
                    piece_selection_widget: piece_selection_index,
                    head_selection_widget: head_selection_index,
                    model_field_container: model_field_index,
                    trim_selection_widget: trim_selection_index,
                    color_selection_widget: color_selection_index,
                    nothing_customizable: nothing_customizable_index,
                },
            }))
        });
        let grid = compat::read(&this, |__t| __t.build_grid(screen));
        compat::write(&this, |__t| __t.base.layout = grid);

        ArmorTab::update_widgets(&this);
        this
    }

    /// The tab's grid (`layout.rowSpacing(PADDING / 2).columnSpacing(PADDING)` and the constructor's `addChild`s), over
    /// the widgets' sizes now (the port's layout leaves keep the size they were given, Java's read the widget's).
    fn build_grid(&self, screen: &mcrust::net::minecraft::client::gui::screens::screen::Screen) -> GridLayout {
        let leaf = |index: usize| {
            let widget = screen.widget(index);
            LayoutElement::widget(index, widget.get_width(), widget.get_height())
        };
        let mut layout = GridLayout::new_default();
        layout.row_spacing(PADDING / 2);
        layout.column_spacing(PADDING);

        // java: LinearLayout vertical = LinearLayout.vertical().spacing(1); vertical.addChild(playerWidget); vertical.addChild(pieceSelectionWidget);
        let mut vertical = LinearLayout::vertical().spacing(1);
        vertical.add_child(leaf(self.widgets.player_widget));
        vertical.add_child(leaf(self.widgets.piece_selection_widget));
        layout.add_child_span_adjusted(LayoutElement::Grid(vertical.wrapped), 0, 0, 2, 1, &mut |s| *s = s.clone().align_vertically_middle()).expect("GridLayout.addChild");

        // java: layout.addChild(headSelectionWidget, 0, 1, 2, 1, LayoutSettings::alignVerticallyMiddle);
        layout.add_child_span_adjusted(leaf(self.widgets.head_selection_widget), 0, 1, 2, 1, &mut |s| *s = s.clone().align_vertically_middle()).expect("GridLayout.addChild");

        // java: LinearLayout layoutWidget = LinearLayout.horizontal().spacing(PADDING / 2); .. layoutWidget.arrangeElements(); layout.addChild(layoutWidget, 0, 1);
        let mut layout_widget = LinearLayout::horizontal().spacing(PADDING / 2);
        layout_widget.add_child(leaf(self.widgets.model_field_container));
        layout_widget.add_child(leaf(self.widgets.trim_selection_widget));
        layout_widget.arrange_elements();
        layout.add_child_at(LayoutElement::Grid(layout_widget.wrapped), 0, 1).expect("GridLayout.addChild");

        // java: layout.addChild(colorSelectionWidget, 1, 1);
        layout.add_child_at(leaf(self.widgets.color_selection_widget), 1, 1).expect("GridLayout.addChild");

        if let Some(text) = self.widgets.nothing_customizable {
            layout.add_child_span_adjusted(leaf(text), 0, 1, 2, 1, &mut |p| *p = p.clone().align_vertically_middle().align_horizontally_center()).expect("GridLayout.addChild");
        }
        layout
    }

    /// java: private static boolean canEdit(ItemStack stack)
    pub fn can_edit(stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack) -> bool {
        let has_uuid: bool = !stack.get_uuid().is_empty();
        if stack.is_item(&items::items().get("player_head")) {
            return has_uuid;
        }
        return stack.is_in_tag(&mcrust::net::minecraft::tags::item_tags::trimmable_armor()) && has_uuid;
    }

    /// java: private void updateWidgets()
    ///
    /// (On the handle: the model field's `setValue` runs its callback, which reads this tab.)
    pub fn update_widgets(this: &compat::Shared<Self>) {
        let (nothing_customizable, head, trim, color, model, parent) = compat::read(this, |__t| {
            (__t.nothing_customizable, __t.head_selection_widget.clone(), __t.trim_selection_widget.clone(), __t.color_selection_widget.clone(), __t.model_field_container.clone(), __t.parent.clone())
        });
        if nothing_customizable {
            compat::write(&head, |__t| __t.base.base.visible = false);
            compat::write(&trim, |__t| __t.base.visible = false);
            compat::write(&color, |__t| __t.base.visible = false);
            compat::write(&model, |__t| __t.base.visible = false);
            return;
        }
        let item = compat::read(this, |__t| __t.armor[__t.selected_slot as usize].clone());
        if let Some(parent) = parent.upgrade() {
            CustomizeScreen::backup_configs(&parent, &item);
        }
        let is_player_head: bool = item.is_item(&items::items().get("player_head"));
        HeadSelectionWidget::set_current_item(&head, item.clone());
        compat::write(&trim, |__t| __t.set_current_item(item.clone()));
        compat::write(&color, |__t| __t.set_current_item(item.clone()));
        compat::write(&head, |__t| __t.base.base.visible = is_player_head);
        compat::write(&trim, |__t| __t.base.visible = !is_player_head);
        compat::write(&color, |__t| __t.base.visible = !is_player_head);
        compat::write(&model, |__t| __t.base.visible = !is_player_head);
        let uuid: String = item.get_uuid();
        // java: modelFieldContainer.field.setValue(..);
        let value = match SkyblockerConfigManager::get().general.custom_armor_model.get(&uuid) {
            Some(identifier) => identifier.to_string(),
            None => String::new(),
        };
        compat::write(&model, |__t| __t.set_field_value(&value));
    }

    /// java: void tick()
    pub fn tick(&self) {
        // java: player.tickCount++;
        compat::write(&self.player, |p| p.base_mut().tick_count += 1);
    }

    /// java: @Override public void doLayout(ScreenRectangle tabArea)
    pub fn do_layout(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, tab_area: ScreenRectangle) {
        let width: i32 = 444.min(tab_area.width()) - PLAYER_WIDGET_WIDTH - PADDING * 3;
        // java: headSelectionWidget.setWidth(width);
        HeadSelectionWidget::set_width_impl(&self.head_selection_widget, width);
        let head_width = compat::read(&self.head_selection_widget, |__t| __t.get_width());
        screen.widget_mut(self.widgets.head_selection_widget).width = head_width;
        let model_field_width: i32 = ((width as f32) * (1f32 / 3f32)) as i32;
        // java: trimSelectionWidget.setWidth(width - modelFieldWidth - PADDING / 2);
        compat::write(&self.trim_selection_widget, |__t| compat::ObjectWidget::set_width(__t, width - model_field_width - PADDING / 2));
        compat::push_object_box(&self.trim_selection_widget, screen.widget_mut(self.widgets.trim_selection_widget));
        // java: modelFieldContainer.setWidth(modelFieldWidth);
        compat::write(&self.model_field_container, |__t| compat::ObjectWidget::set_width(__t, model_field_width));
        compat::push_object_box(&self.model_field_container, screen.widget_mut(self.widgets.model_field_container));
        // java: colorSelectionWidget.setWidth(width);
        compat::write(&self.color_selection_widget, |__t| compat::ObjectWidget::set_width(__t, width));
        compat::push_object_box(&self.color_selection_widget, screen.widget_mut(self.widgets.color_selection_widget));
        // java: super.doLayout(tabArea); - over the grid as the widgets now are
        self.base.layout = self.build_grid(screen);
        self.base.do_layout(screen, tab_area);
    }

    /// java: Tab.visitChildren - `layout.visitWidgets(..)`
    pub fn visit_children(&self) -> Vec<usize> {
        self.base.visit_children()
    }

    /// java: public void recreate()
    pub fn recreate(&self) {
        let timeline = compat::read(&self.color_selection_widget, |__t| __t.get_timeline_widget());
        compat::write(&timeline, |__t| __t.recreate_image());
    }

    /// java: @Override public void close()
    pub fn close(&mut self) {
        compat::write(&self.color_selection_widget, |__t| __t.close());
    }
}

/// java: private class PieceSelectionWidget extends AbstractWidget
pub struct PieceSelectionWidget {
    /// java: extends net.minecraft.client.gui.components.AbstractWidget
    pub base: AbstractContainerWidget,
    /// java: ArmorTab.this
    pub tab: compat::WeakShared<ArmorTab>,
    /// java: private final boolean[] selectable;
    pub selectable: Vec<bool>,
}

impl PieceSelectionWidget {
    /// java: private PieceSelectionWidget(int x, int y)
    pub fn new(x: i32, y: i32, tab: compat::WeakShared<ArmorTab>, armor: &[mcrust::net::minecraft::world::item::item_stack::ItemStack]) -> compat::Shared<Self> {
        // java: super(x, y, 84, 24, Component.nullToEmpty(""));
        // java: selectable = new boolean[armor.length]; for (..) selectable[i] = canEdit(armor[i]);
        let selectable: Vec<bool> = armor.iter().map(ArmorTab::can_edit).collect();
        compat::shared(PieceSelectionWidget { base: AbstractContainerWidget::new(x, y, 84, 24, FormattedText::of("")), tab, selectable })
    }
}

impl compat::ObjectWidget for PieceSelectionWidget {
    fn object_box(&mut self) -> &mut AbstractContainerWidget {
        &mut self.base
    }

    fn object_box_ref(&self) -> &AbstractContainerWidget {
        &self.base
    }

    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_widget_render_state(&mut self, _widget: &AbstractWidget, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, _a: f32) {
        let Some(tab) = self.tab.upgrade() else {
            return;
        };
        let (x, y) = (self.base.get_x(), self.base.get_y());
        gui_graphics_extractor::blit_sprite_identifier(context.gl, context.minecraft, mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), PIECE_SELECTION_WIDGET_HOTBAR_TEXTURE.get(), x + 1, y + 1, 82, 22);

        compat::read(&tab, |tab| {
            let mut hovered_slot: i32 = -1;
            let local_x: i32 = mouse_x - x - 2;
            let local_y: i32 = mouse_y - y - 2;
            if local_y >= 0 && local_y < 20 {
                hovered_slot = if local_x / 20 >= tab.armor.len() as i32 { -1 } else { local_x / 20 };
            }

            if hovered_slot >= 0 && self.selectable[hovered_slot as usize] {
                let i: i32 = x + 2 + hovered_slot * 20;
                context.graphics.fill(context.gl, i, y + 2, i + 20, y + 22, 0x20_FF_FF_FF);
            }

            for i in 0..tab.armor.len() {
                context.graphics.item(&tab.armor[i], x + 4 + (i as i32) * 20, y + 4);
                if !self.selectable[i] && !tab.armor[i].is_empty() {
                    context.graphics.item(BARRIER.get(), x + 4 + (i as i32) * 20, y + 4);
                }
            }
            gui_graphics_extractor::blit_sprite_identifier(context.gl, context.minecraft, mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), PIECE_SELECTION_WIDGET_HOTBAR_SELECTION_TEXTURE.get(), x + tab.selected_slot * 20, y, 24, 24);
        });
        // java: this.handleCursor(graphics); - asked by the game after the frame (`handle_cursor`)
    }

    /// java: @Override public void onClick(MouseButtonEvent click, boolean doubled)
    fn on_click(&mut self, _widget: &mut AbstractWidget, event: &MouseButtonEvent, _double_click: bool) {
        let Some(tab) = self.tab.upgrade() else {
            return;
        };
        let local_x: f64 = event.x() - self.base.get_x() as f64 - 2.0;
        let local_y: f64 = event.y() - self.base.get_y() as f64 - 2.0;
        if local_y < 0.0 || local_y >= 20.0 {
            return;
        }
        let i: i32 = (local_x / 20.0) as i32;
        let armor_length = compat::read(&tab, |__t| __t.armor.len() as i32);
        if i < 0 || i >= armor_length || !self.selectable[i as usize] {
            return;
        }
        if i != compat::read(&tab, |__t| __t.selected_slot) {
            compat::write(&tab, |__t| __t.selected_slot = i);
            ArmorTab::update_widgets(&tab);
        }
    }

    /// java: @Override public boolean isMouseOver(double mouseX, double mouseY)
    fn is_mouse_over(&self, mouse_x: f64, mouse_y: f64) -> bool {
        return self.base.active && self.base.visible && mouse_x >= (self.base.get_x() + 2) as f64 && mouse_y >= (self.base.get_y() + 2) as f64 && mouse_x < (self.base.get_right() - 2) as f64 && mouse_y < (self.base.get_bottom() - 2) as f64;
    }

    /// `this.handleCursor(graphics)` from `extractWidgetRenderState` - `AbstractWidget.handleCursor:68-72`
    fn handle_cursor(&self, widget: &AbstractWidget, cursors: &CursorTypes) -> Option<CursorType> {
        widget.handle_cursor(cursors)
    }
}

/// java: private class ModelFieldContainer extends AbstractContainerWidget
pub struct ModelFieldContainer {
    /// java: extends net.minecraft.client.gui.components.AbstractContainerWidget
    pub base: AbstractContainerWidget,
    /// java: private final Component text = Component.translatable("skyblocker.customization.armor.modelOverride").withStyle(ChatFormatting.ITALIC).withStyle(ChatFormatting.GRAY);
    pub text: MutableComponent,
    /// java: private final FrameLayout containerLayout; (arranged over the field's live size - see `arrange`)
    pub container_layout: LayoutElement,
    /// `containerLayout`'s minimum width (`setMinWidth`)
    pub container_min_width: i32,
    /// java: private final IdentifierTextField field; - the subclass's state; its `EditBox` is `children`'s widget 0
    pub field: IdentifierTextField,
    /// java: children() - `visible ? List.of(field) : List.of()`
    pub children: mcrust::net::minecraft::client::gui::screens::screen::Screen,
}

impl ModelFieldContainer {
    /// java: private ModelFieldContainer(int width, int height)
    pub fn new(width: i32, height: i32, tab: compat::WeakShared<ArmorTab>) -> compat::Shared<Self> {
        // java: super(0, 0, width, height, Component.empty(), AbstractScrollArea.defaultSettings(4));
        let base = AbstractContainerWidget::new_with_scrollbar_settings(0, 0, width, height, FormattedText::Empty, ScrollbarSettings::default_settings(4.0));
        // java: field = containerLayout.addChild(new IdentifierTextField(width - 10, 20, identifier -> {..}));
        let (field, mut field_widget) = IdentifierTextField::new(
            width - 10,
            20,
            Arc::new(move |identifier: Option<Identifier>| {
                let Some(tab) = tab.upgrade() else {
                    return;
                };
                let uuid: String = compat::read(&tab, |__t| __t.armor[__t.selected_slot as usize].get_uuid());
                if uuid.is_empty() {
                    return;
                }
                SkyblockerConfigManager::update_only(Arc::new(move |config: &mut crate::de::hysky::skyblocker::config::skyblocker_config::SkyblockerConfig| match &identifier {
                    None => {
                        config.general.custom_armor_model.remove(&uuid);
                    }
                    Some(identifier) => {
                        config.general.custom_armor_model.insert(uuid.clone(), identifier.clone());
                    }
                }));
                let color = compat::read(&tab, |__t| __t.color_selection_widget.clone());
                compat::write(&color, |__t| __t.refresh());
            }),
        );
        // java: field.setTooltip(Tooltip.create(Component.translatable("skyblocker.customization.armor.modelOverride.tooltip")));
        field_widget.set_tooltip_text(FormattedText::from(component::translatable("skyblocker.customization.armor.modelOverride.tooltip")));
        // java: field.setTooltipDelay(Duration.ofMillis(400));
        field_widget.set_tooltip_delay(400);
        let mut children = mcrust::net::minecraft::client::gui::screens::screen::Screen::new(String::new());
        children.add_renderable_widget(field_widget);
        let mut this = ModelFieldContainer {
            base,
            text: component::translatable("skyblocker.customization.armor.modelOverride").with_style_format(ChatFormatting::Italic).with_style_format(ChatFormatting::Gray),
            container_layout: LayoutElement::Frame(FrameLayout::new()),
            container_min_width: 0,
            field,
            children,
        };
        // java: containerLayout.arrangeElements();
        this.arrange();
        compat::shared(this)
    }

    /// `containerLayout` (a `FrameLayout` holding the field) arranged over the field's size now.
    fn arrange(&mut self) {
        let field = self.children.widget(0);
        let mut layout = FrameLayout::new();
        layout.set_min_width(self.container_min_width);
        layout.add_child(LayoutElement::widget(0, field.get_width(), field.get_height()));
        let (x, y) = (self.container_layout.get_x(), self.container_layout.get_y());
        let mut layout = LayoutElement::Frame(layout);
        layout.arrange_elements();
        layout.set_position(x, y);
        self.container_layout = layout;
        self.place_field();
    }

    /// The layout's position written onto the field.
    fn place_field(&mut self) {
        let mut placed = Vec::new();
        self.container_layout.visit_widgets(&mut placed);
        for (index, x, y) in placed {
            self.children.widget_mut(index).set_position(x, y);
        }
    }

    /// `field.setValue(value)`
    pub fn set_field_value(&mut self, value: &str) {
        compat::set_edit_box_value(self.children.widget_mut(0), value);
    }

    /// The field's `setFocused` override, where this container sees its focus go (`children`'s focus moved).
    fn field_focus_changed(&mut self, was_focused: bool) {
        let is_focused = self.children.get_focused() == Some(0);
        if was_focused && !is_focused {
            let field = &self.field;
            field.set_focused(self.children.widget_mut(0), false);
        }
    }

    /// java: private void extractLabel(ActiveTextCollector drawer)
    fn extract_label(&self, context: &WidgetRenderContext<'_>) {
        let padding = 5;
        let start_y = self.base.get_y() + padding;
        compat::accept_scrolling_with_default_center(context, &FormattedText::from(self.text.clone()), self.base.get_x() + padding, self.base.get_right() - padding, start_y, start_y + 9);
    }
}

impl compat::ObjectWidget for ModelFieldContainer {
    fn object_box(&mut self) -> &mut AbstractContainerWidget {
        &mut self.base
    }

    fn object_box_ref(&self) -> &AbstractContainerWidget {
        &self.base
    }

    /// java: @Override public void setX(int x)
    fn set_x(&mut self, x: i32) {
        self.base.set_x(x);
        // java: FrameLayout.alignInDimension(getX(), getWidth(), containerLayout.getWidth(), containerLayout::setX, 0.5f);
        let new_x = frame_layout::align_in_dimension(self.base.get_x(), self.base.get_width(), self.container_layout.get_width(), 0.5f32);
        self.container_layout.set_x(new_x);
        self.place_field();
    }

    /// java: @Override public void setWidth(int width)
    fn set_width(&mut self, width: i32) {
        self.base.set_width(width);
        // java: containerLayout.setMinWidth(width); containerLayout.arrangeElements(); field.setWidth(width - 10);
        self.container_min_width = width;
        self.arrange();
        self.children.widget_mut(0).set_width(width - 10);
        compat::with_edit_box(self.children.widget_mut(0), |e| e.set_width(width - 10));
        // java: setX(getX());
        let x = self.base.get_x();
        compat::ObjectWidget::set_x(self, x);
    }

    /// java: @Override public void setY(int y)
    fn set_y(&mut self, y: i32) {
        self.base.set_y(y);
        // java: FrameLayout.alignInDimension(getY(), getHeight(), containerLayout.getHeight(), containerLayout::setY, 0.5f);
        let new_y = frame_layout::align_in_dimension(self.base.get_y(), self.base.get_height(), self.container_layout.get_height(), 0.5f32);
        self.container_layout.set_y(new_y);
        self.place_field();
    }

    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_widget_render_state(&mut self, _widget: &AbstractWidget, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, _a: f32) {
        if !self.base.visible {
            return;
        }
        gui_graphics_extractor::blit_sprite_identifier(context.gl, context.minecraft, mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), INNER_SPACE_TEXTURE.get(), self.base.get_x(), self.base.get_y(), self.base.get_width(), self.base.get_height());
        // java: this.field.extractRenderState(graphics, mouseX, mouseY, a);
        context.extract_widgets(&mut self.children, mouse_x, mouse_y);
        // java: this.extractLabel(graphics.textRenderer(GuiGraphicsExtractor.HoveredTextEffects.NONE));
        self.extract_label(context);
    }

    /// ContainerEventHandler.mouseClicked:39-55 over `children()` (the field while visible)
    fn mouse_clicked(&mut self, _widget: &mut AbstractWidget, event: &MouseButtonEvent, _double_click: bool) -> bool {
        if !self.base.visible {
            return false;
        }
        let was_focused = self.children.get_focused() == Some(0);
        let consumed = compat::children_mouse_clicked(&mut self.children, event);
        self.field_focus_changed(was_focused);
        consumed
    }

    /// ContainerEventHandler.mouseReleased:57-66
    fn mouse_released(&mut self, _widget: &mut AbstractWidget, event: &MouseButtonEvent) -> bool {
        compat::children_mouse_released(&mut self.children, event);
        false
    }

    /// ContainerEventHandler.mouseDragged:68-71
    fn mouse_dragged(&mut self, _widget: &mut AbstractWidget, event: &MouseButtonEvent, dx: f64, dy: f64) -> bool {
        compat::children_mouse_dragged(&mut self.children, event, dx, dy);
        false
    }

    /// ContainerEventHandler.keyPressed - the focused child
    fn key_pressed(&mut self, _widget: &mut AbstractWidget, event: &KeyEvent) -> bool {
        if !self.base.visible {
            return false;
        }
        compat::children_key_pressed(&mut self.children, event)
    }

    /// ContainerEventHandler.charTyped - the focused child
    fn char_typed(&mut self, _widget: &mut AbstractWidget, event: &CharacterEvent) -> bool {
        if !self.base.visible {
            return false;
        }
        compat::children_char_typed(&mut self.children, event)
    }
}

/// java: the anonymous `RemotePlayer` subclass of `ArmorTab.player`: invisible to everyone (no name tag), and only the
/// model parts the player's own options show, never the CAPE.
struct ArmorTabPlayer;

impl mcrust::mcrust::modding::entities::RemotePlayerSubclass for ArmorTabPlayer {
    fn is_model_part_shown(&self, _player: &RemotePlayer, part: mcrust::net::minecraft::world::entity::player::player_model_part::PlayerModelPart, _super_answer: bool) -> bool {
        // java: return modelPart != PlayerModelPart.CAPE && CLIENT.options.isModelPartEnabled(modelPart);
        part != mcrust::net::minecraft::world::entity::player::player_model_part::PlayerModelPart::Cape
            && mcrust::net::minecraft::client::options::is_model_part_enabled(&compat::minecraft().options.borrow(), part)
    }

    fn is_invisible_to(&self, _player: &RemotePlayer, _super_answer: bool) -> bool {
        true
    }
}
