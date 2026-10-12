//! Ported from de.hysky.skyblocker.skyblock.item.custom.screen.CustomizeScreen by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::configs::general_config::GeneralConfig;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::item::custom::custom_armor_animated_dyes::AnimatedDye;
use crate::de::hysky::skyblocker::skyblock::item::custom::custom_armor_animated_dyes::CustomArmorAnimatedDyes;
use crate::de::hysky::skyblocker::skyblock::item::custom::custom_armor_trims::ArmorTrimId;
use crate::de::hysky::skyblocker::skyblock::item::custom::screen::armor_tab::ArmorTab;
use crate::de::hysky::skyblocker::skyblock::item::custom::screen::item_tab::ItemTab;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::mcrust::modding::screens::ModScreen;
use mcrust::mcrust::modding::screens::ScreenRenderContext;
use mcrust::mcrust::modding::widgets::ModWidget;
use mcrust::mcrust::modding::widgets::WidgetRenderContext;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::components::abstract_container_widget::AbstractContainerWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::abstract_widget::PressLambda;
use mcrust::net::minecraft::client::gui::components::abstract_widget::WidgetKind;
use mcrust::net::minecraft::client::gui::components::button;
use mcrust::net::minecraft::client::gui::components::tabs::menu_tab_bar::MenuTabBar;
use mcrust::net::minecraft::client::gui::components::tabs::tab_manager::TabManager;
use mcrust::net::minecraft::client::gui::components::tabs::tab_navigation_bar;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::layouts::layout_element::LayoutElement;
use mcrust::net::minecraft::client::gui::layouts::linear_layout::LinearLayout;
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use mcrust::net::minecraft::client::gui::navigation::screen_rectangle::ScreenRectangle;
use mcrust::net::minecraft::client::gui::screens::inventory::inventory_screen::InventoryScreen;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::client::input::mouse_button_info::MouseButtonInfo;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;
use crate::de::hysky::skyblocker::injected::recipe_book_holder::RecipeBookHolder as _;
use crate::de::hysky::skyblocker::injected::skyblocker_stack::SkyblockerStack as _;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final Identifier TEXTURE = SkyblockerMod.id("armor_customization_screen/button");
pub static CUSTOMIZE_BUTTON_TEXTURE: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("armor_customization_screen/button"));

/// java: public class CustomizeScreen extends Screen
pub struct CustomizeScreen {
    /// java: extends net.minecraft.client.gui.screens.Screen
    pub base: mcrust::net::minecraft::client::gui::screens::screen::Screen,
    /// java: private final @Nullable Screen previousScreen;
    pub previous_screen: Option<mcrust::net::minecraft::client::gui::screens::screen::Screen>,
    /// java: private final Map<String, PreviousConfig> previousConfigs = new Object2ObjectOpenHashMap<>();
    pub previous_configs: std::collections::HashMap<String, PreviousConfig>,
    /// java: private final TabManager tabManager = new TabManager(this::addRenderableWidget, this::removeWidget);
    pub tab_manager: Option<TabManager>,
    /// java: private final boolean item;
    pub item: bool,
    /// java: private TabNavigationBar tabNavigation;
    pub tab_navigation: Option<AbstractContainerWidget>,
    /// java: private final LinearLayout footerLayout = LinearLayout.horizontal().spacing(5);
    pub footer_layout: LinearLayout,
    /// java: private ArmorTab armorTab;
    pub armor_tab: Option<ArmorTab>,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
    /// the screen's widgets as this object last saw them (see `compat::WidgetCache`)
    pub __widgets: compat::WidgetCache,
}

