//! Ported from de.hysky.skyblocker.skyblock.searchoverlay.OverlayScreen by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::searchoverlay::search_over_manager;
use crate::de::hysky::skyblocker::skyblock::searchoverlay::search_over_manager::SearchLocation;
use crate::de::hysky::skyblocker::skyblock::searchoverlay::search_over_manager::SearchOverManager;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
use crate::de::hysky::skyblocker::utils::render::texture::fallbacked_texture;
use crate::de::hysky::skyblocker::utils::render::texture::fallbacked_texture::FallbackedTexture;
use mcrust::mcrust::modding::screens::ModScreen;
use mcrust::mcrust::modding::screens::ScreenRenderContext;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::abstract_widget::PressLambda;
use mcrust::net::minecraft::client::gui::components::button;
use mcrust::net::minecraft::client::gui::components::edit_box::EditBox;
use mcrust::net::minecraft::client::gui::components::edit_box::Responder;
use mcrust::net::minecraft::client::gui::components::edit_box::ResponderLambda;
use mcrust::net::minecraft::client::gui::components::tooltip::Tooltip;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::layouts::grid_layout::GridLayout;
use mcrust::net::minecraft::client::gui::layouts::grid_layout::RowHelper;
use mcrust::net::minecraft::client::gui::layouts::layout_element::LayoutElement;
use mcrust::net::minecraft::client::gui::layouts::linear_layout::LinearLayout;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::client::input::mouse_button_info::MouseButtonInfo;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: protected static final Identifier SEARCH_ICON_TEXTURE = Identifier.withDefaultNamespace("icon/search");
pub static SEARCH_ICON_TEXTURE: compat::MainThread<Identifier> = compat::MainThread::new(|| Identifier::with_default_namespace("icon/search"));
/// java: protected static final Identifier DELETE_ICON_TEXTURE = Identifier.withDefaultNamespace("textures/gui/sprites/pending_invite/reject.png");
pub static DELETE_ICON_TEXTURE: compat::MainThread<Identifier> = compat::MainThread::new(|| Identifier::with_default_namespace("textures/gui/sprites/pending_invite/reject.png"));
/// java: private static final FallbackedTexture<Identifier> BACKGROUND_TEXTURE = FallbackedTexture.ofGuiSprite( SkyblockerMod.id("search_overlay/background"), Identifier.withDefaultNamespace("social_interactio
pub static BACKGROUND_TEXTURE: compat::MainThread<compat::Shared<dyn FallbackedTexture<Identifier>>> = compat::MainThread::new(|| fallbacked_texture::of_gui_sprite(&crate::entry::skyblocker_mod::id("search_overlay/background"), &Identifier::with_default_namespace("social_interactions/background")));
/// java: private static final int rowHeight = 20;
pub const ROW_HEIGHT: i32 = 20;
/// java: private static final int specialButtonSize = rowHeight;
pub static SPECIAL_BUTTON_SIZE: LazyLock<i32> = LazyLock::new(|| ROW_HEIGHT);

/// java: public class OverlayScreen extends Screen
pub struct OverlayScreen {
    /// java: extends net.minecraft.client.gui.screens.Screen
    pub base: mcrust::net::minecraft::client::gui::screens::screen::Screen,
    /// java: private EditBox searchField;
    pub search_field: Option<compat::WidgetRef>,
    /// java: private Button finishedButton;
    pub finished_button: Option<compat::WidgetRef>,
    /// java: private Button maxPetButton;
    pub max_pet_button: Option<compat::WidgetRef>,
    /// java: private Button dungeonStarButton;
    pub dungeon_star_button: Option<compat::WidgetRef>,
    /// java: private Button[] suggestionButtons; (references to the screen's widgets; elements null until built)
    pub suggestion_buttons: Vec<Option<compat::WidgetRef>>,
    /// java: private Button[] historyButtons;
    pub history_buttons: Vec<Option<compat::WidgetRef>>,
    /// java: private Button[] deleteButtons;
    pub delete_buttons: Vec<Option<compat::WidgetRef>>,
    /// the screen's widgets as this object last saw them (see `compat::WidgetCache`)
    pub __widgets: compat::WidgetCache,
}

