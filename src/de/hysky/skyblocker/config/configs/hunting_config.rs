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
