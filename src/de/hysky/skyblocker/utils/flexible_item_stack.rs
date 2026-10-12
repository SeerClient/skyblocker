//! Ported from de.hysky.skyblocker.utils.FlexibleItemStack by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::injected::skyblocker_stack::SkyblockerStack;
use crate::de::hysky::skyblocker::skyblock::item::pet_info::PetInfo;
use crate::de::hysky::skyblocker::skyblock::item::skyblock_item_rarity::SkyblockItemRarity;
use crate::de::hysky::skyblocker::utils::item_ability::ItemAbility;
use mcrust::com::mojang::serialization::codec;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::com::mojang::serialization::codecs::record_codec_builder;
use mcrust::net::minecraft::core::component::data_component_getter::DataComponentGetter;
use mcrust::net::minecraft::core::component::data_component_patch::DataComponentPatch;
use mcrust::net::minecraft::core::component::data_component_type::DataComponentType;
use mcrust::net::minecraft::core::holder::Holder;
use mcrust::net::minecraft::util::extra_codecs;
use mcrust::net::minecraft::world::item::item::Item;
use mcrust::net::minecraft::world::item::item_stack;
use mcrust::net::minecraft::world::item::item_stack_template::ItemStackTemplate;
use mcrust::net::minecraft::world::item::items;
use mcrust::net::minecraft::core::component::data_component_map::DataComponentGetter as _;
use mcrust::net::minecraft::world::item::item_instance::ItemInstance as _;
use mcrust::net::minecraft::world::item::item_instance::{FIELD_COMPONENTS, FIELD_COUNT, FIELD_ID};
use mcrust::net::minecraft::core::typed_instance::TypedInstance as _;

/// java: private static final Codec<FlexibleItemStack> MAP_CODEC = RecordCodecBuilder.create(instance -> instance.group( Item.CODEC.fieldOf(FIELD_ID).forGetter(FlexibleItemStack::typeHolder), ExtraCodecs.intRa
pub static MAP_CODEC: compat::MainThread<CodecRef<compat::Shared<FlexibleItemStack>>> = compat::MainThread::new(|| record_codec_builder::create(record_codec_builder::apply3(record_codec_builder::of(|o: &compat::Shared<FlexibleItemStack>| FlexibleItemStack::type_holder(o), codec::field_of(codec::xmap(mcrust::net::minecraft::world::item::item::codec(), |item: Arc<Item>| Holder::Reference(item.built_in_registry_holder()), |holder: &Holder| item_of(holder)), FIELD_ID)), record_codec_builder::of(|o: &compat::Shared<FlexibleItemStack>| FlexibleItemStack::count(o), codec::optional_field_of(extra_codecs::int_range(1, mcrust::net::minecraft::world::item::item::ABSOLUTE_MAX_STACK_SIZE), FIELD_COUNT, 1)), record_codec_builder::of(|o: &compat::Shared<FlexibleItemStack>| FlexibleItemStack::components(o), codec::optional_field_of(mcrust::net::minecraft::core::component::data_component_patch::codec(), FIELD_COMPONENTS, mcrust::net::minecraft::core::component::data_component_patch::empty())), move |__a0, __a1, __a2| FlexibleItemStack::new(__a0.clone(), __a1.clone(), Some(__a2.clone())))));
// TODO(fabric2seer): `.value(..)` on a U
//   java: public static final Codec<FlexibleItemStack> CODEC = Codec.withAlternative(MAP_CODEC, Item.CODEC, item -> new FlexibleItemStack(item.value()));
/// java: public static final Codec<FlexibleItemStack> CODEC = Codec.withAlternative(MAP_CODEC, Item.CODEC, item -> new FlexibleItemStack(item.value()));
pub static CODEC: compat::MainThread<CodecRef<compat::Shared<FlexibleItemStack>>> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `CODEC` - see the TODO above"));
/// java: public static final FlexibleItemStack EMPTY = new FlexibleItemStack((Void) null);
pub static EMPTY: compat::MainThread<compat::Shared<FlexibleItemStack>> = compat::MainThread::new(|| FlexibleItemStack::new5(None));
/// java: private static final DataComponentGetter FALLBACK_COMPONENT_GETTER = new FallbackComponentGetter();
pub static FALLBACK_COMPONENT_GETTER: LazyLock<FallbackComponentGetter> = LazyLock::new(FallbackComponentGetter::new);

