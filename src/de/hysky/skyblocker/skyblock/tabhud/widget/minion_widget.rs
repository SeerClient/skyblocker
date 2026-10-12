//! Ported from de.hysky.skyblocker.skyblock.tabhud.widget.MinionWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::tabhud::config::option_widget_collector::OptionWidgetCollector;
use crate::de::hysky::skyblocker::skyblock::tabhud::util::player_list_manager::Widget;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::element::Element;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::element_collector::ElementCollector;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::elements::Elements;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::plain_text_element::PlainTextElement;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element_based_widget::ElementBasedWidget;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element_based_widget::ElementBasedWidgetBase;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::hud_widget::HudWidget;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::hud_widget::HudWidgetBase;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::tab_hud_widget::TabHudWidget;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::tab_hud_widget::TabHudWidgetBase;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
use crate::de::hysky::skyblocker::utils::json_value_input::JsonValueInput;
use crate::de::hysky::skyblocker::utils::json_value_output::JsonValueOutput;
use crate::de::hysky::skyblocker::utils::location::Location;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::java::text::decimal_format_symbols::Locale;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::world::item::items;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final MutableComponent TITLE = Component.literal("Minions").withStyle(ChatFormatting.DARK_AQUA, ChatFormatting.BOLD);
pub static TITLE: compat::MainThread<MutableComponent> = compat::MainThread::new(|| component::literal("Minions").with_style_formats(&[ChatFormatting::DarkAqua, ChatFormatting::Bold]));
/// java: private static final HashMap<String, FlexibleItemStack> MIN_ICOS = new HashMap<>();
pub static MIN_ICOS: compat::MainThread<std::collections::HashMap<String, compat::Shared<FlexibleItemStack>>> = compat::MainThread::new(|| {
    let mut value = std::collections::HashMap::new();
    value.insert("Blaze".to_string(), FlexibleItemStack::new2(items::items().get("blaze_rod")));
    value.insert("Cave Spider".to_string(), FlexibleItemStack::new2(items::items().get("spider_eye")));
    value.insert("Creeper".to_string(), FlexibleItemStack::new2(items::items().get("gunpowder")));
    value.insert("Enderman".to_string(), FlexibleItemStack::new2(items::items().get("ender_pearl")));
    value.insert("Ghast".to_string(), FlexibleItemStack::new2(items::items().get("ghast_tear")));
    value.insert("Magma Cube".to_string(), FlexibleItemStack::new2(items::items().get("magma_cream")));
    value.insert("Skeleton".to_string(), FlexibleItemStack::new2(items::items().get("bone")));
    value.insert("Slime".to_string(), FlexibleItemStack::new2(items::items().get("slime_ball")));
    value.insert("Spider".to_string(), FlexibleItemStack::new2(items::items().get("string")));
    value.insert("Zombie".to_string(), FlexibleItemStack::new2(items::items().get("rotten_flesh")));
    value.insert("Cactus".to_string(), FlexibleItemStack::new2(items::items().get("cactus")));
    value.insert("Carrot".to_string(), FlexibleItemStack::new2(items::items().get("carrot")));
    value.insert("Chicken".to_string(), FlexibleItemStack::new2(items::items().get("chicken")));
    value.insert("Cocoa Beans".to_string(), FlexibleItemStack::new2(items::items().get("cocoa_beans")));
    value.insert("Cow".to_string(), FlexibleItemStack::new2(items::items().get("beef")));
    value.insert("Melon".to_string(), FlexibleItemStack::new2(items::items().get("melon_slice")));
    value.insert("Mushroom".to_string(), FlexibleItemStack::new2(items::items().get("red_mushroom")));
    value.insert("Nether Wart".to_string(), FlexibleItemStack::new2(items::items().get("nether_wart")));
    value.insert("Pig".to_string(), FlexibleItemStack::new2(items::items().get("porkchop")));
    value.insert("Potato".to_string(), FlexibleItemStack::new2(items::items().get("potato")));
    value.insert("Pumpkin".to_string(), FlexibleItemStack::new2(items::items().get("pumpkin")));
    value.insert("Rabbit".to_string(), FlexibleItemStack::new2(items::items().get("rabbit")));
    value.insert("Sheep".to_string(), FlexibleItemStack::new2(mcrust::net::minecraft::world::item::items::wool().white().clone()));
    value.insert("Sugar Cane".to_string(), FlexibleItemStack::new2(items::items().get("sugar_cane")));
    value.insert("Wheat".to_string(), FlexibleItemStack::new2(items::items().get("wheat")));
    value.insert("Clay".to_string(), FlexibleItemStack::new2(items::items().get("clay")));
    value.insert("Fishing".to_string(), FlexibleItemStack::new2(items::items().get("fishing_rod")));
    value.insert("Lily Pad".to_string(), FlexibleItemStack::new2(items::items().get("lily_pad")));
    value.insert("Coal".to_string(), FlexibleItemStack::new2(items::items().get("coal")));
    value.insert("Cobblestone".to_string(), FlexibleItemStack::new2(items::items().get("cobblestone")));
    value.insert("Diamond".to_string(), FlexibleItemStack::new2(items::items().get("diamond")));
    value.insert("Emerald".to_string(), FlexibleItemStack::new2(items::items().get("emerald")));
    value.insert("End Stone".to_string(), FlexibleItemStack::new2(items::items().get("end_stone")));
    value.insert("Glowstone".to_string(), FlexibleItemStack::new2(items::items().get("glowstone_dust")));
    value.insert("Gold".to_string(), FlexibleItemStack::new2(items::items().get("gold_ingot")));
    value.insert("Gravel".to_string(), FlexibleItemStack::new2(items::items().get("gravel")));
    value.insert("Hard Stone".to_string(), FlexibleItemStack::new2(items::items().get("stone")));
    value.insert("Ice".to_string(), FlexibleItemStack::new2(items::items().get("ice")));
    value.insert("Iron".to_string(), FlexibleItemStack::new2(items::items().get("iron_ingot")));
    value.insert("Lapis".to_string(), FlexibleItemStack::new2(items::items().get("lapis_lazuli")));
    value.insert("Mithril".to_string(), FlexibleItemStack::new2(items::items().get("prismarine_crystals")));
    value.insert("Mycelium".to_string(), FlexibleItemStack::new2(items::items().get("mycelium")));
    value.insert("Obsidian".to_string(), FlexibleItemStack::new2(items::items().get("obsidian")));
    value.insert("Quartz".to_string(), FlexibleItemStack::new2(items::items().get("quartz")));
    value.insert("Red Sand".to_string(), FlexibleItemStack::new2(items::items().get("red_sand")));
    value.insert("Redstone".to_string(), FlexibleItemStack::new2(items::items().get("redstone")));
    value.insert("Sand".to_string(), FlexibleItemStack::new2(items::items().get("sand")));
    value.insert("Snow".to_string(), FlexibleItemStack::new2(items::items().get("snowball")));
    value.insert("Inferno".to_string(), FlexibleItemStack::new2(items::items().get("blaze_spawn_egg")));
    value.insert("Revenant".to_string(), FlexibleItemStack::new2(items::items().get("zombie_spawn_egg")));
    value.insert("Tarantula".to_string(), FlexibleItemStack::new2(items::items().get("spider_spawn_egg")));
    value.insert("Vampire".to_string(), FlexibleItemStack::new2(items::items().get("redstone")));
    value.insert("Voidling".to_string(), FlexibleItemStack::new2(items::items().get("enderman_spawn_egg")));
    value.insert("Acacia".to_string(), FlexibleItemStack::new2(items::items().get("acacia_log")));
    value.insert("Birch".to_string(), FlexibleItemStack::new2(items::items().get("birch_log")));
    value.insert("Dark Oak".to_string(), FlexibleItemStack::new2(items::items().get("dark_oak_log")));
    value.insert("Flower".to_string(), FlexibleItemStack::new2(items::items().get("poppy")));
    value.insert("Jungle".to_string(), FlexibleItemStack::new2(items::items().get("jungle_log")));
    value.insert("Oak".to_string(), FlexibleItemStack::new2(items::items().get("oak_log")));
    value.insert("Spruce".to_string(), FlexibleItemStack::new2(items::items().get("spruce_log")));
    value
});
/// java: public static final Pattern MINION_PATTERN = Pattern.compile("^(?<amount>\\d+)x (?<name>.*) (?<level>[XVI]*) \\[(?<status>.*)]");
pub static MINION_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("^(?<amount>\\d+)x (?<name>.*) (?<level>[XVI]*) \\[(?<status>.*)]"));

