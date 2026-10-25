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
    ///
    /// (open it with `game::set_screen_keeping_parent`: the screen it is opened over is its background screen, which
    /// the base keeps a stand-in for - see `AbstractPopupScreenBase::new_in`)
    pub fn new(background_screen: mcrust::net::minecraft::client::gui::screens::screen::Screen, on_done: Arc<dyn Fn(Option<mcrust::net::minecraft::world::item::item_stack::ItemStack>)>) -> compat::Shared<Self> {
        // java: this(backgroundScreen, onDone, Predicates.alwaysTrue());
        ItemSelectionPopup::new_3(background_screen, on_done, Arc::new(|_: compat::Shared<FlexibleItemStack>| true))
    }

    /// java: public ItemSelectionPopup(Screen backgroundScreen, Consumer<@Nullable ItemStack> onDone, Predicate<FlexibleItemStack> filter)
    pub fn new_3(background_screen: mcrust::net::minecraft::client::gui::screens::screen::Screen, on_done: Arc<dyn Fn(Option<mcrust::net::minecraft::world::item::item_stack::ItemStack>)>, filter: Arc<dyn Fn(compat::Shared<FlexibleItemStack>) -> bool>) -> compat::Shared<Self> {
        // java: super(Component.literal("Select Item"), backgroundScreen, opt -> onDone.accept(opt.map(ItemWidget::getItem).orElse(null)), 20);
        let __on_done: Arc<dyn Fn(Option<compat::Shared<ItemWidget>>)> = Arc::new(move |opt: Option<compat::Shared<ItemWidget>>| on_done(opt.map(|__w| ItemWidget::get_item(&__w))));
        let __filter = filter.clone();
        let (_, __this) = AbstractSelectionPopupBase::new_in(component::literal("Select Item"), background_screen, __on_done, 20, move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                skull_icons: SKULL_ICONS_LIST(),
                filter: Some(__filter),
                items: Vec::new(),
            });
            ((__h.clone() as compat::Shared<dyn AbstractSelectionPopup<compat::Shared<ItemWidget>>>), __h)
        });
        // java: this.items = Stream.concat(skullIcons.stream().map(pair -> {
        //   FlexibleItemStack skull = ItemUtils.createSkull(ItemUtils.toTextureBase64(pair.right()));
        //   skull.set(DataComponents.CUSTOM_NAME, Component.literal(pair.left()).withStyle(style -> style.withItalic(false)));
        //   return skull; }), ItemRepository.getItemsStream()).filter(ItemSelectionPopup.this.filter).map(ItemWidget::new).toList();
        let skulls: Vec<compat::Shared<FlexibleItemStack>> = compat::read(&__this, |__t| __t.skull_icons.clone()).into_iter().map(|(name, texture)| {
            let skull = ItemUtils::create_skull(&ItemUtils::to_texture_base64(&texture));
            FlexibleItemStack::set(&skull, mcrust::net::minecraft::core::component::data_components::custom_name().clone(), mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::literal(&name).with_style_fn(|style| style.with_italic(Some(false)))));
            skull
        }).collect();
        let outer = std::sync::Arc::downgrade(&__this);
        let items: Vec<compat::Shared<ItemWidget>> = skulls.into_iter().chain(ItemRepository::get_items_stream()).filter(|__s| filter(__s.clone())).map(|__s| ItemWidget::new(outer.clone(), __s)).collect();
        compat::write(&__this, |__t| __t.items = items);
        __this
    }

}

