//! Ported from de.hysky.skyblocker.skyblock.item.custom.screen.TrimElementButton by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::item_utils;
use crate::de::hysky::skyblocker::utils::item_utils::ItemUtils;
use crate::de::hysky::skyblocker::utils::registry_utils::RegistryUtils;
use crate::de::hysky::skyblocker::utils::render::gui_helper::GuiHelper;
use mcrust::mcrust::modding::widgets::ModWidget;
use mcrust::mcrust::modding::widgets::WidgetRenderContext;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::abstract_widget::WidgetKind;
use mcrust::net::minecraft::client::gui::components::tooltip::Tooltip;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::input_with_modifiers::InputWithModifiers;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::client::input::mouse_button_info::MouseButtonInfo;
use mcrust::net::minecraft::client::model::humanoid_model::HumanoidModel;
use mcrust::net::minecraft::client::model::player::player_model::PlayerModel;
use mcrust::net::minecraft::client::renderer::entity::armor_model_set::ArmorModelSet;
use mcrust::net::minecraft::client::renderer::entity::layers::equipment_layer_renderer::EquipmentLayerRenderer;
use mcrust::net::minecraft::client::renderer::entity::state::avatar_render_state::AvatarRenderState;
use mcrust::net::minecraft::client::resources::model::equipment_client_info::EquipmentClientInfo;
use mcrust::net::minecraft::client::resources::model::equipment_client_info::LayerType;
use mcrust::net::minecraft::core::holder::Holder;
use mcrust::net::minecraft::nbt::compound_tag;
use mcrust::net::minecraft::nbt::tag::Tag;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::world::entity::equipment_slot::EquipmentSlot;
use mcrust::net::minecraft::world::item::component::custom_data::CustomData;
use mcrust::net::minecraft::world::item::equipment::equippable::Equippable;
use mcrust::net::minecraft::world::item::equipment::trim::armor_trim::ArmorTrim;
use mcrust::net::minecraft::world::item::equipment::trim::trim_material::TrimMaterial;
use mcrust::net::minecraft::world::item::equipment::trim::trim_pattern::TrimPattern;
use mcrust::net::minecraft::world::item::items;
use mcrust::net::minecraft::core::holder_getter::HolderGetter as _;
use mcrust::net::minecraft::core::registry::Registry as _;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final ItemStack BARRIER = new ItemStack(Items.BARRIER);
pub static BARRIER: compat::MainThread<mcrust::net::minecraft::world::item::item_stack::ItemStack> = compat::MainThread::new(|| mcrust::net::minecraft::world::item::item_stack::ItemStack::of_one(items::items().get("barrier")));
/// java: private static final int DEFAULT_ROTATION = 15;
pub const PATTERN_DEFAULT_ROTATION: i32 = 15;
/// java: private static ArmorModelSet<? extends HumanoidModel<?>> equipmentModelData = null;
pub static PATTERN_EQUIPMENT_MODEL_DATA: compat::MainThread<Option<ArmorModelSet<HumanoidModel>>> = compat::MainThread::new(|| None);
/// java: private static EquipmentLayerRenderer equipmentRenderer = null;
pub static PATTERN_EQUIPMENT_RENDERER: compat::MainThread<Option<EquipmentLayerRenderer>> = compat::MainThread::new(|| None);

/// java: public abstract sealed class TrimElementButton extends AbstractButton permits TrimElementButton.Pattern, TrimElementButton.Material
pub struct TrimElementButtonBase {
    /// java: extends net.minecraft.client.gui.components.AbstractButton
    pub base: AbstractWidget,
    /// java: protected final @Nullable Identifier element;
    pub element: Option<Identifier>,
    /// java: protected ItemStack stack;
    pub stack: Option<mcrust::net::minecraft::world::item::item_stack::ItemStack>,
    /// java: private final Consumer<TrimElementButton> onPress;
    pub on_press: Arc<dyn Fn(compat::Shared<dyn TrimElementButton>)>,
    /// `AbstractButton.extractDefaultSprite` - the game's button sprite pass (see `compat::ButtonLook`)
    pub look: compat::ButtonLook,
}

