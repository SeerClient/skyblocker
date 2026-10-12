//! Ported from de.hysky.skyblocker.config.configs.QuickNavigationConfig by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::net::minecraft::world::item::item::Item;
use mcrust::net::minecraft::world::item::items;

/// java: public class QuickNavigationConfig
#[derive(Clone)]
pub struct QuickNavigationConfig {
    /// java: public boolean enableQuickNav = true;
    pub enable_quick_nav: bool,
    /// java: public QuickNavItem button1 = new QuickNavItem(false, new ItemData(Items.DIAMOND_SWORD), "Your Skills", "/skills", "Skills");
    pub button1: compat::Shared<QuickNavItem>,
    /// java: public QuickNavItem button2 = new QuickNavItem(false, new ItemData(Items.PAINTING), "Collections", "/collection", "Collections");
    pub button2: compat::Shared<QuickNavItem>,
    /// java: public QuickNavItem button3 = new QuickNavItem(false, new ItemData(Items.BONE), "(?:\\(\\d+/\\d+\\) )?Pets", "/pets", "Pets");
    pub button3: compat::Shared<QuickNavItem>,
    /// java: public QuickNavItem button4 = new QuickNavItem(false, new ItemData(Items.LEATHER_CHESTPLATE, "[minecraft:dyed_color=8991416]"), "\\(\\d+/\\d+\\) Armor Sets", "/
    pub button4: compat::Shared<QuickNavItem>,
    /// java: public QuickNavItem button5 = new QuickNavItem(false, new ItemData(Items.PLAYER_HEAD, "[minecraft:profile={id:[I;-2081424676,-57521078,-2073572414,158072763],na
    pub button5: compat::Shared<QuickNavItem>,
    /// java: public QuickNavItem button6 = new QuickNavItem(false, new ItemData(Items.PLAYER_HEAD, "[minecraft:profile={name:\"5da6bec64bd942bc\",id:[I;1571208902,1272529596
    pub button6: compat::Shared<QuickNavItem>,
    /// java: public QuickNavItem button7 = new QuickNavItem(false, new ItemData(Items.ENDER_CHEST), "(?:Rift )?Storage(?: \\(\\d/\\d\\))?", "/storage", "Storage");
    pub button7: compat::Shared<QuickNavItem>,
    /// java: public QuickNavItem button8 = new QuickNavItem(true, new ItemData(Items.PLAYER_HEAD, "[minecraft:profile={name:\"421a8ef40eff47f4\",id:[I;1109036788,251611124,-
    pub button8: compat::Shared<QuickNavItem>,
    /// java: public QuickNavItem button9 = new QuickNavItem(true, new ItemData(Items.PLAYER_HEAD, "[minecraft:profile={name:\"e30e30d02878417c\",id:[I;-485609264,678969724,-
    pub button9: compat::Shared<QuickNavItem>,
    /// java: public QuickNavItem button10 = new QuickNavItem(true, new ItemData(Items.PLAYER_HEAD, "[minecraft:profile={id:[I;-300151517,-631415889,-1193921967,-1821784279],
    pub button10: compat::Shared<QuickNavItem>,
    /// java: public QuickNavItem button11 = new QuickNavItem(true, new ItemData(Items.PLAYER_HEAD, "[minecraft:profile={id:[I;1605800870,415127827,-1236127084,15358548],name
    pub button11: compat::Shared<QuickNavItem>,
    /// java: public QuickNavItem button12 = new QuickNavItem(false, new ItemData(Items.GOLD_BLOCK), "^(?:Auctions Browser|Co-op Auction House|Auction House|Manage Auctions|Y
    pub button12: compat::Shared<QuickNavItem>,
    /// java: public QuickNavItem button13 = new QuickNavItem(false, new ItemData(Items.PLAYER_HEAD, "[minecraft:profile={id:[I;-562285948,532499670,-1705302742,775653035],na
    pub button13: compat::Shared<QuickNavItem>,
    /// java: public QuickNavItem button14 = new QuickNavItem(false, new ItemData(Items.CRAFTING_TABLE), "Craft Item", "/craft", "Crafting Table");
    pub button14: compat::Shared<QuickNavItem>,
}

