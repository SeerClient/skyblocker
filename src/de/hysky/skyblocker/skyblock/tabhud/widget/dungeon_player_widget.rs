//! Ported from de.hysky.skyblocker.skyblock.tabhud.widget.DungeonPlayerWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::dungeon::dungeon_class::DungeonClass;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::dungeon_player_manager;
use crate::de::hysky::skyblocker::skyblock::tabhud::config::option_widget_collector::OptionWidgetCollector;
use crate::de::hysky::skyblocker::skyblock::tabhud::util::ico;
use crate::de::hysky::skyblocker::skyblock::tabhud::util::player_list_manager::PlayerListManager;
use crate::de::hysky::skyblocker::skyblock::tabhud::util::player_list_manager::Widget;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::element::Element;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::element_collector::ElementCollector;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::elements::Elements;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::player_element::PlayerElement;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element_based_widget::ElementBasedWidget;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element_based_widget::ElementBasedWidgetBase;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::hud_widget::HudWidget;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::hud_widget::HudWidgetBase;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::tab_hud_widget::TabHudWidget;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::tab_hud_widget::TabHudWidgetBase;
use crate::de::hysky::skyblocker::utils::json_value_input::JsonValueInput;
use crate::de::hysky::skyblocker::utils::json_value_output::JsonValueOutput;
use crate::de::hysky::skyblocker::utils::location::Location;
use crate::de::hysky::skyblocker::utils::roman_numerals::RomanNumerals;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::multiplayer::player_info::PlayerInfo;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final MutableComponent TITLE = Component.literal("Player").withStyle(ChatFormatting.AQUA, ChatFormatting.BOLD);
pub static TITLE: compat::MainThread<MutableComponent> = compat::MainThread::new(|| component::literal("Player").with_style_formats(&[ChatFormatting::Aqua, ChatFormatting::Bold]));
/// java: private static final List<String> MSGS = List.of("???", "PRESS A TO JOIN", "Invite a friend!", "But nobody came.", "More is better!");
pub static MSGS: LazyLock<Vec<String>> = LazyLock::new(|| vec!["???".to_string(), "PRESS A TO JOIN".to_string(), "Invite a friend!".to_string(), "But nobody came.".to_string(), "More is better!".to_string()]);

/// java: public class DungeonPlayerWidget extends TabHudWidget
#[derive(Clone)]
pub struct DungeonPlayerWidget {
    /// java: extends TabHudWidget
    pub base: TabHudWidgetBase,
    /// java: private final int player;
    pub player: i32,
}