impl std::ops::Deref for CustomizeScreen {
    type Target = mcrust::net::minecraft::client::gui::screens::screen::Screen;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for CustomizeScreen {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl CustomizeScreen {
    /// java: public CustomizeScreen(@Nullable Screen previousScreen, boolean item)
    pub fn new(mut previous_screen: Option<mcrust::net::minecraft::client::gui::screens::screen::Screen>, mut item: bool) -> compat::Shared<Self> {
        let __init_previous_screen = previous_screen.clone();
        let __init_item = item;
        let __this = compat::shared(Self {
            base: mcrust::net::minecraft::client::gui::screens::screen::Screen::new((if compat::random() < 0.01 { component::translatable("skyblocker.customization.titleSecret") } else { component::translatable("skyblocker.customization.title") }).with_style_format(ChatFormatting::Gray).with_style_fn(|style| style.with_shadow_color(0)).get_string()),
            self_ref: std::sync::Weak::new(),
            __widgets: Default::default(),
            previous_screen: __init_previous_screen,
            previous_configs: std::collections::HashMap::new(),
            tab_manager: None,
            item: __init_item,
            tab_navigation: None,
            footer_layout: LinearLayout::horizontal().spacing(5),
            armor_tab: None,
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        // TODO(fabric2seer): the port's `new` ports this Java method with other parameters ( for Java's add_widget, remove_widget)
        // TODO(fabric2seer): `new` takes 0 argument(s) in the port (``), Java passes 2
        // TODO(fabric2seer): a lambda for a parameter the port does not type as a closure (`Consumer<net.minecraft.client.gui.components.AbstractWidget>`)
        // TODO(fabric2seer): `Screen.removeWidget` has no Rust method in the port (looked for `remove_widget` in Screen and its supertypes)
        // rust: let __this_w1 = compat::RenderThread::new(std::sync::Arc::downgrade(&__this));
        // rust: let __this_w2 = compat::RenderThread::new(std::sync::Arc::downgrade(&__this));
        // rust: { let __v = Some(TabManager::new(|__a0| { let Some(__this_1) = __this_w1.get().upgrade() else { return Default::default(); }; { { let (__q0,) = (__a0.clone(),); __this_1.lock().borrow_mut().add_renderable_widget(__q0) }; } }, |__a0| { let Some(__this_2) = __this_w2.get().upgrade() else { return Default::default(); }; { __this_2.lock().borrow().remove_widget(&__a0); } })); compat::write(&__this, |__t| __t.tab_manager = __v); }
        __this
    }

    /// java: @Init public static void initThings()
    pub fn init_things() {
        client_command_registration_callback::EVENT.register(Arc::new(
            |dispatcher: &mut client_command_registration_callback::ClientCommandDispatcher| {
                // TODO(fabric2seer): a command tree this tool cannot read (arguments, redirects, requires) - SeerClient commands receive the words after the name in `ctx.args`
                //   java: dispatcher.register(..);
            },
        ));
        screen_events::AFTER_INIT.register(Arc::new(|screen: &mut Screen, width: i32, height: i32| {
            if Utils::is_on_skyblock() && SkyblockerConfigManager::get().ui_and_visuals.show_customize_button && let Some(inventory_screen) = screen.as_any().downcast_ref::<InventoryScreen>() && !(crate::entry::catharsis_compatibility::is_gui_element_hidden("skyblocker:customizeButton")) {
                let mut button: compat::Shared<CustomizeButton> = CustomizeButton::new(inventory_screen.left_pos.wrapping_add(63), inventory_screen.top_pos.wrapping_add(10));
                // TODO(fabric2seer): Fabric API `Screens.getWidgets` has no SeerClient equivalent
                //   java: Screens.getWidgets(inventoryScreen).add(button);
                inventory_screen.register_recipe_book_toggle_callback(Arc::new(move || { let __wa1 = inventory_screen.left_pos.wrapping_add(63); let __wa2 = inventory_screen.top_pos.wrapping_add(10); let __o1 = button.clone(); compat::write(&__o1, |__t| __t.set_position(__wa1, __wa2)); }));
            }
        }));
    }

    /// java: public void backupConfigs(ItemStack stack)
    pub fn backup_configs(__this: &compat::Shared<Self>, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack) {
        let mut uuid: String = stack.get_uuid();
        if uuid.is_empty() || __this.lock().borrow().previous_configs.contains_key(&uuid) {
            return;
        }
        let general = &SkyblockerConfigManager::get().general;
        let mut previous_config: PreviousConfig = PreviousConfig::new((if general.custom_armor_trims.contains_key(&uuid) { general.custom_armor_trims.get(&uuid).cloned() } else { None }), (if general.custom_dye_colors.contains_key(&uuid) { Some(general.custom_dye_colors.get(&uuid).copied().unwrap_or_default()) } else { None }), (if general.custom_animated_dyes.contains_key(&uuid) { general.custom_animated_dyes.get(&uuid).cloned() } else { None }), (if general.custom_helmet_textures.contains_key(&uuid) { general.custom_helmet_textures.get(&uuid).cloned() } else { None }), (if general.custom_item_names.contains_key(&uuid) { general.custom_item_names.get(&uuid).cloned() } else { None }), (if general.custom_glint.contains_key(&uuid) { Some(general.custom_glint.get(&uuid).copied().unwrap_or_default()) } else { None }), (if general.custom_item_model.contains_key(&uuid) { general.custom_item_model.get(&uuid).cloned() } else { None }), (if general.custom_armor_model.contains_key(&uuid) { general.custom_armor_model.get(&uuid).cloned() } else { None }));
        let __h0 = uuid.clone();
        let __h1 = previous_config.clone();
        __this.lock().borrow_mut().previous_configs.insert(__h0, __h1);
    }

    /// java: @Override public void tick()
    pub fn tick(__this: &compat::Shared<Self>) {
        __this.lock().borrow().armor_tab.as_ref().unwrap().tick();
    }

    /// java: @Override protected void init()
    pub fn init(__this: &compat::Shared<Self>) {
        ();
        { let __v = Some(ArmorTab::new(__this.clone())); compat::write(__this, |__t| __t.armor_tab = __v); }
        // TODO(fabric2seer): static `MenuTabBar.builder` has no Rust counterpart in the port
        // TODO(fabric2seer): `Builder.addTabs` has no Rust method in the port (looked for `add_tabs` in MenuTabBar and its supertypes)
        // TODO(fabric2seer): `build` takes 3 argument(s) in the port (`screen: &mut crate::net::minecraft::client::gui::screens::screen::Screen, width: i32, titles: &[crate::net::minecraft::network::chat::formatted_text::FormattedText]`), Java passes 0
        // rust: { let __v = Some(MenuTabBar::build()); compat::write(__this, |__t| __t.tab_navigation = __v); }
        let mut i: i32 = __this.lock().borrow().tab_navigation.as_ref().unwrap().get_rectangle().bottom();
        // TODO(fabric2seer): `arrange_elements` takes 0 argument(s) in the port (``), Java passes 1
        // TODO(fabric2seer): `LayoutElement::arrange_elements` ports `arrangeElements`, but the port's `AbstractContainerWidget` holds no `LayoutElement` (no Deref, no `base` field) to call it on
        // rust: let __r0 = compat::read(__this, |__t| __t.width);
        // rust: { let (__q0,) = (__r0,); __this.lock().borrow_mut().tab_navigation.as_mut().unwrap().arrange_elements(__q0) };
        let __r0 = compat::read(__this, |__t| __t.width);
        let __r1 = compat::read(__this, |__t| __t.height);
        { let (__q0,) = (ScreenRectangle::new_at(0, i, __r0, ((__r1).wrapping_sub(i)).wrapping_sub(30)),); __this.lock().borrow_mut().tab_manager.as_mut().unwrap().set_tab_area(__q0) };
        // TODO(fabric2seer): `select_tab` takes 3 argument(s) in the port (`index: usize, is_focused: bool, active: bool`), Java passes 2
        // rust: tab_navigation_bar::select_tab(((if compat::read(__this, |__t| __t.item) { 1 } else { 0 })) as usize, false);
        let __r0 = compat::read(__this, |__t| __t.tab_navigation.clone());
        { let (__q0,) = (__r0.unwrap(),); __this.lock().borrow_mut().add_renderable_widget(__q0) };
        let __this_w1 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        { let (__q0,) = ({ let (__q0,) = ({ let __w = (button::builder(component::translatable("gui.cancel").get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { let Some(__this_1) = __this_w1.get().upgrade() else { return Default::default(); }; compat::lend_screen(&__this_1, __screen, || { let mut __pidx = compat::WidgetRef::Screen(__index); { CustomizeScreen::cancel((&__this_1)); } }) })))).build(); let (__ww, __wh) = (__w.get_width(), __w.get_height()); LayoutElement::widget(compat::write(__this, |__t| __t.add_widget(__w)), __ww, __wh) },); __this.lock().borrow_mut().footer_layout.add_child(__q0) },); __this.lock().borrow_mut().add_renderable_widget(__q0) };
        let __this_w2 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        { let (__q0,) = ({ let (__q0,) = ({ let __w = (button::builder(component::translatable("gui.done").get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { let Some(__this_2) = __this_w2.get().upgrade() else { return Default::default(); }; compat::lend_screen(&__this_2, __screen, || { let mut __pidx = compat::WidgetRef::Screen(__index); { CustomizeScreen::on_close((&__this_2)); } }) })))).build(); let (__ww, __wh) = (__w.get_width(), __w.get_height()); LayoutElement::widget(compat::write(__this, |__t| __t.add_widget(__w)), __ww, __wh) },); __this.lock().borrow_mut().footer_layout.add_child(__q0) },); __this.lock().borrow_mut().add_renderable_widget(__q0) };
        { __this.lock().borrow_mut().footer_layout.arrange_elements(); let mut __vw = Vec::new(); LayoutElement::Grid(__this.lock().borrow().footer_layout.wrapped.clone()).visit_widgets(&mut __vw); for (__i, __x, __y) in __vw { compat::write(__this, |__t| __t.widget_mut(__i).set_position(__x, __y)); } };
        CustomizeScreen::reposition_elements(__this);
    }

