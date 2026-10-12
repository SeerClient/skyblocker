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
use mcrust::com::mojang::datafixers::data_fix::{DataFix, DataFixFields};
use mcrust::com::mojang::datafixers::typed::Typed;
use mcrust::com::mojang::serialization::dynamic_like::DynamicLike as _;

/// java: public class ConfigFix1 extends ConfigDataFix
pub struct ConfigFix1 {
    /// java: extends ConfigDataFix
    pub base: ConfigDataFixBase,
    /// java: `this`, captured by the rule's lambdas (`this::fix`)
    self_ref: std::sync::Weak<ConfigFix1>,
}

impl ConfigFix1 {
    /// java: public ConfigFix1(Schema outputSchema, boolean changesType)
    pub fn new(output_schema: Arc<Schema>, changes_type: bool) -> Arc<Self> {
        Arc::new_cyclic(|self_ref| Self {
            base: ConfigDataFixBase::new(output_schema, changes_type),
            self_ref: self_ref.clone(),
        })
    }

    /// java: private static <T> Dynamic<T> fixGeneral(Dynamic<T> dynamic)
    pub fn fix_general(dynamic: &Dynamic) -> Dynamic {
        return dynamic.update("general", |general| (general.update("itemTooltip", |item_tooltip| item_tooltip.set_field_if_present("dungeonQuality", general.get("dungeonQuality").result()))).remove("dungeonQuality"));
    }

    /// java: private static <T> Dynamic<T> fixUIAndVisuals(Dynamic<T> dynamic)
    pub fn fix_ui_and_visuals(dynamic: &Dynamic) -> Dynamic {
        let mut general: OptionalDynamic = dynamic.get("general");
        return (dynamic.set("uiAndVisuals", &(dynamic.empty_map().set_field_if_present("compactorDeletorPreview", general.get("compactorDeletorPreview").result())).set_field_if_present("dontStripSkinAlphaValues", general.get("dontStripSkinAlphaValues").result()).set_field_if_present("backpackPreviewWithoutShift", general.get("backpackPreviewWithoutShift").result()).set_field_if_present("hideEmptyTooltips", general.get("hideEmptyTooltips").result()).set_field_if_present("fancyCraftingTable", general.get("fancyCraftingTable").result()).set_field_if_present("hideStatusEffectOverlay", general.get("hideStatusEffectOverlay").result()).set_field_if_present("chestValue", general.get("chestValue").result()).set_field_if_present("itemCooldown", general.get("itemCooldown").result()).set_field_if_present("titleContainer", general.get("titleContainer").result()).set_field_if_present("tabHud", general.get("tabHud").result()).set_field_if_present("fancyAuctionHouse", general.get("fancyAuctionHouse").result()).set_field_if_present("bars", general.get("bars").result()).set_field_if_present("waypoints", general.get("waypoints").result()).set_field_if_present("teleportOverlay", general.get("teleportOverlay").result()).set_field_if_present("searchOverlay", general.get("searchOverlay").result()).set_field_if_present("flameOverlay", general.get("flameOverlay").result()))).update("general", |new_general| new_general.remove("compactorDeletorPreview").remove("dontStripSkinAlphaValues").remove("backpackPreviewWithoutShift").remove("hideEmptyTooltips").remove("fancyCraftingTable").remove("hideStatusEffectOverlay").remove("chestValue").remove("itemCooldown").remove("titleContainer").remove("tabHud").remove("fancyAuctionHouse").remove("bars").remove("waypoints").remove("teleportOverlay").remove("searchOverlay").remove("flameOverlay"));
    }

    /// java: private static <T> Dynamic<T> fixHelpers(Dynamic<T> dynamic)
    pub fn fix_helpers(dynamic: &Dynamic) -> Dynamic {
        let mut general: OptionalDynamic = dynamic.get("general");
        return (dynamic.set("helpers", &(dynamic.empty_map().set_field_if_present("enableNewYearCakesHelper", general.get("enableNewYearCakesHelper").result())).set_field_if_present("mythologicalRitual", general.get("mythologicalRitual").result()).set_field_if_present("experiments", general.get("experiments").result()).set_field_if_present("fishing", general.get("fishing").result()).set_field_if_present("fairySouls", general.get("fairySouls").result()))).update("general", |new_general| new_general.remove("enableNewYearCakesHelper").remove("mythologicalRitual").remove("experiments").remove("fishing").remove("fairySouls"));
    }

