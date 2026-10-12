//! Ported from de.hysky.skyblocker.utils.datafixer.LegacyItemStackFixer by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::registry_utils::RegistryUtils;
use crate::de::hysky::skyblocker::utils::text_transformer::TextTransformer;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::com::mojang::serialization::dynamic::Dynamic;
use mcrust::net::minecraft::core::component::data_component_type::DataComponentType;
use mcrust::net::minecraft::core::holder_lookup;
use mcrust::net::minecraft::nbt::tag::Tag;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::network::chat::formatted_text::FormattedText;
use mcrust::net::minecraft::core::component::data_components;
use mcrust::net::minecraft::resources::registry_ops::RegistryOps;
use mcrust::net::minecraft::world::item::component::custom_data::CustomData;
use mcrust::net::minecraft::world::item::component::item_lore::ItemLore;
use mcrust::net::minecraft::world::item::component::tooltip_display::TooltipDisplay;
use mcrust::com::mojang::serialization::decoder::Decoder as _;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: `<T extends ItemInstance>` in `fixLegacyStack` - the two `ItemInstance` members it reads
/// (`TypedInstance.is(Item)`, `DataComponentGetter.get`) for the two `T`s Java passes: `ItemStack` and
/// `FlexibleItemStack` (a `compat::Shared` handle in this port, which the game's traits cannot be implemented on).
pub trait LegacyFixableStack: Clone + 'static {
    /// java: `TypedInstance.is(T rawType)` = `typeHolder().value() == rawType`
    fn is_item(&self, item: &Arc<mcrust::net::minecraft::world::item::item::Item>) -> bool;
    /// java: `DataComponentGetter.get(type)`
    fn get_component(&self, r#type: &Arc<DataComponentType>) -> Option<mcrust::com::mojang::serialization::Obj>;
}

impl LegacyFixableStack for mcrust::net::minecraft::world::item::item_stack::ItemStack {
    fn is_item(&self, item: &Arc<mcrust::net::minecraft::world::item::item::Item>) -> bool {
        // `ItemStack.typeHolder()` = `isEmpty() ? Items.AIR.builtInRegistryHolder() : item`, which `get_item` answers
        Arc::ptr_eq(&self.get_item(), item)
    }

    fn get_component(&self, r#type: &Arc<DataComponentType>) -> Option<mcrust::com::mojang::serialization::Obj> {
        mcrust::net::minecraft::core::component::data_component_getter::DataComponentGetter::get(self, r#type)
    }
}

impl LegacyFixableStack for compat::Shared<crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack> {
    fn is_item(&self, item: &Arc<mcrust::net::minecraft::world::item::item::Item>) -> bool {
        use crate::de::hysky::skyblocker::utils::flexible_item_stack::{FlexibleItemStack, item_of};
        Arc::ptr_eq(&item_of(&FlexibleItemStack::type_holder(self)), item)
    }

