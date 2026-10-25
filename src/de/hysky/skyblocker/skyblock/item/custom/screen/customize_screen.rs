//! Ported from de.hysky.skyblocker.skyblock.item.custom.screen.CustomizeScreen by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::item::custom::custom_armor_animated_dyes::AnimatedDye;
use crate::de::hysky::skyblocker::skyblock::item::custom::custom_armor_animated_dyes::CustomArmorAnimatedDyes;
use crate::de::hysky::skyblocker::skyblock::item::custom::custom_armor_trims::ArmorTrimId;
use crate::de::hysky::skyblocker::skyblock::item::custom::screen::armor_tab::ArmorTab;
use crate::de::hysky::skyblocker::skyblock::item::custom::screen::item_tab::ItemTab;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler::SchedulerBase;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::com::mojang::blaze3d::platform::cursor::cursor_type::CursorType;
use mcrust::com::mojang::blaze3d::platform::cursor::cursor_types::CursorTypes;
use mcrust::com::mojang::brigadier::builder::argument_builder::ArgumentBuilder as _;
use mcrust::mcrust::modding::events::client_command_registration_callback;
use mcrust::mcrust::modding::screens::ModScreen;
use mcrust::mcrust::modding::screens::ScreenRenderContext;
use mcrust::mcrust::modding::widgets::WidgetRenderContext;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::components::abstract_container_widget::AbstractContainerWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::abstract_widget::PressLambda;
use mcrust::net::minecraft::client::gui::components::button;
use mcrust::net::minecraft::client::gui::components::tabs::menu_tab_bar::MenuTabBar;
use mcrust::net::minecraft::client::gui::components::tabs::tab_manager::TabManager;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::layouts::layout_element::LayoutElement;
use mcrust::net::minecraft::client::gui::layouts::linear_layout::LinearLayout;
use mcrust::net::minecraft::client::gui::navigation::screen_rectangle::ScreenRectangle;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::formatted_text::FormattedText;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;
use crate::de::hysky::skyblocker::injected::skyblocker_stack::SkyblockerStack as _;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final Identifier TEXTURE = SkyblockerMod.id("armor_customization_screen/button");
pub static CUSTOMIZE_BUTTON_TEXTURE: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("armor_customization_screen/button"));

