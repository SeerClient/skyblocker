//! Ported from de.hysky.skyblocker.skyblock.fancybars.StatusBarType by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::fancybars::bar_positioner::BarAnchor;
use crate::de::hysky::skyblocker::skyblock::fancybars::status_bar::ExperienceStatusBar;
use crate::de::hysky::skyblocker::skyblock::fancybars::status_bar::HealthStatusBar;
use crate::de::hysky::skyblocker::skyblock::fancybars::status_bar::ManaStatusBar;
use crate::de::hysky::skyblocker::skyblock::fancybars::status_bar::StatusBar;
use crate::de::hysky::skyblocker::skyblock::fancybars::status_bar::StatusBarBase;
use crate::de::hysky::skyblocker::skyblock::fancybars::status_bar::VitalityStatusBar;
use mcrust::java::awt::color::Color;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: public enum StatusBarType implements StringRepresentable
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum StatusBarType {
    Health,
    Vitality,
    Intelligence,
    Defense,
    Experience,
    Speed,
    Air,
}

impl StatusBarType {
    pub const VALUES: [StatusBarType; 7] = [StatusBarType::Health, StatusBarType::Vitality, StatusBarType::Intelligence, StatusBarType::Defense, StatusBarType::Experience, StatusBarType::Speed, StatusBarType::Air];

    /// java: values()
    pub fn values() -> Vec<StatusBarType> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            StatusBarType::Health => "HEALTH",
            StatusBarType::Vitality => "VITALITY",
            StatusBarType::Intelligence => "INTELLIGENCE",
            StatusBarType::Defense => "DEFENSE",
            StatusBarType::Experience => "EXPERIENCE",
            StatusBarType::Speed => "SPEED",
            StatusBarType::Air => "AIR",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> StatusBarType {
        match name {
            "HEALTH" => StatusBarType::Health,
            "VITALITY" => StatusBarType::Vitality,
            "INTELLIGENCE" => StatusBarType::Intelligence,
            "DEFENSE" => StatusBarType::Defense,
            "EXPERIENCE" => StatusBarType::Experience,
            "SPEED" => StatusBarType::Speed,
            "AIR" => StatusBarType::Air,
            _ => panic!("IllegalArgumentException: No enum constant StatusBarType.{name}"),
        }
    }

    /// the constant's constructor data (Java keeps it in the constant's fields)
    pub fn fields(self) -> &'static StatusBarTypeFields {
        static FIELDS: LazyLock<Vec<StatusBarTypeFields>> = LazyLock::new(|| vec![
            StatusBarTypeFields::new("health".to_string(), BarAnchor::HotbarTop, 0, vec![Color::new(255, 0, 0), Color::new(255, 220, 0)], true, true, Some(Color::new(255, 85, 85)), component::translatable("skyblocker.bars.config.health")),
            StatusBarTypeFields::new("vitality".to_string(), BarAnchor::HotbarRight, 1, vec![Color::new(200, 50, 40)], false, true, Some(Color::new(220, 60, 60)), component::translatable("skyblocker.bars.config.vitality")),
            StatusBarTypeFields::new("intelligence".to_string(), BarAnchor::HotbarTop, 0, vec![Color::new(0, 255, 255), Color::new(180, 0, 255)], true, true, Some(Color::new(85, 255, 255)), component::translatable("skyblocker.bars.config.intelligence")),
            StatusBarTypeFields::new("defense".to_string(), BarAnchor::HotbarRight, 0, vec![Color::new(255, 255, 255)], false, false, Some(Color::new(185, 185, 185)), component::translatable("skyblocker.bars.config.defense")),
            StatusBarTypeFields::new("experience".to_string(), BarAnchor::HotbarTop, 1, vec![Color::new(100, 230, 70)], false, false, Some(Color::new(128, 255, 32)), component::translatable("skyblocker.bars.config.experience")),
            StatusBarTypeFields::new("speed".to_string(), BarAnchor::HotbarRight, 0, vec![Color::new(255, 255, 255)], false, true, Some(Color::new(185, 185, 185)), component::translatable("skyblocker.bars.config.speed")),
            StatusBarTypeFields::new("air".to_string(), BarAnchor::HotbarRight, 1, vec![Color::new(135, 206, 250)], false, true, Some(Color::new(150, 230, 255)), component::translatable("skyblocker.bars.config.air")),
        ]);
        &FIELDS[self as usize]
    }

    /// java: public static StatusBarType from(String id)
    pub fn from(id: &str) -> StatusBarType {
        for r#type in StatusBarType::values() {
            if r#type.fields().id == id {
                return r#type;
            }
        }
        panic!("IllegalArgumentException: {}", format!("Unknown status bar type: {}", id).to_string());
    }

    /// java: @Override public String getSerializedName()
    pub fn get_serialized_name(&self) -> String {
        return self.fields().id.clone();
    }

    /// java: public BarPositioner.BarAnchor getDefaultAnchor()
    pub fn get_default_anchor(&self) -> BarAnchor {
        return self.fields().default_anchor;
    }

    /// java: public int getDefaultGridY()
    pub fn get_default_grid_y(&self) -> i32 {
        return self.fields().default_grid_y;
    }

    /// java: public Color[] getColors()
    pub fn get_colors(&self) -> Vec<Color> {
        return self.fields().colors.clone();
    }

    /// java: public boolean hasOverflow()
    pub fn has_overflow(&self) -> bool {
        return self.fields().has_overflow;
    }

    /// java: public boolean hasMax()
    pub fn has_max(&self) -> bool {
        return self.fields().has_max;
    }

    /// java: public @Nullable Color getTextColor()
    pub fn get_text_color(&self) -> Option<Color> {
        return self.fields().text_color.clone();
    }

    /// java: public Component getName()
    pub fn get_name(&self) -> MutableComponent {
        return self.fields().name.clone();
    }

    /// java: public StatusBar newStatusBar()
    pub fn new_status_bar(&self) -> compat::Shared<dyn StatusBar> {
        return match *self {
            StatusBarType::Health => (HealthStatusBar::new(*self) as compat::Shared<dyn StatusBar>),
            StatusBarType::Vitality => (VitalityStatusBar::new(*self) as compat::Shared<dyn StatusBar>),
            StatusBarType::Intelligence => (ManaStatusBar::new(*self) as compat::Shared<dyn StatusBar>),
            StatusBarType::Experience => (ExperienceStatusBar::new(*self) as compat::Shared<dyn StatusBar>),
            _ => (StatusBarBase::new(*self) as compat::Shared<dyn StatusBar>),
        };
    }

}

impl std::fmt::Display for StatusBarType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

/// the constructor data of each StatusBarType constant
pub struct StatusBarTypeFields {
    pub id: String,
    pub default_anchor: BarAnchor,
    pub default_grid_y: i32,
    pub colors: Vec<Color>,
    pub has_overflow: bool,
    pub has_max: bool,
    pub text_color: Option<Color>,
    pub name: MutableComponent,
}

impl StatusBarTypeFields {
    fn new(mut id: String, mut default_anchor: BarAnchor, mut default_grid_y: i32, mut colors: Vec<Color>, mut has_overflow: bool, mut has_max: bool, mut text_color: Option<Color>, mut name: MutableComponent) -> Self {
        Self { id, default_anchor, default_grid_y, colors, has_overflow, has_max, text_color, name }
    }

}