impl std::ops::Deref for TrimElementButtonBase {
    type Target = AbstractWidget;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for TrimElementButtonBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl TrimElementButtonBase {
    /// java: public TrimElementButton(@Nullable Identifier element, Component name, Consumer<TrimElementButton> onPress) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_in<__R>(mut element: Option<Identifier>, mut name: MutableComponent, on_press: Arc<dyn Fn(compat::Shared<dyn TrimElementButton>)>, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn TrimElementButton>, __R)) -> (compat::Shared<dyn TrimElementButton>, __R) {
        let __init_element = element.clone();
        let __init_on_press = on_press.clone();
        let this = Self {
            base: AbstractWidget::new(0, 0, 20, 20, (&name.get_string()).to_string(), WidgetKind::PlainText, OnPress::Inert),
            element: __init_element,
            stack: None,
            on_press: __init_on_press,
            look: compat::ButtonLook::new(),
        };
        let (__h, __r) = __wrap(this);
        let __this = __h;
        { let (__q0,) = (Tooltip::create(compat::component_of(__this.lock().borrow().trim_element_button_base().get_message_component()).get_string()),); __this.lock().borrow_mut().trim_element_button_base_mut().set_tooltip(__q0) };
        (__this, __r)
    }

}

/// java: the virtual methods of TrimElementButton (implemented by TrimElementButtonBase and every subclass)
pub trait TrimElementButton: compat::DynAny {
    fn trim_element_button_base(&self) -> &TrimElementButtonBase;
    fn trim_element_button_base_mut(&mut self) -> &mut TrimElementButtonBase;
    fn as_any(&self) -> &dyn std::any::Any;

    /// java: public @Nullable Identifier getElement() (dispatch: the body for this class)
    fn get_element(&self) -> fn(&compat::Shared<dyn TrimElementButton>) -> Option<Identifier> {
        <dyn TrimElementButton>::get_element_body
    }

    /// java: @Override public void setMessage(Component message) (dispatch: the body for this class)
    fn set_message(&self) -> fn(&compat::Shared<dyn TrimElementButton>, &MutableComponent) {
        <dyn TrimElementButton>::set_message_body
    }

    /// java: @Override public void extractContents(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a) (dispatch: the body for this class)
    fn extract_contents(&self) -> fn(&compat::Shared<dyn TrimElementButton>, &AbstractWidget, &mut WidgetRenderContext<'_>, i32, i32, f32) {
        <dyn TrimElementButton>::extract_contents_body
    }

    /// java: abstract void extract(GuiGraphicsExtractor graphics); (dispatch: the body for this class)
    fn extract(&self) -> fn(&compat::Shared<dyn TrimElementButton>, &GuiGraphicsExtractor);

    /// java: @Override public void onPress(InputWithModifiers input) (dispatch: the body for this class)
    fn on_press(&self) -> fn(&compat::Shared<dyn TrimElementButton>, &dyn InputWithModifiers) {
        <dyn TrimElementButton>::on_press_body
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder) (dispatch: the body for this class)
    fn update_widget_narration(&self) -> fn(&compat::Shared<dyn TrimElementButton>, &dyn NarrationElementOutput) {
        <dyn TrimElementButton>::update_widget_narration_body
    }

}

impl dyn TrimElementButton {
    /// java: public @Nullable Identifier getElement()
    pub fn get_element_body(__this: &compat::Shared<dyn TrimElementButton>) -> Option<Identifier> {
        return compat::read(__this, |__t| __t.trim_element_button_base().element.clone());
    }

    /// java: @Override public void setMessage(Component message)
    pub fn set_message_body(__this: &compat::Shared<dyn TrimElementButton>, message: &MutableComponent) {
        { let (__q0,) = (mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(message.clone()),); __this.lock().borrow_mut().trim_element_button_base_mut().base.set_message_text(__q0) };
        { let (__q0,) = (Tooltip::create(compat::component_of(__this.lock().borrow().trim_element_button_base().get_message_component()).get_string()),); __this.lock().borrow_mut().trim_element_button_base_mut().set_tooltip(__q0) };
    }