/// java: @RegisterWidget public class MinionWidget extends TabHudWidget
#[derive(Clone)]
pub struct MinionWidget {
    /// java: extends TabHudWidget
    pub base: TabHudWidgetBase,
}

impl std::ops::Deref for MinionWidget {
    type Target = TabHudWidgetBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for MinionWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl MinionWidget {
    /// java: public MinionWidget()
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = TabHudWidgetBase::new3_in("Minions".to_string(), TITLE.get_mut().clone(), mcrust::net::minecraft::network::chat::text_color::TextColor::DARK_AQUA.get_value(), Location::PrivateIsland, move |__base| {
            let __h = compat::shared(Self {
                base: __base,
            });
            ((__h.clone() as compat::Shared<dyn TabHudWidget>), __h)
        });
        __this
    }

    /// java: public void addMinionComponent(String line)
    pub fn add_minion_component(__this: &compat::Shared<Self>, line: &str) {
        let mut m: compat::regex::Matcher = MINION_PATTERN.matcher(line);
        if m.matches() {
            let mut min: String = m.group_named("name").unwrap();
            let mut amount: String = m.group_named("amount").unwrap();
            let mut lvl: String = m.group_named("level").unwrap();
            let mut stat: String = m.group_named("status").unwrap();
            let mut mt: MutableComponent = (component::literal(&format!("{}x {} {}", amount, min, lvl))).append(component::literal(": "));
            let mut format: ChatFormatting = ChatFormatting::Red;
            if stat == "ACTIVE" {
                format = ChatFormatting::Green;
            } else if stat == "SLOW" {
                format = ChatFormatting::Yellow;
            }
            mt = mt.append(component::literal(&stat).with_style_format(format));
            { let (__a0,) = (Elements::icon_text_component_2(MIN_ICOS.get_mut().get(&min).cloned(), Some(mt.clone())),); __this.lock().borrow_mut().add_element(__a0) };
        }
    }

}