/// java: public class CustomizeScreen extends Screen
pub struct CustomizeScreen {
    /// java: extends net.minecraft.client.gui.screens.Screen
    pub base: mcrust::net::minecraft::client::gui::screens::screen::Screen,
    /// java: private final Map<String, PreviousConfig> previousConfigs = new Object2ObjectOpenHashMap<>();
    pub previous_configs: std::collections::HashMap<String, PreviousConfig>,
    /// java: private final TabManager tabManager = new TabManager(this::addRenderableWidget, this::removeWidget);
    pub tab_manager: TabManager,
    /// java: private final boolean item;
    pub item: bool,
    /// java: private TabNavigationBar tabNavigation;
    pub tab_navigation: Option<MenuTabBar>,
    /// java: private final LinearLayout footerLayout = LinearLayout.horizontal().spacing(5); (its buttons' places)
    pub footer_layout: LayoutElement,
    /// java: private ArmorTab armorTab;
    pub armor_tab: Option<compat::Shared<ArmorTab>>,
    /// the item tab (`.addTabs(armorTab, new ItemTab(this))`)
    pub item_tab: Option<compat::Shared<ItemTab>>,
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
    ///
    /// (`previousScreen` is the screen that is up when this one opens - the game holds it,
    /// `game::set_screen_keeping_parent`, and `onClose` returns to it)
    pub fn new(item: bool) -> compat::Shared<Self> {
        // java: super((Math.random() < 0.01 ? Component.translatable("..titleSecret") : Component.translatable("..title")).withStyle(ChatFormatting.GRAY).withStyle(style -> style.withShadowColor(0)));
        let title = (if compat::random() < 0.01 { component::translatable("skyblocker.customization.titleSecret") } else { component::translatable("skyblocker.customization.title") })
            .with_style_format(ChatFormatting::Gray)
            .with_style_fn(|style| style.with_shadow_color(0));
        let __this = compat::shared(Self {
            base: mcrust::net::minecraft::client::gui::screens::screen::Screen::new(title.get_string()),
            self_ref: std::sync::Weak::new(),
            __widgets: Default::default(),
            previous_configs: std::collections::HashMap::new(),
            tab_manager: TabManager::new(),
            item,
            tab_navigation: None,
            footer_layout: LayoutElement::Grid(LinearLayout::horizontal().spacing(5).wrapped),
            armor_tab: None,
            item_tab: None,
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        __this
    }

    /// java: @Init public static void initThings()
    pub fn init_things() {
        client_command_registration_callback::EVENT.register(Arc::new(|dispatcher: &mut client_command_registration_callback::ClientCommandDispatcher| {
            // java: dispatcher.register(ClientCommands.literal(SkyblockerMod.NAMESPACE).then(ClientCommands.literal("custom").executes(Scheduler.queueOpenScreenCommand(() -> new CustomizeScreen(null, false)))));
            let mut custom = client_command_registration_callback::literal("custom");
            custom.executes(SchedulerBase::queue_open_screen_command(Arc::new(|| Box::new(compat::ScreenAdapter::new(CustomizeScreen::new(false))) as Box<dyn ModScreen>)));
            let mut root = client_command_registration_callback::literal(crate::entry::skyblocker_mod::NAMESPACE);
            root.then(&custom);
            dispatcher.dispatcher().register(&root);
        }));
        screen_events::AFTER_INIT.register(Arc::new(|screen: &mut Screen, width: i32, height: i32| {
            // java: screen instanceof InventoryScreen inventoryScreen
            let is_inventory_screen = compat::is_inventory_screen_initialising();
            if Utils::is_on_skyblock() && SkyblockerConfigManager::get().ui_and_visuals.show_customize_button && is_inventory_screen && !crate::entry::catharsis_compatibility::is_gui_element_hidden("skyblocker:customizeButton") {
                // java: new CustomizeButton(accessor.getX() + 63, accessor.getY() + 10) - the panel as the screen's `init` placed
                //   it (`(width - imageWidth) / 2`, `(height - imageHeight) / 2`) when it has not been drawn yet
                let (left, top) = match compat::container_screen_panel() {
                    Some((left, top, _, _, _)) => (left, top),
                    None => ((width - 176) / 2, (height - 166) / 2),
                };
                let button = CustomizeButton::new(left + 63, top + 10);
                // java: Screens.getWidgets(inventoryScreen).add(button);
                screen.add_renderable_widget(compat::object_widget(button, ""));
                // java: inventoryScreen.registerRecipeBookToggleCallback(() -> button.setPosition(accessor.getX() + 63, accessor.getY() + 10));
                // Port note: the button follows the panel itself each frame (`CustomizeButton::extract_widget_render_state`),
                //   which is where the toggle callback moves it
            }
        }));
    }

    /// java: public void backupConfigs(ItemStack stack)
    pub fn backup_configs(__this: &compat::Shared<Self>, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack) {
        let uuid: String = stack.get_uuid();
        if uuid.is_empty() || compat::read(__this, |__t| __t.previous_configs.contains_key(&uuid)) {
            return;
        }
        let general = &SkyblockerConfigManager::get().general;
        let previous_config = PreviousConfig::new(
            general.custom_armor_trims.get(&uuid).cloned(),
            general.custom_dye_colors.get(&uuid).copied(),
            general.custom_animated_dyes.get(&uuid).cloned(),
            general.custom_helmet_textures.get(&uuid).cloned(),
            general.custom_item_names.get(&uuid).cloned(),
            general.custom_glint.get(&uuid).copied(),
            general.custom_item_model.get(&uuid).cloned(),
            general.custom_armor_model.get(&uuid).cloned(),
        );
        compat::write(__this, |__t| __t.previous_configs.insert(uuid, previous_config));
    }

    /// The screen's widget list for `f`, with no borrow of this object held (the tabs' constructors and layouts reach
    /// back to it through `backupConfigs`).
    fn with_screen<R>(__this: &compat::Shared<Self>, f: impl FnOnce(&mut mcrust::net::minecraft::client::gui::screens::screen::Screen) -> R) -> R {
        struct Back<'a>(&'a compat::Shared<CustomizeScreen>, Option<mcrust::net::minecraft::client::gui::screens::screen::Screen>);
        impl Drop for Back<'_> {
            fn drop(&mut self) {
                if let Some(screen) = self.1.take() {
                    compat::write(self.0, |__t| __t.base = screen);
                }
            }
        }
        let screen = compat::write(__this, |__t| std::mem::replace(&mut __t.base, mcrust::net::minecraft::client::gui::screens::screen::Screen::new(String::new())));
        let mut back = Back(__this, Some(screen));
        f(back.1.as_mut().expect("the screen"))
    }

