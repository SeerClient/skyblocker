//! Ported from de.hysky.skyblocker.config.datafixer.ConfigFix1 by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::datafixer::config_data_fix::ConfigDataFix;
use crate::de::hysky::skyblocker::config::datafixer::config_data_fix::ConfigDataFixBase;
use crate::de::hysky::skyblocker::config::datafixer::config_data_fixer;
use crate::de::hysky::skyblocker::utils::datafixer::item_stack_componentization_fixer::ItemStackComponentizationFixer;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
use mcrust::com::mojang::datafixers::data_fix;
use mcrust::com::mojang::datafixers::dsl;
use mcrust::com::mojang::datafixers::schemas::schema::Schema;
use mcrust::com::mojang::datafixers::type_rewrite_rule::TypeRewriteRule;
use mcrust::com::mojang::serialization::dynamic::Dynamic;
use mcrust::com::mojang::serialization::optional_dynamic::OptionalDynamic;
use mcrust::java::text::decimal_format_symbols::Locale;
use mcrust::net::minecraft::nbt::tag_parser;
use mcrust::com::mojang::datafixers::data_fix::DataFix as _;
use mcrust::com::mojang::serialization::dynamic_like::DynamicLike as _;

/// java: public class ConfigFix1 extends ConfigDataFix
pub struct ConfigFix1 {
    /// java: extends ConfigDataFix
    pub base: ConfigDataFixBase,
}

impl std::ops::Deref for ConfigFix1 {
    type Target = ConfigDataFixBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for ConfigFix1 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl ConfigFix1 {
    /// java: public ConfigFix1(Schema outputSchema, boolean changesType)
    pub fn new(mut output_schema: Arc<Schema>, mut changes_type: bool) -> compat::Shared<Self> {
        let (_, __this) = ConfigDataFixBase::new_in(output_schema.clone(), changes_type, move |__base| {
            let __h = compat::shared(Self {
                base: __base,
            });
            ((__h.clone() as compat::Shared<dyn ConfigDataFix>), __h)
        });
        __this
    }

    /// java: private static <T> Dynamic<T> fixGeneral(Dynamic<T> dynamic)
    pub fn fix_general<T: 'static>(dynamic: &mut Dynamic) -> Dynamic {
        return dynamic.update("general", |general| (general.update("itemTooltip", |item_tooltip| item_tooltip.set_field_if_present("dungeonQuality", general.get("dungeonQuality").result()))).remove("dungeonQuality"));
    }