impl QuickNavigationConfig {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            enable_quick_nav: true,
            button1: QuickNavItem::new_5(false, ItemData::new(items::items().get("diamond_sword")), "Your Skills".to_string(), "/skills".to_string(), "Skills".to_string()),
            button2: QuickNavItem::new_5(false, ItemData::new(items::items().get("painting")), "Collections".to_string(), "/collection".to_string(), "Collections".to_string()),
            button3: QuickNavItem::new_5(false, ItemData::new(items::items().get("bone")), "(?:\\(\\d+/\\d+\\) )?Pets".to_string(), "/pets".to_string(), "Pets".to_string()),
            button4: QuickNavItem::new_5(false, ItemData::new_2(items::items().get("leather_chestplate"), "[minecraft:dyed_color=8991416]".to_string()), "\\(\\d+/\\d+\\) Armor Sets".to_string(), "/armor".to_string(), "Armor Sets".to_string()),
            button5: QuickNavItem::new_5(false, ItemData::new_2(items::items().get("player_head"), "[minecraft:profile={id:[I;-2081424676,-57521078,-2073572414,158072763],name:\"\",properties:[{name:\"textures\",value:\"ewogICJ0aW1lc3RhbXAiIDogMTU5MTMxMDU4NTYwOSwKICAicHJvZmlsZUlkIiA6ICI0MWQzYWJjMmQ3NDk0MDBjOTA5MGQ1NDM0ZDAzODMxYiIsCiAgInByb2ZpbGVOYW1lIiA6ICJNZWdha2xvb24iLAogICJzaWduYXR1cmVSZXF1aXJlZCIgOiB0cnVlLAogICJ0ZXh0dXJlcyIgOiB7CiAgICAiU0tJTiIgOiB7CiAgICAgICJ1cmwiIDogImh0dHA6Ly90ZXh0dXJlcy5taW5lY3JhZnQubmV0L3RleHR1cmUvODBhMDc3ZTI0OGQxNDI3NzJlYTgwMDg2NGY4YzU3OGI5ZDM2ODg1YjI5ZGFmODM2YjY0YTcwNjg4MmI2ZWMxMCIKICAgIH0KICB9Cn0=\"}]}]".to_string()), "Sack of Sacks".to_string(), "/sacks".to_string(), "Sacks".to_string()),
            button6: QuickNavItem::new_5(false, ItemData::new_2(items::items().get("player_head"), "[minecraft:profile={name:\"5da6bec64bd942bc\",id:[I;1571208902,1272529596,-1566400349,-679283814],properties:[{name:\"textures\",value:\"eyJ0ZXh0dXJlcyI6eyJTS0lOIjp7InVybCI6Imh0dHA6Ly90ZXh0dXJlcy5taW5lY3JhZnQubmV0L3RleHR1cmUvOTYxYTkxOGMwYzQ5YmE4ZDA1M2U1MjJjYjkxYWJjNzQ2ODkzNjdiNGQ4YWEwNmJmYzFiYTkxNTQ3MzA5ODVmZiJ9fX0=\"}]}]".to_string()), "Accessory Bag(?: \\(\\d/\\d\\))?".to_string(), "/accessories".to_string(), "Accessories".to_string()),
            button7: QuickNavItem::new_5(false, ItemData::new(items::items().get("ender_chest")), "(?:Rift )?Storage(?: \\(\\d/\\d\\))?".to_string(), "/storage".to_string(), "Storage".to_string()),
            button8: QuickNavItem::new_4(true, ItemData::new_2(items::items().get("player_head"), "[minecraft:profile={name:\"421a8ef40eff47f4\",id:[I;1109036788,251611124,-2126904485,-130621758],properties:[{name:\"textures\",value:\"eyJ0ZXh0dXJlcyI6eyJTS0lOIjp7InVybCI6Imh0dHA6Ly90ZXh0dXJlcy5taW5lY3JhZnQubmV0L3RleHR1cmUvYzljODg4MWU0MjkxNWE5ZDI5YmI2MWExNmZiMjZkMDU5OTEzMjA0ZDI2NWRmNWI0MzliM2Q3OTJhY2Q1NiJ9fX0=\"}]}]".to_string()), "/is".to_string(), "Home".to_string()),
            button9: QuickNavItem::new_4(true, ItemData::new_2(items::items().get("player_head"), "[minecraft:profile={name:\"e30e30d02878417c\",id:[I;-485609264,678969724,-1929747597,-718202427],properties:[{name:\"textures\",value:\"eyJ0ZXh0dXJlcyI6eyJTS0lOIjp7InVybCI6Imh0dHA6Ly90ZXh0dXJlcy5taW5lY3JhZnQubmV0L3RleHR1cmUvZjQ4ODBkMmMxZTdiODZlODc1MjJlMjA4ODI2NTZmNDViYWZkNDJmOTQ5MzJiMmM1ZTBkNmVjYWE0OTBjYjRjIn19fQ==\"}]}]".to_string()), "/warp garden".to_string(), "Garden".to_string()),
            button10: QuickNavItem::new_4(true, ItemData::new_2(items::items().get("player_head"), "[minecraft:profile={id:[I;-300151517,-631415889,-1193921967,-1821784279],name:\"\",properties:[{name:\"textures\",value:\"e3RleHR1cmVzOntTS0lOOnt1cmw6Imh0dHA6Ly90ZXh0dXJlcy5taW5lY3JhZnQubmV0L3RleHR1cmUvZDdjYzY2ODc0MjNkMDU3MGQ1NTZhYzUzZTA2NzZjYjU2M2JiZGQ5NzE3Y2Q4MjY5YmRlYmVkNmY2ZDRlN2JmOCJ9fX0=\"}]}]".to_string()), "/hub".to_string(), "Skyblock Hub".to_string()),
            button11: QuickNavItem::new_4(true, ItemData::new_2(items::items().get("player_head"), "[minecraft:profile={id:[I;1605800870,415127827,-1236127084,15358548],name:\"\",properties:[{name:\"textures\",value:\"e3RleHR1cmVzOntTS0lOOnt1cmw6Imh0dHA6Ly90ZXh0dXJlcy5taW5lY3JhZnQubmV0L3RleHR1cmUvNzg5MWQ1YjI3M2ZmMGJjNTBjOTYwYjJjZDg2ZWVmMWM0MGExYjk0MDMyYWU3MWU3NTQ3NWE1NjhhODI1NzQyMSJ9fX0=\"}]}]".to_string()), "/warp dungeon_hub".to_string(), "Dungeons Hub".to_string()),
            button12: QuickNavItem::new_5(false, ItemData::new(items::items().get("gold_block")), "^(?:Auctions Browser|Co-op Auction House|Auction House|Manage Auctions|Your Bids|BIN Auction View|Auction View|Auctions:.*)$".to_string(), "/ah".to_string(), "Auction House".to_string()),
            button13: QuickNavItem::new_5(false, ItemData::new_2(items::items().get("player_head"), "[minecraft:profile={id:[I;-562285948,532499670,-1705302742,775653035],name:\"\",properties:[{name:\"textures\",value:\"e3RleHR1cmVzOntTS0lOOnt1cmw6Imh0dHA6Ly90ZXh0dXJlcy5taW5lY3JhZnQubmV0L3RleHR1cmUvZmZlMmRjZGE0MWVjM2FmZjhhZjUwZjI3MmVjMmUwNmE4ZjUwOWUwZjgwN2YyMzU1YTFmNWEzM2MxYjY2ZTliNCJ9fX0=\"}]}]".to_string()), "(?:Co-op )?Bazaar .*".to_string(), "/bz".to_string(), "Bazaar".to_string()),
            button14: QuickNavItem::new_5(false, ItemData::new(items::items().get("crafting_table")), "Craft Item".to_string(), "/craft".to_string(), "Crafting Table".to_string()),
        };
        this
    }

}