    /// java: @Override public void tick()
    pub fn tick(__this: &compat::Shared<Self>) {
        // java: armorTab.tick();
        if let Some(armor_tab) = compat::read(__this, |__t| __t.armor_tab.clone()) {
            compat::read(&armor_tab, |__t| __t.tick());
        }
        // (the item tab's writes on the screen's widgets and its model field's focus; see `ItemTab::apply`)
        if let Some(item_tab) = compat::read(__this, |__t| __t.item_tab.clone()) {
            CustomizeScreen::with_screen(__this, |screen| {
                ItemTab::apply(&item_tab, screen);
                ItemTab::check_model_field_focus(&item_tab, screen);
            });
        }
    }

    /// java: @Override protected void init()
    pub fn init(__this: &compat::Shared<Self>) {
        // java: super.init(); (empty)
        let (width, height) = compat::read(__this, |__t| (__t.width, __t.height));
        let (armor_tab, item_tab) = CustomizeScreen::with_screen(__this, |screen| (ArmorTab::new(__this, screen), ItemTab::new(__this, screen)));
        compat::write(__this, |__t| {
            __t.armor_tab = Some(armor_tab.clone());
            __t.item_tab = Some(item_tab.clone());
            // the tabs' widgets were made for this screen: a new layout of tabs starts with none selected
            __t.tab_manager = TabManager::new();
        });
        // java: tabNavigation = MenuTabBar.builder(tabManager, width).addTabs(armorTab, new ItemTab(this)).build();
        let titles = [compat::read(&armor_tab, |__t| __t.base.get_tab_title().clone()), compat::read(&item_tab, |__t| __t.base.get_tab_title().clone())];
        let mut bar = CustomizeScreen::with_screen(__this, |screen| MenuTabBar::build(screen, width, &titles));
        // a tab button's press is `TabNavigationBar.selectTab(index, true)` on this screen's tabs
        for (index, &button) in bar.tab_buttons.iter().enumerate() {
            let this = compat::RenderThread::new(Arc::downgrade(__this));
            compat::write(__this, |__t| {
                __t.widget_mut(button).on_press = OnPress::Lambda(PressLambda::new(move |screen, _index| {
                    if let Some(this) = this.get().upgrade() {
                        CustomizeScreen::select_tab(&this, screen, index, true);
                    }
                }));
            });
        }
        let i: i32 = bar.get_rectangle().bottom();
        CustomizeScreen::with_screen(__this, |screen| bar.arrange_elements(screen, width));
        let tab_buttons = bar.tab_buttons.clone();
        compat::write(__this, |__t| __t.tab_navigation = Some(bar));
        CustomizeScreen::set_tab_area(__this, ScreenRectangle::new_at(0, i, width, height - i - 30));
        let item = compat::read(__this, |__t| __t.item);
        CustomizeScreen::with_screen(__this, |screen| CustomizeScreen::select_tab(__this, screen, if item { 1 } else { 0 }, false));
        // java: addRenderableWidget(tabNavigation);
        compat::write(__this, |__t| {
            for button in tab_buttons {
                __t.mark_renderable(button);
            }
        });

        // java: addRenderableWidget(footerLayout.addChild(Button.builder(Component.translatable("gui.cancel"), _ -> cancel()).build()));
        let this = compat::RenderThread::new(Arc::downgrade(__this));
        let cancel = button::builder(component::translatable("gui.cancel").get_string(), OnPress::Lambda(PressLambda::new(move |_screen, _index| {
            if let Some(this) = this.get().upgrade() {
                CustomizeScreen::cancel(&this);
            }
        })))
        .build();
        // java: addRenderableWidget(footerLayout.addChild(Button.builder(Component.translatable("gui.done"), _ -> onClose()).build()));
        let this = compat::RenderThread::new(Arc::downgrade(__this));
        let done = button::builder(component::translatable("gui.done").get_string(), OnPress::Lambda(PressLambda::new(move |_screen, _index| {
            if let Some(this) = this.get().upgrade() {
                CustomizeScreen::on_close(&this);
            }
        })))
        .build();
        compat::write(__this, |__t| {
            let mut footer = LinearLayout::horizontal().spacing(5);
            for button in [cancel, done] {
                let (w, h) = (button.get_width(), button.get_height());
                let index = __t.add_renderable_widget(button);
                footer.add_child(LayoutElement::widget(index, w, h));
            }
            // java: footerLayout.arrangeElements();
            let mut footer = LayoutElement::Grid(footer.wrapped);
            footer.arrange_elements();
            __t.footer_layout = footer;
        });
        CustomizeScreen::reposition_elements(__this);
    }

