//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.utils.ItemLoader by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::model::profile_member::ProfileMember;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::utils::pet_loader::PetLoader;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::utils::profile_item_storage::Backpack;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::utils::profile_item_storage::Bags;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::utils::profile_item_storage::ProfileItemStorage;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
use crate::de::hysky::skyblocker::utils::item_utils::ItemUtils;
use mcrust::java::util::base64;
use mcrust::java::util::concurrent::completable_future::CompletableFuture;
use mcrust::net::minecraft::nbt::list_tag;
use mcrust::net::minecraft::nbt::tag::Tag;

/// java: private static final int INVENTORY_SIZE = 36;
pub const INVENTORY_SIZE: i32 = 36;

/// java: public class ItemLoader
#[derive(Clone)]
pub struct ItemLoader {}

impl ItemLoader {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static CompletableFuture<ProfileItemStorage> decodeItems(ProfileMember member)
    /// (`Executors.newVirtualThreadPerTaskExecutor()`: the member is one of the mod's objects, which a worker cannot
    /// reach - the decode runs deferred on the render thread: `compat::LocalFuture::supply_local`)
    pub fn decode_items(member: &compat::Shared<ProfileMember>) -> compat::LocalFuture<ProfileItemStorage> {
        let member = member.clone();
        return compat::LocalFuture::supply_local(move || ItemLoader::decode_items_internal(&member));
    }