    /// java: private static <T> Dynamic<T> fixDungeons(Dynamic<T> dynamic)
    pub fn fix_dungeons(dynamic: &Dynamic) -> Dynamic {
        let mut general: OptionalDynamic = dynamic.get("general");
        let mut dungeons: OptionalDynamic = dynamic.get("locations").get("dungeons");
        return (dynamic.set("dungeons", &(dynamic.empty_map().set_field_if_present("fancyPartyFinder", general.get("betterPartyFinder").result())).set_field_if_present("croesusHelper", dungeons.get("croesusHelper").result()).set_field_if_present("playerSecretsTracker", dungeons.get("playerSecretsTracker").result()).set_field_if_present("starredMobGlow", dungeons.get("starredMobGlow").result()).set_field_if_present("starredMobBoundingBoxes", dungeons.get("starredMobBoundingBoxes").result()).set_field_if_present("allowDroppingProtectedItems", dungeons.get("allowDroppingProtectedItems").result()).set("dungeonMap", &(dynamic.empty_map().set_field_if_present("enableMap", dungeons.get("enableMap").result())).set_field_if_present("mapScaling", dungeons.get("mapScaling").result()).set_field_if_present("mapX", dungeons.get("mapX").result()).set_field_if_present("mapY", dungeons.get("mapY").result())).set("puzzleSolvers", &(dynamic.empty_map().set_field_if_present("solveThreeWeirdos", dungeons.get("solveThreeWeirdos").result())).set_field_if_present("blazeSolver", dungeons.get("blazeSolver").result()).set_field_if_present("creeperSolver", dungeons.get("creeperSolver").result()).set_field_if_present("solveTrivia", dungeons.get("solveTrivia").result()).set_field_if_present("solveTicTacToe", dungeons.get("solveTicTacToe").result()).set_field_if_present("solveWaterboard", dungeons.get("solveWaterboard").result()).set_field_if_present("solveBoulder", dungeons.get("solveBoulder").result()).set_field_if_present("solveIceFill", dungeons.get("solveIceFill").result()).set_field_if_present("solveSilverfish", dungeons.get("solveSilverfish").result())).set("theProfessor", &(dynamic.empty_map().set_field_if_present("fireFreezeStaffTimer", dungeons.get("fireFreezeStaffTimer").result())).set_field_if_present("floor3GuardianHealthDisplay", dungeons.get("floor3GuardianHealthDisplay").result())).set_field_if_present("livid", dungeons.get("lividColor").result()).set_field_if_present("terminals", dungeons.get("terminals").result()).set_field_if_present("secretWaypoints", dungeons.get("secretWaypoints").result()).set_field_if_present("mimicMessage", dungeons.get("mimicMessage").result()).set_field_if_present("doorHighlight", dungeons.get("doorHighlight").result()).set_field_if_present("dungeonScore", dungeons.get("dungeonScore").result()).set_field_if_present("dungeonChestProfit", dungeons.get("dungeonChestProfit").result()))).update("locations", |locations| locations.remove("dungeons")).update("general", |new_general| new_general.remove("betterPartyFinder"));
    }

    /// java: private static <T> Dynamic<T> fixCrimsonIsle(Dynamic<T> dynamic)
    pub fn fix_crimson_isle(dynamic: &Dynamic) -> Dynamic {
        return (dynamic.set_field_if_present("crimsonIsle", dynamic.get("locations").get("crimsonIsle").result())).update("locations", |locations| locations.remove("crimsonIsle"));
    }

    /// java: private static <T> Dynamic<T> fixMining(Dynamic<T> dynamic)
    pub fn fix_mining(dynamic: &Dynamic) -> Dynamic {
        let mut dwarven_mines: OptionalDynamic = dynamic.get("locations").get("dwarvenMines");
        return dynamic
            .set(
                "mining",
                &dynamic
                    .empty_map()
                    .set_field_if_present("enableDrillFuel", dwarven_mines.get("enableDrillFuel").result())
                    .set(
                        "dwarvenMines",
                        &dynamic
                            .empty_map()
                            .set_field_if_present("solveFetchur", dwarven_mines.get("solveFetchur").result())
                            .set_field_if_present("solvePuzzler", dwarven_mines.get("solvePuzzler").result()),
                    )
                    .set(
                        "dwarvenHud",
                        // java: .result().orElseThrow() - NoSuchElementException when absent
                        &dwarven_mines
                            .get("dwarvenHud")
                            .result()
                            .expect("java.util.NoSuchElementException: No value present")
                            .rename_field("x", "commissionsX")
                            .rename_field("y", "commissionsY"),
                    )
                    .set_field_if_present("crystalsHud", dwarven_mines.get("crystalsHud").result())
                    .set_field_if_present("crystalsWaypoints", dwarven_mines.get("crystalsWaypoints").result())
                    .set(
                        "crystalHollows",
                        &dynamic
                            .empty_map()
                            .set_field_if_present("metalDetectorHelper", dwarven_mines.get("metalDetectorHelper").result()),
                    ),
            )
            .update("locations", |locations| locations.remove("dwarvenMines"));
    }

