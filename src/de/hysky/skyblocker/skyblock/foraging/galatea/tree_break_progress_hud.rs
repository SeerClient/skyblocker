//! Ported from de.hysky.skyblocker.skyblock.foraging.galatea.TreeBreakProgressHud by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::tabhud::config::option_widget_collector::OptionWidgetCollector;
use crate::de::hysky::skyblocker::skyblock::tabhud::util::ico;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::element::Element;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::element_collector::ElementCollector;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element_based_widget::ElementBasedWidget;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element_based_widget::ElementBasedWidgetBase;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::hud_widget::HudWidget;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::hud_widget::HudWidgetBase;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::hud_widget::Information;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
use crate::de::hysky::skyblocker::utils::json_value_input::JsonValueInput;
use crate::de::hysky::skyblocker::utils::json_value_output::JsonValueOutput;
use crate::de::hysky::skyblocker::utils::location::Location;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::multiplayer::client_level::ClientLevel;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::world::entity::decoration::armor_stand::ArmorStand;
use mcrust::net::minecraft::world::phys::vec3::Vec3;
use mcrust::net::minecraft::network::chat::component::Component as _;
use mcrust::net::minecraft::world::entity::entity::Entity as _;

/// java: private static @Nullable TreeBreakProgressHud instance;
pub static INSTANCE: compat::MainThread<Option<compat::Shared<TreeBreakProgressHud>>> = compat::MainThread::new(|| None);
/// java: private static final Int2ObjectMap<ArmorStand> armorstands = new Int2ObjectOpenHashMap<>();
/// (the armor stands' ids: the level keeps the entities, `game::with_entity` reaches them)
pub static ARMORSTANDS: compat::MainThread<std::collections::HashSet<i32>> = compat::MainThread::new(|| std::collections::HashSet::new());

/// java: @RegisterWidget public class TreeBreakProgressHud extends ElementBasedWidget
#[derive(Clone)]
pub struct TreeBreakProgressHud {
    /// java: extends ElementBasedWidget
    pub base: ElementBasedWidgetBase,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl std::ops::Deref for TreeBreakProgressHud {
    type Target = ElementBasedWidgetBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for TreeBreakProgressHud {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl TreeBreakProgressHud {
    /// java: static { .. } - Java runs it when the class is first used; every static method and
    /// constructor calls this first.
    fn clinit() {
        static ONCE: std::sync::Once = std::sync::Once::new();
        ONCE.call_once(|| {
            // java: ClientEntityEvents.ENTITY_UNLOAD.register((entity, _) -> armorstands.remove(entity.getId()));
            compat::batch_w_register_entity_unload(Arc::new(|entity: &dyn mcrust::net::minecraft::world::entity::entity::Entity, _level: &mcrust::net::minecraft::client::multiplayer::client_level::ClientLevel| {
                ARMORSTANDS.get_mut().remove(&entity.base().get_id());
            }));
        });
    }

    /// java: public TreeBreakProgressHud()
    pub fn new() -> compat::Shared<Self> {
        Self::clinit();
        let (_, __this) = ElementBasedWidgetBase::new_in(component::literal("Tree Break Progress").with_style_formats(&[ChatFormatting::Green, ChatFormatting::Bold]), mcrust::net::minecraft::network::chat::text_color::TextColor::GREEN.get_value(), Information::new_4("hud_treeprogress".to_string(), component::literal("Tree Break Progress"), Location::Galatea, vec![Location::TorrhusCanyon]), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                self_ref: std::sync::Weak::new(),
            });
            compat::write(&__h, |__t| __t.self_ref = std::sync::Arc::downgrade(&__h));
            ((__h.clone() as compat::Shared<dyn ElementBasedWidget>), __h)
        });
        *INSTANCE.get_mut() = Some(__this.clone());
        ({ let __f = compat::dispatch(&__this, |__t| __t.update()); __f(&compat::view(&__this, |a| a as compat::Shared<dyn ElementBasedWidget>)) });
        __this
    }

    /// java: public static void onEntityUpdate(ArmorStand entity)
    pub fn on_entity_update(entity: &ArmorStand) {
        Self::clinit();
        if entity.base.base.get_custom_name().is_some() {
            ARMORSTANDS.get_mut().insert(entity.base.base.get_id());
        }
    }

    /// java: public static TreeBreakProgressHud getInstance()
    pub fn get_instance() -> compat::Shared<TreeBreakProgressHud> {
        Self::clinit();
        return INSTANCE.get_mut().clone().unwrap();
    }