    /// java: private static ProfileItemStorage decodeItemsInternal(ProfileMember member)
    pub fn decode_items_internal(member: &compat::Shared<ProfileMember>) -> ProfileItemStorage {
        let mut inventory: Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack> = (if member.lock().borrow().inventories.inventory_contents.is_some() { ItemLoader::fix_hotbar(&ItemLoader::decode(&compat::read(member.lock().borrow().inventories.inventory_contents.as_ref().unwrap(), |__t| __t.data.clone()))) } else { vec![] });
        let mut armour: Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack> = (if member.lock().borrow().inventories.armour_contents.is_some() { (ItemLoader::decode(&compat::read(member.lock().borrow().inventories.armour_contents.as_ref().unwrap(), |__t| __t.data.clone()))).iter().rev().cloned().collect::<Vec<_>>() } else { vec![] });
        let mut equipment: Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack> = (if member.lock().borrow().inventories.equipment_contents.is_some() { ItemLoader::decode(&compat::read(member.lock().borrow().inventories.equipment_contents.as_ref().unwrap(), |__t| __t.data.clone())) } else { vec![] });
        let mut ender_chest: Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack> = (if member.lock().borrow().inventories.ender_chest_contents.is_some() { ItemLoader::decode(&compat::read(member.lock().borrow().inventories.ender_chest_contents.as_ref().unwrap(), |__t| __t.data.clone())) } else { vec![] });
        let mut accessories: Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack> = (if member.lock().borrow().inventories.bag_contents.talisman_bag.is_some() { ItemLoader::decode(&compat::read(member.lock().borrow().inventories.bag_contents.talisman_bag.as_ref().unwrap(), |__t| __t.data.clone())) } else { vec![] });
        let mut backpacks: std::collections::BTreeMap<i32, Backpack> = std::collections::BTreeMap::new();
        for entry in { member.lock().borrow().inventories.backpack_contents.iter().map(|(k, v)| (k.clone(), v.clone())).collect::<Vec<_>>() } {
            let mut icon: mcrust::net::minecraft::world::item::item_stack::ItemStack = mcrust::net::minecraft::world::item::item_stack::empty();
            let mut contents: Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack> = ItemLoader::decode(&compat::read(&entry.1, |__t| __t.data.clone()));
            if member.lock().borrow().inventories.backpack_icons.contains_key(&entry.0) {
                let mut icon_contents: Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack> = ItemLoader::decode(&compat::read(&member.lock().borrow().inventories.backpack_icons.get(&entry.0).cloned().unwrap(), |__t| __t.data.clone()));
                if !icon_contents.is_empty() {
                    icon = icon_contents[0].clone();
                }
            }
            backpacks.insert(entry.0.parse::<i32>().expect("NumberFormatException"), Backpack::new(icon.clone(), contents.clone()));
        }
        let mut armour_sets: Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack> = Vec::new();
        for loadout in member.lock().borrow().loadouts.armour.get_loadouts() {
            let mut helmet: Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack> = (if compat::read(&loadout, |__t| __t.helmet.clone()).is_some() { ItemLoader::decode(&compat::read(&compat::read(&loadout, |__t| __t.helmet.clone()).unwrap(), |__t| __t.data.clone())) } else { vec![mcrust::net::minecraft::world::item::item_stack::empty()] });
            let mut chestplate: Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack> = (if compat::read(&loadout, |__t| __t.chestplate.clone()).is_some() { ItemLoader::decode(&compat::read(&compat::read(&loadout, |__t| __t.chestplate.clone()).unwrap(), |__t| __t.data.clone())) } else { vec![mcrust::net::minecraft::world::item::item_stack::empty()] });
            let mut leggings: Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack> = (if compat::read(&loadout, |__t| __t.leggings.clone()).is_some() { ItemLoader::decode(&compat::read(&compat::read(&loadout, |__t| __t.leggings.clone()).unwrap(), |__t| __t.data.clone())) } else { vec![mcrust::net::minecraft::world::item::item_stack::empty()] });
            let mut boots: Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack> = (if compat::read(&loadout, |__t| __t.boots.clone()).is_some() { ItemLoader::decode(&compat::read(&compat::read(&loadout, |__t| __t.boots.clone()).unwrap(), |__t| __t.data.clone())) } else { vec![mcrust::net::minecraft::world::item::item_stack::empty()] });
            armour_sets.push(helmet[0].clone());
            armour_sets.push(chestplate[0].clone());
            armour_sets.push(leggings[0].clone());
            armour_sets.push(boots[0].clone());
        }
        if member.lock().borrow().loadouts.armour.equipped_set.is_some() {
            let mut starting_index: i32 = (((member.lock().borrow().loadouts.armour.equipped_set.clone().unwrap()).wrapping_sub(1)).wrapping_mul(4)).min((armour_sets.len() as i32));
            armour_sets[(starting_index.wrapping_add(0)) as usize] = armour[0].clone();
            armour_sets[(starting_index.wrapping_add(1)) as usize] = armour[(1) as usize].clone();
            armour_sets[(starting_index.wrapping_add(2)) as usize] = armour[(2) as usize].clone();
            armour_sets[(starting_index.wrapping_add(3)) as usize] = armour[armour.len() - 1].clone();
        }
        let mut equipment_sets: Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack> = Vec::new();
        for loadout in member.lock().borrow().loadouts.equipment.get_loadouts() {
            let mut necklace: Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack> = (if compat::read(&loadout, |__t| __t.necklace.clone()).is_some() { ItemLoader::decode(&compat::read(&compat::read(&loadout, |__t| __t.necklace.clone()).unwrap(), |__t| __t.data.clone())) } else { vec![mcrust::net::minecraft::world::item::item_stack::empty()] });
            let mut cloak: Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack> = (if compat::read(&loadout, |__t| __t.cloak.clone()).is_some() { ItemLoader::decode(&compat::read(&compat::read(&loadout, |__t| __t.cloak.clone()).unwrap(), |__t| __t.data.clone())) } else { vec![mcrust::net::minecraft::world::item::item_stack::empty()] });
            let mut belt: Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack> = (if compat::read(&loadout, |__t| __t.belt.clone()).is_some() { ItemLoader::decode(&compat::read(&compat::read(&loadout, |__t| __t.belt.clone()).unwrap(), |__t| __t.data.clone())) } else { vec![mcrust::net::minecraft::world::item::item_stack::empty()] });
            let mut bracelet_or_gloves: Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack> = (if compat::read(&loadout, |__t| __t.bracelet_or_gloves.clone()).is_some() { ItemLoader::decode(&compat::read(&compat::read(&loadout, |__t| __t.bracelet_or_gloves.clone()).unwrap(), |__t| __t.data.clone())) } else { vec![mcrust::net::minecraft::world::item::item_stack::empty()] });
            equipment_sets.push(necklace[0].clone());
            equipment_sets.push(cloak[0].clone());
            equipment_sets.push(belt[0].clone());
            equipment_sets.push(bracelet_or_gloves[0].clone());
        }
        if member.lock().borrow().loadouts.equipment.equipped_set.is_some() {
            let mut starting_index: i32 = (((member.lock().borrow().loadouts.equipment.equipped_set.clone().unwrap()).wrapping_sub(1)).wrapping_mul(4)).min((equipment_sets.len() as i32));
            equipment_sets[(starting_index.wrapping_add(0)) as usize] = equipment[0].clone();
            equipment_sets[(starting_index.wrapping_add(1)) as usize] = equipment[(1) as usize].clone();
            equipment_sets[(starting_index.wrapping_add(2)) as usize] = equipment[(2) as usize].clone();
            equipment_sets[(starting_index.wrapping_add(3)) as usize] = equipment[equipment.len() - 1].clone();
        }
        return ProfileItemStorage::new(inventory.clone(), armour.clone(), equipment.clone(), ender_chest.clone(), backpacks.clone(), armour_sets.iter().cloned().collect::<Vec<_>>(), equipment_sets.iter().cloned().collect::<Vec<_>>(), PetLoader::parse_pets(&member.lock().borrow().pets_data.pets), Bags::new(accessories.clone()));
    }