    /// `tabManager.setTabArea(area)` - `TabManager.setTabArea:34-40`: the current tab is laid out in it.
    fn set_tab_area(__this: &compat::Shared<Self>, area: ScreenRectangle) {
        let current = compat::write(__this, |__t| __t.tab_manager.set_tab_area(area));
        if let Some(tab) = current {
            CustomizeScreen::with_screen(__this, |screen| CustomizeScreen::layout_tab(__this, screen, tab, area));
        }
    }

    /// `tab.doLayout(area)` for tab `index`
    fn layout_tab(__this: &compat::Shared<Self>, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, index: usize, area: ScreenRectangle) {
        let (armor_tab, item_tab) = compat::read(__this, |__t| (__t.armor_tab.clone(), __t.item_tab.clone()));
        match index {
            0 => {
                if let Some(tab) = armor_tab {
                    compat::write(&tab, |__t| __t.do_layout(screen, area));
                }
            }
            _ => {
                if let Some(tab) = item_tab {
                    compat::write(&tab, |__t| __t.do_layout(screen, area));
                }
            }
        }
    }

    /// `tab.visitChildren(..)` for tab `index`, with each child's `visible` as the tab last set it
    fn tab_children(__this: &compat::Shared<Self>, index: usize) -> Vec<(usize, bool)> {
        let (armor_tab, item_tab) = compat::read(__this, |__t| (__t.armor_tab.clone(), __t.item_tab.clone()));
        match index {
            0 => armor_tab.map(|tab| compat::read(&tab, |__t| __t.visit_children().into_iter().map(|i| (i, true)).collect())).unwrap_or_default(),
            _ => item_tab.map(|tab| compat::read(&tab, |__t| __t.visit_children().into_iter().map(|i| (i, __t.child_visible(i))).collect())).unwrap_or_default(),
        }
    }

