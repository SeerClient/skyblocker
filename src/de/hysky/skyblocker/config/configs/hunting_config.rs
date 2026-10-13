//! Ported from de.hysky.skyblocker.config.configs.HuntingConfig by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::sky_block_colors;
use mcrust::java::awt::color::Color;
use mcrust::net::minecraft::world::item::dye_color::DyeColor;

/// java: public class HuntingConfig
#[derive(Clone)]
pub struct HuntingConfig {
    /// java: public HuntingBox huntingBox = new HuntingBox();
    pub hunting_box: HuntingBox,
    /// java: public MoongladeMobs moongladeMobs = new MoongladeMobs();
    pub moonglade_mobs: MoongladeMobs,
    /// java: public TorrhusMobs torrhusMobs = new TorrhusMobs();
    pub torrhus_mobs: TorrhusMobs,
    /// java: public Safari safari = new Safari();
    pub safari: Safari,
    /// java: public CavernBiome cavernBiome = new CavernBiome();
    pub cavern_biome: CavernBiome,
    /// java: public ForestBiome forestBiome = new ForestBiome();
    pub forest_biome: ForestBiome,
    /// java: public HauntedBiome hauntedBiome = new HauntedBiome();
    pub haunted_biome: HauntedBiome,
    /// java: public IcyBiome icyBiome = new IcyBiome();
    pub icy_biome: IcyBiome,
    /// java: public FloorDrops floorDrops = new FloorDrops();
    pub floor_drops: FloorDrops,
    /// java: @Deprecated public transient LassoHud lassoHud = new LassoHud();
    pub lasso_hud: LassoHud,
}

impl HuntingConfig {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            hunting_box: HuntingBox::new(),
            moonglade_mobs: MoongladeMobs::new(),
            torrhus_mobs: TorrhusMobs::new(),
            safari: Safari::new(),
            cavern_biome: CavernBiome::new(),
            forest_biome: ForestBiome::new(),
            haunted_biome: HauntedBiome::new(),
            icy_biome: IcyBiome::new(),
            floor_drops: FloorDrops::new(),
            lasso_hud: LassoHud::new(),
        };
        this
    }

}

/// java: public static class HuntingBox
#[derive(Clone)]
pub struct HuntingBox {
    /// java: public boolean enabled = true;
    pub enabled: bool,
}

impl HuntingBox {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            enabled: true,
        };
        this
    }

}

/// java: public static class MoongladeMobs
#[derive(Clone)]
pub struct MoongladeMobs {
    /// java: public boolean silencePhantoms = true;
    pub silence_phantoms: bool,
    /// java: public boolean highlightHideonleaf = true;
    pub highlight_hideonleaf: bool,
    /// java: public Color hideonleafGlowColor = new Color(DyeColor.YELLOW.getTextColor(), false);
    pub hideonleaf_glow_color: Color,
    /// java: public boolean highlightShellwise = true;
    pub highlight_shellwise: bool,
    /// java: public Color shellwiseGlowColor = new Color(DyeColor.ORANGE.getTextColor(), false);
    pub shellwise_glow_color: Color,
    /// java: public boolean highlightCoralot = true;
    pub highlight_coralot: bool,
    /// java: public Color coralotGlowColor = new Color(DyeColor.BLUE.getTextColor(), false);
    pub coralot_glow_color: Color,
    /// java: public boolean highlightInvisibug = true;
    pub highlight_invisibug: bool,
    /// java: public Color invisibugGlowColor = new Color(DyeColor.CYAN.getTextColor(), false);
    pub invisibug_glow_color: Color,
}

impl MoongladeMobs {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            silence_phantoms: true,
            highlight_hideonleaf: true,
            hideonleaf_glow_color: Color::from_rgba(DyeColor::Yellow.get_text_color(), false),
            highlight_shellwise: true,
            shellwise_glow_color: Color::from_rgba(DyeColor::Orange.get_text_color(), false),
            highlight_coralot: true,
            coralot_glow_color: Color::from_rgba(DyeColor::Blue.get_text_color(), false),
            highlight_invisibug: true,
            invisibug_glow_color: Color::from_rgba(DyeColor::Cyan.get_text_color(), false),
        };
        this
    }

}

