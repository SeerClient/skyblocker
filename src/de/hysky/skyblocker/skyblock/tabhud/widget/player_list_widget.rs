//! Ported from de.hysky.skyblocker.skyblock.tabhud.widget.PlayerListWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::configs::ui_and_visuals_config;
use crate::de::hysky::skyblocker::config::configs::ui_and_visuals_config::NameSorting;
use crate::de::hysky::skyblocker::skyblock::tabhud::config::option_widget_collector::OptionWidgetCollector;
use crate::de::hysky::skyblocker::skyblock::tabhud::util::player_list_manager::Widget;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::element::Element;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::element_collector::ElementCollector;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::player_element::PlayerElement;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element_based_widget::ElementBasedWidget;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element_based_widget::ElementBasedWidgetBase;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::hud_widget::HudWidget;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::hud_widget::HudWidgetBase;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::tab_hud_widget::TabHudWidget;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::tab_hud_widget::TabHudWidgetBase;
use crate::de::hysky::skyblocker::utils::json_value_input::JsonValueInput;
use crate::de::hysky::skyblocker::utils::json_value_output::JsonValueOutput;
use mcrust::com::mojang::authlib::game_profile::GameProfile;
use mcrust::java::util::uuid::Uuid;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::multiplayer::player_info::PlayerInfo;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::network::chat::text_color::TextColor;

/// java: private static final MutableComponent TITLE = Component.literal("Players").withStyle(ChatFormatting.BOLD);
pub static TITLE: compat::MainThread<MutableComponent> = compat::MainThread::new(|| component::literal("Players").with_style_format(ChatFormatting::Bold));

/// java: @RegisterWidget public class PlayerListWidget extends TabHudWidget
#[derive(Clone)]
pub struct PlayerListWidget {
    /// java: extends TabHudWidget
    pub base: TabHudWidgetBase,
    /// java: private UIAndVisualsConfig.NameSorting nameSorting = UIAndVisualsConfig.NameSorting.DEFAULT;
    pub name_sorting: NameSorting,
}

impl std::ops::Deref for PlayerListWidget {
    type Target = TabHudWidgetBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for PlayerListWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl PlayerListWidget {
    /// java: public PlayerListWidget()
    pub fn new() -> compat::Shared<Self> {
        // NOTE(fabric2seer): `nameSorting` is read by `updateContent`, which TabHudWidget's constructor calls before this constructor assigns it: Java sees its default there (0 / false / null), and so does this port
        let (_, __this) = TabHudWidgetBase::new_3_in("Players".to_string(), TITLE.get_mut().clone(), mcrust::net::minecraft::network::chat::text_color::TextColor::AQUA.get_value(), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                name_sorting: NameSorting::Default,
            });
            ((__h.clone() as compat::Shared<dyn TabHudWidget>), __h)
        });
        __this
    }

}

impl TabHudWidget for PlayerListWidget {
    fn tab_hud_widget_base(&self) -> &TabHudWidgetBase {
        &self.base
    }
    fn tab_hud_widget_base_mut(&mut self) -> &mut TabHudWidgetBase {
        &mut self.base
    }

    /// java: @Override protected void updateContent(PlayerListManager.Widget widget) (dispatch: the body for this class)
    fn update_content_1(&self) -> fn(&compat::Shared<dyn TabHudWidget>, &Widget) {
        PlayerListWidget::update_content_1_dyn
    }

    /// java: @Override protected void updateConfigContentTab(ElementCollector collector) (dispatch: the body for this class)
    fn update_config_content_tab(&self) -> fn(&compat::Shared<dyn TabHudWidget>, &compat::Shared<dyn ElementCollector>) {
        PlayerListWidget::update_config_content_tab_dyn
    }

}

