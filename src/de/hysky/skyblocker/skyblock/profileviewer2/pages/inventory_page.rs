//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.pages.InventoryPage by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::loading_information::LoadingInformation;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::pages::profile_viewer_page::ProfileViewerPage;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::utils::profile_item_storage::ProfileItemStorage;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::widgets::accessory_power_widget::AccessoryPowerWidget;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::widgets::button_widget::ButtonWidget;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::widgets::inventory_widget::InventoryWidget;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::widgets::pagination_widget::PaginationWidget;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::widgets::tuning_slot_widget::TuningSlotWidget;
use crate::de::hysky::skyblocker::skyblock::tabhud::util::ico;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
use mcrust::java::text::decimal_format_symbols::Locale;
use mcrust::java::util::concurrent::completable_future::CompletableFuture;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::abstract_widget::PressLambda;
use mcrust::net::minecraft::client::gui::layouts::frame_layout::FrameLayout;
use mcrust::net::minecraft::client::gui::layouts::layout;
use mcrust::net::minecraft::client::gui::layouts::layout_element::LayoutElement;
use mcrust::net::minecraft::client::gui::layouts::linear_layout::LinearLayout;
use mcrust::net::minecraft::client::gui::layouts::spacer_element::SpacerElement;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: private static final int ACCESSORY_INFO_BOX_WIDTH = 118;
pub const ACCESSORY_INFO_BOX_WIDTH: i32 = 118;

/// java: public final class InventoryPage implements ProfileViewerPage<Pair<LoadingInformation, ProfileItemStorage>>
#[derive(Clone)]
pub struct InventoryPage {
    /// java: private final List<AbstractWidget> widgets = new ArrayList<>();
    pub widgets: Vec<AbstractWidget>,
}

