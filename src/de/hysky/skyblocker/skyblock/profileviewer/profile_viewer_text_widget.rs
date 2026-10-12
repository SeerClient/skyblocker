//! Ported from de.hysky.skyblocker.skyblock.profileviewer.ProfileViewerTextWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::net::minecraft::network::chat::component::Component as _;
use crate::de::hysky::skyblocker::skyblock::accessories::accessories_helper::Accessory;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::info::tooltip_info_type;
use crate::de::hysky::skyblocker::skyblock::profileviewer::inventory::item_loaders::backpack_item_loader::BackpackItemLoader;
use crate::de::hysky::skyblocker::skyblock::profileviewer::inventory::item_loaders::inventory_item_loader::InventoryItemLoader;
use crate::de::hysky::skyblocker::skyblock::profileviewer::inventory::item_loaders::item_loader::ItemLoader;
use crate::de::hysky::skyblocker::skyblock::profileviewer::inventory::item_loaders::item_loader::ItemLoaderBase;
use crate::de::hysky::skyblocker::skyblock::profileviewer::inventory::item_loaders::pets_inventory_item_loader::PetsInventoryItemLoader;
use crate::de::hysky::skyblocker::skyblock::profileviewer::inventory::item_loaders::wardrobe_inventory_item_loader::WardrobeInventoryItemLoader;
use crate::de::hysky::skyblocker::skyblock::profileviewer::utils::profile_viewer_utils::ProfileViewerUtils;
use crate::de::hysky::skyblocker::skyblock::tabhud::util::ico;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
use crate::de::hysky::skyblocker::utils::networth::networth_calculator::NetworthCalculator;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::font::Font;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::org::joml::matrix3x2f_stack::Matrix3x2fStack;
use crate::de::hysky::skyblocker::injected::skyblocker_stack::SkyblockerStack as _;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::info::data_tooltip_info_type::DataTooltipInfoType as _;

/// java: private static final int ROW_GAP = 9;
pub const ROW_GAP: i32 = 9;

/// java: public class ProfileViewerTextWidget
#[derive(Clone)]
pub struct ProfileViewerTextWidget {
    /// java: private String PROFILE_NAME = "UNKNOWN";
    pub profile_name: String,
    /// java: private int SKYBLOCK_LEVEL = 0;
    pub skyblock_level: i32,
    /// java: private double PURSE = 0;
    pub purse: f64,
    /// java: private double BANK = 0;
    pub bank: f64,
    /// java: private double NETWORTH = 0;
    pub networth: f64,
    /// java: private int MAGICAL_POWER = 0;
    pub magical_power: i32,
    /// java: private final List<Component> networthTooltip = new ArrayList<>();
    pub networth_tooltip: Vec<MutableComponent>,
}