impl std::ops::Deref for DungeonPlayerWidget {
    type Target = TabHudWidgetBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for DungeonPlayerWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl DungeonPlayerWidget {
    /// java: public DungeonPlayerWidget(int player)
    pub fn new(mut player: i32) -> compat::Shared<Self> {
        // NOTE(fabric2seer): `player` is read by `updateContent`, which TabHudWidget's constructor calls before this constructor assigns it: Java sees its default there (0 / false / null), and so does this port
        let (_, __this) = TabHudWidgetBase::new3_in(format!("Dungeon Player {}", player), TITLE.get_mut().clone(), mcrust::net::minecraft::network::chat::text_color::TextColor::AQUA.get_value(), Location::Dungeon, move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                player: 0,
            });
            ((__h.clone() as compat::Shared<dyn TabHudWidget>), __h)
        });
        { let __v = player; compat::write(&__this, |__t| __t.player = __v); }
        __this
    }

    /// java: private void addPlayerNameAndClass(int index)
    pub fn add_player_name_and_class(__this: &compat::Shared<Self>, mut index: i32) {
        let Some(mut m) = PlayerListManager::regex_at(index, &dungeon_player_manager::PLAYER_TAB_PATTERN) else {
            { let (__a0,) = (Elements::icon_text_component(),); __this.lock().borrow_mut().add_element(__a0) };
            { let (__a0,) = (Elements::icon_text_component(),); __this.lock().borrow_mut().add_element(__a0) };
            return;
        };
        let mut clazz: String = m.group_named("class").unwrap();
        let mut level: Option<String> = m.group_named("level");
        let mut dungeon_class: DungeonClass = DungeonClass::from(&clazz);
        if RomanNumerals::is_valid_roman_numeral(level.clone()) {
            level = Some(RomanNumerals::roman_to_decimal(level.clone()).to_string());
        }
        let mut player_info: PlayerInfo = PlayerListManager::get_raw(index);
        // TODO(fabric2seer): the mod keeps the game's `PlayerInfo` (Java holds a reference to the live object); the port's `PlayerInfo` is not a value to copy and the port lends the mod no handle to it
        // rust: { let (__a0,) = ((PlayerElement::new_3(player_info.clone(), DungeonPlayerWidget::remove_dungeon_class(__this, player_info.get_tab_list_display_name()), true) as compat::Shared<dyn Element>),); __this.lock().borrow_mut().add_element(__a0) };
        if let Some(level) = &level {
        } else {
            { let (__a0,) = (Elements::icon_text_component_2(Some(ico::SKULL.get_mut().clone()), Some(component::literal("Player is dead").with_style_format(ChatFormatting::Red))),); __this.lock().borrow_mut().add_element(__a0) };
            return;
        }
        { let (__a0,) = (Elements::icon_text_component_2(Some(dungeon_class.icon()), Some(component::literal("Class: ").append((component::literal(&format!("{} {}", clazz, compat::opt_str(&level)))).with_color(dungeon_class.color())))),); __this.lock().borrow_mut().add_element(__a0) };
    }

    /// java: private @Nullable Component removeDungeonClass(@Nullable Component name)
    pub fn remove_dungeon_class(__this: &compat::Shared<Self>, mut name: Option<MutableComponent>) -> Option<MutableComponent> {
        if (name.is_none() || !(((name.as_ref().unwrap().get_siblings()[name.as_ref().unwrap().get_siblings().len() - 1]).get_string() == ")"))) {
            return name;
        }
        let mut player_name: MutableComponent = name.as_ref().unwrap().copy();
        {
            let mut i: i32 = (player_name.get_siblings().len() as i32).wrapping_sub(1);
            while i >= 0 {
                if compat::trim(&((player_name.get_siblings().remove((i) as usize)).get_string())) == "(" {
                    break;
                }
                i -= 1;
            }
        }
        return Some(player_name);
    }

}

impl TabHudWidget for DungeonPlayerWidget {
    fn tab_hud_widget_base(&self) -> &TabHudWidgetBase {
        &self.base
    }
    fn tab_hud_widget_base_mut(&mut self) -> &mut TabHudWidgetBase {
        &mut self.base
    }

    /// java: @Override public void updateContent(PlayerListManager.Widget ignored) (dispatch: the body for this class)
    fn update_content_1(&self) -> fn(&compat::Shared<dyn TabHudWidget>, &Widget) {
        DungeonPlayerWidget::update_content_1_dyn
    }

}

impl ElementBasedWidget for DungeonPlayerWidget {
    fn element_based_widget_base(&self) -> &ElementBasedWidgetBase {
        &self.base.base
    }
    fn element_based_widget_base_mut(&mut self) -> &mut ElementBasedWidgetBase {
        &mut self.base.base
    }

    /// java: @Override public void updateContent() (dispatch: the body for this class)
    fn update_content(&self) -> fn(&compat::Shared<dyn ElementBasedWidget>) {
        <dyn TabHudWidget>::update_content_from::<Self>
    }

    /// java: @Override protected final void updateConfigContent(ElementCollector collector) (dispatch: the body for this class)
    fn update_config_content(&self) -> fn(&compat::Shared<dyn ElementBasedWidget>, &compat::Shared<dyn ElementCollector>) {
        <dyn TabHudWidget>::update_config_content_from::<Self>
    }

}