    /// java: @Override public void extractContents(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    /// (`widget` / `context`: the game's widget and the frame, which the default sprite is drawn through)
    pub fn extract_contents_body(__this: &compat::Shared<dyn TrimElementButton>, widget: &AbstractWidget, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        let _ = a;
        // java: this.extractDefaultSprite(graphics);
        compat::write(__this, |__t| __t.trim_element_button_base_mut().look.draw_sprite(widget, context, mouse_x, mouse_y));
        ({ let __f = compat::dispatch(__this, |__t| __t.extract()); __f(__this, &context.graphics) });
    }

    /// java: @Override public void onPress(InputWithModifiers input)
    pub fn on_press_body(__this: &compat::Shared<dyn TrimElementButton>, input: &dyn InputWithModifiers) {
        ((compat::read(__this, |__t| __t.trim_element_button_base().on_press.clone())))(__this.clone());
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    pub fn update_widget_narration_body(__this: &compat::Shared<dyn TrimElementButton>, builder: &dyn NarrationElementOutput) {
    }

}

/// java: public static final class Pattern extends TrimElementButton
pub struct Pattern {
    /// java: extends TrimElementButton
    pub base: TrimElementButtonBase,
    /// java: private final ArmorTrim trim;
    pub trim: Option<ArmorTrim>,
    /// java: private Equippable equippableComponent;
    pub equippable_component: Option<Equippable>,
    /// java: private float rotation = DEFAULT_ROTATION;
    pub rotation: f32,
}

impl std::ops::Deref for Pattern {
    type Target = TrimElementButtonBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for Pattern {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl Pattern {
    /// java: public Pattern(@Nullable Identifier element, @Nullable TrimPattern pattern, Consumer<TrimElementButton> onPress)
    pub fn new(mut element: Option<Identifier>, mut pattern: Option<TrimPattern>, on_press: Arc<dyn Fn(compat::Shared<dyn TrimElementButton>)>) -> compat::Shared<Self> {
        let (_, __this) = TrimElementButtonBase::new_in(element.clone(), (if pattern.is_none() { component::translatable("gui.none") } else { compat::component_of(pattern.as_ref().unwrap().description().clone()) }), on_press.clone(), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                trim: None,
                equippable_component: None,
                rotation: ((PATTERN_DEFAULT_ROTATION) as f32) as f32,
            });
            ((__h.clone() as compat::Shared<dyn TrimElementButton>), __h)
        });
        let Some(mut element) = element.clone() else {
            { let __v = None; compat::write(&__this, |__t| __t.trim = __v); }
            return __this;
        };
        if PATTERN_EQUIPMENT_MODEL_DATA.get_mut().is_none() {
            // TODO(fabric2seer): `Minecraft.getEntityModels` has no Rust method in the port (looked for `get_entity_models` in Minecraft and its supertypes)
            // TODO(fabric2seer): static `ArmorModelSet.bake` has no Rust counterpart in the port
            // rust: *PATTERN_EQUIPMENT_MODEL_DATA.get_mut() = Some(ArmorModelSet::bake(mcrust::net::minecraft::client::model::geom::model_layers::player_armor().clone(), compat::minecraft().get_entity_models(), Arc::new(move |model_part| PlayerModel::new(model_part.clone(), false))));
            // TODO(fabric2seer): `Minecraft.getAtlasManager` has no Rust method in the port (looked for `get_atlas_manager` in Minecraft and its supertypes)
            // rust: *PATTERN_EQUIPMENT_RENDERER.get_mut() = Some(EquipmentLayerRenderer::new(compat::minecraft().get_entity_render_dispatcher().equipment_assets, Some(compat::minecraft().get_atlas_manager().get_atlas_or_throw(&mcrust::net::minecraft::data::atlas_ids::armor_trims()))));
        }
        // TODO(fabric2seer): `Provider.lookupOrThrow` has no Rust method in the port (looked for `lookup_or_throw` in HolderLookup and its supertypes)
        // rust: { let __v = Some(ArmorTrim::new(RegistryUtils::get_registry_wrapper_lookup().lookup_or_throw(&mcrust::net::minecraft::core::registries::registries::trim_material()).get_or_throw(&mcrust::net::minecraft::world::item::equipment::trim::trim_materials::quartz()).unwrap(), Holder::direct(pattern.clone().unwrap()))); compat::write(&__this, |__t| __t.trim = __v); }
        __this
    }

