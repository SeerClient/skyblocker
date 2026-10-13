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
    /// java: private final @Nullable Screen parent; - the game holds it (`set_screen_keeping_parent`)
    /// java: private final ChatRule chatRule;
    pub chat_rule: compat::Shared<ChatRule>,
    /// java: private final EnumSet<Location> enabledLocations; - the collection the list's checkboxes write into
    /// (`ItemTickList`'s `filters`)
    pub enabled_locations: compat::Shared<Vec<Location>>,
    /// java: ItemTickList<Location> itemTickList (a local of `init` the buttons capture): the list object and its widget
    pub item_tick_list: Option<(Arc<ItemTickList<Location>>, mcrust::mcrust::modding::widgets::ModWidgetSlot)>,
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
    ///
    /// (the parent is the screen up when this one is opened: `game::set_screen_keeping_parent` holds it)
    pub fn new(mut chat_rule: compat::Shared<ChatRule>) -> compat::Shared<Self> {
        let __init_chat_rule = chat_rule.clone();
        //   java: this.enabledLocations = EnumSet.copyOf(chatRule.getValidLocations()); - in `Location` order, as an `EnumSet` iterates
        let mut __locations: Vec<Location> = ChatRule::get_valid_locations(&chat_rule).into_iter().collect();
        __locations.sort();
        let __init_enabled_locations = compat::shared(__locations);
        let __this = compat::shared(Self {
            base: mcrust::net::minecraft::client::gui::screens::screen::Screen::new(component::translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.locationsConfigScreen").get_string()),
            __widgets: Default::default(),
            chat_rule: __init_chat_rule,
            enabled_locations: __init_enabled_locations,
            item_tick_list: None,
        });
        __this
    }

    /// `itemTickList.x(..)`: the list object and its widget's list, lent for `f` (no borrow of this screen held).
    fn with_list<R>(__this: &compat::Shared<Self>, f: impl FnOnce(&ItemTickList<Location>, &mut mcrust::net::minecraft::client::gui::components::abstract_selection_list::AbstractSelectionList) -> R) -> R {
        let (class, slot) = compat::read(__this, |__t| __t.item_tick_list.clone()).expect("ChatRuleLocationConfigScreen.itemTickList before init()");
        slot.with::<mcrust::mcrust::modding::widgets::ModSelectionList, _>(|l| f(&class, &mut l.list)).expect("the item tick list reached from inside its own method")
    }

    /// `adder.addChild(button[, occupiedColumns])`: the button joins the screen's widgets (by index) and the grid.
    fn add_to_grid(__this: &compat::Shared<Self>, grid: &mut GridLayout, adder: &mut RowHelper, button: mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget, columns: i32) {
        let (w, h) = (button.get_width(), button.get_height());
        let index = compat::write(__this, |__t| __t.add_widget(button));
        if columns == 1 {
            adder.add_child(grid, LayoutElement::widget(index, w, h)).expect("GridLayout.addChild");
        } else {
            adder.add_child_width(grid, LayoutElement::widget(index, w, h), columns).expect("GridLayout.addChild");
        }
    }

    /// `addRenderableOnly((context, _, _, _) -> ..)`, the two title lines. (The port's screen keeps no mod renderables;
    /// this draws after the widgets, and the title band, `y < 43`, is outside the list.)
    pub fn extract_render_state(__this: &compat::Shared<Self>, context: &mut ScreenRenderContext<'_>) {
        let width = compat::read(__this, |__t| __t.width);
        let line_height = mcrust::net::minecraft::client::gui::font::LINE_HEIGHT;
        //   java: context.centeredText(minecraft.font, Component.translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.locationsConfigScreen").withStyle(ChatFormatting.BOLD), width / 2, (32 - minecraft.font.lineHeight) / 2, CommonColors.WHITE);
        let title = mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.locationsConfigScreen").with_style_format(mcrust::net::minecraft::chat_formatting::ChatFormatting::Bold));
        mcrust::net::minecraft::client::gui::gui_graphics_extractor::centered_text(context.gl, context.minecraft, &title, width / 2, (32 - line_height) / 2, mcrust::net::minecraft::util::common_colors::WHITE);
        //   java: context.centeredText(minecraft.font, Component.translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.locationsConfigScreen.note"), width / 2, (38 - minecraft.font.lineHeight), CommonColors.WHITE);
        let note = mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.locationsConfigScreen.note"));
        mcrust::net::minecraft::client::gui::gui_graphics_extractor::centered_text(context.gl, context.minecraft, &note, width / 2, 38 - line_height, mcrust::net::minecraft::util::common_colors::WHITE);
    }

    /// java: @Override protected void init()
    pub fn init(__this: &compat::Shared<Self>) {
        //   java: addRenderableOnly((context, _, _, _) -> { .. }); - see `extract_render_state`
        //   java: ItemTickList<Location> itemTickList = addRenderableWidget(new ItemTickList<>(minecraft, width, height - 107, 43, 24, enabledLocations, EnumSet.complementOf(EnumSet.of(Location.UNKNOWN)), true).init());
        let (width, height) = compat::read(__this, |__t| (__t.width, __t.height));
        let enabled_locations = compat::read(__this, |__t| __t.enabled_locations.clone());
        let all_locations: Vec<Location> = Location::values().into_iter().filter(|location| *location != Location::Unknown).collect();
        let (class, widget) = ItemTickList::new_8(width, height - 107, 43, 24, enabled_locations, all_locations, true);
        let slot = widget.mod_widget().expect("ItemTickList's widget");
        slot.with::<mcrust::mcrust::modding::widgets::ModSelectionList, _>(|l| class.init(&mut l.list)).expect("ItemTickList.init");
        compat::write(__this, |__t| {
            __t.item_tick_list = Some((class, slot));
            __t.add_renderable_widget(widget);
        });
        let mut grid_widget: GridLayout = GridLayout::new_default();
        // (the port's `LayoutSettings` setters answer a new value: written back to the cell defaults)
        { let __s = grid_widget.default_cell_setting(); *__s = (*__s).padding_horizontal(5).padding_vertical(2); }
        let mut adder: RowHelper = grid_widget.create_row_helper(2);
        //   java: adder.addChild(Button.builder(Component.translatable("text.skyblocker.reset"), _ -> { enabledLocations.clear(); itemTickList.clearAndInit(); }).build());
        let __this_w = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        let reset = button::builder(component::translatable("text.skyblocker.reset").get_string(), OnPress::Lambda(PressLambda::new(move |_screen, _index| {
            let Some(__this) = __this_w.get().upgrade() else { return; };
            let enabled_locations = compat::read(&__this, |__t| __t.enabled_locations.clone());
            compat::write(&enabled_locations, |l| l.clear());
            ChatRuleLocationConfigScreen::with_list(&__this, |class, list| class.clear_and_init(list));
        })))
        .build();
        ChatRuleLocationConfigScreen::add_to_grid(__this, &mut grid_widget, &mut adder, reset, 1);
        //   java: adder.addChild(Button.builder(Component.translatable("text.skyblocker.undo"), _ -> { enabledLocations.clear(); enabledLocations.addAll(chatRule.getValidLocations()); itemTickList.clearAndInit(); }).build());
        let __this_w = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        let undo = button::builder(component::translatable("text.skyblocker.undo").get_string(), OnPress::Lambda(PressLambda::new(move |_screen, _index| {
            let Some(__this) = __this_w.get().upgrade() else { return; };
            let (enabled_locations, chat_rule) = compat::read(&__this, |__t| (__t.enabled_locations.clone(), __t.chat_rule.clone()));
            let mut valid_locations: Vec<Location> = ChatRule::get_valid_locations(&chat_rule).into_iter().collect();
            valid_locations.sort();
            compat::write(&enabled_locations, |l| {
                l.clear();
                // `EnumSet.addAll`: no duplicates, in `Location` order
                for location in valid_locations {
                    if !l.contains(&location) {
                        l.push(location);
                    }
                }
                l.sort();
            });
            ChatRuleLocationConfigScreen::with_list(&__this, |class, list| class.clear_and_init(list));
        })))
        .build();
        ChatRuleLocationConfigScreen::add_to_grid(__this, &mut grid_widget, &mut adder, undo, 1);
        //   java: adder.addChild(Button.builder(CommonComponents.GUI_DONE, _ -> { saveFilters(); onClose(); }).width((Button.DEFAULT_WIDTH * 2) + 10).build(), 2);
        let __this_w = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        let done = button::builder(mcrust::net::minecraft::network::chat::common_components::gui_done().get_string(), OnPress::Lambda(PressLambda::new(move |_screen, _index| {
            let Some(__this) = __this_w.get().upgrade() else { return; };
            ChatRuleLocationConfigScreen::save_filters(&__this);
            ChatRuleLocationConfigScreen::on_close(&__this);
        })))
        .width((mcrust::net::minecraft::client::gui::components::button::DEFAULT_WIDTH * 2) + 10)
        .build();
        ChatRuleLocationConfigScreen::add_to_grid(__this, &mut grid_widget, &mut adder, done, 2);
        //   java: gridWidget.arrangeElements(); FrameLayout.centerInRectangle(gridWidget, 0, this.height - 64, this.width, 64);
        grid_widget.arrange_elements();
        frame_layout::center_in_rectangle(&mut *compat::LayoutBack::grid(&mut grid_widget), 0, height - 64, width, 64);
        //   java: gridWidget.visitWidgets(this::addRenderableWidget);
        let mut __vw = Vec::new();
        LayoutElement::Grid(grid_widget.clone()).visit_widgets(&mut __vw);
        compat::write(__this, |__t| {
            for (__i, __x, __y) in __vw {
                __t.widget_mut(__i).set_position(__x, __y);
                __t.mark_renderable(__i);
            }
        });
    }

    /// java: public void saveFilters()
    pub fn save_filters(__this: &compat::Shared<Self>) {
        let enabled_locations: std::collections::HashSet<Location> = compat::read(&compat::read(__this, |__t| __t.enabled_locations.clone()), |l| l.iter().cloned().collect());
        ChatRule::set_valid_locations(&compat::read(__this, |__t| __t.chat_rule.clone()), enabled_locations);
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

    /// `addRenderableOnly`'s title (see `ChatRuleLocationConfigScreen::extract_render_state`)
    fn extract_render_state(&mut self, screen: &mcrust::net::minecraft::client::gui::screens::screen::Screen, context: &mut ScreenRenderContext<'_>, _mouse_x: i32, _mouse_y: i32, _a: f32) {
        self.lend_ref(screen, |__h| ChatRuleLocationConfigScreen::extract_render_state(__h, context))
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
