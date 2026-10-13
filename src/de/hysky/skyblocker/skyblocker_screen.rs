//! Ported from de.hysky.skyblocker.SkyblockerScreen by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::tips::Tips;
use crate::de::hysky::skyblocker::skyblocker_credits_screen::SkyblockerCreditsScreen;
use crate::de::hysky::skyblocker::utils::fun_utils::FunUtils;
use crate::de::hysky::skyblocker::utils::logs_folder_finder::LogsFolderFinder;
use crate::entry::skyblocker_mod;
use mcrust::mcrust::modding::screens::ModScreen;
use mcrust::mcrust::modding::screens::ScreenRenderContext;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::abstract_widget::PressLambda;
use mcrust::net::minecraft::client::gui::components::button;
use mcrust::net::minecraft::client::gui::components::multi_line_text_widget::MultiLineTextWidget;
use mcrust::net::minecraft::client::gui::components::string_widget;
use mcrust::net::minecraft::client::gui::font::Font;
use mcrust::net::minecraft::client::gui::gui;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::layouts::grid_layout::GridLayout;
use mcrust::net::minecraft::client::gui::layouts::grid_layout::RowHelper;
use mcrust::net::minecraft::client::gui::layouts::header_and_footer_layout::HeaderAndFooterLayout;
use mcrust::net::minecraft::client::gui::layouts::layout_element::LayoutElement;
use mcrust::net::minecraft::client::gui::screens::screen;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::client::input::mouse_button_info::MouseButtonInfo;
use mcrust::net::minecraft::locale::language;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::util::formatted_char_sequence::FormattedCharSequence;
use mcrust::net::minecraft::util::formatted_char_sequence::Seq;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final int SPACING = 8;
pub const SPACING: i32 = 8;
/// java: private static final int BUTTON_WIDTH = 210;
pub const BUTTON_WIDTH: i32 = 210;
/// java: private static final int HALF_BUTTON_WIDTH = (BUTTON_WIDTH - SPACING) / 2;
pub static HALF_BUTTON_WIDTH: LazyLock<i32> = LazyLock::new(|| (BUTTON_WIDTH.wrapping_sub(SPACING)).wrapping_div(2));
/// java: private static final Component TITLE;
pub static TITLE: compat::MainThread<Option<MutableComponent>> = compat::MainThread::new(|| None);
/// java: private static final Identifier ICON;
pub static ICON: compat::MainThread<Option<Identifier>> = compat::MainThread::new(|| None);
/// java: private static final Component CONFIGURATION_TEXT = Component.translatable("text.skyblocker.config");
pub static CONFIGURATION_TEXT: compat::MainThread<MutableComponent> = compat::MainThread::new(|| component::translatable("text.skyblocker.config"));
/// java: private static final Component SOURCE_TEXT = Component.translatable("text.skyblocker.source");
pub static SOURCE_TEXT: compat::MainThread<MutableComponent> = compat::MainThread::new(|| component::translatable("text.skyblocker.source"));
/// java: private static final Component REPORT_BUGS_TEXT = Component.translatable("menu.reportBugs");
pub static REPORT_BUGS_TEXT: compat::MainThread<MutableComponent> = compat::MainThread::new(|| component::translatable("menu.reportBugs"));
/// java: private static final Component WEBSITE_TEXT = Component.translatable("text.skyblocker.website");
pub static WEBSITE_TEXT: compat::MainThread<MutableComponent> = compat::MainThread::new(|| component::translatable("text.skyblocker.website"));
/// java: private static final Component TRANSLATE_TEXT = Component.translatable("text.skyblocker.translate");
pub static TRANSLATE_TEXT: compat::MainThread<MutableComponent> = compat::MainThread::new(|| component::translatable("text.skyblocker.translate"));
/// java: private static final Component MODRINTH_TEXT = Component.translatable("text.skyblocker.modrinth");
pub static MODRINTH_TEXT: compat::MainThread<MutableComponent> = compat::MainThread::new(|| component::translatable("text.skyblocker.modrinth"));
/// java: private static final Component CURSEFORGE_TEXT = Component.translatable("text.skyblocker.curseforge");
pub static CURSEFORGE_TEXT: compat::MainThread<MutableComponent> = compat::MainThread::new(|| component::translatable("text.skyblocker.curseforge"));
/// java: private static final Component DISCORD_TEXT = Component.translatable("text.skyblocker.discord");
pub static DISCORD_TEXT: compat::MainThread<MutableComponent> = compat::MainThread::new(|| component::translatable("text.skyblocker.discord"));
/// java: private static final Component SUPPORT_US_TEXT = Component.translatable("text.skyblocker.supportUs");
pub static SUPPORT_US_TEXT: compat::MainThread<MutableComponent> = compat::MainThread::new(|| component::translatable("text.skyblocker.supportUs"));
/// java: private static final Component CREDITS_TEXT = Component.translatable("credits_and_attribution.button.credits");
pub static CREDITS_TEXT: compat::MainThread<MutableComponent> = compat::MainThread::new(|| component::translatable("credits_and_attribution.button.credits"));
/// java: private static final Component LOGS_FOLDER_TEXT = Component.translatable("text.skyblocker.logsFolder");
pub static LOGS_FOLDER_TEXT: compat::MainThread<MutableComponent> = compat::MainThread::new(|| component::translatable("text.skyblocker.logsFolder"));
/// java: private static final int ICON_SIZE = 32;
pub const ICON_TEXT_WIDGET_ICON_SIZE: i32 = 32;
/// java: private static final int SPACING = 2;
pub const ICON_TEXT_WIDGET_SPACING: i32 = 2;