    /// java: private @Nullable ArmorStand getClosestTree() (the armor stand's entity id)
    pub fn get_closest_tree(__this: &compat::Shared<Self>) -> Option<i32> {
        let Some(player_pos) = mcrust::mcrust::modding::game::with_player(|player| player.base().position()) else {
            return None;
        };
        // java: armorstands.values().stream().filter(..).min(Comparator.comparingDouble(entity -> entity.distanceToSqr(player)))
        let mut closest: Option<(i32, f64)> = None;
        for id in ARMORSTANDS.get().iter().copied().collect::<Vec<_>>() {
            let Some(Some(distance)) = mcrust::mcrust::modding::game::with_entity(id, |entity| {
                let Some(mut name) = entity.base().get_custom_name() else {
                    return None;
                };
                let mut string: String = name.get_string();
                if !(string.contains("FIG TREE") || string.contains("MANGROVE TREE") || string.contains("HELIX TREE")) {
                    return None;
                }
                Some(entity.base().distance_to_sqr_vec(&player_pos))
            }) else {
                continue;
            };
            if closest.map_or(true, |(_, best)| distance < best) {
                closest = Some((id, distance));
            }
        }
        return closest.map(|(id, _)| id);
    }

    /// java: private boolean isOwnTree(@Nullable ArmorStand tree)
    pub fn is_own_tree(__this: &compat::Shared<Self>, mut tree: Option<i32>) -> bool {
        if mcrust::mcrust::modding::game::with_player(|_| ()).is_none() {
            return false;
        }
        let Some(mut tree) = tree else {
            return false;
        };
        let Some(mut tree_pos) = mcrust::mcrust::modding::game::with_entity(tree, |e| e.base().position()) else {
            return false;
        };
        let mut player_name: String = mcrust::mcrust::modding::game::with_player(|player| compat::component_of(player.get_name()).get_string()).unwrap();
        // java: armorstands.values().stream().filter(near the tree).anyMatch(name contains the player's name or " players")
        return ARMORSTANDS.get().iter().copied().collect::<Vec<_>>().into_iter().any(|id| {
            mcrust::mcrust::modding::game::with_entity(id, |armor_stand| {
                let mut pos: Vec3 = armor_stand.base().position();
                if !((pos.x - tree_pos.x).abs() < 0.1 && (pos.y - tree_pos.y).abs() < (2 as f64) && (pos.z - tree_pos.z).abs() < 0.1) {
                    return false;
                }
                let mut name: String = compat::component_of(armor_stand.get_name()).get_string();
                return (name.contains(&player_name) || name.contains(" players"));
            }).unwrap_or(false)
        });
    }

}

impl ElementBasedWidget for TreeBreakProgressHud {
    fn element_based_widget_base(&self) -> &ElementBasedWidgetBase {
        &self.base
    }
    fn element_based_widget_base_mut(&mut self) -> &mut ElementBasedWidgetBase {
        &mut self.base
    }

    /// java: @Override public boolean shouldUpdateBeforeRendering() (dispatch: the body for this class)
    fn should_update_before_rendering(&self) -> fn(&compat::Shared<dyn ElementBasedWidget>) -> bool {
        TreeBreakProgressHud::should_update_before_rendering_dyn
    }

    /// java: @Override public void updateContent() (dispatch: the body for this class)
    fn update_content(&self) -> fn(&compat::Shared<dyn ElementBasedWidget>) {
        TreeBreakProgressHud::update_content_dyn
    }

    /// java: @Override protected void updateConfigContent(ElementCollector collector) (dispatch: the body for this class)
    fn update_config_content(&self) -> fn(&compat::Shared<dyn ElementBasedWidget>, &compat::Shared<dyn ElementCollector>) {
        TreeBreakProgressHud::update_config_content_dyn
    }

}

