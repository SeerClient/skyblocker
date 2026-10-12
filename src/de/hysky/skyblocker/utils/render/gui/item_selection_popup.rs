//! Ported from de.hysky.skyblocker.utils.render.gui.ItemSelectionPopup by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::itemlist::item_repository::ItemRepository;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
use crate::de::hysky::skyblocker::utils::item_utils;
use crate::de::hysky::skyblocker::utils::item_utils::ItemUtils;
use crate::de::hysky::skyblocker::utils::render::gui::abstract_popup_screen::AbstractPopupScreen;
use crate::de::hysky::skyblocker::utils::render::gui::abstract_popup_screen::AbstractPopupScreenBase;
use crate::de::hysky::skyblocker::utils::render::gui::abstract_selection_popup::AbstractSelectionPopup;
use crate::de::hysky::skyblocker::utils::render::gui::abstract_selection_popup::AbstractSelectionPopupBase;
use mcrust::java::text::decimal_format_symbols::Locale;
use mcrust::mcrust::modding::screens::ModScreen;
use mcrust::mcrust::modding::screens::ScreenRenderContext;
use mcrust::mcrust::modding::widgets::ModWidget;
use mcrust::mcrust::modding::widgets::WidgetRenderContext;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::abstract_widget::WidgetKind;
use mcrust::net::minecraft::client::gui::components::tooltip::Tooltip;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::client::input::mouse_button_info::MouseButtonInfo;
use mcrust::net::minecraft::nbt::tag::Tag;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::world::item::component::custom_data::CustomData;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: public class ItemSelectionPopup extends AbstractSelectionPopup<ItemSelectionPopup.ItemWidget>
pub struct ItemSelectionPopup {
    /// java: extends AbstractSelectionPopup
    pub base: AbstractSelectionPopupBase<compat::Shared<ItemWidget>>,
    /// java: private final List<Pair<String, String>> skullIcons = List.of( Pair.of("Accessory Bag", "1a11a7f11bcd5784903c5201d08261c4df8379109d6e611c1cd3ededf031afed"), Pai
    pub skull_icons: Vec<(String, String)>,
    /// java: private final Predicate<FlexibleItemStack> filter;
    pub filter: Option<Arc<dyn Fn(compat::Shared<FlexibleItemStack>) -> bool>>,
    /// java: private final List<ItemWidget> items;
    pub items: Vec<compat::Shared<ItemWidget>>,
}