/// java: public class SkyblockerScreen extends Screen
pub struct SkyblockerScreen {
    /// java: extends net.minecraft.client.gui.screens.Screen
    pub base: mcrust::net::minecraft::client::gui::screens::screen::Screen,
    /// java: private HeaderAndFooterLayout layout = new HeaderAndFooterLayout(this, 64, 100);
    pub layout: Option<HeaderAndFooterLayout>,
    /// java: private MultiLineTextWidget tip = new MultiLineTextWidget(Component.empty(), this.font);
    /// (the widget lives in the screen's list: its index there, set by `init`)
    pub tip: Option<usize>,
    /// the widgets `init` put in `layout`, in the order it added them (`arrange` rebuilds the layout from them)
    pub layout_indices: Vec<usize>,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
    /// the screen's widgets as this object last saw them (see `compat::WidgetCache`)
    pub __widgets: compat::WidgetCache,
}

impl std::ops::Deref for SkyblockerScreen {
    type Target = mcrust::net::minecraft::client::gui::screens::screen::Screen;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for SkyblockerScreen {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl SkyblockerScreen {
    /// java: static { .. } - Java runs it when the class is first used; every static method and
    /// constructor calls this first.
    fn clinit() {
        static ONCE: std::sync::Once = std::sync::Once::new();
        ONCE.call_once(|| {
            if FunUtils::should_enable_fun() {
                *TITLE.get_mut() = Some(component::literal(&format!("Skibidiblocker {}", *skyblocker_mod::VERSION)));
                *ICON.get_mut() = Some(crate::entry::skyblocker_mod::id("icons.png"));
            } else {
                *TITLE.get_mut() = Some(component::literal(&format!("Skyblocker {}", *skyblocker_mod::VERSION)));
                *ICON.get_mut() = Some(crate::entry::skyblocker_mod::id("icon.png"));
            }
        });
    }

    /// java: public SkyblockerScreen()
    pub fn new() -> compat::Shared<Self> {
        Self::clinit();
        let __this = compat::shared(Self {
            base: mcrust::net::minecraft::client::gui::screens::screen::Screen::new(TITLE.get_mut().as_ref().unwrap().get_string()),
            self_ref: std::sync::Weak::new(),
            __widgets: Default::default(),
            layout: None,
            tip: None,
            layout_indices: Vec::new(),
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        { let __v = Some(HeaderAndFooterLayout::with_heights(0, 0, 64, 100)); compat::write(&__this, |__t| __t.layout = __v); }
        // java: tip = new MultiLineTextWidget(Component.empty(), this.font) - replaced by `init` before anything reads it;
        // the widget is made (and put in the screen's list) there
        __this
    }

    /// java: @Init public static void initClass()
    pub fn init_class() {
        Self::clinit();
        client_command_registration_callback::EVENT.register(Arc::new(
            |dispatcher: &mut client_command_registration_callback::ClientCommandDispatcher| {
                // TODO(fabric2seer): a command tree this tool cannot read (arguments, redirects, requires) - SeerClient commands receive the words after the name in `ctx.args`
                //   java: dispatcher.register(..);
            },
        ));
    }

    /// java: @Override protected void init()
    pub fn init(__this: &compat::Shared<Self>) {
        // Slightly larger footer than header to move body content higher
        // because the body content is positioned at footer height + 30 for some reason
        // 320 is default height at gui scale 3
        // java: this.layout = new HeaderAndFooterLayout(this, ..) - built from the widgets below in `build_layout`
        // java: this.layout.addToHeader(new IconTextWidget(this.getTitle(), this.font, ICON));
        let __title: MutableComponent = component::literal(&compat::read(__this, |__t| __t.get_title().to_string()));
        let mut __icon_widget = compat::WidgetRef::value(compat::WidgetAdapter::widget(compat::shared(IconTextWidget::new(__title, compat::font(), ICON.get().clone().unwrap()))));
        let (icon_index, _, _) = compat::write(__this, |__t| { let __s = &mut *__t; __s.__widgets.add_to_layout(&mut __s.base, &mut __icon_widget) });
        let mut indices: Vec<usize> = vec![icon_index];

        // java: adder.addChild(Button.builder(CONFIGURATION_TEXT, _ -> this.openConfig()).width(BUTTON_WIDTH).build(), 2);
        let __this_w = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        indices.push(SkyblockerScreen::add_button(__this, CONFIGURATION_TEXT.get().get_string(), BUTTON_WIDTH, OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| {
            let Some(__this_1) = __this_w.get().upgrade() else { return; };
            compat::lend_screen(&__this_1, __screen, || SkyblockerScreen::open_config(&__this_1));
        }))));
        // java: adder.addChild(Button.builder(LOGS_FOLDER_TEXT, _ -> LogsFolderFinder.openLogsFolder()).width(HALF_BUTTON_WIDTH).build());
        indices.push(SkyblockerScreen::add_button(__this, LOGS_FOLDER_TEXT.get().get_string(), *HALF_BUTTON_WIDTH, OnPress::Lambda(PressLambda::new(|_: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, _: usize| {
            LogsFolderFinder::open_logs_folder();
        }))));
        // java: adder.addChild(Button.builder(X_TEXT, ConfirmLinkScreen.confirmLink(this, url)).width(HALF_BUTTON_WIDTH).build());
        for (text, url) in [
            (&REPORT_BUGS_TEXT, "https://github.com/SkyblockerMod/Skyblocker/issues"),
            (&SOURCE_TEXT, "https://github.com/SkyblockerMod/Skyblocker"),
            (&TRANSLATE_TEXT, "https://translate.hysky.de/"),
            (&WEBSITE_TEXT, "https://hysky.de/"),
            (&DISCORD_TEXT, "https://discord.gg/aNNJHQykck"),
            (&MODRINTH_TEXT, "https://modrinth.com/mod/skyblocker-liap"),
            (&CURSEFORGE_TEXT, "https://www.curseforge.com/minecraft/mc-mods/skyblocker"),
            (&SUPPORT_US_TEXT, "https://hysky.de/skyblocker/team"),
        ] {
            indices.push(SkyblockerScreen::add_button(__this, text.get().get_string(), *HALF_BUTTON_WIDTH, compat::confirm_link(url)));
        }
        // java: adder.addChild(Button.builder(CREDITS_TEXT, _ -> this.minecraft.gui.setScreen(new SkyblockerCreditsScreen(this))).width(HALF_BUTTON_WIDTH).build());
        indices.push(SkyblockerScreen::add_button(__this, CREDITS_TEXT.get().get_string(), *HALF_BUTTON_WIDTH, OnPress::Lambda(PressLambda::new(|_: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, _: usize| {
            mcrust::mcrust::modding::game::set_screen_keeping_parent(Box::new(compat::ScreenAdapter::new(SkyblockerCreditsScreen::new(None))));
        }))));
        // java: adder.addChild(Button.builder(CommonComponents.GUI_DONE, _ -> this.onClose()).width(BUTTON_WIDTH).build(), 2);
        // (`Screen.onClose:195-197` - `setScreen(null)`)
        indices.push(SkyblockerScreen::add_button(__this, mcrust::net::minecraft::network::chat::common_components::gui_done().get_string(), BUTTON_WIDTH, OnPress::Lambda(PressLambda::new(|_: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, _: usize| {
            mcrust::mcrust::modding::game::set_screen_null();
        }))));

        // java: footerAdder.addChild(tip = new MultiLineTextWidget(Tips.nextTip(), this.font).setCentered(true).setMaxWidth((int) (this.width * 0.8)), 2);
        let width: i32 = compat::read(__this, |__t| __t.width);
        let tip = MultiLineTextWidget::new(0, 0, mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(Tips::next_tip())).set_centered(true).set_max_width(((width as f64) * 0.8) as i32);
        let tip_index: usize = compat::write(__this, |__t| __t.add_widget(SkyblockerScreen::tip_widget(tip)));
        compat::write(__this, |__t| __t.tip = Some(tip_index));
        indices.push(tip_index);
        // java: footerAdder.addChild(Button.builder(Component.translatable("skyblocker.tips.previous"), _ -> { tip.setMessage(Tips.previousTip()); layout.arrangeElements(); }).width(HALF_BUTTON_WIDTH).build());
        let __this_w = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        indices.push(SkyblockerScreen::add_button(__this, component::translatable("skyblocker.tips.previous").get_string(), *HALF_BUTTON_WIDTH, OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| {
            let Some(__this_1) = __this_w.get().upgrade() else { return; };
            compat::lend_screen(&__this_1, __screen, || SkyblockerScreen::set_tip(&__this_1, Tips::previous_tip()));
        }))));
        // java: footerAdder.addChild(Button.builder(Component.translatable("skyblocker.tips.next"), _ -> { tip.setMessage(Tips.nextTip()); layout.arrangeElements(); }).width(HALF_BUTTON_WIDTH).build());
        let __this_w = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        indices.push(SkyblockerScreen::add_button(__this, component::translatable("skyblocker.tips.next").get_string(), *HALF_BUTTON_WIDTH, OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| {
            let Some(__this_1) = __this_w.get().upgrade() else { return; };
            compat::lend_screen(&__this_1, __screen, || SkyblockerScreen::set_tip(&__this_1, Tips::next_tip()));
        }))));
        compat::write(__this, |__t| __t.layout_indices = indices);

        // java: this.layout.arrangeElements(); this.layout.visitWidgets(this::addRenderableWidget);
        SkyblockerScreen::arrange(__this);
        { let __vw = __this.lock().borrow().layout.as_ref().unwrap().visit_widgets(); for (__i, _, _) in __vw { compat::write(__this, |__t| __t.mark_renderable(__i)); } };
    }

    /// `Button.builder(text, onPress).width(width).build()` added to the screen; its index.
    fn add_button(__this: &compat::Shared<Self>, text: String, width: i32, on_press: OnPress) -> usize {
        let __w = button::builder(text, on_press).width(width).build();
        compat::write(__this, |__t| __t.add_widget(__w))
    }

    /// The game's widget for a plain `MultiLineTextWidget` (`WidgetKind::PlainMultiLineText`), sized as the text lays
    /// out in `this.font`.
    fn tip_widget(tip: MultiLineTextWidget) -> AbstractWidget {
        let splitter = compat::minecraft().font.splitter();
        let (w, h) = (tip.get_width(&splitter), tip.get_height(&splitter));
        let message = tip.message.clone();
        let mut widget = AbstractWidget::new(0, 0, w, h, message.get_string(), mcrust::net::minecraft::client::gui::components::abstract_widget::WidgetKind::PlainMultiLineText(Box::new(tip)), OnPress::Inert);
        widget.set_message_text(message);
        widget.active = false;
        widget
    }

    /// `tip.setMessage(text); layout.arrangeElements();` - the two tip buttons. A `MultiLineTextWidget`'s size follows
    /// its text, so the widget is re-measured before the layout is arranged.
    fn set_tip(__this: &compat::Shared<Self>, text: MutableComponent) {
        let Some(tip_index) = compat::read(__this, |__t| __t.tip) else { return; };
        let message = mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(text);
        let splitter = compat::minecraft().font.splitter();
        compat::write(__this, |__t| {
            let widget = __t.widget_mut(tip_index);
            if let mcrust::net::minecraft::client::gui::components::abstract_widget::WidgetKind::PlainMultiLineText(m) = &mut widget.kind {
                m.message = message.clone();
                let (w, h) = (m.get_width(&splitter), m.get_height(&splitter));
                widget.width = w;
                widget.height = h;
            }
            widget.set_message_text(message);
        });
        SkyblockerScreen::arrange(__this);
    }

    /// The `HeaderAndFooterLayout` of `init` over the widgets it added (`layout_indices`), built from their live
    /// sizes (the port's layout leaf keeps the size it was given, and the tip changes size), then arranged and the
    /// positions handed back to the widgets: `layout.arrangeElements()`.
    fn arrange(__this: &compat::Shared<Self>) {
        let (screen_width, screen_height, indices) = compat::read(__this, |__t| (__t.width, __t.height, __t.layout_indices.clone()));
        if indices.len() != 16 {
            return;
        }
        let leaf = |i: usize| -> LayoutElement { let (w, h) = compat::read(__this, |__t| (__t.widget(i).get_width(), __t.widget(i).get_height())); LayoutElement::widget(i, w, h) };
        // java: new HeaderAndFooterLayout(this, this.height < 320 ? 48 : 64, this.height < 320 ? 64 : 100)
        let mut layout = HeaderAndFooterLayout::with_heights(screen_width, screen_height, if screen_height < 320 { 48 } else { 64 }, if screen_height < 320 { 64 } else { 100 });
        layout.add_to_header(leaf(indices[0]));
        // java: GridLayout gridWidget = this.layout.addToContents(new GridLayout()).spacing(SPACING);
        let mut grid_widget: GridLayout = GridLayout::new_default();
        grid_widget.spacing(SPACING);
        if screen_height < 360 {
            grid_widget.row_spacing(4);
        }
        { let __s = grid_widget.default_cell_setting(); *__s = __s.copy().align_horizontally_center(); }
        let mut adder: RowHelper = grid_widget.create_row_helper(2);
        adder.add_child_width(&mut grid_widget, leaf(indices[1]), 2).unwrap();
        for i in 2..12 {
            adder.add_child(&mut grid_widget, leaf(indices[i])).unwrap();
        }
        adder.add_child_width(&mut grid_widget, leaf(indices[12]), 2).unwrap();
        layout.add_to_contents(LayoutElement::Grid(grid_widget));
        // java: GridLayout footerGridWidget = this.layout.addToFooter(new GridLayout()).columnSpacing(SPACING).rowSpacing(0);
        let mut footer_grid_widget: GridLayout = GridLayout::new_default();
        footer_grid_widget.column_spacing(SPACING);
        footer_grid_widget.row_spacing(0);
        { let __s = footer_grid_widget.default_cell_setting(); *__s = __s.copy().align_horizontally_center(); }
        let mut footer_adder: RowHelper = footer_grid_widget.create_row_helper(2);
        footer_adder.add_child_width(&mut footer_grid_widget, leaf(indices[13]), 2).unwrap();
        footer_adder.add_child(&mut footer_grid_widget, leaf(indices[14])).unwrap();
        footer_adder.add_child(&mut footer_grid_widget, leaf(indices[15])).unwrap();
        layout.add_to_footer(LayoutElement::Grid(footer_grid_widget));
        layout.arrange_elements();
        let placed = layout.visit_widgets();
        compat::write(__this, |__t| __t.layout = Some(layout));
        for (__i, __x, __y) in placed {
            compat::write(__this, |__t| __t.widget_mut(__i).set_position(__x, __y));
        }
    }

    /// java: @Override protected void repositionElements()
    pub fn reposition_elements(__this: &compat::Shared<Self>) {
        // java: super.repositionElements(); - the game repositions the screen it lends (`ModScreen::reposition_elements`
        // is called in its place); then `this.layout.arrangeElements()`
        SkyblockerScreen::arrange(__this);
    }

    /// java: private void openConfig()
    pub fn open_config(__this: &compat::Shared<Self>) {
        // java: this.minecraft.gui.setScreen(SkyblockerConfigManager.createGUI(this));
        // (`this` is the screen that is up: the config screen keeps it as its parent)
        let _ = __this;
        crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::create_gui(Some(dandelion::compat::ScreenRef::Current)).set();
    }

    /// java: @Override public void extractRenderState(GuiGraphicsExtractor context, int mouseX, int mouseY, float delta)
    pub fn extract_render_state(__this: &compat::Shared<Self>, context: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut delta: f32) {
        ();
    }

}