impl HudWidget for TreeBreakProgressHud {
    fn hud_widget_base(&self) -> &HudWidgetBase {
        &self.base.base
    }
    fn hud_widget_base_mut(&mut self) -> &mut HudWidgetBase {
        &mut self.base.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public boolean shouldRender() (dispatch: the body for this class)
    fn should_render(&self) -> fn(&compat::Shared<dyn HudWidget>) -> bool {
        TreeBreakProgressHud::should_render_dyn
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
        <dyn ElementBasedWidget>::load_from::<Self>
    }

    /// java: @Override public void save(JsonValueOutput output) (dispatch: the body for this class)
    fn save(&self) -> fn(&compat::Shared<dyn HudWidget>, &mut JsonValueOutput) {
        <dyn ElementBasedWidget>::save_from::<Self>
    }

    /// java: @Override public void getOptionWidgets(OptionWidgetCollector collector) (dispatch: the body for this class)
    fn get_option_widgets(&self) -> fn(&compat::Shared<dyn HudWidget>, &mut OptionWidgetCollector) {
        <dyn ElementBasedWidget>::get_option_widgets_from::<Self>
    }

    /// java: @Override public void updateConfigPreview() (dispatch: the body for this class)
    fn update_config_preview(&self) -> fn(&compat::Shared<dyn HudWidget>) {
        <dyn ElementBasedWidget>::update_config_preview_from::<Self>
    }

}

impl ElementCollector for TreeBreakProgressHud {
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

impl TreeBreakProgressHud {
    /// java: @Override public boolean shouldUpdateBeforeRendering()
    pub fn should_update_before_rendering_impl(__this: &compat::Shared<Self>) -> bool {
        return true;
    }

    /// TreeBreakProgressHud.shouldUpdateBeforeRendering as the dispatch of ElementBasedWidget.shouldUpdateBeforeRendering
    pub fn should_update_before_rendering_dyn(__this: &compat::Shared<dyn ElementBasedWidget>) -> bool {
        Self::should_update_before_rendering_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public void updateContent()
    pub fn update_content_impl(__this: &compat::Shared<Self>) {
        let mut world: Option<Arc<ClientLevel>> = mcrust::mcrust::modding::game::level();
        let mut closest: Option<i32> = None;
        if mcrust::mcrust::modding::game::with_player(|_| ()).is_none() || world.is_none() {
            return;
        }
        closest = TreeBreakProgressHud::get_closest_tree(__this);
        if closest.is_none() || !(TreeBreakProgressHud::is_own_tree(__this, closest)) {
            return;
        }
        let Some(mut closest_name) = mcrust::mcrust::modding::game::with_entity(closest.unwrap(), |closest| compat::component_of(closest.get_name()).get_string()) else {
            return;
        };
        let mut tree_name: String = (if closest_name.contains("HELIX") { "Helix Tree" } else { (if closest_name.contains("FIG") { "Fig Tree" } else { "Mangrove Tree" }) }).to_string();
        let mut wood_icon: compat::Shared<FlexibleItemStack> = (if closest_name.contains("HELIX") { ico::STRIPPED_MANGROVE_LOG.get_mut().clone() } else { (if closest_name.contains("FIG") { ico::STRIPPED_SPRUCE_WOOD.get_mut().clone() } else { ico::MANGROVE_LOG.get_mut().clone() }) });
        ({ let __f = compat::dispatch(__this, |__t| __t.add_simple_ico_text_flexible_item_stack_string_chat_formatting_string()); __f(&compat::view(__this, |a| a as compat::Shared<dyn ElementCollector>), Some(wood_icon.clone()), &format!("{} ", tree_name), &ChatFormatting::Green, &compat::regex::replace_all(&closest_name, "[^0-9%]", "")) });
    }

    /// TreeBreakProgressHud.updateContent as the dispatch of ElementBasedWidget.updateContent
    pub fn update_content_dyn(__this: &compat::Shared<dyn ElementBasedWidget>) {
        Self::update_content_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override protected void updateConfigContent(ElementCollector collector)
    pub fn update_config_content_impl(__this: &compat::Shared<Self>, collector: &compat::Shared<dyn ElementCollector>) {
        ({ let __f = compat::dispatch(collector, |__t| __t.add_simple_ico_text_flexible_item_stack_string_chat_formatting_string()); __f(collector, Some(ico::STRIPPED_SPRUCE_WOOD.get_mut().clone()), "Fig Tree ", &ChatFormatting::Green, "37%") });
    }

    /// TreeBreakProgressHud.updateConfigContent as the dispatch of ElementBasedWidget.updateConfigContent
    pub fn update_config_content_dyn(__this: &compat::Shared<dyn ElementBasedWidget>, __p0: &compat::Shared<dyn ElementCollector>) {
        Self::update_config_content_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public boolean shouldRender()
    pub fn should_render_impl(__this: &compat::Shared<Self>) -> bool {
        return TreeBreakProgressHud::is_own_tree(__this, TreeBreakProgressHud::get_closest_tree(__this));
    }

    /// TreeBreakProgressHud.shouldRender as the dispatch of HudWidget.shouldRender
    pub fn should_render_dyn(__this: &compat::Shared<dyn HudWidget>) -> bool {
        Self::should_render_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}