    /// java: @Override public void added()
    pub fn added(__this: &compat::Shared<Self>) {
        ();
        if compat::read(__this, |__t| __t.armor_tab.clone()).is_some() {
            __this.lock().borrow().armor_tab.as_ref().unwrap().recreate();
        }
    }

    /// java: private void cancel()
    pub fn cancel(__this: &compat::Shared<Self>) {
        let __this_1 = compat::RenderThread::new(__this.clone());
        SkyblockerConfigManager::update_only(Arc::new(move |config| {
            for (uuid, previous_config) in __this_1.get().lock().borrow().previous_configs.iter() {
                if let Some(trim) = previous_config.armor_trim_id() {
                    config.general.custom_armor_trims.insert(uuid.to_string(), trim.clone());
                } else {
                    config.general.custom_armor_trims.remove(uuid);
                }
                if let Some(i) = previous_config.color() {
                    config.general.custom_dye_colors.insert(uuid.to_string(), i);
                } else {
                    config.general.custom_dye_colors.remove(uuid);
                }
                if let Some(animated_dye) = previous_config.animated_dye() {
                    config.general.custom_animated_dyes.insert(uuid.to_string(), animated_dye.clone());
                } else {
                    config.general.custom_animated_dyes.remove(uuid);
                }
                if let Some(tex) = previous_config.helmet_texture() {
                    config.general.custom_helmet_textures.insert(uuid.to_string(), tex.clone());
                } else {
                    config.general.custom_helmet_textures.remove(uuid);
                }
                if let Some(text) = previous_config.item_name() {
                    config.general.custom_item_names.insert(uuid.to_string(), text.clone());
                } else {
                    config.general.custom_item_names.remove(uuid);
                }
                if let Some(b) = previous_config.glint() {
                    config.general.custom_glint.insert(uuid.to_string(), b);
                } else {
                    // TODO(fabric2seer): Map.removeBoolean(..)
                    //   java: config.general.customGlint.removeBoolean(..);
                }
                if let Some(identifier) = previous_config.item_model() {
                    config.general.custom_item_model.insert(uuid.to_string(), identifier.clone());
                } else {
                    config.general.custom_item_model.remove(uuid);
                }
                if let Some(identifier) = previous_config.armor_model() {
                    config.general.custom_armor_model.insert(uuid.to_string(), identifier.clone());
                } else {
                    config.general.custom_armor_model.remove(uuid);
                }
            }
        }));
        CustomizeScreen::on_close(__this);
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    pub fn mouse_clicked(__this: &compat::Shared<Self>, click: &MouseButtonEvent, mut doubled: bool) -> bool {
        let mut b: bool = false;
        if !b {
            // TODO(fabric2seer): `Screen.setFocused` has no Rust method in the port (looked for `set_focused` in Screen and its supertypes)
            // rust: __this.lock().borrow().set_focused(None);
        }
        return b;
    }

    /// java: @Override protected void repositionElements()
    pub fn reposition_elements(__this: &compat::Shared<Self>) {
        let mut i: i32 = __this.lock().borrow().tab_navigation.as_ref().unwrap().get_rectangle().bottom();
        // TODO(fabric2seer): `arrange_elements` takes 0 argument(s) in the port (``), Java passes 1
        // TODO(fabric2seer): `LayoutElement::arrange_elements` ports `arrangeElements`, but the port's `AbstractContainerWidget` holds no `LayoutElement` (no Deref, no `base` field) to call it on
        // rust: let __r0 = compat::read(__this, |__t| __t.width);
        // rust: { let (__q0,) = (__r0,); __this.lock().borrow_mut().tab_navigation.as_mut().unwrap().arrange_elements(__q0) };
        // TODO(fabric2seer): `LayoutElement::set_position` ports `setPosition`, but the port's `LinearLayout` holds no `LayoutElement` (no Deref, no `base` field) to call it on
        // rust: let __r0 = compat::read(__this, |__t| __t.width);
        // rust: let __r1 = compat::read(__this, |__t| __t.height);
        // rust: { let (__q0, __q1,) = (((__r0).wrapping_sub(__this.lock().borrow().footer_layout.get_width())).wrapping_div(2), ((__r1).wrapping_sub(__this.lock().borrow().footer_layout.get_height())).wrapping_sub(5),); __this.lock().borrow_mut().footer_layout.set_position(__q0, __q1) };
        let __r0 = compat::read(__this, |__t| __t.width);
        { let (__q0,) = (ScreenRectangle::new_at(0, i, __r0, ((__this.lock().borrow().footer_layout.get_y()).wrapping_sub(i)).wrapping_sub(2)),); __this.lock().borrow_mut().tab_manager.as_mut().unwrap().set_tab_area(__q0) };
    }

    /// java: @Override public void extractRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_render_state(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        ();
    }

