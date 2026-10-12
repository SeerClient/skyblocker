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
    pub tip: Option<MultiLineTextWidget>,
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
                *TITLE.get_mut() = Some(component::literal(&format!("Skibidiblocker {}", skyblocker_mod::VERSION)));
                *ICON.get_mut() = Some(crate::entry::skyblocker_mod::id("icons.png"));
            } else {
                *TITLE.get_mut() = Some(component::literal(&format!("Skyblocker {}", skyblocker_mod::VERSION)));
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
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        { let __v = Some(HeaderAndFooterLayout::with_heights(0, 0, 64, 100)); compat::write(&__this, |__t| __t.layout = __v); }
        // TODO(fabric2seer): the port's `new` ports this Java method with other parameters (x, y, message for Java's message)
        // TODO(fabric2seer): `new` takes 3 argument(s) in the port (`x: i32, y: i32, message: FormattedText`), Java passes 2
        // rust: { let __v = Some(MultiLineTextWidget::new(component::empty(), compat::font())); compat::write(&__this, |__t| __t.tip = __v); }
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
        { let __v = Some(HeaderAndFooterLayout::with_heights(0, 0, (if compat::read(__this, |__t| __t.height) < 320 { 48 } else { 64 }), (if compat::read(__this, |__t| __t.height) < 320 { 64 } else { 100 }))); compat::write(__this, |__t| __t.layout = __v); }
        { let (__q0,) = (IconTextWidget::new(__this.lock().borrow().get_title().to_string(), compat::font(), ICON.get_mut().clone().unwrap()),); __this.lock().borrow_mut().layout.as_mut().unwrap().add_to_header(__q0) };
        // TODO(fabric2seer): `.spacing(..)` on a value of unknown type
        //   java: GridLayout gridWidget = this.layout.addToContents(new GridLayout()).spacing(SPACING);
        let mut grid_widget: GridLayout = todo!("fabric2seer: the initialiser of `gridWidget` (TODO above)");
        if compat::read(__this, |__t| __t.height) < 360 {
            grid_widget = { grid_widget.row_spacing(4); &mut grid_widget }.clone();
        }
        grid_widget.default_cell_setting().align_horizontally_center();
        let mut adder: RowHelper = grid_widget.create_row_helper(2);
        // TODO(fabric2seer): the port's `add_child_width` ports this Java method with other parameters (grid, child, column_width for Java's widget, column_width)
        // TODO(fabric2seer): `add_child_width` takes 3 argument(s) in the port (`grid: &mut GridLayout, child: LayoutElement, column_width: i32`), Java passes 2
        // rust: let __this_1 = compat::RenderThread::new(__this.clone());
        // rust: adder.add_child_width(&mut (button::builder(CONFIGURATION_TEXT.get_mut().get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { compat::lend_screen(__this_1.get(), __screen, || { let mut __pidx = compat::WidgetRef::Screen(__index); { SkyblockerScreen::open_config(__this_1.get()); } }) })))).width(BUTTON_WIDTH).build(), 2).unwrap();
        // TODO(fabric2seer): the port's `add_child` ports this Java method with other parameters (grid, child for Java's widget)
        // TODO(fabric2seer): `add_child` takes 2 argument(s) in the port (`grid: &mut GridLayout, child: LayoutElement`), Java passes 1
        // rust: adder.add_child(&mut (button::builder(LOGS_FOLDER_TEXT.get_mut().get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { LogsFolderFinder::open_logs_folder(); })))).width(*HALF_BUTTON_WIDTH).build()).unwrap();
        // TODO(fabric2seer): static `ConfirmLinkScreen.confirmLink` has no Rust counterpart in the port
        // TODO(fabric2seer): the port's `add_child` ports this Java method with other parameters (grid, child for Java's widget)
        // TODO(fabric2seer): `add_child` takes 2 argument(s) in the port (`grid: &mut GridLayout, child: LayoutElement`), Java passes 1
        // rust: adder.add_child(&mut (button::builder(REPORT_BUGS_TEXT.get_mut().get_string(), ConfirmLinkScreen::confirm_link(__this.clone(), "https://github.com/SkyblockerMod/Skyblocker/issues".to_string()))).width(*HALF_BUTTON_WIDTH).build()).unwrap();
        // TODO(fabric2seer): static `ConfirmLinkScreen.confirmLink` has no Rust counterpart in the port
        // TODO(fabric2seer): the port's `add_child` ports this Java method with other parameters (grid, child for Java's widget)
        // TODO(fabric2seer): `add_child` takes 2 argument(s) in the port (`grid: &mut GridLayout, child: LayoutElement`), Java passes 1
        // rust: adder.add_child(&mut (button::builder(SOURCE_TEXT.get_mut().get_string(), ConfirmLinkScreen::confirm_link(__this.clone(), "https://github.com/SkyblockerMod/Skyblocker".to_string()))).width(*HALF_BUTTON_WIDTH).build()).unwrap();
        // TODO(fabric2seer): static `ConfirmLinkScreen.confirmLink` has no Rust counterpart in the port
        // TODO(fabric2seer): the port's `add_child` ports this Java method with other parameters (grid, child for Java's widget)
        // TODO(fabric2seer): `add_child` takes 2 argument(s) in the port (`grid: &mut GridLayout, child: LayoutElement`), Java passes 1
        // rust: adder.add_child(&mut (button::builder(TRANSLATE_TEXT.get_mut().get_string(), ConfirmLinkScreen::confirm_link(__this.clone(), "https://translate.hysky.de/".to_string()))).width(*HALF_BUTTON_WIDTH).build()).unwrap();
        // TODO(fabric2seer): static `ConfirmLinkScreen.confirmLink` has no Rust counterpart in the port
        // TODO(fabric2seer): the port's `add_child` ports this Java method with other parameters (grid, child for Java's widget)
        // TODO(fabric2seer): `add_child` takes 2 argument(s) in the port (`grid: &mut GridLayout, child: LayoutElement`), Java passes 1
        // rust: adder.add_child(&mut (button::builder(WEBSITE_TEXT.get_mut().get_string(), ConfirmLinkScreen::confirm_link(__this.clone(), "https://hysky.de/".to_string()))).width(*HALF_BUTTON_WIDTH).build()).unwrap();
        // TODO(fabric2seer): static `ConfirmLinkScreen.confirmLink` has no Rust counterpart in the port
        // TODO(fabric2seer): the port's `add_child` ports this Java method with other parameters (grid, child for Java's widget)
        // TODO(fabric2seer): `add_child` takes 2 argument(s) in the port (`grid: &mut GridLayout, child: LayoutElement`), Java passes 1
        // rust: adder.add_child(&mut (button::builder(DISCORD_TEXT.get_mut().get_string(), ConfirmLinkScreen::confirm_link(__this.clone(), "https://discord.gg/aNNJHQykck".to_string()))).width(*HALF_BUTTON_WIDTH).build()).unwrap();
        // TODO(fabric2seer): static `ConfirmLinkScreen.confirmLink` has no Rust counterpart in the port
        // TODO(fabric2seer): the port's `add_child` ports this Java method with other parameters (grid, child for Java's widget)
        // TODO(fabric2seer): `add_child` takes 2 argument(s) in the port (`grid: &mut GridLayout, child: LayoutElement`), Java passes 1
        // rust: adder.add_child(&mut (button::builder(MODRINTH_TEXT.get_mut().get_string(), ConfirmLinkScreen::confirm_link(__this.clone(), "https://modrinth.com/mod/skyblocker-liap".to_string()))).width(*HALF_BUTTON_WIDTH).build()).unwrap();
        // TODO(fabric2seer): static `ConfirmLinkScreen.confirmLink` has no Rust counterpart in the port
        // TODO(fabric2seer): the port's `add_child` ports this Java method with other parameters (grid, child for Java's widget)
        // TODO(fabric2seer): `add_child` takes 2 argument(s) in the port (`grid: &mut GridLayout, child: LayoutElement`), Java passes 1
        // rust: adder.add_child(&mut (button::builder(CURSEFORGE_TEXT.get_mut().get_string(), ConfirmLinkScreen::confirm_link(__this.clone(), "https://www.curseforge.com/minecraft/mc-mods/skyblocker".to_string()))).width(*HALF_BUTTON_WIDTH).build()).unwrap();
        // TODO(fabric2seer): static `ConfirmLinkScreen.confirmLink` has no Rust counterpart in the port
        // TODO(fabric2seer): the port's `add_child` ports this Java method with other parameters (grid, child for Java's widget)
        // TODO(fabric2seer): `add_child` takes 2 argument(s) in the port (`grid: &mut GridLayout, child: LayoutElement`), Java passes 1
        // rust: adder.add_child(&mut (button::builder(SUPPORT_US_TEXT.get_mut().get_string(), ConfirmLinkScreen::confirm_link(__this.clone(), "https://hysky.de/skyblocker/team".to_string()))).width(*HALF_BUTTON_WIDTH).build()).unwrap();
        // TODO(fabric2seer): the port's `add_child` ports this Java method with other parameters (grid, child for Java's widget)
        // TODO(fabric2seer): `add_child` takes 2 argument(s) in the port (`grid: &mut GridLayout, child: LayoutElement`), Java passes 1
        // rust: adder.add_child(&mut (button::builder(CREDITS_TEXT.get_mut().get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { mcrust::mcrust::modding::game::set_screen_keeping_parent(Box::new(compat::ScreenAdapter::new(SkyblockerCreditsScreen::new(None)))); })))).width(*HALF_BUTTON_WIDTH).build()).unwrap();
        // TODO(fabric2seer): `on_close_for_interrupt` takes 12 argument(s) in the port (`gfx: &mut crate::net::minecraft::client::minecraft::Minecraft, screen: &mut Screen, screen_extra: &mut crate::net::minecraft::client::gui::gui::ScreenExtra, options_data: &mut crate::net::minecraft::client::options::OptionsData, open_sign_e ..`), Java passes 0
        // TODO(fabric2seer): the port's `add_child_width` ports this Java method with other parameters (grid, child, column_width for Java's widget, column_width)
        // TODO(fabric2seer): `add_child_width` takes 3 argument(s) in the port (`grid: &mut GridLayout, child: LayoutElement, column_width: i32`), Java passes 2
        // rust: adder.add_child_width(&mut (button::builder(mcrust::net::minecraft::network::chat::common_components::gui_done().get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { screen::on_close_for_interrupt(); })))).width(BUTTON_WIDTH).build(), 2).unwrap();
        // TODO(fabric2seer): `.columnSpacing(..)` on a value of unknown type
        //   java: GridLayout footerGridWidget = this.layout.addToFooter(new GridLayout()).columnSpacing(SPACING).rowSpacing(0);
        let mut footer_grid_widget: GridLayout = todo!("fabric2seer: the initialiser of `footerGridWidget` (TODO above)");
        footer_grid_widget.default_cell_setting().align_horizontally_center();
        let mut footer_adder: RowHelper = footer_grid_widget.create_row_helper(2);
        // TODO(fabric2seer): an assignment used as a value
        //   java: footerAdder.addChild(tip = new MultiLineTextWidget(Tips.nextTip(), this.font).setCentered(true).setMaxWidth((int) (this.width * 0.8)), 2);
        // TODO(fabric2seer): the port's `add_child` ports this Java method with other parameters (grid, child for Java's widget)
        // TODO(fabric2seer): `add_child` takes 2 argument(s) in the port (`grid: &mut GridLayout, child: LayoutElement`), Java passes 1
        // rust: let __this_2 = compat::RenderThread::new(__this.clone());
        // rust: footer_adder.add_child(&mut (button::builder(component::translatable("skyblocker.tips.previous").get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { compat::lend_screen(__this_2.get(), __screen, || { let mut __pidx = compat::WidgetRef::Screen(__index); {
        // rust: // TODO(fabric2seer): `AbstractWidget::set_message_text` ports `setMessage`, but the port's `MultiLineTextWidget` holds no `AbstractWidget` (no Deref, no `base` field) to call it on
        // rust: // rust: { let (__q0,) = (mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(Tips::previous_tip()),); __this_2.get().lock().borrow_mut().tip.as_mut().unwrap().set_message_text(__q0) };
        // rust: let __r0 = compat::read(__this_2.get(), |__t| (__t.width, __t.height));
        // rust: { let (__sw, __sh) = __r0; __this_2.get().lock().borrow_mut().layout.as_mut().unwrap().screen_width = __sw; __this_2.get().lock().borrow_mut().layout.as_mut().unwrap().screen_height = __sh; __this_2.get().lock().borrow_mut().layout.as_mut().unwrap().arrange_elements(); let __vw = __this_2.get().lock().borrow().layout.as_ref().unwrap().visit_widgets(); for (__i, __x, __y) in __vw { compat::write(__this_2.get(), |__t| __t.widget_mut(__i).set_position(__x, __y)); } };
        // rust: } }) })))).width(*HALF_BUTTON_WIDTH).build()).unwrap();
        // TODO(fabric2seer): the port's `add_child` ports this Java method with other parameters (grid, child for Java's widget)
        // TODO(fabric2seer): `add_child` takes 2 argument(s) in the port (`grid: &mut GridLayout, child: LayoutElement`), Java passes 1
        // rust: let __this_3 = compat::RenderThread::new(__this.clone());
        // rust: footer_adder.add_child(&mut (button::builder(component::translatable("skyblocker.tips.next").get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { compat::lend_screen(__this_3.get(), __screen, || { let mut __pidx = compat::WidgetRef::Screen(__index); {
        // rust: // TODO(fabric2seer): `AbstractWidget::set_message_text` ports `setMessage`, but the port's `MultiLineTextWidget` holds no `AbstractWidget` (no Deref, no `base` field) to call it on
        // rust: // rust: { let (__q0,) = (mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(Tips::next_tip()),); __this_3.get().lock().borrow_mut().tip.as_mut().unwrap().set_message_text(__q0) };
        // rust: let __r0 = compat::read(__this_3.get(), |__t| (__t.width, __t.height));
        // rust: { let (__sw, __sh) = __r0; __this_3.get().lock().borrow_mut().layout.as_mut().unwrap().screen_width = __sw; __this_3.get().lock().borrow_mut().layout.as_mut().unwrap().screen_height = __sh; __this_3.get().lock().borrow_mut().layout.as_mut().unwrap().arrange_elements(); let __vw = __this_3.get().lock().borrow().layout.as_ref().unwrap().visit_widgets(); for (__i, __x, __y) in __vw { compat::write(__this_3.get(), |__t| __t.widget_mut(__i).set_position(__x, __y)); } };
        // rust: } }) })))).width(*HALF_BUTTON_WIDTH).build()).unwrap();
        let __r0 = compat::read(__this, |__t| (__t.width, __t.height));
        { let (__sw, __sh) = __r0; __this.lock().borrow_mut().layout.as_mut().unwrap().screen_width = __sw; __this.lock().borrow_mut().layout.as_mut().unwrap().screen_height = __sh; __this.lock().borrow_mut().layout.as_mut().unwrap().arrange_elements(); let __vw = __this.lock().borrow().layout.as_ref().unwrap().visit_widgets(); for (__i, __x, __y) in __vw { compat::write(__this, |__t| __t.widget_mut(__i).set_position(__x, __y)); } };
        { let __vw = __this.lock().borrow().layout.as_ref().unwrap().visit_widgets(); for (__i, _, _) in __vw { compat::write(__this, |__t| __t.mark_renderable(__i)); } };
    }

    /// java: @Override protected void repositionElements()
    pub fn reposition_elements(__this: &compat::Shared<Self>) {
        // TODO(fabric2seer): `reposition_elements` takes 1 argument(s) in the port (`build: impl FnOnce(&mut Screen)`), Java passes 0
        // rust: __this.lock().borrow_mut().base.reposition_elements();
        let __r0 = compat::read(__this, |__t| (__t.width, __t.height));
        { let (__sw, __sh) = __r0; __this.lock().borrow_mut().layout.as_mut().unwrap().screen_width = __sw; __this.lock().borrow_mut().layout.as_mut().unwrap().screen_height = __sh; __this.lock().borrow_mut().layout.as_mut().unwrap().arrange_elements(); let __vw = __this.lock().borrow().layout.as_ref().unwrap().visit_widgets(); for (__i, __x, __y) in __vw { compat::write(__this, |__t| __t.widget_mut(__i).set_position(__x, __y)); } };
    }

    /// java: private void openConfig()
    pub fn open_config(__this: &compat::Shared<Self>) {
        // TODO(fabric2seer): `set_screen` takes 14 argument(s) in the port (`gfx: &mut Minecraft, screen: &mut GuiScreen, screen_extra: &mut ScreenExtra, screen_stack: &mut Vec<HeldScreen>, screen_kind_before_pause: &mut Option<ScreenKind>, screen_is_open: &mut bool, level_screen_closed: &mut bool, frames_on_this_sc ..`), Java passes 1
        // rust: gui::set_screen(&mut SkyblockerConfigManager::create_gui(Some(__this.clone())));
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
            base: compat::string_widget(0, 0, ((text_renderer.width_of_sequence(&message.get_visual_order_text())).wrapping_add(ICON_TEXT_WIDGET_ICON_SIZE)).wrapping_add(ICON_TEXT_WIDGET_SPACING), ICON_TEXT_WIDGET_ICON_SIZE, message.clone()),
            icon: __init_icon,
        };
        this
    }

    /// java: @Override public void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float delta)
    pub fn extract_widget_render_state(&self, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut delta: f32) {
        let mut text: MutableComponent = compat::component_of(self.get_message_component());
        // TODO(fabric2seer): `StringWidget.getFont` has no Rust method in the port (looked for `get_font` in StringWidget and its supertypes)
        // rust: let mut text_renderer: Font = self.get_font();
        let mut text_renderer: Font = todo!("fabric2seer: the initialiser of `textRenderer` (TODO above)");
        // TODO(fabric2seer): `width` takes 2 argument(s) in the port (`max_width: i32, text_width: i32`), Java passes 0
        // rust: let mut width: i32 = string_widget::width();
        let mut width: i32 = todo!("fabric2seer: the initialiser of `width` (TODO above)");
        let mut text_width: i32 = text_renderer.width_of_sequence(&text.get_visual_order_text());
        let mut horizontal_alignment: f32 = 0.5f32;
        let mut x: i32 = (self.get_x().wrapping_add((ICON_TEXT_WIDGET_ICON_SIZE.wrapping_add(ICON_TEXT_WIDGET_SPACING)).wrapping_div(2))).wrapping_add(compat::java_round_f32((horizontal_alignment * ((width.wrapping_sub(text_width)) as f32))));
        let mut y: i32 = self.get_y().wrapping_add((self.get_height().wrapping_sub(mcrust::net::minecraft::client::gui::font::LINE_HEIGHT)).wrapping_div(2));
        let mut ordered_text: Seq = (if text_width > width { self.trim(&text, width) } else { text.get_visual_order_text() });
        let mut icon_x: i32 = (x.wrapping_sub(ICON_TEXT_WIDGET_ICON_SIZE)).wrapping_sub(ICON_TEXT_WIDGET_SPACING);
        let mut icon_y: i32 = y.wrapping_sub(13);
        gui_graphics_extractor::draw_formatted_char_sequence(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &ordered_text, x, y, mcrust::net::minecraft::util::common_colors::WHITE);
        gui_graphics_extractor::blit_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), &self.icon, icon_x, icon_y, ((0) as f32) as f32, ((0) as f32) as f32, ICON_TEXT_WIDGET_ICON_SIZE, ICON_TEXT_WIDGET_ICON_SIZE, ICON_TEXT_WIDGET_ICON_SIZE, ICON_TEXT_WIDGET_ICON_SIZE);
    }

    /// java: private FormattedCharSequence trim(Component text, int width)
    pub fn trim(&self, text: &MutableComponent, mut width: i32) -> Seq {
        // TODO(fabric2seer): `StringWidget.getFont` has no Rust method in the port (looked for `get_font` in StringWidget and its supertypes)
        // rust: let mut text_renderer: Font = self.get_font();
        let mut text_renderer: Font = todo!("fabric2seer: the initialiser of `textRenderer` (TODO above)");
        let mut string_visitable: mcrust::net::minecraft::network::chat::formatted_text::FormattedText = text_renderer.substr_by_width(&mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(text.clone()), width.wrapping_sub(text_renderer.width_of_cached(&mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(mcrust::net::minecraft::network::chat::common_components::ELLIPSIS))));
        // TODO(fabric2seer): `Language.getVisualOrder` has no Rust method in the port (looked for `get_visual_order` in Language and its supertypes)
        // rust: return language::get_instance().get_visual_order(mcrust::net::minecraft::network::chat::formatted_text::FormattedText::composite(vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of(compat::jstring(&string_visitable)), mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(mcrust::net::minecraft::network::chat::common_components::ELLIPSIS)]));
        return todo!("fabric2seer: see the TODO above");
    }

}

// NOTE(fabric2seer): extractWidgetRenderState override(s) of StringWidget - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them