    /// java: @SuppressWarnings("incomplete-switch") private static float setVisibleAndGetOffset(HumanoidModel<?> humanoidModel, EquipmentSlot slot)
    /// (`humanoidModel.head.visible = ..` - the port reaches a model's part by its field path, `part_fast_mut`)
    pub fn set_visible_and_get_offset(humanoid_model: &mut HumanoidModel, slot: &EquipmentSlot) -> f32 {
        use mcrust::net::minecraft::client::model::humanoid_model::HumanoidPart;
        humanoid_model.part_fast_mut(HumanoidPart::Head).visible = false;
        humanoid_model.part_fast_mut(HumanoidPart::Hat).visible = false;
        humanoid_model.part_fast_mut(HumanoidPart::Body).visible = false;
        humanoid_model.part_fast_mut(HumanoidPart::RightArm).visible = false;
        humanoid_model.part_fast_mut(HumanoidPart::LeftArm).visible = false;
        humanoid_model.part_fast_mut(HumanoidPart::RightLeg).visible = false;
        humanoid_model.part_fast_mut(HumanoidPart::LeftLeg).visible = false;
        match *slot {
            EquipmentSlot::Head => {
                humanoid_model.part_fast_mut(HumanoidPart::Head).visible = true;
                humanoid_model.part_fast_mut(HumanoidPart::Hat).visible = true;
                return ((4) as f32) as f32;
            }
            EquipmentSlot::Chest => {
                humanoid_model.part_fast_mut(HumanoidPart::Body).visible = true;
                humanoid_model.part_fast_mut(HumanoidPart::RightArm).visible = true;
                humanoid_model.part_fast_mut(HumanoidPart::LeftArm).visible = true;
                return ((-6) as f32) as f32;
            }
            EquipmentSlot::Legs => {
                humanoid_model.part_fast_mut(HumanoidPart::Body).visible = true;
                humanoid_model.part_fast_mut(HumanoidPart::RightLeg).visible = true;
                humanoid_model.part_fast_mut(HumanoidPart::LeftLeg).visible = true;
                return ((-14) as f32) as f32;
            }
            EquipmentSlot::Feet => {
                humanoid_model.part_fast_mut(HumanoidPart::RightLeg).visible = true;
                humanoid_model.part_fast_mut(HumanoidPart::LeftLeg).visible = true;
                return ((-20) as f32) as f32;
            }
            _ => {}
        }
        return ((0) as f32) as f32;
    }

    /// java: public void setStack(ItemStack newStack)
    pub fn set_stack(__this: &compat::Shared<Self>, mut new_stack: mcrust::net::minecraft::world::item::item_stack::ItemStack) {
        new_stack = new_stack.copy();
        // Remove the uuid so it doesn't render with the selected trim
        // CompoundTag nbtCopy = ItemUtils.getCustomData(newStack).copy();
        let mut nbt_copy: Tag = crate::de::hysky::skyblocker::utils::item_utils::ItemUtils::get_custom_data(&new_stack).copy();
        nbt_copy.remove(item_utils::UUID);
        new_stack.set(mcrust::net::minecraft::core::component::data_components::custom_data().clone(), Some(mcrust::com::mojang::serialization::obj(CustomData::of(&nbt_copy))));
        new_stack.set(mcrust::net::minecraft::core::component::data_components::trim().clone(), compat::read(__this, |__t| __t.trim.clone()).map(mcrust::com::mojang::serialization::obj));
        { let __v = <_ as mcrust::net::minecraft::core::component::data_component_map::DataComponentGetter>::get(&new_stack, mcrust::net::minecraft::core::component::data_components::equippable()).and_then(|__o| mcrust::com::mojang::serialization::cast::<mcrust::net::minecraft::world::item::equipment::equippable::Equippable>(&__o).cloned()); compat::write(__this, |__t| __t.equippable_component = __v); }
        if compat::read(__this, |__t| __t.equippable_component.clone()).is_none() {
            panic!("IllegalArgumentException: {}", "Trimmed stack must contain an equippable component");
        }
        { let __v = Some(new_stack.clone()); compat::write(__this, |__t| __t.stack = __v); }
    }

}