    /// java: private static <T> Dynamic<T> fixUIAndVisuals(Dynamic<T> dynamic)
    pub fn fix_ui_and_visuals<T: 'static>(dynamic: &mut Dynamic) -> Dynamic {
        let mut general: OptionalDynamic = dynamic.get("general");
        return (dynamic.set("uiAndVisuals", &(dynamic.empty_map().set_field_if_present("compactorDeletorPreview", general.get("compactorDeletorPreview").result())).set_field_if_present("dontStripSkinAlphaValues", general.get("dontStripSkinAlphaValues").result()).set_field_if_present("backpackPreviewWithoutShift", general.get("backpackPreviewWithoutShift").result()).set_field_if_present("hideEmptyTooltips", general.get("hideEmptyTooltips").result()).set_field_if_present("fancyCraftingTable", general.get("fancyCraftingTable").result()).set_field_if_present("hideStatusEffectOverlay", general.get("hideStatusEffectOverlay").result()).set_field_if_present("chestValue", general.get("chestValue").result()).set_field_if_present("itemCooldown", general.get("itemCooldown").result()).set_field_if_present("titleContainer", general.get("titleContainer").result()).set_field_if_present("tabHud", general.get("tabHud").result()).set_field_if_present("fancyAuctionHouse", general.get("fancyAuctionHouse").result()).set_field_if_present("bars", general.get("bars").result()).set_field_if_present("waypoints", general.get("waypoints").result()).set_field_if_present("teleportOverlay", general.get("teleportOverlay").result()).set_field_if_present("searchOverlay", general.get("searchOverlay").result()).set_field_if_present("flameOverlay", general.get("flameOverlay").result()))).update("general", |new_general| new_general.remove("compactorDeletorPreview").remove("dontStripSkinAlphaValues").remove("backpackPreviewWithoutShift").remove("hideEmptyTooltips").remove("fancyCraftingTable").remove("hideStatusEffectOverlay").remove("chestValue").remove("itemCooldown").remove("titleContainer").remove("tabHud").remove("fancyAuctionHouse").remove("bars").remove("waypoints").remove("teleportOverlay").remove("searchOverlay").remove("flameOverlay"));
    }

    /// java: private static <T> Dynamic<T> fixHelpers(Dynamic<T> dynamic)
    pub fn fix_helpers<T: 'static>(dynamic: &mut Dynamic) -> Dynamic {
        let mut general: OptionalDynamic = dynamic.get("general");
        return (dynamic.set("helpers", &(dynamic.empty_map().set_field_if_present("enableNewYearCakesHelper", general.get("enableNewYearCakesHelper").result())).set_field_if_present("mythologicalRitual", general.get("mythologicalRitual").result()).set_field_if_present("experiments", general.get("experiments").result()).set_field_if_present("fishing", general.get("fishing").result()).set_field_if_present("fairySouls", general.get("fairySouls").result()))).update("general", |new_general| new_general.remove("enableNewYearCakesHelper").remove("mythologicalRitual").remove("experiments").remove("fishing").remove("fairySouls"));
    }

    /// java: private static <T> Dynamic<T> fixDungeons(Dynamic<T> dynamic)
    pub fn fix_dungeons<T: 'static>(dynamic: &mut Dynamic) -> Dynamic {
        let mut general: OptionalDynamic = dynamic.get("general");
        let mut dungeons: OptionalDynamic = dynamic.get("locations").get("dungeons");
        return (dynamic.set("dungeons", &(dynamic.empty_map().set_field_if_present("fancyPartyFinder", general.get("betterPartyFinder").result())).set_field_if_present("croesusHelper", dungeons.get("croesusHelper").result()).set_field_if_present("playerSecretsTracker", dungeons.get("playerSecretsTracker").result()).set_field_if_present("starredMobGlow", dungeons.get("starredMobGlow").result()).set_field_if_present("starredMobBoundingBoxes", dungeons.get("starredMobBoundingBoxes").result()).set_field_if_present("allowDroppingProtectedItems", dungeons.get("allowDroppingProtectedItems").result()).set("dungeonMap", &(dynamic.empty_map().set_field_if_present("enableMap", dungeons.get("enableMap").result())).set_field_if_present("mapScaling", dungeons.get("mapScaling").result()).set_field_if_present("mapX", dungeons.get("mapX").result()).set_field_if_present("mapY", dungeons.get("mapY").result())).set("puzzleSolvers", &(dynamic.empty_map().set_field_if_present("solveThreeWeirdos", dungeons.get("solveThreeWeirdos").result())).set_field_if_present("blazeSolver", dungeons.get("blazeSolver").result()).set_field_if_present("creeperSolver", dungeons.get("creeperSolver").result()).set_field_if_present("solveTrivia", dungeons.get("solveTrivia").result()).set_field_if_present("solveTicTacToe", dungeons.get("solveTicTacToe").result()).set_field_if_present("solveWaterboard", dungeons.get("solveWaterboard").result()).set_field_if_present("solveBoulder", dungeons.get("solveBoulder").result()).set_field_if_present("solveIceFill", dungeons.get("solveIceFill").result()).set_field_if_present("solveSilverfish", dungeons.get("solveSilverfish").result())).set("theProfessor", &(dynamic.empty_map().set_field_if_present("fireFreezeStaffTimer", dungeons.get("fireFreezeStaffTimer").result())).set_field_if_present("floor3GuardianHealthDisplay", dungeons.get("floor3GuardianHealthDisplay").result())).set_field_if_present("livid", dungeons.get("lividColor").result()).set_field_if_present("terminals", dungeons.get("terminals").result()).set_field_if_present("secretWaypoints", dungeons.get("secretWaypoints").result()).set_field_if_present("mimicMessage", dungeons.get("mimicMessage").result()).set_field_if_present("doorHighlight", dungeons.get("doorHighlight").result()).set_field_if_present("dungeonScore", dungeons.get("dungeonScore").result()).set_field_if_present("dungeonChestProfit", dungeons.get("dungeonChestProfit").result()))).update("locations", |locations| locations.remove("dungeons")).update("general", |new_general| new_general.remove("betterPartyFinder"));
    }

    /// java: private static <T> Dynamic<T> fixCrimsonIsle(Dynamic<T> dynamic)
    pub fn fix_crimson_isle<T: 'static>(dynamic: &mut Dynamic) -> Dynamic {
        return (dynamic.set_field_if_present("crimsonIsle", dynamic.get("locations").get("crimsonIsle").result())).update("locations", |locations| locations.remove("crimsonIsle"));
    }

    /// java: private static <T> Dynamic<T> fixMining(Dynamic<T> dynamic)
    pub fn fix_mining<T: 'static>(dynamic: &mut Dynamic) -> Dynamic {
        let mut dwarven_mines: OptionalDynamic = dynamic.get("locations").get("dwarvenMines");
        // TODO(fabric2seer): Optional.renameField
        //   java: return dynamic.set("mining", dynamic.emptyMap()
        //   java: 				.setFieldIfPresent("enableDrillFuel", dwarvenMines.get("enableDrillFuel").result())
        //   java: 				.set("dwarvenMines", dynamic.emptyMap()
        //   java: 						.setFieldIfPresent("solveFetchur", dwarvenMines.get("solveFetchur").result())
        //   java: 						.setFieldIfPresent("solvePuzzler", dwarvenMines.get("solvePuzzler").result())
        //   java: 				)
        //   java: 				.set("dwarvenHud", dwarvenMines.get("dwarvenHud").result().orElseThrow()
        //   java: 						.renameField("x", "commissionsX")
        //   java: 						.renameField("y", "commissionsY")
        //   java: 				)
        //   java: 				.setFieldIfPresent("crystalsHud", dwarvenMines.get("crystalsHud").result())
        //   java: 				.setFieldIfPresent("crystalsWaypoints", dwarvenMines.get("crystalsWaypoints").result())
        //   java: 				.set("crystalHollows", dynamic.emptyMap()
        //   java: 						.setFieldIfPresent("metalDetectorHelper", dwarvenMines.get("metalDetectorHelper").result())
        //   java: 				)
        //   java: 		).update("locations", locations -> locations.remove("dwarvenMines"));
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: private static <T> Dynamic<T> fixFarming(Dynamic<T> dynamic)
    pub fn fix_farming<T: 'static>(dynamic: &mut Dynamic) -> Dynamic {
        return (dynamic.set("farming", &dynamic.empty_map().set_field_if_present("garden", dynamic.get("locations").get("garden").result()))).update("locations", |locations| locations.remove("garden"));
    }

    /// java: private static <T> Dynamic<T> fixOtherLocations(Dynamic<T> dynamic)
    pub fn fix_other_locations<T: 'static>(dynamic: &Dynamic) -> Dynamic {
        return dynamic.rename_field("locations", "otherLocations");
    }

    /// java: private static <T> Dynamic<T> fixSlayers(Dynamic<T> dynamic)
    pub fn fix_slayers<T: 'static>(dynamic: &Dynamic) -> Dynamic {
        return dynamic.rename_field("slayer", "slayers");
    }

    /// java: private static <T> Dynamic<T> fixChat(Dynamic<T> dynamic)
    pub fn fix_chat<T: 'static>(dynamic: &Dynamic) -> Dynamic {
        return dynamic.rename_field("messages", "chat");
    }

    /// java: private static <T> Dynamic<T> fixQuickNav(Dynamic<T> dynamic)
    pub fn fix_quick_nav<T: 'static>(dynamic: &mut Dynamic) -> Dynamic {
        // TODO(fabric2seer): Map.Entry.mapSecond
        //   java: return dynamic.update("quickNav", quickNav -> quickNav.updateMapValues(button ->
        //   java: 				button.getFirst().asString().getOrThrow().startsWith("button") ? button.mapSecond(ConfigFix1::fixQuickNavButton) : button
        //   java: 		));
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: private static <T> Dynamic<T> fixQuickNavButton(Dynamic<T> button)
    pub fn fix_quick_nav_button<T: 'static>(button: &mut Dynamic) -> Dynamic {
        return button.update("item", |item| (item.rename_field("itemName", "id")).rename_and_fix_field("nbt", "components", |nbt| ConfigFix1::fix_nbt(&item.get("itemName"), &nbt)));
    }

    /// java: private static Dynamic<?> fixNbt(OptionalDynamic<?> id, Dynamic<?> nbt)
    pub fn fix_nbt(id: &OptionalDynamic, nbt: &Dynamic) -> Dynamic {
        // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`; the body's `return` is the closure's value
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut item_nbt: String = format!("{{id:\"minecraft:{}\",Count:1", id.as_string().get_or_throw().unwrap().to_lowercase());
            let mut extra_nbt: String = nbt.as_string().get_or_throw().unwrap();
            if compat::str_len(&extra_nbt) > 2 {
                item_nbt.push_str(&format!(",{}", extra_nbt).to_string());
            }
            item_nbt.push_str("}");
            let mut fixed: compat::Shared<FlexibleItemStack> = ItemStackComponentizationFixer::fix_up_flexible_item(&tag_parser::parse_compound_fully(&item_nbt).unwrap());
            return nbt.create_string(ItemStackComponentizationFixer::components_as_string_data_component_patch(&FlexibleItemStack::components(&fixed)).as_deref().unwrap());
        })) {
            Ok(__v) => return __v,
            Err(e) => {
                // TODO(fabric2seer): a log call with a non-literal format
                //   java: ConfigDataFixer.LOGGER.error(LogUtils.FATAL_MARKER, "[Skyblocker Config Data Fixer] Failed to convert nbt to components!", e);
            }
        }
        return nbt.create_string("[]");
    }

    /// java: private static <T> Dynamic<T> fixMisc(Dynamic<T> dynamic)
    pub fn fix_misc<T: 'static>(dynamic: &mut Dynamic) -> Dynamic {
        return (dynamic.set("misc", &dynamic.empty_map().set_field_if_present("richPresence", dynamic.get("richPresence").result()))).remove("richPresence");
    }

    /// java: @Override protected TypeRewriteRule makeRule()
    pub fn make_rule(__this: &compat::Shared<Self>) -> TypeRewriteRule {
        let __this_w1 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        return data_fix::fix_type_everywhere_typed_same("ConfigFix1", { let (__q0,) = (config_data_fixer::CONFIG_TYPE.get_mut(),); __this.lock().borrow().get_input_schema().get_type(__q0) }.unwrap(), Arc::new(move |typed| typed.update(&dsl::remainder_finder(), &|__a0| { let Some(__this_1) = __this_w1.get().upgrade() else { return Default::default(); }; ConfigFix1::fix((&__this_1), &mut __a0) }).unwrap()));
    }

    /// java: private <T> Dynamic<T> fix(Dynamic<T> dynamic)
    pub fn fix<T: 'static>(__this: &compat::Shared<Self>, dynamic: &mut Dynamic) -> Dynamic {
        return ConfigFix1::fix_misc(&mut ConfigFix1::fix_quick_nav(&mut ConfigFix1::fix_chat(&ConfigFix1::fix_slayers(&ConfigFix1::fix_other_locations(&ConfigFix1::fix_farming(&mut ConfigFix1::fix_mining(&mut ConfigFix1::fix_crimson_isle(&mut ConfigFix1::fix_dungeons(&mut ConfigFix1::fix_helpers(&mut ConfigFix1::fix_ui_and_visuals(&mut ConfigFix1::fix_general(&mut __this.lock().borrow().fix_version(dynamic)))))))))))));
    }

}

impl ConfigDataFix for ConfigFix1 {
    fn config_data_fix_base(&self) -> &ConfigDataFixBase {
        &self.base
    }
    fn config_data_fix_base_mut(&mut self) -> &mut ConfigDataFixBase {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

}