impl ElementBasedWidget for PlayerListWidget {
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

impl HudWidget for PlayerListWidget {
    fn hud_widget_base(&self) -> &HudWidgetBase {
        &self.base.base.base
    }
    fn hud_widget_base_mut(&mut self) -> &mut HudWidgetBase {
        &mut self.base.base.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void getOptionWidgets(OptionWidgetCollector collector) (dispatch: the body for this class)
    fn get_option_widgets(&self) -> fn(&compat::Shared<dyn HudWidget>, &mut OptionWidgetCollector) {
        PlayerListWidget::get_option_widgets_dyn
    }

    /// java: @Override public void load(JsonValueInput input) (dispatch: the body for this class)
    fn load(&self) -> fn(&compat::Shared<dyn HudWidget>, &JsonValueInput) {
        PlayerListWidget::load_dyn
    }

    /// java: @Override public void save(JsonValueOutput output) (dispatch: the body for this class)
    fn save(&self) -> fn(&compat::Shared<dyn HudWidget>, &mut JsonValueOutput) {
        PlayerListWidget::save_dyn
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

    /// java: @Override public void updateConfigPreview() (dispatch: the body for this class)
    fn update_config_preview(&self) -> fn(&compat::Shared<dyn HudWidget>) {
        <dyn TabHudWidget>::update_config_preview_from::<Self>
    }

}

impl ElementCollector for PlayerListWidget {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: public <T extends Element> T addElement(T c)
    fn add_element(&mut self, mut c: compat::Shared<dyn Element>) -> compat::Shared<dyn Element> {
        ({ let __f = compat::dispatch(&c, |__t| __t.set_parent()); __f(&c, self.hud_widget_base().as_parent()) });
        self.elements.push(c.clone());
        return c;
    }

}

impl PlayerListWidget {
    /// java: @Override protected void updateContent(PlayerListManager.Widget widget)
    pub fn update_content_1_impl(__this: &compat::Shared<Self>, widget: &Widget) {
        let __comparator = compat::read(__this, |__t| __t.name_sorting.fields().comparator.clone());
        for player_list_entry in compat::sorted_by(widget.player_list_entries().into_iter(), |a, b| compat::read(a, |a| compat::read(b, |b| (__comparator)(a, b)))) {
            { let (__a0,) = ((PlayerElement::new(&player_list_entry) as compat::Shared<dyn Element>),); __this.lock().borrow_mut().add_element(__a0) };
        }
    }

    /// PlayerListWidget.updateContent as the dispatch of TabHudWidget.updateContent
    pub fn update_content_1_dyn(__this: &compat::Shared<dyn TabHudWidget>, __p0: &Widget) {
        Self::update_content_1_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override protected void updateConfigContentTab(ElementCollector collector)
    pub fn update_config_content_tab_impl(__this: &compat::Shared<Self>, collector: &compat::Shared<dyn ElementCollector>) {
        let mut random: compat::JavaRandom = compat::JavaRandom::new_seeded(1234);
        let mut infos: Vec<compat::Shared<PlayerInfo>> = Vec::with_capacity((15) as usize);
        let mut colors: Vec<TextColor> = vec![mcrust::net::minecraft::network::chat::text_color::TextColor::AQUA, mcrust::net::minecraft::network::chat::text_color::TextColor::GREEN, mcrust::net::minecraft::network::chat::text_color::TextColor::GRAY];
        for i in 0i32..3 {
            let color = &colors[(i) as usize];
            // java: int[] array = random.ints(5, 0, 500).sorted().toArray();
            let mut array: Vec<i32> = random.ints(5, 0, 500);
            array.sort();
            // java: ArrayUtils.reverse(array);
            array.reverse();
            for j in 0i32..5 {
                // java: new GameProfile(UUID.randomUUID(), String.valueOf((char) ('a' + i * 5 + j)))
                let mut profile: GameProfile = GameProfile::new(Uuid::random_uuid(), &char::from_u32(((('a' as u32 as i32).wrapping_add(i.wrapping_mul(5))).wrapping_add(j)) as u32).unwrap_or('\0').to_string(), mcrust::com::mojang::authlib::properties::property_map::PropertyMap::empty());
                let mut info: PlayerInfo = PlayerInfo::new(profile, false);
                let mut builder: String = "Player ".to_string();
                // java: random.ints(3, 'A', 'D').forEach(builder::appendCodePoint);
                for __c in random.ints(3, 'A' as i32, 'D' as i32) {
                    builder.push(char::from_u32(__c as u32).unwrap_or('\u{FFFD}'));
                }
                info.set_tab_list_display_name(Some(mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from((component::literal(&format!("[{}] ", array[(j) as usize]))).append(component::literal(&builder).with_color_text(color.clone())))));
                infos.push(compat::shared(info));
            }
        }
        let __comparator = compat::read(__this, |__t| __t.name_sorting.fields().comparator.clone());
        for player_list_entry in compat::sorted_by(infos.into_iter(), |a, b| compat::read(a, |a| compat::read(b, |b| (__comparator)(a, b)))) {
            { let (__a0,) = ((PlayerElement::new(&player_list_entry) as compat::Shared<dyn Element>),); collector.lock().borrow_mut().add_element(__a0) };
        }
    }

    /// PlayerListWidget.updateConfigContentTab as the dispatch of TabHudWidget.updateConfigContentTab
    pub fn update_config_content_tab_dyn(__this: &compat::Shared<dyn TabHudWidget>, __p0: &compat::Shared<dyn ElementCollector>) {
        Self::update_config_content_tab_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public void getOptionWidgets(OptionWidgetCollector collector)
    pub fn get_option_widgets_impl(__this: &compat::Shared<Self>, collector: &mut OptionWidgetCollector) {
        <dyn TabHudWidget>::get_option_widgets_body(&compat::view(__this, |a| a as compat::Shared<dyn TabHudWidget>), collector);
        let __this_w1 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        (collector.enum_button(NameSorting::values(), &component::translatable("skyblocker.config.uiAndVisuals.tabHud.nameSorting"), Arc::new(move |v| { let Some(__this_1) = __this_w1.get().upgrade() else { return Default::default(); }; { { let __v = v.clone(); compat::write((&__this_1), |__t| __t.name_sorting = __v); } } }), &compat::read(__this, |__t| __t.name_sorting))).tooltip(component::translatable("skyblocker.config.uiAndVisuals.tabHud.nameSorting.@Tooltip"));
    }

    /// PlayerListWidget.getOptionWidgets as the dispatch of HudWidget.getOptionWidgets
    pub fn get_option_widgets_dyn(__this: &compat::Shared<dyn HudWidget>, __p0: &mut OptionWidgetCollector) {
        Self::get_option_widgets_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public void load(JsonValueInput input)
    pub fn load_impl(__this: &compat::Shared<Self>, input: &JsonValueInput) {
        <dyn TabHudWidget>::load_body(&compat::view(__this, |a| a as compat::Shared<dyn TabHudWidget>), input);
        { let __v = input.read("name_sorting", ui_and_visuals_config::NAME_SORTING_CODEC.get_mut()).unwrap_or(NameSorting::Default); compat::write(__this, |__t| __t.name_sorting = __v); }
    }

    /// PlayerListWidget.load as the dispatch of HudWidget.load
    pub fn load_dyn(__this: &compat::Shared<dyn HudWidget>, __p0: &JsonValueInput) {
        Self::load_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public void save(JsonValueOutput output)
    pub fn save_impl(__this: &compat::Shared<Self>, output: &mut JsonValueOutput) {
        <dyn TabHudWidget>::save_body(&compat::view(__this, |a| a as compat::Shared<dyn TabHudWidget>), output);
        output.write("name_sorting".to_string(), ui_and_visuals_config::NAME_SORTING_CODEC.get_mut(), &compat::read(__this, |__t| __t.name_sorting));
    }

    /// PlayerListWidget.save as the dispatch of HudWidget.save
    pub fn save_dyn(__this: &compat::Shared<dyn HudWidget>, __p0: &mut JsonValueOutput) {
        Self::save_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

}
