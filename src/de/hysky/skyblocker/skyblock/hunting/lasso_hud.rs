//! Ported from de.hysky.skyblocker.skyblock.hunting.LassoHud by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::tabhud::config::option_widget_collector::OptionWidgetCollector;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::widget_manager::WidgetManager;
use crate::de::hysky::skyblocker::skyblock::tabhud::util::ico;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::element::Element;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::element_collector::ElementCollector;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::elements::Elements;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element_based_widget::ElementBasedWidget;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element_based_widget::ElementBasedWidgetBase;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::hud_widget::HudWidget;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::hud_widget::HudWidgetBase;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::hud_widget::Information;
use crate::de::hysky::skyblocker::utils::json_value_input::JsonValueInput;
use crate::de::hysky::skyblocker::utils::json_value_output::JsonValueOutput;
use crate::de::hysky::skyblocker::utils::location::Location;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::network::protocol::game::clientbound_set_entity_link_packet::ClientboundSetEntityLinkPacket;
use mcrust::net::minecraft::world::entity::decoration::armor_stand::ArmorStand;
use mcrust::net::minecraft::world::entity::entity::Entity;
use mcrust::net::minecraft::world::entity::player::player::Player;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final String LASSO_COUNT_DOWN_NAME = " ";
pub const LASSO_COUNT_DOWN_NAME: &str = "                    ";
/// java: private static @Nullable LassoHud instance;
pub static INSTANCE: compat::MainThread<Option<compat::Shared<LassoHud>>> = compat::MainThread::new(|| None);
/// java: private static int percentage = 0;
pub static PERCENTAGE: LazyLock<Mutex<i32>> = LazyLock::new(|| Mutex::new(0));
/// java: private static int reelValue = 0;
pub static REEL_VALUE: LazyLock<Mutex<i32>> = LazyLock::new(|| Mutex::new(0));
/// java: private static @Nullable Entity lassoEntity = null;
pub static LASSO_ENTITY: compat::MainThread<Option<Box<dyn Entity>>> = compat::MainThread::new(|| None);

/// java: @RegisterWidget public class LassoHud extends ElementBasedWidget
#[derive(Clone)]
pub struct LassoHud {
    /// java: extends ElementBasedWidget
    pub base: ElementBasedWidgetBase,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl std::ops::Deref for LassoHud {
    type Target = ElementBasedWidgetBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for LassoHud {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl LassoHud {
    /// java: public LassoHud()
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = ElementBasedWidgetBase::new_in(component::literal("Lasso").with_style_formats(&[ChatFormatting::DarkAqua, ChatFormatting::Bold]), mcrust::net::minecraft::network::chat::text_color::TextColor::DARK_AQUA.get_value(), Information::new_4("hud_lasso".to_string(), component::literal("Lasso HUD"), Location::Galatea, vec![Location::TorrhusCanyon]), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                self_ref: std::sync::Weak::new(),
            });
            compat::write(&__h, |__t| __t.self_ref = std::sync::Arc::downgrade(&__h));
            ((__h.clone() as compat::Shared<dyn ElementBasedWidget>), __h)
        });
        *INSTANCE.get_mut() = Some(__this.clone());
        __this
    }

    /// java: public static LassoHud getInstance()
    pub fn get_instance() -> compat::Shared<LassoHud> {
        return INSTANCE.get_mut().clone().unwrap();
    }

    /// java: public static void onEntityUpdate(ArmorStand entity)
    pub fn on_entity_update(entity: &ArmorStand) {
        if !(WidgetManager::is_widget_in_current_screen(&(LassoHud::get_instance() as compat::Shared<dyn HudWidget>))) || LASSO_ENTITY.get_mut().is_none() || entity.base.base.distance_to_sqr_entity(LASSO_ENTITY.get_mut().as_ref().unwrap()) > (16 as f64) {
            return;
        }
        let mut name: Option<MutableComponent> = entity.base.base.get_custom_name();
        if let Some(name) = &name {
            if name.get_string() == LASSO_COUNT_DOWN_NAME && (name.get_siblings().len() as i32) == 2 {
                let mut new_percentage: i32 = ((((((compat::str_len(&name.get_siblings()[0].get_string())).wrapping_sub(compat::get_state(&REEL_VALUE)) as f32) / (20.0f32 - (compat::get_state(&REEL_VALUE) as f32))) * (100 as f32))) as i32);
                compat::set_state(&PERCENTAGE, new_percentage.max(0));
            }
        }
    }

    /// java: public static void onEntityAttach(ClientboundSetEntityLinkPacket packet)
    pub fn on_entity_attach(packet: &ClientboundSetEntityLinkPacket) {
        if !(WidgetManager::is_widget_in_current_screen(&(LassoHud::get_instance() as compat::Shared<dyn HudWidget>))) || mcrust::mcrust::modding::game::level().is_none() {
            return;
        }
        // TODO(fabric2seer): instanceof an interface (`Player`): Rust cannot test for a trait at run time
        //   java: if (CLIENT.level.getEntity(packet.getDestId()) instanceof Player player) {
        //   java: 			if (player.equals(CLIENT.player)) {
        //   java: 				//save lasso end entity
        //   java: 				lassoEntity = CLIENT.level.getEntity(packet.getSourceId());
        //   java:
        //   java: 				//get rarity of lasso used
        //   java: 				String usedItemId = player.getMainHandItem().getSkyblockId();
        //   java:
        //   java: 				reelValue = switch (usedItemId) {
        //   java: 					case "ABYSMAL_LASSO" -> 2;
        //   java: 					case "VINERIP_LASSO", "ENTANGLER_LASSO" -> 3;
        //   java: 					case "EVERSTRETCH_LASSO" -> 4;
        //   java: 					//Moody Grappleshot is a thing and we don't want to throw (crash) when it is used.
        //   java: 					default -> 0;
        //   java: 				};
        //   java:
        //   java: 				//reset percentage
        //   java: 				percentage = -1;
        //   java: 			}
        //   java: 		}
    }

}