impl TabHudWidget for MinionWidget {
    fn tab_hud_widget_base(&self) -> &TabHudWidgetBase {
        &self.base
    }
    fn tab_hud_widget_base_mut(&mut self) -> &mut TabHudWidgetBase {
        &mut self.base
    }

    /// java: @Override public void updateContent(PlayerListManager.Widget widget) (dispatch: the body for this class)
    fn update_content_1(&self) -> fn(&compat::Shared<dyn TabHudWidget>, &Widget) {
        MinionWidget::update_content_1_dyn
    }

}

impl ElementBasedWidget for MinionWidget {
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

impl HudWidget for MinionWidget {
    fn hud_widget_base(&self) -> &HudWidgetBase {
        &self.base.base.base
    }
    fn hud_widget_base_mut(&mut self) -> &mut HudWidgetBase {
        &mut self.base.base.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public boolean shouldRender() (dispatch: the body for this class)
    fn should_render(&self) -> fn(&compat::Shared<dyn HudWidget>) -> bool {
        MinionWidget::should_render_dyn
    }

    /// java: @Override protected final void extractWidgetRenderState(GuiGraphicsExtractor context, float delta) (dispatch: the body for this class)
    fn extract_widget_render_state(&self) -> fn(&compat::Shared<dyn HudWidget>, &mut GuiGraphicsExtractor, f32) {
        <dyn ElementBasedWidget>::extract_widget_render_state_from::<Self>
    }

    /// java: @Override protected void extractWidgetRenderStateForConfig(GuiGraphicsExtractor graphics, float delta) (dispatch: the body for this class)
    fn extract_widget_render_state_for_config(&self) -> fn(&compat::Shared<dyn HudWidget>, &mut GuiGraphicsExtractor, f32) {
        <dyn ElementBasedWidget>::extract_widget_render_state_for_config_from::<Self>
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

impl ElementCollector for MinionWidget {
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

impl MinionWidget {
    /// java: @Override public void updateContent(PlayerListManager.Widget widget)
    pub fn update_content_1_impl(__this: &compat::Shared<Self>, widget: &Widget) {
        { let (__a0,) = ((PlainTextElement::new(Some(widget.detail().copy().append(component::literal(" minions")))) as compat::Shared<dyn Element>),); __this.lock().borrow_mut().add_element(__a0) };
        for line in widget.lines() {
            let mut string: String = line.get_string();
            if string.to_lowercase().starts_with("...") {
                { let (__a0,) = ((PlainTextElement::new(Some(line.copy().with_style_format(ChatFormatting::Gray))) as compat::Shared<dyn Element>),); __this.lock().borrow_mut().add_element(__a0) };
            } else {
                MinionWidget::add_minion_component(__this, &string);
            }
        }
    }

    /// MinionWidget.updateContent as the dispatch of TabHudWidget.updateContent
    pub fn update_content_1_dyn(__this: &compat::Shared<dyn TabHudWidget>, __p0: &Widget) {
        Self::update_content_1_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public boolean shouldRender()
    pub fn should_render_impl(__this: &compat::Shared<Self>) -> bool {
        return (<dyn TabHudWidget>::should_render_body(&compat::view(__this, |a| a as compat::Shared<dyn TabHudWidget>)) && !(Utils::is_guest()));
    }

    /// MinionWidget.shouldRender as the dispatch of HudWidget.shouldRender
    pub fn should_render_dyn(__this: &compat::Shared<dyn HudWidget>) -> bool {
        Self::should_render_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}