    /// `TabNavigationBar.selectTab(index, playSound)` - `TabManager.setCurrentTab:46-68` over this screen's tabs: the
    /// old tab's widgets leave the screen, the new tab's join it and are laid out.
    pub fn select_tab(__this: &compat::Shared<Self>, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, index: usize, play_sound: bool) {
        let Some(switch) = compat::write(__this, |__t| __t.tab_manager.set_current_tab(index, play_sound)) else {
            return;
        };
        // :48-50 - `removeWidget` for the old tab's children
        if let Some(old) = switch.remove_children_of {
            for (child, _) in CustomizeScreen::tab_children(__this, old) {
                screen.remove_widget(child);
            }
        }
        // :54-56 - `addRenderableWidget` for the new tab's children
        if let Some(new) = switch.add_children_of {
            for (child, visible) in CustomizeScreen::tab_children(__this, new) {
                screen.widget_mut(child).visible = visible;
                screen.mark_renderable(child);
            }
        }
        if let Some(bar) = compat::read(__this, |__t| __t.tab_navigation.clone()) {
            bar.update_selected(screen, compat::read(__this, |__t| __t.tab_manager.get_current_tab()));
        }
        // :58-60
        if switch.layout
            && let Some(area) = compat::read(__this, |__t| __t.tab_manager.tab_area())
        {
            CustomizeScreen::layout_tab(__this, screen, index, area);
        }
        // :62-64 - `Minecraft.getInstance().getSoundManager().play(SimpleSoundInstance.forUI(SoundEvents.UI_BUTTON_CLICK, 1.0F))`
        mcrust::mcrust::modding::game::play_ui_sound(mcrust::net::minecraft::sounds::sound_events::ui_button_click(), 1.0f32);
    }

    /// java: @Override public void added()
    pub fn added(__this: &compat::Shared<Self>) {
        // java: super.added(); (empty)
        if let Some(armor_tab) = compat::read(__this, |__t| __t.armor_tab.clone()) {
            compat::read(&armor_tab, |__t| __t.recreate());
        }
    }

    /// java: private void cancel()
    pub fn cancel(__this: &compat::Shared<Self>) {
        let previous_configs: Vec<(String, PreviousConfig)> = compat::read(__this, |__t| __t.previous_configs.iter().map(|(k, v)| (k.clone(), v.clone())).collect());
        SkyblockerConfigManager::update_only(Arc::new(move |config: &mut crate::de::hysky::skyblocker::config::skyblocker_config::SkyblockerConfig| {
            for (uuid, previous_config) in previous_configs.iter() {
                match previous_config.armor_trim_id() {
                    Some(trim) => config.general.custom_armor_trims.insert(uuid.clone(), trim),
                    None => config.general.custom_armor_trims.remove(uuid),
                };
                match previous_config.color() {
                    Some(i) => config.general.custom_dye_colors.insert(uuid.clone(), i),
                    None => config.general.custom_dye_colors.remove(uuid),
                };
                match previous_config.animated_dye() {
                    Some(animated_dye) => config.general.custom_animated_dyes.insert(uuid.clone(), animated_dye),
                    None => config.general.custom_animated_dyes.remove(uuid),
                };
                match previous_config.helmet_texture() {
                    Some(tex) => config.general.custom_helmet_textures.insert(uuid.clone(), tex),
                    None => config.general.custom_helmet_textures.remove(uuid),
                };
                match previous_config.item_name() {
                    Some(text) => config.general.custom_item_names.insert(uuid.clone(), text),
                    None => config.general.custom_item_names.remove(uuid),
                };
                match previous_config.glint() {
                    Some(b) => config.general.custom_glint.insert(uuid.clone(), b),
                    None => config.general.custom_glint.remove(uuid),
                };
                match previous_config.item_model() {
                    Some(identifier) => config.general.custom_item_model.insert(uuid.clone(), identifier),
                    None => config.general.custom_item_model.remove(uuid),
                };
                match previous_config.armor_model() {
                    Some(identifier) => config.general.custom_armor_model.insert(uuid.clone(), identifier),
                    None => config.general.custom_armor_model.remove(uuid),
                };
            }
        }));
        CustomizeScreen::on_close(__this);
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    ///
    /// `boolean b = super.mouseClicked(click, doubled); if (!b) setFocused(null); return b;` - the port's generic
    /// pass is `super`, and runs after this answers `false`: a click no child is under clears the focus first.
    pub fn mouse_clicked(__this: &compat::Shared<Self>, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, click: &MouseButtonEvent) -> bool {
        let b = screen.get_child_at(click.x(), click.y()).is_some();
        if !b {
            screen.set_focused(None);
            if let Some(item_tab) = compat::read(__this, |__t| __t.item_tab.clone()) {
                ItemTab::check_model_field_focus(&item_tab, screen);
            }
        }
        false
    }

    /// java: @Override protected void repositionElements()
    pub fn reposition_elements(__this: &compat::Shared<Self>) {
        let (width, height) = compat::read(__this, |__t| (__t.width, __t.height));
        let Some(mut bar) = compat::read(__this, |__t| __t.tab_navigation.clone()) else {
            return;
        };
        let i: i32 = bar.get_rectangle().bottom();
        CustomizeScreen::with_screen(__this, |screen| bar.arrange_elements(screen, width));
        let footer_y = compat::write(__this, |__t| {
            __t.tab_navigation = Some(bar);
            // java: footerLayout.setPosition((width - footerLayout.getWidth()) / 2, height - footerLayout.getHeight() - 5);
            let (fw, fh) = (__t.footer_layout.get_width(), __t.footer_layout.get_height());
            __t.footer_layout.set_position((width - fw) / 2, height - fh - 5);
            let mut placed = Vec::new();
            __t.footer_layout.visit_widgets(&mut placed);
            for (index, x, y) in placed {
                __t.widget_mut(index).set_position(x, y);
            }
            __t.footer_layout.get_y()
        });
        // java: tabManager.setTabArea(new ScreenRectangle(0, i, width, footerLayout.getY() - i - 2));
        CustomizeScreen::set_tab_area(__this, ScreenRectangle::new_at(0, i, width, footer_y - i - 2));
        // (the item tab's writes made while a popup was up - this runs as the screen comes back)
        if let Some(item_tab) = compat::read(__this, |__t| __t.item_tab.clone()) {
            CustomizeScreen::with_screen(__this, |screen| ItemTab::apply(&item_tab, screen));
        }
    }

    /// java: @Override public void extractRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_render_state(__this: &compat::Shared<Self>, context: &mut ScreenRenderContext<'_>) {
        // java: super.extractRenderState(graphics, mouseX, mouseY, a); - the tab bar draws its header separators
        if let Some(bar) = compat::read(__this, |__t| __t.tab_navigation.clone()) {
            bar.extract_header_separators(context.gl, context.minecraft);
        }
        //context.drawCenteredTextWithShadow(textRenderer, getTitle(), this.width / 2, footerLayout.getY() + footerLayout.getHeight() + 2, Colors.WHITE);
    }

