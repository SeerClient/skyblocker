//! Ported from de.hysky.skyblocker.skyblock.chat.ChatRuleLocationConfigScreen by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::screens::powdertracker::item_tick_list::ItemTickList;
use crate::de::hysky::skyblocker::skyblock::chat::chat_rule::ChatRule;
use crate::de::hysky::skyblocker::utils::location::Location;
use mcrust::mcrust::modding::screens::ModScreen;
use mcrust::mcrust::modding::screens::ScreenRenderContext;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::abstract_widget::PressLambda;
use mcrust::net::minecraft::client::gui::components::button;
use mcrust::net::minecraft::client::gui::layouts::frame_layout;
use mcrust::net::minecraft::client::gui::layouts::grid_layout::GridLayout;
use mcrust::net::minecraft::client::gui::layouts::grid_layout::RowHelper;
use mcrust::net::minecraft::client::gui::layouts::layout_element::LayoutElement;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::client::input::mouse_button_info::MouseButtonInfo;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: public class ChatRuleLocationConfigScreen extends Screen
pub struct ChatRuleLocationConfigScreen {
    /// java: extends net.minecraft.client.gui.screens.Screen
    pub base: mcrust::net::minecraft::client::gui::screens::screen::Screen,
    /// java: private final @Nullable Screen parent;
    pub parent: Option<mcrust::net::minecraft::client::gui::screens::screen::Screen>,
    /// java: private final ChatRule chatRule;
    pub chat_rule: compat::Shared<ChatRule>,
    /// java: private final EnumSet<Location> enabledLocations;
    pub enabled_locations: std::collections::HashSet<Location>,
    /// the screen's widgets as this object last saw them (see `compat::WidgetCache`)
    pub __widgets: compat::WidgetCache,
}

