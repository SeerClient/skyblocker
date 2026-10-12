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

/// java: private static final Codec<FlexibleItemStack> MAP_CODEC = RecordCodecBuilder.create(instance -> instance.group( Item.CODEC.fieldOf(FIELD_ID).forGetter(FlexibleItemStack::typeHolder), ExtraCodecs.intRa
pub static MAP_CODEC: compat::MainThread<CodecRef<compat::Shared<FlexibleItemStack>>> = compat::MainThread::new(|| record_codec_builder::create(record_codec_builder::apply3(record_codec_builder::of(|o: &compat::Shared<FlexibleItemStack>| FlexibleItemStack::type_holder(o), codec::field_of(mcrust::net::minecraft::world::item::item::codec(), &compat::jstring(&FIELD_ID))), record_codec_builder::of(|o: &compat::Shared<FlexibleItemStack>| FlexibleItemStack::count(o), codec::optional_field_of(extra_codecs::int_range(1, mcrust::net::minecraft::world::item::item::ABSOLUTE_MAX_STACK_SIZE), &compat::jstring(&FIELD_COUNT), 1)), record_codec_builder::of(|o: &compat::Shared<FlexibleItemStack>| FlexibleItemStack::components(o), codec::optional_field_of(mcrust::net::minecraft::core::component::data_component_patch::codec(), &compat::jstring(&FIELD_COMPONENTS), mcrust::net::minecraft::core::component::data_component_patch::empty())), move |__a0, __a1, __a2| FlexibleItemStack::new(__a0.clone(), __a1.clone(), Some(__a2.clone())))));
// TODO(fabric2seer): `.value(..)` on a U
//   java: public static final Codec<FlexibleItemStack> CODEC = Codec.withAlternative(MAP_CODEC, Item.CODEC, item -> new FlexibleItemStack(item.value()));
/// java: public static final Codec<FlexibleItemStack> CODEC = Codec.withAlternative(MAP_CODEC, Item.CODEC, item -> new FlexibleItemStack(item.value()));
pub static CODEC: compat::MainThread<CodecRef<compat::Shared<FlexibleItemStack>>> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `CODEC` - see the TODO above"));
/// java: public static final FlexibleItemStack EMPTY = new FlexibleItemStack((Void) null);
pub static EMPTY: compat::MainThread<compat::Shared<FlexibleItemStack>> = compat::MainThread::new(|| FlexibleItemStack::new5(Some(((None) as ()))));
/// java: private static final DataComponentGetter FALLBACK_COMPONENT_GETTER = new FallbackComponentGetter();
pub static FALLBACK_COMPONENT_GETTER: compat::MainThread<Box<dyn DataComponentGetter>> = compat::MainThread::new(|| FallbackComponentGetter::new());

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
        // TODO(fabric2seer): static call DataComponentPatchAccessor.invokeInit(..)
        //   java: DataComponentPatch copy = DataComponentPatchAccessor.invokeInit((components == null || components == DataComponentPatch.EMPTY) ? new Reference2ObjectArrayMap<>() : new Reference2ObjectArrayMap<>(((DataComponentPatchAccessor) (Object) components).getMap()));
        let mut copy: DataComponentPatch = todo!("fabric2seer: the initialiser of `copy` (TODO above)");
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
        let __this = Self::new(item.built_in_registry_holder(), 1, None);
        __this
    }

    /// java: public FlexibleItemStack(ItemStackTemplate template)
    pub fn new3(mut template: ItemStackTemplate) -> compat::Shared<Self> {
        let __this = Self::new(*template.item(), template.count(), Some(template.components().clone()));
        __this
    }

    /// java: public FlexibleItemStack(ItemStack stack)
    pub fn new4(mut stack: mcrust::net::minecraft::world::item::item_stack::ItemStack) -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            self_ref: std::sync::Weak::new(),
            item: None,
            count: 0,
            components: None,
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
        if stack.is_empty() {
            panic!("IllegalStateException: {}", "Stack must be non-empty");
        }
        // TODO(fabric2seer): this statement
        //   java: this(stack.typeHolder(), stack.count(), stack.getComponentsPatch());
        __this
    }

    /// java: @SuppressWarnings("deprecation") private FlexibleItemStack(@Nullable Void voidMarker)
    pub fn new5(mut void_marker: Option<()>) -> compat::Shared<Self> {
        let __this = Self::new(items::items().get("air").built_in_registry_holder(), 0, None);
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
    pub fn get<T: 'static>(__this: &compat::Shared<Self>, r#type: &Arc<DataComponentType>) -> T {
        if compat::read(__this, |__t| __t.item_stack.clone()).is_some() {
            return __this.lock().borrow().item_stack.as_ref().unwrap().get(r#type).unwrap();
        } else {
            return { let (__q0, __q1,) = (FALLBACK_COMPONENT_GETTER.get_mut(), r#type,); __this.lock().borrow().components.as_ref().unwrap().get(__q0, __q1) }.unwrap();
        }
    }

    /// java: public <T> void set(DataComponentType<T> type, T value)
    pub fn set<T: Clone + 'static>(__this: &compat::Shared<Self>, mut r#type: Arc<DataComponentType>, mut value: T) {
        if compat::read(__this, |__t| __t.item_stack.clone()).is_some() {
            { let (__q0, __q1,) = (r#type.clone(), Some(value.clone()),); __this.lock().borrow_mut().item_stack.as_mut().unwrap().set(__q0, __q1) };
        }
        (compat::read(__this, |__t| __t.components.clone()).map).insert(r#type.clone(), Some(value.clone()));
    }

    /// java: @SuppressWarnings("unchecked") public void applyComponents(DataComponentPatch patch)
    pub fn apply_components(__this: &compat::Shared<Self>, patch: &DataComponentPatch) {
        for entry in patch.entry_set().iter() {
            if entry.1.is_some() {
                FlexibleItemStack::set(__this, entry.0.clone(), entry.1.clone().unwrap());
            }
        }
    }

    /// java: public boolean isEmpty()
    pub fn is_empty(__this: &compat::Shared<Self>) -> bool {
        return (std::ptr::addr_eq(std::sync::Arc::as_ptr(__this), std::sync::Arc::as_ptr(EMPTY.get_mut())) || std::ptr::addr_eq(std::sync::Arc::as_ptr(&__this.lock().borrow().item.as_ref().unwrap().value()), std::sync::Arc::as_ptr(&items::items().get("air"))));
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
        return ItemStackTemplate::new(compat::read(__this, |__t| __t.item.clone()).unwrap(), compat::read(__this, |__t| __t.count), compat::read(__this, |__t| __t.components.clone()).unwrap()).unwrap();
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
        } else if Holder::is_bound_direct() && Holder::are_components_bound_direct() {
            if compat::read(__this, |__t| __t.item_stack.clone()).is_none() {
                let mut result: mcrust::net::minecraft::world::item::item_stack::ItemStack = mcrust::net::minecraft::world::item::item_stack::ItemStack::with_patch(compat::read(__this, |__t| __t.item.clone()).unwrap(), compat::read(__this, |__t| __t.count), &compat::read(__this, |__t| __t.components.clone()).unwrap());
                let mut error: Option<mcrust::com::mojang::serialization::data_result::Error<mcrust::net::minecraft::world::item::item_stack::ItemStack>> = item_stack::validate_strict(result.clone()).error_ref().unwrap();
                if error.is_some() {
                    logger().warn(&format!("[Skyblocker Flexible Item Stack] Can't create item stack with properties {}, error: {}", compat::jstring(&__this), error.clone().unwrap().message()));
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

/// java: private record FallbackComponentGetter() implements DataComponentGetter
#[derive(Clone, Debug, PartialEq)]
pub struct FallbackComponentGetter {}

impl FallbackComponentGetter {
    /// java: the canonical record constructor
    pub fn new() -> Self {
        Self {  }
    }

    /// java: @Override public <T> @Nullable T get(DataComponentType<? extends T> type)
    pub fn get<T: 'static>(&self, r#type: &Arc<DataComponentType>) -> Option<T> {
        return None;
    }

}