/// java: private final List<Pair<String, String>> skullIcons = List.of(..);
#[allow(non_snake_case)]
fn SKULL_ICONS_LIST() -> Vec<(String, String)> {
        vec![
            ("Accessory Bag".to_string(), "1a11a7f11bcd5784903c5201d08261c4df8379109d6e611c1cd3ededf031afed".to_string()),
            ("Sack of Sacks".to_string(), "80a077e248d142772ea800864f8c578b9d36885b29daf836b64a706882b6ec10".to_string()),
            ("Fishing Bag".to_string(), "eab810b1b4ff598d1d99336e71da4e356d3ad321e169dbd1fa9d506aae61717d".to_string()),
            ("Potion Bag".to_string(), "2fc626b3a12d099c44da04b5f15c95f2e360e0789bccecf842ef09c72b93d842".to_string()),
            ("Quiver".to_string(), "44e1843df8f91a7033e4c810f10b003e07ade5100079ac44df74a4e46a489447".to_string()),
            ("Personal Bank".to_string(), "e36e94f6c34a35465fce4a90f2e25976389eb9709a12273574ff70fd4daa6852".to_string()),
            ("Skyblock Hub".to_string(), "d7cc6687423d0570d556ac53e0676cb563bbdd9717cd8269bdebed6f6d4e7bf8".to_string()),
            ("Private Island".to_string(), "c9c8881e42915a9d29bb61a16fb26d059913204d265df5b439b3d792acd56".to_string()),
            ("Castle".to_string(), "f4559d75464b2e40a518e4de8e6cf3085f0a3ca0b1b7012614c4cd96fed60378".to_string()),
            ("Sirius' Shack".to_string(), "7ab83858ebc8ee85c3e54ab13aabfcc1ef2ad446d6a900e471c3f33b78906a5b".to_string()),
            ("Crypts".to_string(), "25d2f31ba162fe6272e831aed17f53213db6fa1c4cbe4fc827f3963cc98b9".to_string()),
            ("Spider's Den".to_string(), "c754318a3376f470e481dfcd6c83a59aa690ad4b4dd7577fdad1c2ef08d8aee6".to_string()),
            ("Top of the Nest".to_string(), "9d7e3b19ac4f3dee9c5677c135333b9d35a7f568b63d1ef4ada4b068b5a25".to_string()),
            ("The End".to_string(), "7840b87d52271d2a755dedc82877e0ed3df67dcc42ea479ec146176b02779a5".to_string()),
            ("The End Dragon's Nest".to_string(), "a1cd6d2d03f135e7c6b5d6cdae1b3a68743db4eb749faf7341e9fb347aa283b".to_string()),
            ("The Park".to_string(), "a221f813dacee0fef8c59f76894dbb26415478d9ddfc44c2e708a6d3b7549b".to_string()),
            ("The Park - Jungle".to_string(), "79ca3540621c1c79c32bf42438708ff1f5f7d0af9b14a074731107edfeb691c".to_string()),
            ("Howling Cave".to_string(), "1832d53997b451635c9cf9004b0f22bb3d99ab5a093942b5b5f6bb4e4de47065".to_string()),
            ("Gold Mines".to_string(), "73bc965d579c3c6039f0a17eb7c2e6faf538c7a5de8e60ec7a719360d0a857a9".to_string()),
            ("Deep Caverns".to_string(), "569a1f114151b4521373f34bc14c2963a5011cdc25a6554c48c708cd96ebfc".to_string()),
            ("The Barn".to_string(), "4d3a6bd98ac1833c664c4909ff8d2dc62ce887bdcf3cc5b3848651ae5af6b".to_string()),
            ("Mushroom Desert".to_string(), "6b20b23c1aa2be0270f016b4c90d6ee6b8330a17cfef87869d6ad60b2ffbf3b5".to_string()),
            ("Dungeon Hub".to_string(), "9b56895b9659896ad647f58599238af532d46db9c1b0389b8bbeb70999dab33d".to_string()),
            ("Dwarven Mines".to_string(), "51539dddf9ed255ece6348193cd75012c82c93aec381f05572cecf7379711b3b".to_string()),
            ("Heart Of The Mountain (HOTM)".to_string(), "86f06eaa3004aeed09b3d5b45d976de584e691c0e9cade133635de93d23b9edb".to_string()),
            ("Bazaar".to_string(), "c232e3820897429157619b0ee099fec0628f602fff12b695de54aef11d923ad7".to_string()),
            ("Museum".to_string(), "438cf3f8e54afc3b3f91d20a49f324dca1486007fe545399055524c17941f4dc".to_string()),
            ("Crystal Hollows".to_string(), "21dbe30b027acbceb612563bd877cd7ebb719ea6ed1399027dcee58bb9049d4a".to_string()),
            ("Dwarven Forge".to_string(), "5cbd9f5ec1ed007259996491e69ff649a3106cf920227b1bb3a71ee7a89863f".to_string()),
            ("Forgotten Skull".to_string(), "6becc645f129c8bc2faa4d8145481fab11ad2ee75749d628dcd999aa94e7".to_string()),
            ("Crystal Nucleus".to_string(), "34d42f9c461cee1997b67bf3610c6411bf852b9e5db607bbf626527cfb42912c".to_string()),
            ("Void Sepulture".to_string(), "eb07594e2df273921a77c101d0bfdfa1115abed5b9b2029eb496ceba9bdbb4b3".to_string()),
            ("Crimson Isle".to_string(), "c3687e25c632bce8aa61e0d64c24e694c3eea629ea944f4cf30dcfb4fbce071".to_string()),
            ("Trapper's Den".to_string(), "6102f82148461ced1f7b62e326eb2db3a94a33cba81d4281452af4d8aeca4991".to_string()),
            ("Arachne's Sanctuary".to_string(), "35e248da2e108f09813a6b848a0fcef111300978180eda41d3d1a7a8e4dba3c3".to_string()),
            ("Garden".to_string(), "f4880d2c1e7b86e87522e20882656f45bafd42f94932b2c5e0d6ecaa490cb4c".to_string()),
            ("Winter".to_string(), "6dd663136cafa11806fdbca6b596afd85166b4ec02142c8d5ac8941d89ab7".to_string()),
            ("Wizard Tower".to_string(), "838564e28aba98301dbda5fafd86d1da4e2eaeef12ea94dcf440b883e559311c".to_string()),
            ("Base Camp".to_string(), "2461ec3bd654f62ca9a393a32629e21b4e497c877d3f3380bcf2db0e20fc0244".to_string()),
        ]
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
        // java: return items.stream().filter(w -> w.getMessage().getString().toLowerCase().contains(input.toLowerCase())).toList();
        return self.items.iter().cloned().filter(|w| (compat::read(w, |__t| compat::component_of(__t.get_message_component()))).get_string().to_lowercase().contains(&input.to_lowercase())).collect::<Vec<_>>();
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
        <dyn AbstractSelectionPopup<compat::Shared<crate::de::hysky::skyblocker::utils::render::gui::item_selection_popup::ItemWidget>>>::extract_background_from::<Self>
    }

    /// java: @Override public void init() (dispatch: the body for this class)
    fn init(&self) -> fn(&compat::Shared<dyn AbstractPopupScreen>) {
        <dyn AbstractSelectionPopup<compat::Shared<crate::de::hysky::skyblocker::utils::render::gui::item_selection_popup::ItemWidget>>>::init_from::<Self>
    }

    /// java: @Override protected void repositionElements() (dispatch: the body for this class)
    fn reposition_elements(&self) -> fn(&compat::Shared<dyn AbstractPopupScreen>) {
        <dyn AbstractSelectionPopup<compat::Shared<crate::de::hysky::skyblocker::utils::render::gui::item_selection_popup::ItemWidget>>>::reposition_elements_from::<Self>
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
    /// java: AbstractPopupScreen.extractBackground overrides `Screen.extractBackground` without `super`
    fn extract_background_calls_super(&self) -> bool {
        false
    }

    /// java: @Override public void extractBackground(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_background(&mut self, _screen: &mcrust::net::minecraft::client::gui::screens::screen::Screen, context: &mut mcrust::mcrust::modding::screens::ScreenRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        self.call(|__h| ({ let __f = compat::dispatch(__h, |__t| __t.extract_background()); __f(&compat::view(__h, |a| a as compat::Shared<dyn AbstractPopupScreen>), &mut context.graphics, mouse_x, mouse_y, a) }))
    }

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
    /// java: ItemSelectionPopup.this
    pub outer: compat::WeakShared<ItemSelectionPopup>,
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
    pub fn new(outer: compat::WeakShared<ItemSelectionPopup>, stack: compat::Shared<FlexibleItemStack>) -> compat::Shared<Self> {
        // java: item = new ItemStack(stack.typeHolder()); - `ItemStack(Holder<Item>)` is `this(item.value(), 1)`
        let __init_item = mcrust::net::minecraft::world::item::item_stack::ItemStack::new(&crate::de::hysky::skyblocker::utils::flexible_item_stack::item_of(&FlexibleItemStack::type_holder(&stack)));
        let __this = compat::shared(Self {
            base: AbstractWidget::new(0, 0, 20, 20, (&component::literal(&compat::component_of(FlexibleItemStack::get_stack_or_throw(&stack).get_hover_name()).get_string()).get_string()).to_string(), WidgetKind::PlainText, OnPress::Inert),
            self_ref: std::sync::Weak::new(),
            item: __init_item,
            outer,
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        { let __q0 = mcrust::net::minecraft::core::component::data_components::profile().clone(); compat::read(&stack, |__s| compat::write(&__this, |__t| __t.item.copy_from(__q0, __s))) };
        { let __q0 = mcrust::net::minecraft::core::component::data_components::item_model().clone(); compat::read(&stack, |__s| compat::write(&__this, |__t| __t.item.copy_from(__q0, __s))) };
        { let __q0 = mcrust::net::minecraft::core::component::data_components::banner_patterns().clone(); compat::read(&stack, |__s| compat::write(&__this, |__t| __t.item.copy_from(__q0, __s))) };
        let mut item_id: String = stack.lock().borrow().get_skyblock_id();
        let mut custom_data: Tag = Tag::compound();
        custom_data.put_string(item_utils::ID, &item_id);
        { let (__q0, __q1,) = (mcrust::net::minecraft::core::component::data_components::custom_data().clone(), Some(mcrust::com::mojang::serialization::obj(CustomData::of(&custom_data))),); __this.lock().borrow_mut().item.set(__q0, __q1) };
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
        // java: if (selectedItem == this)
        let __outer = compat::read(__this, |__t| __t.outer.upgrade());
        let __selected = __outer.is_some_and(|__o| compat::read(&__o, |__t| __t.base.selected_item.as_ref().is_some_and(|__s| std::sync::Arc::ptr_eq(__s, __this))));
        if __selected {
            graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), __this.lock().borrow().get_x(), __this.lock().borrow().get_y(), (__this.lock().borrow().get_x()).wrapping_add(__this.lock().borrow().get_width()), (__this.lock().borrow().get_y()).wrapping_add(__this.lock().borrow().get_height()), 0x3000FF00);
        }
        if __this.lock().borrow().is_hovered() {
            graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), __this.lock().borrow().get_x(), __this.lock().borrow().get_y(), (__this.lock().borrow().get_x()).wrapping_add(__this.lock().borrow().get_width()), (__this.lock().borrow().get_y()).wrapping_add(__this.lock().borrow().get_height()), 0x20FFFFFF);
        }
    }

    /// java: @Override public void onClick(MouseButtonEvent click, boolean doubled)
    pub fn on_click(__this: &compat::Shared<Self>, click: &MouseButtonEvent, mut doubled: bool) {
        // java: super.onClick(click, doubled); - `AbstractWidget.onClick` is empty
        let _ = (click, doubled);
        // java: setSelectedItem(this);
        let Some(outer) = compat::read(__this, |__t| __t.outer.upgrade()) else { return; };
        compat::write(&outer, |__t| __t.set_selected_item(__this.clone()));
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

/// `ItemWidget` as the game's widget (the item list's grid holds it)
impl crate::de::hysky::skyblocker::utils::render::gui::abstract_selection_popup::SelectionWidget for compat::Shared<ItemWidget> {
    fn game_widget(&self) -> AbstractWidget {
        compat::WidgetAdapter::widget(self.clone())
    }
}
