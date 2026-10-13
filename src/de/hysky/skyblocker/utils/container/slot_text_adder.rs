//! Ported from de.hysky.skyblocker.utils.container.SlotTextAdder by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config::SkyblockerConfig;
use crate::de::hysky::skyblocker::skyblock::item::slottext::slot_text::SlotText;
use crate::de::hysky::skyblocker::skyblock::item::slottext::slot_text_manager::SlotTextManager;
use crate::de::hysky::skyblocker::utils::container::container_matcher::ContainerMatcher;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::world::inventory::slot::Slot;

/// java: public interface SlotTextAdder extends ContainerMatcher
pub trait SlotTextAdder: ContainerMatcher {
    /// java: List<SlotText> getText(@Nullable Slot slot, ItemStack stack, int slotId); (dispatch: the body for this class)
    fn get_text(&self) -> fn(&compat::Shared<dyn SlotTextAdder>, Option<Slot>, &mcrust::net::minecraft::world::item::item_stack::ItemStack, i32) -> Vec<SlotText>;

    /// java: default @Nullable ConfigInformation getConfigInformation() (dispatch: the body for this class)
    fn get_config_information(&self) -> fn(&compat::Shared<dyn SlotTextAdder>) -> Option<ConfigInformation> {
        <dyn SlotTextAdder>::get_config_information_body
    }

}

impl dyn SlotTextAdder {
    /// java: @Override default boolean isEnabled()
    pub fn is_enabled_body(__this: &compat::Shared<dyn SlotTextAdder>) -> bool {
        if ({ let __f = compat::dispatch(__this, |__t| __t.get_config_information()); __f(__this) }).is_none() {
            return SlotTextManager::is_enabled_0();
        }
        return SlotTextManager::is_enabled(&({ let __f = compat::dispatch(__this, |__t| __t.get_config_information()); __f(__this) }).unwrap().id());
    }

    /// SlotTextAdder.isEnabled as the dispatch of ContainerMatcher.isEnabled for the concrete class `__C`
    pub fn is_enabled_from<__C: SlotTextAdder + 'static>(__this: &compat::Shared<dyn ContainerMatcher>) -> bool {
        <dyn SlotTextAdder>::is_enabled_body(&compat::view_as::<__C, _, _>(__this, |a| a as compat::Shared<dyn SlotTextAdder>))
    }

    /// java: default @Nullable ConfigInformation getConfigInformation()
    pub fn get_config_information_body(__this: &compat::Shared<dyn SlotTextAdder>) -> Option<ConfigInformation> {
        return None;
    }

}

/// java: record ConfigInformation(String id, Component name, @Nullable Component... description)
#[derive(Clone, Debug)]
pub struct ConfigInformation {
    /// java: String id
    pub id: String,
    /// java: Component name
    pub name: MutableComponent,
    /// java: Component[] description
    pub description: Option<Vec<MutableComponent>>,
}

impl ConfigInformation {
    /// java: the canonical record constructor `ConfigInformation(String id, Component name, @Nullable Component... description)`
    pub fn new_canonical(id: String, name: MutableComponent, description: Option<Vec<MutableComponent>>) -> Self {
        Self { id, name, description }
    }

    /// java: public ConfigInformation(String id, Component name)
    pub fn new(mut id: String, mut name: MutableComponent) -> Self {
        // java: this(id, name, (Component[]) null);
        let mut this = Self::new_canonical(id.clone(), name.clone(), None);
        this
    }

    /// java: public ConfigInformation(String id, @Translatable String name)
    pub fn new2(mut id: String, mut name: String) -> Self {
        let mut this = Self::new(id.clone(), component::translatable(&name));
        this
    }

    /// java: public ConfigInformation(String id, @Translatable String name, @Translatable String... descriptionLines)
    pub fn new3(mut id: String, mut name: String, mut description_lines: Vec<String>) -> Self {
        // java: this(id, name, Stream.of(descriptionLines).map(Component::translatable).toArray(Component[]::new));
        let mut this = Self::new4(id.clone(), name.clone(), description_lines.iter().map(|line| component::translatable(line)).collect());
        this
    }

    /// java: public ConfigInformation(String id, @Translatable String name, Component... descriptionLines)
    pub fn new4(mut id: String, mut name: String, mut description_lines: Vec<MutableComponent>) -> Self {
        // java: this(id, Component.translatable(name), descriptionLines);
        let mut this = Self::new_canonical(id.clone(), component::translatable(&name), Some(description_lines.clone()));
        this
    }

    /// java: the record accessor `id()`
    pub fn id(&self) -> String {
        self.id.clone()
    }

    /// java: the record accessor `name()`
    pub fn name(&self) -> MutableComponent {
        self.name.clone()
    }

    /// java: the record accessor `description()`
    pub fn description(&self) -> Option<Vec<MutableComponent>> {
        self.description.clone()
    }

    /// java: public Option<Boolean> getOption(SkyblockerConfig config)
    pub fn get_option(&self, config: &compat::Shared<SkyblockerConfig>) -> Arc<dyn dandelion::net::azureaaron::dandelion::api::option::Option<bool>> {
        use dandelion::net::azureaaron::dandelion::api::option::{self, Builder as _};
        // java: return Option.<Boolean>createBuilder()
        //       .name(name)
        //       .description(description != null ? description : new Component[0])
        //       .binding(true,
        //               () -> config.uiAndVisuals.slotText.textEnabled.getOrDefault(id, true),
        //               newValue -> config.uiAndVisuals.slotText.textEnabled.put(id, newValue.booleanValue()))
        //       .controller(ConfigUtils.createBooleanController())
        //       .build();
        let (get_id, get_config) = (self.id.clone(), config.clone());
        let (set_id, set_config) = (self.id.clone(), config.clone());
        return option::create_builder::<bool>()
            .name(self.name.clone())
            .description(self.description.clone().unwrap_or_default())
            .binding(
                true,
                move || compat::read(&get_config, |c| c.ui_and_visuals.slot_text.text_enabled.get(&get_id).cloned().unwrap_or(true)),
                move |new_value: bool| compat::write(&set_config, |c| {
                    c.ui_and_visuals.slot_text.text_enabled.insert(set_id.clone(), new_value);
                }),
            )
            .controller(crate::de::hysky::skyblocker::config::config_utils::ConfigUtils::create_boolean_controller())
            .build();
    }

    /// java: @Override public boolean equals(Object obj)
    pub fn equals(&self, obj: &compat::Object) -> bool {
        // java: return obj instanceof ConfigInformation other && Objects.equals(id, other.id);
        return obj.downcast_ref::<ConfigInformation>().is_some_and(|other| self.id == other.id);
    }

    /// java: @Override public int hashCode()
    pub fn hash_code(&self) -> i32 {
        // java: return Objects.hashCode(id); - `String.hashCode`
        return compat::string_hash_code(&self.id);
    }

}