    /// java: private static <T> Dynamic<T> fixFarming(Dynamic<T> dynamic)
    pub fn fix_farming(dynamic: &Dynamic) -> Dynamic {
        return (dynamic.set("farming", &dynamic.empty_map().set_field_if_present("garden", dynamic.get("locations").get("garden").result()))).update("locations", |locations| locations.remove("garden"));
    }

    /// java: private static <T> Dynamic<T> fixOtherLocations(Dynamic<T> dynamic)
    pub fn fix_other_locations(dynamic: &Dynamic) -> Dynamic {
        return dynamic.rename_field("locations", "otherLocations");
    }

    /// java: private static <T> Dynamic<T> fixSlayers(Dynamic<T> dynamic)
    pub fn fix_slayers(dynamic: &Dynamic) -> Dynamic {
        return dynamic.rename_field("slayer", "slayers");
    }

    /// java: private static <T> Dynamic<T> fixChat(Dynamic<T> dynamic)
    pub fn fix_chat(dynamic: &Dynamic) -> Dynamic {
        return dynamic.rename_field("messages", "chat");
    }

    /// java: private static <T> Dynamic<T> fixQuickNav(Dynamic<T> dynamic)
    pub fn fix_quick_nav(dynamic: &Dynamic) -> Dynamic {
        return dynamic.update("quickNav", |quick_nav| {
            quick_nav.update_map_values(|button| {
                // java: button.getFirst().asString().getOrThrow() - throws on a non-string key
                if button.0.as_string().get_or_throw().unwrap_or_else(|e| panic!("java.lang.IllegalStateException: {e}")).starts_with("button") {
                    // java: button.mapSecond(ConfigFix1::fixQuickNavButton)
                    let second = ConfigFix1::fix_quick_nav_button(&button.1);
                    (button.0, second)
                } else {
                    button
                }
            })
        });
    }

    /// java: private static <T> Dynamic<T> fixQuickNavButton(Dynamic<T> button)
    pub fn fix_quick_nav_button(button: &Dynamic) -> Dynamic {
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
                // java: ConfigDataFixer.LOGGER.error(LogUtils.FATAL_MARKER, "[Skyblocker Config Data Fixer] Failed to convert nbt to components!", e);
                logger().error(&format!("[Skyblocker Config Data Fixer] Failed to convert nbt to components! {}", compat::panic_message(&e)));
            }
        }
        return nbt.create_string("[]");
    }

    /// java: private static <T> Dynamic<T> fixMisc(Dynamic<T> dynamic)
    pub fn fix_misc(dynamic: &Dynamic) -> Dynamic {
        return (dynamic.set("misc", &dynamic.empty_map().set_field_if_present("richPresence", dynamic.get("richPresence").result()))).remove("richPresence");
    }

    /// java: private <T> Dynamic<T> fix(Dynamic<T> dynamic)
    pub fn fix(&self, dynamic: &Dynamic) -> Dynamic {
        return ConfigFix1::fix_misc(&ConfigFix1::fix_quick_nav(&ConfigFix1::fix_chat(&ConfigFix1::fix_slayers(&ConfigFix1::fix_other_locations(&ConfigFix1::fix_farming(&ConfigFix1::fix_mining(&ConfigFix1::fix_crimson_isle(&ConfigFix1::fix_dungeons(&ConfigFix1::fix_helpers(&ConfigFix1::fix_ui_and_visuals(&ConfigFix1::fix_general(&self.fix_version(dynamic)))))))))))));
    }
}

impl DataFix for ConfigFix1 {
    fn data_fix(&self) -> &DataFixFields {
        &self.base.fields
    }

    /// java: @Override protected TypeRewriteRule makeRule()
    fn make_rule(&self) -> TypeRewriteRule {
        let this = self.self_ref.clone();
        return data_fix::fix_type_everywhere_typed_same(
            "ConfigFix1",
            self.get_input_schema()
                .get_type(config_data_fixer::CONFIG_TYPE.get())
                .unwrap_or_else(|e| panic!("java.lang.IllegalArgumentException: {e}")),
            Arc::new(move |typed: Typed| {
                let Some(this) = this.upgrade() else { return Err("ConfigFix1: the fix was dropped".to_string()) };
                typed.update(&dsl::remainder_finder(), &|dynamic| this.fix(&dynamic))
            }),
        );
    }
}

impl ConfigDataFix for ConfigFix1 {
    fn config_data_fix_base(&self) -> &ConfigDataFixBase {
        &self.base
    }
}