impl std::ops::Deref for ChatRuleLocationConfigScreen {
    type Target = mcrust::net::minecraft::client::gui::screens::screen::Screen;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for ChatRuleLocationConfigScreen {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl ChatRuleLocationConfigScreen {
    /// java: public ChatRuleLocationConfigScreen(@Nullable Screen parent, ChatRule chatRule)
    pub fn new(mut parent: Option<mcrust::net::minecraft::client::gui::screens::screen::Screen>, mut chat_rule: compat::Shared<ChatRule>) -> compat::Shared<Self> {
        let __init_parent = parent;
        let __init_chat_rule = chat_rule.clone();
        let __init_enabled_locations = ChatRule::get_valid_locations(&chat_rule).iter().cloned().collect::<std::collections::HashSet<_>>();
        let __this = compat::shared(Self {
            base: mcrust::net::minecraft::client::gui::screens::screen::Screen::new(component::translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.locationsConfigScreen").get_string()),
            __widgets: Default::default(),
            parent: __init_parent,
            chat_rule: __init_chat_rule,
            enabled_locations: __init_enabled_locations,
        });
        __this
    }

    /// java: @Override protected void init()
    pub fn init(__this: &compat::Shared<Self>) {
        // TODO(fabric2seer): `Screen.addRenderableOnly` has no Rust method in the port (looked for `add_renderable_only` in Screen and its supertypes)
        // rust: __this.lock().borrow().add_renderable_only(&Arc::new(move |context, _, _, _| {
        // rust: // TODO(fabric2seer): `.centeredText(..)` on a value of unknown type
        // rust: //   java: context.centeredText(minecraft.font, Component.translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.locationsConfigScreen").withStyle(ChatFormatting.BOLD), width / 2, (32 - minecraft.font.lineHeight) / 2, CommonColors.WHITE);
        // rust: // TODO(fabric2seer): `.centeredText(..)` on a value of unknown type
        // rust: //   java: context.centeredText(minecraft.font, Component.translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.locationsConfigScreen.note"), width / 2, (38 - minecraft.font.lineHeight), CommonColors.WHITE);
        // rust: }));
        // TODO(fabric2seer): static call EnumSet.complementOf(..)
        //   java: ItemTickList<Location> itemTickList = addRenderableWidget(new ItemTickList<>(minecraft, width, height - 107, 43, 24, enabledLocations, EnumSet.complementOf(EnumSet.of(Location.UNKNOWN)), true).init());
        let mut item_tick_list: compat::Shared<ItemTickList<Location>> = todo!("fabric2seer: the initialiser of `itemTickList` (TODO above)");
        let mut grid_widget: GridLayout = GridLayout::new_default();
        // (the port's `LayoutSettings` setters answer a new value: written back to the cell defaults)
        { let __s = grid_widget.default_cell_setting(); *__s = (*__s).padding_horizontal(5).padding_vertical(2); }
        let mut adder: RowHelper = grid_widget.create_row_helper(2);
        // TODO(fabric2seer): the port's `add_child` ports this Java method with other parameters (grid, child for Java's widget)
        // TODO(fabric2seer): `add_child` takes 2 argument(s) in the port (`grid: &mut GridLayout, child: LayoutElement`), Java passes 1
        // rust: let __this_1 = compat::RenderThread::new(__this.clone());
        // rust: adder.add_child(&mut (button::builder(component::translatable("text.skyblocker.reset").get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { compat::lend_screen(__this_1.get(), __screen, || { let mut __pidx = compat::WidgetRef::Screen(__index); {
        // rust: __this_1.get().lock().borrow_mut().enabled_locations.clear();
        // rust: ItemTickList::clear_and_init(&item_tick_list);
        // rust: } }) })))).build()).unwrap();
        // TODO(fabric2seer): the port's `add_child` ports this Java method with other parameters (grid, child for Java's widget)
        // TODO(fabric2seer): `add_child` takes 2 argument(s) in the port (`grid: &mut GridLayout, child: LayoutElement`), Java passes 1
        // rust: let __this_2 = compat::RenderThread::new(__this.clone());
        // rust: adder.add_child(&mut (button::builder(component::translatable("text.skyblocker.undo").get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { compat::lend_screen(__this_2.get(), __screen, || { let mut __pidx = compat::WidgetRef::Screen(__index); {
        // rust: __this_2.get().lock().borrow_mut().enabled_locations.clear();
        // rust: let __h0 = ChatRule::get_valid_locations(&compat::read(__this_2.get(), |__t| __t.chat_rule.clone())).into_iter();
        // rust: __this_2.get().lock().borrow_mut().enabled_locations.extend(__h0);
        // rust: ItemTickList::clear_and_init(&item_tick_list);
        // rust: } }) })))).build()).unwrap();
        // TODO(fabric2seer): the port's `add_child_width` ports this Java method with other parameters (grid, child, column_width for Java's widget, column_width)
        // TODO(fabric2seer): `add_child_width` takes 3 argument(s) in the port (`grid: &mut GridLayout, child: LayoutElement, column_width: i32`), Java passes 2
        // rust: let __this_3 = compat::RenderThread::new(__this.clone());
        // rust: adder.add_child_width(&mut (button::builder(mcrust::net::minecraft::network::chat::common_components::gui_done().get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { compat::lend_screen(__this_3.get(), __screen, || { let mut __pidx = compat::WidgetRef::Screen(__index); {
        // rust: ChatRuleLocationConfigScreen::save_filters(__this_3.get());
        // rust: ChatRuleLocationConfigScreen::on_close(__this_3.get());
        // rust: } }) })))).width(((mcrust::net::minecraft::client::gui::components::button::DEFAULT_WIDTH).wrapping_mul(2)).wrapping_add(10)).build(), 2).unwrap();
        { grid_widget.arrange_elements(); let mut __vw = Vec::new(); LayoutElement::Grid(grid_widget.clone()).visit_widgets(&mut __vw); for (__i, __x, __y) in __vw { compat::write(__this, |__t| __t.widget_mut(__i).set_position(__x, __y)); } };
        frame_layout::center_in_rectangle(&mut *compat::LayoutBack::grid(&mut grid_widget), 0, (compat::read(__this, |__t| __t.height)).wrapping_sub(64), compat::read(__this, |__t| __t.width), 64);
        { let mut __vw = Vec::new(); LayoutElement::Grid(grid_widget.clone()).visit_widgets(&mut __vw); for (__i, _, _) in __vw { compat::write(__this, |__t| __t.mark_renderable(__i)); } };
    }

    /// java: public void saveFilters()
    pub fn save_filters(__this: &compat::Shared<Self>) {
        ChatRule::set_valid_locations(&compat::read(__this, |__t| __t.chat_rule.clone()), compat::read(__this, |__t| __t.enabled_locations.clone()));
    }

    /// java: @Override public void onClose()
    pub fn on_close(__this: &compat::Shared<Self>) {
        mcrust::mcrust::modding::game::set_screen_to_parent();
    }

}

/// ChatRuleLocationConfigScreen's `Screen` half (`super`'s fields and widget tree), lent by the game for each call
impl compat::ScreenHalf for ChatRuleLocationConfigScreen {
    fn screen_half(&mut self) -> &mut mcrust::net::minecraft::client::gui::screens::screen::Screen {
        &mut self.base
    }
    fn screen_parts(&mut self) -> (&mut mcrust::net::minecraft::client::gui::screens::screen::Screen, Option<&mut compat::WidgetCache>) {
        let __s = self;
        (&mut __s.base, Some(&mut __s.__widgets))
    }
}

/// ChatRuleLocationConfigScreen as the game's screen: `game::set_screen(Box::new(compat::ScreenAdapter::new(..)))`
impl ModScreen for compat::ScreenAdapter<ChatRuleLocationConfigScreen> {
    fn get_title(&self) -> String {
        self.title()
    }

    /// java: @Override protected void init()
    fn init(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        self.lend(screen, |__h| ChatRuleLocationConfigScreen::init(__h))
    }

    /// java: Screen.tick (the base body)
    fn tick(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        self.lend(screen, |_| ())
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
        self.call(|__h| ChatRuleLocationConfigScreen::on_close(__h))
    }

}

// NOTE(fabric2seer): init, onClose override(s) of Screen - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them