    /// java: @Override public boolean isPauseScreen()
    pub fn is_pause_screen(_this: &compat::Shared<Self>) -> bool {
        return false;
    }

    /// java: @Override public void removed()
    pub fn removed(__this: &compat::Shared<Self>) {
        // java: super.removed(); armorTab.close();
        if let Some(armor_tab) = compat::read(__this, |__t| __t.armor_tab.clone()) {
            compat::write(&armor_tab, |__t| __t.close());
        }
        CustomArmorAnimatedDyes::clean_trackers();
    }

    /// java: @Override public void onClose()
    pub fn on_close(_this: &compat::Shared<Self>) {
        // java: minecraft.gui.setScreen(previousScreen);
        mcrust::mcrust::modding::game::set_screen_to_parent();
        // java: SkyblockerConfigManager.update(Consumers.nop());
        SkyblockerConfigManager::update(Arc::new(|_: &mut crate::de::hysky::skyblocker::config::skyblocker_config::SkyblockerConfig| {}));
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

/// CustomizeScreen as the game's screen: `game::set_screen_keeping_parent(Box::new(compat::ScreenAdapter::new(..)))`
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
    fn extract_render_state(&mut self, screen: &mcrust::net::minecraft::client::gui::screens::screen::Screen, context: &mut ScreenRenderContext<'_>, _mouse_x: i32, _mouse_y: i32, _a: f32) {
        self.lend_ref(screen, |__h| CustomizeScreen::extract_render_state(__h, context))
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    fn mouse_clicked(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, x: f64, y: f64, button: i32) -> bool {
        let event = MouseButtonEvent::new(x, y, mcrust::net::minecraft::client::input::mouse_button_info::MouseButtonInfo::new(button, 0));
        self.call(|__h| CustomizeScreen::mouse_clicked(__h, screen, &event))
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

/// java: private record PreviousConfig(Optional<CustomArmorTrims.ArmorTrimId> armorTrimId, OptionalInt color, Optional<CustomArmorAnimatedDyes.AnimatedDye> animatedDye, Optional<String> helmetTexture, Optional<Component> itemName, Optional<Boolean> glint, Optional<Identifier> itemModel, Optional<Identifier> armorModel)
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
    pub fn new(armor_trim_id: Option<ArmorTrimId>, color: Option<i32>, animated_dye: Option<AnimatedDye>, helmet_texture: Option<String>, item_name: Option<MutableComponent>, glint: Option<bool>, item_model: Option<Identifier>, armor_model: Option<Identifier>) -> Self {
        Self { armor_trim_id, color, animated_dye, helmet_texture, item_name, glint, item_model, armor_model }
    }

    /// java: the record accessor `armorTrimId()`
    pub fn armor_trim_id(&self) -> Option<ArmorTrimId> {
        self.armor_trim_id.clone()
    }

    /// java: the record accessor `color()`
    pub fn color(&self) -> Option<i32> {
        self.color
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
        self.glint
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
pub struct CustomizeButton {
    /// java: extends net.minecraft.client.gui.components.AbstractWidget
    pub base: AbstractContainerWidget,
}

impl CustomizeButton {
    /// java: private CustomizeButton(int x, int y)
    pub fn new(x: i32, y: i32) -> compat::Shared<Self> {
        // java: super(x, y, 10, 10, Component.empty());
        compat::shared(CustomizeButton { base: AbstractContainerWidget::new(x, y, 10, 10, FormattedText::Empty) })
    }
}

impl compat::ObjectWidget for CustomizeButton {
    fn object_box(&mut self) -> &mut AbstractContainerWidget {
        &mut self.base
    }

    fn object_box_ref(&self) -> &AbstractContainerWidget {
        &self.base
    }

    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_widget_render_state(&mut self, _widget: &AbstractWidget, context: &mut WidgetRenderContext<'_>, _mouse_x: i32, _mouse_y: i32, _a: f32) {
        // (`registerRecipeBookToggleCallback`'s `button.setPosition(accessor.getX() + 63, accessor.getY() + 10)`: the
        // panel moves when the recipe book opens or closes, and the button with it)
        if let Some((left, top, _, _, _)) = compat::container_screen_panel() {
            self.base.set_position(left + 63, top + 10);
        }
        let color = if self.base.is_hovered() { 0xFFFAFA96u32 as i32 } else { 0x80FFFFFFu32 as i32 };
        gui_graphics_extractor::blit_sprite_identifier_with_color(context.gl, context.minecraft, mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), CUSTOMIZE_BUTTON_TEXTURE.get(), self.base.get_x(), self.base.get_y(), self.base.get_width(), self.base.get_height(), color);
        // java: this.handleCursor(graphics); - asked by the game after the frame (`handle_cursor`)
    }

    /// java: @Override public void onClick(MouseButtonEvent click, boolean doubled)
    fn on_click(&mut self, _widget: &mut AbstractWidget, _event: &MouseButtonEvent, _double_click: bool) {
        // java: CLIENT.gui.setScreen(new CustomizeScreen(CLIENT.gui.screen(), false));
        mcrust::mcrust::modding::game::set_screen_keeping_parent(Box::new(compat::ScreenAdapter::new(CustomizeScreen::new(false))));
    }

    /// `this.handleCursor(graphics)` - `AbstractWidget.handleCursor:68-72`
    fn handle_cursor(&self, widget: &AbstractWidget, cursors: &CursorTypes) -> Option<CursorType> {
        widget.handle_cursor(cursors)
    }
}
