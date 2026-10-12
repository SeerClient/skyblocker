//! Ported from de.hysky.skyblocker.skyblock.fancybars.VanillaStyleManaBar by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::fancybars::fancy_status_bars;
use crate::de::hysky::skyblocker::skyblock::fancybars::fancy_status_bars::FancyStatusBars;
use crate::de::hysky::skyblocker::skyblock::status_bar_tracker::EstimatedResource;
use crate::de::hysky::skyblocker::skyblock::status_bar_tracker::StatusBarTracker;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::util::util;

/// java: private static int lastManaValue;
pub static LAST_MANA_VALUE: LazyLock<Mutex<i32>> = LazyLock::new(|| Mutex::new(0));
/// java: private static int manaValueBlinkStart;
pub static MANA_VALUE_BLINK_START: LazyLock<Mutex<i32>> = LazyLock::new(|| Mutex::new(0));
/// java: private static int lastOverflowValue;
pub static LAST_OVERFLOW_VALUE: LazyLock<Mutex<i32>> = LazyLock::new(|| Mutex::new(0));
/// java: private static int overflowValueBlinkStart;
pub static OVERFLOW_VALUE_BLINK_START: LazyLock<Mutex<i32>> = LazyLock::new(|| Mutex::new(0));
/// java: private static long blinkEndTime;
pub static BLINK_END_TIME: LazyLock<Mutex<i64>> = LazyLock::new(|| Mutex::new(0));
/// java: private static final Identifier MANABAR_FOOD_HUD_ID = SkyblockerMod.id("vanilla_style_mana_bar_food");
pub static MANABAR_FOOD_HUD_ID: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("vanilla_style_mana_bar_food"));
/// java: private static final Identifier MANABAR_MOUNT_HUD_ID = SkyblockerMod.id("vanilla_style_mana_bar_mount");
pub static MANABAR_MOUNT_HUD_ID: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("vanilla_style_mana_bar_mount"));
/// java: private static final Identifier CONTAINER_TEXTURE = SkyblockerMod.id("bars/vanilla_mana/container");
pub static CONTAINER_TEXTURE: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("bars/vanilla_mana/container"));
/// java: private static final Identifier MANA_FULL_TEXTURE = SkyblockerMod.id("bars/vanilla_mana/mana_full");
pub static MANA_FULL_TEXTURE: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("bars/vanilla_mana/mana_full"));
/// java: private static final Identifier MANA_HALF_TEXTURE = SkyblockerMod.id("bars/vanilla_mana/mana_half");
pub static MANA_HALF_TEXTURE: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("bars/vanilla_mana/mana_half"));
/// java: private static final Identifier OVERFLOW_FULL_TEXTURE = SkyblockerMod.id("bars/vanilla_mana/overflow_full");
pub static OVERFLOW_FULL_TEXTURE: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("bars/vanilla_mana/overflow_full"));
/// java: private static final Identifier OVERFLOW_HALF_TEXTURE = SkyblockerMod.id("bars/vanilla_mana/overflow_half");
pub static OVERFLOW_HALF_TEXTURE: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("bars/vanilla_mana/overflow_half"));
/// java: private static final Identifier OVERFLOW_DARK_FULL_TEXTURE = SkyblockerMod.id("bars/vanilla_mana/overflow_dark_full");
pub static OVERFLOW_DARK_FULL_TEXTURE: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("bars/vanilla_mana/overflow_dark_full"));
/// java: private static final Identifier OVERFLOW_DARK_HALF_TEXTURE = SkyblockerMod.id("bars/vanilla_mana/overflow_dark_half");
pub static OVERFLOW_DARK_HALF_TEXTURE: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("bars/vanilla_mana/overflow_dark_half"));
/// java: private static final Identifier CONTAINER_BLINK_TEXTURE = SkyblockerMod.id("bars/vanilla_mana/container_blink");
pub static CONTAINER_BLINK_TEXTURE: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("bars/vanilla_mana/container_blink"));
/// java: private static final Identifier MANA_FULL_BLINK_TEXTURE = SkyblockerMod.id("bars/vanilla_mana/mana_full_blink");
pub static MANA_FULL_BLINK_TEXTURE: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("bars/vanilla_mana/mana_full_blink"));
/// java: private static final Identifier MANA_HALF_BLINK_TEXTURE = SkyblockerMod.id("bars/vanilla_mana/mana_half_blink");
pub static MANA_HALF_BLINK_TEXTURE: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("bars/vanilla_mana/mana_half_blink"));
/// java: private static final Identifier OVERFLOW_FULL_BLINK_TEXTURE = SkyblockerMod.id("bars/vanilla_mana/overflow_full_blink");
pub static OVERFLOW_FULL_BLINK_TEXTURE: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("bars/vanilla_mana/overflow_full_blink"));
/// java: private static final Identifier OVERFLOW_HALF_BLINK_TEXTURE = SkyblockerMod.id("bars/vanilla_mana/overflow_half_blink");
pub static OVERFLOW_HALF_BLINK_TEXTURE: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("bars/vanilla_mana/overflow_half_blink"));
/// java: private static final Identifier OVERFLOW_DARK_FULL_BLINK_TEXTURE = SkyblockerMod.id("bars/vanilla_mana/overflow_dark_full_blink");
pub static OVERFLOW_DARK_FULL_BLINK_TEXTURE: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("bars/vanilla_mana/overflow_dark_full_blink"));
/// java: private static final Identifier OVERFLOW_DARK_HALF_BLINK_TEXTURE = SkyblockerMod.id("bars/vanilla_mana/overflow_dark_half_blink");
pub static OVERFLOW_DARK_HALF_BLINK_TEXTURE: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("bars/vanilla_mana/overflow_dark_half_blink"));