/// java: public static class QuickNavItem
#[derive(Clone)]
pub struct QuickNavItem {
    /// java: public boolean render = true;
    pub render: bool,
    /// java: public boolean doubleClick = false;
    pub double_click: bool,
    /// java: public ItemData itemData;
    pub item_data: Option<ItemData>,
    /// java: public String uiTitle;
    pub ui_title: String,
    /// java: public String tooltip = "";
    pub tooltip: String,
    /// java: public String clickEvent;
    pub click_event: String,
}

impl QuickNavItem {
    /// java: @SuppressWarnings("unused") private QuickNavItem()
    pub fn new() -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            render: true,
            double_click: false,
            item_data: None,
            ui_title: Default::default(),
            tooltip: "".to_string(),
            click_event: Default::default(),
        });
        __this
    }

    /// java: public QuickNavItem(boolean doubleClick, ItemData itemData, String clickEvent, String tooltip)
    pub fn new_4(mut double_click: bool, mut item_data: ItemData, mut click_event: String, mut tooltip: String) -> compat::Shared<Self> {
        let __this = Self::new_5(double_click, item_data.clone(), "none".to_string(), click_event.clone(), tooltip.clone());
        __this
    }

    /// java: public QuickNavItem(boolean doubleClick, ItemData itemData, @Language("RegExp") String uiTitle, String clickEvent, String tooltip)
    pub fn new_5(mut double_click: bool, mut item_data: ItemData, mut ui_title: String, mut click_event: String, mut tooltip: String) -> compat::Shared<Self> {
        let __init_double_click = double_click;
        let __init_item_data = Some(item_data.clone());
        let __init_ui_title = ui_title.clone();
        let __init_click_event = click_event.clone();
        let __init_tooltip = tooltip.clone();
        let __this = compat::shared(Self {
            render: true,
            double_click: __init_double_click,
            item_data: __init_item_data,
            ui_title: __init_ui_title,
            tooltip: __init_tooltip,
            click_event: __init_click_event,
        });
        __this
    }

}

/// java: public static class ItemData
#[derive(Clone)]
pub struct ItemData {
    /// java: public Item item;
    pub item: Option<Arc<Item>>,
    /// java: public int count = 1;
    pub count: i32,
    /// java: public String components;
    pub components: String,
}

impl ItemData {
    /// java: public ItemData(Item item)
    pub fn new(mut item: Arc<Item>) -> Self {
        let mut this = Self::new_2(item.clone(), "[]".to_string());
        this
    }

    /// java: public ItemData(Item item, String components)
    pub fn new_2(mut item: Arc<Item>, mut components: String) -> Self {
        let __init_item = Some(item.clone());
        let __init_components = components.clone();
        let mut this = Self {
            item: __init_item,
            count: 1,
            components: __init_components,
        };
        this
    }

}
