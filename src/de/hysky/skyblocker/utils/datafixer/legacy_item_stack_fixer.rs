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
use mcrust::net::minecraft::resources::registry_ops::RegistryOps;
use mcrust::net::minecraft::world::item::component::custom_data::CustomData;
use mcrust::net::minecraft::world::item::component::item_lore::ItemLore;
use mcrust::net::minecraft::world::item::component::tooltip_display::TooltipDisplay;
use mcrust::com::mojang::serialization::decoder::Decoder as _;
use mcrust::net::minecraft::network::chat::component::Component as _;

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
    pub fn fix_legacy_stack<T: Clone + 'static>(nbt: &Tag, codec: &CodecRef<T>, default_value: &T, set_component: &mut crate::unresolved::TriConsumer) -> T {
        // TODO(fabric2seer): `create_serialization_context` takes 2 argument(s) in the port (`provider: Arc<dyn Provider>, parent: crate::com::mojang::serialization::dynamic_ops::Ops`), Java passes 1
        // rust: let mut ops: RegistryOps = holder_lookup::create_serialization_context(mcrust::net::minecraft::nbt::nbt_ops::NbtOps::instance());
        let mut ops: RegistryOps = todo!("fabric2seer: the initialiser of `ops` (TODO above)");
        // TODO(fabric2seer): static call LegacyItemStackFixer.getFixer(..)
        //   java: Dynamic<Tag> fixed = getFixer().update(TypeReferences.LEGACY_ITEM_STACK, new Dynamic<>(ops, nbt), getFirstVersion(), getLatestVersion());
        let mut fixed: Dynamic = todo!("fabric2seer: the initialiser of `fixed` (TODO above)");
        let mut stack: T = codec.parse_dynamic(&fixed).result_or_partial_with(|__a0| { LegacyItemStackFixer::log(&__a0); }).unwrap_or(default_value.clone());
        // TODO(fabric2seer): the condition: `.is(..)` on a T
        //   java: stack.is(..)
        if todo!("fabric2seer: the condition (TODO above)") {
            return stack;
        }
        // TODO(fabric2seer): `.get(..)` on a T
        //   java: Component name = stack.get(DataComponents.CUSTOM_NAME);
        let mut name: MutableComponent = todo!("fabric2seer: the initialiser of `name` (TODO above)");
        if true /* java: name != null: never null here */ {
            // TODO(fabric2seer): `.accept(..)` on a TriConsumer
            //   java: setComponent.accept(stack, DataComponents.CUSTOM_NAME, TextTransformer.fromLegacy(name.getString()));
        }
        // TODO(fabric2seer): `.get(..)` on a T
        //   java: ItemLore lore = stack.get(DataComponents.LORE);
        let mut lore: ItemLore = todo!("fabric2seer: the initialiser of `lore` (TODO above)");
        if true /* java: lore != null: never null here */ {
            // TODO(fabric2seer): `.cast(..)` on a Class
            //   java: List<Component> fixedLore = lore.lines().stream()
            //   java: 					.map(Component::getString)
            //   java: 					.map(TextTransformer::fromLegacy)
            //   java: 					.map(Component.class::cast)
            //   java: 					.toList();
            let mut fixed_lore: Vec<MutableComponent> = todo!("fabric2seer: the initialiser of `fixedLore` (TODO above)");
            // TODO(fabric2seer): `.accept(..)` on a TriConsumer
            //   java: setComponent.accept(stack, DataComponents.LORE, new ItemLore(fixedLore));
        }
        // TODO(fabric2seer): `.get(..)` on a T
        //   java: CustomData customData = stack.get(DataComponents.CUSTOM_DATA);
        let mut custom_data: CustomData = todo!("fabric2seer: the initialiser of `customData` (TODO above)");
        if true /* java: custom_data != null: never null here */ {
            // TODO(fabric2seer): `.accept(..)` on a TriConsumer
            //   java: setComponent.accept(stack, DataComponents.CUSTOM_DATA, CustomData.of(customData.copyTag().getCompoundOrEmpty("ExtraAttributes")));
        }
        // TODO(fabric2seer): `.getOrDefault(..)` on a T
        //   java: TooltipDisplay display = stack.getOrDefault(DataComponents.TOOLTIP_DISPLAY, TooltipDisplay.DEFAULT)
        //   java: 				.withHidden(DataComponents.ATTRIBUTE_MODIFIERS, true)
        //   java: 				.withHidden(DataComponents.ENCHANTMENTS, true);
        let mut display: TooltipDisplay = todo!("fabric2seer: the initialiser of `display` (TODO above)");
        // TODO(fabric2seer): `.accept(..)` on a TriConsumer
        //   java: setComponent.accept(stack, DataComponents.TOOLTIP_DISPLAY, display);
        return stack;
    }

    /// java: private static void log(String error)
    pub fn log(error: &str) {
        logger().error(&format!("[Skyblocker Legacy Item Fixer] Failed to fix up item! Error: {}", error));
    }

}