    /// java: @Override public boolean isPauseScreen()
    pub fn is_pause_screen(__this: &compat::Shared<Self>) -> bool {
        return false;
    }

    /// java: @Override public void removed()
    pub fn removed(__this: &compat::Shared<Self>) {
        ();
        __this.lock().borrow_mut().armor_tab.as_mut().unwrap().close();
        CustomArmorAnimatedDyes::clean_trackers();
    }

    /// java: @Override public void onClose()
    pub fn on_close(__this: &compat::Shared<Self>) {
        mcrust::mcrust::modding::game::set_screen_to_parent();
        // TODO(fabric2seer): static call Consumers.nop(..)
        //   java: SkyblockerConfigManager.update(Consumers.nop());
    }

}

/// CustomizeScreen's `Screen` half (`super`'s fields and widget tree), lent by the game for each call
impl compat::ScreenHalf for CustomizeScreen {
    fn screen_half(&mut self) -> &mut mcrust::net::minecraft::client::gui::screens::screen::Screen {
        &mut self.base
    }
    fn screen_parts(&mut self) -> (&mut mcrust::net::minecraft::client::gui::screens::screen::Screen, Option<&mut compat::WidgetCache>) {
        let __s = self;
        (&mut __s.base, Some(&mut __s.__widgets))
    }
}