impl std::ops::Deref for ItemSelectionPopup {
    type Target = AbstractSelectionPopupBase<compat::Shared<ItemWidget>>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for ItemSelectionPopup {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl ItemSelectionPopup {
    /// java: public ItemSelectionPopup(Screen backgroundScreen, Consumer<@Nullable ItemStack> onDone)
    pub fn new(mut background_screen: mcrust::net::minecraft::client::gui::screens::screen::Screen, on_done: Arc<dyn Fn(Option<mcrust::net::minecraft::world::item::item_stack::ItemStack>)>) -> compat::Shared<Self> {
        // TODO(fabric2seer): the mod keeps the game's `Screen` (Java holds a reference to the live object); the port's `Screen` is not a value to copy and the port lends the mod no handle to it
        //   java: this(backgroundScreen, onDone, Predicates.alwaysTrue());
        let __this: compat::Shared<Self> = todo!("constructor");
        __this
    }

    /// java: public ItemSelectionPopup(Screen backgroundScreen, Consumer<@Nullable ItemStack> onDone, Predicate<FlexibleItemStack> filter)
    pub fn new_3(mut background_screen: mcrust::net::minecraft::client::gui::screens::screen::Screen, on_done: Arc<dyn Fn(Option<mcrust::net::minecraft::world::item::item_stack::ItemStack>)>, filter: Arc<dyn Fn(compat::Shared<FlexibleItemStack>) -> bool>) -> compat::Shared<Self> {
        let __init_filter = Some(filter.clone());
        // TODO(fabric2seer): the mod keeps the game's `Screen` (Java holds a reference to the live object); the port's `Screen` is not a value to copy and the port lends the mod no handle to it
        //   java: super(Component.literal("Select Item"), backgroundScreen, opt -> onDone.accept(opt.map(ItemWidget::getItem).orElse(null)), 20);
        let __this: compat::Shared<Self> = todo!("constructor");
        // TODO(fabric2seer): not a lambda
        //   java: this.items = Stream.concat(skullIcons.stream().map(pair -> {
        //   java: 			FlexibleItemStack skull = ItemUtils.createSkull(ItemUtils.toTextureBase64(pair.right()));
        //   java: 			skull.set(DataComponents.CUSTOM_NAME, Component.literal(pair.left()).withStyle(style -> style.withItalic(false)));
        //   java: 			return skull;
        //   java: 		}), ItemRepository.getItemsStream())
        //   java: 		.filter(ItemSelectionPopup.this.filter)
        //   java: 		.map(ItemWidget::new)
        //   java: 		.toList();
        __this
    }

}

impl AbstractSelectionPopup<compat::Shared<crate::de::hysky::skyblocker::utils::render::gui::item_selection_popup::ItemWidget>> for ItemSelectionPopup {
    fn abstract_selection_popup_base(&self) -> &AbstractSelectionPopupBase<compat::Shared<crate::de::hysky::skyblocker::utils::render::gui::item_selection_popup::ItemWidget>> {
        &self.base
    }
    fn abstract_selection_popup_base_mut(&mut self) -> &mut AbstractSelectionPopupBase<compat::Shared<crate::de::hysky::skyblocker::utils::render::gui::item_selection_popup::ItemWidget>> {
        &mut self.base
    }

    /// java: @Override protected Collection<ItemWidget> filterWidgets(String input)
    fn filter_widgets(&self, input: &str) -> Vec<compat::Shared<ItemWidget>> {
        let __o1 = w.clone();
        return self.items.iter().cloned().filter(|w| (compat::write(&__o1, |__t| compat::component_of(__t.get_message_component()))).get_string().to_lowercase().contains(&input.to_lowercase())).collect::<Vec<_>>();
    }

}

impl AbstractPopupScreen for ItemSelectionPopup {
    fn abstract_popup_screen_base(&self) -> &AbstractPopupScreenBase {
        &self.base.base
    }
    fn abstract_popup_screen_base_mut(&mut self) -> &mut AbstractPopupScreenBase {
        &mut self.base.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void extractBackground(GuiGraphicsExtractor context, int mouseX, int mouseY, float delta) (dispatch: the body for this class)
    fn extract_background(&self) -> fn(&compat::Shared<dyn AbstractPopupScreen>, &mut GuiGraphicsExtractor, i32, i32, f32) {
        <dyn AbstractSelectionPopup>::extract_background_from::<Self>
    }

    /// java: @Override public void init() (dispatch: the body for this class)
    fn init(&self) -> fn(&compat::Shared<dyn AbstractPopupScreen>) {
        <dyn AbstractSelectionPopup>::init_from::<Self>
    }

    /// java: @Override protected void repositionElements() (dispatch: the body for this class)
    fn reposition_elements(&self) -> fn(&compat::Shared<dyn AbstractPopupScreen>) {
        <dyn AbstractSelectionPopup>::reposition_elements_from::<Self>
    }

}

/// ItemSelectionPopup's `Screen` half (`super`'s fields and widget tree), lent by the game for each call
impl compat::ScreenHalf for ItemSelectionPopup {
    fn screen_half(&mut self) -> &mut mcrust::net::minecraft::client::gui::screens::screen::Screen {
        &mut self.base.base.base
    }
    fn screen_parts(&mut self) -> (&mut mcrust::net::minecraft::client::gui::screens::screen::Screen, Option<&mut compat::WidgetCache>) {
        let __s = &mut self.base.base;
        (&mut __s.base, Some(&mut __s.__widgets))
    }
}

/// ItemSelectionPopup as the game's screen: `game::set_screen(Box::new(compat::ScreenAdapter::new(..)))`
impl ModScreen for compat::ScreenAdapter<ItemSelectionPopup> {
    fn get_title(&self) -> String {
        self.title()
    }

    /// java: @Override public void init()
    fn init(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        self.lend(screen, |__h| ({ let __f = compat::dispatch(__h, |__t| __t.init()); __f(&compat::view(__h, |a| a as compat::Shared<dyn AbstractPopupScreen>)) }))
    }

    /// java: @Override protected void repositionElements()
    fn reposition_elements(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        self.lend(screen, |__h| ({ let __f = compat::dispatch(__h, |__t| __t.reposition_elements()); __f(&compat::view(__h, |a| a as compat::Shared<dyn AbstractPopupScreen>)) }))
    }

    /// java: Screen.tick (the base body)
    fn tick(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        self.lend(screen, |_| ())
    }

    /// java: @Override public void added()
    fn added(&mut self) {
        self.call(|__h| ({ let __f = compat::dispatch(__h, |__t| __t.added()); __f(&compat::view(__h, |a| a as compat::Shared<dyn AbstractPopupScreen>)) }))
    }

    /// java: Screen.keyPressed (the base body)
    fn key_pressed(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, event: &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        self.lend(screen, |_| ());
        if event.is_escape() && self.should_close_on_esc() {
            self.on_close();
            return true;
        }
        false
    }

    /// java: Screen.mouseClicked (the base body)
    fn mouse_clicked(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, x: f64, y: f64, button: i32) -> bool {
        self.lend(screen, |_| false)
    }

    /// java: @Override public void onClose()
    fn on_close(&mut self) {
        self.call(|__h| ({ let __f = compat::dispatch(__h, |__t| __t.on_close()); __f(&compat::view(__h, |a| a as compat::Shared<dyn AbstractPopupScreen>)) }))
    }

}

/// java: protected class ItemWidget extends AbstractWidget
#[derive(Clone)]
pub struct ItemWidget {
    /// java: extends net.minecraft.client.gui.components.AbstractWidget
    pub base: AbstractWidget,
    /// java: private final ItemStack item;
    pub item: mcrust::net::minecraft::world::item::item_stack::ItemStack,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl std::ops::Deref for ItemWidget {
    type Target = AbstractWidget;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for ItemWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl ItemWidget {
    /// java: private ItemWidget(FlexibleItemStack stack)
    pub fn new(mut stack: compat::Shared<FlexibleItemStack>) -> compat::Shared<Self> {
        let __init_item = mcrust::net::minecraft::world::item::item_stack::ItemStack::new(&FlexibleItemStack::type_holder(&stack));
        let __this = compat::shared(Self {
            base: AbstractWidget::new(0, 0, 20, 20, (&component::literal(&compat::component_of(FlexibleItemStack::get_stack_or_throw(&stack).get_hover_name()).get_string()).get_string()).to_string(), WidgetKind::PlainText, OnPress::Inert),
            self_ref: std::sync::Weak::new(),
            item: __init_item,
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        { let (__q0, __q1,) = (mcrust::net::minecraft::core::component::data_components::profile().clone(), &stack,); __this.lock().borrow_mut().item.copy_from(__q0, __q1) };
        { let (__q0, __q1,) = (mcrust::net::minecraft::core::component::data_components::item_model().clone(), &stack,); __this.lock().borrow_mut().item.copy_from(__q0, __q1) };
        { let (__q0, __q1,) = (mcrust::net::minecraft::core::component::data_components::banner_patterns().clone(), &stack,); __this.lock().borrow_mut().item.copy_from(__q0, __q1) };
        let mut item_id: String = stack.lock().borrow().get_skyblock_id();
        let mut custom_data: Tag = mcrust::net::minecraft::nbt::compound_tag::Tag::compound();
        custom_data.put_string(item_utils::ID, &item_id);
        { let (__q0, __q1,) = (mcrust::net::minecraft::core::component::data_components::custom_data().clone(), Some(CustomData::of(&custom_data)),); __this.lock().borrow_mut().item.set(__q0, __q1) };
        { let (__q0,) = (Tooltip::create(compat::component_of(__this.lock().borrow().get_message_component()).get_string()),); __this.lock().borrow_mut().set_tooltip(__q0) };
        __this
    }

    /// java: protected ItemStack getItem()
    pub fn get_item(__this: &compat::Shared<Self>) -> mcrust::net::minecraft::world::item::item_stack::ItemStack {
        return compat::read(__this, |__t| __t.item.clone());
    }

    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float deltaTicks)
    pub fn extract_widget_render_state(__this: &compat::Shared<Self>, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut delta_ticks: f32) {
        graphics.item(&compat::read(__this, |__t| __t.item.clone()), (__this.lock().borrow().get_x()).wrapping_add(2), (__this.lock().borrow().get_y()).wrapping_add(2));
        // TODO(fabric2seer): the condition: `selectedItem` is a field of the enclosing instance (ItemSelectionPopup.this) - an inner class has no outer reference in Rust
        //   java: selectedItem == this
        if todo!("fabric2seer: the condition (TODO above)") {
            graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), __this.lock().borrow().get_x(), __this.lock().borrow().get_y(), (__this.lock().borrow().get_x()).wrapping_add(__this.lock().borrow().get_width()), (__this.lock().borrow().get_y()).wrapping_add(__this.lock().borrow().get_height()), 0x3000FF00);
        }
        if __this.lock().borrow().is_hovered() {
            graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), __this.lock().borrow().get_x(), __this.lock().borrow().get_y(), (__this.lock().borrow().get_x()).wrapping_add(__this.lock().borrow().get_width()), (__this.lock().borrow().get_y()).wrapping_add(__this.lock().borrow().get_height()), 0x20FFFFFF);
        }
    }

    /// java: @Override public void onClick(MouseButtonEvent click, boolean doubled)
    pub fn on_click(__this: &compat::Shared<Self>, click: &MouseButtonEvent, mut doubled: bool) {
        // TODO(fabric2seer): `AbstractWidget.onClick` has no Rust method in the port (looked for `on_click` in AbstractWidget and its supertypes)
        // rust: __this.lock().borrow_mut().base.on_click(click, doubled);
        // TODO(fabric2seer): `setSelectedItem(..)` is a method of the enclosing instance (ItemSelectionPopup.this) - an inner class has no outer reference in Rust
        //   java: setSelectedItem(this);
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    pub fn update_widget_narration(__this: &compat::Shared<Self>, builder: &dyn NarrationElementOutput) {
    }

}

/// ItemWidget's widget half (`super`'s fields: x, y, size, message, ...), lent by the game for each call
impl compat::WidgetHalf for ItemWidget {
    fn widget_half(&mut self) -> &mut AbstractWidget {
        &mut self.base
    }
}

/// ItemWidget as the game's widget: `compat::WidgetAdapter::widget(handle)` where Java hands it on
impl ModWidget for compat::WidgetAdapter<ItemWidget> {
    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float deltaTicks)
    fn extract_widget_render_state(&mut self, widget: &AbstractWidget, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        self.lend_ref(widget, |__h| ItemWidget::extract_widget_render_state(__h, &mut context.graphics, mouse_x, mouse_y, a))
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    fn update_widget_narration(&mut self, widget: &AbstractWidget, output: &mut dyn NarrationElementOutput) {
        self.lend_ref(widget, |__h| ItemWidget::update_widget_narration(__h, output))
    }

    /// java: @Override public void onClick(MouseButtonEvent click, boolean doubled)
    fn on_click(&mut self, widget: &mut AbstractWidget, event: &MouseButtonEvent, double_click: bool) {
        self.lend(widget, |__h| ItemWidget::on_click(__h, event, double_click))
    }

}

// NOTE(fabric2seer): extractWidgetRenderState, onClick, updateWidgetNarration override(s) of AbstractWidget - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them