/// `Holder<Item>.value()`: the item the holder holds.
pub fn item_of(holder: &Holder) -> Arc<Item> {
    mcrust::net::minecraft::world::item::item::from_obj(&holder.value()).expect("a Holder<Item>")
}

/// `item.builtInRegistryHolder()` as the `Holder<Item>` Java passes on.
fn holder_of(item: &Arc<Item>) -> Holder {
    Holder::Reference(item.built_in_registry_holder())
}

/// `DataComponentPatchAccessor.invokeInit(new Reference2ObjectArrayMap<>(components.getMap()))` (an empty map for null /
/// EMPTY), with `put(type, Optional.of(value))` applied when `set` asks: the port's patch map is not the mod's to
/// write, so the copy is rebuilt through the patch builder - the same entries, removals included.
fn copy_patch(components: Option<&DataComponentPatch>, put: Option<(Arc<DataComponentType>, mcrust::com::mojang::serialization::Obj)>) -> DataComponentPatch {
    let mut builder = mcrust::net::minecraft::core::component::data_component_patch::builder();
    if let Some(components) = components {
        for (r#type, value) in components.entry_set().iter() {
            match value {
                Some(value) => {
                    builder.set(r#type.clone(), value.clone());
                }
                None => {
                    builder.remove(r#type.clone());
                }
            }
        }
    }
    if let Some((r#type, value)) = put {
        builder.set(r#type, value);
    }
    builder.build()
}

/// java: public final class FlexibleItemStack implements ItemInstance, SkyblockerStack
#[derive(Clone)]
pub struct FlexibleItemStack {
    /// java: private final Holder<Item> item;
    pub item: Option<Holder>,
    /// java: private final int count;
    pub count: i32,
    /// java: private final DataComponentPatch components;
    pub components: Option<DataComponentPatch>,
    /// java: private @Nullable ItemStack itemStack;
    pub item_stack: Option<mcrust::net::minecraft::world::item::item_stack::ItemStack>,
    /// java: private @Nullable String skyblockId;
    pub skyblock_id: Option<String>,
    /// java: private @Nullable String skyblockApiId;
    pub skyblock_api_id: Option<String>,
    /// java: private @Nullable String neuName;
    pub neu_name: Option<String>,
    /// java: private @Nullable String uuid;
    pub uuid: Option<String>,
    /// java: private @Nullable List<String> loreStrings;
    pub lore_strings: Option<Vec<String>>,
    /// java: private @Nullable List<ItemAbility> abilities;
    pub abilities: Option<Vec<ItemAbility>>,
    /// java: private @Nullable PetInfo petInfo;
    pub pet_info: Option<PetInfo>,
    /// java: private @Nullable SkyblockItemRarity skyblockRarity;
    pub skyblock_rarity: Option<SkyblockItemRarity>,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl FlexibleItemStack {
    /// java: public FlexibleItemStack(Holder<Item> item, int count, @Nullable DataComponentPatch components)
    pub fn new(mut item: Holder, mut count: i32, mut components: Option<DataComponentPatch>) -> compat::Shared<Self> {
        // java: DataComponentPatch copy = DataComponentPatchAccessor.invokeInit((components == null || components == DataComponentPatch.EMPTY) ? new Reference2ObjectArrayMap<>() : new Reference2ObjectArrayMap<>(((DataComponentPatchAccessor) (Object) components).getMap()));
        let mut copy: DataComponentPatch = copy_patch(components.as_ref(), None);
        let __init_item = Some(item);
        let __init_count = count;
        let __init_components = Some(copy.clone());
        let __this = compat::shared(Self {
            self_ref: std::sync::Weak::new(),
            item: __init_item,
            count: __init_count,
            components: __init_components,
            item_stack: None,
            skyblock_id: None,
            skyblock_api_id: None,
            neu_name: None,
            uuid: None,
            lore_strings: None,
            abilities: None,
            pet_info: None,
            skyblock_rarity: None,
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        __this
    }

    /// java: @SuppressWarnings("deprecation") public FlexibleItemStack(Item item)
    pub fn new2(mut item: Arc<Item>) -> compat::Shared<Self> {
        let __this = Self::new(holder_of(&item), 1, None);
        __this
    }

    /// java: public FlexibleItemStack(ItemStackTemplate template)
    pub fn new3(mut template: ItemStackTemplate) -> compat::Shared<Self> {
        let __this = Self::new(holder_of(template.item()), template.count(), Some(template.components().clone()));
        __this
    }

    /// java: public FlexibleItemStack(ItemStack stack)
    pub fn new4(mut stack: mcrust::net::minecraft::world::item::item_stack::ItemStack) -> compat::Shared<Self> {
        if stack.is_empty() {
            panic!("IllegalStateException: {}", "Stack must be non-empty");
        }
        // java: this(stack.typeHolder(), stack.count(), stack.getComponentsPatch());
        let __this = Self::new(stack.type_holder(), stack.get_count(), Some(stack.get_components_patch()));
        __this
    }

    /// java: @SuppressWarnings("deprecation") private FlexibleItemStack(@Nullable Void voidMarker)
    pub fn new5(mut void_marker: Option<()>) -> compat::Shared<Self> {
        let __this = Self::new(holder_of(&items::items().get("air")), 0, None);
        __this
    }

    /// java: @Override public Holder<Item> typeHolder()
    pub fn type_holder(__this: &compat::Shared<Self>) -> Holder {
        return compat::read(__this, |__t| __t.item.clone()).unwrap();
    }

    /// java: @Override public int count()
    pub fn count(__this: &compat::Shared<Self>) -> i32 {
        return compat::read(__this, |__t| __t.count);
    }

    /// java: public DataComponentPatch components()
    pub fn components(__this: &compat::Shared<Self>) -> DataComponentPatch {
        return compat::read(__this, |__t| __t.components.clone()).unwrap();
    }

    /// java: @Override public <T> @Nullable T get(DataComponentType<? extends T> type)
    pub fn get<T: Clone + 'static>(__this: &compat::Shared<Self>, r#type: &Arc<DataComponentType>) -> Option<T> {
        // java: `@Nullable T` - the component's value (the erased `Obj`, read back with Java's cast)
        let value: Option<mcrust::com::mojang::serialization::Obj> = if compat::read(__this, |__t| __t.item_stack.is_some()) {
            compat::read(__this, |__t| DataComponentGetter::get(__t.item_stack.as_ref().unwrap(), r#type))
        } else {
            compat::read(__this, |__t| __t.components.as_ref().unwrap().get(&*FALLBACK_COMPONENT_GETTER, r#type))
        };
        return value.and_then(|value| mcrust::com::mojang::serialization::cast::<T>(&value).cloned());
    }

    /// java: public <T> void set(DataComponentType<T> type, T value)
    pub fn set<T: std::any::Any + std::fmt::Debug + PartialEq + Send + Sync>(__this: &compat::Shared<Self>, mut r#type: Arc<DataComponentType>, mut value: T) {
        let value: mcrust::com::mojang::serialization::Obj = mcrust::com::mojang::serialization::obj(value);
        FlexibleItemStack::set_obj(__this, r#type, value);
    }

    /// `set` with the value already erased (`applyComponents` hands the patch's values on as they are).
    pub fn set_obj(__this: &compat::Shared<Self>, r#type: Arc<DataComponentType>, value: mcrust::com::mojang::serialization::Obj) {
        if compat::read(__this, |__t| __t.item_stack.is_some()) {
            compat::write(__this, |__t| __t.item_stack.as_mut().unwrap().set(r#type.clone(), Some(value.clone())));
        }
        // java: ((DataComponentPatchAccessor) (Object) this.components).getMap().put(type, Optional.of(value));
        compat::write(__this, |__t| {
            let copy = copy_patch(__t.components.as_ref(), Some((r#type, value)));
            __t.components = Some(copy);
        });
    }

    /// java: @SuppressWarnings("unchecked") public void applyComponents(DataComponentPatch patch)
    pub fn apply_components(__this: &compat::Shared<Self>, patch: &DataComponentPatch) {
        for entry in patch.entry_set().iter() {
            if entry.1.is_some() {
                FlexibleItemStack::set_obj(__this, entry.0.clone(), entry.1.clone().unwrap());
            }
        }
    }

    /// java: public boolean isEmpty()
    pub fn is_empty(__this: &compat::Shared<Self>) -> bool {
        return (std::sync::Arc::ptr_eq(__this, EMPTY.get_mut()) || std::sync::Arc::ptr_eq(&item_of(&compat::read(__this, |__t| __t.item.clone().unwrap())), &items::items().get("air")));
    }

    /// java: public FlexibleItemStack copy()
    pub fn copy(__this: &compat::Shared<Self>) -> compat::Shared<FlexibleItemStack> {
        return FlexibleItemStack::copy_with_count(__this, FlexibleItemStack::count(__this));
    }

    /// java: public FlexibleItemStack copyWithCount(int count)
    pub fn copy_with_count(__this: &compat::Shared<Self>, mut count: i32) -> compat::Shared<FlexibleItemStack> {
        return FlexibleItemStack::new(compat::read(__this, |__t| __t.item.clone()).unwrap(), count, compat::read(__this, |__t| __t.components.clone()));
    }

    /// java: public ItemStackTemplate toTemplate()
    pub fn to_template(__this: &compat::Shared<Self>) -> ItemStackTemplate {
        return ItemStackTemplate::new(item_of(&compat::read(__this, |__t| __t.item.clone()).unwrap()), compat::read(__this, |__t| __t.count), compat::read(__this, |__t| __t.components.clone()).unwrap()).unwrap_or_else(|e| panic!("IllegalArgumentException: {e}"));
    }

    /// java: public @Nullable ItemStack getStack()
    pub fn get_stack(__this: &compat::Shared<Self>) -> Option<mcrust::net::minecraft::world::item::item_stack::ItemStack> {
        FlexibleItemStack::try_create_stack(__this);
        return compat::read(__this, |__t| __t.item_stack.clone());
    }

    /// java: public ItemStack getStackOrThrow()
    pub fn get_stack_or_throw(__this: &compat::Shared<Self>) -> mcrust::net::minecraft::world::item::item_stack::ItemStack {
        FlexibleItemStack::try_create_stack(__this);
        return compat::read(__this, |__t| __t.item_stack.clone()).unwrap();
    }

    /// java: public ItemStack getStackOrEmpty()
    pub fn get_stack_or_empty(__this: &compat::Shared<Self>) -> mcrust::net::minecraft::world::item::item_stack::ItemStack {
        FlexibleItemStack::try_create_stack(__this);
        return (if compat::read(__this, |__t| __t.item_stack.clone()).is_none() { Some(mcrust::net::minecraft::world::item::item_stack::empty()) } else { compat::read(__this, |__t| __t.item_stack.clone()) }).unwrap();
    }

    /// java: private void tryCreateStack()
    pub fn try_create_stack(__this: &compat::Shared<Self>) {
        let mut type_holder: Holder = FlexibleItemStack::type_holder(__this);
        if FlexibleItemStack::is_empty(__this) {
            { let __v = Some(mcrust::net::minecraft::world::item::item_stack::empty()); compat::write(__this, |__t| __t.item_stack = __v); }
        } else if type_holder.is_bound() && type_holder.are_components_bound() {
            if compat::read(__this, |__t| __t.item_stack.is_none()) {
                let mut result: mcrust::net::minecraft::world::item::item_stack::ItemStack = mcrust::net::minecraft::world::item::item_stack::ItemStack::with_patch(item_of(&compat::read(__this, |__t| __t.item.clone()).unwrap()), compat::read(__this, |__t| __t.count), &compat::read(__this, |__t| __t.components.clone()).unwrap());
                let validated = item_stack::validate_strict(result.clone());
                let mut error: Option<String> = validated.error_ref().map(|error| error.message());
                if error.is_some() {
                    // java: LOGGER.warn(.., this, error.get().message()) - `this` is Object.toString(): the class and identity
                    logger().warn(&format!("[Skyblocker Flexible Item Stack] Can't create item stack with properties {}@{:x}, error: {}", "de.hysky.skyblocker.utils.FlexibleItemStack", std::sync::Arc::as_ptr(__this) as *const () as usize, error.clone().unwrap()));
                } else {
                    { let __v = Some(result.clone()); compat::write(__this, |__t| __t.item_stack = __v); }
                }
            }
        } else {
            { let __v = None; compat::write(__this, |__t| __t.item_stack = __v); }
        }
    }

}

impl SkyblockerStack for FlexibleItemStack {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @SuppressWarnings("deprecation") @Override public String getSkyblockId()
    fn get_skyblock_id(&self) -> String {
        if self.skyblock_id.is_some() && !(self.skyblock_id.as_ref().unwrap().is_empty()) {
            return self.skyblock_id.clone().unwrap();
        }
        // TODO(fabric2seer): an assignment used as a value
        //   java: return this.skyblockId = ItemUtils.getItemId(this);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @SuppressWarnings("deprecation") @Override public String getSkyblockApiId()
    fn get_skyblock_api_id(&self) -> String {
        if self.skyblock_api_id.is_some() && !(self.skyblock_api_id.as_ref().unwrap().is_empty()) {
            return self.skyblock_api_id.clone().unwrap();
        }
        // TODO(fabric2seer): an assignment used as a value
        //   java: return this.skyblockApiId = ItemUtils.getSkyblockApiId(this);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @SuppressWarnings("deprecation") @Override public String getNeuName()
    fn get_neu_name(&self) -> String {
        if self.neu_name.is_some() && !(self.neu_name.as_ref().unwrap().is_empty()) {
            return self.neu_name.clone().unwrap();
        }
        // TODO(fabric2seer): an assignment used as a value
        //   java: return this.neuName = ItemUtils.getNeuId(this);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @SuppressWarnings("deprecation") @Override public String getUuid()
    fn get_uuid(&self) -> String {
        if self.uuid.is_some() {
            return self.uuid.clone().unwrap();
        }
        // TODO(fabric2seer): an assignment used as a value
        //   java: return this.uuid = ItemUtils.getItemUuid(this);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @SuppressWarnings("deprecation") @Override public List<String> skyblocker$getLoreStrings()
    fn skyblocker_get_lore_strings(&self) -> Vec<String> {
        if self.lore_strings.is_some() {
            return self.lore_strings.clone().unwrap();
        }
        // TODO(fabric2seer): an assignment used as a value
        //   java: return this.loreStrings = ItemUtils.getLore(this).stream().map(Component::getString).toList();
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @SuppressWarnings("deprecation") @Override public List<ItemAbility> skyblocker$getAbilities()
    fn skyblocker_get_abilities(&self) -> Vec<ItemAbility> {
        if self.abilities.is_some() {
            return self.abilities.clone().unwrap();
        }
        // TODO(fabric2seer): an assignment used as a value
        //   java: return this.abilities = ItemAbility.getAbilities(this);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @SuppressWarnings("deprecation") @Override public PetInfo getPetInfo()
    fn get_pet_info(&self) -> PetInfo {
        if self.pet_info.is_some() {
            return self.pet_info.clone().unwrap();
        }
        // TODO(fabric2seer): an assignment used as a value
        //   java: return this.petInfo = ItemUtils.getPetInfo(this);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @SuppressWarnings("deprecation") @Override public SkyblockItemRarity getSkyblockRarity()
    fn get_skyblock_rarity(&self) -> SkyblockItemRarity {
        if self.skyblock_rarity.is_some() {
            return self.skyblock_rarity.clone().unwrap();
        }
        // TODO(fabric2seer): an assignment used as a value
        //   java: return this.skyblockRarity = ItemUtils.getItemRarity(this);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}

/// java: `FlexibleItemStack implements ItemInstance` (`ItemInstance extends DataComponentGetter`): the getter half,
/// which `ItemStack.copyFrom(type, stack)` reads - the same answer as `FlexibleItemStack.get`, still erased.
impl DataComponentGetter for FlexibleItemStack {
    /// java: @Override public <T> @Nullable T get(DataComponentType<? extends T> type)
    fn get(&self, r#type: &Arc<DataComponentType>) -> Option<mcrust::com::mojang::serialization::Obj> {
        if self.item_stack.is_some() {
            DataComponentGetter::get(self.item_stack.as_ref().unwrap(), r#type)
        } else {
            self.components.as_ref().unwrap().get(&*FALLBACK_COMPONENT_GETTER, r#type)
        }
    }
}

/// java: private record FallbackComponentGetter() implements DataComponentGetter
#[derive(Clone, Debug, PartialEq)]
pub struct FallbackComponentGetter {}

impl FallbackComponentGetter {
    /// java: the canonical record constructor
    pub fn new() -> Self {
        Self {  }
    }

}

impl DataComponentGetter for FallbackComponentGetter {
    /// java: @Override public <T> @Nullable T get(DataComponentType<? extends T> type)
    fn get(&self, r#type: &Arc<DataComponentType>) -> Option<mcrust::com::mojang::serialization::Obj> {
        return None;
    }
}

/// `DataComponentPatch.get(DataComponentGetter prototype, type)`: the port's patch takes its prototype as a
/// `DataComponentMap`; this getter answers no component, so as a map it holds none.
impl mcrust::net::minecraft::core::component::data_component_map::DataComponentMap for FallbackComponentGetter {
    fn key_set(&self) -> Vec<Arc<DataComponentType>> {
        Vec::new()
    }
}