/// CustomizeScreen as the game's screen: `game::set_screen(Box::new(compat::ScreenAdapter::new(..)))`
impl ModScreen for compat::ScreenAdapter<CustomizeScreen> {
    fn get_title(&self) -> String {
        self.title()
    }

    /// java: @Override protected void init()
    fn init(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        self.lend(screen, |__h| CustomizeScreen::init(__h))
    }

    /// java: @Override protected void repositionElements()
    fn reposition_elements(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        self.lend(screen, |__h| CustomizeScreen::reposition_elements(__h))
    }

    /// java: @Override public void tick()
    fn tick(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        self.lend(screen, |__h| CustomizeScreen::tick(__h))
    }

    /// java: @Override public void removed()
    fn removed(&mut self) {
        self.call(|__h| CustomizeScreen::removed(__h))
    }

    /// java: @Override public void added()
    fn added(&mut self) {
        self.call(|__h| CustomizeScreen::added(__h))
    }

    /// java: @Override public void extractRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_render_state(&mut self, screen: &mcrust::net::minecraft::client::gui::screens::screen::Screen, context: &mut ScreenRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        self.lend_ref(screen, |__h| CustomizeScreen::extract_render_state(__h, &context.graphics, mouse_x, mouse_y, a))
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

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    fn mouse_clicked(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, x: f64, y: f64, button: i32) -> bool {
        self.lend(screen, |__h| CustomizeScreen::mouse_clicked(__h, &MouseButtonEvent::new(x, y, MouseButtonInfo::new(button, 0)), false))
    }