impl TrimElementButton for Pattern {
    fn trim_element_button_base(&self) -> &TrimElementButtonBase {
        &self.base
    }
    fn trim_element_button_base_mut(&mut self) -> &mut TrimElementButtonBase {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override void extract(GuiGraphicsExtractor graphics) (dispatch: the body for this class)
    fn extract(&self) -> fn(&compat::Shared<dyn TrimElementButton>, &GuiGraphicsExtractor) {
        Pattern::extract_dyn
    }

}

impl Pattern {
    /// java: @Override void extract(GuiGraphicsExtractor graphics)
    pub fn extract_impl(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor) {
        if compat::read(__this, |__t| __t.trim.clone()).is_none() {
            graphics.item(BARRIER.get_mut(), ((__this.lock().borrow().get_x()).wrapping_add((__this.lock().borrow().get_width()).wrapping_div(2))).wrapping_sub(8), ((__this.lock().borrow().get_y()).wrapping_add((__this.lock().borrow().get_height()).wrapping_div(2))).wrapping_sub(8));
            return;
        }
        if __this.lock().borrow().is_hovered() {
            // java: rotation += Minecraft.getInstance().getDeltaTracker().getGameTimeDeltaTicks() * 0.05f * 90;
            { let __v = (compat::read(__this, |__t| __t.rotation) + ((compat::minecraft().delta_tracker.get_game_time_delta_ticks() * 0.05f32) * (90 as f32))); compat::write(__this, |__t| __t.rotation = __v); }
            { let __v = (compat::read(__this, |__t| __t.rotation) % (360 as f32)); compat::write(__this, |__t| __t.rotation = __v); }
        } else {
            { let __v = ((PATTERN_DEFAULT_ROTATION) as f32) as f32; compat::write(__this, |__t| __t.rotation = __v); }
        }
        let mut slot: EquipmentSlot = *__this.lock().borrow().equippable_component.as_ref().unwrap().slot();
        // java: HumanoidModel<AvatarRenderState> model = (HumanoidModel<AvatarRenderState>) equipmentModelData.get(slot);
        // rust: port gap - `equipmentModelData` is baked from `Minecraft.getEntityModels()`, which the port does not lend a
        //   mod (see `Pattern::new`), so it stays null here and the trimmed piece cannot be drawn (GAPS_batchE.md)
        let mut __model_data = PATTERN_EQUIPMENT_MODEL_DATA.get_mut();
        let Some(__model_data) = __model_data.as_mut() else {
            return;
        };
        let model = __model_data.get_mut(slot);
        // java: AvatarRenderState state = new AvatarRenderState(); - the state the model is posed with (held by reference)
        let state: std::rc::Rc<AvatarRenderState> = std::rc::Rc::new(AvatarRenderState::default());
        // java: EquipmentClientInfo.LayerType layerType = slot == EquipmentSlot.LEGS ? EquipmentClientInfo.LayerType.HUMANOID_LEGGINGS : EquipmentClientInfo.LayerType.HUMANOID;
        let layer_type: LayerType = if slot == EquipmentSlot::Legs { LayerType::HumanoidLeggings } else { LayerType::Humanoid };
        let mut offset: f32 = Pattern::set_visible_and_get_offset(model, &slot);
        GuiHelper::equipment(graphics, PATTERN_EQUIPMENT_RENDERER.get_mut().as_ref().unwrap(), &layer_type, &__this.lock().borrow().equippable_component.as_ref().unwrap().asset_id().cloned().unwrap_or_else(mcrust::net::minecraft::world::item::equipment::equipment_assets::iron), &model.base, &state, &compat::read(__this, |__t| __t.stack.clone()).unwrap(), __this.lock().borrow().get_x(), __this.lock().borrow().get_y(), (__this.lock().borrow().get_x()).wrapping_add(__this.lock().borrow().get_width()), (__this.lock().borrow().get_y()).wrapping_add(__this.lock().borrow().get_height()), compat::read(__this, |__t| __t.rotation), ((14) as f32) as f32, offset);
    }