impl HudWidget for DungeonPlayerWidget {
    fn hud_widget_base(&self) -> &HudWidgetBase {
        &self.base.base.base
    }
    fn hud_widget_base_mut(&mut self) -> &mut HudWidgetBase {
        &mut self.base.base.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override protected final void extractWidgetRenderState(GuiGraphicsExtractor context, float delta) (dispatch: the body for this class)
    fn extract_widget_render_state(&self) -> fn(&compat::Shared<dyn HudWidget>, &mut GuiGraphicsExtractor, f32) {
        <dyn ElementBasedWidget>::extract_widget_render_state_from::<Self>
    }

    /// java: @Override protected void extractWidgetRenderStateForConfig(GuiGraphicsExtractor graphics, float delta) (dispatch: the body for this class)
    fn extract_widget_render_state_for_config(&self) -> fn(&compat::Shared<dyn HudWidget>, &mut GuiGraphicsExtractor, f32) {
        <dyn ElementBasedWidget>::extract_widget_render_state_for_config_from::<Self>
    }

    /// java: @Override public boolean shouldRender() (dispatch: the body for this class)
    fn should_render(&self) -> fn(&compat::Shared<dyn HudWidget>) -> bool {
        <dyn TabHudWidget>::should_render_from::<Self>
    }

    /// java: @Override public void load(JsonValueInput input) (dispatch: the body for this class)
    fn load(&self) -> fn(&compat::Shared<dyn HudWidget>, &JsonValueInput) {
        <dyn TabHudWidget>::load_from::<Self>
    }

    /// java: @Override public void save(JsonValueOutput output) (dispatch: the body for this class)
    fn save(&self) -> fn(&compat::Shared<dyn HudWidget>, &mut JsonValueOutput) {
        <dyn TabHudWidget>::save_from::<Self>
    }

    /// java: @Override public void getOptionWidgets(OptionWidgetCollector collector) (dispatch: the body for this class)
    fn get_option_widgets(&self) -> fn(&compat::Shared<dyn HudWidget>, &mut OptionWidgetCollector) {
        <dyn TabHudWidget>::get_option_widgets_from::<Self>
    }

    /// java: @Override public void updateConfigPreview() (dispatch: the body for this class)
    fn update_config_preview(&self) -> fn(&compat::Shared<dyn HudWidget>) {
        <dyn TabHudWidget>::update_config_preview_from::<Self>
    }

}

impl ElementCollector for DungeonPlayerWidget {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: public <T extends Element> T addElement(T c)
    fn add_element(&mut self, mut c: compat::Shared<dyn Element>) -> compat::Shared<dyn Element> {
        ({ let __f = compat::dispatch(&c, |__t| __t.set_parent()); __f(&c, self.clone()) });
        self.elements.push(c.clone());
        return c;
    }

}

impl DungeonPlayerWidget {
    /// java: @Override public void updateContent(PlayerListManager.Widget ignored)
    pub fn update_content_1_impl(__this: &compat::Shared<Self>, ignored: &Widget) {
        let mut start: i32 = 1i32.wrapping_add(((compat::read(__this, |__t| __t.player)).wrapping_sub(1)).wrapping_mul(4));
        if PlayerListManager::str_at(start).is_none() {
            let mut idx: i32 = (compat::read(__this, |__t| __t.player)).wrapping_sub(1);
            { let (__a0,) = (Elements::icon_text_component_2(Some(ico::SIGN.get_mut().clone()), Some((component::literal(&MSGS[(idx) as usize])).with_style_format(ChatFormatting::Gray))),); __this.lock().borrow_mut().add_element(__a0) };
            return;
        }
        DungeonPlayerWidget::add_player_name_and_class(__this, start);
        ({ let __f = compat::dispatch(__this, |__t| __t.add_simple_ico_text()); __f(&compat::view(__this, |a| a as compat::Shared<dyn ElementCollector>), Some(ico::CLOCK.get_mut().clone()), "Ult Cooldown:", &ChatFormatting::Gold, start.wrapping_add(1)) });
        ({ let __f = compat::dispatch(__this, |__t| __t.add_simple_ico_text()); __f(&compat::view(__this, |a| a as compat::Shared<dyn ElementCollector>), Some(ico::POTION.get_mut().clone()), "Revives:", &ChatFormatting::DarkPurple, start.wrapping_add(2)) });
    }

    /// DungeonPlayerWidget.updateContent as the dispatch of TabHudWidget.updateContent
    pub fn update_content_1_dyn(__this: &compat::Shared<dyn TabHudWidget>, __p0: &Widget) {
        Self::update_content_1_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

}