/// java: public static class TorrhusMobs
#[derive(Clone)]
pub struct TorrhusMobs {
    /// java: public boolean highlightHideonsun = true;
    pub highlight_hideonsun: bool,
    /// java: public Color hideonsunHighlightColor = new Color(DyeColor.CYAN.getTextColor(), false);
    pub hideonsun_highlight_color: Color,
    /// java: public boolean highlightBlueJay = true;
    pub highlight_blue_jay: bool,
    /// java: public Color blueJayHighlightColor = new Color(DyeColor.GREEN.getTextColor(), false);
    pub blue_jay_highlight_color: Color,
}

impl TorrhusMobs {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            highlight_hideonsun: true,
            hideonsun_highlight_color: Color::from_rgba(DyeColor::Cyan.get_text_color(), false),
            highlight_blue_jay: true,
            blue_jay_highlight_color: Color::from_rgba(DyeColor::Green.get_text_color(), false),
        };
        this
    }

}

/// java: public static class Safari
#[derive(Clone)]
pub struct Safari {
    /// java: public boolean silencePhantoms = true;
    pub silence_phantoms: bool,
    /// java: public boolean highlightSparklingCritters = false;
    pub highlight_sparkling_critters: bool,
    /// java: public Color sparklingCritterHighlightColor = new Color(SkyBlockColors.GOLD.getValue(), false);
    pub sparkling_critter_highlight_color: Color,
    /// java: public boolean critterCapsuleHelper = false;
    pub critter_capsule_helper: bool,
    /// java: public Color critterCapsuleHelperColor = new Color(DyeColor.GREEN.getTextColor(), false);
    pub critter_capsule_helper_color: Color,
}

impl Safari {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            silence_phantoms: true,
            highlight_sparkling_critters: false,
            sparkling_critter_highlight_color: Color::from_rgba(sky_block_colors::GOLD.get().get_value(), false),
            critter_capsule_helper: false,
            critter_capsule_helper_color: Color::from_rgba(DyeColor::Green.get_text_color(), false),
        };
        this
    }

}

/// java: public static class CavernBiome
#[derive(Clone)]
pub struct CavernBiome {
    /// java: public boolean highlightRockmiteMounds = true;
    pub highlight_rockmite_mounds: bool,
    /// java: public Color rockmiteMoundHighlightColor = new Color(DyeColor.LIGHT_BLUE.getTextColor(), false);
    pub rockmite_mound_highlight_color: Color,
}

impl CavernBiome {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            highlight_rockmite_mounds: true,
            rockmite_mound_highlight_color: Color::from_rgba(DyeColor::LightBlue.get_text_color(), false),
        };
        this
    }

}

/// java: public static class ForestBiome
#[derive(Clone)]
pub struct ForestBiome {
    /// java: public boolean highlightHideonfloor = true;
    pub highlight_hideonfloor: bool,
    /// java: public Color hideonfloorHighlightColor = new Color(DyeColor.MAGENTA.getTextColor(), false);
    pub hideonfloor_highlight_color: Color,
}

impl ForestBiome {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            highlight_hideonfloor: true,
            hideonfloor_highlight_color: Color::from_rgba(DyeColor::Magenta.get_text_color(), false),
        };
        this
    }

}

/// java: public static class HauntedBiome
#[derive(Clone)]
pub struct HauntedBiome {
    /// java: public boolean highlightDuplico = true;
    pub highlight_duplico: bool,
    /// java: public Color duplicoHighlightColor = new Color(DyeColor.RED.getTextColor(), false);
    pub duplico_highlight_color: Color,
    /// java: public boolean highlightBloodbat = true;
    pub highlight_bloodbat: bool,
    /// java: public Color bloodbatHighlightColor = new Color(DyeColor.LIME.getTextColor(), false);
    pub bloodbat_highlight_color: Color,
    /// java: public boolean ignoreSlotLockingForShiningCoins = false;
    pub ignore_slot_locking_for_shining_coins: bool,
}

impl HauntedBiome {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            highlight_duplico: true,
            duplico_highlight_color: Color::from_rgba(DyeColor::Red.get_text_color(), false),
            highlight_bloodbat: true,
            bloodbat_highlight_color: Color::from_rgba(DyeColor::Lime.get_text_color(), false),
            ignore_slot_locking_for_shining_coins: false,
        };
        this
    }

}

/// java: public static class IcyBiome
#[derive(Clone)]
pub struct IcyBiome {
    /// java: public boolean hideColdOverlay = false;
    pub hide_cold_overlay: bool,
}

impl IcyBiome {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            hide_cold_overlay: false,
        };
        this
    }

}