impl ElementBasedWidget for LassoHud {
    fn element_based_widget_base(&self) -> &ElementBasedWidgetBase {
        &self.base
    }
    fn element_based_widget_base_mut(&mut self) -> &mut ElementBasedWidgetBase {
        &mut self.base
    }

    /// java: @Override public boolean shouldUpdateBeforeRendering() (dispatch: the body for this class)
    fn should_update_before_rendering(&self) -> fn(&compat::Shared<dyn ElementBasedWidget>) -> bool {
        LassoHud::should_update_before_rendering_dyn
    }

    /// java: public void updateContent() (dispatch: the body for this class)
    fn update_content(&self) -> fn(&compat::Shared<dyn ElementBasedWidget>) {
        LassoHud::update_content_dyn
    }

}

impl HudWidget for LassoHud {
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
        LassoHud::should_render_dyn
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

impl ElementCollector for LassoHud {
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

impl LassoHud {
    /// java: @Override public boolean shouldUpdateBeforeRendering()
    pub fn should_update_before_rendering_impl(__this: &compat::Shared<Self>) -> bool {
        return true;
    }

    /// LassoHud.shouldUpdateBeforeRendering as the dispatch of ElementBasedWidget.shouldUpdateBeforeRendering
    pub fn should_update_before_rendering_dyn(__this: &compat::Shared<dyn ElementBasedWidget>) -> bool {
        Self::should_update_before_rendering_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: public void updateContent()
    pub fn update_content_impl(__this: &compat::Shared<Self>) {
        if compat::get_state(&PERCENTAGE) == 0 {
            { let (__a0,) = (Elements::progress_component_flexible_item_stack_component_float_int(Some(ico::LEAD.get_mut().clone()), &component::translatable("skyblocker.config.hunting.lassoHud.reel"), component::translatable("skyblocker.config.hunting.lassoHud.now").with_style_format(ChatFormatting::Green), compat::get_state(&PERCENTAGE)),); __this.lock().borrow_mut().add_element(__a0) };
            return;
        }
        { let (__a0,) = (Elements::progress_component_flexible_item_stack_component_float_int(Some(ico::LEAD.get_mut().clone()), &component::translatable("skyblocker.config.hunting.lassoHud.reel"), component::translatable("skyblocker.config.hunting.lassoHud.wait"), compat::get_state(&PERCENTAGE)),); __this.lock().borrow_mut().add_element(__a0) };
    }

    /// LassoHud.updateContent as the dispatch of ElementBasedWidget.updateContent
    pub fn update_content_dyn(__this: &compat::Shared<dyn ElementBasedWidget>) {
        Self::update_content_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public boolean shouldRender()
    pub fn should_render_impl(__this: &compat::Shared<Self>) -> bool {
        if LASSO_ENTITY.get_mut().is_some() && !(LASSO_ENTITY.get_mut().as_ref().unwrap().is_alive()) {
            *LASSO_ENTITY.get_mut() = None;
        }
        return (compat::get_state(&PERCENTAGE) != -1 && LASSO_ENTITY.get_mut().is_some());
    }

    /// LassoHud.shouldRender as the dispatch of HudWidget.shouldRender
    pub fn should_render_dyn(__this: &compat::Shared<dyn HudWidget>) -> bool {
        Self::should_render_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}