/// SkyblockerScreen's `Screen` half (`super`'s fields and widget tree), lent by the game for each call
impl compat::ScreenHalf for SkyblockerScreen {
    fn screen_half(&mut self) -> &mut mcrust::net::minecraft::client::gui::screens::screen::Screen {
        &mut self.base
    }
    fn screen_parts(&mut self) -> (&mut mcrust::net::minecraft::client::gui::screens::screen::Screen, Option<&mut compat::WidgetCache>) {
        let __s = self;
        (&mut __s.base, Some(&mut __s.__widgets))
    }
}

/// SkyblockerScreen as the game's screen: `game::set_screen(Box::new(compat::ScreenAdapter::new(..)))`
impl ModScreen for compat::ScreenAdapter<SkyblockerScreen> {
    fn get_title(&self) -> String {
        self.title()
    }

    /// java: @Override protected void init()
    fn init(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        self.lend(screen, |__h| SkyblockerScreen::init(__h))
    }

    /// java: @Override protected void repositionElements()
    fn reposition_elements(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        self.lend(screen, |__h| SkyblockerScreen::reposition_elements(__h))
    }

    /// java: Screen.tick (the base body)
    fn tick(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        self.lend(screen, |_| ())
    }

    /// java: @Override public void extractRenderState(GuiGraphicsExtractor context, int mouseX, int mouseY, float delta)
    fn extract_render_state(&mut self, screen: &mcrust::net::minecraft::client::gui::screens::screen::Screen, context: &mut ScreenRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        self.lend_ref(screen, |__h| SkyblockerScreen::extract_render_state(__h, &context.graphics, mouse_x, mouse_y, a))
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

}

// NOTE(fabric2seer): extractRenderState, init, repositionElements override(s) of Screen - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them

/// java: private static class IconTextWidget extends StringWidget
pub struct IconTextWidget {
    /// java: extends net.minecraft.client.gui.components.StringWidget
    pub base: AbstractWidget,
    /// java: private final Identifier icon;
    pub icon: Identifier,
}

impl std::ops::Deref for IconTextWidget {
    type Target = AbstractWidget;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for IconTextWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl IconTextWidget {
    /// java: IconTextWidget(Component message, Font textRenderer, Identifier icon)
    pub fn new(mut message: MutableComponent, mut text_renderer: Font, mut icon: Identifier) -> Self {
        let __init_icon = icon.clone();
        let mut this = Self {
            base: compat::string_widget(0, 0, ((text_renderer.width_of_sequence(&*message.get_visual_order_text())).wrapping_add(ICON_TEXT_WIDGET_ICON_SIZE)).wrapping_add(ICON_TEXT_WIDGET_SPACING), ICON_TEXT_WIDGET_ICON_SIZE, message.clone()),
            icon: __init_icon,
        };
        this
    }

    /// java: @Override public void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float delta)
    pub fn extract_widget_render_state(&self, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut delta: f32) {
        let mut text: MutableComponent = compat::component_of(self.get_message_component());
        // java: Font textRenderer = this.getFont(); - the font the constructor was handed (`this.font`)
        let mut text_renderer: Font = compat::font();
        // java: int width = this.getWidth();
        let mut width: i32 = self.get_width();
        let mut text_width: i32 = text_renderer.width_of_sequence(&*text.get_visual_order_text());
        let mut horizontal_alignment: f32 = 0.5f32;
        let mut x: i32 = (self.get_x().wrapping_add((ICON_TEXT_WIDGET_ICON_SIZE.wrapping_add(ICON_TEXT_WIDGET_SPACING)).wrapping_div(2))).wrapping_add(compat::java_round_f32((horizontal_alignment * ((width.wrapping_sub(text_width)) as f32))));
        let mut y: i32 = self.get_y().wrapping_add((self.get_height().wrapping_sub(mcrust::net::minecraft::client::gui::font::LINE_HEIGHT)).wrapping_div(2));
        let mut ordered_text: Seq = (if text_width > width { self.trim(&text, width) } else { text.get_visual_order_text() });
        let mut icon_x: i32 = (x.wrapping_sub(ICON_TEXT_WIDGET_ICON_SIZE)).wrapping_sub(ICON_TEXT_WIDGET_SPACING);
        let mut icon_y: i32 = y.wrapping_sub(13);
        gui_graphics_extractor::draw_formatted_char_sequence(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &sequence_text(&*ordered_text), x, y, mcrust::net::minecraft::util::common_colors::WHITE);
        gui_graphics_extractor::blit_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), &self.icon, icon_x, icon_y, ((0) as f32) as f32, ((0) as f32) as f32, ICON_TEXT_WIDGET_ICON_SIZE, ICON_TEXT_WIDGET_ICON_SIZE, ICON_TEXT_WIDGET_ICON_SIZE, ICON_TEXT_WIDGET_ICON_SIZE);
    }