    /// Pattern.extract as the dispatch of TrimElementButton.extract
    pub fn extract_dyn(__this: &compat::Shared<dyn TrimElementButton>, __p0: &GuiGraphicsExtractor) {
        Self::extract_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

}

/// Pattern's widget half (`super`'s fields: x, y, size, message, ...), lent by the game for each call
impl compat::WidgetHalf for Pattern {
    fn widget_half(&mut self) -> &mut AbstractWidget {
        &mut self.base.base
    }
}

/// Pattern as the game's widget: `compat::WidgetAdapter::widget(handle)` where Java hands it on
impl ModWidget for compat::WidgetAdapter<Pattern> {
    /// java: AbstractButton.extractWidgetRenderState - `extractContents(..)`, then `handleCursor(graphics)`
    fn extract_widget_render_state(&mut self, widget: &AbstractWidget, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        self.lend_ref(widget, |__h| ({ let __f = compat::dispatch(__h, |__t| __t.extract_contents()); __f(&compat::view(__h, |a| a as compat::Shared<dyn TrimElementButton>), widget, context, mouse_x, mouse_y, a) }))
    }

    /// java: AbstractButton.keyPressed - a selection key presses an active button
    fn key_pressed(&mut self, widget: &mut AbstractWidget, event: &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        if !widget.is_active() || !event.is_selection() {
            return false;
        }
        self.lend(widget, |__h| ({ let __f = compat::dispatch(__h, |__t| __t.on_press()); __f(&compat::view(__h, |a| a as compat::Shared<dyn TrimElementButton>), event) }));
        true
    }

    /// java: AbstractButton.extractWidgetRenderState's `handleCursor(graphics)` - `AbstractWidget.handleCursor:68-72`
    fn handle_cursor(&self, widget: &AbstractWidget, cursors: &mcrust::com::mojang::blaze3d::platform::cursor::cursor_types::CursorTypes) -> Option<mcrust::com::mojang::blaze3d::platform::cursor::cursor_type::CursorType> {
        widget.handle_cursor(cursors)
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    fn update_widget_narration(&mut self, widget: &AbstractWidget, output: &mut dyn NarrationElementOutput) {
        self.lend_ref(widget, |__h| ({ let __f = compat::dispatch(__h, |__t| __t.update_widget_narration()); __f(&compat::view(__h, |a| a as compat::Shared<dyn TrimElementButton>), output) }))
    }

    /// java: @Override public void onPress(InputWithModifiers input)
    fn on_click(&mut self, widget: &mut AbstractWidget, event: &MouseButtonEvent, double_click: bool) {
        self.lend(widget, |__h| ({ let __f = compat::dispatch(__h, |__t| __t.on_press()); __f(&compat::view(__h, |a| a as compat::Shared<dyn TrimElementButton>), event) }))
    }

}

/// java: public static final class Material extends TrimElementButton
pub struct Material {
    /// java: extends TrimElementButton
    pub base: TrimElementButtonBase,
}

impl std::ops::Deref for Material {
    type Target = TrimElementButtonBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for Material {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl Material {
    /// java: public Material(Identifier element, TrimMaterial material, Consumer<TrimElementButton> onPress)
    pub fn new(mut element: Identifier, mut material: TrimMaterial, on_press: Arc<dyn Fn(compat::Shared<dyn TrimElementButton>)>) -> compat::Shared<Self> {
        let (_, __this) = TrimElementButtonBase::new_in(Some(element.clone()), compat::component_of(material.description().clone()), on_press.clone(), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
            });
            ((__h.clone() as compat::Shared<dyn TrimElementButton>), __h)
        });
        // TODO(fabric2seer): `.components(..)` on a T
        //   java: stack = BuiltInRegistries.ITEM.stream()
        //   java: 					.filter(item -> Optional.ofNullable(item.components().get(DataComponents.PROVIDES_TRIM_MATERIAL))
        //   java: 							.map(provided -> provided.is(element))
        //   java: 							.orElse(false)
        //   java: 					)
        //   java: 					.findAny()
        //   java: 					.map(ItemStack::new)
        //   java: 					.orElse(BARRIER);
        __this
    }

}

impl TrimElementButton for Material {
    fn trim_element_button_base(&self) -> &TrimElementButtonBase {
        &self.base
    }
    fn trim_element_button_base_mut(&mut self) -> &mut TrimElementButtonBase {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override void extract(GuiGraphicsExtractor graphics) (dispatch: the body for this class)
    fn extract(&self) -> fn(&compat::Shared<dyn TrimElementButton>, &GuiGraphicsExtractor) {
        Material::extract_dyn
    }

}

impl Material {
    /// java: @Override void extract(GuiGraphicsExtractor graphics)
    pub fn extract_impl(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor) {
        graphics.item(&compat::read(__this, |__t| __t.stack.clone()).unwrap(), ((__this.lock().borrow().get_x()).wrapping_add((__this.lock().borrow().get_width()).wrapping_div(2))).wrapping_sub(8), ((__this.lock().borrow().get_y()).wrapping_add((__this.lock().borrow().get_height()).wrapping_div(2))).wrapping_sub(8));
    }