impl std::ops::Deref for OverlayScreen {
    type Target = mcrust::net::minecraft::client::gui::screens::screen::Screen;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for OverlayScreen {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl OverlayScreen {
    /// java: public OverlayScreen()
    pub fn new() -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            base: mcrust::net::minecraft::client::gui::screens::screen::Screen::new(component::empty().get_string()),
            __widgets: Default::default(),
            search_field: None,
            finished_button: None,
            max_pet_button: None,
            dungeon_star_button: None,
            suggestion_buttons: Default::default(),
            history_buttons: Default::default(),
            delete_buttons: Default::default(),
        });
        __this
    }

    /// java: @Override protected void init()
    pub fn init(__this: &compat::Shared<Self>) {
        ();
        let mut row_width: i32 = ((((compat::read(__this, |__t| __t.width) as f64) * 0.4)) as i32);
        let mut start_x: i32 = ((((compat::read(__this, |__t| __t.width) as f64) * 0.5)) as i32).wrapping_sub(row_width.wrapping_div(2));
        let mut start_y: i32 = ((((((((compat::read(__this, |__t| __t.height) as f64) * 0.5)) as i32) as f64) - (((ROW_HEIGHT as f64) * (((1i32.wrapping_add(SkyblockerConfigManager::get().ui_and_visuals.search_overlay.max_suggestions) as f64) + 0.75) + (SkyblockerConfigManager::get().ui_and_visuals.search_overlay.history_length as f64))) / (2 as f64)))) as i32);
        // java: this.searchField = new EditBox(font, startX, startY, rowWidth - rowHeight, rowHeight, Component.translatable("gui.recipebook.search_hint"));
        { let __v = Some(compat::WidgetRef::value(compat::edit_box(start_x, start_y, row_width.wrapping_sub(ROW_HEIGHT), ROW_HEIGHT, component::translatable("gui.recipebook.search_hint")))); compat::write(__this, |__t| __t.search_field = __v); }
        // java: searchField.setValue(SearchOverManager.search); searchField.setResponder(SearchOverManager::updateSearch); searchField.setMaxLength(30);
        let __search = compat::get_state(&search_over_manager::SEARCH);
        OverlayScreen::with_search_field(__this, |__w| compat::set_edit_box_value(__w, &__search));
        OverlayScreen::with_search_field(__this, |__w| compat::with_edit_box(__w, |__e| __e.set_responder(Responder::Lambda(ResponderLambda::new(move |__a0| { SearchOverManager::update_search(__a0.to_string()); })))));
        OverlayScreen::with_search_field(__this, |__w| compat::with_edit_box(__w, |__e| __e.set_max_length(30)));
        // (the screen takes the search field first: Java's `addRenderableWidget(searchField)` comes before the layouts'
        // widgets, and a layout here holds its widgets by their index in the screen's list)
        compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.search_field.as_mut().unwrap(); __s.__widgets.add(&mut __s.base, __r, false) });
        let __this_w1 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        { let __v = (Some((button::builder(component::literal("").get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { let Some(__this_1) = __this_w1.get().upgrade() else { return Default::default(); }; compat::lend_screen(&__this_1, __screen, || { let mut __pidx = compat::WidgetRef::Screen(__index); { OverlayScreen::on_close((&__this_1)); } }) })))).pos((start_x.wrapping_add(row_width)).wrapping_sub(ROW_HEIGHT), start_y).size(*SPECIAL_BUTTON_SIZE, *SPECIAL_BUTTON_SIZE).build())).map(compat::WidgetRef::value); compat::write(__this, |__t| { let __s = &mut *__t; __s.finished_button = __v; }); }
        let mut total_suggestions: i32 = SkyblockerConfigManager::get().ui_and_visuals.search_overlay.max_suggestions;
        // java: this.suggestionButtons = new Button[totalSuggestions];
        { let __v = vec![None; (total_suggestions).max(0) as usize]; compat::write(__this, |__t| __t.suggestion_buttons = __v); }
        // java: LinearLayout suggestionLayoutWidget = new LinearLayout(startX, startY + rowHeight, LinearLayout.Orientation.VERTICAL);
        let mut suggestion_layout_widget: LinearLayout = LinearLayout::new(start_x, start_y.wrapping_add(ROW_HEIGHT), mcrust::net::minecraft::client::gui::layouts::linear_layout::Orientation::Vertical);
        for i in 0..total_suggestions {
            let __this_w2 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
            let __button = (button::builder(component::empty().get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { let Some(__this_2) = __this_w2.get().upgrade() else { return Default::default(); }; compat::lend_screen(&__this_2, __screen, || { let mut __pidx = compat::WidgetRef::Screen(__index); {
                SearchOverManager::update_search((compat::write((&__this_2), |__t| { let __s = &mut *__t; let __r = &mut __pidx; __s.__widgets.with(&mut __s.base, __r, |__w| compat::component_of(__w.get_message_component())) })).get_string());
                OverlayScreen::on_close((&__this_2));
            } }) })))).size(row_width, ROW_HEIGHT).build();
            compat::write(__this, |__t| __t.suggestion_buttons[(i) as usize] = Some(compat::WidgetRef::value(__button)));
            // java: suggestionLayoutWidget.addChild(suggestionButtons[i]);
            suggestion_layout_widget.add_child(OverlayScreen::button_layout_element(__this, ButtonArray::Suggestion, i));
            OverlayScreen::with_button(__this, ButtonArray::Suggestion, i, |__w| __w.visible = false);
        }
        let mut history_offset: i32 = ((((ROW_HEIGHT as f64) * ((total_suggestions as f64) + 1.75))) as i32);
        let mut history_length: i32 = SkyblockerConfigManager::get().ui_and_visuals.search_overlay.history_length;
        { let __v = vec![None; (history_length).max(0) as usize]; compat::write(__this, |__t| __t.history_buttons = __v); }
        { let __v = vec![None; (history_length).max(0) as usize]; compat::write(__this, |__t| __t.delete_buttons = __v); }
        let mut history_grid_widget: GridLayout = GridLayout::new(start_x, start_y.wrapping_add(history_offset));
        let mut history_adder: RowHelper = history_grid_widget.create_row_helper(2);
        for i in 0..history_length {
            let __this_w3 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
            let __button = (button::builder(component::empty().get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { let Some(__this_3) = __this_w3.get().upgrade() else { return Default::default(); }; compat::lend_screen(&__this_3, __screen, || { let mut __pidx = compat::WidgetRef::Screen(__index); {
                compat::set_state(&search_over_manager::SEARCH, (compat::write((&__this_3), |__t| { let __s = &mut *__t; let __r = &mut __pidx; __s.__widgets.with(&mut __s.base, __r, |__w| compat::component_of(__w.get_message_component())) })).get_string());
                SearchOverManager::update_search((compat::write((&__this_3), |__t| { let __s = &mut *__t; let __r = &mut __pidx; __s.__widgets.with(&mut __s.base, __r, |__w| compat::component_of(__w.get_message_component())) })).get_string());
                OverlayScreen::on_close((&__this_3));
            } }) })))).size(row_width.wrapping_sub(ROW_HEIGHT), ROW_HEIGHT).build();
            compat::write(__this, |__t| __t.history_buttons[(i) as usize] = Some(compat::WidgetRef::value(__button)));
            OverlayScreen::with_button(__this, ButtonArray::History, i, |__w| __w.visible = false);
            // java: historyAdder.addChild(historyButtons[i]);
            history_adder.add_child(&mut history_grid_widget, OverlayScreen::button_layout_element(__this, ButtonArray::History, i)).unwrap();
            let mut slot_id: i32 = i;
            let __this_w4 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
            let __button = (button::builder(component::empty().get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { let Some(__this_4) = __this_w4.get().upgrade() else { return Default::default(); }; compat::lend_screen(&__this_4, __screen, || { let mut __pidx = compat::WidgetRef::Screen(__index); { OverlayScreen::remove_history_item((&__this_4), slot_id); } }) })))).size(*SPECIAL_BUTTON_SIZE, *SPECIAL_BUTTON_SIZE).tooltip_text(Some(Tooltip::create(component::translatable("skyblocker.config.general.searchOverlay.deleteTooltip").get_string()).message.clone())).build();
            compat::write(__this, |__t| __t.delete_buttons[(i) as usize] = Some(compat::WidgetRef::value(__button)));
            OverlayScreen::with_button(__this, ButtonArray::Delete, i, |__w| __w.visible = false);
            // java: historyAdder.addChild(deleteButtons[i]);
            history_adder.add_child(&mut history_grid_widget, OverlayScreen::button_layout_element(__this, ButtonArray::Delete, i)).unwrap();
        }
        OverlayScreen::update_history_buttons(__this);
        if compat::get_state(&search_over_manager::LOCATION) == SearchLocation::Auction {
            let __this_w5 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
            { let __v = (Some((button::builder(component::literal("").get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { let Some(__this_5) = __this_w5.get().upgrade() else { return Default::default(); }; compat::lend_screen(&__this_5, __screen, || { let mut __pidx = compat::WidgetRef::Screen(__index); {
                compat::set_state(&search_over_manager::MAX_PET_LEVEL, !(compat::get_state(&search_over_manager::MAX_PET_LEVEL)));
                OverlayScreen::update_max_pet_text((&__this_5));
            } }) })))).tooltip_text(Some((Tooltip::create((component::translatable("skyblocker.config.general.searchOverlay.maxPet.@Tooltip")).get_string())).message.clone())).pos(start_x, (start_y.wrapping_sub(ROW_HEIGHT)).wrapping_sub(8)).size(row_width.wrapping_div(2), ROW_HEIGHT).build())).map(compat::WidgetRef::value); compat::write(__this, |__t| { let __s = &mut *__t; __s.max_pet_button = __v; }); }
            OverlayScreen::update_max_pet_text(__this);
            let __this_w6 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
            { let __v = (Some((((button::builder((component::literal("✪")).get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { let Some(__this_6) = __this_w6.get().upgrade() else { return Default::default(); }; compat::lend_screen(&__this_6, __screen, || { let mut __pidx = compat::WidgetRef::Screen(__index); { OverlayScreen::update_stars((&__this_6)); } }) })))).tooltip_text(Some(Tooltip::create(component::translatable("skyblocker.config.general.searchOverlay.starsTooltip").get_string()).message.clone())).pos(start_x.wrapping_add((((row_width as f64) * 0.5)) as i32), (start_y.wrapping_sub(ROW_HEIGHT)).wrapping_sub(8))).size(row_width.wrapping_div(2), ROW_HEIGHT)).build())).map(compat::WidgetRef::value); compat::write(__this, |__t| { let __s = &mut *__t; __s.dungeon_star_button = __v; }); }
            OverlayScreen::update_stars(__this);
        }
        { suggestion_layout_widget.arrange_elements(); let mut __vw = Vec::new(); LayoutElement::Grid(suggestion_layout_widget.wrapped.clone()).visit_widgets(&mut __vw); for (__i, __x, __y) in __vw { compat::write(__this, |__t| __t.widget_mut(__i).set_position(__x, __y)); } };
        { history_grid_widget.arrange_elements(); let mut __vw = Vec::new(); LayoutElement::Grid(history_grid_widget.clone()).visit_widgets(&mut __vw); for (__i, __x, __y) in __vw { compat::write(__this, |__t| __t.widget_mut(__i).set_position(__x, __y)); } };
        // java: addRenderableWidget(searchField); suggestionLayoutWidget.visitWidgets(this::addRenderableWidget); historyGridWidget.visitWidgets(..);
        compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.search_field.as_mut().unwrap(); __s.__widgets.add(&mut __s.base, __r, true) });
        { let mut __vw = Vec::new(); LayoutElement::Grid(suggestion_layout_widget.wrapped.clone()).visit_widgets(&mut __vw); for (__i, _, _) in __vw { compat::write(__this, |__t| __t.mark_renderable(__i)); } };
        { let mut __vw = Vec::new(); LayoutElement::Grid(history_grid_widget.clone()).visit_widgets(&mut __vw); for (__i, _, _) in __vw { compat::write(__this, |__t| __t.mark_renderable(__i)); } };
        compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.finished_button.as_mut().unwrap(); __s.__widgets.add(&mut __s.base, __r, true) });
        if compat::get_state(&search_over_manager::LOCATION) == SearchLocation::Auction {
            compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.max_pet_button.as_mut().unwrap(); __s.__widgets.add(&mut __s.base, __r, true) });
            compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.dungeon_star_button.as_mut().unwrap(); __s.__widgets.add(&mut __s.base, __r, true) });
        }
        // java: this.setInitialFocus(searchField);
        if let Some(compat::WidgetRef::Screen(__index)) = compat::read(__this, |__t| __t.search_field.clone()) {
            __this.lock().borrow_mut().set_initial_focus_on(__index);
        }
    }

    /// One access through the search field reference (`searchField.x(..)`).
    fn with_search_field<R>(__this: &compat::Shared<Self>, f: impl FnOnce(&mut AbstractWidget) -> R) -> R {
        compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.search_field.as_mut().expect("NullPointerException"); __s.__widgets.with(&mut __s.base, __r, f) })
    }

    /// One access through `buttons[i]` of one of the three button arrays (`suggestionButtons[i].visible = false`).
    fn with_button<R>(__this: &compat::Shared<Self>, which: ButtonArray, i: i32, f: impl FnOnce(&mut AbstractWidget) -> R) -> R {
        compat::write(__this, |__t| {
            let __s = &mut *__t;
            let __array = match which {
                ButtonArray::Suggestion => &mut __s.suggestion_buttons,
                ButtonArray::History => &mut __s.history_buttons,
                ButtonArray::Delete => &mut __s.delete_buttons,
            };
            let __r = __array[(i) as usize].as_mut().expect("NullPointerException");
            __s.__widgets.with(&mut __s.base, __r, f)
        })
    }

    /// `layout.addChild(buttons[i])`: the button joins the screen's list (a layout holds it by index).
    fn button_layout_element(__this: &compat::Shared<Self>, which: ButtonArray, i: i32) -> LayoutElement {
        let (__i, __ww, __wh) = compat::write(__this, |__t| {
            let __s = &mut *__t;
            let __array = match which {
                ButtonArray::Suggestion => &mut __s.suggestion_buttons,
                ButtonArray::History => &mut __s.history_buttons,
                ButtonArray::Delete => &mut __s.delete_buttons,
            };
            let __r = __array[(i) as usize].as_mut().expect("NullPointerException");
            __s.__widgets.add_to_layout(&mut __s.base, __r)
        });
        LayoutElement::widget(__i, __ww, __wh)
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    pub fn mouse_clicked(__this: &compat::Shared<Self>, click: &MouseButtonEvent, mut doubled: bool) -> bool {
        if compat::write(__this, |__t| { let __s = &mut *__t; match __s.dungeon_star_button.as_mut() { Some(__r) => Some(__s.__widgets.with(&mut __s.base, __r, |__w| __w.clone())), None => None } }).is_some() && compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.dungeon_star_button.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.is_hovered()) }) {
            let mut actual_text_width: f64 = ((compat::font().width_of_cached(&mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.dungeon_star_button.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| compat::component_of(__w.get_message_component())) })))) as f64) as f64;
            let mut text_offset: f64 = (((compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.dungeon_star_button.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.get_width()) }) as f64) - actual_text_width) / (2 as f64));
            let mut offset: f64 = (click.x() - ((compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.dungeon_star_button.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.get_x()) }) as f64) + text_offset));
            let mut star_count: i32 = ((((offset / actual_text_width) * (10 as f64))) as i32);
            star_count = (star_count.wrapping_add(1)).clamp(0, 10);
            if star_count == compat::get_state(&search_over_manager::DUNGEON_STARS) {
                compat::set_state(&search_over_manager::DUNGEON_STARS, 0);
            } else {
                compat::set_state(&search_over_manager::DUNGEON_STARS, star_count);
            }
        }
        return false;
    }

    /// java: private void updateMaxPetText()
    pub fn update_max_pet_text(__this: &compat::Shared<Self>) {
        if compat::get_state(&search_over_manager::MAX_PET_LEVEL) {
            let __wa1 = (component::translatable("skyblocker.config.general.searchOverlay.maxPet").append(component::literal(" ✔"))).with_style_format(ChatFormatting::Green);
            compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.max_pet_button.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.set_message_text(mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(__wa1.clone()))) });
        } else {
            let __wa2 = (component::translatable("skyblocker.config.general.searchOverlay.maxPet").append(component::literal(" ❌"))).with_style_format(ChatFormatting::Red);
            compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.max_pet_button.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.set_message_text(mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(__wa2.clone()))) });
        }
    }

    /// java: private void updateStars()
    pub fn update_stars(__this: &compat::Shared<Self>) {
        let mut stars: MutableComponent = component::empty();
        for i in 0..compat::get_state(&search_over_manager::DUNGEON_STARS) {
            stars = stars.append((component::literal("✪")).with_style_format((if i < 5 { ChatFormatting::Yellow } else { ChatFormatting::Red })));
        }
        for i in compat::get_state(&search_over_manager::DUNGEON_STARS)..10 {
            stars = stars.append(component::literal("✪"));
        }
        let __wa1 = stars.clone();
        compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.dungeon_star_button.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.set_message_text(mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(__wa1.clone()))) });
    }

    /// java: private void removeHistoryItem(int slotId)
    pub fn remove_history_item(__this: &compat::Shared<Self>, mut slot_id: i32) {
        SearchOverManager::remove_history_item(slot_id);
        OverlayScreen::update_history_buttons(__this);
    }

    /// java: private void updateHistoryButtons()
    pub fn update_history_buttons(__this: &compat::Shared<Self>) {
        let mut history_length: i32 = SkyblockerConfigManager::get().ui_and_visuals.search_overlay.history_length;
        for i in 0..history_length {
            let mut text: String = SearchOverManager::get_history(i);
            if text.is_empty() {
                OverlayScreen::with_button(__this, ButtonArray::History, i, |__w| __w.visible = false);
                OverlayScreen::with_button(__this, ButtonArray::Delete, i, |__w| __w.visible = false);
                continue;
            }
            { let (__q0,) = (mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::literal(&text)),); OverlayScreen::with_button(__this, ButtonArray::History, i, |__w| __w.set_message_text(__q0)) };
            OverlayScreen::with_button(__this, ButtonArray::History, i, |__w| __w.visible = true);
            OverlayScreen::with_button(__this, ButtonArray::Delete, i, |__w| __w.visible = true);
        }
    }

    /// java: @Override public void extractBackground(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_background(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        // TODO(fabric2seer): `Screen.extractBackground` has no Rust method in the port (looked for `extract_background` in Screen and its supertypes)
        // rust: __this.lock().borrow_mut().base.extract_background(graphics, mouse_x, mouse_y, a);
        let mut max_height: i32 = ROW_HEIGHT.wrapping_mul((1i32.wrapping_add(__this.lock().borrow().suggestion_buttons.len() as i32)).wrapping_add(__this.lock().borrow().history_buttons.len() as i32));
        if (__this.lock().borrow().history_buttons.len() as i32) > 0 {
            max_height = max_height.wrapping_add((((ROW_HEIGHT as f64) * 0.75)) as i32);
        }
        // TODO(fabric2seer): `get` is not a method of FallbackedTexture
        //   java: graphics.blitSprite(RenderPipelines.GUI_TEXTURED, BACKGROUND_TEXTURE.get(), searchField.getX() - 8, searchField.getY() - 8, (int) (this.width * 0.4) + 16, maxHeight + 16);
    }

    /// java: @Override public void extractRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_render_state(__this: &compat::Shared<Self>, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        ();
        let mut render_offset: i32 = (ROW_HEIGHT.wrapping_sub(16)).wrapping_div(2);
        gui_graphics_extractor::blit_sprite_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), SEARCH_ICON_TEXTURE.get_mut(), (compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.finished_button.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.get_x()) })).wrapping_add(render_offset), (compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.finished_button.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.get_y()) })).wrapping_add(render_offset), 16, 16);
        // java: if (historyButtons.length > 0 && historyButtons[0] != null)
        if (__this.lock().borrow().history_buttons.len() as i32) > 0 && compat::read(__this, |__t| __t.history_buttons[(0) as usize].is_some()) {
            let (__hx, __hy) = OverlayScreen::with_button(__this, ButtonArray::History, 0, |__w| (__w.get_x(), __w.get_y()));
            gui_graphics_extractor::draw_formatted_char_sequence_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::translatable("skyblocker.config.general.searchOverlay.historyLabel")), __hx.wrapping_add(render_offset), __hy.wrapping_sub(ROW_HEIGHT.wrapping_div(2)), 0xFFFFFFFFu32 as i32, true);
        }
        for i in 0..(__this.lock().borrow().suggestion_buttons.len() as i32) {
            let __button = OverlayScreen::with_button(__this, ButtonArray::Suggestion, i, |__w| __w.clone());
            OverlayScreen::extract_item_and_tooltip(__this, graphics, mouse_x, mouse_y, &SearchOverManager::get_suggestion_id(i), &__button, render_offset);
        }
        for i in 0..(__this.lock().borrow().history_buttons.len() as i32) {
            let __button = OverlayScreen::with_button(__this, ButtonArray::History, i, |__w| __w.clone());
            OverlayScreen::extract_item_and_tooltip(__this, graphics, mouse_x, mouse_y, &SearchOverManager::get_history_id(i), &__button, render_offset);
        }
        for __i in 0..(__this.lock().borrow().delete_buttons.len() as i32) {
            let delete_button = OverlayScreen::with_button(__this, ButtonArray::Delete, __i, |__w| __w.clone());
            if !delete_button.visible {
                break;
            }
            gui_graphics_extractor::blit_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), DELETE_ICON_TEXTURE.get_mut(), delete_button.get_x().wrapping_add(render_offset), delete_button.get_y().wrapping_add(render_offset), ((0) as f32) as f32, ((0) as f32) as f32, 16, 16, 16, 16);
        }
    }

    /// java: private void extractItemAndTooltip(GuiGraphicsExtractor graphics, int mouseX, int mouseY, String id, Button button, int renderOffset)
    pub fn extract_item_and_tooltip(__this: &compat::Shared<Self>, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, id: &str, button: &AbstractWidget, mut render_offset: i32) {
        if id.is_empty() {
            return;
        }
        // TODO(fabric2seer): `Screen.getItemStack` has no Rust method in the port (looked for `get_item_stack` in Screen and JavaSrc)
        // rust: let mut flexible: Option<compat::Shared<FlexibleItemStack>> = Some(__this.lock().borrow().get_item_stack(id));
        let mut flexible: compat::Shared<FlexibleItemStack> = todo!("fabric2seer: the initialiser of `flexible` (TODO above)");
        if false /* java: flexible == null: never null here */ {
            return;
        }
        let mut item: mcrust::net::minecraft::world::item::item_stack::ItemStack = FlexibleItemStack::get_stack_or_throw(&flexible);
        graphics.item(&item, button.get_x().wrapping_add(render_offset), button.get_y().wrapping_add(render_offset));
        if button.is_mouse_over((mouse_x) as f64, (mouse_y) as f64) {
            // TODO(fabric2seer): `GuiGraphicsExtractor.setTooltipForNextFrame` has no Rust method in the port (looked for `set_tooltip_for_next_frame` in GuiGraphicsExtractor and its supertypes)
            // rust: graphics.set_tooltip_for_next_frame(&compat::font(), &item, mouse_x, mouse_y);
        }
    }

    /// java: @Override public final void tick()
    pub fn tick(__this: &compat::Shared<Self>) {
        ();
        for i in 0..SkyblockerConfigManager::get().ui_and_visuals.search_overlay.max_suggestions {
            let mut text: String = SearchOverManager::get_suggestion(i);
            if !text.is_empty() {
                OverlayScreen::with_button(__this, ButtonArray::Suggestion, i, |__w| __w.visible = true);
                let mut is_new_text: bool = !(text == OverlayScreen::with_button(__this, ButtonArray::Suggestion, i, |__w| compat::component_of(__w.get_message_component()).get_string()));
                if !is_new_text {
                    continue;
                }
                { let (__q0,) = (mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::literal(&text)),); OverlayScreen::with_button(__this, ButtonArray::Suggestion, i, |__w| __w.set_message_text(__q0)) };
            } else {
                OverlayScreen::with_button(__this, ButtonArray::Suggestion, i, |__w| __w.visible = false);
            }
        }
    }

    /// java: @Override public boolean keyPressed(KeyEvent input)
    pub fn key_pressed(__this: &compat::Shared<Self>, input: &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        // java: searchField.canConsumeInput() - `isActive() && isFocused() && isEditable()`, the first two the widget's
        if input.is_confirmation() && OverlayScreen::with_search_field(__this, |__w| { let (__active, __focused) = (__w.visible && __w.active, __w.is_focused()); compat::with_edit_box(__w, |__e| __e.can_consume_input(__active, __focused)).unwrap_or(false) }) {
            OverlayScreen::on_close(__this);
            return true;
        }
        if input.is_escape() {
            compat::set_state(&search_over_manager::SEARCH, "".to_string());
            OverlayScreen::on_close(__this);
            return true;
        }
        return (if input.is_escape() && true { OverlayScreen::on_close(__this); true } else { false });
    }

    /// java: @Override public boolean isPauseScreen()
    pub fn is_pause_screen(__this: &compat::Shared<Self>) -> bool {
        return false;
    }

    /// java: @Override public void onClose()
    pub fn on_close(__this: &compat::Shared<Self>) {
        debug_assert!(mcrust::mcrust::modding::game::with_player(|_| ()).is_some());
        SearchOverManager::push_search();
        mcrust::mcrust::modding::game::set_screen_null();
    }

}