impl InventoryPage {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            widgets: Vec::new(),
        };
        this
    }

    /// java: private static void selectTab(int index, List<LayoutElement> tabContentLayouts)
    pub fn select_tab(mut index: i32, tab_content_layouts: &Vec<LayoutElement>) {
        for i in 0..(tab_content_layouts.len() as i32) {
            let mut should_be_visible: bool = (i == index);
            // TODO(fabric2seer): a lambda for a parameter the port does not type as a closure (`Consumer<net.minecraft.client.gui.components.AbstractWidget>`)
            // rust: (tab_content_layouts[(i) as usize]).visit_widgets(|widget| { widget.visible = should_be_visible; });
        }
    }

    /// java: private static List<List<ItemStack>> divideIntoPages(List<ItemStack> items, int pageSize)
    pub fn divide_into_pages(items: &Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack>, mut page_size: i32) -> Vec<Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack>> {
        return InventoryPage::divide_into_pages_3(items.clone(), page_size, false);
    }

    /// java: private static List<List<ItemStack>> divideIntoPages(List<ItemStack> items, int pageSize, boolean padding)
    pub fn divide_into_pages_3(mut items: Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack>, mut page_size: i32, mut padding: bool) -> Vec<Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack>> {
        let mut page_count: i32 = ((((((items.len() as i32)) as f64) / ((page_size) as f64)).ceil()) as i32);
        if padding {
            let mut max_size: i32 = page_count.wrapping_mul(page_size);
            let mut missing_items: i32 = max_size.wrapping_sub(items.len() as i32);
            let mut appendage: Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack> = vec![mcrust::net::minecraft::world::item::item_stack::empty(); (missing_items) as usize];
            items = items.iter().cloned().collect::<Vec<_>>();
            items.extend(appendage.iter().cloned());
        }
        let mut pages: Vec<Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack>> = Vec::with_capacity((page_count) as usize);
        for i in 0..page_count {
            let mut from_index: i32 = i.wrapping_mul(page_size);
            let mut to_index: i32 = (from_index.wrapping_add(page_size)).min((items.len() as i32));
            pages.push(items[(from_index) as usize..(to_index) as usize].to_vec());
        }
        return pages.iter().cloned().collect::<Vec<_>>();
    }

    /// java: private LayoutElement buildInventoryLayout(ProfileItemStorage itemStorage)
    pub fn build_inventory_layout(&self, item_storage: &ProfileItemStorage) -> LayoutElement {
        let mut layout: LinearLayout = LinearLayout::vertical();
        // TODO(fabric2seer): a widget put in a layout outside a mod screen's own methods: the port's layouts hold a widget by its index in the screen's widget list
        // rust: layout.add_child(InventoryWidget::new(component::literal("Inventory"), 4, 9, vec![item_storage.inventory()], true));
        layout.add_child(SpacerElement::height(6).as_layout_element());
        let mut gear_layout: LinearLayout = LinearLayout::horizontal().spacing(4);
        // TODO(fabric2seer): a widget put in a layout outside a mod screen's own methods: the port's layouts hold a widget by its index in the screen's widget list
        // rust: gear_layout.add_child(InventoryWidget::new(component::literal("Armour"), 1, 4, vec![item_storage.armour()], false));
        // TODO(fabric2seer): a widget put in a layout outside a mod screen's own methods: the port's layouts hold a widget by its index in the screen's widget list
        // rust: gear_layout.add_child(InventoryWidget::new(component::literal("Equipment"), 1, 4, vec![item_storage.equipment()], false));
        layout.add_child_with(LayoutElement::Grid(gear_layout.clone().wrapped), layout.new_cell_settings().align_horizontally_center());
        return LayoutElement::Grid(layout.wrapped);
    }

    /// java: private LayoutElement buildEnderChestLayout(ProfileItemStorage itemStorage)
    pub fn build_ender_chest_layout(&self, item_storage: &ProfileItemStorage) -> LayoutElement {
        let mut pages: Vec<Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack>> = InventoryPage::divide_into_pages(&item_storage.ender_chest_contents(), 5i32.wrapping_mul(9));
        return self.build_paginated_inventory_layout(&component::literal("Ender Chest"), &pages);
    }

    /// java: private LayoutElement buildBackpackLayout(ProfileItemStorage itemStorage)
    pub fn build_backpack_layout(&self, item_storage: &ProfileItemStorage) -> LayoutElement {
        // TODO(fabric2seer): `Backpack.contents` is not a static method of Backpack
        //   java: List<List<ItemStack>> pages = itemStorage.backpacks().values().stream()
        //   java: 				.map(ProfileItemStorage.Backpack::contents)
        //   java: 				.toList();
        let mut pages: Vec<Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack>> = todo!("fabric2seer: the initialiser of `pages` (TODO above)");
        return self.build_paginated_inventory_layout(&component::literal("Backpack"), &pages);
    }

    /// java: private LayoutElement buildGenericWardrobeLayout(String name, List<ItemStack> items)
    pub fn build_generic_wardrobe_layout(&self, name: &str, items: &Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> LayoutElement {
        let mut unordered_pages: Vec<Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack>> = InventoryPage::divide_into_pages_3(items.clone(), 4i32.wrapping_mul(9), true);
        let mut ordered_pages: Vec<Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack>> = Vec::new();
        for page in 0..(unordered_pages.len() as i32) {
            let unordered_page = &unordered_pages[(page) as usize];
            let mut ordered_page: Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack> = Vec::new();
            for offset in 0..4 {
                {
                    let mut i: i32 = offset;
                    while i < (unordered_page.len() as i32) {
                        ordered_page.push(unordered_page[(i) as usize].clone());
                        i = i.wrapping_add(4);
                    }
                }
            }
            ordered_pages.push(ordered_page.iter().cloned().collect::<Vec<_>>());
        }
        return self.build_paginated_inventory_layout_3(&component::literal(name), 4, &ordered_pages);
    }

    /// java: private LayoutElement buildPetsLayout(ProfileItemStorage itemStorage)
    pub fn build_pets_layout(&self, item_storage: &ProfileItemStorage) -> LayoutElement {
        let mut pages: Vec<Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack>> = InventoryPage::divide_into_pages(&item_storage.pets(), 5i32.wrapping_mul(9));
        return self.build_paginated_inventory_layout(&component::literal("Pets"), &pages);
    }

    /// java: private LayoutElement buildAccessoryBagLayout(LoadingInformation info, ProfileItemStorage itemStorage)
    pub fn build_accessory_bag_layout(&self, info: &LoadingInformation, item_storage: &ProfileItemStorage) -> LayoutElement {
        let mut pages: Vec<Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack>> = InventoryPage::divide_into_pages(&item_storage.bags().accessories(), 5i32.wrapping_mul(9));
        let mut accessories: LayoutElement = self.build_paginated_inventory_layout(&component::literal("Accessory Bag"), &pages);
        let mut accessories_stats: LinearLayout = LinearLayout::vertical().spacing(2);
        // TODO(fabric2seer): a widget put in a layout outside a mod screen's own methods: the port's layouts hold a widget by its index in the screen's widget list
        // rust: accessories_stats.add_child(AccessoryPowerWidget::new(ACCESSORY_INFO_BOX_WIDTH, info.clone(), item_storage.clone()));
        // TODO(fabric2seer): a widget put in a layout outside a mod screen's own methods: the port's layouts hold a widget by its index in the screen's widget list
        // rust: accessories_stats.add_child(TuningSlotWidget::new(ACCESSORY_INFO_BOX_WIDTH, info.member().lock().borrow().accessory_bag_storage.tuning.active_slot.clone()));
        let mut accessories_section: LinearLayout = LinearLayout::horizontal();
        accessories_section.add_child(accessories);
        accessories_section.add_child(SpacerElement::width(4).as_layout_element());
        accessories_section.add_child(LayoutElement::Grid(accessories_stats.clone().wrapped));
        return LayoutElement::Grid(accessories_section.wrapped);
    }

    /// java: private LayoutElement buildPaginatedInventoryLayout(Component name, List<List<ItemStack>> pages)
    pub fn build_paginated_inventory_layout(&self, name: &MutableComponent, pages: &Vec<Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack>>) -> LayoutElement {
        return self.build_paginated_inventory_layout_3(name, 5, pages);
    }

    /// java: private LayoutElement buildPaginatedInventoryLayout(Component name, int rows, List<List<ItemStack>> pages)
    pub fn build_paginated_inventory_layout_3(&self, name: &MutableComponent, mut rows: i32, pages: &Vec<Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack>>) -> LayoutElement {
        let mut layout: LinearLayout = LinearLayout::vertical();
        let mut inventory: compat::Shared<InventoryWidget> = InventoryWidget::new(name.clone(), rows, 9, pages.clone(), false);
        // TODO(fabric2seer): a widget put in a layout outside a mod screen's own methods: the port's layouts hold a widget by its index in the screen's widget list
        // rust: layout.add_child(inventory.clone());
        if (pages.len() as i32) > 1 {
            layout.add_child(SpacerElement::height(8).as_layout_element());
            let mut page_button_layout: LayoutElement = self.build_page_button_inventory_layout(&inventory);
            layout.add_child_with(page_button_layout, layout.new_cell_settings().align_horizontally_center());
        }
        return LayoutElement::Grid(layout.wrapped);
    }

    /// java: private LayoutElement buildPageButtonInventoryLayout(InventoryWidget inventory)
    pub fn build_page_button_inventory_layout(&self, inventory: &compat::Shared<InventoryWidget>) -> LayoutElement {
        let mut page_text: AbstractWidget = compat::string_widget(0, 0, -1, 9, component::literal(&format!("Page {}/{}", InventoryWidget::get_page(inventory), InventoryWidget::get_max_pages(inventory))));
        let mut backwards: OnPress = Arc::new(move |_| {
            InventoryWidget::backwards(inventory);
            page_text.set_message_text(mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::literal(&format!("Page {}/{}", InventoryWidget::get_page(inventory), InventoryWidget::get_max_pages(inventory)))));
        });
        let mut forwards: OnPress = Arc::new(move |_| {
            InventoryWidget::forwards(inventory);
            page_text.set_message_text(mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::literal(&format!("Page {}/{}", InventoryWidget::get_page(inventory), InventoryWidget::get_max_pages(inventory)))));
        });
        return self.build_page_button_layout(&page_text, &backwards, &forwards);
    }

    /// java: private LayoutElement buildPageButtonLayout(StringWidget pageText, Button.OnPress backwards, Button.OnPress forwards)
    pub fn build_page_button_layout(&self, page_text: &AbstractWidget, backwards: &OnPress, forwards: &OnPress) -> LayoutElement {
        let mut layout: LinearLayout = LinearLayout::horizontal().spacing(8);
        layout.add_child(PaginationWidget::new(false, *backwards));
        // TODO(fabric2seer): a widget put in a layout outside a mod screen's own methods: the port's layouts hold a widget by its index in the screen's widget list
        // rust: layout.add_child_with(page_text.clone(), layout.new_cell_settings().align_vertically_middle());
        layout.add_child(PaginationWidget::new(true, *forwards));
        return LayoutElement::Grid(layout.wrapped);
    }

}