impl ProfileViewerTextWidget {
    /// java: public ProfileViewerTextWidget(JsonObject hypixelProfile, JsonObject playerProfile)
    pub fn new(mut hypixel_profile: serde_json::Map<String, serde_json::Value>, mut player_profile: serde_json::Map<String, serde_json::Value>) -> Self {
        let mut this = Self {
            profile_name: "UNKNOWN".to_string(),
            skyblock_level: 0,
            purse: ((0) as f64) as f64,
            bank: ((0) as f64) as f64,
            networth: ((0) as f64) as f64,
            magical_power: 0,
            networth_tooltip: Vec::new(),
        };
        // java: try {..} catch (Exception _) - an exception here is a panic, caught by `catch_unwind`
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            this.profile_name = compat::json::get_as_string(&hypixel_profile.get("cute_name").cloned().unwrap());
            this.skyblock_level = (compat::json::get_as_int(&(compat::json::get_as_json_object_of(&player_profile, "leveling")).get("experience").cloned().unwrap())).wrapping_div(100);
            this.purse = compat::json::get_as_double(&(compat::json::get_as_json_object_of(&player_profile, "currencies")).get("coin_purse").cloned().unwrap());
            this.bank = compat::json::get_as_double(&(compat::json::get_as_json_object_of(&hypixel_profile, "banking")).get("balance").cloned().unwrap());
        }));
        if let Err(_) = __result {
        }
        this.networth = ((this.purse + this.bank) + this.get_items_networth(&player_profile));
        this.magical_power = this.get_magical_power(&player_profile);
        this
    }

    /// java: public int getMagicalPower(JsonObject playerProfile)
    pub fn get_magical_power(&self, player_profile: &serde_json::Map<String, serde_json::Value>) -> i32 {
        let mut total_magical_power: i32 = 0;
        let mut hat_counted: bool = false;
        let mut abicase_counted: bool = false;
        let mut accessories: std::collections::HashMap<String, i32> = std::collections::HashMap::new();
        if tooltip_info_type::ACCESSORIES.get_mut().lock().borrow().get_data().is_none() {
            return total_magical_power;
        }
        // java: try {..} catch (Exception _) - an exception here is a panic, caught by `catch_unwind`
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if compat::json::has(&player_profile, "rift") && compat::json::has(&(compat::json::get_as_json_object_of(&player_profile, "rift")), "access") && compat::json::has(&(compat::json::get_as_json_object_of(&(compat::json::get_as_json_object_of(&player_profile, "rift")), "access")), "consumed_prism") {
                total_magical_power = total_magical_power.wrapping_add(11);
            }
            let mut inventory_data: serde_json::Map<String, serde_json::Value> = compat::json::get_as_json_object_of(&player_profile, "inventory");
            if compat::json::has(&inventory_data, "bag_contents") && compat::json::has(&(compat::json::get_as_json_object_of(&inventory_data, "bag_contents")), "talisman_bag") {
                for item in ({ let __r = &(ItemLoaderBase::new() as compat::Shared<dyn ItemLoader>); let __f = compat::dispatch(__r, |__t| __t.load_items()); __f(__r, &compat::json::get_as_json_object_of(&(compat::json::get_as_json_object_of(&inventory_data, "bag_contents")), "talisman_bag")) }) {
                    if item.get_skyblock_id().is_empty() {
                        continue;
                    }
                    // java: AccessoriesHelper.Accessory accessory = Objects.requireNonNull(TooltipInfoType.ACCESSORIES.getData()).get(item.getSkyblockId());
                    // (a missing accessory is Java's NPE at `accessory.family()`, which the catch below swallows)
                    let mut accessory: Accessory = tooltip_info_type::ACCESSORIES.get_mut().lock().borrow().get_data().expect("NullPointerException").get(&item.get_skyblock_id()).cloned().expect("NullPointerException: accessory");
                    let mut name: String = accessory.family().unwrap_or(accessory.id());
                    if accessories.get(&name).cloned().unwrap_or_else(|| 0) >= item.get_skyblock_rarity().get_mp() {
                        continue;
                    }
                    if item.get_skyblock_id().starts_with("ABICASE") && !abicase_counted {
                        if compat::json::has(&player_profile, "nether_island_player_data") {
                            let mut data: serde_json::Map<String, serde_json::Value> = compat::json::get_as_json_object(&player_profile.get("nether_island_player_data").cloned().unwrap());
                            if compat::json::has(&data, "abiphone") && compat::json::has(&compat::json::get_as_json_object(&data.get("abiphone").cloned().unwrap()), "active_contacts") {
                                total_magical_power = total_magical_power.wrapping_add(((((compat::json::get_as_json_array(&compat::json::get_as_json_object(&data.get("abiphone").cloned().unwrap()).get("active_contacts").cloned().unwrap()).len() as i32).wrapping_div(2)) as f64)) as i32);
                                abicase_counted = true;
                            }
                        }
                    } else if item.get_skyblock_id().starts_with("BALLOON_HAT") || item.get_skyblock_id().starts_with("PARTY_HAT") {
                        if hat_counted {
                            continue;
                        }
                        hat_counted = true;
                    } else if item.get_skyblock_id() == "HEGEMONY_ARTIFACT" {
                        accessories.insert(name.clone(), (item.get_skyblock_rarity().get_mp()).wrapping_mul(2));
                        continue;
                    }
                    accessories.insert(name.clone(), item.get_skyblock_rarity().get_mp());
                }
            }
        }));
        if let Err(_) = __result {
            return -1;
        }
        for entry in accessories.iter().map(|(k, v)| (k.clone(), v.clone())) {
            total_magical_power = total_magical_power.wrapping_add(entry.1);
        }
        return total_magical_power;
    }

    /// java: private double getItemsNetworth(JsonObject playerProfile)
    pub fn get_items_networth(&mut self, player_profile: &serde_json::Map<String, serde_json::Value>) -> f64 {
        let mut value: f64 = ((0) as f64) as f64;
        // java: PriorityQueue<ItemValue> top = new PriorityQueue<>(Comparator.comparingDouble(ItemValue::price));
        let mut top: compat::JavaPriorityQueue<ItemValue> = compat::JavaPriorityQueue::new(|a: &ItemValue, b: &ItemValue| mcrust::javacompat::java_double_compare(a.price, b.price));
        // java: try {..} catch (Exception _) - an exception here is a panic, caught by `catch_unwind`
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut inventory_data: Option<serde_json::Map<String, serde_json::Value>> = Some(compat::json::get_as_json_object_of(&player_profile, "inventory"));
            if let Some(inventory_data) = &inventory_data {
                for stack in ({ let __r = &InventoryItemLoader::new(); let __f = compat::dispatch(__r, |__t| __t.load_items()); __f(&compat::view(__r, |a| a as compat::Shared<dyn ItemLoader>), inventory_data) }) {
                    value = (value + self.add_item_networth(&mut top, &stack));
                }
                if compat::json::has(&inventory_data, "ender_chest_contents") {
                    for stack in ({ let __r = &(ItemLoaderBase::new() as compat::Shared<dyn ItemLoader>); let __f = compat::dispatch(__r, |__t| __t.load_items()); __f(__r, &compat::json::get_as_json_object_of(&inventory_data, "ender_chest_contents")) }) {
                        value = (value + self.add_item_networth(&mut top, &stack));
                    }
                }
                if compat::json::has(&inventory_data, "backpack_contents") {
                    for stack in ({ let __r = &BackpackItemLoader::new(); let __f = compat::dispatch(__r, |__t| __t.load_items()); __f(&compat::view(__r, |a| a as compat::Shared<dyn ItemLoader>), &compat::json::get_as_json_object_of(&inventory_data, "backpack_contents")) }) {
                        value = (value + self.add_item_networth(&mut top, &stack));
                    }
                }
                if compat::json::has(&inventory_data, "wardrobe_contents") {
                    let mut active_slot: i32 = compat::json::get_as_int(&inventory_data.get("wardrobe_equipped_slot").cloned().unwrap());
                    let mut skip: std::collections::HashSet<i32> = std::collections::HashSet::new();
                    if active_slot != -1 {
                        for i in 0..4i32 {
                            let mut base_index: i32 = (active_slot.wrapping_sub(1)).wrapping_rem(9);
                            let mut page: i32 = (active_slot.wrapping_sub(1)).wrapping_div(9);
                            let mut slot_index: i32 = ((page.wrapping_mul(36)).wrapping_add(i.wrapping_mul(9))).wrapping_add(base_index);
                            skip.insert(slot_index);
                        }
                    }
                    let mut loader: compat::Shared<WardrobeInventoryItemLoader> = WardrobeInventoryItemLoader::new(inventory_data.clone());
                    let mut wardrobe_items: Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack> = ({ let __f = compat::dispatch(&loader, |__t| __t.load_items()); __f(&compat::view(&loader, |a| a as compat::Shared<dyn ItemLoader>), &compat::json::get_as_json_object_of(&inventory_data, "wardrobe_contents")) });
                    for index in 0..(wardrobe_items.len() as i32) {
                        if skip.contains(&index) {
                            continue;
                        }
                        let stack = &wardrobe_items[(index) as usize];
                        value = (value + self.add_item_networth(&mut top, stack));
                    }
                }
                if compat::json::has(&inventory_data, "bag_contents") && compat::json::has(&(compat::json::get_as_json_object_of(&inventory_data, "bag_contents")), "talisman_bag") {
                    for stack in ({ let __r = &(ItemLoaderBase::new() as compat::Shared<dyn ItemLoader>); let __f = compat::dispatch(__r, |__t| __t.load_items()); __f(__r, &compat::json::get_as_json_object_of(&(compat::json::get_as_json_object_of(&inventory_data, "bag_contents")), "talisman_bag")) }) {
                        value = (value + self.add_item_networth(&mut top, &stack));
                    }
                }
            }
            for stack in ({ let __r = &PetsInventoryItemLoader::new(); let __f = compat::dispatch(__r, |__t| __t.load_items()); __f(&compat::view(__r, |a| a as compat::Shared<dyn ItemLoader>), player_profile) }) {
                value = (value + self.add_item_networth(&mut top, &stack));
            }
        }));
        if let Err(_) = __result {
        }
        // java: List<ItemValue> list = new ArrayList<>(top); list.sort(Comparator.comparingDouble(ItemValue::price).reversed());
        let mut list: Vec<ItemValue> = top.iter().cloned().collect::<Vec<_>>();
        list.sort_by(|a, b| mcrust::javacompat::java_double_compare(b.price, a.price).cmp(&0));
        self.networth_tooltip.push(component::literal("Top Items:").with_style_format(ChatFormatting::Gold));
        for iv in list.iter() {
            self.networth_tooltip.push((component::literal(&format!("{}: ", iv.name))).append(component::literal(&ProfileViewerUtils::num_letter_format(iv.price)).with_style_format(ChatFormatting::Yellow)));
        }
        return value;
    }

    /// java: private double addItemNetworth(PriorityQueue<ItemValue> top, ItemStack stack)
    pub fn add_item_networth(&self, top: &mut compat::JavaPriorityQueue<ItemValue>, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack) -> f64 {
        // java: double p = NetworthCalculator.getItemNetworth(stack).price();
        // rust: port gap (GAPS_J #7) - `NetworthResult` is a `crate::unresolved` placeholder (the external
        // `net.azureaaron.networth` library is unported), so it has no `price()`.
        let mut p: f64 = todo!("fabric2seer: the initialiser of `p` (TODO above)");
        if p > (0 as f64) {
            // java: top.offer(new ItemValue(stack.getHoverName().getString(), p));
            top.offer(ItemValue::new(compat::component_of(stack.get_hover_name()).get_string(), p));
            // java: if (top.size() > 10) top.poll();
            if top.size() > 10 {
                top.poll();
            }
        }
        return p;
    }

    /// java: public void extractRenderState(GuiGraphicsExtractor graphics, Font textRenderer, int root_x, int root_y, int mouseX, int mouseY)
    pub fn extract_render_state(&self, graphics: &mut GuiGraphicsExtractor, text_renderer: &Font, mut root_x: i32, mut root_y: i32, mut mouse_x: i32, mut mouse_y: i32) {
        let __pose_1 = gui_graphics_extractor::push_pose();
        gui_graphics_extractor::pose_scale((0.75f32) as f32, (0.75f32) as f32);
        let mut root_adjusted_x: i32 = ((((root_x as f32) / 0.75f32)) as i32);
        let mut root_adjusted_y: i32 = ((((root_y as f32) / 0.75f32)) as i32);
        graphics.item(&FlexibleItemStack::get_stack_or_throw(ico::PAINTING.get_mut()), root_adjusted_x, root_adjusted_y.wrapping_add(8));
        gui_graphics_extractor::pop_pose(__pose_1);
        gui_graphics_extractor::text_string_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), Some(&format!("§n{}", self.profile_name)), root_x.wrapping_add(14), root_y.wrapping_add(7), mcrust::net::minecraft::util::common_colors::WHITE, true);
        gui_graphics_extractor::text_string_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), Some(&format!("§aLevel:§r {}", self.skyblock_level)), root_x.wrapping_add(2), (root_y.wrapping_add(9)).wrapping_add(ROW_GAP), mcrust::net::minecraft::util::common_colors::WHITE, true);
        gui_graphics_extractor::text_string_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), Some(&format!("§6Purse:§r {}", ProfileViewerUtils::num_letter_format(self.purse))), root_x.wrapping_add(2), (root_y.wrapping_add(8)).wrapping_add(ROW_GAP.wrapping_mul(2)), mcrust::net::minecraft::util::common_colors::WHITE, true);
        gui_graphics_extractor::text_string_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), Some(&format!("§6Bank:§r {}", ProfileViewerUtils::num_letter_format(self.bank))), root_x.wrapping_add(2), (root_y.wrapping_add(7)).wrapping_add(ROW_GAP.wrapping_mul(3)), mcrust::net::minecraft::util::common_colors::WHITE, true);
        let mut nw_string: String = format!("§6NW:§r {}", ProfileViewerUtils::num_letter_format(self.networth));
        let mut nw_x: i32 = root_x.wrapping_add(2);
        let mut nw_y: i32 = (root_y.wrapping_add(7)).wrapping_add(ROW_GAP.wrapping_mul(4));
        gui_graphics_extractor::text_string_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), Some(&nw_string), nw_x, nw_y, mcrust::net::minecraft::util::common_colors::WHITE, true);
        if mouse_x >= nw_x && mouse_x <= nw_x.wrapping_add(text_renderer.width(&nw_string)) && mouse_y >= nw_y && mouse_y <= nw_y.wrapping_add(mcrust::net::minecraft::client::gui::font::LINE_HEIGHT) {
            // java: graphics.setComponentTooltipForNextFrame(textRenderer, networthTooltip, mouseX, mouseY);
            // rust: port gap (GAPS_batchE #1) - the frame's `DeferredTooltip` (`tooltip_for_next_frame`) lives in the
            // render loop and is not reachable from mod draw code; the port's call is:
            // rust: gui_graphics_extractor::set_component_tooltip_for_next_frame(&mut text_renderer, self.networth_tooltip.clone(), mouse_x, mouse_y);
        }
        gui_graphics_extractor::text_string_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), Some(&format!("§6MP:§r {}", self.magical_power)), root_x.wrapping_add(2), (root_y.wrapping_add(7)).wrapping_add(ROW_GAP.wrapping_mul(5)), mcrust::net::minecraft::util::common_colors::WHITE, true);
    }

}

/// java: private record ItemValue(String name, double price)
#[derive(Clone, Debug, PartialEq)]
pub struct ItemValue {
    /// java: String name
    pub name: String,
    /// java: double price
    pub price: f64,
}

impl ItemValue {
    /// java: the canonical record constructor
    pub fn new(mut name: String, mut price: f64) -> Self {
        Self { name, price }
    }

    /// java: the record accessor `name()`
    pub fn name(&self) -> String {
        self.name.clone()
    }

    /// java: the record accessor `price()`
    pub fn price(&self) -> f64 {
        self.price
    }

}