    /// java: public static List<ItemStack> decode(String itemData)
    pub fn decode(item_data: &str) -> Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack> {
        // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`; the body's `return` is the closure's value
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            // java: byte[] gzippedData = Base64.getDecoder().decode(itemData);
            let mut gzipped_data: Vec<u8> = base64::get_decoder().decode(item_data).unwrap_or_else(|e| panic!("IllegalArgumentException: {e}"));
            // java: CompoundTag decoded = NbtIo.readCompressed(new ByteArrayInputStream(gzippedData), NbtAccounter.unlimitedHeap());
            let mut decoded: Tag = mcrust::net::minecraft::nbt::nbt_io::read_compressed(&gzipped_data, mcrust::net::minecraft::nbt::nbt_accounter::NbtAccounter::unlimited_heap()).unwrap_or_else(|e| panic!("IOException: {e}"));
            // java: ListTag items = decoded.getListOrEmpty("i"); (its elements)
            let mut items: Vec<Tag> = decoded.get_list_or_empty("i");
            let mut stacks: Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack> = Vec::new();
            for i in 0..(items.len() as i32) {
                let mut tag: Tag = list_tag::get_compound_or_empty(&items, i);
                if tag.get_int_or("id", 0) == 0 {
                    stacks.push(mcrust::net::minecraft::world::item::item_stack::empty());
                    continue;
                }
                // java: ItemStack stack = LegacyItemStackFixer.fixLegacyStack(tag, ItemStack.CODEC, ItemStack.EMPTY, ItemStack::set);
                let mut stack: mcrust::net::minecraft::world::item::item_stack::ItemStack = crate::de::hysky::skyblocker::utils::datafixer::legacy_item_stack_fixer::LegacyItemStackFixer::fix_legacy_stack(
                    &tag,
                    &mcrust::net::minecraft::world::item::item_stack::codec(),
                    &mcrust::net::minecraft::world::item::item_stack::empty(),
                    &|s: &mut mcrust::net::minecraft::world::item::item_stack::ItemStack, r#type, value| {
                        s.set(r#type, Some(value));
                    },
                );
                if stack.is_empty() {
                    let mut name: String = format!("Error: {}", tag.get_compound_or_empty("tag").get_compound_or_empty("ExtraAttributes").get_string_or("id", ""));
                    let mut placeholder: mcrust::net::minecraft::world::item::item_stack::ItemStack = FlexibleItemStack::get_stack_or_throw(&ItemUtils::get_named_placeholder(&name));
                    stacks.push(placeholder.clone());
                    continue;
                }
                if stack.has(mcrust::net::minecraft::core::component::data_components::custom_data()) {
                    let mut custom_data: Tag = ItemUtils::get_custom_data(&stack);
                    // java: customData.put("aaron-mod", Util.make(new CompoundTag(), comp -> comp.putBoolean("alwaysDisplaySkyblockInfo", true)));
                    // (`getCustomData` is the stack's own tag in Java, mutated in place; the port's is a copy, set back)
                    let mut aaron_mod: Tag = Tag::compound();
                    aaron_mod.put_boolean("alwaysDisplaySkyblockInfo", true);
                    custom_data.put("aaron-mod", aaron_mod);
                    stack.set(mcrust::net::minecraft::core::component::data_components::custom_data().clone(), Some(mcrust::com::mojang::serialization::obj(mcrust::net::minecraft::world::item::component::custom_data::CustomData::of(&custom_data))));
                }
                stacks.push(stack.clone());
            }
            return stacks.iter().cloned().collect::<Vec<_>>();
        })) {
            Ok(__v) => return __v,
            Err(e) => {
                logger().error(&format!("[Skyblocker Profile Viewer Item Loader] Failed to decode items."));
            }
        }
        return vec![];
    }

    /// java: private static List<ItemStack> fixHotbar(List<ItemStack> inventory)
    pub fn fix_hotbar(inventory: &Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack> {
        let mut fixed_inventory: Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack> = Vec::new();
        fixed_inventory.extend(inventory[(9) as usize..(INVENTORY_SIZE) as usize].to_vec().into_iter());
        fixed_inventory.extend(inventory[(0) as usize..(9) as usize].to_vec().into_iter());
        return fixed_inventory.iter().cloned().collect::<Vec<_>>();
    }

}