impl ProfileViewerPage<(crate::de::hysky::skyblocker::skyblock::profileviewer2::loading_information::LoadingInformation, crate::de::hysky::skyblocker::skyblock::profileviewer2::utils::profile_item_storage::ProfileItemStorage)> for InventoryPage {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public FlexibleItemStack getIcon()
    fn get_icon(&self) -> compat::Shared<FlexibleItemStack> {
        return ico::E_CHEST.get_mut().clone();
    }

    /// java: @Override public Component getName()
    fn get_name(&self) -> MutableComponent {
        return component::literal("Inventory");
    }

    /// java: @Override public CompletableFuture<LayoutElement> load(LoadingInformation info)
    fn load(&self, info: &LoadingInformation) -> mcrust::java::util::concurrent::completable_future::CompletableFuture<LayoutElement> {
        // TODO(fabric2seer): Pair.of
        //   java: return CompletableFuture.completedFuture(info)
        //   java: 				.thenCombineAsync(info.itemStorage(), Pair::of)
        //   java: 				.thenApplyAsync(this::buildWidgets, Minecraft.getInstance());
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @SuppressWarnings("unused") @Override public LayoutElement buildWidgets(Pair<LoadingInformation, ProfileItemStorage> data)
    fn build_widgets(&self, data: &(LoadingInformation, ProfileItemStorage)) -> LayoutElement {
        let info = &data.0;
        let item_storage = &data.1;
        let mut page_layout: LinearLayout = LinearLayout::horizontal();
        let mut tab_content_layouts: Vec<LayoutElement> = vec![self.build_inventory_layout(item_storage), self.build_ender_chest_layout(item_storage), self.build_backpack_layout(item_storage), self.build_generic_wardrobe_layout("Armour Sets", &item_storage.armour_sets()), self.build_generic_wardrobe_layout("Equipment Sets", &item_storage.equipment_sets()), self.build_pets_layout(item_storage), self.build_accessory_bag_layout(info, item_storage)];
        let mut tab_buttons: Vec<ButtonWidget> = vec![ButtonWidget::new(ico::CHEST.get_mut().clone(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { InventoryPage::select_tab(0, &tab_content_layouts); }))), ButtonWidget::new(ico::E_CHEST.get_mut().clone(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { InventoryPage::select_tab(1, &tab_content_layouts); }))), ButtonWidget::new(ico::JUMBO_BACKPACK.get_mut().clone(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { InventoryPage::select_tab(2, &tab_content_layouts); }))), ButtonWidget::new(ico::L_CHESTPLATE.get_mut().clone(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { InventoryPage::select_tab(3, &tab_content_layouts); }))), ButtonWidget::new(ico::BROWN_HARNESS.get_mut().clone(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { InventoryPage::select_tab(4, &tab_content_layouts); }))), ButtonWidget::new(ico::BONE.get_mut().clone(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { InventoryPage::select_tab(5, &tab_content_layouts); }))), ButtonWidget::new(ico::ACCESSORY_BAG.get_mut().clone(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { InventoryPage::select_tab(6, &tab_content_layouts); })))];
        let mut tab_layout: LinearLayout = LinearLayout::vertical().spacing(1);
        for button in tab_buttons.iter() {
            tab_layout.add_child(button.clone());
        }
        page_layout.add_child_with(LayoutElement::Grid(tab_layout.clone().wrapped), page_layout.new_cell_settings().align_vertically_middle());
        page_layout.add_child(SpacerElement::width(16).as_layout_element());
        let mut content_frame: FrameLayout = FrameLayout::new();
        for layout in tab_content_layouts.iter().copied() {
            content_frame.add_child_with(layout, content_frame.new_child_layout_settings().align_horizontally_left());
        }
        page_layout.add_child(LayoutElement::Frame(content_frame.clone()));
        // TODO(fabric2seer): a lambda for a parameter the port does not type as a closure (`Consumer<net.minecraft.client.gui.components.AbstractWidget>`)
        // rust: layout::visit_widgets(|__a0| { self.widgets.push(__a0.clone()); });
        InventoryPage::select_tab(0, &tab_content_layouts);
        return LayoutElement::Grid(page_layout.wrapped);
    }

    /// java: @Override public List<AbstractWidget> getWidgets()
    fn get_widgets(&self) -> Vec<AbstractWidget> {
        return self.widgets.clone();
    }

}