/// OverlayScreen's `Screen` half (`super`'s fields and widget tree), lent by the game for each call
impl compat::ScreenHalf for OverlayScreen {
    fn screen_half(&mut self) -> &mut mcrust::net::minecraft::client::gui::screens::screen::Screen {
        &mut self.base
    }
    fn screen_parts(&mut self) -> (&mut mcrust::net::minecraft::client::gui::screens::screen::Screen, Option<&mut compat::WidgetCache>) {
        let __s = self;
        (&mut __s.base, Some(&mut __s.__widgets))
    }
}

/// OverlayScreen as the game's screen: `game::set_screen(Box::new(compat::ScreenAdapter::new(..)))`
impl ModScreen for compat::ScreenAdapter<OverlayScreen> {
    fn get_title(&self) -> String {
        self.title()
    }

    /// java: @Override protected void init()
    fn init(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        self.lend(screen, |__h| OverlayScreen::init(__h))
    }

    /// java: @Override public final void tick()
    fn tick(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        self.lend(screen, |__h| OverlayScreen::tick(__h))
    }

    /// java: @Override public void extractRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_render_state(&mut self, screen: &mcrust::net::minecraft::client::gui::screens::screen::Screen, context: &mut ScreenRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        self.lend_ref(screen, |__h| OverlayScreen::extract_render_state(__h, &mut context.graphics, mouse_x, mouse_y, a))
    }

    /// java: @Override public boolean keyPressed(KeyEvent input)
    fn key_pressed(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, event: &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        self.lend(screen, |__h| OverlayScreen::key_pressed(__h, event))
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    fn mouse_clicked(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, x: f64, y: f64, button: i32) -> bool {
        self.lend(screen, |__h| OverlayScreen::mouse_clicked(__h, &MouseButtonEvent::new(x, y, MouseButtonInfo::new(button, 0)), false))
    }

    /// java: @Override public void onClose()
    fn on_close(&mut self) {
        self.call(|__h| OverlayScreen::on_close(__h))
    }

    /// java: @Override public boolean isPauseScreen()
    fn is_pause_screen(&self) -> bool {
        self.call(|__h| OverlayScreen::is_pause_screen(__h))
    }

}

// NOTE(fabric2seer): extractBackground, extractRenderState, init, isPauseScreen, keyPressed, mouseClicked, onClose, tick override(s) of Screen - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them

/// Which of the screen's three button arrays a reference is in.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ButtonArray {
    Suggestion,
    History,
    Delete,
}