    /// java: @Override public void onClose()
    fn on_close(&mut self) {
        self.call(|__h| CustomizeScreen::on_close(__h))
    }

    /// java: @Override public boolean isPauseScreen()
    fn is_pause_screen(&self) -> bool {
        self.call(|__h| CustomizeScreen::is_pause_screen(__h))
    }

}

// NOTE(fabric2seer): added, extractRenderState, init, isPauseScreen, mouseClicked, onClose, removed, repositionElements, tick override(s) of Screen - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them

/// java: private record PreviousConfig(Optional<CustomArmorTrims.ArmorTrimId> armorTrimId, OptionalInt color, Optional<CustomArmorAnimatedDyes.AnimatedDye> animatedDye, Optional<String> helmetTexture, Optional<Component> itemName, Optional<Boolean> 
#[derive(Clone, Debug)]
pub struct PreviousConfig {
    /// java: Optional<CustomArmorTrims.ArmorTrimId> armorTrimId
    pub armor_trim_id: Option<ArmorTrimId>,
    /// java: OptionalInt color
    pub color: Option<i32>,
    /// java: Optional<CustomArmorAnimatedDyes.AnimatedDye> animatedDye
    pub animated_dye: Option<AnimatedDye>,
    /// java: Optional<String> helmetTexture
    pub helmet_texture: Option<String>,
    /// java: Optional<Component> itemName
    pub item_name: Option<MutableComponent>,
    /// java: Optional<Boolean> glint
    pub glint: Option<bool>,
    /// java: Optional<Identifier> itemModel
    pub item_model: Option<Identifier>,
    /// java: Optional<Identifier> armorModel
    pub armor_model: Option<Identifier>,
}

impl PreviousConfig {
    /// java: the canonical record constructor
    pub fn new(mut armor_trim_id: Option<ArmorTrimId>, mut color: Option<i32>, mut animated_dye: Option<AnimatedDye>, mut helmet_texture: Option<String>, mut item_name: Option<MutableComponent>, mut glint: Option<bool>, mut item_model: Option<Identifier>, mut armor_model: Option<Identifier>) -> Self {
        Self { armor_trim_id, color, animated_dye, helmet_texture, item_name, glint, item_model, armor_model }
    }

    /// java: the record accessor `armorTrimId()`
    pub fn armor_trim_id(&self) -> Option<ArmorTrimId> {
        self.armor_trim_id.clone()
    }

    /// java: the record accessor `color()`
    pub fn color(&self) -> Option<i32> {
        self.color.clone()
    }