/// java: public class VanillaStyleManaBar
#[derive(Clone)]
pub struct VanillaStyleManaBar {}

impl VanillaStyleManaBar {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        let mut hide_if_vanilla_style_mana_bar_enabled: Arc<dyn Fn(crate::unresolved::HudElement) -> crate::unresolved::HudElement> = Arc::new(move |hud_element| {
            if VanillaStyleManaBar::is_enabled() {
                return Arc::new(move |_, _| {

                });
            }
            return hud_element;
        });
        // TODO(fabric2seer): HudElementRegistry.replaceElement: no SeerClient equivalent yet
        //   java: HudElementRegistry.replaceElement(VanillaHudElements.FOOD_BAR, hideIfVanillaStyleManaBarEnabled);
        // TODO(fabric2seer): HudElementRegistry.replaceElement: no SeerClient equivalent yet
        //   java: HudElementRegistry.replaceElement(VanillaHudElements.MOUNT_HEALTH, hideIfVanillaStyleManaBarEnabled);
        hud_render_callback::EVENT.register(Arc::new(|ctx: &mut hud_render_callback::HudRenderContext<'_>| {
            // SAFETY: `ctx` outlives every call made through `graphics` (this callback).
            let graphics = unsafe { ScriptGraphics::new(ctx.gl, &*ctx.minecraft, ctx.gui_width, ctx.gui_height) };
            let _session = graphics.session();
            if VanillaStyleManaBar::is_enabled() {
                VanillaStyleManaBar::extract_render_state(&graphics);
            }
        }));
        hud_render_callback::EVENT.register(Arc::new(|ctx: &mut hud_render_callback::HudRenderContext<'_>| {
            // SAFETY: `ctx` outlives every call made through `graphics` (this callback).
            let graphics = unsafe { ScriptGraphics::new(ctx.gl, &*ctx.minecraft, ctx.gui_width, ctx.gui_height) };
            let _session = graphics.session();
            if VanillaStyleManaBar::is_enabled() {
                VanillaStyleManaBar::extract_render_state(&graphics);
            }
        }));
    }

    /// java: private static boolean isEnabled()
    pub fn is_enabled() -> bool {
        return (Utils::is_on_skyblock() && SkyblockerConfigManager::get().ui_and_visuals.bars.enable_vanilla_style_mana_bar && !(FancyStatusBars::is_enabled(fancy_status_bars::INSTANCE.get_mut())));
    }

    /// java: private static void extractNotch(GuiGraphicsExtractor graphics, int column, int row, NotchType notchtype, boolean isHalf, boolean isBlinking)
    pub fn extract_notch(graphics: &GuiGraphicsExtractor, mut column: i32, mut row: i32, mut notchtype: NotchType, mut is_half: bool, mut is_blinking: bool) {
        let mut top: i32 = (gui_graphics_extractor::gui_height(&*compat::minecraft())).wrapping_sub(39);
        let mut right: i32 = ((gui_graphics_extractor::gui_width(&*compat::minecraft())).wrapping_div(2)).wrapping_add(91);
        // TODO(fabric2seer): static call HudAccessor.getFOOD_EMPTY_SPRITE(..)
        //   java: Identifier texture = SkyblockerConfigManager.get().uiAndVisuals.bars.useHungerBarSprites ? switch (notchtype) {
        //   java: 			case CONTAINER -> HudAccessor.getFOOD_EMPTY_SPRITE();
        //   java: 			case MANA -> !isHalf ? HudAccessor.getFOOD_FULL_SPRITE() : HudAccessor.getFOOD_HALF_SPRITE();
        //   java: 			case OVERFLOW -> !isHalf ? HudAccessor.getFOOD_FULL_HUNGER_SPRITE() : HudAccessor.getFOOD_HALF_HUNGER_SPRITE();
        //   java: 			case OVERFLOW_DARK -> HudAccessor.getFOOD_EMPTY_HUNGER_SPRITE();
        //   java: 		} : switch (notchtype) {
        //   java: 			case CONTAINER -> isBlinking ? CONTAINER_BLINK_TEXTURE : CONTAINER_TEXTURE;
        //   java: 			case MANA -> !isHalf ? (isBlinking ? MANA_FULL_BLINK_TEXTURE : MANA_FULL_TEXTURE) : (isBlinking ? MANA_HALF_BLINK_TEXTURE : MANA_HALF_TEXTURE);
        //   java: 			case OVERFLOW -> !isHalf ? (isBlinking ? OVERFLOW_FULL_BLINK_TEXTURE : OVERFLOW_FULL_TEXTURE) : (isBlinking ? OVERFLOW_HALF_BLINK_TEXTURE : OVERFLOW_HALF_TEXTURE);
        //   java: 			case OVERFLOW_DARK -> !isHalf ? (isBlinking ? OVERFLOW_DARK_FULL_BLINK_TEXTURE : OVERFLOW_DARK_FULL_TEXTURE) : (isBlinking ? OVERFLOW_DARK_HALF_BLINK_TEXTURE : OVERFLOW_DARK_HALF_TEXTURE);
        //   java: 		};
        let mut texture: Identifier = todo!("fabric2seer: the initialiser of `texture` (TODO above)");
        gui_graphics_extractor::blit_sprite_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), &texture, (right.wrapping_sub(column.wrapping_mul(8))).wrapping_sub(9), top.wrapping_sub(row.wrapping_mul(10)), 9, 9);
    }

    /// java: public static boolean extractRenderState(GuiGraphicsExtractor graphics)
    pub fn extract_render_state(graphics: &GuiGraphicsExtractor) -> bool {
        let mut mana: compat::Shared<EstimatedResource> = StatusBarTracker::get_mana();
        let mut current_time: i64 = util::get_millis();
        let mut blink_time_length: i64 = ((1000) as i64) as i64;
        let mut blink_frequency: i64 = ((300) as i64) as i64;
        let mut blinking_enabled: bool = SkyblockerConfigManager::get().ui_and_visuals.bars.enable_vanilla_style_mana_bar_blinking;
        if blinking_enabled && (compat::get_state(&LAST_MANA_VALUE)).wrapping_add(compat::get_state(&LAST_OVERFLOW_VALUE)) > (EstimatedResource::value(&mana)).wrapping_add(EstimatedResource::overflow(&mana)) && EstimatedResource::value(&mana) != EstimatedResource::max(&mana) {
            let mut just_started_blinking: bool = (compat::get_state(&BLINK_END_TIME) <= current_time);
            if just_started_blinking {
                compat::set_state(&MANA_VALUE_BLINK_START, compat::get_state(&LAST_MANA_VALUE));
                compat::set_state(&OVERFLOW_VALUE_BLINK_START, compat::get_state(&LAST_OVERFLOW_VALUE));
            }
            if compat::get_state(&BLINK_END_TIME) >= current_time {
                compat::set_state(&BLINK_END_TIME, (((current_time.wrapping_add(blink_time_length)).wrapping_div(blink_frequency)).wrapping_mul(blink_frequency)).wrapping_add((compat::get_state(&BLINK_END_TIME)).wrapping_rem(blink_frequency)));
            } else {
                compat::set_state(&BLINK_END_TIME, current_time.wrapping_add(blink_time_length));
            }
        }
        let mut blinking: bool = (blinking_enabled && compat::get_state(&BLINK_END_TIME) > current_time && (((compat::get_state(&BLINK_END_TIME)).wrapping_sub(current_time)).wrapping_div(blink_frequency.wrapping_div(2 as i64))).wrapping_rem(2 as i64) == (1 as i64));
        if compat::get_state(&BLINK_END_TIME) <= current_time {
            compat::set_state(&MANA_VALUE_BLINK_START, 0);
            compat::set_state(&OVERFLOW_VALUE_BLINK_START, 0);
        }
        compat::set_state(&LAST_MANA_VALUE, EstimatedResource::value(&mana));
        compat::set_state(&LAST_OVERFLOW_VALUE, EstimatedResource::overflow(&mana));
        let mut mana_notch_count: i32 = 20;
        let mut mana_half_notches: i32 = (((EstimatedResource::value(&mana)).wrapping_mul(mana_notch_count)).wrapping_mul(2)).wrapping_div(EstimatedResource::max(&mana));
        let mut mana_notches: i32 = ((((mana_half_notches as f64) / 2.0).ceil()) as i32);
        let mut mana_blink_half_notches: i32 = (((compat::get_state(&MANA_VALUE_BLINK_START)).wrapping_mul(mana_notch_count)).wrapping_mul(2)).wrapping_div(EstimatedResource::max(&mana));
        let mut mana_blink_notches: i32 = ((((mana_blink_half_notches as f64) / 2.0).ceil()) as i32);
        let mut overflow_half_notches: i32 = (((EstimatedResource::overflow(&mana)).wrapping_mul(mana_notch_count)).wrapping_mul(2)).wrapping_div(EstimatedResource::max(&mana));
        let mut overflow_notches: i32 = ((((overflow_half_notches as f64) / 2.0).ceil()) as i32);
        let mut overflow_blink_half_notches: i32 = (((compat::get_state(&OVERFLOW_VALUE_BLINK_START)).wrapping_mul(mana_notch_count)).wrapping_mul(2)).wrapping_div(EstimatedResource::max(&mana));
        let mut overflow_blink_notches: i32 = ((((overflow_blink_half_notches as f64) / 2.0).ceil()) as i32);
        for i in 0..mana_notch_count {
            let mut row: i32 = i.wrapping_div(10);
            let mut column: i32 = i.wrapping_rem(10);
            let mut mana_notch: bool = (i < mana_notches);
            let mut mana_notch_is_half: bool = (mana_notch && mana_notches.wrapping_sub(1) == i && mana_half_notches.wrapping_rem(2) == 1);
            let mut mana_blink_notch: bool = (i < mana_blink_notches);
            let mut mana_blink_notch_is_half: bool = (mana_blink_notch && mana_blink_notches.wrapping_sub(1) == i && mana_blink_half_notches.wrapping_rem(2) == 1);
            let mut overflow_notch: bool = (i < overflow_notches);
            let mut overflow_notch_is_half: bool = (overflow_notch && overflow_notches.wrapping_sub(1) == i && overflow_half_notches.wrapping_rem(2) == 1);
            let mut overflow_blink_notch: bool = (i < overflow_blink_notches);
            let mut overflow_blink_notch_is_half: bool = (overflow_blink_notch && overflow_blink_notches.wrapping_sub(1) == i && overflow_blink_half_notches.wrapping_rem(2) == 1);
            VanillaStyleManaBar::extract_notch(graphics, column, row, NotchType::Container, false, blinking);
            if mana_notches > 0 {
                if overflow_notch {
                    VanillaStyleManaBar::extract_notch(graphics, column, row, NotchType::OverflowDark, overflow_notch_is_half, blinking);
                }
                if mana_blink_notch && blinking {
                    VanillaStyleManaBar::extract_notch(graphics, column, row, NotchType::Mana, mana_blink_notch_is_half, true);
                }
                if mana_notch {
                    VanillaStyleManaBar::extract_notch(graphics, column, row, NotchType::Mana, mana_notch_is_half, false);
                }
            } else {
                if mana_blink_notch && blinking {
                    VanillaStyleManaBar::extract_notch(graphics, column, row, NotchType::Mana, mana_blink_notch_is_half, true);
                }
                if overflow_blink_notch && blinking {
                    VanillaStyleManaBar::extract_notch(graphics, column, row, NotchType::Overflow, overflow_blink_notch_is_half, true);
                }
                if overflow_notch {
                    VanillaStyleManaBar::extract_notch(graphics, column, row, NotchType::Overflow, overflow_notch_is_half, false);
                }
            }
        }
        return true;
    }

}

/// java: enum NotchType
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum NotchType {
    Container,
    Mana,
    Overflow,
    OverflowDark,
}

impl NotchType {
    pub const VALUES: [NotchType; 4] = [NotchType::Container, NotchType::Mana, NotchType::Overflow, NotchType::OverflowDark];

    /// java: values()
    pub fn values() -> Vec<NotchType> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            NotchType::Container => "CONTAINER",
            NotchType::Mana => "MANA",
            NotchType::Overflow => "OVERFLOW",
            NotchType::OverflowDark => "OVERFLOW_DARK",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> NotchType {
        match name {
            "CONTAINER" => NotchType::Container,
            "MANA" => NotchType::Mana,
            "OVERFLOW" => NotchType::Overflow,
            "OVERFLOW_DARK" => NotchType::OverflowDark,
            _ => panic!("IllegalArgumentException: No enum constant NotchType.{name}"),
        }
    }

}

impl std::fmt::Display for NotchType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