/// java: public static class FloorDrops
#[derive(Clone)]
pub struct FloorDrops {
    /// java: public boolean highlightFloorDrops = true;
    pub highlight_floor_drops: bool,
    /// java: public Color floorDropHighlightColor = new Color(DyeColor.ORANGE.getTextureDiffuseColor(), false);
    pub floor_drop_highlight_color: Color,
}

impl FloorDrops {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            highlight_floor_drops: true,
            floor_drop_highlight_color: Color::from_rgba(DyeColor::Orange.get_texture_diffuse_color(), false),
        };
        this
    }

}

/// java: public static class LassoHud
#[derive(Clone)]
pub struct LassoHud {
    /// java: public boolean enabled = true;
    pub enabled: bool,
}

impl LassoHud {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            enabled: true,
        };
        this
    }

}

// Generated Gson forms for src/de/hysky/skyblocker/config/configs/hunting_config.rs (fabric2seer round 24): APPEND to that file.
// Needs `serde_json` in Cargo.toml and the `compat` of this round (dandelion::compat::GsonValue, ClassOf, ...).

/// HuntingConfig's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl HuntingConfig {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("huntingBox".to_string(), ((&self.hunting_box)).__gson_write()),
            ("moongladeMobs".to_string(), ((&self.moonglade_mobs)).__gson_write()),
            ("torrhusMobs".to_string(), ((&self.torrhus_mobs)).__gson_write()),
            ("safari".to_string(), ((&self.safari)).__gson_write()),
            ("cavernBiome".to_string(), ((&self.cavern_biome)).__gson_write()),
            ("forestBiome".to_string(), ((&self.forest_biome)).__gson_write()),
            ("hauntedBiome".to_string(), ((&self.haunted_biome)).__gson_write()),
            ("icyBiome".to_string(), ((&self.icy_biome)).__gson_write()),
            ("floorDrops".to_string(), ((&self.floor_drops)).__gson_write()),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<HuntingConfig> {
        let __o = __v.as_object()?;
        let mut __this = HuntingConfig::new();
        if let Some(__v) = __o.get("huntingBox") {
            if let Some(__x) = HuntingBox::__gson_read(__v) { __this.hunting_box = __x; }
        }
        if let Some(__v) = __o.get("moongladeMobs") {
            if let Some(__x) = MoongladeMobs::__gson_read(__v) { __this.moonglade_mobs = __x; }
        }
        if let Some(__v) = __o.get("torrhusMobs") {
            if let Some(__x) = TorrhusMobs::__gson_read(__v) { __this.torrhus_mobs = __x; }
        }
        if let Some(__v) = __o.get("safari") {
            if let Some(__x) = Safari::__gson_read(__v) { __this.safari = __x; }
        }
        if let Some(__v) = __o.get("cavernBiome") {
            if let Some(__x) = CavernBiome::__gson_read(__v) { __this.cavern_biome = __x; }
        }
        if let Some(__v) = __o.get("forestBiome") {
            if let Some(__x) = ForestBiome::__gson_read(__v) { __this.forest_biome = __x; }
        }
        if let Some(__v) = __o.get("hauntedBiome") {
            if let Some(__x) = HauntedBiome::__gson_read(__v) { __this.haunted_biome = __x; }
        }
        if let Some(__v) = __o.get("icyBiome") {
            if let Some(__x) = IcyBiome::__gson_read(__v) { __this.icy_biome = __x; }
        }
        if let Some(__v) = __o.get("floorDrops") {
            if let Some(__x) = FloorDrops::__gson_read(__v) { __this.floor_drops = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                "huntingBox" => self.hunting_box.__gson_patch(__rest, __value),
                "moongladeMobs" => self.moonglade_mobs.__gson_patch(__rest, __value),
                "torrhusMobs" => self.torrhus_mobs.__gson_patch(__rest, __value),
                "safari" => self.safari.__gson_patch(__rest, __value),
                "cavernBiome" => self.cavern_biome.__gson_patch(__rest, __value),
                "forestBiome" => self.forest_biome.__gson_patch(__rest, __value),
                "hauntedBiome" => self.haunted_biome.__gson_patch(__rest, __value),
                "icyBiome" => self.icy_biome.__gson_patch(__rest, __value),
                "floorDrops" => self.floor_drops.__gson_patch(__rest, __value),
                _ => false,
            },
            [] => false,
        }
    }

    /// `HuntingConfig.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<HuntingConfig> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.HuntingConfig",
            new: HuntingConfig::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: HuntingConfig::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// HuntingBox's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl HuntingBox {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("enabled".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enabled))),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<HuntingBox> {
        let __o = __v.as_object()?;
        let mut __this = HuntingBox::new();
        if let Some(__v) = __o.get("enabled") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enabled = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "enabled" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enabled = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `HuntingBox.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<HuntingBox> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.HuntingConfig$HuntingBox",
            new: HuntingBox::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: HuntingBox::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// MoongladeMobs's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl MoongladeMobs {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("silencePhantoms".to_string(), dandelion::compat::GsonValue::Bool(*(&self.silence_phantoms))),
            ("highlightHideonleaf".to_string(), dandelion::compat::GsonValue::Bool(*(&self.highlight_hideonleaf))),
            ("hideonleafGlowColor".to_string(), dandelion::compat::GsonValue::int(((&self.hideonleaf_glow_color)).get_rgb() as i64)),
            ("highlightShellwise".to_string(), dandelion::compat::GsonValue::Bool(*(&self.highlight_shellwise))),
            ("shellwiseGlowColor".to_string(), dandelion::compat::GsonValue::int(((&self.shellwise_glow_color)).get_rgb() as i64)),
            ("highlightCoralot".to_string(), dandelion::compat::GsonValue::Bool(*(&self.highlight_coralot))),
            ("coralotGlowColor".to_string(), dandelion::compat::GsonValue::int(((&self.coralot_glow_color)).get_rgb() as i64)),
            ("highlightInvisibug".to_string(), dandelion::compat::GsonValue::Bool(*(&self.highlight_invisibug))),
            ("invisibugGlowColor".to_string(), dandelion::compat::GsonValue::int(((&self.invisibug_glow_color)).get_rgb() as i64)),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<MoongladeMobs> {
        let __o = __v.as_object()?;
        let mut __this = MoongladeMobs::new();
        if let Some(__v) = __o.get("silencePhantoms") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.silence_phantoms = __x; }
        }
        if let Some(__v) = __o.get("highlightHideonleaf") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.highlight_hideonleaf = __x; }
        }
        if let Some(__v) = __o.get("hideonleafGlowColor") {
            if let Some(__x) = dandelion::compat::gson_i64(__v).map(|__n| Color::from_rgba(__n as i32, true)) { __this.hideonleaf_glow_color = __x; }
        }
        if let Some(__v) = __o.get("highlightShellwise") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.highlight_shellwise = __x; }
        }
        if let Some(__v) = __o.get("shellwiseGlowColor") {
            if let Some(__x) = dandelion::compat::gson_i64(__v).map(|__n| Color::from_rgba(__n as i32, true)) { __this.shellwise_glow_color = __x; }
        }
        if let Some(__v) = __o.get("highlightCoralot") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.highlight_coralot = __x; }
        }
        if let Some(__v) = __o.get("coralotGlowColor") {
            if let Some(__x) = dandelion::compat::gson_i64(__v).map(|__n| Color::from_rgba(__n as i32, true)) { __this.coralot_glow_color = __x; }
        }
        if let Some(__v) = __o.get("highlightInvisibug") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.highlight_invisibug = __x; }
        }
        if let Some(__v) = __o.get("invisibugGlowColor") {
            if let Some(__x) = dandelion::compat::gson_i64(__v).map(|__n| Color::from_rgba(__n as i32, true)) { __this.invisibug_glow_color = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "silencePhantoms" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.silence_phantoms = *__b; true } else { false },
                "highlightHideonleaf" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.highlight_hideonleaf = *__b; true } else { false },
                "highlightShellwise" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.highlight_shellwise = *__b; true } else { false },
                "highlightCoralot" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.highlight_coralot = *__b; true } else { false },
                "highlightInvisibug" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.highlight_invisibug = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `MoongladeMobs.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<MoongladeMobs> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.HuntingConfig$MoongladeMobs",
            new: MoongladeMobs::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: MoongladeMobs::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// TorrhusMobs's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl TorrhusMobs {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("highlightHideonsun".to_string(), dandelion::compat::GsonValue::Bool(*(&self.highlight_hideonsun))),
            ("hideonsunHighlightColor".to_string(), dandelion::compat::GsonValue::int(((&self.hideonsun_highlight_color)).get_rgb() as i64)),
            ("highlightBlueJay".to_string(), dandelion::compat::GsonValue::Bool(*(&self.highlight_blue_jay))),
            ("blueJayHighlightColor".to_string(), dandelion::compat::GsonValue::int(((&self.blue_jay_highlight_color)).get_rgb() as i64)),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<TorrhusMobs> {
        let __o = __v.as_object()?;
        let mut __this = TorrhusMobs::new();
        if let Some(__v) = __o.get("highlightHideonsun") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.highlight_hideonsun = __x; }
        }
        if let Some(__v) = __o.get("hideonsunHighlightColor") {
            if let Some(__x) = dandelion::compat::gson_i64(__v).map(|__n| Color::from_rgba(__n as i32, true)) { __this.hideonsun_highlight_color = __x; }
        }
        if let Some(__v) = __o.get("highlightBlueJay") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.highlight_blue_jay = __x; }
        }
        if let Some(__v) = __o.get("blueJayHighlightColor") {
            if let Some(__x) = dandelion::compat::gson_i64(__v).map(|__n| Color::from_rgba(__n as i32, true)) { __this.blue_jay_highlight_color = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "highlightHideonsun" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.highlight_hideonsun = *__b; true } else { false },
                "highlightBlueJay" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.highlight_blue_jay = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `TorrhusMobs.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<TorrhusMobs> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.HuntingConfig$TorrhusMobs",
            new: TorrhusMobs::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: TorrhusMobs::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// Safari's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl Safari {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("silencePhantoms".to_string(), dandelion::compat::GsonValue::Bool(*(&self.silence_phantoms))),
            ("highlightSparklingCritters".to_string(), dandelion::compat::GsonValue::Bool(*(&self.highlight_sparkling_critters))),
            ("sparklingCritterHighlightColor".to_string(), dandelion::compat::GsonValue::int(((&self.sparkling_critter_highlight_color)).get_rgb() as i64)),
            ("critterCapsuleHelper".to_string(), dandelion::compat::GsonValue::Bool(*(&self.critter_capsule_helper))),
            ("critterCapsuleHelperColor".to_string(), dandelion::compat::GsonValue::int(((&self.critter_capsule_helper_color)).get_rgb() as i64)),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<Safari> {
        let __o = __v.as_object()?;
        let mut __this = Safari::new();
        if let Some(__v) = __o.get("silencePhantoms") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.silence_phantoms = __x; }
        }
        if let Some(__v) = __o.get("highlightSparklingCritters") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.highlight_sparkling_critters = __x; }
        }
        if let Some(__v) = __o.get("sparklingCritterHighlightColor") {
            if let Some(__x) = dandelion::compat::gson_i64(__v).map(|__n| Color::from_rgba(__n as i32, true)) { __this.sparkling_critter_highlight_color = __x; }
        }
        if let Some(__v) = __o.get("critterCapsuleHelper") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.critter_capsule_helper = __x; }
        }
        if let Some(__v) = __o.get("critterCapsuleHelperColor") {
            if let Some(__x) = dandelion::compat::gson_i64(__v).map(|__n| Color::from_rgba(__n as i32, true)) { __this.critter_capsule_helper_color = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "silencePhantoms" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.silence_phantoms = *__b; true } else { false },
                "highlightSparklingCritters" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.highlight_sparkling_critters = *__b; true } else { false },
                "critterCapsuleHelper" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.critter_capsule_helper = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `Safari.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<Safari> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.HuntingConfig$Safari",
            new: Safari::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: Safari::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// CavernBiome's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl CavernBiome {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("highlightRockmiteMounds".to_string(), dandelion::compat::GsonValue::Bool(*(&self.highlight_rockmite_mounds))),
            ("rockmiteMoundHighlightColor".to_string(), dandelion::compat::GsonValue::int(((&self.rockmite_mound_highlight_color)).get_rgb() as i64)),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<CavernBiome> {
        let __o = __v.as_object()?;
        let mut __this = CavernBiome::new();
        if let Some(__v) = __o.get("highlightRockmiteMounds") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.highlight_rockmite_mounds = __x; }
        }
        if let Some(__v) = __o.get("rockmiteMoundHighlightColor") {
            if let Some(__x) = dandelion::compat::gson_i64(__v).map(|__n| Color::from_rgba(__n as i32, true)) { __this.rockmite_mound_highlight_color = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "highlightRockmiteMounds" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.highlight_rockmite_mounds = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `CavernBiome.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<CavernBiome> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.HuntingConfig$CavernBiome",
            new: CavernBiome::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: CavernBiome::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// ForestBiome's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl ForestBiome {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("highlightHideonfloor".to_string(), dandelion::compat::GsonValue::Bool(*(&self.highlight_hideonfloor))),
            ("hideonfloorHighlightColor".to_string(), dandelion::compat::GsonValue::int(((&self.hideonfloor_highlight_color)).get_rgb() as i64)),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<ForestBiome> {
        let __o = __v.as_object()?;
        let mut __this = ForestBiome::new();
        if let Some(__v) = __o.get("highlightHideonfloor") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.highlight_hideonfloor = __x; }
        }
        if let Some(__v) = __o.get("hideonfloorHighlightColor") {
            if let Some(__x) = dandelion::compat::gson_i64(__v).map(|__n| Color::from_rgba(__n as i32, true)) { __this.hideonfloor_highlight_color = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "highlightHideonfloor" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.highlight_hideonfloor = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `ForestBiome.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<ForestBiome> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.HuntingConfig$ForestBiome",
            new: ForestBiome::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: ForestBiome::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// HauntedBiome's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl HauntedBiome {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("highlightDuplico".to_string(), dandelion::compat::GsonValue::Bool(*(&self.highlight_duplico))),
            ("duplicoHighlightColor".to_string(), dandelion::compat::GsonValue::int(((&self.duplico_highlight_color)).get_rgb() as i64)),
            ("highlightBloodbat".to_string(), dandelion::compat::GsonValue::Bool(*(&self.highlight_bloodbat))),
            ("bloodbatHighlightColor".to_string(), dandelion::compat::GsonValue::int(((&self.bloodbat_highlight_color)).get_rgb() as i64)),
            ("ignoreSlotLockingForShiningCoins".to_string(), dandelion::compat::GsonValue::Bool(*(&self.ignore_slot_locking_for_shining_coins))),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<HauntedBiome> {
        let __o = __v.as_object()?;
        let mut __this = HauntedBiome::new();
        if let Some(__v) = __o.get("highlightDuplico") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.highlight_duplico = __x; }
        }
        if let Some(__v) = __o.get("duplicoHighlightColor") {
            if let Some(__x) = dandelion::compat::gson_i64(__v).map(|__n| Color::from_rgba(__n as i32, true)) { __this.duplico_highlight_color = __x; }
        }
        if let Some(__v) = __o.get("highlightBloodbat") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.highlight_bloodbat = __x; }
        }
        if let Some(__v) = __o.get("bloodbatHighlightColor") {
            if let Some(__x) = dandelion::compat::gson_i64(__v).map(|__n| Color::from_rgba(__n as i32, true)) { __this.bloodbat_highlight_color = __x; }
        }
        if let Some(__v) = __o.get("ignoreSlotLockingForShiningCoins") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.ignore_slot_locking_for_shining_coins = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "highlightDuplico" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.highlight_duplico = *__b; true } else { false },
                "highlightBloodbat" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.highlight_bloodbat = *__b; true } else { false },
                "ignoreSlotLockingForShiningCoins" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.ignore_slot_locking_for_shining_coins = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `HauntedBiome.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<HauntedBiome> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.HuntingConfig$HauntedBiome",
            new: HauntedBiome::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: HauntedBiome::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// IcyBiome's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl IcyBiome {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("hideColdOverlay".to_string(), dandelion::compat::GsonValue::Bool(*(&self.hide_cold_overlay))),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<IcyBiome> {
        let __o = __v.as_object()?;
        let mut __this = IcyBiome::new();
        if let Some(__v) = __o.get("hideColdOverlay") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.hide_cold_overlay = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "hideColdOverlay" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.hide_cold_overlay = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `IcyBiome.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<IcyBiome> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.HuntingConfig$IcyBiome",
            new: IcyBiome::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: IcyBiome::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// FloorDrops's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl FloorDrops {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("highlightFloorDrops".to_string(), dandelion::compat::GsonValue::Bool(*(&self.highlight_floor_drops))),
            ("floorDropHighlightColor".to_string(), dandelion::compat::GsonValue::int(((&self.floor_drop_highlight_color)).get_rgb() as i64)),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<FloorDrops> {
        let __o = __v.as_object()?;
        let mut __this = FloorDrops::new();
        if let Some(__v) = __o.get("highlightFloorDrops") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.highlight_floor_drops = __x; }
        }
        if let Some(__v) = __o.get("floorDropHighlightColor") {
            if let Some(__x) = dandelion::compat::gson_i64(__v).map(|__n| Color::from_rgba(__n as i32, true)) { __this.floor_drop_highlight_color = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "highlightFloorDrops" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.highlight_floor_drops = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `FloorDrops.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<FloorDrops> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.HuntingConfig$FloorDrops",
            new: FloorDrops::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: FloorDrops::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}