    /// java: the record accessor `animatedDye()`
    pub fn animated_dye(&self) -> Option<AnimatedDye> {
        self.animated_dye.clone()
    }

    /// java: the record accessor `helmetTexture()`
    pub fn helmet_texture(&self) -> Option<String> {
        self.helmet_texture.clone()
    }

    /// java: the record accessor `itemName()`
    pub fn item_name(&self) -> Option<MutableComponent> {
        self.item_name.clone()
    }

    /// java: the record accessor `glint()`
    pub fn glint(&self) -> Option<bool> {
        self.glint.clone()
    }

    /// java: the record accessor `itemModel()`
    pub fn item_model(&self) -> Option<Identifier> {
        self.item_model.clone()
    }

    /// java: the record accessor `armorModel()`
    pub fn armor_model(&self) -> Option<Identifier> {
        self.armor_model.clone()
    }

}

/// java: private static class CustomizeButton extends AbstractWidget
#[derive(Clone)]
pub struct CustomizeButton {
    /// java: extends net.minecraft.client.gui.components.AbstractWidget
    pub base: AbstractWidget,
}

impl std::ops::Deref for CustomizeButton {
    type Target = AbstractWidget;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for CustomizeButton {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl CustomizeButton {
    /// java: private CustomizeButton(int x, int y)
    pub fn new(mut x: i32, mut y: i32) -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            base: AbstractWidget::new(x, y, 10, 10, (&component::empty().get_string()).to_string(), WidgetKind::PlainText, OnPress::Inert),
        });
        __this
    }

    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_widget_render_state(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        gui_graphics_extractor::blit_sprite_identifier_with_color(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), CUSTOMIZE_BUTTON_TEXTURE.get_mut(), __this.lock().borrow().get_x(), __this.lock().borrow().get_y(), __this.lock().borrow().get_width(), __this.lock().borrow().get_height(), (((if __this.lock().borrow().is_hovered() { (0xFFFAFA96u32 as i32) as f32 } else { (0x80FFFFFFu32 as i32) as f32 })) as f32) as f32);
        __this.lock().borrow().handle_cursor(graphics);
    }

    /// java: @Override public void onClick(MouseButtonEvent click, boolean doubled)
    pub fn on_click(__this: &compat::Shared<Self>, click: &MouseButtonEvent, mut doubled: bool) {
        mcrust::mcrust::modding::game::set_screen_keeping_parent(Box::new(compat::ScreenAdapter::new(CustomizeScreen::new(None, false))));
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    pub fn update_widget_narration(__this: &compat::Shared<Self>, builder: &dyn NarrationElementOutput) {
    }

}

/// CustomizeButton's widget half (`super`'s fields: x, y, size, message, ...), lent by the game for each call
impl compat::WidgetHalf for CustomizeButton {
    fn widget_half(&mut self) -> &mut AbstractWidget {
        &mut self.base
    }
}

/// CustomizeButton as the game's widget: `compat::WidgetAdapter::widget(handle)` where Java hands it on
impl ModWidget for compat::WidgetAdapter<CustomizeButton> {
    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_widget_render_state(&mut self, widget: &AbstractWidget, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        self.lend_ref(widget, |__h| CustomizeButton::extract_widget_render_state(__h, &context.graphics, mouse_x, mouse_y, a))
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    fn update_widget_narration(&mut self, widget: &AbstractWidget, output: &mut dyn NarrationElementOutput) {
        self.lend_ref(widget, |__h| CustomizeButton::update_widget_narration(__h, output))
    }

    /// java: @Override public void onClick(MouseButtonEvent click, boolean doubled)
    fn on_click(&mut self, widget: &mut AbstractWidget, event: &MouseButtonEvent, double_click: bool) {
        self.lend(widget, |__h| CustomizeButton::on_click(__h, event, double_click))
    }

}

// NOTE(fabric2seer): extractWidgetRenderState, onClick, updateWidgetNarration override(s) of AbstractWidget - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them