    fn get_component(&self, r#type: &Arc<DataComponentType>) -> Option<mcrust::com::mojang::serialization::Obj> {
        compat::read(self, |s| mcrust::net::minecraft::core::component::data_component_getter::DataComponentGetter::get(s, r#type))
    }
}

/// java: public class LegacyItemStackFixer
#[derive(Clone)]
pub struct LegacyItemStackFixer {}

impl LegacyItemStackFixer {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static <T extends ItemInstance> T fixLegacyStack(CompoundTag nbt, Codec<T> codec, T defaultValue, @SuppressWarnings("rawtypes") TriConsumer<T, DataComponentType, Object> setComponent)
    ///
    /// `TriConsumer<T, DataComponentType, Object>` is `&dyn Fn(&mut T, Arc<DataComponentType>, Obj)` (Java's callers
    /// pass `ItemStack::set` / `FlexibleItemStack::set`, which mutate the stack in place).
    pub fn fix_legacy_stack<T: LegacyFixableStack>(nbt: &Tag, codec: &CodecRef<T>, default_value: &T, set_component: &dyn Fn(&mut T, Arc<DataComponentType>, mcrust::com::mojang::serialization::Obj)) -> T {
        // java: RegistryOps<Tag> ops = RegistryUtils.getRegistryWrapperLookup().createSerializationContext(NbtOps.INSTANCE);
        let mut ops: mcrust::com::mojang::serialization::dynamic_ops::Ops = holder_lookup::create_serialization_context(RegistryUtils::get_registry_wrapper_lookup(), mcrust::net::minecraft::nbt::nbt_ops::NbtOps::instance());
        // java: Dynamic<Tag> fixed = getFixer().update(TypeReferences.LEGACY_ITEM_STACK, new Dynamic<>(ops, nbt), getFirstVersion(), getLatestVersion());
        // rust: port gap (GAPS_J #6) - `getFixer` / `getFirstVersion` / `getLatestVersion` / `TypeReferences.LEGACY_ITEM_STACK`
        // are the external Legacy Item DFU library (`net.azureaaron.legacyitemdfu`, its own pre-1.13 schemas and fixes),
        // which has no Rust port. The input half is `Dynamic::new_with(ops, Some(obj(nbt)))`.
        let _ = (&ops, nbt);
        let mut fixed: Dynamic = todo!("fabric2seer: the initialiser of `fixed` (TODO above)");
        let mut stack: T = codec.parse_dynamic(&fixed).result_or_partial_with(|__a0| { LegacyItemStackFixer::log(&__a0); }).unwrap_or(default_value.clone());
        // java: if (stack.is(Items.AIR)) return stack;
        if stack.is_item(&mcrust::net::minecraft::world::item::items::items().get("air")) {
            return stack;
        }
        // java: Component name = stack.get(DataComponents.CUSTOM_NAME);
        let mut name: Option<FormattedText> = stack.get_component(data_components::custom_name()).and_then(|o| mcrust::com::mojang::serialization::cast::<FormattedText>(&o).cloned());
        if let Some(name) = name {
            // java: setComponent.accept(stack, DataComponents.CUSTOM_NAME, TextTransformer.fromLegacy(name.getString()));
            set_component(&mut stack, data_components::custom_name().clone(), mcrust::com::mojang::serialization::obj(FormattedText::from(TextTransformer::from_legacy(&name.get_string()))));
        }
        // java: ItemLore lore = stack.get(DataComponents.LORE);
        let mut lore: Option<ItemLore> = stack.get_component(data_components::lore()).and_then(|o| mcrust::com::mojang::serialization::cast::<ItemLore>(&o).cloned());
        if let Some(lore) = lore {
            // java: List<Component> fixedLore = lore.lines().stream().map(Component::getString).map(TextTransformer::fromLegacy).map(Component.class::cast).toList();
            let mut fixed_lore: Vec<FormattedText> = lore.lines().iter().map(|line| line.get_string()).map(|line| FormattedText::from(TextTransformer::from_legacy(&line))).collect::<Vec<_>>();
            // java: setComponent.accept(stack, DataComponents.LORE, new ItemLore(fixedLore));
            set_component(&mut stack, data_components::lore().clone(), mcrust::com::mojang::serialization::obj(ItemLore::new(fixed_lore).expect("IllegalArgumentException: ItemLore")));
        }
        // java: CustomData customData = stack.get(DataComponents.CUSTOM_DATA);
        let mut custom_data: Option<CustomData> = stack.get_component(data_components::custom_data()).and_then(|o| mcrust::com::mojang::serialization::cast::<CustomData>(&o).cloned());
        if let Some(custom_data) = custom_data {
            // java: setComponent.accept(stack, DataComponents.CUSTOM_DATA, CustomData.of(customData.copyTag().getCompoundOrEmpty("ExtraAttributes")));
            set_component(&mut stack, data_components::custom_data().clone(), mcrust::com::mojang::serialization::obj(CustomData::of(&custom_data.copy_tag().get_compound_or_empty("ExtraAttributes"))));
        }
        // java: TooltipDisplay display = stack.getOrDefault(DataComponents.TOOLTIP_DISPLAY, TooltipDisplay.DEFAULT)
        // java: 				.withHidden(DataComponents.ATTRIBUTE_MODIFIERS, true)
        // java: 				.withHidden(DataComponents.ENCHANTMENTS, true);
        let mut display: TooltipDisplay = stack.get_component(data_components::tooltip_display()).and_then(|o| mcrust::com::mojang::serialization::cast::<TooltipDisplay>(&o).cloned()).unwrap_or_else(mcrust::net::minecraft::world::item::component::tooltip_display::default_display)
            .with_hidden(data_components::attribute_modifiers(), true)
            .with_hidden(data_components::enchantments(), true);
        // java: setComponent.accept(stack, DataComponents.TOOLTIP_DISPLAY, display);
        set_component(&mut stack, data_components::tooltip_display().clone(), mcrust::com::mojang::serialization::obj(display));
        return stack;
    }

    /// java: private static void log(String error)
    pub fn log(error: &str) {
        logger().error(&format!("[Skyblocker Legacy Item Fixer] Failed to fix up item! Error: {}", error));
    }

}