    /// Material.extract as the dispatch of TrimElementButton.extract
    pub fn extract_dyn(__this: &compat::Shared<dyn TrimElementButton>, __p0: &GuiGraphicsExtractor) {
        Self::extract_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

}

/// Material's widget half (`super`'s fields: x, y, size, message, ...), lent by the game for each call
impl compat::WidgetHalf for Material {
    fn widget_half(&mut self) -> &mut AbstractWidget {
        &mut self.base.base
    }
}

/// Material as the game's widget: `compat::WidgetAdapter::widget(handle)` where Java hands it on
impl ModWidget for compat::WidgetAdapter<Material> {
    /// java: AbstractButton.extractWidgetRenderState - `extractContents(..)`, then `handleCursor(graphics)`
    fn extract_widget_render_state(&mut self, widget: &AbstractWidget, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        self.lend_ref(widget, |__h| ({ let __f = compat::dispatch(__h, |__t| __t.extract_contents()); __f(&compat::view(__h, |a| a as compat::Shared<dyn TrimElementButton>), widget, context, mouse_x, mouse_y, a) }))
    }

    /// java: AbstractButton.keyPressed - a selection key presses an active button
    fn key_pressed(&mut self, widget: &mut AbstractWidget, event: &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        if !widget.is_active() || !event.is_selection() {
            return false;
        }
        self.lend(widget, |__h| ({ let __f = compat::dispatch(__h, |__t| __t.on_press()); __f(&compat::view(__h, |a| a as compat::Shared<dyn TrimElementButton>), event) }));
        true
    }

    /// java: AbstractButton.extractWidgetRenderState's `handleCursor(graphics)` - `AbstractWidget.handleCursor:68-72`
    fn handle_cursor(&self, widget: &AbstractWidget, cursors: &mcrust::com::mojang::blaze3d::platform::cursor::cursor_types::CursorTypes) -> Option<mcrust::com::mojang::blaze3d::platform::cursor::cursor_type::CursorType> {
        widget.handle_cursor(cursors)
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    fn update_widget_narration(&mut self, widget: &AbstractWidget, output: &mut dyn NarrationElementOutput) {
        self.lend_ref(widget, |__h| ({ let __f = compat::dispatch(__h, |__t| __t.update_widget_narration()); __f(&compat::view(__h, |a| a as compat::Shared<dyn TrimElementButton>), output) }))
    }

    /// java: @Override public void onPress(InputWithModifiers input)
    fn on_click(&mut self, widget: &mut AbstractWidget, event: &MouseButtonEvent, double_click: bool) {
        self.lend(widget, |__h| ({ let __f = compat::dispatch(__h, |__t| __t.on_press()); __f(&compat::view(__h, |a| a as compat::Shared<dyn TrimElementButton>), event) }))
    }

}