    /// java: private FormattedCharSequence trim(Component text, int width)
    pub fn trim(&self, text: &MutableComponent, mut width: i32) -> Seq {
        // java: Font textRenderer = this.getFont();
        let mut text_renderer: Font = compat::font();
        let mut string_visitable: mcrust::net::minecraft::network::chat::formatted_text::FormattedText = text_renderer.substr_by_width(&mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(text.clone()), width.wrapping_sub(text_renderer.width_of_cached(&mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of(mcrust::net::minecraft::network::chat::common_components::ELLIPSIS))));
        // java: return Language.getInstance().getVisualOrder(FormattedText.composite(stringVisitable, CommonComponents.ELLIPSIS));
        return language::get_instance().get_visual_order(&mcrust::net::minecraft::network::chat::formatted_text::FormattedText::composite(vec![string_visitable, mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of(mcrust::net::minecraft::network::chat::common_components::ELLIPSIS)]));
    }

}

/// IconTextWidget's widget half
impl compat::WidgetHalf for IconTextWidget {
    fn widget_half(&mut self) -> &mut AbstractWidget {
        &mut self.base
    }
}

/// IconTextWidget as the game's widget (`this.layout.addToHeader(new IconTextWidget(..))`)
impl mcrust::mcrust::modding::widgets::ModWidget for compat::WidgetAdapter<IconTextWidget> {
    /// java: @Override public void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float delta)
    fn extract_widget_render_state(&mut self, widget: &AbstractWidget, context: &mut mcrust::mcrust::modding::widgets::WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        self.lend_ref(widget, |__h| compat::read(__h, |__t| __t.extract_widget_render_state(&context.graphics, mouse_x, mouse_y, a)))
    }

    /// java: StringWidget.updateWidgetNarration (an empty body)
    fn update_widget_narration(&mut self, _widget: &AbstractWidget, _output: &mut dyn mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput) {}
}

/// The port draws a `FormattedText` where Java draws a `FormattedCharSequence`: the sequence's styled
/// characters, gathered back into runs of one style each.
fn sequence_text(seq: &dyn FormattedCharSequence) -> mcrust::net::minecraft::network::chat::formatted_text::FormattedText {
    use mcrust::net::minecraft::network::chat::formatted_text::FormattedText;
    use mcrust::net::minecraft::network::chat::style::Style;
    let mut runs: Vec<(Style, String)> = Vec::new();
    seq.accept(&mut |_position: i32, style: &Style, codepoint: u32| {
        if let Some(c) = char::from_u32(codepoint) {
            match runs.last_mut() {
                Some((s, text)) if s == style => text.push(c),
                _ => runs.push((style.clone(), c.to_string())),
            }
        }
        true
    });
    FormattedText::composite(runs.into_iter().map(|(style, text)| FormattedText::of_styled(text, style)).collect())
}

